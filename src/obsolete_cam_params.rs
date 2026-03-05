//! 已废弃的相机参数定义
//!
//! 该模块包含了海康威视 MVS SDK 中已废弃但仍需支持的相机参数相关的结构体和枚举定义

use crate::MV_MAX_XML_SYMBOLIC_NUM;
use crate::camera_params::{
    MV_CC_CCM_PARAM_EX, MV_CC_GAMMA_PARAM, MV_SAVE_IMAGE_TYPE, MV_XML_AccessMode,
    MV_XML_InterfaceType,
};
use crate::pixel_type::MvGvspPixelType;
use core::ffi::{c_char, c_float, c_int, c_uchar, c_uint, c_ushort, c_void};

/// 某个节点对应的子节点个数最大值
pub const MV_MAX_XML_NODE_NUM_C: usize = 128;

/// 节点名称字符串最大长度
pub const MV_MAX_XML_NODE_STRLEN_C: usize = 64;

/// 节点 String 值最大长度
pub const MV_MAX_XML_STRVALUE_STRLEN_C: usize = 64;

/// 节点描述字段最大长度
pub const MV_MAX_XML_DISC_STRLEN_C: usize = 512;

/// 最多的单元数
pub const MV_MAX_XML_ENTRY_NUM: usize = 10;

/// 父节点个数上限
pub const MV_MAX_XML_PARENTS_NUM: usize = 8;

/// 每个已经实现单元的名称长度
pub const MV_MAX_XML_SYMBOLIC_STRLEN_C: usize = 64;

/// 输出帧的信息
#[repr(C)]
#[deprecated]
pub struct MV_FRAME_OUT_INFO {
    /// 图像宽
    pub nWidth: c_ushort,
    /// 图像高
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
    /// 帧长度
    pub nFrameLen: c_uint,
    /// 本帧丢包数
    pub nLostPacket: c_uint,
    /// 预留
    pub nReserved: [c_uint; 2],
}

/// 保存图片参数
#[repr(C)]
#[deprecated]
pub struct MV_SAVE_IMAGE_PARAM {
    /// 输入数据缓存
    pub pData: *mut c_uchar,
    /// 输入数据大小
    pub nDataLen: c_uint,
    /// 输入像素格式
    pub enPixelType: MvGvspPixelType,
    /// 图像宽
    pub nWidth: c_ushort,
    /// 图像高
    pub nHeight: c_ushort,
    /// 输出图片缓存
    pub pImageBuffer: *mut c_uchar,
    /// 输出图片大小
    pub nImageLen: c_uint,
    /// 提供的输出缓冲区大小
    pub nBufferSize: c_uint,
    /// 输出图片格式
    pub enImageType: MV_SAVE_IMAGE_TYPE,
}

/// 图像基本信息
#[repr(C)]
#[deprecated]
pub struct MV_IMAGE_BASIC_INFO {
    /// 宽度当前值
    pub nWidthValue: c_ushort,
    /// 宽度最小值
    pub nWidthMin: c_ushort,
    /// 宽度最大值
    pub nWidthMax: c_uint,
    /// 宽度增量
    pub nWidthInc: c_uint,
    /// 高度当前值
    pub nHeightValue: c_uint,
    /// 高度最小值
    pub nHeightMin: c_uint,
    /// 高度最大值
    pub nHeightMax: c_uint,
    /// 高度增量
    pub nHeightInc: c_uint,
    /// 帧率当前值
    pub fFrameRateValue: c_float,
    /// 帧率最小值
    pub fFrameRateMin: c_float,
    /// 帧率最大值
    pub fFrameRateMax: c_float,
    /// 当前的像素格式
    pub enPixelType: c_uint,
    /// 支持的像素格式种类
    pub nSupportedPixelFmtNum: c_uint,
    /// 像素格式列表
    pub enPixelList: [c_uint; MV_MAX_XML_SYMBOLIC_NUM],
    /// 预留
    pub nReserved: [c_uint; 8],
}

/// 噪声特性类型
#[repr(u32)]
#[deprecated]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MV_CC_BAYER_NOISE_FEATURE_TYPE {
    /// 无效值
    MV_CC_BAYER_NOISE_FEATURE_TYPE_INVALID = 0,
    /// 噪声曲线
    MV_CC_BAYER_NOISE_FEATURE_TYPE_PROFILE = 1,
    /// 噪声水平
    MV_CC_BAYER_NOISE_FEATURE_TYPE_LEVEL = 2,
    /// 默认值（与 PROFILE 相同）
    MV_CC_BAYER_NOISE_FEATURE_TYPE_DEFAULT = 3,
}

/// Bayer 格式降噪特性信息
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_CC_BAYER_NOISE_PROFILE_INFO {
    /// 版本
    pub nVersion: c_uint,
    /// 噪声特性类型
    pub enNoiseFeatureType: MV_CC_BAYER_NOISE_FEATURE_TYPE,
    /// 图像格式
    pub enPixelType: MvGvspPixelType,
    /// 平均噪声水平
    pub nNoiseLevel: c_int,
    /// 曲线点数
    pub nCurvePointNum: c_uint,
    /// 噪声曲线
    pub nNoiseCurve: *mut c_int,
    /// 亮度曲线
    pub nLumCurve: *mut c_int,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// Bayer 格式噪声估计参数
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_CC_BAYER_NOISE_ESTIMATE_PARAM {
    /// 图像宽 (大于等于 8)
    pub nWidth: c_uint,
    /// 图像高 (大于等于 8)
    pub nHeight: c_uint,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输入数据缓存
    pub pSrcData: *mut c_uchar,
    /// 输入数据大小
    pub nSrcDataLen: c_uint,
    /// 噪声阈值 (0-4095)
    pub nNoiseThreshold: c_uint,
    /// 用于存储噪声曲线和亮度曲线
    pub pCurveBuf: *mut c_uchar,
    /// 降噪特性信息
    pub stNoiseProfile: MV_CC_BAYER_NOISE_PROFILE_INFO,
    /// 线程数量
    pub nThreadNum: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// Bayer 格式空域降噪参数
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_CC_BAYER_SPATIAL_DENOISE_PARAM {
    /// 图像宽 (大于等于 8)
    pub nWidth: c_uint,
    /// 图像高 (大于等于 8)
    pub nHeight: c_uint,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输入数据缓存
    pub pSrcData: *mut c_uchar,
    /// 输入数据大小
    pub nSrcDataLen: c_uint,
    /// 输出降噪后的数据
    pub pDstBuf: *mut c_uchar,
    /// 提供的输出缓冲区大小
    pub nDstBufSize: c_uint,
    /// 输出降噪后的数据长度
    pub nDstBufLen: c_uint,
    /// 降噪特性信息
    pub stNoiseProfile: MV_CC_BAYER_NOISE_PROFILE_INFO,
    /// 降噪强度 (0-100)
    pub nDenoiseStrength: c_uint,
    /// 锐化强度 (0-32)
    pub nSharpenStrength: c_uint,
    /// 噪声校正系数 (0-1280)
    pub nNoiseCorrect: c_uint,
    /// 线程数量
    pub nThreadNum: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// CLUT 参数
#[repr(C)]
#[deprecated]
pub struct MV_CC_CLUT_PARAM {
    /// 是否启用 CLUT
    pub bCLUTEnable: bool,
    /// 量化系数 (2 的整数幂，最大 65536)
    pub nCLUTScale: c_uint,
    /// CLUT 大小，目前仅支持 17
    pub nCLUTSize: c_uint,
    /// 量化 CLUT 表
    pub pCLUTBuf: *mut c_uchar,
    /// 量化 CLUT 缓存大小
    pub nCLUTBufLen: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// 锐化结构体
#[repr(C)]
#[deprecated]
pub struct MV_CC_SHARPEN_PARAM {
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
    /// 锐度调节强度，\[0, 500\]
    pub nSharpenAmount: c_uint,
    /// 锐度调节半径 (半径越大，耗时越长)，\[1, 21\]
    pub nSharpenRadius: c_uint,
    /// 锐度调节阈值，\[0, 255\]
    pub nSharpenThreshold: c_uint,
    /// 锐度调节正向强度，范围:\[0, 500\]
    pub nSharpenPosAmount: c_uint,
    /// 锐度调节负向强度，范围:\[0, 500\]
    pub nSharpenNegAmount: c_uint,
    /// 预留
    pub nRes: [c_uint; 6],
}

/// 色彩校正结构体
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_CC_COLOR_CORRECT_PARAM {
    /// 图像宽度
    pub nWidth: c_uint,
    /// 图像高度
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
    /// 有效图像位数 (8,10,12,16)
    pub nImageBit: c_uint,
    /// Gamma 信息
    pub stGammaParam: MV_CC_GAMMA_PARAM,
    /// CCM 信息
    pub stCCMParam: MV_CC_CCM_PARAM_EX,
    /// CLUT 信息
    pub stCLUTParam: MV_CC_CLUT_PARAM,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// 矩形 ROI 结构体
#[repr(C)]
#[deprecated]
pub struct MV_CC_RECT_I {
    /// 矩形左上角 X 轴坐标
    pub nX: c_uint,
    /// 矩形左上角 Y 轴坐标
    pub nY: c_uint,
    /// 矩形宽度
    pub nWidth: c_uint,
    /// 矩形高度
    pub nHeight: c_uint,
}

/// 噪声估计结构体
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_CC_NOISE_ESTIMATE_PARAM {
    /// 图像宽度 (最小 8)
    pub nWidth: c_uint,
    /// 图像高度 (最小 8)
    pub nHeight: c_uint,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输入数据缓存
    pub pSrcBuf: *mut c_uchar,
    /// 输入数据大小
    pub nSrcBufLen: c_uint,
    /// 图像 ROI
    pub pstROIRect: *mut MV_CC_RECT_I,
    /// ROI 个数
    pub nROINum: c_uint,
    /// 噪声阈值 \[0, 4095\]
    pub nNoiseThreshold: c_uint,
    /// 输出噪声特性
    pub pNoiseProfile: *mut c_uchar,
    /// 提供的输出缓冲区大小
    pub nNoiseProfileSize: c_uint,
    /// 输出噪声特性长度
    pub nNoiseProfileLen: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// 空域降噪结构体
#[repr(C)]
#[deprecated]
pub struct MV_CC_SPATIAL_DENOISE_PARAM {
    /// 图像宽度 (最小 8)
    pub nWidth: c_uint,
    /// 图像高度 (最小 8)
    pub nHeight: c_uint,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输入数据缓存
    pub pSrcBuf: *mut c_uchar,
    /// 输入数据大小
    pub nSrcBufLen: c_uint,
    /// 输出降噪后的数据
    pub pDstBuf: *mut c_uchar,
    /// 提供的输出缓冲区大小
    pub nDstBufSize: c_uint,
    /// 输出降噪后的数据长度
    pub nDstBufLen: c_uint,
    /// 输入噪声特性
    pub pNoiseProfile: *mut c_uchar,
    /// 输入噪声特性长度
    pub nNoiseProfileLen: c_uint,
    /// 降噪强度 \[0, 100\]
    pub nBayerDenoiseStrength: c_uint,
    /// 锐化强度 \[0, 32\]
    pub nBayerSharpenStrength: c_uint,
    /// 噪声校正系数 \[0, 1280\]
    pub nBayerNoiseCorrect: c_uint,
    /// 亮度校正系数 \[1, 2000\]
    pub nNoiseCorrectLum: c_uint,
    /// 色调校正系数 \[1, 2000\]
    pub nNoiseCorrectChrom: c_uint,
    /// 亮度降噪强度 \[0, 100\]
    pub nStrengthLum: c_uint,
    /// 色调降噪强度 \[0, 100\]
    pub nStrengthChrom: c_uint,
    /// 锐化强度 \[1, 1000\]
    pub nStrengthSharpen: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// LSC 标定结构体
#[repr(C)]
#[deprecated]
pub struct MV_CC_LSC_CALIB_PARAM {
    /// 图像宽度 \[16, 65535\]
    pub nWidth: c_uint,
    /// 图像高度 \[16, 65535\]
    pub nHeight: c_uint,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输入数据缓存
    pub pSrcBuf: *mut c_uchar,
    /// 输入数据长度
    pub nSrcBufLen: c_uint,
    /// 输出标定表缓存
    pub pCalibBuf: *mut c_uchar,
    /// 提供的标定表缓冲大小
    pub nCalibBufSize: c_uint,
    /// 输出标定表缓存长度
    pub nCalibBufLen: c_uint,
    /// 宽度分块数
    pub nSecNumW: c_uint,
    /// 高度分块数
    pub nSecNumH: c_uint,
    /// 边缘填充系数 \[1, 5\]
    pub nPadCoef: c_uint,
    /// 标定方式 (0-中心为基准;1-最亮区域为基准;2-目标亮度为基准)
    pub nCalibMethod: c_uint,
    /// 目标亮度 (标定方式为 2 时有效)
    pub nTargetGray: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// LSC 校正结构体
#[repr(C)]
#[deprecated]
pub struct MV_CC_LSC_CORRECT_PARAM {
    /// 图像宽度 \[16, 65535\]
    pub nWidth: c_uint,
    /// 图像高度 \[16, 65535\]
    pub nHeight: c_uint,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输入数据缓存
    pub pSrcBuf: *mut c_uchar,
    /// 输入数据长度
    pub nSrcBufLen: c_uint,
    /// 输出数据缓存
    pub pDstBuf: *mut c_uchar,
    /// 提供的输出缓冲区大小
    pub nDstBufSize: c_uint,
    /// 输出数据长度
    pub nDstBufLen: c_uint,
    /// 输入标定表缓存
    pub pCalibBuf: *mut c_uchar,
    /// 输入标定表缓存长度
    pub nCalibBufLen: c_uint,
    /// 预留
    pub nRes: [c_uint; 8],
}

/// 可见性枚举
#[repr(u32)]
#[deprecated]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MV_XML_Visibility {
    /// 总是可见
    V_Beginner = 0,
    /// 专家或 Guru 可见
    V_Expert = 1,
    /// 仅 Guru 可见
    V_Guru = 2,
    /// 不可见
    V_Invisible = 3,
    /// 未初始化
    V_Undefined = 99,
}

/// 单个节点基本属性
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_XML_NODE_FEATURE {
    /// 节点类型
    pub enType: MV_XML_InterfaceType,
    /// 是否可见
    pub enVisivility: MV_XML_Visibility,
    /// 节点描述
    pub strDescription: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 显示名称
    pub strDisplayName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 节点名
    pub strName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 提示
    pub strToolTip: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// 节点列表
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_XML_NODES_LIST {
    /// 节点个数
    pub nNodeNum: c_uint,
    /// 节点信息
    pub stNodes: [MV_XML_NODE_FEATURE; MV_MAX_XML_NODE_NUM_C],
}

/// Value 节点特征
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_XML_FEATURE_Value {
    /// 节点类型
    pub enType: MV_XML_InterfaceType,
    /// 节点描述
    pub strDescription: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 显示名称
    pub strDisplayName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 节点名
    pub strName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 提示
    pub strToolTip: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// Base 节点特征
#[repr(C)]
#[deprecated]
pub struct MV_XML_FEATURE_Base {
    /// 访问模式
    pub enAccessMode: MV_XML_AccessMode,
}

/// Integer 节点特征
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
#[derive(Debug, Clone, Copy)]
pub struct MV_XML_FEATURE_Integer {
    /// 节点名
    pub strName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 显示名称
    pub strDisplayName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 节点描述
    pub strDescription: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 提示
    pub strToolTip: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 是否可见
    pub enVisivility: MV_XML_Visibility,
    /// 访问模式
    pub enAccessMode: MV_XML_AccessMode,
    /// 是否锁定
    pub bIsLocked: c_int,
    /// 当前值
    pub nValue: i64,
    /// 最小值
    pub nMinValue: i64,
    /// 最大值
    pub nMaxValue: i64,
    /// 增量
    pub nIncrement: i64,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// Boolean 节点特征
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_XML_FEATURE_Boolean {
    /// 节点名
    pub strName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 显示名称
    pub strDisplayName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 节点描述
    pub strDescription: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 提示
    pub strToolTip: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 是否可见
    pub enVisivility: MV_XML_Visibility,
    /// 访问模式
    pub enAccessMode: MV_XML_AccessMode,
    /// 是否锁定
    pub bIsLocked: c_int,
    /// 当前值
    pub bValue: bool,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// Command 节点特征
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_XML_FEATURE_Command {
    /// 节点名
    pub strName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 显示名称
    pub strDisplayName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 节点描述
    pub strDescription: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 提示
    pub strToolTip: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 是否可见
    pub enVisivility: MV_XML_Visibility,
    /// 访问模式
    pub enAccessMode: MV_XML_AccessMode,
    /// 是否锁定
    pub bIsLocked: c_int,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// Float 节点特征
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
#[derive(Debug, Clone, Copy)]
pub struct MV_XML_FEATURE_Float {
    /// 节点名
    pub strName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 显示名称
    pub strDisplayName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 节点描述
    pub strDescription: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 提示
    pub strToolTip: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 是否可见
    pub enVisivility: MV_XML_Visibility,
    /// 访问模式
    pub enAccessMode: MV_XML_AccessMode,
    /// 是否锁定
    pub bIsLocked: c_int,
    /// 当前值
    pub dfValue: f64,
    /// 最小值
    pub dfMinValue: f64,
    /// 最大值
    pub dfMaxValue: f64,
    /// 增量
    pub dfIncrement: f64,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// String 节点特征
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
#[derive(Debug, Clone, Copy)]
pub struct MV_XML_FEATURE_String {
    /// 节点名
    pub strName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 显示名称
    pub strDisplayName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 节点描述
    pub strDescription: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 提示
    pub strToolTip: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 是否可见
    pub enVisivility: MV_XML_Visibility,
    /// 访问模式
    pub enAccessMode: MV_XML_AccessMode,
    /// 是否锁定
    pub bIsLocked: c_int,
    /// 当前值
    pub strValue: [c_char; MV_MAX_XML_STRVALUE_STRLEN_C],
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// Register 节点特征
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_XML_FEATURE_Register {
    /// 节点名
    pub strName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 显示名称
    pub strDisplayName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 节点描述
    pub strDescription: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 提示
    pub strToolTip: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 是否可见
    pub enVisivility: MV_XML_Visibility,
    /// 访问模式
    pub enAccessMode: MV_XML_AccessMode,
    /// 是否锁定
    pub bIsLocked: c_int,
    /// 当前值
    pub nAddrValue: i64,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// Category 节点特征
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_XML_FEATURE_Category {
    /// 节点描述
    pub strDescription: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 显示名称
    pub strDisplayName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 节点名
    pub strName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 提示
    pub strToolTip: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 是否可见
    pub enVisivility: MV_XML_Visibility,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// EnumEntry 节点特征
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_XML_FEATURE_EnumEntry {
    /// 节点名
    pub strName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 显示名称
    pub strDisplayName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 节点描述
    pub strDescription: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 提示
    pub strToolTip: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 是否已实现
    pub bIsImplemented: c_int,
    /// 父节点个数
    pub nParentsNum: c_int,
    /// 父节点列表
    pub stParentsList: [MV_XML_NODE_FEATURE; MV_MAX_XML_PARENTS_NUM],
    /// 是否可见
    pub enVisivility: MV_XML_Visibility,
    /// 当前值
    pub nValue: i64,
    /// 访问模式
    pub enAccessMode: MV_XML_AccessMode,
    /// 是否锁定
    pub bIsLocked: c_int,
    /// 预留
    pub nReserved: [c_uint; 8],
}

/// Enumeration 节点特征
#[repr(C)]
#[deprecated]
#[derive(Debug, Clone, Copy)]
#[allow(deprecated)]
pub struct MV_XML_FEATURE_Enumeration {
    /// 是否可见
    pub enVisivility: MV_XML_Visibility,
    /// 节点描述
    pub strDescription: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 显示名称
    pub strDisplayName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 节点名
    pub strName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 提示
    pub strToolTip: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// Symbolic 数
    pub nSymbolicNum: c_int,
    /// 当前 Symbolic 索引
    pub strCurrentSymbolic: [c_char; MV_MAX_XML_SYMBOLIC_STRLEN_C],
    /// Symbolic 列表
    pub strSymbolic: [[c_char; MV_MAX_XML_SYMBOLIC_STRLEN_C]; MV_MAX_XML_SYMBOLIC_NUM],
    /// 访问模式
    pub enAccessMode: MV_XML_AccessMode,
    /// 是否锁定
    pub bIsLocked: c_int,
    /// 当前值
    pub nValue: i64,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// Port 节点特征
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_XML_FEATURE_Port {
    /// 是否可见
    pub enVisivility: MV_XML_Visibility,
    /// 节点描述
    pub strDescription: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 显示名称
    pub strDisplayName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 节点名
    pub strName: [c_char; MV_MAX_XML_NODE_STRLEN_C],
    /// 提示
    pub strToolTip: [c_char; MV_MAX_XML_DISC_STRLEN_C],
    /// 访问模式
    pub enAccessMode: MV_XML_AccessMode,
    /// 是否锁定
    pub bIsLocked: c_int,
    /// 预留
    pub nReserved: [c_uint; 4],
}

/// XML 相机特征联合体
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub union MV_XML_CAMERA_FEATURE_SPECIAL {
    /// Integer 特征
    pub stIntegerFeature: MV_XML_FEATURE_Integer,
    /// Float 特征
    pub stFloatFeature: MV_XML_FEATURE_Float,
    /// Enumeration 特征
    pub stEnumerationFeature: MV_XML_FEATURE_Enumeration,
    /// String 特征
    pub stStringFeature: MV_XML_FEATURE_String,
}

/// XML 相机特征
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_XML_CAMERA_FEATURE {
    /// 节点类型
    pub enType: MV_XML_InterfaceType,
    /// 特殊特征联合体
    pub SpecialFeature: MV_XML_CAMERA_FEATURE_SPECIAL,
}

/// 图片保存参数 Ex
#[repr(C)]
#[deprecated]
pub struct MV_SAVE_IMAGE_PARAM_EX {
    /// 输入数据缓存
    pub pData: *mut c_uchar,
    /// 输入数据长度
    pub nDataLen: c_uint,
    /// 输入数据的像素格式
    pub enPixelType: MvGvspPixelType,
    /// 图像宽
    pub nWidth: c_ushort,
    /// 图像高
    pub nHeight: c_ushort,
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
#[deprecated]
pub struct MV_SAVE_IMG_TO_FILE_PARAM {
    /// 输入数据的像素格式
    pub enPixelType: MvGvspPixelType,
    /// 输入数据缓存
    pub pData: *mut c_uchar,
    /// 输入数据长度
    pub nDataLen: c_uint,
    /// 图像宽
    pub nWidth: c_ushort,
    /// 图像高
    pub nHeight: c_ushort,
    /// 输入图片格式
    pub enImageType: MV_SAVE_IMAGE_TYPE,
    /// JPG 编码质量 (50-99]，其它格式无效
    pub nQuality: c_uint,
    /// 输入文件路径
    pub pImagePath: [c_char; 256],
    /// 插值方法
    pub iMethodValue: c_int,
    /// 预留
    pub nReserved: [c_uint; 8],
}

/// 像素转换结构体
#[repr(C)]
#[deprecated]
pub struct MV_CC_PIXEL_CONVERT_PARAM {
    /// 图像宽
    pub nWidth: c_ushort,
    /// 图像高
    pub nHeight: c_ushort,
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

/// 保存的 3D 数据格式
#[repr(u32)]
#[deprecated]
pub enum MV_SAVE_POINT_CLOUD_FILE_TYPE {
    /// 未定义的点云格式
    MV_PointCloudFile_Undefined = 0,
    /// PLY 点云格式
    MV_PointCloudFile_PLY = 1,
    /// CSV 点云格式
    MV_PointCloudFile_CSV = 2,
    /// OBJ 点云格式
    MV_PointCloudFile_OBJ = 3,
}

/// 保存 3D 数据到缓存
#[repr(C)]
#[deprecated]
#[allow(deprecated)]
pub struct MV_SAVE_POINT_CLOUD_PARAM {
    /// 行点数，即图像宽
    pub nLinePntNum: c_uint,
    /// 行数，即图像高
    pub nLineNum: c_uint,
    /// 输入数据的像素格式
    pub enSrcPixelType: MvGvspPixelType,
    /// 输入数据缓存
    pub pSrcData: *mut c_uchar,
    /// 输入数据长度
    pub nSrcDataLen: c_uint,
    /// 输出像素数据缓存
    pub pDstBuf: *mut c_uchar,
    /// 提供的输出缓冲区大小
    pub nDstBufSize: c_uint,
    /// 输出像素数据缓存长度
    pub nDstBufLen: c_uint,
    /// 提供输出的点云文件类型
    pub enPointCloudFileType: MV_SAVE_POINT_CLOUD_FILE_TYPE,
    /// 预留
    pub nReserved: [c_uint; 8],
}

/// 显示帧信息
#[repr(C)]
#[deprecated]
pub struct MV_DISPLAY_FRAME_INFO {
    /// 窗口句柄
    pub hWnd: *mut c_void,
    /// 显示的数据
    pub pData: *mut c_uchar,
    /// 数据长度
    pub nDataLen: c_uint,
    /// 图像宽
    pub nWidth: c_ushort,
    /// 图像高
    pub nHeight: c_ushort,
    /// 像素格式
    pub enPixelType: MvGvspPixelType,
    /// 图像渲染方式
    pub enRenderMode: c_uint,
    /// 保留
    pub nRes: [c_uint; 3],
}
