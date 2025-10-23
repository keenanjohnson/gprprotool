use crate::models::{ConversionConfig, GprFile, OutputFormat};
use crate::gpr;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    MainMenu,
    FileBrowser,
    FileInfo,
    ConversionConfig,
    Converting,
    Complete,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuItem {
    BrowseFiles,
    BatchConvert,
    Settings,
    Help,
    Quit,
}

impl MainMenuItem {
    pub fn as_str(&self) -> &str {
        match self {
            MainMenuItem::BrowseFiles => "Browse and Convert Files",
            MainMenuItem::BatchConvert => "Batch Convert Directory",
            MainMenuItem::Settings => "Settings",
            MainMenuItem::Help => "Help",
            MainMenuItem::Quit => "Quit",
        }
    }

    pub fn all() -> Vec<MainMenuItem> {
        vec![
            MainMenuItem::BrowseFiles,
            MainMenuItem::BatchConvert,
            MainMenuItem::Settings,
            MainMenuItem::Help,
            MainMenuItem::Quit,
        ]
    }
}

pub struct App {
    pub state: AppState,
    pub main_menu_index: usize,
    pub current_directory: PathBuf,
    pub files: Vec<PathBuf>,
    pub file_index: usize,
    pub selected_file: Option<GprFile>,
    pub conversion_config: ConversionConfig,
    pub config_option_index: usize,
    pub conversion_progress: f32,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let current_directory = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        Self {
            state: AppState::MainMenu,
            main_menu_index: 0,
            current_directory,
            files: Vec::new(),
            file_index: 0,
            selected_file: None,
            conversion_config: ConversionConfig::default(),
            config_option_index: 0,
            conversion_progress: 0.0,
            error_message: None,
            success_message: None,
        }
    }

    // Main menu navigation
    pub fn next_menu_item(&mut self) {
        let items = MainMenuItem::all();
        self.main_menu_index = (self.main_menu_index + 1) % items.len();
    }

    pub fn previous_menu_item(&mut self) {
        let items = MainMenuItem::all();
        self.main_menu_index = if self.main_menu_index == 0 {
            items.len() - 1
        } else {
            self.main_menu_index - 1
        };
    }

    pub fn select_menu_item(&mut self) {
        let items = MainMenuItem::all();
        match items.get(self.main_menu_index) {
            Some(MainMenuItem::BrowseFiles) => {
                self.load_directory();
                self.state = AppState::FileBrowser;
            }
            Some(MainMenuItem::BatchConvert) => {
                // TODO: Implement batch convert
                self.error_message = Some("Batch convert not yet implemented".to_string());
                self.state = AppState::Error;
            }
            Some(MainMenuItem::Settings) => {
                // TODO: Implement settings
                self.error_message = Some("Settings not yet implemented".to_string());
                self.state = AppState::Error;
            }
            Some(MainMenuItem::Help) => {
                // TODO: Implement help
                self.error_message = Some("Help screen not yet implemented".to_string());
                self.state = AppState::Error;
            }
            Some(MainMenuItem::Quit) => {
                // This will be handled by the main loop
            }
            None => {}
        }
    }

    // File browser
    pub fn load_directory(&mut self) {
        use std::fs;

        self.files.clear();

        if let Ok(entries) = fs::read_dir(&self.current_directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    self.files.push(path);
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext.eq_ignore_ascii_case("gpr") {
                        self.files.push(path);
                    }
                }
            }
        }

        self.files.sort();
        self.file_index = 0;
    }

    pub fn next_file(&mut self) {
        if !self.files.is_empty() {
            self.file_index = (self.file_index + 1) % self.files.len();
        }
    }

    pub fn previous_file(&mut self) {
        if !self.files.is_empty() {
            self.file_index = if self.file_index == 0 {
                self.files.len() - 1
            } else {
                self.file_index - 1
            };
        }
    }

    pub fn select_file(&mut self) {
        if let Some(path) = self.files.get(self.file_index) {
            if path.is_dir() {
                self.current_directory = path.clone();
                self.load_directory();
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext.eq_ignore_ascii_case("gpr") {
                    let mut gpr_file = GprFile::new(path.clone());

                    // Try to load metadata
                    if let Err(e) = self.load_metadata(&mut gpr_file) {
                        log::error!("Failed to load metadata: {}", e);
                    }

                    self.selected_file = Some(gpr_file);
                    self.state = AppState::FileInfo;
                }
            }
        }
    }

    pub fn go_to_parent_directory(&mut self) {
        if let Some(parent) = self.current_directory.parent() {
            self.current_directory = parent.to_path_buf();
            self.load_directory();
        }
    }

    fn load_metadata(&self, gpr_file: &mut GprFile) -> Result<(), String> {
        match gpr::read_metadata(&gpr_file.path) {
            Ok(metadata) => {
                gpr_file.metadata = Some(metadata);
                Ok(())
            }
            Err(e) => Err(format!("Failed to read metadata: {}", e)),
        }
    }

    // File info
    pub fn back_to_file_browser(&mut self) {
        self.selected_file = None;
        self.state = AppState::FileBrowser;
    }

    pub fn go_to_conversion_config(&mut self) {
        self.state = AppState::ConversionConfig;
        self.config_option_index = 0;
    }

    // Conversion config
    pub fn back_to_file_info(&mut self) {
        self.state = AppState::FileInfo;
    }

    pub fn next_config_option(&mut self) {
        self.config_option_index = (self.config_option_index + 1) % 4;
    }

    pub fn previous_config_option(&mut self) {
        self.config_option_index = if self.config_option_index == 0 {
            3
        } else {
            self.config_option_index - 1
        };
    }

    pub fn adjust_config_option(&mut self, delta: i32) {
        match self.config_option_index {
            0 => {
                // Toggle output format
                self.conversion_config.output_format = match self.conversion_config.output_format {
                    OutputFormat::Jpeg => OutputFormat::Png,
                    OutputFormat::Png => OutputFormat::Jpeg,
                };
            }
            1 => {
                // Adjust quality (only for JPEG)
                if self.conversion_config.output_format == OutputFormat::Jpeg {
                    let new_quality = (self.conversion_config.quality as i32 + delta * 5)
                        .clamp(1, 100) as u8;
                    self.conversion_config.quality = new_quality;
                }
            }
            2 => {
                // Toggle preserve metadata
                self.conversion_config.preserve_metadata = !self.conversion_config.preserve_metadata;
            }
            3 => {
                // Output directory selection (TODO)
            }
            _ => {}
        }
    }

    pub fn start_conversion(&mut self) {
        if let Some(ref gpr_file) = self.selected_file {
            self.state = AppState::Converting;
            self.conversion_progress = 0.0;

            // Perform actual conversion
            match crate::gpr::GprConverter::convert(gpr_file, &self.conversion_config) {
                Ok(output_path) => {
                    self.conversion_progress = 100.0;
                    self.success_message = Some(format!(
                        "Conversion completed successfully!\n\nOutput: {}",
                        output_path.display()
                    ));
                    self.state = AppState::Complete;
                    log::info!("Conversion successful: {}", output_path.display());
                }
                Err(e) => {
                    self.error_message = Some(format!("Conversion failed: {}", e));
                    self.state = AppState::Error;
                    log::error!("Conversion error: {}", e);
                }
            }
        }
    }

    pub fn cancel_conversion(&mut self) {
        self.state = AppState::ConversionConfig;
        self.conversion_progress = 0.0;
    }

    // Navigation
    pub fn back_to_main_menu(&mut self) {
        self.state = AppState::MainMenu;
        self.selected_file = None;
        self.error_message = None;
        self.success_message = None;
        self.conversion_progress = 0.0;
    }
}
