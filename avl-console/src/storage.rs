//! Storage Browser - S3-compatible file management

use crate::{error::Result, state::AppState};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(storage_browser_page))
        .route("/buckets", get(list_buckets))
        .route("/buckets/:bucket/files", get(list_files))
        .route("/buckets/:bucket/upload", post(upload_file))
        .with_state(state)
}

async fn storage_browser_page() -> impl IntoResponse {
    Html(STORAGE_BROWSER_HTML)
}

#[derive(Serialize)]
struct BucketInfo {
    name: String,
    region: String,
    files: usize,
    size_bytes: u64,
    created_at: String,
}

async fn list_buckets(State(_state): State<Arc<AppState>>) -> Result<Json<Vec<BucketInfo>>> {
    // TODO: Query actual AVL Storage service
    let buckets = vec![
        BucketInfo {
            name: "user-uploads".to_string(),
            region: "sa-east-1".to_string(),
            files: 1_250,
            size_bytes: 5_000_000_000,
            created_at: "2024-10-01T10:00:00Z".to_string(),
        },
        BucketInfo {
            name: "static-assets".to_string(),
            region: "sa-east-1".to_string(),
            files: 3_500,
            size_bytes: 15_000_000_000,
            created_at: "2024-09-15T14:30:00Z".to_string(),
        },
        BucketInfo {
            name: "backups".to_string(),
            region: "sa-east-1".to_string(),
            files: 45,
            size_bytes: 50_000_000_000,
            created_at: "2024-08-01T09:00:00Z".to_string(),
        },
    ];

    Ok(Json(buckets))
}

#[derive(Serialize)]
struct FileInfo {
    name: String,
    size_bytes: u64,
    content_type: String,
    last_modified: String,
}

async fn list_files(
    Path(bucket): Path<String>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<FileInfo>>> {
    // TODO: Query actual files from storage
    tracing::info!("Listing files in bucket: {}", bucket);

    let files = vec![
        FileInfo {
            name: "image1.jpg".to_string(),
            size_bytes: 2_500_000,
            content_type: "image/jpeg".to_string(),
            last_modified: "2024-11-20T15:30:00Z".to_string(),
        },
        FileInfo {
            name: "document.pdf".to_string(),
            size_bytes: 500_000,
            content_type: "application/pdf".to_string(),
            last_modified: "2024-11-21T10:00:00Z".to_string(),
        },
    ];

    Ok(Json(files))
}

#[derive(Deserialize)]
struct UploadRequest {
    filename: String,
    _content_type: String,
    _data: String, // Base64 encoded
}

async fn upload_file(
    Path(bucket): Path<String>,
    State(_state): State<Arc<AppState>>,
    Json(req): Json<UploadRequest>,
) -> Result<Json<serde_json::Value>> {
    // TODO: Upload to actual storage
    tracing::info!("Uploading file {} to bucket {}", req.filename, bucket);

    Ok(Json(serde_json::json!({
        "success": true,
        "filename": req.filename,
        "bucket": bucket,
    })))
}

const STORAGE_BROWSER_HTML: &str = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Storage Browser - AVL Console</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: #0a0e1a;
            color: #e0e6ed;
        }
        .header {
            background: #0f1419;
            border-bottom: 1px solid #1a1f2e;
            padding: 1rem 2rem;
        }
        .container { max-width: 1400px; margin: 2rem auto; padding: 0 2rem; }
        .bucket-card {
            background: #0f1419;
            border: 1px solid #1a1f2e;
            border-radius: 8px;
            padding: 1.5rem;
            margin-bottom: 1rem;
            cursor: pointer;
            transition: all 0.2s;
        }
        .bucket-card:hover {
            border-color: #00d4ff;
            transform: translateX(4px);
        }
        .bucket-name {
            font-size: 1.25rem;
            font-weight: bold;
            color: #00d4ff;
            margin-bottom: 0.5rem;
        }
        .bucket-info {
            display: flex;
            gap: 2rem;
            font-size: 0.875rem;
            color: #8b92a0;
        }
        .file-list {
            background: #0f1419;
            border: 1px solid #1a1f2e;
            border-radius: 8px;
            padding: 1.5rem;
            margin-top: 2rem;
        }
        .file-item {
            padding: 1rem;
            border-bottom: 1px solid #1a1f2e;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .file-item:last-child { border-bottom: none; }
        button {
            background: #00d4ff;
            color: #0a0e1a;
            border: none;
            padding: 0.75rem 1.5rem;
            border-radius: 4px;
            font-weight: bold;
            cursor: pointer;
        }
        button:hover { background: #00b8e6; }
    </style>
</head>
<body>
    <div class="header">
        <h1>💾 Storage Browser</h1>
    </div>

    <div class="container">
        <h2 style="margin-bottom: 1rem;">Seus Buckets</h2>
        <div id="buckets"></div>

        <div class="file-list" id="files" style="display: none;">
            <h3 style="margin-bottom: 1rem;">Arquivos</h3>
            <div id="fileList"></div>
        </div>
    </div>

    <script>
        let currentBucket = null;

        async function loadBuckets() {
            const res = await fetch('/storage/buckets');
            const buckets = await res.json();
            const container = document.getElementById('buckets');
            container.innerHTML = buckets.map(bucket => `
                <div class="bucket-card" onclick="loadFiles('${bucket.name}')">
                    <div class="bucket-name">📦 ${bucket.name}</div>
                    <div class="bucket-info">
                        <span>Região: ${bucket.region}</span>
                        <span>Arquivos: ${bucket.files.toLocaleString()}</span>
                        <span>Tamanho: ${(bucket.size_bytes / 1_000_000_000).toFixed(2)} GB</span>
                    </div>
                </div>
            `).join('');
        }

        async function loadFiles(bucket) {
            currentBucket = bucket;
            const res = await fetch(\`/storage/buckets/\${bucket}/files\`);
            const files = await res.json();
            const container = document.getElementById('fileList');
            container.innerHTML = files.map(file => `
                <div class="file-item">
                    <div>
                        <strong>${file.name}</strong><br>
                        <span style="color: #8b92a0; font-size: 0.875rem;">
                            ${(file.size_bytes / 1_000_000).toFixed(2)} MB • ${file.content_type}
                        </span>
                    </div>
                    <button onclick="downloadFile('${file.name}')">Download</button>
                </div>
            `).join('');
            document.getElementById('files').style.display = 'block';
        }

        function downloadFile(filename) {
            alert('Download: ' + filename);
        }

        loadBuckets();
    </script>
</body>
</html>"#;
