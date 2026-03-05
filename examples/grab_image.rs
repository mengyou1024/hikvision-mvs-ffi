//! Grab Image Example
//!
//! This example demonstrates how to:
//! - Initialize the SDK
//! - Enumerate devices
//! - Create and open device handle
//! - Start image acquisition using worker thread
//! - Display frame information
//! - Stop acquisition and cleanup

use hikvision_mvs_ffi::camera_params::{MV_CC_DEVICE_INFO_LIST, MV_FRAME_OUT};
use hikvision_mvs_ffi::error_define::MV_OK;
use hikvision_mvs_ffi::{MV_ACCESS_Exclusive, MV_GENTL_GIGE_DEVICE, camera_control};
use hikvision_mvs_ffi::{
    MV_GENTL_CAMERALINK_DEVICE, MV_GENTL_CXP_DEVICE, MV_GENTL_XOF_DEVICE, MV_GIGE_DEVICE,
    MV_USB_DEVICE,
};
use std::ffi::CString;
use std::io::{self, Write};
use std::mem::MaybeUninit;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

// Global flag for exit signal
static EXIT_FLAG: AtomicBool = AtomicBool::new(false);

/// Print device information based on device type
fn print_device_info(device_info: &hikvision_mvs_ffi::camera_params::MV_CC_DEVICE_INFO) {
    match device_info.nTLayerType {
        MV_GIGE_DEVICE => {
            let special_info = unsafe { &device_info.SpecialInfo.stGigEInfo };
            let n_ip = special_info.nCurrentIp;
            let ip1 = (n_ip >> 24) & 0xFF;
            let ip2 = (n_ip >> 16) & 0xFF;
            let ip3 = (n_ip >> 8) & 0xFF;
            let ip4 = n_ip & 0xFF;

            // Convert user defined name to string
            let user_name_bytes: Vec<u8> = special_info
                .chUserDefinedName
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let user_name = String::from_utf8_lossy(&user_name_bytes);

            println!("CurrentIp: {}.{}.{}.{}", ip1, ip2, ip3, ip4);
            println!("UserDefinedName: {}\n", user_name);
        }
        MV_USB_DEVICE => {
            let special_info = unsafe { &device_info.SpecialInfo.stUsb3VInfo };
            let user_name_bytes: Vec<u8> = special_info
                .chUserDefinedName
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let user_name = String::from_utf8_lossy(&user_name_bytes);
            let serial_bytes: Vec<u8> = special_info
                .chSerialNumber
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let serial = String::from_utf8_lossy(&serial_bytes);

            println!("UserDefinedName: {}", user_name);
            println!("Serial Number: {}", serial);
            println!("Device Number: {}\n", special_info.nDeviceNumber);
        }
        MV_GENTL_GIGE_DEVICE => {
            let special_info = unsafe { &device_info.SpecialInfo.stGigEInfo };
            let user_name_bytes: Vec<u8> = special_info
                .chUserDefinedName
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let user_name = String::from_utf8_lossy(&user_name_bytes);
            let serial_bytes: Vec<u8> = special_info
                .chSerialNumber
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let serial = String::from_utf8_lossy(&serial_bytes);
            let model_bytes: Vec<u8> = special_info
                .chModelName
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let model = String::from_utf8_lossy(&model_bytes);

            println!("UserDefinedName: {}", user_name);
            println!("Serial Number: {}", serial);
            println!("Model Name: {}\n", model);
        }
        MV_GENTL_CAMERALINK_DEVICE => {
            let special_info = unsafe { &device_info.SpecialInfo.stCMLInfo };
            let user_name_bytes: Vec<u8> = special_info
                .chUserDefinedName
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let user_name = String::from_utf8_lossy(&user_name_bytes);
            let serial_bytes: Vec<u8> = special_info
                .chSerialNumber
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let serial = String::from_utf8_lossy(&serial_bytes);
            let model_bytes: Vec<u8> = special_info
                .chModelName
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let model = String::from_utf8_lossy(&model_bytes);

            println!("UserDefinedName: {}", user_name);
            println!("Serial Number: {}", serial);
            println!("Model Name: {}\n", model);
        }
        MV_GENTL_CXP_DEVICE => {
            let special_info = unsafe { &device_info.SpecialInfo.stCXPInfo };
            let user_name_bytes: Vec<u8> = special_info
                .chUserDefinedName
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let user_name = String::from_utf8_lossy(&user_name_bytes);
            let serial_bytes: Vec<u8> = special_info
                .chSerialNumber
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let serial = String::from_utf8_lossy(&serial_bytes);
            let model_bytes: Vec<u8> = special_info
                .chModelName
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let model = String::from_utf8_lossy(&model_bytes);

            println!("UserDefinedName: {}", user_name);
            println!("Serial Number: {}", serial);
            println!("Model Name: {}\n", model);
        }
        MV_GENTL_XOF_DEVICE => {
            let special_info = unsafe { &device_info.SpecialInfo.stXoFInfo };
            let user_name_bytes: Vec<u8> = special_info
                .chUserDefinedName
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let user_name = String::from_utf8_lossy(&user_name_bytes);
            let serial_bytes: Vec<u8> = special_info
                .chSerialNumber
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let serial = String::from_utf8_lossy(&serial_bytes);
            let model_bytes: Vec<u8> = special_info
                .chModelName
                .iter()
                .take_while(|&&b| b != 0)
                .copied()
                .collect();
            let model = String::from_utf8_lossy(&model_bytes);

            println!("UserDefinedName: {}", user_name);
            println!("Serial Number: {}", serial);
            println!("Model Name: {}\n", model);
        }
        _ => {
            println!("Not support.\n");
        }
    }
}

/// Worker thread function for image acquisition
unsafe fn work_thread(handle: *mut c_void) -> c_int {
    println!("thread handle:{}", handle as usize);

    loop {
        let mut frame_out = MaybeUninit::<MV_FRAME_OUT>::uninit();

        let ret =
            unsafe { camera_control::MV_CC_GetImageBuffer(handle, frame_out.as_mut_ptr(), 1000) };

        let mut frame_out = unsafe { frame_out.assume_init() };

        if ret == MV_OK as c_int {
            println!(
                "Get Image Buffer: Width[{}], Height[{}], FrameNum[{}]",
                frame_out.stFrameInfo.nExtendWidth,
                frame_out.stFrameInfo.nExtendHeight,
                frame_out.stFrameInfo.nFrameNum
            );

            // Free the buffer
            let free_ret = unsafe { camera_control::MV_CC_FreeImageBuffer(handle, &mut frame_out) };
            if free_ret != MV_OK as c_int {
                eprintln!("Free Image Buffer fail! nRet [0x{:08X}]", free_ret);
            }
        } else {
            eprintln!("Get Image fail! nRet [0x{:08X}]", ret);
        }

        // Check exit flag
        if EXIT_FLAG.load(Ordering::Relaxed) {
            break;
        }
    }

    0
}

fn main() {
    let mut n_ret: c_int;
    let mut handle: *mut c_void = ptr::null_mut();

    unsafe {
        // Initialize SDK
        n_ret = camera_control::MV_CC_Initialize();
        if n_ret != MV_OK as c_int {
            eprintln!("Initialize SDK fail! nRet [0x{:08X}]", n_ret);
            return;
        }

        // Enumerate devices
        let mut device_list = MaybeUninit::<MV_CC_DEVICE_INFO_LIST>::uninit();
        n_ret = camera_control::MV_CC_EnumDevices(
            MV_GIGE_DEVICE
                | MV_USB_DEVICE
                | MV_GENTL_CAMERALINK_DEVICE
                | MV_GENTL_CXP_DEVICE
                | MV_GENTL_XOF_DEVICE,
            device_list.as_mut_ptr(),
        );

        let device_list = device_list.assume_init();

        if n_ret != MV_OK as c_int {
            eprintln!("Enum Devices fail! nRet [0x{:08X}]", n_ret);
            camera_control::MV_CC_Finalize();
            return;
        }

        // Print device information
        if device_list.nDeviceNum > 0 {
            for i in 0..device_list.nDeviceNum as usize {
                println!("[device {}]:", i);
                if let Some(info) = device_list.pDeviceInfo[i].as_ref() {
                    print_device_info(info);
                }
            }
        } else {
            println!("Find No Devices!");
            camera_control::MV_CC_Finalize();
            return;
        }

        // Get user input for camera index
        print!(
            "Please Input camera index(0-{}): ",
            device_list.nDeviceNum - 1
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let n_index: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Input error!");
                camera_control::MV_CC_Finalize();
                return;
            }
        };

        if n_index >= device_list.nDeviceNum {
            eprintln!("Input error! Index out of range.");
            camera_control::MV_CC_Finalize();
            return;
        }

        // Create handle
        n_ret = camera_control::MV_CC_CreateHandle(
            &mut handle,
            device_list.pDeviceInfo[n_index as usize],
        );
        if n_ret != MV_OK as c_int {
            eprintln!("Create Handle fail! nRet [0x{:08X}]", n_ret);
            camera_control::MV_CC_Finalize();
            return;
        }

        // Open device
        n_ret = camera_control::MV_CC_OpenDevice(handle, MV_ACCESS_Exclusive, 0);
        if n_ret != MV_OK as c_int {
            eprintln!("Open Device fail! nRet [0x{:08X}]", n_ret);
            camera_control::MV_CC_DestroyHandle(handle);
            camera_control::MV_CC_Finalize();
            return;
        }

        // Set optimal packet size for GigE cameras
        if device_list.pDeviceInfo[n_index as usize]
            .as_ref()
            .unwrap()
            .nTLayerType
            == MV_GIGE_DEVICE
        {
            let packet_size = camera_control::MV_CC_GetOptimalPacketSize(handle);
            if packet_size > 0 {
                n_ret = camera_control::MV_CC_SetIntValueEx(
                    handle,
                    b"GevSCPSPacketSize\0".as_ptr() as *const i8,
                    packet_size as i64,
                );
                if n_ret != MV_OK as c_int {
                    eprintln!("Warning: Set Packet Size fail nRet [0x{:08X}]!", n_ret);
                }
            } else {
                eprintln!(
                    "Warning: Get Packet Size fail nRet [0x{:08X}]!",
                    packet_size
                );
            }
        }

        // Set trigger mode to off
        n_ret = camera_control::MV_CC_SetEnumValue(
            handle,
            CString::new("TriggerMode").unwrap().as_ptr(),
            0,
        );
        if n_ret != MV_OK as c_int {
            eprintln!("Set Trigger Mode fail! nRet [0x{:08X}]", n_ret);
            camera_control::MV_CC_CloseDevice(handle);
            camera_control::MV_CC_DestroyHandle(handle);
            camera_control::MV_CC_Finalize();
            return;
        }

        // Start grabbing
        n_ret = camera_control::MV_CC_StartGrabbing(handle);
        if n_ret != MV_OK as c_int {
            eprintln!("Start Grabbing fail! nRet [0x{:08X}]", n_ret);
            camera_control::MV_CC_CloseDevice(handle);
            camera_control::MV_CC_DestroyHandle(handle);
            camera_control::MV_CC_Finalize();
            return;
        }

        // Spawn worker thread
        let thread_handle = handle as usize;

        let worker = thread::spawn(move || work_thread(thread_handle as *mut c_void));

        println!("Press Enter key to stop grabbing...");
        io::stdin().read_line(&mut input).unwrap();

        // Signal exit
        EXIT_FLAG.store(true, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(1000));

        // Wait for worker thread to finish
        let _ = worker.join();

        // Stop grabbing
        n_ret = camera_control::MV_CC_StopGrabbing(handle);
        if n_ret != MV_OK as c_int {
            eprintln!("Stop Grabbing fail! nRet [0x{:08X}]", n_ret);
        }

        // Close device
        n_ret = camera_control::MV_CC_CloseDevice(handle);
        if n_ret != MV_OK as c_int {
            eprintln!("Close Device fail! nRet [0x{:08X}]", n_ret);
        }

        // Destroy handle
        n_ret = camera_control::MV_CC_DestroyHandle(handle);
        if n_ret != MV_OK as c_int {
            eprintln!("Destroy Handle fail! nRet [0x{:08X}]", n_ret);
        }
    }

    // Finalize SDK
    unsafe {
        camera_control::MV_CC_Finalize();
    }

    println!("Press Enter key to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
