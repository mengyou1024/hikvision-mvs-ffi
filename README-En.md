# Hikvision MVS SDK Rust FFI Bindings

[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)](https://developer.microsoft.com/en-us/windows/)
[![SDK Version](https://img.shields.io/badge/MVS%20SDK-4.5.1-orange.svg)](https://www.hikrobotics.com/cn/machinevision/service/download)
[![Rust Edition](https://img.shields.io/badge/rust-edition%202024-lightgrey.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

This project provides Rust FFI bindings for the Hikvision MVS (Machine Vision Software SDK), allowing Rust applications to directly interact with Hikvision industrial cameras.

## ⚠️ Important Notes

- **Platform Limitation**: This project **only supports Windows platform**, does not support Linux or macOS
- **SDK Dependency**: Must install **Hikvision MVS SDK 4.5.1** runtime environment
- **Download Link**: [MVS SDK Download](https://www.hikrobotics.com/cn/machinevision/service/download)

## 📋 Environment Requirements

### System Requirements
- **Operating System**: Windows 10/11
- **Rust Toolchain**: Rust version supporting edition 2024
- **MVS SDK**: Hikvision MVS SDK 4.5.1 runtime environment (Required)

### Installation Steps

1. **Install MVS SDK**
   ```bash
   # Visit the following link to download and install MVS SDK 4.5.1
   https://www.hikrobotics.com/cn/machinevision/service/download
   ```

2. **Verify Installation**
   - Ensure SDK is correctly installed to system directory
   - Check if environment variables include SDK path

3. **Build Project**
   ```bash
   cargo build
   ```

4. **Run Examples**
   
   ```bash
   # Device enumeration example
   cargo run --example enum_device
   
   # Image capture example
   cargo run --example grab_image
   ```

## 📚 Documentation

For detailed API documentation, please refer to:
- **Official Development Guide**: [`Docs/Industrial Camera Windows SDK Development Guide V4.5.1(C).chm`](./Docs/工业相机Windows SDK开发指南V4.5.1（C）.chm)

The CHM document contains:
- SDK initialization process
- Device enumeration and connection
- Image acquisition and control
- Parameter configuration interfaces
- Error code explanation
- Complete API reference

## 🏗️ Project Structure

```
hikvision-mvs-ffi/
├── Includes/                   # C language header files (Original SDK)
│   ├── CameraParams.h          # Camera parameter definitions
│   ├── MvCameraControl.h       # Camera control interfaces
│   ├── MvErrorDefine.h         # Error code definitions
│   ├── MvISPErrorDefine.h      # ISP error code definitions
│   ├── MvObsoleteInterfaces.h  # Obsolete interfaces
│   ├── ObsoleteCamParams.h     # Obsolete parameters
│   └── PixelType.h             # Pixel type definitions
├── src/
│   ├── lib.rs                  # Library entry point
│   ├── camera_control.rs       # Camera control interface FFI bindings
│   ├── camera_params.rs        # Camera parameter structure FFI bindings
│   ├── error_define.rs         # Error code constant FFI bindings
│   ├── isp_error_define.rs     # ISP error code FFI bindings
│   ├── obsolete_cam_params.rs  # Obsolete parameter FFI bindings
│   ├── obsolete_interface.rs   # Obsolete interface FFI bindings
│   ├── pixel_type.rs           # Pixel type FFI bindings
│   └── tests.rs                # Test code
├── examples/                   # Example code
│   ├── enum_device.rs          # Device enumeration example
│   └── grab_image.rs           # Image capture example
├── Docs/                       # Official documentation
│   └── Industrial Camera Windows SDK Development Guide V4.5.1(C).chm
├── Libraries/                  # SDK library files
├── Cargo.toml                  # Project configuration
└── build.rs                    # Build script
```

## ✨ Main Features

1. **camera_params.rs** - Camera Parameter Module
   - Device information structure 
   - Image information structure 
   - Property configuration structure
   - Related constant definitions

2. **error_define.rs** - Error Code Module
   - General error codes
   - GenICam series error codes
   - GigE/USB device error codes
   - Upgrade-related error codes

3. **pixel_type.rs** - Pixel Type Module
   - Monochrome formats (Mono8, Mono10, Mono12, Mono16)
   - Bayer formats (BayerGR8, BayerRG10, BayerGB12)
   - RGB/YUV Packed formats
   - 3D coordinate formats

4. **camera_control.rs** - Camera Control Interface Module
   - SDK initialization/de-initialization
   - Device enumeration and control
   - Image acquisition
   - Frame grabber control
   - Property configuration

## 🔧 Usage Examples

### Device Enumeration

```rust
use hikvision_mvs_ffi::*;

fn main() {
    unsafe {
        // Initialize SDK
        MV_CC_Initialize();
        
        // Enumerate GigE devices
        let mut device_list = MV_CC_DEVICE_INFO_LIST::default();
        MV_CC_EnumDevices(MV_GIGE_DEVICE, &mut device_list);
        
        println!("Found {} devices", device_list.nDeviceNum);
        
        // Cleanup
        MV_CC_Finalize();
    }
}
```

### Image Capture

```rust
use hikvision_mvs_ffi::*;

fn main() {
    unsafe {
        MV_CC_Initialize();
        
        // Create device handle, open device...
        let mut handle: *mut c_void = std::ptr::null_mut();
        // MV_CC_CreateHandle(&mut handle, &device_info);
        // MV_CC_OpenDevice(handle, MV_ACCESS_Exclusive, 0);
        
        // Start acquisition
        MV_CC_StartGrabbing(handle);
        
        // Get image
        let mut frame = MV_FRAME_OUT::default();
        MV_CC_GetImageBuffer(handle, &mut frame, 1000);
        
        // Process image data...
        
        // Free buffer
        MV_CC_FreeImageBuffer(handle, &frame);
        
        // Stop acquisition
        MV_CC_StopGrabbing(handle);
        
        // Close device
        MV_CC_CloseDevice(handle);
        MV_CC_DestroyHandle(handle);
        
        MV_CC_Finalize();
    }
}
```

For more examples, please check the [`examples/`](examples) directory.

### Important Notes

1. **Safety**: All FFI calls are `unsafe` and require careful handling of pointers and memory
2. **Thread Safety**: Some SDK interfaces are not thread-safe, please refer to official documentation
3. **Resource Management**: Ensure proper release of memory and resources allocated by SDK
4. **Error Handling**: Check return values of all API calls and handle error codes properly

## 📖 References

- [Hikvision Robotics Official Website](https://www.hikrobotics.com/cn/)
- [MVS SDK Download](https://www.hikrobotics.com/cn/machinevision/service/download)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [Rust Edition 2024](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
