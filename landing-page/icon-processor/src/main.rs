/// Main CLI application - Simplified version

mod config;
mod generator;
mod processor;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::{Path, PathBuf};
use std::time::Instant;

use config::PlatformConfig;
use generator::{generate_html_snippet, generate_manifest, generate_readme};
use processor::{load_image, process_icon, save_image};

#[derive(Parser)]
#[command(name = "arxis-icons")]
#[command(author = "Nícolas Ávila <nicolas@avila.cloud>")]
#[command(version = "1.0.0")]
#[command(about = "High-performance icon processor with background removal")]
struct Cli {
    /// Input directory with images
    #[arg(short, long, default_value = "./input")]
    input: PathBuf,

    /// Output directory for generated icons
    #[arg(short, long, default_value = "./output")]
    output: PathBuf,

    /// Image quality (1-100)
    #[arg(short, long, default_value = "95")]
    quality: u8,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let start_time = Instant::now();

    println!("\n{}", "═".repeat(60));
    println!("  🎨 ARXIS ICON PROCESSOR - v1.0.0");
    println!("  High-performance Rust-powered icon generation");
    println!("{}\n", "═".repeat(60));

    // Validate input directory
    if !cli.input.exists() {
        anyhow::bail!("Input directory does not exist: {}", cli.input.display());
    }

    // Create output directory
    std::fs::create_dir_all(&cli.output)?;

    // Get all platform configurations
    let platforms = PlatformConfig::all_platforms();

    println!("📂 Configuration");
    println!("  Input:  {}", cli.input.display());
    println!("  Output: {}", cli.output.display());
    println!("  Quality: {}%", cli.quality);

    // Find images
    let images = find_images(&cli.input)?;

    if images.is_empty() {
        println!("\n❌ No images found in input directory!");
        return Ok(());
    }

    println!("\n✅ Found {} image(s) to process\n", images.len());

    // Process each image
    for (idx, image_path) in images.iter().enumerate() {
        process_single_image(
            image_path,
            idx + 1,
            images.len(),
            &cli.output,
            &platforms,
            cli.quality,
        )?;
    }

    let elapsed = start_time.elapsed();

    println!("\n{}", "═".repeat(60));
    println!("  ✅ PROCESSING COMPLETED!");
    println!("{}", "═".repeat(60));
    println!("\n📊 Statistics:");
    println!("  Images processed: {}", images.len());
    println!("  Time elapsed: {:.2}s", elapsed.as_secs_f64());
    println!("  Output directory: {}", cli.output.display());
    println!();

    Ok(())
}

fn process_single_image(
    image_path: &Path,
    current: usize,
    total: usize,
    output_dir: &Path,
    platforms: &[PlatformConfig],
    quality: u8,
) -> Result<()> {
    let file_name = image_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image");

    println!("[{}/{}] Processing: {}", current, total, file_name);

    // Create output folder
    let image_output_dir = output_dir.join(file_name);
    std::fs::create_dir_all(&image_output_dir)?;

    // Load image
    let img = load_image(image_path)?;
    let rgba = img.to_rgba8();

    // Save original
    let no_bg_path = image_output_dir.join(format!("{}_original.png", file_name));
    save_image(&rgba, &no_bg_path, quality)?;

    // Generate icons
    let mut icon_files = Vec::new();

    for platform in platforms {
        for (width, height) in &platform.sizes {
            let icon = process_icon(&rgba, *width, *height, platform.padding_percent)?;
            let filename = format!("icon_{}_{}x{}.png", platform.name, width, height);
            let icon_path = image_output_dir.join(&filename);
            save_image(&icon, &icon_path, quality)?;
            icon_files.push(filename);
        }
    }

    // Generate auxiliary files
    generate_manifest(&image_output_dir, file_name, &icon_files)?;
    generate_html_snippet(&image_output_dir, &icon_files)?;
    generate_readme(&image_output_dir, file_name)?;

    println!("  ✅ Generated {} icons\n", icon_files.len());

    Ok(())
}

fn find_images(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut images = Vec::new();
    let supported_extensions = ["png", "jpg", "jpeg"];

    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if supported_extensions
                        .iter()
                        .any(|&e| e.eq_ignore_ascii_case(&ext.to_string_lossy()))
                    {
                        images.push(path);
                    }
                }
            }
        }
    }

    Ok(images)
}
