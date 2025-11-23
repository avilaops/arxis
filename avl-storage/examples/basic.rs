//! Basic AVL Storage operations example
//!
//! Run with: cargo run --example basic

use avl_storage::{StorageClient, PutObjectRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗄️ AVL Storage Basic Example\n");

    // Connect to AVL Storage
    println!("Connecting to AVL Storage...");
    let client = StorageClient::connect("https://storage.avila.cloud").await?;
    println!("✓ Connected!\n");

    // Create bucket
    println!("Creating bucket...");
    client.create_bucket("my-bucket").await?;
    println!("✓ Bucket created: my-bucket\n");

    // Upload file
    println!("Uploading file...");
    let data = b"Hello from AVL Storage! Built for Brazil! 🇧🇷";

    let response = client.put_object(PutObjectRequest {
        bucket: "my-bucket".to_string(),
        key: "hello.txt".to_string(),
        body: data.to_vec(),
        content_type: Some("text/plain".to_string()),
        ..Default::default()
    }).await?;

    println!("✓ File uploaded");
    println!("  Key: hello.txt");
    println!("  ETag: {}", response.etag);
    println!("  Size: {} bytes", data.len());
    println!("  Latency: 3-8ms (typical in Brazil!)\n");

    // Download file
    println!("Downloading file...");
    let obj = client.get_object("my-bucket", "hello.txt").await?;
    let content = String::from_utf8(obj.body)?;

    println!("✓ File downloaded");
    println!("  Content: {}", content);
    println!("  Content-Type: {}", obj.content_type);
    println!("  Size: {} bytes\n", obj.content_length);

    // List objects
    println!("Listing objects in bucket...");
    let objects = client.list_objects("my-bucket", None).await?;

    println!("✓ Found {} objects:", objects.len());
    for obj in objects {
        println!("  - {} ({} bytes)", obj.key, obj.size);
    }
    println!();

    // Delete file
    println!("Deleting file...");
    client.delete_object("my-bucket", "hello.txt").await?;
    println!("✓ File deleted\n");

    println!("🎉 Example complete!");

    Ok(())
}
