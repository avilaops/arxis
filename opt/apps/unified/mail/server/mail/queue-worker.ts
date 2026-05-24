/**
 * Worker da fila de saída — entrega mensagens em mail_outbox.
 * Loop a cada 15 s. Tx FOR UPDATE SKIP LOCKED para concorrência segura.
 */

import nodemailer from "nodemailer";
import { pool, MAIL_HOSTNAME, readRawEmail, PORTS } from "./common";
import { log } from "@/lib/logger";

const POLL_MS = 15_000;
const MAX_ATTEMPTS = 5;
const N8N_EMAIL_WEBHOOK_URL = process.env.N8N_EMAIL_WEBHOOK_URL || "";
const N8N_EMAIL_WEBHOOK_TOKEN = process.env.N8N_EMAIL_WEBHOOK_TOKEN || "";

/** Verifica se o domínio é gerenciado localmente */
async function isLocalDomain(domain: string): Promise<boolean> {
  const { rows } = await pool.query(
    "SELECT 1 FROM mail_domains WHERE LOWER(domain) = LOWER($1) AND status = 'active' LIMIT 1",
    [domain]
  );
  return rows.length > 0;
}

async function relayExternalViaN8n(row: any, addrs: string[], raw: Buffer): Promise<{ ok: boolean; error?: string }> {
  if (!N8N_EMAIL_WEBHOOK_URL) {
    return { ok: false, error: "N8N_EMAIL_WEBHOOK_URL not configured" };
  }

  const response = await fetch(N8N_EMAIL_WEBHOOK_URL, {
    method: "POST",
    headers: {
      "content-type": "application/json",
      ...(N8N_EMAIL_WEBHOOK_TOKEN ? { "x-avilaops-email-token": N8N_EMAIL_WEBHOOK_TOKEN } : {}),
    },
    body: JSON.stringify({
      source: "avilaops-mail",
      messageId: row.message_id,
      outboxId: row.id,
      from: row.envelope_from,
      to: addrs,
      rawBase64: raw.toString("base64"),
      rawPath: row.raw_path,
      createdAt: row.created_at,
    }),
  });

  const text = await response.text().catch(() => "");
  if (!response.ok) {
    return { ok: false, error: `n8n webhook ${response.status}: ${text.slice(0, 500)}` };
  }

  return { ok: true };
}

async function deliverOne(row: any): Promise<{ ok: boolean; error?: string; bounce?: boolean }> {
  const raw = await readRawEmail(row.raw_path);

  // agrupa rcpts por domínio (entrega 1 conexão por domínio)
  const byDomain = new Map<string, string[]>();
  for (const addr of row.envelope_to as string[]) {
    const d = addr.split("@")[1]?.toLowerCase();
    if (!d) continue;
    if (!byDomain.has(d)) byDomain.set(d, []);
    byDomain.get(d)!.push(addr);
  }

  const errors: string[] = [];
  let bounce = false;

  for (const [domain, addrs] of byDomain) {
    // Verifica se o domínio é gerenciado localmente
    const isLocal = await isLocalDomain(domain);

    if (isLocal) {
      // Entrega local via SMTP Inbound (localhost:2525)
      try {
        const transport = nodemailer.createTransport({
          host: 'localhost',
          port: PORTS.smtpInbound,
          secure: false,
          name: MAIL_HOSTNAME,
          tls: { rejectUnauthorized: false },
          connectionTimeout: 15_000,
          greetingTimeout: 15_000,
          socketTimeout: 30_000,
        });
        await transport.sendMail({
          envelope: { from: row.envelope_from, to: addrs },
          raw,
        });
        transport.close();
        console.log(`[queue] local delivery: ${domain} → localhost:${PORTS.smtpInbound}`);
        continue; // Sucesso, próximo domínio
      } catch (err: any) {
        errors.push(`localhost:${PORTS.smtpInbound}: ${err.message ?? err}`);
        const msg = String(err?.responseCode ?? err?.message ?? "");
        if (/^5\d\d/.test(msg)) bounce = true;
        continue;
      }
    }

    // Entrega externa via n8n relay. Evita SMTP direto/MX na porta 25, bloqueada na Hetzner.
    try {
      const result = await relayExternalViaN8n(row, addrs, raw);
      if (!result.ok) {
        errors.push(`${domain}: ${result.error}`);
      }
    } catch (err: any) {
      errors.push(`${domain}: n8n relay failed: ${err.message ?? err}`);
    }
  }

  if (errors.length) return { ok: false, error: errors.join(" | "), bounce };
  return { ok: true };
}

async function tick(): Promise<void> {
  const client = await pool.connect();
  try {
    await client.query("BEGIN");
    const { rows } = await client.query(
      `SELECT * FROM mail_outbox
        WHERE status IN ('queued','deferred') AND next_attempt_at <= NOW()
        ORDER BY next_attempt_at ASC
        FOR UPDATE SKIP LOCKED LIMIT 5`
    );
    if (!rows.length) { await client.query("COMMIT"); return; }
    for (const r of rows) {
      await client.query("UPDATE mail_outbox SET status = 'sending' WHERE id = $1", [r.id]);
    }
    await client.query("COMMIT");

    for (const r of rows) {
      try {
        const result = await deliverOne(r);
        if (result.ok) {
          await pool.query(
            "UPDATE mail_outbox SET status='sent', sent_at=NOW() WHERE id=$1",
            [r.id]
          );
          // Log estruturado para cada destinatário
          for (const to of r.envelope_to as string[]) {
            log.mail.sent(r.envelope_from, to, `queue-${r.id}`);
          }
          console.log(`[queue] sent #${r.id} from=${r.envelope_from} to=${r.envelope_to.join(',')}`);
        } else {
          const attempts = (r.attempts ?? 0) + 1;
          const giveUp = attempts >= MAX_ATTEMPTS || result.bounce;
          const status = giveUp ? (result.bounce ? "bounced" : "failed") : "deferred";
          const backoff = Math.min(60 * Math.pow(2, attempts), 24 * 3600); // segs, cap 1 dia
          await pool.query(
            `UPDATE mail_outbox
                SET status=$1, attempts=$2, last_error=$3,
                    next_attempt_at = CASE WHEN $1='deferred' THEN NOW() + ($4 || ' seconds')::interval ELSE next_attempt_at END
              WHERE id=$5`,
            [status, attempts, result.error?.slice(0, 1000) ?? null, String(backoff), r.id]
          );

          // Log estruturado de falhas
          for (const to of r.envelope_to as string[]) {
            if (status === 'bounced') {
              log.mail.bounced(r.envelope_from, to, result.error ?? 'unknown');
            } else if (status === 'failed') {
              log.mail.failed(r.envelope_from, to, result.error ?? 'max attempts reached');
            }
          }
          console.warn(`[queue] ${status} #${r.id} attempt=${attempts}: ${result.error}`);
        }
      } catch (err: any) {
        console.error(`[queue] crash on #${r.id}:`, err);
        await pool.query(
          "UPDATE mail_outbox SET status='deferred', attempts=attempts+1, last_error=$2 WHERE id=$1",
          [r.id, String(err.message ?? err).slice(0, 1000)]
        );
      }
    }
  } catch (err) {
    await client.query("ROLLBACK").catch(() => {});
    console.error("[queue] tick error:", err);
  } finally {
    client.release();
  }
}

export function startQueueWorker(): void {
  console.log("[queue] worker started");
  void tick();
  setInterval(() => void tick(), POLL_MS);
}
