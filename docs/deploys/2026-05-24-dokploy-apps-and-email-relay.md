# Dokploy apps and email relay - 2026-05-24

## Deployed apps

| App | Containers | Public domains |
| --- | --- | --- |
| `arxis-web` | `arxis-web` | `arxis.avilaops.com` |
| `arxisvr-backend` | `arxisvr-backend` | `arxisvr.avilaops.com` (`/api`, `/health`) |
| `arxisvr-web` | `arxisvr-web` | `arxisvr.avilaops.com` |
| `mail-web` | `mail-web`, `mail-server` | `mail.avilaops.com` |
| `platform-ui` | `platform-ui` | `platform.avilaops.com` |
| `infra-n8n` | `infra-n8n`, `infra-n8n-worker` | `n8n.avilaops.com`, `flow182.avilaops.com` |

## Email standard

Outbound SMTP through direct MX delivery on port `25` is blocked by the host provider. Application outbound email must use the n8n relay webhook instead of direct SMTP.

Standard environment variables:

```env
EMAIL_DELIVERY_PROVIDER=n8n
N8N_EMAIL_WEBHOOK_URL=https://n8n.avilaops.com/webhook/avilaops-email-relay
N8N_EMAIL_WEBHOOK_TOKEN=<secret-token>
SMTP_OUTBOUND_DISABLED=true
```

The Mail queue worker keeps local-domain delivery through the local SMTP inbound service, but routes external recipients to the n8n relay. If the relay returns a non-2xx response, the outbox row remains deferred/retryable instead of being marked as sent.

## n8n queue worker

The n8n deployment uses `EXECUTIONS_MODE=queue`, so `infra-n8n-worker` must run alongside `infra-n8n`; otherwise production webhooks enqueue jobs but do not respond.

## Current relay workflow

`etc/n8n/workflows/platform/avilaops-email-relay.workflow.json` is a placeholder relay workflow. It returns `503` until an outbound provider is configured in n8n, which is intentional to keep mail queued safely.

To enable actual sending, replace the placeholder node with an approved provider integration such as Resend, Brevo, SES, or another API-based email provider that does not require direct outbound port `25`.
