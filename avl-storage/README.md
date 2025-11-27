# 🗄️ AVL Storage

### *The Storage Fortress. The Object Engine.*

**AVL** (fortress) + **STORAGE** (engine) = **AVL Storage**

> *Where objects find permanent refuge and engines deliver at speed*

[![Crates.io](https://img.shields.io/crates/v/avl-storage.svg)](https://crates.io/crates/avl-storage)
[![Documentation](https://docs.rs/avl-storage/badge.svg)](https://docs.rs/avl-storage)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-CE422B.svg)](https://www.rust-lang.org/)
[![AVL Cloud](https://img.shields.io/badge/AVL-Cloud%20Platform-00d4ff)](https://avila.inc)

**🇧🇷 Latency 3-8ms in Brazil** | **💰 50% cheaper than S3** | **🔄 S3-compatible API**

---

## 🎯 What is AVL Storage?

**AVL Storage** is the **S3-compatible object storage** for the **AVL Cloud Platform** - built as a **fortress for your files** and an **engine for your data**.

Like Arxis provides the mathematical citadel, AVL Storage provides the **object citadel**:
- 🏛️ **Solid Storage**: Durable, replicated, versioned object storage
- ⚙️ **Transfer Engine**: High-speed uploads/downloads optimized for Brazil
- 🛡️ **Protection**: Encryption, immutability, lifecycle policies
- 🚀 **Performance**: 3-8ms latency in Brazil, multipart uploads

---

## 🌟 Key Features

### 🇧🇷 **Optimized for Brazil & LATAM**
- **3-8ms latency** in São Paulo, Rio, Brasília
- **Sub-30ms** throughout Latin America
- **50% cheaper** than AWS S3 for Brazilian traffic
- **No egress fees** within Brazil

### 📦 **S3-Compatible API**
- **Drop-in replacement** for AWS S3
- **SDK compatibility**: aws-sdk-rust, boto3, s3cmd
- **Standard operations**: PUT, GET, DELETE, LIST
- **Multipart uploads** for large files (>5 GB)

### 🗜️ **Intelligent Compression**
- **Automatic compression** via `avila-compress`
- **LZ4 for hot data** (fast access)
- **Zstandard for cold data** (high ratio)
- **Transparent decompression** on read

### 🔒 **Security & Compliance**
- **Encryption at rest** (AES-256)
- **Encryption in transit** (TLS 1.3)
- **Versioning** and immutability
- **Access logs** and audit trails

### 💰 **Pricing (Brazil)**
- **R$ 0,15** per GB/month storage
- **R$ 0,05** per GB transfer (within Brazil)
- **FREE** transfers between AVL services
- **No minimum storage duration**

---

## 🏗️ Architecture

AVL Storage follows the **Arxis philosophy** - solid as a fortress, fast as an engine:

```
┌─────────────────────────────────────────┐
│       AVL Storage - Object Citadel      │
├─────────────────────────────────────────┤
│  🏛️ Object Layer                        │
│     - S3-compatible API                  │
│     - Multipart uploads                  │
│     - Versioning & metadata             │
├─────────────────────────────────────────┤
│  ⚙️ Compression Engine                  │
│     - avila-compress (LZ4/Zstd)         │
│     - Content-type detection            │
│     - Smart tier selection              │
├─────────────────────────────────────────┤
│  🛡️ Storage Backend                     │
│     - Local filesystem (dev)            │
│     - Distributed storage (prod)        │
│     - Replication (3 copies)            │
├─────────────────────────────────────────┤
│  🚀 Transfer Engine                     │
│     - Parallel uploads/downloads        │
│     - Resumable transfers               │
│     - CDN integration                   │
└─────────────────────────────────────────┘
```

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
avl-storage = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Using the CLI

```bash
# Install AVL CLI
curl -sSL https://avila.cloud/install.sh | sh

# Configure credentials
avl storage configure

# Upload file
avl storage put my-bucket/file.txt local-file.txt

# Download file
avl storage get my-bucket/file.txt downloaded.txt

# List objects
avl storage ls my-bucket/
```

### S3 Compatibility

```bash
# Use with s3cmd
s3cmd --host=storage.avila.cloud --host-bucket='%(bucket)s.storage.avila.cloud' \
      put file.txt s3://my-bucket/

# Use with AWS CLI
aws s3 cp file.txt s3://my-bucket/ --endpoint-url=https://storage.avila.cloud
```

---

## 🚀 Quick Start

### Basic Operations

```rust
use avl_storage::{StorageClient, PutObjectRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to AVL Storage
    let client = StorageClient::connect("https://storage.avila.cloud").await?;

    // Create bucket
    client.create_bucket("my-bucket").await?;

    // Upload object
    let data = b"Hello from AVL Storage!";
    client.put_object(PutObjectRequest {
        bucket: "my-bucket".to_string(),
        key: "hello.txt".to_string(),
        body: data.to_vec(),
        content_type: Some("text/plain".to_string()),
        metadata: Default::default(),
    }).await?;

    // Download object
    let obj = client.get_object("my-bucket", "hello.txt").await?;
    println!("Content: {}", String::from_utf8(obj.body)?);

    // List objects
    let objects = client.list_objects("my-bucket", None).await?;
    for obj in objects {
        println!("- {} ({} bytes)", obj.key, obj.size);
    }

    // Delete object
    client.delete_object("my-bucket", "hello.txt").await?;

    Ok(())
}
```

### Multipart Uploads (Large Files)

```rust
use avl_storage::{StorageClient, MultipartUpload};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = StorageClient::connect("https://storage.avila.cloud").await?;

    // Initiate multipart upload
    let upload = client.create_multipart_upload(
        "my-bucket",
        "large-file.bin"
    ).await?;

    // Upload parts (5 MB chunks)
    let mut file = File::open("large-local-file.bin").await?;
    let chunk_size = 5 * 1024 * 1024; // 5 MB
    let mut part_number = 1;
    let mut parts = Vec::new();

    loop {
        let mut buffer = vec![0u8; chunk_size];
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        buffer.truncate(n);

        let etag = client.upload_part(
            "my-bucket",
            "large-file.bin",
            &upload.upload_id,
            part_number,
            buffer,
        ).await?;

        parts.push((part_number, etag));
        part_number += 1;
    }

    // Complete upload
    client.complete_multipart_upload(
        "my-bucket",
        "large-file.bin",
        &upload.upload_id,
        parts,
    ).await?;

    println!("Upload complete!");
    Ok(())
}
```

### Automatic Compression

```rust
use avl_storage::{StorageClient, PutObjectRequest, StorageClass};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = StorageClient::connect("https://storage.avila.cloud").await?;

    // Hot data (frequent access) - LZ4 compression
    client.put_object(PutObjectRequest {
        bucket: "my-bucket".to_string(),
        key: "hot-data.json".to_string(),
        body: large_json_data,
        storage_class: Some(StorageClass::Standard), // LZ4
        ..Default::default()
    }).await?;

    // Cold data (archival) - Zstandard compression
    client.put_object(PutObjectRequest {
        bucket: "my-bucket".to_string(),
        key: "archive.tar".to_string(),
        body: archive_data,
        storage_class: Some(StorageClass::Archive), // Zstd
        ..Default::default()
    }).await?;

    // Transparent decompression on GET
    let obj = client.get_object("my-bucket", "hot-data.json").await?;
    // obj.body is automatically decompressed!

    Ok(())
}
```

---

## 📊 Use Cases

### 🎮 **Game Assets & Media**
- Game textures, models, sounds
- Video streaming (HLS/DASH)
- Player-generated content
- Patch distribution
- **3-8ms latency** for Brazilian players

### 🤖 **ML Model Storage**
- Trained models and checkpoints
- Training datasets
- Inference artifacts
- Model versioning
- **Automatic compression** saves 50-70% space

### 📸 **Image & Video Processing**
- User uploads (photos, videos)
- Thumbnails and transcoded versions
- CDN origin storage
- Backup and archival
- **Multipart uploads** for large files

### 📊 **Data Lake & Analytics**
- Raw data ingestion
- Parquet/ORC files
- Log aggregation
- Time-series data
- **S3-compatible** with Spark/Presto

### 🔬 **Scientific Data**
- LIGO/LISA gravitational wave data
- Telescope observations
- Simulation results
- Dataset sharing
- **High-throughput** parallel uploads

---

## 🆚 Comparison with Competitors

| Feature                    | AVL Storage     | AWS S3              | Azure Blob          |
| -------------------------- | --------------- | ------------------- | ------------------- |
| **Brazil latency**         | **3-8ms** ✅     | 50-80ms             | 40-60ms             |
| **Storage (GB/month)**     | **R$ 0,15** ✅   | USD 0.023 (~R$0,12) | USD 0.018 (~R$0,09) |
| **Transfer (within BR)**   | **R$ 0,05** ✅   | USD 0.09 (~R$0,45)  | USD 0.08 (~R$0,40)  |
| **Compression**            | **Automatic** ✅ | Manual              | Manual              |
| **Egress within services** | **FREE** ✅      | Paid                | Paid                |
| **S3 API compatibility**   | **100%** ✅      | Native              | Via adapter         |
| **Multipart uploads**      | ✅               | ✅                   | ✅                   |
| **Versioning**             | ✅               | ✅                   | ✅                   |

**AVL Storage is 50% cheaper** for Brazilian workloads! 🇧🇷

---

## 🛠️ Best Practices

### Naming Buckets
```rust
// ✅ GOOD: Descriptive, DNS-compatible
"my-app-uploads"
"prod-ml-models"
"user-avatars-2024"

// ❌ BAD: Ambiguous, special characters
"bucket1"
"my_bucket" // underscores not recommended
"UPPERCASE" // use lowercase
```

### Object Keys
```rust
// ✅ GOOD: Hierarchical, organized
"users/user123/profile.jpg"
"models/v2/checkpoint-1000.pt"
"logs/2024/11/23/app.log"

// ❌ BAD: Flat, no structure
"file1.jpg"
"data.bin"
```

### Compression
```rust
// ✅ GOOD: Let AVL Storage compress
client.put_object(PutObjectRequest {
    body: uncompressed_data,
    // AVL Storage compresses automatically
    ..Default::default()
}).await?;

// ❌ BAD: Pre-compress yourself
// let compressed = manual_compress(data); // Redundant!
```

### Large Files
```rust
// ✅ GOOD: Use multipart for files > 100 MB
if file_size > 100 * 1024 * 1024 {
    upload_multipart(&client, bucket, key, file_path).await?;
} else {
    upload_single(&client, bucket, key, file_path).await?;
}
```

---

## 🧪 Development Tools

### CLI
```bash
# Configure
avl storage configure
# Access Key: your-key
# Secret Key: your-secret
# Region: sa-east-1 (São Paulo)

# Create bucket
avl storage mb s3://my-bucket

# Upload
avl storage put my-bucket/file.txt local-file.txt

# Upload directory (recursive)
avl storage sync ./local-dir/ s3://my-bucket/remote-dir/

# Download
avl storage get my-bucket/file.txt downloaded.txt

# List
avl storage ls s3://my-bucket/

# Delete
avl storage rm s3://my-bucket/file.txt

# Get object metadata
avl storage info s3://my-bucket/file.txt
```

### Local Emulator (Development)
```bash
# Run locally (no cloud costs!)
docker run -p 9000:9000 avilacloud/avl-storage-emulator:latest

# Update endpoint
export AVL_STORAGE_ENDPOINT=http://localhost:9000
```

---

## 📚 Documentation

- **[Official Docs](https://docs.avila.cloud/storage)** - Complete guide
- **[API Reference](https://docs.rs/avl-storage)** - Rust API docs
- **[S3 Compatibility](https://docs.avila.cloud/storage/s3-compat)** - S3 API mapping
- **[Examples](./examples)** - Code samples

---

## 🏛️ Philosophy - The Storage Citadel

AVL Storage embodies the **Arxis philosophy**:

### 🏛️ **ARX - Fortress**
- **Durability**: 11 nines (99.999999999%) with 3-copy replication
- **Protection**: Encryption, versioning, immutability
- **Reliability**: Geographic redundancy, automatic healing

### ⚙️ **AXIS - Engine**
- **Speed**: 3-8ms latency in Brazil, parallel transfers
- **Efficiency**: Automatic compression saves 50-70% space
- **Scalability**: Unlimited storage, elastic throughput

### 🇧🇷 **Built for Brazil**
- **Local presence**: Data centers in São Paulo, Rio
- **Low latency**: 3-8ms for Brazilian users
- **Fair pricing**: R$ instead of USD, 50% cheaper
- **Portuguese-first**: Docs, support, community

---

## 🤝 Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/awesome-feature`)
3. Commit your changes (`git commit -m 'Add awesome feature'`)
4. Push to the branch (`git push origin feature/awesome-feature`)
5. Open a Pull Request

---

## 📞 Support

**Email**: nicolas@avila.inc
**WhatsApp**: +55 17 99781-1471
**GitHub**: https://github.com/avilaops/arxis
**Docs**: https://docs.avila.inc/storage

---

## 📜 License

Dual-licensed under MIT OR Apache-2.0 - See [LICENSE-MIT](../LICENSE-MIT) and [LICENSE-APACHE](../LICENSE-APACHE) for details.

---

## 🏛️ Built by Avila

**AVL Storage** - *The Storage Fortress*
Part of the **AVL Cloud Platform**

🏛️ **Durable as a fortress**
⚙️ **Fast as an engine**
🇧🇷 **Built for Brazil**

Built with ❤️ in Rust for the Brazilian and LATAM tech community.
