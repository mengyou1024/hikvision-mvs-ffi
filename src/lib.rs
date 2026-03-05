//! 海康威视 MVS (Machine Vision Software SDK) Rust的FFI绑定
//!
//! 这个crate提供了Rust FFI绑定到海康威视MVS SDK,
//! 允许Rust应用程序与海康威视相机接口.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod pixel_type;

pub mod error_define;

pub mod isp_error_define;

#[deprecated]
pub mod obsolete_cam_params;

#[deprecated]
pub mod obsolete_interface;

pub mod camera_params;

pub mod camera_control;

// Re-export commonly used types
pub use pixel_type::*;

pub use error_define::*;

pub use isp_error_define::*;

#[allow(deprecated)]
pub use obsolete_cam_params::*;

#[allow(deprecated)]
pub use obsolete_interface::*;

pub use camera_params::*;

pub use camera_control::*;
