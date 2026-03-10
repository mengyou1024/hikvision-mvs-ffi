# 海康威视 MVS SDK Rust FFI 绑定

[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)](https://developer.microsoft.com/en-us/windows/)
[![SDK Version](https://img.shields.io/badge/MVS%20SDK-4.5.1-orange.svg)](https://www.hikrobotics.com/cn/machinevision/service/download)
[![Rust Edition](https://img.shields.io/badge/rust-edition%202024-lightgrey.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

本项目提供了海康威视 MVS (Machine Vision Software SDK) SDK 的 Rust FFI 绑定，允许 Rust 应用程序直接与海康威视工业相机进行交互。

## ⚠️ 重要说明

- **平台限制**: 本项目**仅支持 Windows 平台**,不支持 Linux 或 macOS
- **SDK 依赖**: 必须安装**海康威视 MVS SDK 4.5.1**运行时环境
- **下载地址**: [MVS SDK 下载](https://www.hikrobotics.com/cn/machinevision/service/download)

## 📋 环境要求

### 系统要求
- **操作系统**: Windows 10/11
- **Rust 工具链**: 支持 edition 2024 的 Rust 版本
- **MVS SDK**: 海康威视 MVS SDK 4.5.1 运行时环境 (必需)

### 安装步骤

1. **安装 MVS SDK**
   ```bash
   # 访问以下地址下载并安装 MVS SDK 4.5.1
   https://www.hikrobotics.com/cn/machinevision/service/download
   ```

2. **验证安装**
   - 确保 SDK 已正确安装到系统目录
   - 检查环境变量是否包含 SDK 路径

3. **构建项目**
   ```bash
   cargo build
   ```

4. **运行示例**
   
   ```bash
   # 枚举设备示例
   cargo run --example enum_device
   
   # 采集图像示例
   cargo run --example grab_image
   ```

## 📚 文档

详细的 API文档请参考:
- **官方开发指南**: [`Docs/工业相机Windows SDK开发指南V4.5.1(C).chm`](./Docs/工业相机Windows SDK开发指南V4.5.1（C）.chm)

该 CHM 文档包含:
- SDK 初始化流程
- 设备枚举与连接
- 图像采集与控制
- 参数配置接口
- 错误码说明
- 完整的 API 参考

## 🏗️ 项目结构

```
hikvision-mvs-ffi/
├── Includes/                   # C 语言头文件（原始 SDK）
│   ├── CameraParams.h          # 相机参数定义
│   ├── MvCameraControl.h       # 相机控制接口
│   ├── MvErrorDefine.h         # 错误码定义
│   ├── MvISPErrorDefine.h      # ISP 错误码定义
│   ├── MvObsoleteInterfaces.h  # 废弃接口
│   ├── ObsoleteCamParams.h     # 废弃参数
│   └── PixelType.h             # 像素类型定义
├── src/
│   ├── lib.rs                  # 库入口
│   ├── camera_control.rs       # 相机控制接口 FFI 绑定
│   ├── camera_params.rs        # 相机参数结构体 FFI 绑定
│   ├── error_define.rs         # 错误码常量 FFI 绑定
│   ├── isp_error_define.rs     # ISP 错误码 FFI 绑定
│   ├── obsolete_cam_params.rs  # 废弃参数 FFI 绑定
│   ├── obsolete_interface.rs   # 废弃接口 FFI 绑定
│   ├── pixel_type.rs           # 像素类型 FFI 绑定
│   └── tests.rs                # 测试代码
├── examples/                   # 示例代码
│   ├── enum_device.rs          # 枚举设备示例
│   └── grab_image.rs           # 采集图像示例
├── Docs/                       # 官方文档
│   └── 工业相机Windows SDK开发指南V4.5.1(C).chm
├── Libraries/                  # SDK 库文件
├── Cargo.toml                  # 项目配置
└── build.rs                    # 构建脚本
```

## ✨ 主要功能

1. **camera_params.rs** - 相机参数模块
   - 设备信息结构体 
   - 图像信息结构体 
   - 属性配置结构体
   - 相关常量定义

2. **error_define.rs** - 错误码模块
   - 通用错误码
   - GenICam 系列错误码
   - GigE/USB设备错误码
   - 升级相关错误码

3. **pixel_type.rs** - 像素类型模块
   - 单色格式 (Mono8, Mono10, Mono12, Mono16)
   - Bayer 格式 (BayerGR8, BayerRG10, BayerGB12)
   - RGB/YUV Packed 格式
   - 3D 坐标格式

4. **camera_control.rs** - 相机控制接口模块
   - SDK 初始化/反初始化
   - 设备枚举与控制
   - 图像采集
   - 采集卡控制
   - 属性配置

## 🔧 使用示例

### 枚举设备

```rust
use hikvision_mvs_ffi::*;

fn main() {
    unsafe {
        // 初始化 SDK
        MV_CC_Initialize();
        
        // 枚举 GigE 设备
        let mut device_list = MV_CC_DEVICE_INFO_LIST::default();
        MV_CC_EnumDevices(MV_GIGE_DEVICE, &mut device_list);
        
        println!("找到 {} 个设备", device_list.nDeviceNum);
        
        // 清理
        MV_CC_Finalize();
    }
}
```

### 采集图像

```rust
use hikvision_mvs_ffi::*;

fn main() {
    unsafe {
        MV_CC_Initialize();
        
        // 创建设备句柄、打开设备...
        let mut handle: *mut c_void = std::ptr::null_mut();
        // MV_CC_CreateHandle(&mut handle, &device_info);
        // MV_CC_OpenDevice(handle, MV_ACCESS_Exclusive, 0);
        
        // 开始采集
        MV_CC_StartGrabbing(handle);
        
        // 获取图像
        let mut frame = MV_FRAME_OUT::default();
        MV_CC_GetImageBuffer(handle, &mut frame, 1000);
        
        // 处理图像数据...
        
        // 释放缓存
        MV_CC_FreeImageBuffer(handle, &frame);
        
        // 停止采集
        MV_CC_StopGrabbing(handle);
        
        // 关闭设备
        MV_CC_CloseDevice(handle);
        MV_CC_DestroyHandle(handle);
        
        MV_CC_Finalize();
    }
}
```

更多示例请查看 [`examples/`](examples) 目录。

### 注意事项

1. **安全性**: 所有 FFI 调用都是 `unsafe` 的，需要谨慎处理指针和内存
2. **线程安全**: 部分 SDK 接口不是线程安全的，请查阅官方文档
3. **资源管理**: 确保正确释放 SDK 分配的内存和资源
4. **错误处理**: 检查所有 API 调用的返回值，妥善处理错误码

## 📖 参考资料

- [海康威视机器人官网](https://www.hikrobotics.com/cn/)
- [MVS SDK 下载](https://www.hikrobotics.com/cn/machinevision/service/download)
- [Rust FFI 指南](https://doc.rust-lang.org/nomicon/ffi.html)
- [Rust Edition 2024](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
