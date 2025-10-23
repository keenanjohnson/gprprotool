use crate::models::gpr_file::GprMetadata;
use anyhow::{Context, Result};
use std::path::Path;

/// Read metadata from a GPR file using EXIF data
///
/// GPR files are based on Adobe DNG format with VC-5 compression.
/// This function reads EXIF metadata from the GPR/DNG container.
pub fn read_metadata(path: &Path) -> Result<GprMetadata> {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open(path)
        .with_context(|| format!("Failed to open file for EXIF reading: {}", path.display()))?;
    let mut reader = BufReader::new(file);

    let exif_reader = exif::Reader::new();
    let exif_data = exif_reader
        .read_from_container(&mut reader)
        .context("Failed to read EXIF data from GPR file")?;

    // Extract camera make and model
    let make = exif_data
        .get_field(exif::Tag::Make, exif::In::PRIMARY)
        .map(|f| f.display_value().to_string());

    let model = exif_data
        .get_field(exif::Tag::Model, exif::In::PRIMARY)
        .map(|f| f.display_value().to_string());

    let camera_model = match (make, model) {
        (Some(make), Some(model)) => format!("{} {}", make.trim(), model.trim()),
        (None, Some(model)) => model.trim().to_string(),
        (Some(make), None) => make.trim().to_string(),
        (None, None) => "Unknown Camera".to_string(),
    };

    // Get image dimensions
    let width = exif_data
        .get_field(exif::Tag::ImageWidth, exif::In::PRIMARY)
        .and_then(|f| match f.value {
            exif::Value::Short(ref v) if !v.is_empty() => Some(v[0] as u32),
            exif::Value::Long(ref v) if !v.is_empty() => Some(v[0]),
            _ => None,
        })
        .unwrap_or(0);

    let height = exif_data
        .get_field(exif::Tag::ImageLength, exif::In::PRIMARY)
        .and_then(|f| match f.value {
            exif::Value::Short(ref v) if !v.is_empty() => Some(v[0] as u32),
            exif::Value::Long(ref v) if !v.is_empty() => Some(v[0]),
            _ => None,
        })
        .unwrap_or(0);

    // Extract ISO
    let iso = exif_data
        .get_field(exif::Tag::PhotographicSensitivity, exif::In::PRIMARY)
        .or_else(|| exif_data.get_field(exif::Tag::ISOSpeed, exif::In::PRIMARY))
        .and_then(|f| match f.value {
            exif::Value::Short(ref v) if !v.is_empty() => Some(v[0] as u32),
            _ => None,
        });

    // Extract exposure time
    let exposure_time = exif_data
        .get_field(exif::Tag::ExposureTime, exif::In::PRIMARY)
        .map(|f| {
            let display = f.display_value().to_string();
            // Clean up the display value
            display.trim_matches('"').to_string()
        });

    // Extract f-number
    let f_number = exif_data
        .get_field(exif::Tag::FNumber, exif::In::PRIMARY)
        .map(|f| {
            let display = f.display_value().to_string();
            format!("f/{}", display.trim_matches('"'))
        });

    // Extract focal length
    let focal_length = exif_data
        .get_field(exif::Tag::FocalLength, exif::In::PRIMARY)
        .map(|f| {
            let display = f.display_value().to_string();
            format!("{} mm", display.trim_matches('"'))
        });

    // Extract date/time
    let date_taken = exif_data
        .get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)
        .or_else(|| exif_data.get_field(exif::Tag::DateTime, exif::In::PRIMARY))
        .map(|f| {
            let display = f.display_value().to_string();
            display.trim_matches('"').to_string()
        });

    // Extract GPS coordinates
    let gps_latitude = extract_gps_coordinate(&exif_data, exif::Tag::GPSLatitude, exif::Tag::GPSLatitudeRef);
    let gps_longitude = extract_gps_coordinate(&exif_data, exif::Tag::GPSLongitude, exif::Tag::GPSLongitudeRef);

    Ok(GprMetadata {
        camera_model,
        width,
        height,
        iso,
        exposure_time,
        f_number,
        focal_length,
        date_taken,
        gps_latitude,
        gps_longitude,
    })
}

/// Extract GPS coordinate from EXIF data
fn extract_gps_coordinate(
    exif_data: &exif::Exif,
    coord_tag: exif::Tag,
    ref_tag: exif::Tag,
) -> Option<f64> {
    let coord = exif_data.get_field(coord_tag, exif::In::PRIMARY)?;
    let ref_val = exif_data.get_field(ref_tag, exif::In::PRIMARY)?;

    if let exif::Value::Rational(ref v) = coord.value {
        if v.len() >= 3 {
            let degrees = v[0].to_f64();
            let minutes = v[1].to_f64();
            let seconds = v[2].to_f64();

            let mut result = degrees + minutes / 60.0 + seconds / 3600.0;

            // Check reference (N/S for latitude, E/W for longitude)
            if let exif::Value::Ascii(ref ascii) = ref_val.value {
                if !ascii.is_empty() {
                    let ref_str = String::from_utf8_lossy(&ascii[0]);
                    if ref_str == "S" || ref_str == "W" {
                        result = -result;
                    }
                }
            }

            return Some(result);
        }
    }

    None
}
