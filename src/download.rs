use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::Command;
use tokio::fs::File as AsyncFile;
use tokio::io::AsyncWriteExt;

// Download a file from a URL to a destination path
pub async fn download_file(
    client: &Client,
    url: &str,
    dest_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get file name for display
    let file_name = dest_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file");

    // Send request
    let mut response = client.get(url).send().await?;
    let total_size = response.content_length().unwrap_or(0);

    // Create progress bar
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message(format!("Downloading {}", file_name));

    // Download with progress
    let mut file = AsyncFile::create(dest_path).await?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message(format!("Downloaded {}", file_name));

    Ok(())
}

// Verify file checksum using SHA256
pub fn verify_checksum(
    file_path: &Path,
    expected_shasum: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use sha2::Digest;

    let mut file = File::open(file_path)?;
    let mut hasher = sha2::Sha256::new();
    copy(&mut file, &mut hasher)?;

    let actual_hash = hasher.finalize();
    let actual_shasum = format!("{:x}", actual_hash);

    if actual_shasum != expected_shasum {
        return Err(format!(
            "Checksum mismatch: expected {}, got {}",
            expected_shasum, actual_shasum
        )
        .into());
    }

    Ok(())
}

// Extract a tar.xz archive to a destination directory
pub fn extract_tarball(
    archive_path: &Path,
    dest_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Use tar command to extract
    let status = Command::new("tar")
        .args([
            "-xJf",
            archive_path.to_str().unwrap(),
            "-C",
            dest_dir.to_str().unwrap(),
            "--strip-components=1",
        ])
        .status()?;

    if !status.success() {
        return Err("Failed to extract tarball".into());
    }

    Ok(())
}

// Extract a zip archive to a destination directory
pub fn extract_zip(archive_path: &Path, dest_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Use unzip command to extract
    let status = Command::new("unzip")
        .args([
            "-o",
            archive_path.to_str().unwrap(),
            "-d",
            dest_dir.to_str().unwrap(),
        ])
        .status()?;

    if !status.success() {
        return Err("Failed to extract zip archive".into());
    }

    Ok(())
}
