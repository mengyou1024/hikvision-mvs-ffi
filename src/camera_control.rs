//! MVS SDK 相机控制接口定义
//!
//! 该模块包含了海康威视 MVS SDK 的主要 C API 接口绑定

use crate::{
    MV_CC_DEVICE_INFO, MV_CC_DEVICE_INFO_LIST, MV_FRAME_OUT, MV_FRAME_OUT_INFO_EX, MV_SORT_METHOD,
    MV_XML_InterfaceType,
};
use core::ffi::{c_int, c_short, c_uchar, c_uint, c_void};
use std::ffi::c_char;

/// 无限等待超时时间
pub const MV_INFINITE: u32 = 0xFFFFFFFF;

/// 图像数据回调函数类型
pub type MV_OutputCallback = unsafe extern "C" fn(
    pData: *mut c_uchar,
    pFrameInfo: *mut MV_FRAME_OUT_INFO_EX,
    pUser: *mut c_void,
);

unsafe extern "C" {

    // ========================================================================
    // Part 1: SDK 初始化
    // ========================================================================

    /// 初始化 SDK
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_Initialize() -> c_int;

    /// 反初始化 SDK，释放资源
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// main 函数退出前调用
    pub unsafe fn MV_CC_Finalize() -> c_int;

    /// 获取 SDK 版本号
    ///
    /// # 返回值
    /// 返回 4 字节版本号
    /// |主|次|修正|测试 |
    /// 8bits 8bits 8bits 8bits
    ///
    /// # 示例
    /// 返回值为 0x01000001，即 SDK 版本号为 V1.0.0.1
    pub unsafe fn MV_CC_GetSDKVersion() -> c_uint;

    // ========================================================================
    // Part 2: 相机的控制和取流
    // ========================================================================

    /// 枚举设备
    ///
    /// # 参数
    /// * `nTLayerType` - 枚举传输层，如 MV_GIGE_DEVICE (GigE 设备)
    /// * `pstDevList` - 设备列表（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 设备列表的内存是在 SDK 内部分配的，多线程调用该接口时会进行设备列表内存的释放和申请，
    /// 建议尽量避免多线程枚举操作。
    ///
    /// 枚举传输层适配传入：
    /// - MV_GIGE_DEVICE: 传出所有 GigE 相关的设备信息（包含虚拟 GigE 和 GenTL 下的 GigE 设备）
    /// - MV_USB_DEVICE: 传出所有 USB 设备，包含虚拟 USB 设备
    pub unsafe fn MV_CC_EnumDevices(
        nTLayerType: c_uint,
        pstDevList: *mut MV_CC_DEVICE_INFO_LIST,
    ) -> c_int;

    /// 根据厂商名字枚举设备
    ///
    /// # 参数
    /// * `nTLayerType` - 枚举传输层
    /// * `pstDevList` - 设备列表（输出参数）
    /// * `strManufacturerName` - 厂商名字
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 设备列表的内存是在 SDK 内部分配的，多线程调用该接口时会进行设备列表内存的释放和申请，
    /// 建议尽量避免多线程枚举操作。
    pub unsafe fn MV_CC_EnumDevicesEx(
        nTLayerType: c_uint,
        pstDevList: *mut MV_CC_DEVICE_INFO_LIST,
        strManufacturerName: *const c_char,
    ) -> c_int;

    /// 枚举设备扩展（可指定排序方式枚举、根据厂商名字过滤）
    ///
    /// # 参数
    /// * `nTLayerType` - 枚举传输层（区分每一种传输层类型，不耦合）
    /// * `pstDevList` - 设备列表（输出参数）
    /// * `strManufacturerName` - 厂商名字（可传 NULL，即不过滤）
    /// * `enSortMethod` - 排序方式
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 设备列表的内存是在 SDK 内部分配的，多线程调用该接口时会进行设备列表内存的释放和申请，
    /// 建议尽量避免多线程枚举操作。
    ///
    /// strManufacturerName 可传入 NULL，若传入 NULL 则返回排好序的所有设备列表，
    /// 若不为 NULL 则只返回排好序的指定厂商设备列表。
    pub unsafe fn MV_CC_EnumDevicesEx2(
        nTLayerType: c_uint,
        pstDevList: *mut MV_CC_DEVICE_INFO_LIST,
        strManufacturerName: *const c_char,
        enSortMethod: MV_SORT_METHOD,
    ) -> c_int;

    /// 设备是否可连接
    ///
    /// # 参数
    /// * `pstDevInfo` - 设备信息结构体
    /// * `nAccessMode` - 访问权限
    ///
    /// # 返回值
    /// 可连接返回 true，不可连接返回 false
    ///
    /// # 注意事项
    /// - GIGE 相机：读取设备 CCP 寄存器的值，判断当前状态是否具有某种访问权限
    /// - 如果设备不支持 MV_ACCESS_ExclusiveWithSwitch、MV_ACCESS_ControlWithSwitch、
    ///   MV_ACCESS_ControlSwitchEnable 这三种模式，接口返回 false
    /// - MV_GIGE_DEVICE/MV_GENTL_GIGE_DEVICE 类型设备：按照 nAccessMode，返回当前是否可以被连接
    /// - 该接口支持虚拟相机、U3V 相机、cxp、xof、cameralink 采集卡相机，nAccessMode 无效
    /// - 如果相机没有被连接返回 true，如果设备被第三方连接，则返回 false
    /// - 该接口不支持 CameraLink 设备 (返回 false)
    pub unsafe fn MV_CC_IsDeviceAccessible(
        pstDevInfo: *const MV_CC_DEVICE_INFO,
        nAccessMode: c_uint,
    ) -> bool;

    /// 创建设备句柄
    ///
    /// # 参数
    /// * `handle` - 设备句柄（输出参数）
    /// * `pstDevInfo` - 设备信息结构体
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 根据输入的设备信息，创建库内部必须的资源和初始化内部模块。
    /// 通过该接口创建句柄，调用 SDK 接口，会默认生成 SDK 日志文件，
    /// 如果不需要生成日志文件，可以将日志配置文件中的日志等级改成 off。
    pub unsafe fn MV_CC_CreateHandle(
        handle: *mut *mut c_void,
        pstDevInfo: *const MV_CC_DEVICE_INFO,
    ) -> c_int;

    /// 销毁设备句柄
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 如果传入采集卡句柄，其效果和 MV_CC_DestroyInterface 相同
    pub unsafe fn MV_CC_DestroyHandle(handle: *mut c_void) -> c_int;

    /// 打开设备
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nAccessMode` - 访问权限（仅对 MV_GIGE_DEVICE/MV_GENTL_GIGE_DEVICE 类型的设备有效）
    /// * `nSwitchoverKey` - 切换访问权限时的密钥（仅对 MV_GIGE_DEVICE 类型的设备有效）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 根据设置的设备参数，找到对应的设备，连接设备。
    /// 调用接口时可不传入 nAccessMode 和 nSwitchoverKey，此时默认设备访问模式为独占权限。
    ///
    /// - MV_GIGE_DEVICE 类型设备：目前相机固件暂不支持 MV_ACCESS_ExclusiveWithSwitch、
    ///   MV_ACCESS_ControlWithSwitch、MV_ACCESS_ControlSwitchEnable、
    ///   MV_ACCESS_ControlSwitchEnableWithKey 这四种抢占模式，SDK 接口支持设置
    /// - MV_GENTL_GIGE_DEVICE 设备只支持 nAccessMode 是 MV_ACCESS_Exclusive、
    ///   MV_ACCESS_Control、MV_ACCESS_Monitor 权限
    /// - 对于 U3V 设备、CXP、Cameralink、Xof 设备、虚拟 GEV、虚拟 U3V 设备：
    ///   nAccessMode、nSwitchoverKey 这两个参数无效；默认以控制权限打开设备
    /// - 该接口支持网口设备不枚举直接打开，不支持 U 口和 GenTL 设备不枚举打开设备
    pub unsafe fn MV_CC_OpenDevice(
        handle: *mut c_void,
        nAccessMode: c_uint,
        nSwitchoverKey: c_short,
    ) -> c_int;

    /// 关闭设备
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 通过 MV_CC_OpenDevice 连接设备后，可以通过该接口断开设备连接，释放资源
    pub unsafe fn MV_CC_CloseDevice(handle: *mut c_void) -> c_int;

    /// 判断设备是否处于连接状态
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    ///
    /// # 返回值
    /// 设备处于连接状态返回 true；没连接或失去连接返回 false
    pub unsafe fn MV_CC_IsDeviceConnected(handle: *mut c_void) -> bool;

    /// 注册图像数据回调
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `cbOutput` - 回调函数指针
    /// * `pUser` - 用户自定义变量
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 通过该接口可以设置图像数据回调函数，在 MV_CC_CreateHandle 之后即可调用。
    /// 图像数据采集有两种方式，两种方式不能复用：
    ///
    /// 方式一：调用 MV_CC_RegisterImageCallBackEx 设置图像数据回调函数，
    ///       然后调用 MV_CC_StartGrabbing 开始采集，采集的图像数据在设置的回调函数中返回
    ///
    /// 方式二：调用 MV_CC_StartGrabbing 开始采集，然后在应用层循环调用
    ///       MV_CC_GetOneFrameTimeout 获取指定像素格式的帧数据，
    ///       获取帧数据时上层应用程序需要根据帧率控制好调用该接口的频率
    ///
    /// 该接口不支持 MV_CAMERALINK_DEVICE 类型的设备
    pub unsafe fn MV_CC_RegisterImageCallBackEx(
        handle: *mut c_void,
        cbOutput: MV_OutputCallback,
        pUser: *mut c_void,
    ) -> c_int;

    /// 开始取流
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口不支持 MV_CAMERALINK_DEVICE 类型的设备
    pub unsafe fn MV_CC_StartGrabbing(handle: *mut c_void) -> c_int;

    /// 停止取流
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口不支持 MV_CAMERALINK_DEVICE 类型的设备
    pub unsafe fn MV_CC_StopGrabbing(handle: *mut c_void) -> c_int;

    /// 使用内部缓存获取一帧图片（与 MV_CC_Display 不能同时使用）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstFrame` - 图像数据和图像信息（输出参数）
    /// * `nMsec` - 等待超时时间，输入 MV_INFINITE 时表示无限等待
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 调用该接口获取图像数据帧之前需要先调用 MV_CC_StartGrabbing 启动图像采集。
    /// 该接口为主动式获取帧数据，上层应用程序需要根据帧率，控制好调用该接口的频率。
    /// 该接口支持设置超时时间，SDK 内部等待直到有数据时返回，可以增加取流平稳性，
    /// 适合用于对平稳性要求较高的场合。
    ///
    /// 该接口与 MV_CC_FreeImageBuffer 配套使用，当处理完取到的数据后，
    /// 需要用 MV_CC_FreeImageBuffer 接口将 pFrame 内的数据指针权限进行释放。
    ///
    /// 该接口与 MV_CC_GetOneFrameTimeout 相比，有着更高的效率。
    /// 且其取流缓存的分配是由 sdk 内部自动分配的，
    /// 而 MV_CC_GetOneFrameTimeout 接口是需要客户自行分配。
    ///
    /// 该接口在调用 MV_CC_Display 后无法取流。
    /// 该接口对于 U3V、GIGE 设备均可支持。
    /// 该接口不支持 CameraLink 设备。
    pub unsafe fn MV_CC_GetImageBuffer(
        handle: *mut c_void,
        pstFrame: *mut MV_FRAME_OUT,
        nMsec: c_uint,
    ) -> c_int;

    /// 释放图像缓存（此接口用于释放不再使用的图像缓存，与 MV_CC_GetImageBuffer 配套使用）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstFrame` - 图像数据和图像数据
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口与 MV_CC_GetImageBuffer 配套使用，使用 MV_CC_GetImageBuffer 接口取到的图像数据 pFrame，
    /// 需要用 MV_CC_FreeImageBuffer 接口进行权限释放。
    ///
    /// 该接口取流效率高于 GetOneFrameTimeout 接口。
    /// 当 GetImageBuffer 不进行 FreeImageBuffer 时，最大输出图像个数为当前配置下
    /// SDK 的缓存节点个数 (用户可以调用 SetImageNode 接口，调节 SDK 的缓存个数）。
    ///
    /// 该接口对于 U3V、GIGE 设备均可支持。
    /// 该接口不支持 CameraLink 设备。
    pub unsafe fn MV_CC_FreeImageBuffer(handle: *mut c_void, pstFrame: *mut MV_FRAME_OUT) -> c_int;

    /// 采用超时机制获取一帧图片，SDK 内部等待直到有数据时返回
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pData` - 图像数据接收指针
    /// * `nDataSize` - 接收缓存大小
    /// * `pstFrameInfo` - 图像信息结构体
    /// * `nMsec` - 等待超时时间
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 调用该接口获取图像数据帧之前需要先调用 MV_CC_StartGrabbing 启动图像采集。
    /// 该接口为主动式获取帧数据，上层应用程序需要根据帧率，控制好调用该接口的频率。
    /// 该接口支持设置超时时间，SDK 内部等待直到有数据时返回，可以增加取流平稳性，
    /// 适合用于对平稳性要求较高的场合。
    ///
    /// 该接口对于 U3V、GIGE 设备均可支持。
    /// 该接口不支持 CameraLink 设备。
    pub unsafe fn MV_CC_GetOneFrameTimeout(
        handle: *mut c_void,
        pData: *mut c_uchar,
        nDataSize: c_uint,
        pstFrameInfo: *mut MV_FRAME_OUT_INFO_EX,
        nMsec: c_uint,
    ) -> c_int;

    /// 清除取流数据缓存
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口允许用户在不停止取流的时候，就能清除缓存中不需要的图像。
    /// 该接口在连续模式切触发模式后，可以清除历史数据。
    /// 该接口目前只能清除 SDK 内部的图像缓存，采集卡内的缓存还无法清除。
    pub unsafe fn MV_CC_ClearImageBuffer(handle: *mut c_void) -> c_int;

    /// 获取当前图像缓存区的有效图像个数
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnValidImageNum` - 当前图像缓存区中有效图像个数的指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口只统计 SDK 内部的有效图像个数，不包括采集卡缓存内的有效图像个数
    pub unsafe fn MV_CC_GetValidImageNum(
        handle: *mut c_void,
        pnValidImageNum: *mut c_uint,
    ) -> c_int;

    /// 显示一帧图像
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `hWnd` - 窗口句柄
    /// * `pstDisplayInfo` - 图像信息
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口支持渲染宽高大小至 int 类型
    /// 渲染模式为 D3D 时，支持的最大分辨率为 16384 * 163840
    pub unsafe fn MV_CC_DisplayOneFrameEx(
        handle: *mut c_void,
        hWnd: *mut c_void,
        pstDisplayInfo: *mut crate::MV_DISPLAY_FRAME_INFO_EX,
    ) -> c_int;

    /// 显示一帧图像（扩展版本）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `hWnd` - 窗口句柄
    /// * `pstImage` - 图像信息
    /// * `enRenderMode` - 渲染方式，Windows:0-GDI 1-D3D 2-OpenGL Linux:0-OpenGL
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 可选择 OpenGL 渲染模式，支持 PixelType_Gvsp_RGB8_Packed，PixelType_Gvsp_BGR8_Packed，PixelType_Gvsp_Mono8 三种像素格式图像大小超过 4GB 的渲染，其他渲染模式不支持。
    /// 若图像大小未超过 4GB，支持宽高大小至 int 类型
    /// 调用时需要输入 MV_CC_IMAGE 结构体中 nImageLen 的值
    /// 渲染模式为 D3D 时，支持的最大分辨率为 16384 * 163840
    pub unsafe fn MV_CC_DisplayOneFrameEx2(
        handle: *mut c_void,
        hWnd: *mut c_void,
        pstImage: *mut crate::MV_CC_IMAGE,
        enRenderMode: c_uint,
    ) -> c_int;

    /// 设置 SDK 内部图像缓存节点个数，大于等于 1，在抓图前调用
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nNum` - 缓存节点个数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 调用该接口可以设置 SDK 内部图像缓存节点个数，在调用 MV_CC_StartGrabbing 开始抓图前调用。
    /// 不同相机因为取流方式不同，不调用 MV_CC_SetImageNodeNum 接口情况下，默认不同相机默认缓存节点个数不同：比如 双 U 内部分配默认 3 个节点
    /// SDK 实际分配的节点个数 = SDK 内部预分配的个数 + 用户分配的节点 (MV_CC_SetImageNodeNum);若系统内存资源不够，SDK 内部会重新计算，以重新计算的节点个数为准
    /// 该接口不支持 MV_CAMERALINK_DEVICE 类型的设备。
    /// 该接口仅对 SDK 内部分配缓存模式有效，外部分配缓存模式（即调用 MV_CC_RegisterBuffer）无效;
    pub unsafe fn MV_CC_SetImageNodeNum(handle: *mut c_void, nNum: c_uint) -> c_int;

    /// 设置取流策略
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `enGrabStrategy` - 策略枚举值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口定义了四种取流策略，用户可以根据实际需求进行选择。具体描述如下：
    /// - OneByOne: 从旧到新一帧一帧的从输出缓存列表中获取图像，打开设备后默认为该策略
    /// - LatestImagesOnly: 仅从输出缓存列表中获取最新的一帧图像，同时清空输出缓存列表
    /// - LatestImages: 从输出缓存列表中获取最新的 OutputQueueSize 帧图像，其中 OutputQueueSize 范围为 1-ImageNodeNum，可用 MV_CC_SetOutputQueueSize 接口设置，ImageNodeNum 默认为 1，
    ///   可用 MV_CC_SetImageNodeNum 接口设置 OutputQueueSize 设置成 1 等同于 LatestImagesOnly 策略，OutputQueueSize 设置成 ImageNodeNum 等同于 OneByOne 策略
    /// - UpcomingImage: 在调用取流接口时忽略输出缓存列表中所有图像，并等待设备即将生成的一帧图像。（该策略不支持 MV_USB_DEVICE 设备）
    ///   该接口在 Windows 平台仅支持 MV_GIGE_DEVICE、MV_USB_DEVICE 设备，在 Linux 平台仅支持 MV_USB_DEVICE 设备；
    pub unsafe fn MV_CC_SetGrabStrategy(
        handle: *mut c_void,
        enGrabStrategy: crate::MV_GRAB_STRATEGY,
    ) -> c_int;

    /// 设置输出缓存个数（只有在 MV_GrabStrategy_LatestImages 策略下才有效，范围：1-ImageNodeNum）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nOutputQueueSize` - 输出缓存个数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口需与 LatestImages 取流策略配套调用，用于设置 LatestImages 策略下最多允许缓存图像的个数。可以在取流过程中动态调节输出缓存个数
    /// 若为双 U 口相机，nOutputQueueSize 最小应设置为 2
    /// 该接口在 Windows 平台仅支持 MV_GIGE_DEVICE、MV_USB_DEVICE 设备，在 Linux 平台仅支持 MV_USB_DEVICE 设备；
    pub unsafe fn MV_CC_SetOutputQueueSize(handle: *mut c_void, nOutputQueueSize: c_uint) -> c_int;

    /// 获取设备信息，取流之前调用
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstDevInfo` - 返回给调用者有关设备信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 支持用户在打开设备后获取设备信息，不支持 GenTL 设备
    /// 若该设备是 GigE 设备，则调用该接口存在阻塞风险，因此不建议在取流过程中调用该接口。
    pub unsafe fn MV_CC_GetDeviceInfo(
        handle: *mut c_void,
        pstDevInfo: *mut MV_CC_DEVICE_INFO,
    ) -> c_int;

    /// 获取各种类型的信息
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstInfo` - 返回给调用者有关设备各种类型的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 接口里面输入需要获取的信息类型（指定 MV_ALL_MATCH_INFO 结构体中的 nType 类型），获取对应的信息（在 MV_ALL_MATCH_INFO 结构体中 pInfo 里返回）
    /// 该接口的调用前置条件取决于所获取的信息类型，获取 GigE 设备的 MV_MATCH_TYPE_NET_DETECT 信息需在开启抓图之后调用，获取 U3V 设备的 MV_MATCH_TYPE_USB_DETECT 信息需在打开设备之后调用
    /// 信息类型 MV_MATCH_TYPE_NET_DETECT 对应结构体 MV_MATCH_INFO_NET_DETECT，只支持 MV_GIGE_DEVICE 相机/MV_GENTL_GIGE_DEVICE 相机
    /// 信息类型 MV_MATCH_TYPE_USB_DETECT 对应结构体 MV_MATCH_INFO_USB_DETECT，只支持 MV_USB_DEVICE 类型相机
    /// 该接口不支持 MV_CAMERALINK_DEVICE 设备。
    pub unsafe fn MV_CC_GetAllMatchInfo(
        handle: *mut c_void,
        pstInfo: *mut crate::MV_ALL_MATCH_INFO,
    ) -> c_int;

    // ========================================================================
    // Part 3: 采集卡的配置
    // ========================================================================

    /// 枚举采集卡
    ///
    /// # 参数
    /// * `nTLayerType` - 采集卡接口类型 eg: (MV_GIGE_INTERFACE | MV_CAMERALINK_INTERFACE | MV_CXP_INTERFACE| MV_XOF_INTERFACE | MV_VIR_INTERFACE）
    /// * `pInterfaceInfoList` - 采集卡列表（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口不支持 arm 和 Linux32 平台
    pub unsafe fn MV_CC_EnumInterfaces(
        nTLayerType: c_uint,
        pInterfaceInfoList: *mut crate::MV_INTERFACE_INFO_LIST,
    ) -> c_int;

    /// 创建采集卡句柄
    ///
    /// # 参数
    /// * `handle` - 采集卡句柄（输出参数）
    /// * `pInterfaceInfo` - 采集卡信息
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口不支持 arm 和 Linux32 平台
    pub unsafe fn MV_CC_CreateInterface(
        handle: *mut *mut c_void,
        pInterfaceInfo: *mut crate::MV_INTERFACE_INFO,
    ) -> c_int;

    /// 通过采集卡 ID 创建采集卡句柄
    ///
    /// # 参数
    /// * `handle` - 采集卡句柄（输出参数）
    /// * `pInterfaceID` - 采集卡 ID
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口不支持 arm 和 Linux32 平台
    pub unsafe fn MV_CC_CreateInterfaceByID(
        handle: *mut *mut c_void,
        pInterfaceID: *const c_char,
    ) -> c_int;

    /// 打开采集卡
    ///
    /// # 参数
    /// * `handle` - 采集卡句柄
    /// * `pReserved` - 预留，直接填 NULL
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口不支持 arm 和 Linux32 平台
    pub unsafe fn MV_CC_OpenInterface(handle: *mut c_void, pReserved: *mut c_char) -> c_int;

    /// 关闭采集卡
    ///
    /// # 参数
    /// * `handle` - 采集卡句柄
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口不支持 arm 和 Linux32 平台
    pub unsafe fn MV_CC_CloseInterface(handle: *mut c_void) -> c_int;

    /// 销毁采集卡句柄
    ///
    /// # 参数
    /// * `handle` - 采集卡句柄
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// MV_CC_DestroyInterface 如果传入相机句柄，其效果和 MV_CC_DestroyHandle 相同; 该接口不支持 arm 和 Linux32 平台
    pub unsafe fn MV_CC_DestroyInterface(handle: *mut c_void) -> c_int;

    /// 通过采集卡句柄枚举设备
    ///
    /// # 参数
    /// * `handle` - 采集卡句柄
    /// * `pstDevList` - 设备列表（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 设备列表的内存是在 SDK 内部分配的，多线程调用该接口时会进行设备列表内存的释放和申请
    /// 建议尽量避免多线程枚举操作。
    pub unsafe fn MV_CC_EnumDevicesByInterface(
        handle: *mut c_void,
        pstDevList: *mut MV_CC_DEVICE_INFO_LIST,
    ) -> c_int;

    // ========================================================================
    // Part 4: 相机/采集卡属性万能配置接口
    // ========================================================================

    /// 获取 Integer 属性值
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值，如获取宽度信息则为"Width"
    /// * `pstIntValue` - 返回给调用者有关设备属性结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以获取 int 类型的指定节点的值。
    pub unsafe fn MV_CC_GetIntValueEx(
        handle: *mut c_void,
        strKey: *const c_char,
        pstIntValue: *mut crate::MVCC_INTVALUE_EX,
    ) -> c_int;

    /// 设置 Integer 型属性值
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值，如获取宽度信息则为"Width"
    /// * `nValue` - 想要设置的设备的属性值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以设置 int 类型的指定节点的值。
    pub unsafe fn MV_CC_SetIntValueEx(
        handle: *mut c_void,
        strKey: *const c_char,
        nValue: i64,
    ) -> c_int;

    /// 获取 Enum 属性值
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值，如获取像素格式信息则为"PixelFormat"
    /// * `pstEnumValue` - 返回给调用者有关设备属性结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以获取 Enum 类型的指定节点的值。
    pub unsafe fn MV_CC_GetEnumValue(
        handle: *mut c_void,
        strKey: *const c_char,
        pstEnumValue: *mut crate::MVCC_ENUMVALUE,
    ) -> c_int;

    /// 获取 Enum 属性值（扩展版本）
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值，如获取像素格式信息则为"PixelFormat"
    /// * `pstEnumValue` - 返回给调用者有关设备属性结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以获取 Enum 类型的指定节点的值，区别与 MV_CC_GetEnumValue，此接口返回的枚举有效个数扩展到 256 个。
    pub unsafe fn MV_CC_GetEnumValueEx(
        handle: *mut c_void,
        strKey: *const c_char,
        pstEnumValue: *mut crate::MVCC_ENUMVALUE_EX,
    ) -> c_int;

    /// 设置 Enum 型属性值
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值，如获取像素格式信息则为"PixelFormat"
    /// * `nValue` - 想要设置的设备的属性值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以设置 Enum 类型的指定节点的值
    pub unsafe fn MV_CC_SetEnumValue(
        handle: *mut c_void,
        strKey: *const c_char,
        nValue: c_uint,
    ) -> c_int;

    /// 获取 Enum 型节点指定值的符号
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值，如获取像素格式信息则为"PixelFormat"
    /// * `pstEnumEntry` - 想要获取的设备的属性符号（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以获取 Enum 类型的指定节点的值所对应的符号
    pub unsafe fn MV_CC_GetEnumEntrySymbolic(
        handle: *mut c_void,
        strKey: *const c_char,
        pstEnumEntry: *mut crate::MVCC_ENUMENTRY,
    ) -> c_int;

    /// 设置 Enum 型属性值（通过字符串）
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值，如获取像素格式信息则为"PixelFormat"
    /// * `strValue` - 想要设置的设备的属性字符串
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以设置 Enum 类型的指定节点的值
    pub unsafe fn MV_CC_SetEnumValueByString(
        handle: *mut c_void,
        strKey: *const c_char,
        strValue: *const c_char,
    ) -> c_int;

    /// 获取 Float 属性值
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值
    /// * `pstFloatValue` - 返回给调用者有关设备属性结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以获取 float 类型的指定节点的值
    pub unsafe fn MV_CC_GetFloatValue(
        handle: *mut c_void,
        strKey: *const c_char,
        pstFloatValue: *mut crate::MVCC_FLOATVALUE,
    ) -> c_int;

    /// 设置 float 型属性值
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值
    /// * `fValue` - 想要设置的设备的属性值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以设置 float 类型的指定节点的值
    pub unsafe fn MV_CC_SetFloatValue(
        handle: *mut c_void,
        strKey: *const c_char,
        fValue: f32,
    ) -> c_int;

    /// 获取 Boolean 属性值
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值
    /// * `pbValue` - 返回给调用者有关设备属性值（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以获取 bool 类型的指定节点的值
    pub unsafe fn MV_CC_GetBoolValue(
        handle: *mut c_void,
        strKey: *const c_char,
        pbValue: *mut bool,
    ) -> c_int;

    /// 设置 Boolean 型属性值
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值
    /// * `bValue` - 想要设置的设备的属性值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以设置 bool 类型的指定节点的值
    pub unsafe fn MV_CC_SetBoolValue(
        handle: *mut c_void,
        strKey: *const c_char,
        bValue: bool,
    ) -> c_int;

    /// 获取 String 属性值
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值
    /// * `pstStringValue` - 返回给调用者有关设备属性结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以获取 string 类型的指定节点的值
    pub unsafe fn MV_CC_GetStringValue(
        handle: *mut c_void,
        strKey: *const c_char,
        pstStringValue: *mut crate::MVCC_STRINGVALUE,
    ) -> c_int;

    /// 设置 String 型属性值
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值
    /// * `strValue` - 想要设置的设备的属性值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以设置 string 类型的指定节点的值
    pub unsafe fn MV_CC_SetStringValue(
        handle: *mut c_void,
        strKey: *const c_char,
        strValue: *const c_char,
    ) -> c_int;

    /// 设置 Command 型属性值
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strKey` - 属性键值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以设置指定的 Command 类型节点
    pub unsafe fn MV_CC_SetCommandValue(handle: *mut c_void, strKey: *const c_char) -> c_int;

    // ========================================================================
    // Part 4: 读写寄存器内存接口
    // ========================================================================

    /// 读内存
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `pBuffer` - 作为返回值使用，保存读到的内存值（输出参数）
    /// * `nAddress` - 待读取的内存地址，该地址可以从设备的 Camera.xml 文件中获取
    /// * `nLength` - 待读取的内存长度
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 访问设备，读取某段寄存器的数据。
    /// GEV 设备内存值是按照大端模式存储的，采集卡设备和采集卡下相机按照大端存储，
    /// 其它协议设备按照小端存储
    pub unsafe fn MV_CC_ReadMemory(
        handle: *mut c_void,
        pBuffer: *mut c_void,
        nAddress: i64,
        nLength: i64,
    ) -> c_int;

    /// 写内存
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `pBuffer` - 待写入的内存值
    /// * `nAddress` - 待写入的内存地址，该地址可以从设备的 Camera.xml 文件中获取
    /// * `nLength` - 待写入的内存长度
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 访问设备，把一段数据写入某段寄存器。
    /// 注意 GEV 设备内存值要按照大端模式存储，采集卡设备和采集卡下相机按照大端存储，
    /// 其它协议设备按照小端存储
    pub unsafe fn MV_CC_WriteMemory(
        handle: *mut c_void,
        pBuffer: *const c_void,
        nAddress: i64,
        nLength: i64,
    ) -> c_int;

    /// 清除 GenICam 节点缓存
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_InvalidateNodes(handle: *mut c_void) -> c_int;

    /// 获取设备属性树 XML
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `pData` - XML 数据接收缓存（输出参数）
    /// * `nDataSize` - 接收缓存大小
    /// * `pnDataLen` - 实际数据大小（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 当 pData 为 NULL 或 nDataSize 比实际的 xml 文件小时，不拷贝数据，由 pnDataLen 返回 xml 文件大小。
    /// 当 pData 为有效缓存地址，且缓存足够大时，拷贝完整数据保存在该缓存里面，并由 pnDataLen 返回 xml 文件实际大小。
    pub unsafe fn MV_XML_GetGenICamXML(
        handle: *mut c_void,
        pData: *mut c_uchar,
        nDataSize: c_uint,
        pnDataLen: *mut c_uint,
    ) -> c_int;

    /// 获得当前节点的访问模式
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strName` - 节点名称
    /// * `penAccessMode` - 节点的访问模式（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_XML_GetNodeAccessMode(
        handle: *mut c_void,
        strName: *const c_char,
        penAccessMode: *mut crate::MV_XML_AccessMode,
    ) -> c_int;

    /// 获得当前节点的类型
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strName` - 节点名称
    /// * `penInterfaceType` - 节点的类型（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口可以在调用万能接口之前，提前知道节点类型，
    /// 方便用户选择合适的万能接口进行节点值的设置和获取
    pub unsafe fn MV_XML_GetNodeInterfaceType(
        handle: *mut c_void,
        strName: *const c_char,
        penInterfaceType: *mut MV_XML_InterfaceType,
    ) -> c_int;

    /// 保存设备属性
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strFileName` - 属性文件名
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_FeatureSave(handle: *mut c_void, strFileName: *const c_char) -> c_int;

    /// 导入设备属性
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strFileName` - 属性文件名
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_FeatureLoad(handle: *mut c_void, strFileName: *const c_char) -> c_int;

    /// 导入设备属性并保存错误信息列表
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `strFileName` - 属性文件名
    /// * `pstNodeErrorList` - 错误信息列表（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 部分节点导入失败时，接口返回 MV_OK，通过错误信息列表中 stNodeError 获取出错节点及失败原因
    pub unsafe fn MV_CC_FeatureLoadEx(
        handle: *mut c_void,
        strFileName: *const c_char,
        pstNodeErrorList: *mut crate::MVCC_NODE_ERROR_LIST,
    ) -> c_int;

    /// 从设备读取文件
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `pstFileAccess` - 文件存取结构体
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_FileAccessRead(
        handle: *mut c_void,
        pstFileAccess: *mut crate::MV_CC_FILE_ACCESS,
    ) -> c_int;

    /// 从设备读取文件，文件是 Data 数据
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `pstFileAccess` - 文件存取结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 避免文件操作权限问题读失败
    pub unsafe fn MV_CC_FileAccessReadEx(
        handle: *mut c_void,
        pstFileAccess: *mut crate::MV_CC_FILE_ACCESS_EX,
    ) -> c_int;

    /// 将文件写入设备
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `pstFileAccess` - 文件存取结构体
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_FileAccessWrite(
        handle: *mut c_void,
        pstFileAccess: *mut crate::MV_CC_FILE_ACCESS,
    ) -> c_int;

    /// 将缓存 (buffer) 写入设备
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `pstFileAccessEx` - 文件存取结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口直接使用缓存数据，进行读写操作，避免直接操作文件出现无权限的问题，
    /// 是 MV_CC_FileAccessWrite 的扩展接口
    pub unsafe fn MV_CC_FileAccessWriteEx(
        handle: *mut c_void,
        pstFileAccessEx: *mut crate::MV_CC_FILE_ACCESS_EX,
    ) -> c_int;

    /// 获取文件存取的进度
    ///
    /// # 参数
    /// * `handle` - 设备句柄/采集卡句柄
    /// * `pstFileAccessProgress` - 进度内容（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码（当前文件存取的状态）
    pub unsafe fn MV_CC_GetFileAccessProgress(
        handle: *mut c_void,
        pstFileAccessProgress: *mut crate::MV_CC_FILE_ACCESS_PROGRESS,
    ) -> c_int;

    // ========================================================================
    // Part 5: 相机和采集卡升级
    // ========================================================================

    /// 设备本地升级
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `strFilePathName` - 文件名
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 通过该接口可以将升级固件文件发送给设备进行升级。
    /// 该接口需要等待升级固件文件成功传给设备端之后再返回，响应时间可能较长。
    pub unsafe fn MV_CC_LocalUpgrade(handle: *mut c_void, strFilePathName: *const c_void) -> c_int;

    /// 获取升级进度
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnProcess` - 进度接收地址（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_GetUpgradeProcess(handle: *mut c_void, pnProcess: *mut c_uint) -> c_int;

    // ========================================================================
    // Part 6: 相机和采集卡注册异常回调和事件接口
    // ========================================================================

    /// 注册异常消息回调，在打开设备之后调用
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `cbException` - 异常回调函数指针
    /// * `pUser` - 用户自定义变量
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口需要在 MV_CC_OpenDevice 打开设备之后调用。
    /// 设备异常断开连接后可以在回调里面获取到异常消息，
    /// GigE 设备掉线之后需要先调用 MV_CC_CloseDevice 接口关闭设备，
    /// 再调用 MV_CC_OpenDevice 接口重新打开设备。
    pub unsafe fn MV_CC_RegisterExceptionCallBack(
        handle: *mut c_void,
        cbException: unsafe extern "C" fn(c_uint, *mut c_void),
        pUser: *mut c_void,
    ) -> c_int;

    /// 注册全部事件回调，在打开设备之后调用
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `cbEvent` - 事件回调函数指针
    /// * `pUser` - 用户自定义变量
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 通过该接口设置事件回调，可以在回调函数里面获取采集、曝光等事件信息。
    /// 该接口不支持 CameraLink 设备。
    pub unsafe fn MV_CC_RegisterAllEventCallBack(
        handle: *mut c_void,
        cbEvent: unsafe extern "C" fn(*mut crate::MV_EVENT_OUT_INFO, *mut c_void),
        pUser: *mut c_void,
    ) -> c_int;

    /// 注册单个事件回调，在打开设备之后调用
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `strEventName` - 事件名称
    /// * `cbEvent` - 事件回调函数指针
    /// * `pUser` - 用户自定义变量
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 通过该接口设置事件回调，可以在回调函数里面获取采集、曝光等事件信息。
    /// 该接口不支持 CameraLink 设备。
    pub unsafe fn MV_CC_RegisterEventCallBackEx(
        handle: *mut c_void,
        strEventName: *const c_char,
        cbEvent: unsafe extern "C" fn(*mut crate::MV_EVENT_OUT_INFO, *mut c_void),
        pUser: *mut c_void,
    ) -> c_int;

    /// 开启设备指定事件
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `strEventName` - 事件名称
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_EventNotificationOn(
        handle: *mut c_void,
        strEventName: *const c_char,
    ) -> c_int;

    /// 关闭设备指定事件
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `strEventName` - 事件名称
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_EventNotificationOff(
        handle: *mut c_void,
        strEventName: *const c_char,
    ) -> c_int;

    // ========================================================================
    // Part 7: 仅 GigE 设备支持的接口
    // ========================================================================

    /// 设置枚举超时时间，仅支持 GigE 协议，范围:[1, UINT_MAX)
    ///
    /// # 参数
    /// * `nMilTimeout` - 超时时间，应为无符号整数，默认 100ms
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 在调用 MV_CC_EnumDevices 等枚举接口前使用该接口，可设置枚举GIGE设备的网卡最大超时时间（默认 100ms），
    /// 可以减少最大超时时间，来加快枚举GIGE设备的速度。
    ///
    /// 仅支持 GigEVision 设备。
    pub unsafe fn MV_GIGE_SetEnumDevTimeout(nMilTimeout: c_uint) -> c_int;

    /// 强制 IP
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nIP` - 设置的 IP
    /// * `nSubNetMask` - 子网掩码
    /// * `nDefaultGateWay` - 默认网关
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 强制设置设备网络参数（包括 IP、子网掩码、默认网关），
    /// 强制设置之后将需要重新创建设备句柄，
    /// 支持 GigEVision(MV_GIGE_DEVICE) 设备和 GenTL(MV_GENTL_GIGE_DEVICE) 设备。
    ///
    /// 如果设备为 DHCP 的状态，调用该接口强制设置设备网络参数之后设备将会重启。
    pub unsafe fn MV_GIGE_ForceIpEx(
        handle: *mut c_void,
        nIP: c_uint,
        nSubNetMask: c_uint,
        nDefaultGateWay: c_uint,
    ) -> c_int;

    /// 配置 IP 方式
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nType` - IP 类型，见 MV_IP_CFG_x
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 发送命令设置设备的 IP 方式，如 DHCP、LLA 等，
    /// 仅支持 GigEVision(MV_GIGE_DEVICE) 和 GenTl(MV_GENTL_GIGE_DEVICE) 的设备。
    pub unsafe fn MV_GIGE_SetIpConfig(handle: *mut c_void, nType: c_uint) -> c_int;

    /// 设置仅使用某种模式，type: MV_NET_TRANS_x，不设置时，默认优先使用 driver
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nType` - 网络传输模式，见 MV_NET_TRANS_x
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 通过该接口可以设置 SDK 内部优先使用的网络模式，默认优先使用驱动模式，
    /// 仅 GigEVision 设备支持。
    pub unsafe fn MV_GIGE_SetNetTransMode(handle: *mut c_void, nType: c_uint) -> c_int;

    /// 获取网络传输信息
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstInfo` - 信息结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 通过该接口可以获取网络传输相关信息，包括已接收数据大小、丢帧数量等，
    /// 在 MV_CC_StartGrabbing 开启采集之后调用。
    /// 仅 GigEVision 设备支持。
    pub unsafe fn MV_GIGE_GetNetTransInfo(
        handle: *mut c_void,
        pstInfo: *mut crate::MV_NETTRANS_INFO,
    ) -> c_int;

    /// 设置枚举命令的回复包类型
    ///
    /// # 参数
    /// * `nMode` - 回复包类型（默认广播），0-单播，1-广播
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口只对 GigE 相机有效
    pub unsafe fn MV_GIGE_SetDiscoveryMode(nMode: c_uint) -> c_int;

    /// 设置 GVSP 取流超时时间
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nMillisec` - 超时时间，默认 300ms，范围:[10 - UINT_MAX)
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后，取流动作发生前，调用该接口可以设置 GVSP 取流超时时间。
    /// GVSP 取流超时设置过短可能造成图像异常，设置过长可能造成取流时间变长。
    pub unsafe fn MV_GIGE_SetGvspTimeout(handle: *mut c_void, nMillisec: c_uint) -> c_int;

    /// 获取 GVSP 取流超时时间
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnMillisec` - 超时时间指针，以毫秒为单位（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口用于获取当前的 GVSP 取流超时时间
    pub unsafe fn MV_GIGE_GetGvspTimeout(handle: *mut c_void, pnMillisec: *mut c_uint) -> c_int;

    /// 设置 GVCP 命令超时时间
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nMillisec` - 超时时间 (ms)，默认 500ms，范围：\[0, 10000\]
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以设置 GVCP 命令超时时间
    pub unsafe fn MV_GIGE_SetGvcpTimeout(handle: *mut c_void, nMillisec: c_uint) -> c_int;

    /// 获取 GVCP 命令超时时间
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnMillisec` - 超时时间指针，以毫秒为单位（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口用于获取当前的 GVCP 超时时间
    pub unsafe fn MV_GIGE_GetGvcpTimeout(handle: *mut c_void, pnMillisec: *mut c_uint) -> c_int;

    /// 设置重传 GVCP 命令次数
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nRetryGvcpTimes` - 重传次数，范围：0-100
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口用于在 GVCP 包传输异常时，增加重传的次数，
    /// 在一定程度上可以避免设备掉线，范围为 0-100
    pub unsafe fn MV_GIGE_SetRetryGvcpTimes(handle: *mut c_void, nRetryGvcpTimes: c_uint) -> c_int;

    /// 获取重传 GVCP 命令次数
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnRetryGvcpTimes` - 重传次数指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口用于获取当前的 GVCP 重传次数，默认 3 次
    pub unsafe fn MV_GIGE_GetRetryGvcpTimes(
        handle: *mut c_void,
        pnRetryGvcpTimes: *mut c_uint,
    ) -> c_int;

    /// 获取最佳的 packet size，该接口目前只支持 GigE 设备
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    ///
    /// # 返回值
    /// 最佳 packetsize
    ///
    /// # 注意事项
    /// 获取最佳的 packet size，对应 GigEVision 设备是 SCPS。
    /// 该接口需要在 MV_CC_OpenDevice 之后、MV_CC_StartGrabbing 之前调用。
    ///
    /// 该接口不支持 CameraLink 设备、U3V 设备。
    /// 该接口不支持 GenTL 设备（协议不支持），如果是 GenTL 方式添加的网口相机，
    /// 建议根据网络实际情况配置 GevSCPSPacketSize，或者配置 1500。
    pub unsafe fn MV_CC_GetOptimalPacketSize(handle: *mut c_void) -> c_int;

    /// 设置是否打开重发包支持，及重发包设置
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `bEnable` - 是否支持重发包
    /// * `nMaxResendPercent` - 最大重发比
    /// * `nResendTimeout` - 重发超时时间，范围：0-10000ms
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 连接设备之后调用该接口可以设置重发包属性，仅 GigEVision 设备支持
    pub unsafe fn MV_GIGE_SetResend(
        handle: *mut c_void,
        bEnable: c_uint,
        nMaxResendPercent: c_uint,
        nResendTimeout: c_uint,
    ) -> c_int;

    /// 设置重传命令最大尝试次数
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nRetryTimes` - 重传命令最大尝试次数，默认 20
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口必须在调用 MV_GIGE_SetResend 开启重传包功能之后调用，否则失败且返回 MV_E_CALLORDER
    pub unsafe fn MV_GIGE_SetResendMaxRetryTimes(handle: *mut c_void, nRetryTimes: c_uint)
    -> c_int;

    /// 获取重传命令最大尝试次数
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnRetryTimes` - 重传命令最大尝试次数（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口必须在调用 MV_GIGE_SetResend 开启重传包功能之后调用，否则失败且返回 MV_E_CALLORDER
    pub unsafe fn MV_GIGE_GetResendMaxRetryTimes(
        handle: *mut c_void,
        pnRetryTimes: *mut c_uint,
    ) -> c_int;

    /// 设置同一重传包多次请求之间的时间间隔
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nMillisec` - 同一重传包多次请求之间的时间间隔，默认 10ms
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口必须在调用 MV_GIGE_SetResend 开启重传包功能之后调用，否则失败且返回 MV_E_CALLORDER
    pub unsafe fn MV_GIGE_SetResendTimeInterval(handle: *mut c_void, nMillisec: c_uint) -> c_int;

    /// 获取同一重传包多次请求之间的时间间隔
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnMillisec` - 同一重传包多次请求之间的时间间隔（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口必须在调用 MV_GIGE_SetResend 开启重传包功能之后调用，否则失败且返回 MV_E_CALLORDER
    pub unsafe fn MV_GIGE_GetResendTimeInterval(
        handle: *mut c_void,
        pnMillisec: *mut c_uint,
    ) -> c_int;

    /// 设置传输模式，可以为单播模式、组播模式等
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstTransmissionType` - 传输模式结构体
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 通过该接口可以设置传输模式为单播、组播等模式，仅 GigEVision 设备支持
    pub unsafe fn MV_GIGE_SetTransmissionType(
        handle: *mut c_void,
        pstTransmissionType: *mut crate::MV_TRANSMISSION_TYPE,
    ) -> c_int;

    /// 发出动作命令
    ///
    /// # 参数
    /// * `pstActionCmdInfo` - 动作命令信息
    /// * `pstActionCmdResults` - 动作命令返回信息列表（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 仅 GigEVision 设备支持
    pub unsafe fn MV_GIGE_IssueActionCommand(
        pstActionCmdInfo: *mut crate::MV_ACTION_CMD_INFO,
        pstActionCmdResults: *mut crate::MV_ACTION_CMD_RESULT_LIST,
    ) -> c_int;

    /// 获取组播状态
    ///
    /// # 参数
    /// * `pstDevInfo` - 设备信息结构体
    /// * `pbStatus` - 组播状态，true:组播状态，false:非组播（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口用于判断设备当前是否处于组播状态，解决客户端枚举时需要打开设备判断组播的问题。
    /// 仅支持标准 GigE Vision 设备
    pub unsafe fn MV_GIGE_GetMulticastStatus(
        pstDevInfo: *mut MV_CC_DEVICE_INFO,
        pbStatus: *mut bool,
    ) -> c_int;

    // ========================================================================
    // Part 8: 仅 CameraLink 设备支持的接口
    // ========================================================================

    /// 获取串口信息列表
    ///
    /// # 参数
    /// * `pstSerialPortList` - 串口信息列表（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口用于获取本地的串口信息。
    /// 该接口不支持 arm 和 Linux32 平台
    pub unsafe fn MV_CAML_GetSerialPortList(
        pstSerialPortList: *mut crate::MV_CAML_SERIAL_PORT_LIST,
    ) -> c_int;

    /// 设置取指定枚举串口
    ///
    /// # 参数
    /// * `pstSerialPortList` - 串口信息列表
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口用于设置枚举 CameraLink 设备的指定串口。
    /// 该接口不支持 arm 和 Linux32 平台
    pub unsafe fn MV_CAML_SetEnumSerialPorts(
        pstSerialPortList: *mut crate::MV_CAML_SERIAL_PORT_LIST,
    ) -> c_int;

    /// 设置设备波特率
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nBaudrate` - 设置的波特率值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口支持在设备未连接时调用。
    /// 通过 GenTL 协议访问设备时，需要先连接设备，才能调用该接口。
    ///
    /// 因硬件/系统/外部干扰等因素，配置高波特率可能导致通信异常，
    /// 建议配置波特率最大小于 115200。
    ///
    /// 该接口不支持 arm 和 Linux32 平台
    pub unsafe fn MV_CAML_SetDeviceBaudrate(handle: *mut c_void, nBaudrate: c_uint) -> c_int;

    /// 获取设备波特率
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnCurrentBaudrate` - 波特率信息指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口支持在设备未连接时调用。
    /// 该接口不支持 arm 和 Linux32 平台
    pub unsafe fn MV_CAML_GetDeviceBaudrate(
        handle: *mut c_void,
        pnCurrentBaudrate: *mut c_uint,
    ) -> c_int;

    /// 获取设备与主机间连接支持的波特率
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnBaudrateAblity` - 支持的波特率信息的指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 所支持波特率的或运算结果，单个数值参考 CameraParams.h 中宏定义
    ///
    /// 该接口支持在设备未连接时调用。
    /// 该接口不支持 arm 和 Linux32 平台
    pub unsafe fn MV_CAML_GetSupportBaudrates(
        handle: *mut c_void,
        pnBaudrateAblity: *mut c_uint,
    ) -> c_int;

    /// 设置串口操作等待时长
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nMillisec` - 串口操作的等待时长，单位为 ms
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口不支持 arm 和 Linux32 平台
    pub unsafe fn MV_CAML_SetGenCPTimeOut(handle: *mut c_void, nMillisec: c_uint) -> c_int;

    // ========================================================================
    // Part 9: 仅 U3V 设备支持的接口
    // ========================================================================

    /// 设置 U3V 的传输包大小
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nTransferSize` - 传输的包大小，Byte，默认为 1M，范围：>=0x400
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 增加传输包大小可以适当降低取流时的 CPU 占用率。
    /// 但不同的 PC 和不同 USB 扩展卡存在不同的兼容性，
    /// 如果该参数设置过大可能会出现取不到图像的风险。
    pub unsafe fn MV_USB_SetTransferSize(handle: *mut c_void, nTransferSize: c_uint) -> c_int;

    /// 获取 U3V 的传输包大小
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnTransferSize` - 传输的包大小指针，Byte（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口用于获取当前的 U3V 传输包大小，默认 1M
    pub unsafe fn MV_USB_GetTransferSize(handle: *mut c_void, pnTransferSize: *mut c_uint)
    -> c_int;

    /// 设置 U3V 的传输通道个数
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nTransferWays` - 传输通道个数，范围：1-10
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 用户可以根据 PC 的性能、设备出图帧率、图像大小和内存使用率等因素对该参数进行调节。
    /// 但不同的 PC 和不同的 USB 扩展卡存在不同的兼容性。
    pub unsafe fn MV_USB_SetTransferWays(handle: *mut c_void, nTransferWays: c_uint) -> c_int;

    /// 获取 U3V 的传输通道个数
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnTransferWays` - 传输通道个数指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口用于获取当前的 U3V 异步取流节点个数，
    /// U 口相机传输通道个数和像素格式对应的负载包大小相关，
    /// 通过最大异步注册长度 / 像素格式对应的负载包大小 计算得出
    pub unsafe fn MV_USB_GetTransferWays(handle: *mut c_void, pnTransferWays: *mut c_uint)
    -> c_int;

    /// 注册流异常消息回调，在打开设备之后调用
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `cbException` - 异常回调函数指针
    /// * `pUser` - 用户自定义变量
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 只支持 U3V 相机，不支持 GenTL 设备
    pub unsafe fn MV_USB_RegisterStreamExceptionCallBack(
        handle: *mut c_void,
        cbException: unsafe extern "C" fn(crate::MV_CC_STREAM_EXCEPTION_TYPE, *mut c_void),
        pUser: *mut c_void,
    ) -> c_int;

    /// 设置 U3V 的事件缓存节点个数
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nEventNodeNum` - 事件缓存节点个数，范围：1-64
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口用于设置当前的 U3V 事件缓存节点个数，默认情况下为 5 个
    pub unsafe fn MV_USB_SetEventNodeNum(handle: *mut c_void, nEventNodeNum: c_uint) -> c_int;

    /// 设置 U3V 的同步读写超时时间，范围为:[1000, UINT_MAX)
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nMills` - 设置同步读写超时时间，默认时间为 1000ms
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 增加设置同步读取时间接口，兼容部分相机配置参数很慢，超过 1000ms 的情况
    pub unsafe fn MV_USB_SetSyncTimeOut(handle: *mut c_void, nMills: c_uint) -> c_int;

    /// 获取 U3V 相机同步读写超时时间
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnMills` - 获取的超时时间 (ms)（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口用于获取当前的 U3V 同步读写超时时间大小，默认 1000ms
    pub unsafe fn MV_USB_GetSyncTimeOut(handle: *mut c_void, pnMills: *mut c_uint) -> c_int;

    // ========================================================================
    // Part 10: GenTL 相关接口
    // ========================================================================

    /// 通过 GenTL 枚举 Interfaces
    ///
    /// # 参数
    /// * `pstIFList` - Interfaces 列表（输出参数）
    /// * `strGenTLPath` - GenTL 的 cti 文件路径
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// Interfaces 列表的内存是在 SDK 内部分配的，多线程调用该接口时会进行设备列表内存的释放和申请，
    /// 建议尽量避免多线程枚举操作。
    ///
    /// 暂不支持工业相机 SDK 直接调用 MvProducerU3V.cti 和 MvProducerGEV.cti，
    /// 支持调用其他.cti
    pub unsafe fn MV_CC_EnumInterfacesByGenTL(
        pstIFList: *mut crate::MV_GENTL_IF_INFO_LIST,
        strGenTLPath: *const c_char,
    ) -> c_int;

    /// 卸载 cti 库
    ///
    /// # 参数
    /// * `pGenTLPath` - 枚举卡时加载的 cti 文件路径
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 卸载前需要保证通过该 cti 枚举出的相机已全部关闭，否则报错前置条件错误
    pub unsafe fn MV_CC_UnloadGenTLLibrary(pGenTLPath: *const c_char) -> c_int;

    /// 通过 GenTL Interface 枚举设备
    ///
    /// # 参数
    /// * `pstIFInfo` - Interface 信息
    /// * `pstDevList` - 设备列表（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 设备列表的内存是在 SDK 内部分配的，多线程调用该接口时会进行设备列表内存的释放和申请，
    /// 建议尽量避免多线程枚举操作
    pub unsafe fn MV_CC_EnumDevicesByGenTL(
        pstIFInfo: *mut crate::MV_GENTL_IF_INFO,
        pstDevList: *mut crate::MV_GENTL_DEV_INFO_LIST,
    ) -> c_int;

    /// 通过 GenTL 设备信息创建设备句柄
    ///
    /// # 参数
    /// * `handle` - 设备句柄（输出参数）
    /// * `pstDevInfo` - 设备信息结构体指针
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 根据输入的设备信息，创建库内部必须的资源和初始化内部模块
    pub unsafe fn MV_CC_CreateHandleByGenTL(
        handle: *mut *mut c_void,
        pstDevInfo: *const crate::MV_GENTL_DEV_INFO,
    ) -> c_int;

    // ========================================================================
    // Part 11: 图像保存、格式转换等相关接口
    // ========================================================================

    /// 保存图片，支持 Bmp 和 Jpeg
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstSaveParam` - 保存图片参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 通过该接口可以将从设备采集到的原始图像数据转换成 JPEG 或者 BMP 等格式并存放在指定内存中，
    /// 然后用户可以将转换之后的数据直接保存成图片文件。
    ///
    /// 该接口调用无接口顺序要求，有图像源数据就可以进行转换，
    /// 可以先调用 MV_CC_GetOneFrameTimeout 或者 MV_CC_RegisterImageCallBackEx 设置回调函数，
    /// 获取一帧图像数据，然后再通过该接口转换格式。
    ///
    /// 该接口支持图像宽、高、总长最大至 UINT_MAX
    /// JPEG格式最大支持宽高为 65500
    pub unsafe fn MV_CC_SaveImageEx3(
        handle: *mut c_void,
        pstSaveParam: *mut crate::MV_SAVE_IMAGE_PARAM_EX3,
    ) -> c_int;

    /// 保存图像到文件
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstSaveFileParam` - 保存图片文件参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口支持 BMP/JPEG/PNG/TIFF。
    /// 该接口支持图像宽、高、总长最大至 UINT_MAX
    /// JPEG格式最大支持宽高为 65500
    pub unsafe fn MV_CC_SaveImageToFileEx(
        handle: *mut c_void,
        pstSaveFileParam: *mut crate::MV_SAVE_IMAGE_TO_FILE_PARAM_EX,
    ) -> c_int;

    /// 保存图像到文件（扩展版本 2，支持 4G 以上超大图）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstImage` - 图像信息
    /// * `pSaveImageParam` - 存图参数
    /// * `pcImagePath` - 存图路径
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口支持 4G 以上超大图的 PNG/TIFF 存图，非超大图像支持 BMP/JPEG/TIFF/PNG
    /// JPEG格式最大支持宽高为 65500
    pub unsafe fn MV_CC_SaveImageToFileEx2(
        handle: *mut c_void,
        pstImage: *mut crate::MV_CC_IMAGE,
        pSaveImageParam: *mut crate::MV_CC_SAVE_IMAGE_PARAM,
        pcImagePath: *const c_char,
    ) -> c_int;

    /// 图像旋转
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstRotateParam` - 图像旋转参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口只支持 MONO8/RGB24/BGR24 格式数据的 90/180/270 度旋转
    pub unsafe fn MV_CC_RotateImage(
        handle: *mut c_void,
        pstRotateParam: *mut crate::MV_CC_ROTATE_IMAGE_PARAM,
    ) -> c_int;

    /// 图像翻转
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstFlipParam` - 图像翻转参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口只支持 MONO8/RGB24/BGR24 格式数据的垂直和水平翻转
    pub unsafe fn MV_CC_FlipImage(
        handle: *mut c_void,
        pstFlipParam: *mut crate::MV_CC_FLIP_IMAGE_PARAM,
    ) -> c_int;

    /// 像素格式转换
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstCvtParam` - 像素格式转换参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 通过将接口可以将从设备采集到的原始图像数据转换成用户所需的像素格式并存放在指定内存中。
    ///
    /// 该接口调用无接口顺序要求，有图像源数据就可以进行转换，
    /// 可以先调用 MV_CC_GetOneFrameTimeout 或者 MV_CC_RegisterImageCallBackEx 设置回调函数，
    /// 获取一帧图像数据，然后再通过该接口转换格式。
    ///
    /// 如果设备当前采集图像是 JPEG 压缩的格式，则不支持调用该接口进行转换。
    ///
    /// 该接口支持图像宽、高、总长最大至 UINT_MAX
    pub unsafe fn MV_CC_ConvertPixelTypeEx(
        handle: *mut c_void,
        pstCvtParam: *mut crate::MV_CC_PIXEL_CONVERT_PARAM_EX,
    ) -> c_int;

    /// 设置插值算法类型
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nBayerCvtQuality` - Bayer 的插值方法  0-快速 1-均衡（默认为均衡）2-最优 3-最优 +
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 设置内部图像转换接口的 Bayer 插值算法类型参数，
    /// MV_CC_ConvertPixelTypeEx、MV_CC_GetImageForRGB/BGR 接口内部使用的插值算法是该接口所设定的
    pub unsafe fn MV_CC_SetBayerCvtQuality(handle: *mut c_void, nBayerCvtQuality: c_uint) -> c_int;

    /// 插值算法平滑使能设置
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `bFilterEnable` - 平滑使能 (默认关闭)
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 设置内部图像转换接口的贝尔插值平滑使能参数，
    /// MV_CC_ConvertPixelTypeEx、MV_CC_SaveImageEx3 接口内部使用的插值算法是该接口所设定的
    pub unsafe fn MV_CC_SetBayerFilterEnable(handle: *mut c_void, bFilterEnable: bool) -> c_int;

    /// 设置 Bayer 格式的 Gamma 值
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `fBayerGammaValue` - Gamma 值：0.1 ~ 4.0
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 设置该值后，在调用 MV_CC_ConvertPixelTypeEx、MV_CC_SaveImageEx3、MV_CC_SaveImageToFileEx 接口
    /// 将 Bayer8/10/12/16 格式转成 RGB24/48，RGBA32/64，BGR24/48，BGRA32/64 时起效
    pub unsafe fn MV_CC_SetBayerGammaValue(handle: *mut c_void, fBayerGammaValue: f32) -> c_int;

    /// 设置 Mono8/Bayer8/10/12/16 格式的 Gamma 值
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `enSrcPixelType` - 像素格式，支持 PixelType_Gvsp_Mono8, Bayer8/10/12/16
    /// * `fGammaValue` - Gamma 值：0.1 ~ 4.0
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 设置 Mono8 的 gamma 值后，在调用 MV_CC_ConvertPixelTypeEx 接口将 Mono8 转成 Mono8 时 gamma 值起效。
    ///
    /// 设置 Bayer8/10/12/16 的 gamma 值后，在调用 MV_CC_ConvertPixelTypeEx、MV_CC_SaveImageToFileEx、
    /// MV_CC_SaveImageEx3 接口将 Bayer8/10/12/16 转 RGB24/48，RGBA32/64，BGR24/48，BGRA32/64 时 gamma 值起效。
    ///
    /// 该接口兼容 MV_CC_SetBayerGammaValue 接口，新增支持 Mono8 像素格式
    pub unsafe fn MV_CC_SetGammaValue(
        handle: *mut c_void,
        enSrcPixelType: crate::MvGvspPixelType,
        fGammaValue: f32,
    ) -> c_int;

    /// 设置 Bayer 格式的 Gamma 信息
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstGammaParam` - Gamma 信息
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 设置该信息后，在调用 MV_CC_ConvertPixelTypeEx、MV_CC_SaveImageEx3、MV_CC_SaveImageToFileEx 接口
    /// 将 Bayer8/10/12/16 格式转成 RGB24/48，RGBA32/64，BGR24/48，BGRA32/64 时 Gamma 值起效
    pub unsafe fn MV_CC_SetBayerGammaParam(
        handle: *mut c_void,
        pstGammaParam: *mut crate::MV_CC_GAMMA_PARAM,
    ) -> c_int;

    /// 设置 Bayer 格式的 CCM 使能和矩阵，量化系数默认 1024
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstCCMParam` - CCM 参数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 开启 CCM 并设置 CCM 矩阵后，在调用 MV_CC_ConvertPixelTypeEx、MV_CC_SaveImageEx3 接口
    /// 将 Bayer8/10/12/16 格式转成 RGB24/48，RGBA32/64，BGR24/48，BGRA32/64 时起效
    pub unsafe fn MV_CC_SetBayerCCMParam(
        handle: *mut c_void,
        pstCCMParam: *mut crate::MV_CC_CCM_PARAM,
    ) -> c_int;

    /// 设置 Bayer 格式的 CCM 使能和矩阵
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstCCMParam` - CCM 参数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 开启 CCM 并设置 CCM 矩阵后，在调用 MV_CC_ConvertPixelTypeEx、MV_CC_SaveImageEx3 接口
    /// 将 Bayer8/10/12/16 格式转成 RGB24/48，RGBA32/64，BGR24/48，BGRA32/64 时起效
    pub unsafe fn MV_CC_SetBayerCCMParamEx(
        handle: *mut c_void,
        pstCCMParam: *mut crate::MV_CC_CCM_PARAM_EX,
    ) -> c_int;

    /// 图像对比度调节
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstContrastParam` - 对比度调节参数（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_ImageContrast(
        handle: *mut c_void,
        pstContrastParam: *mut crate::MV_CC_CONTRAST_PARAM,
    ) -> c_int;

    /// 图像去紫边
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstPurpleFringingParam` - 去紫边参数（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 像素格式仅支持 PixelType_Gvsp_RGB8_Packed 和 PixelType_Gvsp_BGR8_Packed
    pub unsafe fn MV_CC_PurpleFringing(
        handle: *mut c_void,
        pstPurpleFringingParam: *mut crate::MV_CC_PURPLE_FRINGING_PARAM,
    ) -> c_int;

    /// 设置 ISP 参数
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstParam` - ISP 配置参数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_SetISPConfig(
        handle: *mut c_void,
        pstParam: *mut crate::MV_CC_ISP_CONFIG_PARAM,
    ) -> c_int;

    /// 对图像进行 ISP 算法处理
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstInputImage` - 输入图像结构体
    /// * `pstOutputImage` - 输出图像结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 需要先调用 MV_CC_SetISPConfig 传入配置文件，配置文件由 ISP 工具生成
    pub unsafe fn MV_CC_ISPProcess(
        handle: *mut c_void,
        pstInputImage: *mut crate::MV_CC_IMAGE,
        pstOutputImage: *mut crate::MV_CC_IMAGE,
    ) -> c_int;

    /// 无损解码
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstDecodeParam` - 无损解码参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 将从相机中取到的无损压缩码流解码成裸数据，同时支持解析当前相机实时图像的水印信息
    /// （如果输入的无损码流不是当前相机或者不是实时取流的，则水印解析可能异常）。
    ///
    /// 若解码失败，请检查以下情况：
    /// （1）需要 CPU 支持 SSE AVX 指令集
    /// （2）若当前帧异常（丢包等），可能导致解码异常
    /// （3）相机出图异常，即使不丢包也会异常
    pub unsafe fn MV_CC_HB_Decode(
        handle: *mut c_void,
        pstDecodeParam: *mut crate::MV_CC_HB_DECODE_PARAM,
    ) -> c_int;

    /// 在图像上绘制矩形框辅助线
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pRectInfo` - 矩形辅助线的信息
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口仅支持 windows 平台
    pub unsafe fn MV_CC_DrawRect(
        handle: *mut c_void,
        pRectInfo: *mut crate::MVCC_RECT_INFO,
    ) -> c_int;

    /// 在图像上绘制圆形辅助线
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pCircleInfo` - 圆形辅助线的信息
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口仅支持 windows 平台
    pub unsafe fn MV_CC_DrawCircle(
        handle: *mut c_void,
        pCircleInfo: *mut crate::MVCC_CIRCLE_INFO,
    ) -> c_int;

    /// 在图像上绘制线条
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pLinesInfo` - 线条辅助线信息
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口仅支持 windows 平台
    pub unsafe fn MV_CC_DrawLines(
        handle: *mut c_void,
        pLinesInfo: *mut crate::MVCC_LINES_INFO,
    ) -> c_int;

    /// 开始录像
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstRecordParam` - 录像参数结构体
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口最大支持 Width*Height 为 8000*8000 大小，否则会导致调用 MV_CC_InputOneFrame 接口错误
    pub unsafe fn MV_CC_StartRecord(
        handle: *mut c_void,
        pstRecordParam: *mut crate::MV_CC_RECORD_PARAM,
    ) -> c_int;

    /// 输入录像数据
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstInputFrameInfo` - 录像数据结构体
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_InputOneFrame(
        handle: *mut c_void,
        pstInputFrameInfo: *mut crate::MV_CC_INPUT_FRAME_INFO,
    ) -> c_int;

    /// 停止录像
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_StopRecord(handle: *mut c_void) -> c_int;

    /// 重构图像 (用于分时曝光功能)
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstReconstructParam` - 重构图像参数（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 图像分割支持任意像素格式，图像分割应与线阵相机的"MultiLightControl"节点搭配使用，
    /// 该节点可设置多个不同的曝光值，如 MultiLightControl=2，
    /// 相机会将两个不同曝光值所对应的两张图像交叠合并为一张图像 (实际高度为两张图像的高度) 发送给上层应用程序，
    /// 调用该接口并传入分时曝光值 nExposureNum 为 2，可将相机发送的一张图像分割为 2 张图像，
    /// 这两张图像分别对应一个曝光值。
    ///
    /// 若使用普通相机或未打开线阵相机的"MultiLightControl"节点，则图像分割无意义，
    /// 只是将图像按行分割为 2，3，4 张图像，每张图像的高度变为原图像的 1/2，1/3，1/4(由 nExposureNum 决定)
    pub unsafe fn MV_CC_ReconstructImage(
        handle: *mut c_void,
        pstReconstructParam: *mut crate::MV_RECONSTRUCT_IMAGE_PARAM,
    ) -> c_int;

    /// 分配对齐内存
    ///
    /// # 参数
    /// * `nBufSize` - 分配内存的长度
    /// * `nAlignment` - 内存对齐字节数 (必须是大于 0，并且是 2 的整数次幂)
    ///
    /// # 返回值
    /// 成功返回申请内存地址，失败返回 NULL
    pub unsafe fn MV_CC_AllocAlignedBuffer(nBufSize: u64, nAlignment: c_uint) -> *mut c_void;

    /// 对齐内存释放
    ///
    /// # 参数
    /// * `pBuffer` - 内存地址
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 对齐内存的释放，搭配 MV_CC_AllocAlignedBuffer 使用
    pub unsafe fn MV_CC_FreeAlignedBuffer(pBuffer: *mut c_void) -> c_int;

    /// 获取设备 payload 大小（payload 包含图像数据和 Chunk 数据）和内存对其方式，
    /// 用于 SDK 外部注册缓存时，应用层分配足够的缓存及正确的内存对齐方式
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnPayloadSize` - 负载长度（输出参数）
    /// * `pnAlignment` - 负载内存对齐的字节数（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_GetPayloadSize(
        handle: *mut c_void,
        pnPayloadSize: *mut u64,
        pnAlignment: *mut c_uint,
    ) -> c_int;

    /// 应用程序分配缓存，并注册到 SDK 内部，供 SDK 使用
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pBuffer` - 内存地址
    /// * `nBufSize` - 内存长度
    /// * `pUser` - 用户指针
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 可以使用 MV_CC_GetPayloadSize 获取缓存大小，并使用 MV_CC_AllocAlignedBuffer 分配空间，
    /// 之后进行 MV_CC_RegisterBuffer 注册。
    ///
    /// 注册的缓存需要由应用层通知 SDK 取消注册（MV_CC_UnRegisterBuffer）后，
    /// 进行释放（MV_CC_FreeAlignedBuffer）。
    ///
    /// 使用该接口后，仅仅支持 MV_CC_GetImageBuffer/MV_CC_FreeImageBuffer/
    /// MV_CC_RegisterImageCallBackEx 获取图像，不支持其他接口获取图像。
    ///
    /// 使用该接口后，如果之前配置了 SDK 内部节点（MV_CC_SetImageNodeNum）无效。
    ///
    /// 双 USB 接口相机要求至少注册 3 块空间到 SDK 内部;
    /// 其他相机暂无限制，但是为了避免缓存不足，请配置足够的缓存到底层
    pub unsafe fn MV_CC_RegisterBuffer(
        handle: *mut c_void,
        pBuffer: *mut c_void,
        nBufSize: u64,
        pUser: *mut c_void,
    ) -> c_int;

    /// 外部内存取消 SDK 内部注册
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pBuffer` - 外部内存地址
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    pub unsafe fn MV_CC_UnRegisterBuffer(handle: *mut c_void, pBuffer: *mut c_void) -> c_int;

} // unsafe extern "C"
