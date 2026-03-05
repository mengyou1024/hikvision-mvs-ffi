//! Example usage of Hikvision MVS FFI bindings

use std::mem::MaybeUninit;

use hikvision_mvs_ffi::*;

fn main() {
    println!("Hikvision MVS FFI Example");
    println!("========================\n");

    // Initialize SDK
    unsafe {
        let ret = camera_control::MV_CC_Initialize();
        if ret != MV_OK as i32 {
            println!("Failed to initialize SDK: 0x{:08X}", ret);
            return;
        }
        println!("SDK initialized successfully");
    }

    // Get SDK version
    unsafe {
        let version = camera_control::MV_CC_GetSDKVersion();
        println!(
            "SDK Version: {}.{}.{}.{}",
            (version >> 24) & 0xFF,
            (version >> 16) & 0xFF,
            (version >> 8) & 0xFF,
            version & 0xFF
        );
    }

    // Enumerate GigE devices
    unsafe {
        let mut device_list = MaybeUninit::<MV_CC_DEVICE_INFO_LIST>::uninit();
        let ret = camera_control::MV_CC_EnumDevices(MV_GIGE_DEVICE, device_list.as_mut_ptr());

        let device_list = device_list.assume_init();

        if ret == MV_OK as i32 {
            println!("\nFound {} GigE device(s)", device_list.nDeviceNum);

            for i in 0..device_list.nDeviceNum as usize {
                if let Some(dev_info) = device_list.pDeviceInfo[i].as_ref() {
                    println!("\nDevice #{}:", i + 1);
                    println!("  Major Version: {}", dev_info.nMajorVer);
                    println!("  Minor Version: {}", dev_info.nMinorVer);
                    println!("  Device Type: 0x{:08X}", dev_info.nTLayerType);

                    // Access specific device info based on type
                    if dev_info.nTLayerType == MV_GIGE_DEVICE {
                        let gige_info = dev_info.SpecialInfo.stGigEInfo;
                        let serial = String::from_utf8_lossy(&gige_info.chSerialNumber[..])
                            .trim_end_matches('\0')
                            .to_string();
                        println!("  Serial Number: {}", serial);

                        let model = String::from_utf8_lossy(&gige_info.chModelName[..])
                            .trim_end_matches('\0')
                            .to_string();
                        println!("  Model Name: {}", model);
                    }
                }
            }
        } else {
            println!("Failed to enumerate devices: 0x{:08X}", ret);
        }
    }

    // Enumerate USB devices
    unsafe {
        let mut device_list = MaybeUninit::<MV_CC_DEVICE_INFO_LIST>::uninit();

        let ret = camera_control::MV_CC_EnumDevices(MV_USB_DEVICE, device_list.as_mut_ptr());

        let device_list = device_list.assume_init();

        if ret == MV_OK as i32 {
            println!("\nFound {} USB device(s)", device_list.nDeviceNum);

            for i in 0..device_list.nDeviceNum as usize {
                if let Some(dev_info) = device_list.pDeviceInfo[i].as_ref() {
                    println!("\nUSB Device #{}:", i + 1);
                    println!("  Device Type: 0x{:08X}", dev_info.nTLayerType);

                    if dev_info.nTLayerType == MV_USB_DEVICE {
                        let usb_info = dev_info.SpecialInfo.stUsb3VInfo;
                        let serial = String::from_utf8_lossy(&usb_info.chSerialNumber[..])
                            .trim_end_matches('\0')
                            .to_string();
                        println!("  Serial Number: {}", serial);

                        let model = String::from_utf8_lossy(&usb_info.chModelName[..])
                            .trim_end_matches('\0')
                            .to_string();
                        println!("  Model Name: {}", model);
                    }
                }
            }
        } else {
            println!("Failed to enumerate USB devices: 0x{:08X}", ret);
        }
    }

    // Cleanup
    unsafe {
        let ret = camera_control::MV_CC_Finalize();
        if ret != MV_OK as i32 {
            println!("Failed to finalize SDK: 0x{:08X}", ret);
            return;
        }
        println!("\nSDK finalized successfully");
    }

    println!("\nExample completed!");
}
