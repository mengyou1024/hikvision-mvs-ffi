//! ISP错误码定义
//!
//! 该模块包含了来自ISP算法库的错误码定义

// 来自ISP算法库的错误码

// 通用类型
/// 处理正确
pub const MV_ALG_OK: u32 = 0x00000000;
/// 不确定类型错误
pub const MV_ALG_ERR: u32 = 0x10000000;

// 能力检查
/// 能力集中存在无效参数
pub const MV_ALG_E_ABILITY_ARG: u32 = 0x10000001;

// 内存检查
/// 内存地址为空
pub const MV_ALG_E_MEM_NULL: u32 = 0x10000002;
/// 内存对齐不满足要求
pub const MV_ALG_E_MEM_ALIGN: u32 = 0x10000003;
/// 内存空间大小不够
pub const MV_ALG_E_MEM_LACK: u32 = 0x10000004;
/// 内存空间大小不满足对齐要求
pub const MV_ALG_E_MEM_SIZE_ALIGN: u32 = 0x10000005;
/// 内存地址不满足对齐要求
pub const MV_ALG_E_MEM_ADDR_ALIGN: u32 = 0x10000006;

// 图像检查
/// 图像格式不正确或者不支持
pub const MV_ALG_E_IMG_FORMAT: u32 = 0x10000007;
/// 图像宽高不正确或者超出范围
pub const MV_ALG_E_IMG_SIZE: u32 = 0x10000008;
/// 图像宽高与step参数不匹配
pub const MV_ALG_E_IMG_STEP: u32 = 0x10000009;
/// 图像数据存储地址为空
pub const MV_ALG_E_IMG_DATA_NULL: u32 = 0x1000000A;

// 输入输出参数检查
/// 设置或者获取参数类型不正确
pub const MV_ALG_E_CFG_TYPE: u32 = 0x1000000B;
/// 设置或者获取参数的输入、输出结构体大小不正确
pub const MV_ALG_E_CFG_SIZE: u32 = 0x1000000C;
/// 处理类型不正确
pub const MV_ALG_E_PRC_TYPE: u32 = 0x1000000D;
/// 处理时输入、输出参数大小不正确
pub const MV_ALG_E_PRC_SIZE: u32 = 0x1000000E;
/// 子处理类型不正确
pub const MV_ALG_E_FUNC_TYPE: u32 = 0x1000000F;
/// 子处理时输入、输出参数大小不正确
pub const MV_ALG_E_FUNC_SIZE: u32 = 0x10000010;

// 运行参数检查
/// index参数不正确
pub const MV_ALG_E_PARAM_INDEX: u32 = 0x10000011;
/// value参数不正确或者超出范围
pub const MV_ALG_E_PARAM_VALUE: u32 = 0x10000012;
/// param_num参数不正确
pub const MV_ALG_E_PARAM_NUM: u32 = 0x10000013;

// 接口调用检查
/// 函数参数指针为空
pub const MV_ALG_E_NULL_PTR: u32 = 0x10000014;
/// 超过限定的最大内存
pub const MV_ALG_E_OVER_MAX_MEM: u32 = 0x10000015;
/// 回调函数出错
pub const MV_ALG_E_CALL_BACK: u32 = 0x10000016;

// 算法库加密相关检查
/// 加密错误
pub const MV_ALG_E_ENCRYPT: u32 = 0x10000017;
/// 算法库使用期限错误
pub const MV_ALG_E_EXPIRE: u32 = 0x10000018;

// 内部模块返回的基本错误类型
/// 参数范围不正确
pub const MV_ALG_E_BAD_ARG: u32 = 0x10000019;
/// 数据大小不正确
pub const MV_ALG_E_DATA_SIZE: u32 = 0x1000001A;
/// 数据step不正确
pub const MV_ALG_E_STEP: u32 = 0x1000001B;

// cpu指令集支持错误码
/// cpu不支持优化代码中的指令集
pub const MV_ALG_E_CPUID: u32 = 0x1000001C;

/// 警告
pub const MV_ALG_WARNING: u32 = 0x1000001D;

/// 算法库超时
pub const MV_ALG_E_TIME_OUT: u32 = 0x1000001E;
/// 算法版本号出错
pub const MV_ALG_E_LIB_VERSION: u32 = 0x1000001F;
/// 模型版本号出错
pub const MV_ALG_E_MODEL_VERSION: u32 = 0x10000020;
/// GPU内存分配错误
pub const MV_ALG_E_GPU_MEM_ALLOC: u32 = 0x10000021;
/// 文件不存在
pub const MV_ALG_E_FILE_NON_EXIST: u32 = 0x10000022;
/// 字符串为空
pub const MV_ALG_E_NONE_STRING: u32 = 0x10000023;
/// 图像解码器错误
pub const MV_ALG_E_IMAGE_CODEC: u32 = 0x10000024;
/// 打开文件错误
pub const MV_ALG_E_FILE_OPEN: u32 = 0x10000025;
/// 文件读取错误
pub const MV_ALG_E_FILE_READ: u32 = 0x10000026;
/// 文件写错误
pub const MV_ALG_E_FILE_WRITE: u32 = 0x10000027;
/// 文件读取大小错误
pub const MV_ALG_E_FILE_READ_SIZE: u32 = 0x10000028;
/// 文件类型错误
pub const MV_ALG_E_FILE_TYPE: u32 = 0x10000029;
/// 模型类型错误
pub const MV_ALG_E_MODEL_TYPE: u32 = 0x1000002A;
/// 分配内存错误
pub const MV_ALG_E_MALLOC_MEM: u32 = 0x1000002B;
/// 线程绑核失败
pub const MV_ALG_E_BIND_CORE_FAILED: u32 = 0x1000002C;

// 降噪特有错误码
/// 噪声特性图像格式错误
pub const MV_ALG_E_DENOISE_NE_IMG_FORMAT: u32 = 0x10402001;
/// 噪声特性类型错误
pub const MV_ALG_E_DENOISE_NE_FEATURE_TYPE: u32 = 0x10402002;
/// 噪声特性个数错误
pub const MV_ALG_E_DENOISE_NE_PROFILE_NUM: u32 = 0x10402003;
/// 噪声特性增益个数错误
pub const MV_ALG_E_DENOISE_NE_GAIN_NUM: u32 = 0x10402004;
/// 噪声曲线增益值输入错误
pub const MV_ALG_E_DENOISE_NE_GAIN_VAL: u32 = 0x10402005;
/// 噪声曲线柱数错误
pub const MV_ALG_E_DENOISE_NE_BIN_NUM: u32 = 0x10402006;
/// 噪声估计初始化增益设置错误
pub const MV_ALG_E_DENOISE_NE_INIT_GAIN: u32 = 0x10402007;
/// 噪声估计未初始化
pub const MV_ALG_E_DENOISE_NE_NOT_INIT: u32 = 0x10402008;
/// 颜色空间模式错误
pub const MV_ALG_E_DENOISE_COLOR_MODE: u32 = 0x10402009;
/// 图像ROI个数错误
pub const MV_ALG_E_DENOISE_ROI_NUM: u32 = 0x1040200A;
/// 图像ROI原点错误
pub const MV_ALG_E_DENOISE_ROI_ORI_PT: u32 = 0x1040200B;
/// 图像ROI大小错误
pub const MV_ALG_E_DENOISE_ROI_SIZE: u32 = 0x1040200C;
/// 输入的相机增益不存在(增益个数已达上限)
pub const MV_ALG_E_DENOISE_GAIN_NOT_EXIST: u32 = 0x1040200D;
/// 输入的相机增益不在范围内
pub const MV_ALG_E_DENOISE_GAIN_BEYOND_RANGE: u32 = 0x1040200E;
/// 输入的噪声特性内存大小错误
pub const MV_ALG_E_DENOISE_NP_BUF_SIZE: u32 = 0x1040200F;

// 去紫边特有错误码
/// 去紫边算法ROI原点错误
pub const MV_ALG_E_PFC_ROI_PT: u32 = 0x10405000;
/// 去紫边算法ROI大小错误
pub const MV_ALG_E_PFC_ROI_SIZE: u32 = 0x10405001;
/// 去紫边算法滤波核尺寸错误
pub const MV_ALG_E_PFC_KERNEL_SIZE: u32 = 0x10405002;
