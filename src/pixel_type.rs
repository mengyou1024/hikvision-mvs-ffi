//! 像素类型定义
//!
//! 该模块包含了GigE Vision (2.0.03) 像素格式的定义

// 指示像素是单色还是RGB
pub const MV_GVSP_PIX_MONO: u32 = 0x01000000;
#[deprecated(note = "在版本1.1中已废弃")]
pub const MV_GVSP_PIX_RGB: u32 = 0x02000000;
pub const MV_GVSP_PIX_COLOR: u32 = 0x02000000;
pub const MV_GVSP_PIX_CUSTOM: u32 = 0x80000000;
pub const MV_GVSP_PIX_COLOR_MASK: u32 = 0xFF000000;

/// 指示像素占用的有效位数（包括填充）
/// 可用于计算存储图像所需的内存量
macro_rules! MV_PIXEL_BIT_COUNT {
    ($n:expr) => {
        ($n) << 16
    };
}

pub const MV_GVSP_PIX_EFFECTIVE_PIXEL_SIZE_MASK: u32 = 0x00FF0000;
pub const MV_GVSP_PIX_EFFECTIVE_PIXEL_SIZE_SHIFT: u32 = 16;

/// 像素ID：像素格式的低16位
pub const MV_GVSP_PIX_ID_MASK: u32 = 0x0000FFFF;
/// 下一个可用的像素ID
pub const MV_GVSP_PIX_COUNT: u32 = 0x46;

/// GigE Vision像素类型枚举
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MvGvspPixelType {
    // 未定义的像素类型
    PixelType_Gvsp_Undefined = 0xFFFFFFFF,

    // 单色缓冲区格式定义
    PixelType_Gvsp_Mono1p = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(1) | 0x0037),
    PixelType_Gvsp_Mono2p = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(2) | 0x0038),
    PixelType_Gvsp_Mono4p = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(4) | 0x0039),
    PixelType_Gvsp_Mono8 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x0001),
    PixelType_Gvsp_Mono8_Signed = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x0002),
    PixelType_Gvsp_Mono10 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0003),
    PixelType_Gvsp_Mono10_Packed = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x0004),
    PixelType_Gvsp_Mono12 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0005),
    PixelType_Gvsp_Mono12_Packed = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x0006),
    PixelType_Gvsp_Mono14 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0025),
    PixelType_Gvsp_Mono16 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0007),

    // Bayer缓冲区格式定义
    PixelType_Gvsp_BayerGR8 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x0008),
    PixelType_Gvsp_BayerRG8 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x0009),
    PixelType_Gvsp_BayerGB8 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x000A),
    PixelType_Gvsp_BayerBG8 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x000B),
    PixelType_Gvsp_BayerRBGG8 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x0046),
    PixelType_Gvsp_BayerGR10 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x000C),
    PixelType_Gvsp_BayerRG10 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x000D),
    PixelType_Gvsp_BayerGB10 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x000E),
    PixelType_Gvsp_BayerBG10 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x000F),
    PixelType_Gvsp_BayerGR12 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0010),
    PixelType_Gvsp_BayerRG12 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0011),
    PixelType_Gvsp_BayerGB12 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0012),
    PixelType_Gvsp_BayerBG12 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0013),
    PixelType_Gvsp_BayerGR10_Packed = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x0026),
    PixelType_Gvsp_BayerRG10_Packed = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x0027),
    PixelType_Gvsp_BayerGB10_Packed = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x0028),
    PixelType_Gvsp_BayerBG10_Packed = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x0029),
    PixelType_Gvsp_BayerGR12_Packed = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x002A),
    PixelType_Gvsp_BayerRG12_Packed = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x002B),
    PixelType_Gvsp_BayerGB12_Packed = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x002C),
    PixelType_Gvsp_BayerBG12_Packed = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x002D),
    PixelType_Gvsp_BayerGR16 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x002E),
    PixelType_Gvsp_BayerRG16 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x002F),
    PixelType_Gvsp_BayerGB16 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0030),
    PixelType_Gvsp_BayerBG16 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0031),

    // RGB打包缓冲区格式定义
    PixelType_Gvsp_RGB8_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(24) | 0x0014),
    PixelType_Gvsp_BGR8_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(24) | 0x0015),
    PixelType_Gvsp_RGBA8_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(32) | 0x0016),
    PixelType_Gvsp_BGRA8_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(32) | 0x0017),
    PixelType_Gvsp_RGB10_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(48) | 0x0018),
    PixelType_Gvsp_BGR10_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(48) | 0x0019),
    PixelType_Gvsp_RGB12_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(48) | 0x001A),
    PixelType_Gvsp_BGR12_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(48) | 0x001B),
    PixelType_Gvsp_RGB16_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(48) | 0x0033),
    PixelType_Gvsp_BGR16_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(48) | 0x004B),
    PixelType_Gvsp_RGBA16_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(64) | 0x0064),
    PixelType_Gvsp_BGRA16_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(64) | 0x0051),
    PixelType_Gvsp_RGB10V1_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(32) | 0x001C),
    PixelType_Gvsp_RGB10V2_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(32) | 0x001D),
    PixelType_Gvsp_RGB12V1_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(36) | 0x0034),
    PixelType_Gvsp_RGB565_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(16) | 0x0035),
    PixelType_Gvsp_BGR565_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(16) | 0x0036),

    // YUV打包缓冲区格式定义
    PixelType_Gvsp_YUV411_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(12) | 0x001E),
    PixelType_Gvsp_YUV422_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(16) | 0x001F),
    PixelType_Gvsp_YUV422_YUYV_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(16) | 0x0032),
    PixelType_Gvsp_YUV444_Packed = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(24) | 0x0020),
    PixelType_Gvsp_YCBCR8_CBYCR = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(24) | 0x003A),
    PixelType_Gvsp_YCBCR422_8 = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(16) | 0x003B),
    PixelType_Gvsp_YCBCR422_8_CBYCRY = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(16) | 0x0043),
    PixelType_Gvsp_YCBCR411_8_CBYYCRYY = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(12) | 0x003C),
    PixelType_Gvsp_YCBCR601_8_CBYCR = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(24) | 0x003D),
    PixelType_Gvsp_YCBCR601_422_8 = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(16) | 0x003E),
    PixelType_Gvsp_YCBCR601_422_8_CBYCRY = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(16) | 0x0044),
    PixelType_Gvsp_YCBCR601_411_8_CBYYCRYY = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(12) | 0x003F),
    PixelType_Gvsp_YCBCR709_8_CBYCR = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(24) | 0x0040),
    PixelType_Gvsp_YCBCR709_422_8 = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(16) | 0x0041),
    PixelType_Gvsp_YCBCR709_422_8_CBYCRY = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(16) | 0x0045),
    PixelType_Gvsp_YCBCR709_411_8_CBYYCRYY = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(12) | 0x0042),

    // YUV420
    PixelType_Gvsp_YUV420SP_NV12 = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(12) | 0x8001),
    PixelType_Gvsp_YUV420SP_NV21 = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(12) | 0x8002),

    // RGB平面缓冲区格式定义
    PixelType_Gvsp_RGB8_Planar = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(24) | 0x0021),
    PixelType_Gvsp_RGB10_Planar = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(48) | 0x0022),
    PixelType_Gvsp_RGB12_Planar = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(48) | 0x0023),
    PixelType_Gvsp_RGB16_Planar = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(48) | 0x0024),

    // 自定义的图片格式
    PixelType_Gvsp_Jpeg = (MV_GVSP_PIX_CUSTOM | MV_PIXEL_BIT_COUNT!(24) | 0x0001),

    PixelType_Gvsp_Coord3D_ABC32f = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(96) | 0x00C0), //0x026000C0
    PixelType_Gvsp_Coord3D_ABC32f_Planar = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(96) | 0x00C1), //0x026000C1

    // 该值被废弃，请参考PixelType_Gvsp_Coord3D_AC32f_64
    PixelType_Gvsp_Coord3D_AC32f = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(40) | 0x00C2),
    // 该值被废弃 (已放入Chunkdata)
    PixelType_Gvsp_COORD3D_DEPTH_PLUS_MASK =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(28) | 0x0001),

    PixelType_Gvsp_Coord3D_ABC32 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(96) | 0x3001), //0x82603001
    PixelType_Gvsp_Coord3D_AB32f =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(64) | 0x3002), //0x82403002
    PixelType_Gvsp_Coord3D_AB32 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(64) | 0x3003), //0x82403003
    PixelType_Gvsp_Coord3D_AC32f_64 = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(64) | 0x00C2), //0x024000C2
    PixelType_Gvsp_Coord3D_AC32f_Planar = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(64) | 0x00C3), //0x024000C3
    PixelType_Gvsp_Coord3D_AC32 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(64) | 0x3004), //0x82403004
    PixelType_Gvsp_Coord3D_A32f = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(32) | 0x00BD), //0x012000BD
    PixelType_Gvsp_Coord3D_A32 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(32) | 0x3005), //0x81203005
    PixelType_Gvsp_Coord3D_C32f = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(32) | 0x00BF), //0x012000BF
    PixelType_Gvsp_Coord3D_C32 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(32) | 0x3006), //0x81203006
    PixelType_Gvsp_Coord3D_ABC16 = (MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(48) | 0x00B9), //0x023000B9
    PixelType_Gvsp_Coord3D_C16 = (MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x00B8), //0x011000B8

    PixelType_Gvsp_Float32 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(32) | 0x0001), //0x81200001

    // 无损压缩像素格式定义
    PixelType_Gvsp_HB_Mono8 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x0001),
    PixelType_Gvsp_HB_Mono10 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0003),
    PixelType_Gvsp_HB_Mono10_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x0004),
    PixelType_Gvsp_HB_Mono12 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0005),
    PixelType_Gvsp_HB_Mono12_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x0006),
    PixelType_Gvsp_HB_Mono16 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0007),
    PixelType_Gvsp_HB_BayerGR8 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x0008),
    PixelType_Gvsp_HB_BayerRG8 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x0009),
    PixelType_Gvsp_HB_BayerGB8 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x000A),
    PixelType_Gvsp_HB_BayerBG8 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x000B),
    PixelType_Gvsp_HB_BayerRBGG8 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(8) | 0x0046),
    PixelType_Gvsp_HB_BayerGR10 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x000C),
    PixelType_Gvsp_HB_BayerRG10 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x000D),
    PixelType_Gvsp_HB_BayerGB10 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x000E),
    PixelType_Gvsp_HB_BayerBG10 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x000F),
    PixelType_Gvsp_HB_BayerGR12 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0010),
    PixelType_Gvsp_HB_BayerRG12 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0011),
    PixelType_Gvsp_HB_BayerGB12 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0012),
    PixelType_Gvsp_HB_BayerBG12 =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(16) | 0x0013),
    PixelType_Gvsp_HB_BayerGR10_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x0026),
    PixelType_Gvsp_HB_BayerRG10_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x0027),
    PixelType_Gvsp_HB_BayerGB10_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x0028),
    PixelType_Gvsp_HB_BayerBG10_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x0029),
    PixelType_Gvsp_HB_BayerGR12_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x002A),
    PixelType_Gvsp_HB_BayerRG12_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x002B),
    PixelType_Gvsp_HB_BayerGB12_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x002C),
    PixelType_Gvsp_HB_BayerBG12_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_MONO | MV_PIXEL_BIT_COUNT!(12) | 0x002D),
    PixelType_Gvsp_HB_YUV422_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(16) | 0x001F),
    PixelType_Gvsp_HB_YUV422_YUYV_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(16) | 0x0032),
    PixelType_Gvsp_HB_RGB8_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(24) | 0x0014),
    PixelType_Gvsp_HB_BGR8_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(24) | 0x0015),
    PixelType_Gvsp_HB_RGBA8_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(32) | 0x0016),
    PixelType_Gvsp_HB_BGRA8_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(32) | 0x0017),
    PixelType_Gvsp_HB_RGB16_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(48) | 0x0033),
    PixelType_Gvsp_HB_BGR16_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(48) | 0x004B),
    PixelType_Gvsp_HB_RGBA16_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(64) | 0x0064),
    PixelType_Gvsp_HB_BGRA16_Packed =
        (MV_GVSP_PIX_CUSTOM | MV_GVSP_PIX_COLOR | MV_PIXEL_BIT_COUNT!(64) | 0x0051),
}
