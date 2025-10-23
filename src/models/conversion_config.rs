use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    Jpeg,
    Png,
}

impl OutputFormat {
    pub fn as_str(&self) -> &str {
        match self {
            OutputFormat::Jpeg => "JPEG",
            OutputFormat::Png => "PNG",
        }
    }

    #[allow(dead_code)]
    pub fn extension(&self) -> &str {
        match self {
            OutputFormat::Jpeg => "jpg",
            OutputFormat::Png => "png",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionConfig {
    pub output_format: OutputFormat,
    pub quality: u8, // 1-100 for JPEG, ignored for PNG
    pub output_directory: Option<String>,
    pub preserve_metadata: bool,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::Jpeg,
            quality: 95,
            output_directory: None,
            preserve_metadata: true,
        }
    }
}

impl ConversionConfig {
    pub fn quality_display(&self) -> String {
        match self.output_format {
            OutputFormat::Jpeg => format!("{}%", self.quality),
            OutputFormat::Png => "N/A".to_string(),
        }
    }
}
