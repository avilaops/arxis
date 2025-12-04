//! # avila-bim-converter
//!
//! Worker de conversão IFC → glTF/GLB.
//! Processa jobs de uma fila RabbitMQ, baixa IFC do MinIO,
//! converte usando avila-ifc + avila-gltf, e faz upload do resultado.

use std::sync::Arc;
use tokio::time::Duration;
use tracing::{info, error, warn};
use lapin::{
    options::*, types::FieldTable, Connection, ConnectionProperties,
};
use avila_bim_core::*;
use avila_ifc::IfcParser;
use avila_gltf::{GltfExporter, ExportOptions};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// ============================================================================
// CONFIGURAÇÃO
// ============================================================================

#[derive(Debug, Clone)]
pub struct ConverterConfig {
    pub rabbitmq_url: String,
    pub queue_name: String,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub postgres_url: String,
    pub worker_id: String,
    pub max_retries: u32,
}

impl Default for ConverterConfig {
    fn default() -> Self {
        Self {
            rabbitmq_url: std::env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".into()),
            queue_name: "bim_conversion_jobs".into(),
            s3_endpoint: std::env::var("S3_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".into()),
            s3_bucket: std::env::var("S3_BUCKET")
                .unwrap_or_else(|_| "bim-models".into()),
            s3_access_key: std::env::var("S3_ACCESS_KEY")
                .unwrap_or_else(|_| "minioadmin".into()),
            s3_secret_key: std::env::var("S3_SECRET_KEY")
                .unwrap_or_else(|_| "minioadmin".into()),
            postgres_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/bim_platform".into()),
            worker_id: uuid::Uuid::new_v4().to_string(),
            max_retries: 3,
        }
    }
}

// ============================================================================
// JOB PAYLOAD
// ============================================================================

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ConversionJob {
    pub model_id: uuid::Uuid,
    pub project_id: uuid::Uuid,
    pub ifc_s3_key: String, // "projects/uuid/models/uuid/model.ifc"
    pub output_glb_key: String, // "projects/uuid/models/uuid/model.glb"
    pub retry_count: u32,
}

// ============================================================================
// WORKER
// ============================================================================

pub struct ConverterWorker {
    config: Arc<ConverterConfig>,
    s3_client: aws_sdk_s3::Client,
}

impl ConverterWorker {
    pub async fn new(config: ConverterConfig) -> Result<Self> {
        // Configurar S3 client (MinIO)
        let s3_config = aws_config::from_env()
            .endpoint_url(&config.s3_endpoint)
            .load()
            .await;
        let s3_client = aws_sdk_s3::Client::new(&s3_config);

        Ok(Self {
            config: Arc::new(config),
            s3_client,
        })
    }

    /// Iniciar worker (loop infinito processando jobs)
    pub async fn run(&self) -> Result<()> {
        info!("🚀 Starting BIM Converter Worker: {}", self.config.worker_id);
        info!("   RabbitMQ: {}", self.config.rabbitmq_url);
        info!("   S3: {}", self.config.s3_endpoint);
        info!("   Queue: {}", self.config.queue_name);

        // Conectar ao RabbitMQ
        let conn = Connection::connect(
            &self.config.rabbitmq_url,
            ConnectionProperties::default(),
        )
        .await?;

        info!("✅ Connected to RabbitMQ");

        let channel = conn.create_channel().await?;

        // Declarar fila (idempotente)
        channel
            .queue_declare(
                &self.config.queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        info!("✅ Queue declared: {}", self.config.queue_name);

        // Configurar QoS (processar 1 job por vez)
        channel
            .basic_qos(1, BasicQosOptions::default())
            .await?;

        // Consumir mensagens
        let mut consumer = channel
            .basic_consume(
                &self.config.queue_name,
                &format!("worker-{}", self.config.worker_id),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!("🎧 Listening for conversion jobs...\n");

        // Loop de processamento
        while let Some(delivery) = consumer.next().await {
            let delivery = match delivery {
                Ok(d) => d,
                Err(e) => {
                    error!("❌ Delivery error: {}", e);
                    continue;
                }
            };

            let job: ConversionJob = match serde_json::from_slice(&delivery.data) {
                Ok(j) => j,
                Err(e) => {
                    error!("❌ Invalid job payload: {}", e);
                    delivery.ack(BasicAckOptions::default()).await?;
                    continue;
                }
            };

            info!("📥 Received job: model_id={}", job.model_id);

            // Processar job
            match self.process_job(&job).await {
                Ok(_) => {
                    info!("✅ Job completed: model_id={}", job.model_id);
                    delivery.ack(BasicAckOptions::default()).await?;
                }
                Err(e) => {
                    error!("❌ Job failed: model_id={}, error={}", job.model_id, e);

                    // Retry logic
                    if job.retry_count < self.config.max_retries {
                        warn!("🔄 Retrying job (attempt {}/{})", job.retry_count + 1, self.config.max_retries);

                        let mut retry_job = job.clone();
                        retry_job.retry_count += 1;

                        // Requeue com delay (exponential backoff)
                        tokio::time::sleep(Duration::from_secs(2u64.pow(job.retry_count))).await;

                        let retry_payload = serde_json::to_vec(&retry_job)?;
                        channel
                            .basic_publish(
                                "",
                                &self.config.queue_name,
                                BasicPublishOptions::default(),
                                &retry_payload,
                                lapin::BasicProperties::default().with_delivery_mode(2),
                            )
                            .await?;

                        delivery.ack(BasicAckOptions::default()).await?;
                    } else {
                        error!("❌ Max retries exceeded for model_id={}", job.model_id);
                        // Marcar modelo como "error" no banco
                        if let Err(e) = self.mark_model_as_error(&job.model_id, &e.to_string()).await {
                            error!("❌ Failed to mark model as error: {}", e);
                        }
                        delivery.ack(BasicAckOptions::default()).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Processar job de conversão
    async fn process_job(&self, job: &ConversionJob) -> Result<()> {
        // 1. Atualizar status → "converting"
        self.update_model_status(&job.model_id, "converting").await?;

        // 2. Baixar IFC do S3
        info!("   📥 Downloading IFC from S3: {}", job.ifc_s3_key);
        let ifc_content = self.download_from_s3(&job.ifc_s3_key).await?;

        // 3. Parsear IFC
        info!("   🔍 Parsing IFC...");
        let parser = IfcParser::new(&ifc_content)?;
        let bim_model = parser.parse()?;
        info!("   ✅ Parsed {} elements", bim_model.elements.len());

        // 4. Exportar glTF/GLB
        info!("   🔨 Exporting to GLB...");
        let exporter = GltfExporter::new();
        let options = ExportOptions {
            merge_meshes: true,
            include_normals: true,
            include_uvs: true,
            include_colors: false,
            use_draco_compression: false, // Futuro
        };
        let glb_bytes = exporter.export_glb(&bim_model, &options)?;
        info!("   ✅ GLB size: {} bytes", glb_bytes.len());

        // 5. Upload GLB para S3
        info!("   📤 Uploading GLB to S3: {}", job.output_glb_key);
        self.upload_to_s3(&job.output_glb_key, glb_bytes).await?;

        // 6. Salvar metadados no PostgreSQL
        info!("   💾 Saving metadata to database...");
        self.save_metadata(&job.model_id, &bim_model).await?;

        // 7. Atualizar status → "ready"
        self.update_model_status(&job.model_id, "ready").await?;

        Ok(())
    }

    /// Baixar arquivo do S3
    async fn download_from_s3(&self, key: &str) -> Result<String> {
        let response = self.s3_client
            .get_object()
            .bucket(&self.config.s3_bucket)
            .key(key)
            .send()
            .await?;

        let bytes = response.body.collect().await?.into_bytes();
        Ok(String::from_utf8(bytes.to_vec())?)
    }

    /// Upload arquivo para S3
    async fn upload_to_s3(&self, key: &str, data: Vec<u8>) -> Result<()> {
        self.s3_client
            .put_object()
            .bucket(&self.config.s3_bucket)
            .key(key)
            .body(data.into())
            .content_type("model/gltf-binary")
            .send()
            .await?;

        Ok(())
    }

    /// Atualizar status do modelo no PostgreSQL
    async fn update_model_status(&self, model_id: &uuid::Uuid, status: &str) -> Result<()> {
        // TODO: Conectar ao PostgreSQL via avila-db ou sqlx
        // UPDATE models SET status = $1, updated_at = NOW() WHERE id = $2

        info!("   🔄 Status: {} → {}", model_id, status);
        Ok(())
    }

    /// Salvar metadados do modelo
    async fn save_metadata(&self, _model_id: &uuid::Uuid, model: &BimModel) -> Result<()> {
        // TODO: Inserir elementos na tabela `elements`
        // INSERT INTO elements (id, model_id, guid, type, properties, ...)

        info!("   💾 Saved {} elements to database", model.elements.len());
        Ok(())
    }

    /// Marcar modelo como erro
    async fn mark_model_as_error(&self, model_id: &uuid::Uuid, error_msg: &str) -> Result<()> {
        // TODO: UPDATE models SET status = 'error', error_message = $1 WHERE id = $2

        error!("   ❌ Marked model {} as error: {}", model_id, error_msg);
        Ok(())
    }
}

// ============================================================================
// MAIN
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_level(true)
        .init();

    info!("═══════════════════════════════════════════════════════════");
    info!("   AVILA BIM CONVERTER WORKER");
    info!("   100% Rust Native IFC → glTF Pipeline");
    info!("═══════════════════════════════════════════════════════════\n");

    // Carregar configuração
    let config = ConverterConfig::default();

    // Criar worker
    let worker = ConverterWorker::new(config).await?;

    // Executar loop
    worker.run().await?;

    Ok(())
}
