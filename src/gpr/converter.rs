use crate::models::{ConversionConfig, GprFile, OutputFormat};
use crate::gpr::ffi::*;
use anyhow::{anyhow, Context, Result};
use image::{ImageBuffer, Rgb};
use std::path::PathBuf;
use std::ptr;

pub struct GprConverter;

impl GprConverter {
    /// Convert a GPR file to the specified output format using the official GoPro GPR library
    pub fn convert(gpr_file: &GprFile, config: &ConversionConfig) -> Result<PathBuf> {
        log::info!("Starting conversion of {} using GoPro GPR library", gpr_file.filename);

        // Determine output path
        let output_path = Self::determine_output_path(gpr_file, config)?;

        // Read GPR file into memory
        log::debug!("Reading GPR file: {}", gpr_file.path.display());
        let gpr_data = std::fs::read(&gpr_file.path)
            .with_context(|| format!("Failed to read GPR file: {}", gpr_file.path.display()))?;

        log::debug!("GPR file size: {} bytes", gpr_data.len());

        // Create allocator
        let allocator = create_allocator();

        // Create GPR buffer
        let mut inp_buffer = gpr_buffer {
            buffer: gpr_data.as_ptr() as *mut std::os::raw::c_void,
            size: gpr_data.len(),
        };

        // Parse metadata
        log::debug!("Parsing GPR metadata...");
        let mut parameters: gpr_parameters = unsafe { std::mem::zeroed() };
        unsafe {
            gpr_parameters_set_defaults(&mut parameters);
        }

        let parse_result = unsafe {
            gpr_parse_metadata(&allocator, &mut inp_buffer, &mut parameters)
        };

        if !parse_result {
            return Err(anyhow!("Failed to parse GPR metadata"));
        }

        log::info!(
            "Parsed metadata: {}x{} pixels",
            parameters.input_width,
            parameters.input_height
        );

        // Convert GPR to RGB
        log::debug!("Converting GPR to RGB...");
        let mut out_rgb_buffer = gpr_rgb_buffer {
            buffer: ptr::null_mut(),
            size: 0,
            width: 0,
            height: 0,
        };

        let rgb_result = unsafe {
            gpr_convert_gpr_to_rgb(
                &allocator,
                GPR_RGB_RESOLUTION::GPR_RGB_RESOLUTION_FULL,
                8, // 8-bit per channel
                &mut inp_buffer,
                &mut out_rgb_buffer,
            )
        };

        if !rgb_result || out_rgb_buffer.buffer.is_null() {
            return Err(anyhow!("Failed to convert GPR to RGB"));
        }

        log::info!(
            "RGB conversion successful - buffer: {} bytes, dimensions: {}x{} (metadata was: {}x{})",
            out_rgb_buffer.size,
            out_rgb_buffer.width,
            out_rgb_buffer.height,
            parameters.input_width,
            parameters.input_height
        );

        // Use the actual dimensions from the RGB buffer, not the metadata
        let width = out_rgb_buffer.width as u32;
        let height = out_rgb_buffer.height as u32;
        let rgb_image = Self::rgb_buffer_to_image(&out_rgb_buffer, width, height)?;

        // Free RGB buffer
        if let Some(free_fn) = allocator.mem_free {
            free_fn(out_rgb_buffer.buffer);
        }

        // Save to output format
        log::info!(
            "Encoding to {} (quality: {})...",
            config.output_format.as_str(),
            config.quality_display()
        );
        Self::save_image(&rgb_image, &output_path, config)?;

        log::info!("Conversion complete: {}", output_path.display());
        Ok(output_path)
    }

    /// Convert GPR RGB buffer to ImageBuffer
    fn rgb_buffer_to_image(
        rgb_buffer: &gpr_rgb_buffer,
        width: u32,
        height: u32,
    ) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        // The buffer size tells us how much data we actually have
        let actual_size = rgb_buffer.size;

        log::debug!(
            "RGB buffer info - size: {} bytes, expected for {}x{}: {} bytes",
            actual_size,
            width,
            height,
            width * height * 3
        );

        // Determine actual dimensions from buffer size
        // Buffer could be RGB (3 bytes) or RGBA (4 bytes) per pixel
        let bytes_per_pixel = if actual_size == (width * height * 4) as usize {
            log::info!("Detected RGBA format (4 bytes per pixel)");
            4
        } else if actual_size == (width * height * 3) as usize {
            log::info!("Detected RGB format (3 bytes per pixel)");
            3
        } else {
            // Try to infer dimensions from actual buffer size
            log::warn!("Buffer size doesn't match expected dimensions, inferring from buffer");

            // Assume RGB format and calculate actual dimensions
            let total_pixels = actual_size / 3;
            let inferred_width = (total_pixels as f64).sqrt() as u32;
            let inferred_height = total_pixels as u32 / inferred_width;

            log::info!(
                "Inferred dimensions: {}x{} (from {} pixels)",
                inferred_width,
                inferred_height,
                total_pixels
            );

            // Update dimensions to match actual data
            return Self::rgb_buffer_to_image_with_size(rgb_buffer, inferred_width, inferred_height, 3);
        };

        Self::rgb_buffer_to_image_with_size(rgb_buffer, width, height, bytes_per_pixel)
    }

    fn rgb_buffer_to_image_with_size(
        rgb_buffer: &gpr_rgb_buffer,
        width: u32,
        height: u32,
        bytes_per_pixel: usize,
    ) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        let data_size = (width * height * bytes_per_pixel as u32) as usize;

        if rgb_buffer.size < data_size {
            return Err(anyhow!(
                "RGB buffer too small: need {} bytes, got {}",
                data_size,
                rgb_buffer.size
            ));
        }

        // Copy RGB data from C buffer to Rust Vec
        let rgb_data = unsafe {
            std::slice::from_raw_parts(rgb_buffer.buffer as *const u8, data_size)
        };

        let mut img_buffer = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * bytes_per_pixel as u32) as usize;
                let r = rgb_data[idx];
                let g = rgb_data[idx + 1];
                let b = rgb_data[idx + 2];
                // Skip alpha channel if present (idx + 3)
                img_buffer.put_pixel(x, y, Rgb([r, g, b]));
            }
        }

        Ok(img_buffer)
    }

    /// Save image to file
    fn save_image(
        image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        path: &PathBuf,
        config: &ConversionConfig,
    ) -> Result<()> {
        match config.output_format {
            OutputFormat::Jpeg => {
                let file = std::fs::File::create(path)
                    .with_context(|| format!("Failed to create output file: {}", path.display()))?;

                let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(
                    file,
                    config.quality,
                );

                encoder
                    .encode(
                        image.as_raw(),
                        image.width(),
                        image.height(),
                        image::ExtendedColorType::Rgb8,
                    )
                    .context("Failed to encode JPEG")?;
            }
            OutputFormat::Png => {
                image
                    .save(path)
                    .with_context(|| format!("Failed to save PNG: {}", path.display()))?;
            }
        }
        Ok(())
    }

    /// Convert multiple GPR files in batch
    #[allow(dead_code)]
    pub fn batch_convert(
        files: &[GprFile],
        config: &ConversionConfig,
        progress_callback: Option<Box<dyn Fn(usize, usize)>>,
    ) -> Result<Vec<PathBuf>> {
        let mut output_paths = Vec::new();
        let mut errors = Vec::new();

        for (i, file) in files.iter().enumerate() {
            if let Some(ref callback) = progress_callback {
                callback(i, files.len());
            }

            match Self::convert(file, config) {
                Ok(path) => {
                    log::info!("Successfully converted: {}", file.filename);
                    output_paths.push(path);
                }
                Err(e) => {
                    log::error!("Failed to convert {}: {}", file.filename, e);
                    errors.push((file.filename.clone(), e));
                }
            }
        }

        if let Some(ref callback) = progress_callback {
            callback(files.len(), files.len());
        }

        if !errors.is_empty() {
            log::warn!("Batch conversion completed with {} errors", errors.len());
        }

        Ok(output_paths)
    }

    fn determine_output_path(
        gpr_file: &GprFile,
        config: &ConversionConfig,
    ) -> Result<PathBuf> {
        let output_dir = if let Some(ref dir) = config.output_directory {
            PathBuf::from(dir)
        } else {
            gpr_file
                .path
                .parent()
                .ok_or_else(|| anyhow!("Could not determine parent directory"))?
                .to_path_buf()
        };

        let stem = gpr_file
            .path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Could not extract filename stem"))?;

        let extension = match config.output_format {
            OutputFormat::Jpeg => "jpg",
            OutputFormat::Png => "png",
        };

        let filename = format!("{}.{}", stem, extension);

        Ok(output_dir.join(filename))
    }
}
