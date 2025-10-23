// FFI bindings for the GoPro GPR C API

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::{c_int, c_uint, c_void};

// GPR buffer structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_buffer {
    pub buffer: *mut c_void,
    pub size: usize,
}

// GPR RGB buffer structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_rgb_buffer {
    pub buffer: *mut c_void,
    pub size: usize,
    pub width: usize,
    pub height: usize,
}

// GPR allocator structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_allocator {
    pub mem_alloc: Option<extern "C" fn(size: usize) -> *mut c_void>,
    pub mem_free: Option<extern "C" fn(ptr: *mut c_void)>,
}

// RGB resolution enum
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GPR_RGB_RESOLUTION {
    GPR_RGB_RESOLUTION_NONE = 0,
    GPR_RGB_RESOLUTION_EIGHTH = 1,
    GPR_RGB_RESOLUTION_QUARTER = 2,
    GPR_RGB_RESOLUTION_HALF = 3,
    GPR_RGB_RESOLUTION_FULL = 4,
}

// EXIF info structure (simplified)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_exif_info {
    // Add EXIF fields as needed
    pub _placeholder: [u8; 1024],
}

// Profile info structure (simplified)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_profile_info {
    pub _placeholder: [u8; 1024],
}

// Tuning info structure (simplified)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_tuning_info {
    pub _placeholder: [u8; 1024],
}

// Preview image structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_preview_image {
    pub jpg_preview: gpr_buffer,
    pub preview_width: c_uint,
    pub preview_height: c_uint,
}

// GPR parameters structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpr_parameters {
    pub input_width: c_uint,
    pub input_height: c_uint,
    pub input_pitch: c_uint,
    pub fast_encoding: bool,
    pub compute_md5sum: bool,
    pub gpmf_payload: gpr_buffer,
    pub preview_image: gpr_preview_image,
    pub enable_preview: bool,
    pub exif_info: gpr_exif_info,
    pub profile_info: gpr_profile_info,
    pub tuning_info: gpr_tuning_info,
}

extern "C" {
    // Initialize GPR parameters with defaults
    pub fn gpr_parameters_set_defaults(params: *mut gpr_parameters);

    // Parse metadata from GPR/DNG file
    pub fn gpr_parse_metadata(
        allocator: *const gpr_allocator,
        inp_buffer: *mut gpr_buffer,
        parameters: *mut gpr_parameters,
    ) -> bool;

    // Convert GPR to RGB
    pub fn gpr_convert_gpr_to_rgb(
        allocator: *const gpr_allocator,
        rgb_resolution: GPR_RGB_RESOLUTION,
        rgb_bits: c_int,
        inp_gpr_buffer: *mut gpr_buffer,
        out_rgb_buffer: *mut gpr_rgb_buffer,
    ) -> bool;

    // Convert GPR to DNG
    pub fn gpr_convert_gpr_to_dng(
        allocator: *const gpr_allocator,
        parameters: *const gpr_parameters,
        inp_gpr_buffer: *mut gpr_buffer,
        out_dng_buffer: *mut gpr_buffer,
    ) -> bool;
}

// Helper functions for memory allocation
pub extern "C" fn gpr_alloc(size: usize) -> *mut c_void {
    unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(size, 8);
        std::alloc::alloc(layout) as *mut c_void
    }
}

pub extern "C" fn gpr_free(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe {
            std::alloc::dealloc(ptr as *mut u8, std::alloc::Layout::from_size_align_unchecked(1, 8));
        }
    }
}

// Create default allocator
pub fn create_allocator() -> gpr_allocator {
    gpr_allocator {
        mem_alloc: Some(gpr_alloc),
        mem_free: Some(gpr_free),
    }
}
