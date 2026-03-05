//! 相机参数定义
//!
//! 该模块包含了海康威视 MVS SDK 的相机参数相关的结构体和枚举定义

use crate::pixel_type::MvGvspPixelType;
use core::ffi::{c_char, c_float, c_int, c_uchar, c_uint, c_ushort, c_void};

/// 最大数据信息缓冲区大小
pub const INFO_MAX_BUFFER_SIZE: usize = 64;

/// 最多支持的传输层实例个数
pub const MV_MAX_TLS_NUM: usize = 8;

/// 最大支持的设备个数
pub const MV_MAX_DEVICE_NUM: usize = 256;

/// 最大支持的采集卡数量
pub const MV_MAX_INTERFACE_NUM: usize = 64;

/// 最大支持的 GenTL 接口数量
pub const MV_MAX_GENTL_IF_NUM: usize = 256;

/// 最大支持的 GenTL 设备数量
pub const MV_MAX_GENTL_DEV_NUM: usize = 256;

/// 设备的访问模式 - 独占权限
pub const MV_ACCESS_Exclusive: u32 = 1;

/// 设备的访问模式 - 可从 5 模式下抢占权限，然后以独占权限打开
pub const MV_ACCESS_ExclusiveWithSwitch: u32 = 2;

/// 设备的访问模式 - 控制权限
pub const MV_ACCESS_Control: u32 = 3;

/// 设备的访问模式 - 可从 5 的模式下抢占权限，然后以控制权限打开
pub const MV_ACCESS_ControlWithSwitch: u32 = 4;

/// 设备的访问模式 - 以可被抢占的控制权限打开
pub const MV_ACCESS_ControlSwitchEnable: u32 = 5;

/// 设备的访问模式 - 可从 5 的模式下抢占权限，然后以可被抢占的控制权限打开
pub const MV_ACCESS_ControlSwitchEnableWithKey: u32 = 6;

/// 设备的访问模式 - 读模式打开设备，适用于控制权限下
pub const MV_ACCESS_Monitor: u32 = 7;

/// 未知设备类型
pub const MV_UNKNOW_DEVICE: u32 = 0x00000000;

/// GigE 设备
pub const MV_GIGE_DEVICE: u32 = 0x00000001;

/// 1394-a/b 设备
pub const MV_1394_DEVICE: u32 = 0x00000002;

/// USB 设备
pub const MV_USB_DEVICE: u32 = 0x00000004;

/// CameraLink 设备
pub const MV_CAMERALINK_DEVICE: u32 = 0x00000008;

/// 虚拟 GigE 设备
pub const MV_VIR_GIGE_DEVICE: u32 = 0x00000010;

/// 虚拟 USB 设备
pub const MV_VIR_USB_DEVICE: u32 = 0x00000020;

/// GenTL GigE 设备
pub const MV_GENTL_GIGE_DEVICE: u32 = 0x00000040;

/// GenTL CameraLink 相机设备
pub const MV_GENTL_CAMERALINK_DEVICE: u32 = 0x00000080;

/// GenTL CoaXPress 设备
pub const MV_GENTL_CXP_DEVICE: u32 = 0x00000100;

/// GenTL XoF 设备
pub const MV_GENTL_XOF_DEVICE: u32 = 0x00000200;

/// GenTL 虚拟设备
pub const MV_GENTL_VIR_DEVICE: u32 = 0x00000800;

/// GigE Vision 采集卡
pub const MV_GIGE_INTERFACE: u32 = 0x00000001;

/// Camera Link 采集卡
pub const MV_CAMERALINK_INTERFACE: u32 = 0x00000004;

/// CoaXPress 采集卡
pub const MV_CXP_INTERFACE: u32 = 0x00000008;

/// XoFLink 采集卡
pub const MV_XOF_INTERFACE: u32 = 0x00000010;

/// 虚拟采集卡
pub const MV_VIR_INTERFACE: u32 = 0x00000020;

/// 网络流量和丢包信息
pub const MV_MATCH_TYPE_NET_DETECT: u32 = 0x00000001;

/// host 接收到来自 U3V 设备的字节总数
pub const MV_MATCH_TYPE_USB_DETECT: u32 = 0x00000002;

/// GigEVision IP 配置 - 静态
pub const MV_IP_CFG_STATIC: u32 = 0x05000000;

/// GigEVision IP 配置 - DHCP
pub const MV_IP_CFG_DHCP: u32 = 0x06000000;

/// GigEVision IP 配置 - LLA
pub const MV_IP_CFG_LLA: u32 = 0x04000000;

/// GigEVision 网络传输模式 - 驱动
pub const MV_NET_TRANS_DRIVER: u32 = 0x00000001;

/// GigEVision 网络传输模式 - Socket
pub const MV_NET_TRANS_SOCKET: u32 = 0x00000002;

/// CameraLink 波特率 - 9600
pub const MV_CAML_BAUDRATE_9600: u32 = 0x00000001;

/// CameraLink 波特率 - 19200
pub const MV_CAML_BAUDRATE_19200: u32 = 0x00000002;

/// CameraLink 波特率 - 38400
pub const MV_CAML_BAUDRATE_38400: u32 = 0x00000004;

/// CameraLink 波特率 - 57600
pub const MV_CAML_BAUDRATE_57600: u32 = 0x00000008;

/// CameraLink 波特率 - 115200
pub const MV_CAML_BAUDRATE_115200: u32 = 0x00000010;

/// CameraLink 波特率 - 230400
pub const MV_CAML_BAUDRATE_230400: u32 = 0x00000020;

/// CameraLink 波特率 - 460800
pub const MV_CAML_BAUDRATE_460800: u32 = 0x00000040;

/// CameraLink 波特率 - 921600
pub const MV_CAML_BAUDRATE_921600: u32 = 0x00000080;

/// CameraLink 波特率 - 最大值
pub const MV_CAML_BAUDRATE_AUTOMAX: u32 = 0x40000000;

/// 异常消息类型 - 设备断开连接
pub const MV_EXCEPTION_DEV_DISCONNECT: u32 = 0x00008001;

/// 异常消息类型 - SDK 与驱动版本不匹配
pub const MV_EXCEPTION_VERSION_CHECK: u32 = 0x00008002;

/// 最大节点个数
pub const MV_MAX_NODE_NUM: usize = 1024;

/// 节点名称的最大长度
pub const MV_MAX_NODE_NAME_LEN: usize = 64;

/// 最大错误个数
pub const MV_MAX_NODE_ERROR_NUM: usize = 64;

/// 最大 XML 符号数
pub const MV_MAX_XML_SYMBOLIC_NUM: usize = 64;

/// 最大枚举条目对应的符号数量
pub const MV_MAX_ENUM_SYMBOLIC_NUM: usize = 256;

/// 最大枚举条目对应的符号长度
pub const MV_MAX_SYMBOLIC_LEN: usize = 64;

/// 分时曝光时最多将源图像拆分的个数
pub const MV_MAX_SPLIT_NUM: usize = 8;

/// 最大 Event 名称长度
pub const MAX_EVENT_NAME_SIZE: usize = 128;

/// 排序方式
#[repr(u32)]
pub enum MV_SORT_METHOD {
    /// 按序列号排序
    SortMethod_SerialNumber = 0,
    /// 按用户自定义名字排序
    SortMethod_UserID = 1,
    /// 按当前 IP 地址排序（升序，只对 GEV 相机有效）
    SortMethod_CurrentIP_ASC = 2,
    /// 按当前 IP 地址排序（降序，只对 GEV 相机有效）
    SortMethod_CurrentIP_DESC = 3,
}

/// 图片保存格式
#[repr(u32)]
pub enum MV_SAVE_IMAGE_TYPE {
    /// 未定义的图像格式
    MV_Image_Undefined = 0,
    /// BMP 图像格式
    MV_Image_Bmp = 1,
    /// JPEG 图像格式
    MV_Image_Jpeg = 2,
    /// PNG 图像格式
    MV_Image_Png = 3,
    /// TIFF 图像格式
    MV_Image_Tif = 4,
}

/// 旋转角度
#[repr(u32)]
pub enum MV_IMG_ROTATION_ANGLE {
    /// 旋转 90 度
    MV_IMAGE_ROTATE_90 = 1,
    /// 旋转 180 度
    MV_IMAGE_ROTATE_180 = 2,
    /// 旋转 270 度
    MV_IMAGE_ROTATE_270 = 3,
}

/// 翻转类型
#[repr(u32)]
pub enum MV_IMG_FLIP_TYPE {
    /// 垂直翻转
    MV_FLIP_VERTICAL = 1,
    /// 水平翻转
    MV_FLIP_HORIZONTAL = 2,
}

/// Gamma 类型
#[repr(u32)]
pub enum MV_CC_GAMMA_TYPE {
    /// 不启用
    MV_CC_GAMMA_TYPE_NONE = 0,
    /// Gamma 值
    MV_CC_GAMMA_TYPE_VALUE = 1,
    /// Gamma 曲线
    MV_CC_GAMMA_TYPE_USER_CURVE = 2,
    /// linear RGB to sRGB
    MV_CC_GAMMA_TYPE_LRGB2SRGB = 3,
    /// sRGB to linear RGB
    MV_CC_GAMMA_TYPE_SRGB2LRGB = 4,
}

/// 录像格式定义
#[repr(u32)]
pub enum MV_RECORD_FORMAT_TYPE {
    /// 未定义的录像格式
    MV_FormatType_Undefined = 0,
    /// AVI 录像格式
    MV_FormatType_AVI = 1,
}

/// 采集模式
#[repr(u32)]
pub enum MV_CAM_ACQUISITION_MODE {
    /// 单帧模式
    MV_ACQ_MODE_SINGLE = 0,
    /// 多帧模式
    MV_ACQ_MODE_MUTLI = 1,
    /// 持续采集模式
    MV_ACQ_MODE_CONTINUOUS = 2,
}

/// 增益模式
#[repr(u32)]
pub enum MV_CAM_GAIN_MODE {
    /// 关闭
    MV_GAIN_MODE_OFF = 0,
    /// 一次
    MV_GAIN_MODE_ONCE = 1,
    /// 连续
    MV_GAIN_MODE_CONTINUOUS = 2,
}

/// 曝光模式
#[repr(u32)]
pub enum MV_CAM_EXPOSURE_MODE {
    /// 时间
    MV_EXPOSURE_MODE_TIMED = 0,
    /// 触发脉冲宽度
    MV_EXPOSURE_MODE_TRIGGER_WIDTH = 1,
}

/// 自动曝光模式
#[repr(u32)]
pub enum MV_CAM_EXPOSURE_AUTO_MODE {
    /// 关闭
    MV_EXPOSURE_AUTO_MODE_OFF = 0,
    /// 一次
    MV_EXPOSURE_AUTO_MODE_ONCE = 1,
    /// 连续
    MV_EXPOSURE_AUTO_MODE_CONTINUOUS = 2,
}

/// 触发模式
#[repr(u32)]
pub enum MV_CAM_TRIGGER_MODE {
    /// 关闭
    MV_TRIGGER_MODE_OFF = 0,
    /// 打开
    MV_TRIGGER_MODE_ON = 1,
}

/// Gamma 选择器
#[repr(u32)]
pub enum MV_CAM_GAMMA_SELECTOR {
    /// 用户
    MV_GAMMA_SELECTOR_USER = 1,
    /// sRGB
    MV_GAMMA_SELECTOR_SRGB = 2,
}

/// 白平衡
#[repr(u32)]
pub enum MV_CAM_BALANCEWHITE_AUTO {
    /// 关闭
    MV_BALANCEWHITE_AUTO_OFF = 0,
    /// 一次
    MV_BALANCEWHITE_AUTO_ONCE = 2,
    /// 连续
    MV_BALANCEWHITE_AUTO_CONTINUOUS = 1,
}

/// 触发源
#[repr(u32)]
pub enum MV_CAM_TRIGGER_SOURCE {
    /// Line0
    MV_TRIGGER_SOURCE_LINE0 = 0,
    /// Line1
    MV_TRIGGER_SOURCE_LINE1 = 1,
    /// Line2
    MV_TRIGGER_SOURCE_LINE2 = 2,
    /// Line3
    MV_TRIGGER_SOURCE_LINE3 = 3,
    /// Counter0
    MV_TRIGGER_SOURCE_COUNTER0 = 4,
    /// 软触发
    MV_TRIGGER_SOURCE_SOFTWARE = 7,
    /// 变频器
    MV_TRIGGER_SOURCE_FrequencyConverter = 8,
}

/// U3V 流异常类型
#[repr(u32)]
pub enum MV_CC_STREAM_EXCEPTION_TYPE {
    /// 异常的图像，该帧被丢弃
    MV_CC_STREAM_EXCEPTION_ABNORMAL_IMAGE = 0x4001,
    /// 缓存列表溢出，清除最旧的一帧
    MV_CC_STREAM_EXCEPTION_LIST_OVERFLOW = 0x4002,
    /// 缓存列表为空，该帧被丢弃
    MV_CC_STREAM_EXCEPTION_LIST_EMPTY = 0x4003,
    /// 断流恢复
    MV_CC_STREAM_EXCEPTION_RECONNECTION = 0x4004,
    /// 断流，恢复失败，取流被中止
    MV_CC_STREAM_EXCEPTION_DISCONNECTED = 0x4005,
    /// 设备异常，取流被中止
    MV_CC_STREAM_EXCEPTION_DEVICE = 0x4006,
}

/// Gige 的传输类型
#[repr(u32)]
pub enum MV_GIGE_TRANSMISSION_TYPE {
    /// 表示单播 (默认)
    MV_GIGE_TRANSTYPE_UNICAST = 0x0,
    /// 表示组播 (组播 IP 范围 [224.*.*.*-239.*.*.*])
    MV_GIGE_TRANSTYPE_MULTICAST = 0x1,
    /// 表示局域网内广播，暂不支持
    MV_GIGE_TRANSTYPE_LIMITEDBROADCAST = 0x2,
    /// 表示子网内广播，暂不支持
    MV_GIGE_TRANSTYPE_SUBNETBROADCAST = 0x3,
    /// 表示从设备获取，暂不支持
    MV_GIGE_TRANSTYPE_CAMERADEFINED = 0x4,
    /// 表示用户自定义应用端接收图像数据 Port 号
    MV_GIGE_TRANSTYPE_UNICAST_DEFINED_PORT = 0x5,
    /// 表示设置了单播，但本实例不接收图像数据
    MV_GIGE_TRANSTYPE_UNICAST_WITHOUT_RECV = 0x00010000,
    /// 表示组播模式，但本实例不接收图像数据
    MV_GIGE_TRANSTYPE_MULTICAST_WITHOUT_RECV = 0x00010001,
}

/// 每个节点对应的接口类型
#[repr(u32)]
pub enum MV_XML_InterfaceType {
    /// IValue interface
    IFT_IValue = 0,
    /// IBase interface
    IFT_IBase = 1,
    /// IInteger interface
    IFT_IInteger = 2,
    /// IBoolean interface
    IFT_IBoolean = 3,
    /// ICommand interface
    IFT_ICommand = 4,
    /// IFloat interface
    IFT_IFloat = 5,
    /// IString interface
    IFT_IString = 6,
    /// IRegister interface
    IFT_IRegister = 7,
    /// ICategory interface
    IFT_ICategory = 8,
    /// IEnumeration interface
    IFT_IEnumeration = 9,
    /// IEnumEntry interface
    IFT_IEnumEntry = 10,
    /// IPort interface
    IFT_IPort = 11,
}

/// 节点的访问模式
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MV_XML_AccessMode {
    /// 不可实现
    AM_NI = 0,
    /// 不可用
    AM_NA = 1,
    /// 只写
    AM_WO = 2,
    /// 只读
    AM_RO = 3,
    /// 读写
    AM_RW = 4,
    /// 未定义
    AM_Undefined = 5,
    /// 内部用于 AccessMode 循环检测
    AM_CycleDetect = 6,
}

/// 导入参数报错时的原因，错误码
#[repr(u32)]
pub enum MVCC_NODE_ERR_TYPE {
    /// 节点不存在
    MVCC_NODE_ERR_NODE_INVALID = 1,
    /// 访问条件错误，通常是节点不可读写
    MVCC_NODE_ERR_ACCESS = 2,
    /// 写入越界，超出该节点支持的范围
    MVCC_NODE_ERR_OUT_RANGE = 3,
    /// 校验失败，通常是写入的值与文件中的值不匹配
    MVCC_NODE_ERR_VERIFY_FAILD = 4,
    /// 其它错误，可查阅日志
    MVCC_NODE_ERR_OTHER = 100,
}

/// 图像重构的方式
#[repr(u32)]
pub enum MV_IMAGE_RECONSTRUCTION_METHOD {
    /// 源图像单行拆分成多张图像
    MV_SPLIT_BY_LINE = 1,
}

/// GigE 设备信息
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MV_GIGE_DEVICE_INFO {
    /// IP 配置选项
    pub nIpCfgOption: c_uint,
    /// 当前 IP 配置
    pub nIpCfgCurrent: c_uint,
    /// 当前 IP 地址
    pub nCurrentIp: c_uint,
    /// 当前子网掩码
    pub nCurrentSubNetMask: c_uint,
    /// 当前网关
    pub nDefultGateWay: c_uint,
    /// 制造商名称
    pub chManufacturerName: [c_uchar; 32],
    /// 型号名称
    pub chModelName: [c_uchar; 32],
    /// 设备版本
    pub chDeviceVersion: [c_uchar; 32],
    /// 制造商的具体信息
    pub chManufacturerSpecificInfo: [c_uchar; 48],
    /// 序列号
    pub chSerialNumber: [c_uchar; 16],
    /// 用户自定义名称
    pub chUserDefinedName: [c_uchar; 16],
    /// 网口 IP 地址
    pub nNetExport: c_uint,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// USB 设备信息
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MV_USB3_DEVICE_INFO {
    /// 控制输入端点
    pub CrtlInEndPoint: c_uchar,
    /// 控制输出端点
    pub CrtlOutEndPoint: c_uchar,
    /// 流端点
    pub StreamEndPoint: c_uchar,
    /// 事件端点
    pub EventEndPoint: c_uchar,
    /// 供应商 ID 号
    pub idVendor: c_ushort,
    /// 产品 ID 号
    pub idProduct: c_ushort,
    /// 设备索引号
    pub nDeviceNumber: c_uint,
    /// 设备 GUID 号
    pub chDeviceGUID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 供应商名字
    pub chVendorName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 型号名字
    pub chModelName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 家族名字
    pub chFamilyName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 设备版本
    pub chDeviceVersion: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 制造商名字
    pub chManufacturerName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 序列号
    pub chSerialNumber: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 用户自定义名字
    pub chUserDefinedName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 支持的 USB 协议
    pub nbcdUSB: c_uint,
    /// 设备地址
    pub nDeviceAddress: c_uint,
    /// 预留
    pub nReserved: [c_uint; 2],
}

/// CameraLink 设备信息
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MV_CamL_DEV_INFO {
    /// 串口号
    pub chPortID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 型号名字
    pub chModelName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 名称
    pub chFamilyName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 设备版本
    pub chDeviceVersion: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 制造商名字
    pub chManufacturerName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 序列号
    pub chSerialNumber: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 预留
    pub nReserved: [c_uint; 38],
}

/// CoaXPress 相机信息
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MV_CXP_DEVICE_INFO {
    /// 采集卡 ID
    pub chInterfaceID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 供应商名字
    pub chVendorName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 型号名字
    pub chModelName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 厂商信息
    pub chManufacturerInfo: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 相机版本
    pub chDeviceVersion: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 序列号
    pub chSerialNumber: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 用户自定义名字
    pub chUserDefinedName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 相机 ID
    pub chDeviceID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 保留字段
    pub nReserved: [c_uint; 7],
}

/// 采集卡 Camera Link 相机信息
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MV_CML_DEVICE_INFO {
    /// 采集卡 ID
    pub chInterfaceID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 供应商名字
    pub chVendorName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 型号名字
    pub chModelName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 厂商信息
    pub chManufacturerInfo: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 相机版本
    pub chDeviceVersion: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 序列号
    pub chSerialNumber: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 用户自定义名字
    pub chUserDefinedName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 相机 ID
    pub chDeviceID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 保留字段
    pub nReserved: [c_uint; 7],
}

/// XoFLink 相机信息
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MV_XOF_DEVICE_INFO {
    /// 采集卡 ID
    pub chInterfaceID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 供应商名字
    pub chVendorName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 型号名字
    pub chModelName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 厂商信息
    pub chManufacturerInfo: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 相机版本
    pub chDeviceVersion: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 序列号
    pub chSerialNumber: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 用户自定义名字
    pub chUserDefinedName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 相机 ID
    pub chDeviceID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 保留字段
    pub nReserved: [c_uint; 7],
}

/// 虚拟相机信息
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MV_GENTL_VIR_DEVICE_INFO {
    /// 采集卡 ID
    pub chInterfaceID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 供应商名字
    pub chVendorName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 型号名字
    pub chModelName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 厂商信息
    pub chManufacturerInfo: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 相机版本
    pub chDeviceVersion: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 序列号
    pub chSerialNumber: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 用户自定义名字
    pub chUserDefinedName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 相机 ID
    pub chDeviceID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 传输层类型
    pub chTLType: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 保留字段
    pub nReserved: [c_uint; 7],
}

/// 设备信息联合体
#[repr(C)]
pub union MV_CC_DEVICE_INFO_SPECIAL {
    /// GigE 设备信息
    pub stGigEInfo: MV_GIGE_DEVICE_INFO,
    /// USB 设备信息
    pub stUsb3VInfo: MV_USB3_DEVICE_INFO,
    /// CameraLink 设备信息
    pub stCamLInfo: MV_CamL_DEV_INFO,
    /// 采集卡 CameraLink 设备信息
    pub stCMLInfo: MV_CML_DEVICE_INFO,
    /// 采集卡 CoaXPress 设备信息
    pub stCXPInfo: MV_CXP_DEVICE_INFO,
    /// 采集卡 XoF 设备信息
    pub stXoFInfo: MV_XOF_DEVICE_INFO,
    /// 采集卡虚拟设备信息
    pub stVirInfo: MV_GENTL_VIR_DEVICE_INFO,
}

/// 设备信息
#[repr(C)]
pub struct MV_CC_DEVICE_INFO {
    /// 主要版本
    pub nMajorVer: c_ushort,
    /// 次要版本
    pub nMinorVer: c_ushort,
    /// 高 MAC 地址
    pub nMacAddrHigh: c_uint,
    /// 低 MAC 地址
    pub nMacAddrLow: c_uint,
    /// 设备传输层协议类型
    pub nTLayerType: c_uint,
    /// 设备类型信息
    pub nDevTypeInfo: c_uint,
    /// 预留
    pub nReserved: [c_uint; 3],
    /// 特殊信息联合体
    pub SpecialInfo: MV_CC_DEVICE_INFO_SPECIAL,
}

/// 设备信息列表
#[repr(C)]
pub struct MV_CC_DEVICE_INFO_LIST {
    /// 在线设备数量
    pub nDeviceNum: c_uint,
    /// 支持最多 256 个设备
    pub pDeviceInfo: [*mut MV_CC_DEVICE_INFO; MV_MAX_DEVICE_NUM],
}

/// 采集卡信息
#[repr(C)]
pub struct MV_INTERFACE_INFO {
    /// 采集卡类型
    pub nTLayerType: c_uint,
    /// 采集卡的 PCIE 插槽信息
    pub nPCIEInfo: c_uint,
    /// 采集卡 ID
    pub chInterfaceID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 显示名称
    pub chDisplayName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 序列号
    pub chSerialNumber: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 型号
    pub chModelName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 厂商
    pub chManufacturer: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 版本号
    pub chDeviceVersion: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 自定义名称
    pub chUserDefinedName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 保留字段
    pub nReserved: [c_uint; 64],
}

/// 采集卡信息列表
#[repr(C)]
pub struct MV_INTERFACE_INFO_LIST {
    /// 采集卡数量
    pub nInterfaceNum: c_uint,
    /// 采集卡信息，支持最多 64 个设备
    pub pInterfaceInfos: [*mut MV_INTERFACE_INFO; MV_MAX_INTERFACE_NUM],
}

/// 通过 GenTL 枚举到的接口信息
#[repr(C)]
pub struct MV_GENTL_IF_INFO {
    /// GenTL 接口 ID
    pub chInterfaceID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 传输层类型
    pub chTLType: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// Interface 显示名称
    pub chDisplayName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// GenTL 的 cti 文件索引
    pub nCtiIndex: c_uint,
    /// 预留
    pub nReserved: [c_uint; 8],
}

/// 通过 GenTL 枚举到的接口信息列表
#[repr(C)]
pub struct MV_GENTL_IF_INFO_LIST {
    /// 在线接口数量
    pub nInterfaceNum: c_uint,
    /// 支持最多 256 个接口
    pub pIFInfo: [*mut MV_GENTL_IF_INFO; MV_MAX_GENTL_IF_NUM],
}

/// 通过 GenTL 枚举到的设备信息
#[repr(C)]
pub struct MV_GENTL_DEV_INFO {
    /// GenTL 接口 ID
    pub chInterfaceID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 设备 ID
    pub chDeviceID: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 供应商名字
    pub chVendorName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 型号名字
    pub chModelName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 传输层类型
    pub chTLType: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 设备显示名称
    pub chDisplayName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 用户自定义名字
    pub chUserDefinedName: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 序列号
    pub chSerialNumber: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 设备版本号
    pub chDeviceVersion: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// GenTL 的 cti 文件索引
    pub nCtiIndex: c_uint,
    /// 预留
    pub nReserved: [c_uint; 8],
}

/// 通过 GenTL 枚举到的设备信息列表
#[repr(C)]
pub struct MV_GENTL_DEV_INFO_LIST {
    /// 在线设备数量
    pub nDeviceNum: c_uint,
    /// 支持最多 256 个设备
    pub pDeviceInfo: [*mut MV_GENTL_DEV_INFO; MV_MAX_GENTL_DEV_NUM],
}

/// Chunk 内容
#[repr(C)]
pub struct MV_CHUNK_DATA_CONTENT {
    /// Chunk 数据
    pub pChunkData: *mut c_uchar,
    /// Chunk ID
    pub nChunkID: c_uint,
    /// Chunk 的长度
    pub nChunkLen: c_uint,
    /// 预留
    pub nReserved: [c_uint; 8],
}

/// Chunk 数据联合体
#[repr(C)]
pub union MV_CHUNK_DATA_LIST {
    /// 未解析的 Chunk Content
    pub pUnparsedChunkContent: *mut MV_CHUNK_DATA_CONTENT,
    /// 校准
    pub nAligning: i64,
}

/// 子图信息联合体
#[repr(C)]
pub union MV_SUB_IMAGE_LIST {
    /// 子图信息
    pub pstSubImage: *mut MV_CC_IMAGE,
    /// 校准
    pub nAligning: i64,
}

/// 用户指针联合体
#[repr(C)]
pub union MV_USER_PTR {
    /// 自定义指针
    pub pUser: *mut c_void,
    /// 校准
    pub nAligning: i64,
}

/// 图像信息
#[repr(C)]
pub struct MV_CC_IMAGE {
    /// 图像宽
    pub nWidth: c_uint,
    /// 图像高
    pub nHeight: c_uint,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 图像缓存
    pub pImageBuf: *mut c_uchar,
    /// 图像缓存大小
    pub nImageBufSize: u64,
    /// 图像长度
    pub nImageLen: u64,
    /// 预留字段
    pub nReserved: [c_uint; 4],
}

/// 输出帧的信息
#[repr(C)]
pub struct MV_FRAME_OUT_INFO_EX {
    /// 图像宽 (最大 65535，超出请用 nExtendWidth)
    pub nWidth: c_ushort,
    /// 图像高 (最大 65535，超出请用 nExtendHeight)
    pub nHeight: c_ushort,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 帧号
    pub nFrameNum: c_uint,
    /// 时间戳高 32 位
    pub nDevTimeStampHigh: c_uint,
    /// 时间戳低 32 位
    pub nDevTimeStampLow: c_uint,
    /// 保留，8 字节对齐
    pub nReserved0: c_uint,
    /// 主机生成的时间戳
    pub nHostTimeStamp: i64,
    /// 帧的长度 (4GB 以上图像使用 nFrameLenEx 替代)
    pub nFrameLen: c_uint,
    /// 秒数
    pub nSecondCount: c_uint,
    /// 周期数
    pub nCycleCount: c_uint,
    /// 周期偏移量
    pub nCycleOffset: c_uint,
    /// 增益
    pub fGain: c_float,
    /// 曝光时间
    pub fExposureTime: c_float,
    /// 平均亮度
    pub nAverageBrightness: c_uint,
    /// 红色
    pub nRed: c_uint,
    /// 绿色
    pub nGreen: c_uint,
    /// 蓝色
    pub nBlue: c_uint,
    /// 总帧数
    pub nFrameCounter: c_uint,
    /// 触发计数
    pub nTriggerIndex: c_uint,
    /// 输入
    pub nInput: c_uint,
    /// 输出
    pub nOutput: c_uint,
    /// 水平偏移量
    pub nOffsetX: c_ushort,
    /// 垂直偏移量
    pub nOffsetY: c_ushort,
    /// Chunk 宽
    pub nChunkWidth: c_ushort,
    /// Chunk 高
    pub nChunkHeight: c_ushort,
    /// 本帧丢包数
    pub nLostPacket: c_uint,
    /// 未解析的 Chunkdata 个数
    pub nUnparsedChunkNum: c_uint,
    /// 未解析的 Chunk
    pub UnparsedChunkList: MV_CHUNK_DATA_LIST,
    /// 图像宽 (扩展变量)
    pub nExtendWidth: c_uint,
    /// 图像高 (扩展变量)
    pub nExtendHeight: c_uint,
    /// 帧的长度
    pub nFrameLenEx: u64,
    /// 保留，用于对齐
    pub nReserved1: c_uint,
    /// 图像缓存中的子图个数
    pub nSubImageNum: c_uint,
    /// 子图信息
    pub SubImageList: MV_SUB_IMAGE_LIST,
    /// 自定义指针
    pub UserPtr: MV_USER_PTR,
    /// 预留
    pub nReserved: [c_uint; 26],
}

/// 图像结构体，输出图像地址及图像信息
#[repr(C)]
pub struct MV_FRAME_OUT {
    /// 图像指针地址
    pub pBufAddr: *mut c_uchar,
    /// 图像信息
    pub stFrameInfo: MV_FRAME_OUT_INFO_EX,
    /// 预留
    pub nRes: [c_uint; 16],
}

/// 取流策略
#[repr(u32)]
pub enum MV_GRAB_STRATEGY {
    /// 从旧到新一帧一帧的获取图像
    MV_GrabStrategy_OneByOne = 0,
    /// 获取列表中最新的一帧图像
    MV_GrabStrategy_LatestImagesOnly = 1,
    /// 获取列表中最新的图像
    MV_GrabStrategy_LatestImages = 2,
    /// 等待下一帧图像
    MV_GrabStrategy_UpcomingImage = 3,
}

/// 网络传输的相关信息
#[repr(C)]
pub struct MV_NETTRANS_INFO {
    /// 已接收数据大小 [Start 和 Stop 之间]
    pub nReceiveDataSize: i64,
    /// 丢帧数量
    pub nThrowFrameCount: c_int,
    /// 已接收的帧数
    pub nNetRecvFrameCount: c_uint,
    /// 请求重发包数
    pub nRequestResendPacketCount: i64,
    /// 重发包数
    pub nResendPacketCount: i64,
}

/// 全匹配的一种信息结构体
#[repr(C)]
pub struct MV_ALL_MATCH_INFO {
    /// 需要输出的信息类型
    pub nType: c_uint,
    /// 输出的信息缓存，由调用者分配
    pub pInfo: *mut c_void,
    /// 信息缓存的大小
    pub nInfoSize: c_uint,
}

/// 网络流量和丢包信息反馈结构体
#[repr(C)]
pub struct MV_MATCH_INFO_NET_DETECT {
    /// 已接收数据大小
    pub nReceiveDataSize: i64,
    /// 丢失的包数量
    pub nLostPacketCount: i64,
    /// 丢帧数量
    pub nLostFrameCount: c_uint,
    /// 接收到的图像帧数
    pub nNetRecvFrameCount: c_uint,
    /// 请求重发包数
    pub nRequestResendPacketCount: i64,
    /// 重发包数
    pub nResendPacketCount: i64,
}

/// host 收到从 u3v 设备端的总字节数
#[repr(C)]
pub struct MV_MATCH_INFO_USB_DETECT {
    /// 已接收数据大小 [Open 和 Close 之间]
    pub nReceiveDataSize: i64,
    /// 已收到的帧数
    pub nReceivedFrameCount: c_uint,
    /// 错误帧数
    pub nErrorFrameCount: c_uint,
    /// 保留
    pub nReserved: [c_uint; 2],
}

/// 显示帧信息
#[repr(C)]
pub struct MV_DISPLAY_FRAME_INFO_EX {
    /// 图像宽
    pub nWidth: c_uint,
    /// 图像高
    pub nHeight: c_uint,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输入图像缓存
    pub pImageBuf: *mut c_uchar,
    /// 输入图像长度
    pub nImageBufLen: c_uint,
    /// 图像渲染方式
    pub enRenderMode: c_uint,
    /// 保留
    pub nRes: [c_uint; 3],
}

/// 图片保存参数
#[repr(C)]
pub struct MV_SAVE_IMAGE_PARAM_EX3 {
    /// 输入数据缓存
    pub pData: *mut c_uchar,
    /// 输入数据长度
    pub nDataLen: c_uint,
    /// 输入数据的像素格式
    pub enPixelType: MvGvspPixelType,
    /// 图像宽
    pub nWidth: c_uint,
    /// 图像高
    pub nHeight: c_uint,
    /// 输出图片缓存
    pub pImageBuffer: *mut c_uchar,
    /// 输出图片长度
    pub nImageLen: c_uint,
    /// 提供的输出缓冲区大小
    pub nBufferSize: c_uint,
    /// 输出图片格式
    pub enImageType: MV_SAVE_IMAGE_TYPE,
    /// JPG 编码质量 (50-99]，其它格式无效
    pub nJpgQuality: c_uint,
    /// 插值方法
    pub iMethodValue: c_uint,
    /// 预留
    pub nReserved: [c_uint; 3],
}

/// 保存图片到文件参数
#[repr(C)]
pub struct MV_SAVE_IMAGE_TO_FILE_PARAM_EX {
    /// 图像宽
    pub nWidth: c_uint,
    /// 图像高
    pub nHeight: c_uint,
    /// 输入数据的像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输入数据缓存
    pub pData: *mut c_uchar,
    /// 输入数据大小
    pub nDataLen: c_uint,
    /// 输入图片格式
    pub enImageType: MV_SAVE_IMAGE_TYPE,
    /// 输入文件路径
    pub pcImagePath: *mut c_char,
    /// JPG 编码质量 (50-99]，其它格式无效
    pub nQuality: c_uint,
    /// 插值方法
    pub iMethodValue: c_int,
    /// 预留
    pub nReserved: [c_uint; 8],
}

/// 保存图片所需参数
#[repr(C)]
pub struct MV_CC_SAVE_IMAGE_PARAM {
    /// 输入图片格式
    pub enImageType: MV_SAVE_IMAGE_TYPE,
    /// JPG 编码质量 (50-99]，其它格式无效
    pub nQuality: c_uint,
    /// 插值方法
    pub iMethodValue: c_int,
    /// 预留
    pub nReserved: [c_uint; 8],
}

/// 图像旋转结构体
#[repr(C)]
pub struct MV_CC_ROTATE_IMAGE_PARAM {
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 图像宽
    pub nWidth: c_uint,
    /// 图像高
    pub nHeight: c_uint,
    /// 输入数据缓存
    pub pSrcData: *mut c_uchar,
    /// 输入数据长度
    pub nSrcDataLen: c_uint,
    /// 输出数据缓存
    pub pDstBuf: *mut c_uchar,
    /// 输出数据长度
    pub nDstBufLen: c_uint,
    /// 提供的输出缓冲区大小
    pub nDstBufSize: c_uint,
    /// 旋转角度
    pub enRotationAngle: MV_IMG_ROTATION_ANGLE,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// 图像翻转结构体
#[repr(C)]
pub struct MV_CC_FLIP_IMAGE_PARAM {
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 图像宽
    pub nWidth: c_uint,
    /// 图像高
    pub nHeight: c_uint,
    /// 输入数据缓存
    pub pSrcData: *mut c_uchar,
    /// 输入数据长度
    pub nSrcDataLen: c_uint,
    /// 输出数据缓存
    pub pDstBuf: *mut c_uchar,
    /// 输出数据长度
    pub nDstBufLen: c_uint,
    /// 提供的输出缓冲区大小
    pub nDstBufSize: c_uint,
    /// 翻转类型
    pub enFlipType: MV_IMG_FLIP_TYPE,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// 像素转换结构体
#[repr(C)]
pub struct MV_CC_PIXEL_CONVERT_PARAM_EX {
    /// 图像宽
    pub nWidth: c_uint,
    /// 图像高
    pub nHeight: c_uint,
    /// 源像素格式
    pub enSrcPixelType: MvGvspPixelType,
    /// 输入数据缓存
    pub pSrcData: *mut c_uchar,
    /// 输入数据长度
    pub nSrcDataLen: c_uint,
    /// 目标像素格式
    pub enDstPixelType: MvGvspPixelType,
    /// 输出数据缓存
    pub pDstBuffer: *mut c_uchar,
    /// 输出数据长度
    pub nDstLen: c_uint,
    /// 提供的输出缓冲区大小
    pub nDstBufferSize: c_uint,
    /// 预留
    pub nRes: [c_uint; 4],
}

/// Gamma 信息结构体
#[repr(C)]
pub struct MV_CC_GAMMA_PARAM {
    /// Gamma 类型
    pub enGammaType: MV_CC_GAMMA_TYPE,
    /// Gamma 值：0.1 ~ 4.0
    pub fGammaValue: c_float,
    /// Gamma 曲线缓存
    pub pGammaCurveBuf: *mut c_uchar,
    /// Gamma 曲线长度
    pub nGammaCurveBufLen: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// CCM param
#[repr(C)]
pub struct MV_CC_CCM_PARAM {
    /// 是否启用 CCM
    pub bCCMEnable: bool,
    /// CCM 矩阵 [-8192~8192]
    pub nCCMat: [c_int; 9],
    /// 预留
    pub nRes: [c_uint; 8],
}

/// CCM param Ex
#[repr(C)]
pub struct MV_CC_CCM_PARAM_EX {
    /// 是否启用 CCM
    pub bCCMEnable: bool,
    /// CCM 矩阵 [-65536~65536]
    pub nCCMat: [c_int; 9],
    /// 量化系数（2 的整数幂，最大 65536）
    pub nCCMScale: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// 对比度调节结构体
#[repr(C)]
pub struct MV_CC_CONTRAST_PARAM {
    /// 图像宽度 (最小 8)
    pub nWidth: c_uint,
    /// 图像高度 (最小 8)
    pub nHeight: c_uint,
    /// 输入数据缓存
    pub pSrcBuf: *mut c_uchar,
    /// 输入数据大小
    pub nSrcBufLen: c_uint,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输出数据缓存
    pub pDstBuf: *mut c_uchar,
    /// 提供的输出缓冲区大小
    pub nDstBufSize: c_uint,
    /// 输出数据长度
    pub nDstBufLen: c_uint,
    /// 对比度值，\[1, 10000\]
    pub nContrastFactor: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// 水印信息
#[repr(C)]
pub struct MV_CC_FRAME_SPEC_INFO {
    /// 秒数
    pub nSecondCount: c_uint,
    /// 周期数
    pub nCycleCount: c_uint,
    /// 周期偏移量
    pub nCycleOffset: c_uint,
    /// 增益
    pub fGain: c_float,
    /// 曝光时间
    pub fExposureTime: c_float,
    /// 平均亮度
    pub nAverageBrightness: c_uint,
    /// 红色
    pub nRed: c_uint,
    /// 绿色
    pub nGreen: c_uint,
    /// 蓝色
    pub nBlue: c_uint,
    /// 总帧数
    pub nFrameCounter: c_uint,
    /// 触发计数
    pub nTriggerIndex: c_uint,
    /// 输入
    pub nInput: c_uint,
    /// 输出
    pub nOutput: c_uint,
    /// 水平偏移量
    pub nOffsetX: c_ushort,
    /// 垂直偏移量
    pub nOffsetY: c_ushort,
    /// 水印宽
    pub nFrameWidth: c_ushort,
    /// 水印高
    pub nFrameHeight: c_ushort,
    /// 预留
    pub nReserved: [c_uint; 16],
}

/// 去紫边结构体
#[repr(C)]
pub struct MV_CC_PURPLE_FRINGING_PARAM {
    /// 图像宽度 (最小 4)
    pub nWidth: c_uint,
    /// 图像高度 (最小 4)
    pub nHeight: c_uint,
    /// 输入数据缓存
    pub pSrcBuf: *mut c_uchar,
    /// 输入数据大小
    pub nSrcBufLen: c_uint,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输出数据缓存
    pub pDstBuf: *mut c_uchar,
    /// 提供的输出缓冲区大小
    pub nDstBufSize: c_uint,
    /// 输出数据长度
    pub nDstBufLen: c_uint,
    /// 滤波核尺寸，仅支持 3,5,7,9
    pub nKernelSize: c_uint,
    /// 边缘阈值 \[0, 2040\]
    pub nEdgeThreshold: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// ISP 配置结构体
#[repr(C)]
pub struct MV_CC_ISP_CONFIG_PARAM {
    /// 配置文件路径
    pub pcConfigPath: *mut c_char,
    /// 预留
    pub nRes: [c_uint; 16],
}

/// 无损解码参数
#[repr(C)]
pub struct MV_CC_HB_DECODE_PARAM {
    /// 输入数据缓存
    pub pSrcBuf: *mut c_uchar,
    /// 输入数据大小
    pub nSrcLen: c_uint,
    /// 图像宽
    pub nWidth: c_uint,
    /// 图像高
    pub nHeight: c_uint,
    /// 输出数据缓存
    pub pDstBuf: *mut c_uchar,
    /// 提供的输出缓冲区大小
    pub nDstBufSize: c_uint,
    /// 输出数据大小
    pub nDstBufLen: c_uint,
    /// 输出的像素格式
    pub enDstPixelType: MvGvspPixelType,
    /// 水印信息
    pub stFrameSpecInfo: MV_CC_FRAME_SPEC_INFO,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// 录像参数
#[repr(C)]
pub struct MV_CC_RECORD_PARAM {
    /// 输入数据的像素格式
    pub enPixelType: MvGvspPixelType,
    /// 图像宽 (2 的倍数)
    pub nWidth: c_ushort,
    /// 图像高 (2 的倍数)
    pub nHeight: c_ushort,
    /// 帧率 fps(大于 1/16)
    pub fFrameRate: c_float,
    /// 码率 kbps(128-16*1024)
    pub nBitRate: c_uint,
    /// 录像格式
    pub enRecordFmtType: MV_RECORD_FORMAT_TYPE,
    /// 录像文件存放路径 (如果路径中存在中文，需转成 utf-8)
    pub strFilePath: *mut c_char,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// 传入的图像数据
#[repr(C)]
pub struct MV_CC_INPUT_FRAME_INFO {
    /// 图像数据指针
    pub pData: *mut c_uchar,
    /// 图像大小
    pub nDataLen: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// Event 事件回调信息
#[repr(C)]
pub struct MV_EVENT_OUT_INFO {
    /// Event 名称
    pub EventName: [c_char; MAX_EVENT_NAME_SIZE],
    /// Event 号
    pub nEventID: c_ushort,
    /// 流通道序号
    pub nStreamChannel: c_ushort,
    /// 帧号高位
    pub nBlockIdHigh: c_uint,
    /// 帧号低位
    pub nBlockIdLow: c_uint,
    /// 时间戳高位
    pub nTimestampHigh: c_uint,
    /// 时间戳低位
    pub nTimestampLow: c_uint,
    /// Event 数据
    pub pEventData: *mut c_void,
    /// Event 数据长度
    pub nEventDataSize: c_uint,
    /// 预留
    pub nReserved: [c_uint; 16],
}

/// 文件存取
#[repr(C)]
pub struct MV_CC_FILE_ACCESS {
    /// 用户文件名
    pub pUserFileName: *const c_char,
    /// 设备文件名
    pub pDevFileName: *const c_char,
    /// 预留
    pub nReserved: [c_uint; 32],
}

/// 文件存取 Ex
#[repr(C)]
pub struct MV_CC_FILE_ACCESS_EX {
    /// 用户数据缓存
    pub pUserFileBuf: *mut c_char,
    /// 用户数据缓存大小
    pub pFileBufSize: c_uint,
    /// 文件数据缓存总长度
    pub pFileBufLen: c_uint,
    /// 设备文件名
    pub pDevFileName: *const c_char,
    /// 预留
    pub nReserved: [c_uint; 32],
}

/// 文件存取进度
#[repr(C)]
pub struct MV_CC_FILE_ACCESS_PROGRESS {
    /// 已完成的长度
    pub nCompleted: i64,
    /// 总长度
    pub nTotal: i64,
    /// 预留
    pub nReserved: [c_uint; 8],
}

/// 网络传输模式
#[repr(C)]
pub struct MV_TRANSMISSION_TYPE {
    /// 传输模式
    pub enTransmissionType: MV_GIGE_TRANSMISSION_TYPE,
    /// 目标 IP，组播模式下有意义
    pub nDestIp: c_uint,
    /// 目标 Port，组播模式下有意义
    pub nDestPort: c_ushort,
    /// 预留
    pub nReserved: [c_uint; 32],
}

/// 动作命令信息
#[repr(C)]
pub struct MV_ACTION_CMD_INFO {
    /// 设备密钥
    pub nDeviceKey: c_uint,
    /// 组键
    pub nGroupKey: c_uint,
    /// 组掩码
    pub nGroupMask: c_uint,
    /// 只有设置成 1 时 Action Time 才有效
    pub bActionTimeEnable: c_uint,
    /// 预定的时间，和主频有关
    pub nActionTime: i64,
    /// 广播包地址
    pub pBroadcastAddress: *const c_char,
    /// 等待 ACK 的超时时间
    pub nTimeOut: c_uint,
    /// 只有设置成 1 时指定的网卡 IP 才有效
    pub bSpecialNetEnable: c_uint,
    /// 指定的网卡 IP
    pub nSpecialNetIP: c_uint,
    /// 预留
    pub nReserved: [c_uint; 14],
}

/// 动作命令返回信息
#[repr(C)]
pub struct MV_ACTION_CMD_RESULT {
    /// 设备 IP
    pub strDeviceAddress: [c_uchar; 16],
    /// 状态码
    pub nStatus: c_int,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 动作命令返回信息列表
#[repr(C)]
pub struct MV_ACTION_CMD_RESULT_LIST {
    /// 返回值个数
    pub nNumResults: c_uint,
    /// 动作命令结果
    pub pResults: *mut MV_ACTION_CMD_RESULT,
}

/// 节点名称
#[repr(C)]
pub struct MVCC_NODE_NAME {
    /// 节点名称
    pub strName: [c_char; MV_MAX_NODE_NAME_LEN],
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 节点列表
#[repr(C)]
pub struct MVCC_NODE_NAME_LIST {
    /// 节点个数
    pub nNodeNum: c_uint,
    /// 节点名称
    pub stNodeName: [MVCC_NODE_NAME; MV_MAX_NODE_NUM],
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 错误信息
#[repr(C)]
pub struct MVCC_NODE_ERROR {
    /// 节点名称
    pub strName: [c_char; MV_MAX_NODE_NAME_LEN],
    /// 错误类型
    pub enErrType: MVCC_NODE_ERR_TYPE,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 错误信息列表
#[repr(C)]
pub struct MVCC_NODE_ERROR_LIST {
    /// 错误个数
    pub nErrorNum: c_uint,
    /// 错误信息
    pub stNodeError: [MVCC_NODE_ERROR; MV_MAX_NODE_ERROR_NUM],
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 枚举类型值
#[repr(C)]
pub struct MVCC_ENUMVALUE {
    /// 当前值
    pub nCurValue: c_uint,
    /// 数据的有效数据个数
    pub nSupportedNum: c_uint,
    /// 支持的枚举值
    pub nSupportValue: [c_uint; MV_MAX_XML_SYMBOLIC_NUM],
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 枚举类型值 Ex
#[repr(C)]
pub struct MVCC_ENUMVALUE_EX {
    /// 当前值
    pub nCurValue: c_uint,
    /// 数据的有效数据个数
    pub nSupportedNum: c_uint,
    /// 支持的枚举值
    pub nSupportValue: [c_uint; MV_MAX_ENUM_SYMBOLIC_NUM],
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 枚举类型条目
#[repr(C)]
pub struct MVCC_ENUMENTRY {
    /// 指定值
    pub nValue: c_uint,
    /// 指定值对应的符号
    pub chSymbolic: [c_char; MV_MAX_SYMBOLIC_LEN],
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// Int 类型值
#[repr(C)]
pub struct MVCC_INTVALUE {
    /// 当前值
    pub nCurValue: c_uint,
    /// 最大值
    pub nMax: c_uint,
    /// 最小值
    pub nMin: c_uint,
    /// Inc
    pub nInc: c_uint,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// Int 类型值 Ex
#[repr(C)]
pub struct MVCC_INTVALUE_EX {
    /// 当前值
    pub nCurValue: i64,
    /// 最大值
    pub nMax: i64,
    /// 最小值
    pub nMin: i64,
    /// Inc
    pub nInc: i64,
    /// 预留
    pub nReserved: [c_uint; 16],
}

/// Float 类型值
#[repr(C)]
pub struct MVCC_FLOATVALUE {
    /// 当前值
    pub fCurValue: c_float,
    /// 最大值
    pub fMax: c_float,
    /// 最小值
    pub fMin: c_float,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// String 类型值
#[repr(C)]
pub struct MVCC_STRINGVALUE {
    /// 当前值
    pub chCurValue: [c_char; 256],
    /// 最大长度
    pub nMaxLength: i64,
    /// 预留
    pub nReserved: [c_uint; 2],
}

/// 辅助线颜色
#[repr(C)]
pub struct MVCC_COLORF {
    /// 红色，范围 [0.0, 1.0]
    pub fR: c_float,
    /// 绿色，范围 [0.0, 1.0]
    pub fG: c_float,
    /// 蓝色，范围 [0.0, 1.0]
    pub fB: c_float,
    /// 透明度，范围 [0.0, 1.0]
    pub fAlpha: c_float,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 自定义点
#[repr(C)]
pub struct MVCC_POINTF {
    /// 该点距离图像左边缘距离，范围 [0.0, 1.0]
    pub fX: c_float,
    /// 该点距离图像上边缘距离，范围 [0.0, 1.0]
    pub fY: c_float,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 矩形框区域信息
#[repr(C)]
pub struct MVCC_RECT_INFO {
    /// 矩形上边缘距离图像上边缘的距离，范围 [0.0, 1.0]
    pub fTop: c_float,
    /// 矩形下边缘距离图像上边缘的距离，范围 [0.0, 1.0]
    pub fBottom: c_float,
    /// 矩形左边缘距离图像左边缘的距离，范围 [0.0, 1.0]
    pub fLeft: c_float,
    /// 矩形右边缘距离图像左边缘的距离，范围 [0.0, 1.0]
    pub fRight: c_float,
    /// 辅助线颜色
    pub stColor: MVCC_COLORF,
    /// 辅助线宽度，宽度只能是 1 或 2
    pub nLineWidth: c_uint,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 圆形框区域信息
#[repr(C)]
pub struct MVCC_CIRCLE_INFO {
    /// 圆心信息
    pub stCenterPoint: MVCC_POINTF,
    /// 宽向半径，范围 [0, 1.0]
    pub fR1: c_float,
    /// 高向半径，范围 [0, 1.0]
    pub fR2: c_float,
    /// 辅助线颜色信息
    pub stColor: MVCC_COLORF,
    /// 辅助线宽度，宽度只能是 1 或 2
    pub nLineWidth: c_uint,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 线条辅助线信息
#[repr(C)]
pub struct MVCC_LINES_INFO {
    /// 线条辅助线的起始点坐标
    pub stStartPoint: MVCC_POINTF,
    /// 线条辅助线的终点坐标
    pub stEndPoint: MVCC_POINTF,
    /// 辅助线颜色信息
    pub stColor: MVCC_COLORF,
    /// 辅助线宽度，宽度只能是 1 或 2
    pub nLineWidth: c_uint,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 输出图像信息
#[repr(C)]
pub struct MV_OUTPUT_IMAGE_INFO {
    /// 源图像宽
    pub nWidth: c_uint,
    /// 源图像高
    pub nHeight: c_uint,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输出数据缓存
    pub pBuf: *mut c_uchar,
    /// 输出数据长度
    pub nBufLen: c_uint,
    /// 提供的输出缓冲区大小
    pub nBufSize: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// 重构图像参数信息
#[repr(C)]
pub struct MV_RECONSTRUCT_IMAGE_PARAM {
    /// 源图像宽
    pub nWidth: c_uint,
    /// 源图像高
    pub nHeight: c_uint,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输入数据缓存
    pub pSrcData: *mut c_uchar,
    /// 输入数据长度
    pub nSrcDataLen: c_uint,
    /// 曝光个数 (1-8]
    pub nExposureNum: c_uint,
    /// 图像重构方式
    pub enReconstructMethod: MV_IMAGE_RECONSTRUCTION_METHOD,
    /// 输出数据缓存信息
    pub stDstBufList: [MV_OUTPUT_IMAGE_INFO; MV_MAX_SPLIT_NUM],
    /// 预留
    pub nRes: [c_uint; 4],
}

/// 串口信息
#[repr(C)]
pub struct MV_CAML_SERIAL_PORT {
    /// 串口号
    pub chSerialPort: [c_uchar; INFO_MAX_BUFFER_SIZE],
    /// 预留
    pub nRes: [c_uint; 4],
}

/// 串口信息列表
#[repr(C)]
pub struct MV_CAML_SERIAL_PORT_LIST {
    /// 串口数量
    pub nSerialPortNum: c_uint,
    /// 串口信息
    pub stSerialPort: [MV_CAML_SERIAL_PORT; MV_MAX_SERIAL_PORT_NUM],
    /// 预留
    pub nRes: [c_uint; 4],
}

/// 最大支持的串口数量
pub const MV_MAX_SERIAL_PORT_NUM: usize = 64;
