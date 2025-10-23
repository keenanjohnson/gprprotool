use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GprFile {
    pub path: PathBuf,
    pub filename: String,
    pub size: u64,
    pub metadata: Option<GprMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GprMetadata {
    pub camera_model: String,
    pub width: u32,
    pub height: u32,
    pub iso: Option<u32>,
    pub exposure_time: Option<String>,
    pub f_number: Option<String>,
    pub focal_length: Option<String>,
    pub date_taken: Option<String>,
    pub gps_latitude: Option<f64>,
    pub gps_longitude: Option<f64>,
}

impl GprFile {
    pub fn new(path: PathBuf) -> Self {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let size = std::fs::metadata(&path)
            .map(|m| m.len())
            .unwrap_or(0);

        Self {
            path,
            filename,
            size,
            metadata: None,
        }
    }

    pub fn format_size(&self) -> String {
        if self.size < 1024 {
            format!("{} B", self.size)
        } else if self.size < 1024 * 1024 {
            format!("{:.2} KB", self.size as f64 / 1024.0)
        } else if self.size < 1024 * 1024 * 1024 {
            format!("{:.2} MB", self.size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", self.size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}
