//! MVS SDK 废弃接口定义
//!
//! 该模块包含海康威视 MVS SDK 中不建议使用的旧版 C API 接口绑定
//!
//! # 注意事项
//! 这些接口已被标记为废弃或不推荐使用，建议在新项目中使用更新的替代接口

#[allow(deprecated)]
#[allow(deprecated_in_future)]
use crate::{
    MV_CC_DEVICE_INFO, MV_FRAME_OUT_INFO_EX, MV_IMAGE_BASIC_INFO, MV_XML_NODE_FEATURE,
    MV_XML_NODES_LIST,
};
use core::ffi::{c_int, c_uchar, c_uint, c_void};
use std::ffi::c_char;

// ============================================================================
// 常量定义
// ============================================================================

// ============================================================================
// FFI 函数声明
// ============================================================================

unsafe extern "C" {

    /// 获取图像基本信息（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstInfo` - 返回给调用者有关相机图像基本信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_GetImageInfo(
        handle: *mut c_void,
        pstInfo: *mut MV_IMAGE_BASIC_INFO,
    ) -> c_int;

    /// 获取 GenICam 代理（已废弃）
    ///
    /// # 参数
    /// * `handle` - 句柄地址
    ///
    /// # 返回值
    /// GenICam 代理类指针，正常返回值非 NULL；异常返回 NULL
    #[deprecated]
    pub unsafe fn MV_CC_GetTlProxy(handle: *mut c_void) -> *mut c_void;

    /// 获取根节点（已废弃）
    ///
    /// # 参数
    /// * `handle` - 句柄
    /// * `pstNode` - 根节点信息结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_XML_GetRootNode(
        handle: *mut c_void,
        pstNode: *mut MV_XML_NODE_FEATURE,
    ) -> c_int;

    /// 从 xml 中获取指定节点的所有子节点，根节点为 Root（已废弃）
    ///
    /// # 参数
    /// * `handle` - 句柄
    /// * `pstNode` - 根节点信息结构体
    /// * `pstNodesList` - 节点列表结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_XML_GetChildren(
        handle: *mut c_void,
        pstNode: *mut MV_XML_NODE_FEATURE,
        pstNodesList: *mut MV_XML_NODES_LIST,
    ) -> c_int;

    /// 获得当前节点的属性（已废弃）
    ///
    /// # 参数
    /// * `handle` - 句柄
    /// * `pstNode` - 根节点信息结构体
    /// * `pstFeature` - 当前节点属性结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// pstFeature 具体结构体内容参考 MV_XML_FEATURE_x
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_XML_GetNodeFeature(
        handle: *mut c_void,
        pstNode: *mut MV_XML_NODE_FEATURE,
        pstFeature: *mut c_void,
    ) -> c_int;

    /// 更新节点（已废弃）
    ///
    /// # 参数
    /// * `handle` - 句柄
    /// * `enType` - 节点类型
    /// * `pstFeature` - 当前节点属性结构体
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_XML_UpdateNodeFeature(
        handle: *mut c_void,
        enType: crate::MV_XML_InterfaceType,
        pstFeature: *mut c_void,
    ) -> c_int;

    /// 注册更新回调（已废弃）
    ///
    /// # 参数
    /// * `handle` - 句柄
    /// * `cbUpdate` - 回调函数指针
    /// * `pUser` - 用户自定义变量
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 当调用 MV_XML_UpdateNodeFeature 接口更新节点属性时，
    /// 注册的回调函数 cbUpdate 会在 pstNodesList 中返回与之相关联的节点
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_XML_RegisterUpdateCallBack(
        handle: *mut c_void,
        cbUpdate: unsafe extern "C" fn(
            crate::MV_XML_InterfaceType,
            *mut c_void,
            *mut MV_XML_NODES_LIST,
            *mut c_void,
        ),
        pUser: *mut c_void,
    ) -> c_int;

    /// 获取一帧图像，此函数为查询式获取（已废弃）
    ///
    /// # 参数
    /// * `handle` - 句柄
    /// * `pData` - 图像数据接收指针（输出参数）
    /// * `nDataSize` - 接收缓存大小
    /// * `pFrameInfo` - 图像信息结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 每次调用查询内部缓存有无数据，有数据则返回数据，无数据返回错误码。
    /// 该接口已弃用，建议改用 MV_CC_GetOneFrameTimeOut 接口
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_GetOneFrame(
        handle: *mut c_void,
        pData: *mut c_uchar,
        nDataSize: c_uint,
        pFrameInfo: *mut crate::MV_FRAME_OUT_INFO,
    ) -> c_int;

    /// 获取一帧 trunck 数据，此函数为查询式获取（已废弃）
    ///
    /// # 参数
    /// * `handle` - 句柄
    /// * `pData` - 图像数据接收指针（输出参数）
    /// * `nDataSize` - 接收缓存大小
    /// * `pFrameInfo` - 图像信息结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 每次调用查询内部缓存有无数据，有数据则返回数据，无数据返回错误码。
    /// 该接口已弃用，建议改用 MV_CC_GetOneFrameTimeOut 接口
    #[deprecated]
    pub unsafe fn MV_CC_GetOneFrameEx(
        handle: *mut c_void,
        pData: *mut c_uchar,
        nDataSize: c_uint,
        pFrameInfo: *mut MV_FRAME_OUT_INFO_EX,
    ) -> c_int;

    /// 注册图像数据回调（已废弃）
    ///
    /// # 参数
    /// * `handle` - 句柄
    /// * `cbOutput` - 回调函数指针
    /// * `pUser` - 用户自定义变量
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口已弃用，建议改用 MV_CC_RegisterImageCallBackEx 接口
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_RegisterImageCallBack(
        handle: *mut c_void,
        cbOutput: unsafe extern "C" fn(*mut c_uchar, *mut crate::MV_FRAME_OUT_INFO, *mut c_void),
        pUser: *mut c_void,
    ) -> c_int;

    /// 保存图片（已废弃）
    ///
    /// # 参数
    /// * `pSaveParam` - 保存图片参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口仅支持 Windows 且已弃用，建议改用 MV_CC_SaveImageEx2 接口
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_SaveImage(pSaveParam: *mut crate::MV_SAVE_IMAGE_PARAM) -> c_int;

    /// 保存图片，支持 Bmp 和 Jpeg，编码质量在 50-99 之前（已废弃）
    ///
    /// # 参数
    /// * `pSaveParam` - 保存图片参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口仅支持 Windows 且已弃用，建议改用 MV_CC_SaveImageEx2 接口
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_SaveImageEx(pSaveParam: *mut crate::MV_SAVE_IMAGE_PARAM_EX) -> c_int;

    /// Bayer 噪声估计（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstNoiseEstimateParam` - Bayer 噪声估计参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口只支持 Bayer8/Bayer10/Bayer12 格式，其它Bayer格式需先转成Bayer8/Bayer10/Bayer12 格式。
    /// 该接口只有在打开我司特定彩色相机后才可以正常使用，当相机被断开或者掉线后，
    /// 继续使用该接口会报错。
    ///
    /// 该接口已弃用，建议改用 ISP Tool 方式进行标定
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_BayerNoiseEstimate(
        handle: *mut c_void,
        pstNoiseEstimateParam: *mut crate::MV_CC_BAYER_NOISE_ESTIMATE_PARAM,
    ) -> c_int;

    /// Bayer 空域降噪（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstSpatialDenoiseParam` - Bayer 空域降噪参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口只支持 Bayer8/Bayer10/Bayer12 格式，其它Bayer格式需先转成Bayer8/Bayer10/Bayer12 格式。
    /// 该接口只有在打开我司特定彩色相机后才可以正常使用，当相机被断开或者掉线后，
    /// 继续使用该接口会报错。
    ///
    /// 该接口已弃用，建议改用 ISP Tool 方式进行降噪
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_BayerSpatialDenoise(
        handle: *mut c_void,
        pstSpatialDenoiseParam: *mut crate::MV_CC_BAYER_SPATIAL_DENOISE_PARAM,
    ) -> c_int;

    /// 设置 Bayer 格式的 CLUT 使能和信息（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstCLUTParam` - CLUT 参数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 开启 CLUT 并设置 CLUT 信息后，在调用 MV_CC_ConvertPixelType、MV_CC_SaveImageEx2 接口
    /// 将 Bayer8/10/12/16 格式转成 RGB24/48，RGBA32/64，BGR24/48，BGRA32/64 时起效。
    ///
    /// 该接口已弃用，建议改用 ISP Tool 方式进行设置
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_SetBayerCLUTParam(
        handle: *mut c_void,
        pstCLUTParam: *mut crate::MV_CC_CLUT_PARAM,
    ) -> c_int;

    /// 图像锐化（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstSharpenParam` - 锐化参数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口已弃用，建议改用 ISP Tool 方式进行锐化
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_ImageSharpen(
        handle: *mut c_void,
        pstSharpenParam: *mut crate::MV_CC_SHARPEN_PARAM,
    ) -> c_int;

    /// 色彩校正（包括 CCM 和 CLUT）（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstColorCorrectParam` - 色彩校正参数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口支持单独 CCM 或者 CLUT，也支持同时进行 CCM 和 CLUT，
    /// 用户可以通过 CCM 和 CLUT 信息中的使能开关进行选择。
    ///
    /// 该接口已弃用，建议改用 ISP Tool 方式进行校正
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_ColorCorrect(
        handle: *mut c_void,
        pstColorCorrectParam: *mut crate::MV_CC_COLOR_CORRECT_PARAM,
    ) -> c_int;

    /// 噪声估计（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstNoiseEstimateParam` - 噪声估计参数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 如果用户选择全图做噪声估计，nROINum 可输入 0，pstROIRect 可置空。
    ///
    /// 该接口已弃用，建议改用 ISP Tool 方式进行标定
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_NoiseEstimate(
        handle: *mut c_void,
        pstNoiseEstimateParam: *mut crate::MV_CC_NOISE_ESTIMATE_PARAM,
    ) -> c_int;

    /// 空域降噪（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstSpatialDenoiseParam` - 空域降噪参数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口已弃用，建议改用 ISP Tool 方式进行降噪
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_SpatialDenoise(
        handle: *mut c_void,
        pstSpatialDenoiseParam: *mut crate::MV_CC_SPATIAL_DENOISE_PARAM,
    ) -> c_int;

    /// LSC 标定（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstLSCCalibParam` - 标定参数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_LSCCalib(
        handle: *mut c_void,
        pstLSCCalibParam: *mut crate::MV_CC_LSC_CALIB_PARAM,
    ) -> c_int;

    /// LSC 校正（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstLSCCorrectParam` - 校正参数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_LSCCorrect(
        handle: *mut c_void,
        pstLSCCorrectParam: *mut crate::MV_CC_LSC_CORRECT_PARAM,
    ) -> c_int;

    /// 强制 IP（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nIP` - 设置的 IP
    ///
    /// # 返回值
    /// 见返回错误码
    ///
    /// # 注意事项
    /// 该接口已弃用，建议改用 MV_GIGE_ForceIpEx 接口
    #[deprecated]
    pub unsafe fn MV_GIGE_ForceIp(handle: *mut c_void, nIP: c_uint) -> c_int;

    /// 注册事件回调（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `cbEvent` - 事件回调函数指针
    /// * `pUser` - 用户自定义变量
    ///
    /// # 返回值
    /// 见返回错误码
    ///
    /// # 注意事项
    /// 该接口只支持网口设备，不支持 U 口和 GenTL 设备。
    /// 该接口已弃用，建议改用 MV_CC_RegisterEventCallBackEx 接口
    #[deprecated]
    pub unsafe fn MV_CC_RegisterEventCallBack(
        handle: *mut c_void,
        cbEvent: unsafe extern "C" fn(c_uint, *mut c_void),
        pUser: *mut c_void,
    ) -> c_int;

    /// 显示图像，注册显示窗口，内部自动显示（已废弃）
    ///
    /// # 参数
    /// * `handle` - 句柄
    /// * `hWnd` - 显示窗口句柄
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 与 MV_CC_GetImageBuffer 不能同时使用，
    /// 建议改用 MV_CC_DisplayOneFrame 接口
    #[deprecated]
    pub unsafe fn MV_CC_Display(handle: *mut c_void, hWnd: *mut c_void) -> c_int;

    /// 获取 Integer 属性值（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `strKey` - 属性键值，如获取宽度信息则为"Width"
    /// * `pIntValue` - 返回给调用者有关相机属性结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 建议改用 MV_CC_GetIntValueEx 接口
    #[deprecated]
    pub unsafe fn MV_CC_GetIntValue(
        handle: *mut c_void,
        strKey: *const c_char,
        pIntValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置 Integer 型属性值（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `strKey` - 属性键值，如获取宽度信息则为"Width"
    /// * `nValue` - 想要设置的相机的属性值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 建议改用 MV_CC_SetIntValueEx 接口
    #[deprecated]
    pub unsafe fn MV_CC_SetIntValue(
        handle: *mut c_void,
        strKey: *const c_char,
        nValue: c_uint,
    ) -> c_int;

    /// 获取图像宽度（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机宽度的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    ///
    /// # 注意事项
    /// 返回的 pstValue 结构体的意义：
    /// - nCurValue: 代表相机当前的宽度值
    /// - nMax: 表示相机允许的最大可设置的宽度值
    /// - nMin: 表示相机允许的最小可设置的宽度值
    /// - nInc: 表示相机设置的宽度增量必须是 nInc 的倍数，否则无效
    ///
    /// 其他整型结构体参数的接口可参照此接口
    #[deprecated]
    pub unsafe fn MV_CC_GetWidth(handle: *mut c_void, pstValue: *mut crate::MVCC_INTVALUE)
    -> c_int;

    /// 设置图像宽度（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的相机宽度的值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机宽度将会更改为相应值，失败返回错误码
    ///
    /// # 注意事项
    /// 此宽度值必须是 MV_CC_GetWidth 接口返回的 pstValue 中的 nInc 的倍数才能设置成功
    #[deprecated]
    pub unsafe fn MV_CC_SetWidth(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取图像高度（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机高度的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并将高度信息返回到结构体中，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetHeight(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置图像高度（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 相机高度值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机高度将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetHeight(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取图像 X 偏移（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机 X 偏移的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetAOIoffsetX(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置图像 AOI 偏移（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 相机 X 偏移值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机 AOI 偏移将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetAOIoffsetX(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取图像 Y 偏移（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机 Y 偏移的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetAOIoffsetY(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置图像 Y 偏移（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 相机 Y 偏移值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机 Y 偏移将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetAOIoffsetY(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取曝光下限（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机曝光值下限结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetAutoExposureTimeLower(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置曝光值下限（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 曝光值下限
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机曝光下限将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetAutoExposureTimeLower(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取曝光上限（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机曝光值上限结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetAutoExposureTimeUpper(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置曝光值上限（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 曝光值上限
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机曝光上限将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetAutoExposureTimeUpper(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取亮度值（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机亮度结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetBrightness(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置亮度值（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 亮度值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机亮度将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetBrightness(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取帧率（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机帧率的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    ///
    /// # 注意事项
    /// 返回的 pstValue 结构体的意义：
    /// - fCurValue: 表示相机当前的帧率
    /// - fMax: 表示相机允许设置的最大帧率
    /// - fMin: 表示相机允许设置的最小帧率
    ///
    /// 其他浮点型结构体参数的接口可参照此接口
    #[deprecated]
    pub unsafe fn MV_CC_GetFrameRate(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_FLOATVALUE,
    ) -> c_int;

    /// 设置帧率（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `fValue` - 相机帧率
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机帧率将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetFrameRate(handle: *mut c_void, fValue: f32) -> c_int;

    /// 获取增益（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机增益的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    ///
    /// # 注意事项
    /// 返回的 pstValue 结构体的意义：
    /// - fCurValue: 相机当前的增益
    /// - fMax: 相机允许的最大增益
    /// - fMin: 相机允许的最小增益
    #[deprecated]
    pub unsafe fn MV_CC_GetGain(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_FLOATVALUE,
    ) -> c_int;

    /// 设置增益（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `fValue` - 增益值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机增益值将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetGain(handle: *mut c_void, fValue: f32) -> c_int;

    /// 获取曝光时间（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机曝光时间的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    ///
    /// # 注意事项
    /// 返回的 pstValue 结构体的意义：
    /// - fCurValue: 表示相机当前的帧率
    ///
    /// 其他浮点型结构体参数的接口可参照此接口
    #[deprecated]
    pub unsafe fn MV_CC_GetExposureTime(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_FLOATVALUE,
    ) -> c_int;

    /// 设置曝光时间（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `fValue` - 曝光时间
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机曝光时间值将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetExposureTime(handle: *mut c_void, fValue: f32) -> c_int;

    /// 获取像素格式（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者的有关像素格式的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    ///
    /// # 注意事项
    /// 返回的 pstValue 结构体的意义：
    /// - nCurValue: 相机当前的像素格式，是枚举类型
    /// - nSupportedNum: 相机支持的像素格式的个数
    /// - nSupportValue\[MV_MAX_XML_SYMBOLIC_NUM\]: 相机所有支持的像素格式对应的整型值列表
    ///
    /// 其他枚举类型参数接口可参照此接口
    #[deprecated]
    pub unsafe fn MV_CC_GetPixelFormat(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_ENUMVALUE,
    ) -> c_int;

    /// 设置像素格式（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 要设置的像素格式对应的整型值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机像素格式将会更改为相应值，失败返回错误码
    ///
    /// # 注意事项
    /// 要设置的枚举类型必须是 Get 接口返回的 nSupportValue\[MV_MAX_XML_SYMBOLIC_NUM\]中的一种，否则会失败
    #[deprecated]
    pub unsafe fn MV_CC_SetPixelFormat(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取采集模式（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者的有关采集模式的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetAcquisitionMode(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_ENUMVALUE,
    ) -> c_int;

    /// 设置采集模式（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 要设置的采集模式对应的整型值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机采集模式将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetAcquisitionMode(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取增益模式（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者的有关增益模式的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetGainMode(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_ENUMVALUE,
    ) -> c_int;

    /// 设置增益模式（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 要设置的增益模式对应的整型值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机增益模式将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetGainMode(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取自动曝光模式（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者的有关自动曝光模式的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetExposureAutoMode(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_ENUMVALUE,
    ) -> c_int;

    /// 设置自动曝光模式（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 要设置的自动曝光模式对应的整型值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机自动曝光模式将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetExposureAutoMode(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取触发模式（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者的有关触发模式的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetTriggerMode(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_ENUMVALUE,
    ) -> c_int;

    /// 设置触发模式（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 要设置的触发模式对应的整型值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机触发模式将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetTriggerMode(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取触发延时（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机触发延时的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetTriggerDelay(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_FLOATVALUE,
    ) -> c_int;

    /// 设置触发延时（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `fValue` - 想要设置的相机触发延时
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机触发延时将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetTriggerDelay(handle: *mut c_void, fValue: f32) -> c_int;

    /// 获取触发源（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者的有关触发源的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetTriggerSource(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_ENUMVALUE,
    ) -> c_int;

    /// 设置触发源（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 要设置的触发源对应的整型值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机触发源将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetTriggerSource(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 软触发一次（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 接口仅在已选择的触发源为软件触发时有效
    #[deprecated]
    pub unsafe fn MV_CC_TriggerSoftwareExecute(handle: *mut c_void) -> c_int;

    /// 获取 Gamma 类型（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者的有关 Gamma 类型的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetGammaSelector(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_ENUMVALUE,
    ) -> c_int;

    /// 设置 Gamma 类型（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 要设置的 Gamma 类型对应的整型值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机 Gamma 类型将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetGammaSelector(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取 Gamma 值（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机 Gamma 值的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetGamma(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_FLOATVALUE,
    ) -> c_int;

    /// 设置 Gamma 值（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `fValue` - Gamma 值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机 Gamma 值将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetGamma(handle: *mut c_void, fValue: f32) -> c_int;

    /// 获取锐度（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机锐度结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetSharpness(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置锐度（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的锐度
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机锐度将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetSharpness(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取灰度（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机灰度结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetHue(handle: *mut c_void, pstValue: *mut crate::MVCC_INTVALUE) -> c_int;

    /// 设置灰度（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的灰度
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机灰度将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetHue(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取饱和度（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机饱和度结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetSaturation(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置饱和度（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的饱和度
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机饱和度将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetSaturation(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取自动白平衡（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者的有关自动白平衡的信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并获得相应参数信息的结构体，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetBalanceWhiteAuto(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_ENUMVALUE,
    ) -> c_int;

    /// 设置自动白平衡（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 要设置的自动白平衡对应的整型值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机自动白平衡将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetBalanceWhiteAuto(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取白平衡红（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机白平衡红结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetBalanceRatioRed(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置白平衡红（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的白平衡红
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机白平衡红将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetBalanceRatioRed(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取白平衡绿（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机白平衡绿结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetBalanceRatioGreen(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置白平衡绿（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的白平衡绿
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机白平衡绿将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetBalanceRatioGreen(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取白平衡蓝（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机白平衡蓝结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetBalanceRatioBlue(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置白平衡蓝（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的白平衡蓝
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机白平衡蓝将会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetBalanceRatioBlue(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取水印信息内包含的信息类型（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机水印信息内包含的信息类型结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetFrameSpecInfoAbility(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置水印信息内包含的信息类型（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的水印信息内包含的信息类型
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机水印信息内包含的信息类型会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetFrameSpecInfoAbility(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取设备自定义名字（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机名字结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且获取到相机的自定义名字，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetDeviceUserID(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_STRINGVALUE,
    ) -> c_int;

    /// 设置设备自定义名字（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `chValue` - 设备名字
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且设置设备自定义名字，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetDeviceUserID(handle: *mut c_void, chValue: *const c_char) -> c_int;

    /// 获取一次触发的帧数（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机一次触发的帧数结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetBurstFrameCount(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置一次触发的帧数（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的一次触发的帧数
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机一次触发的帧数会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetBurstFrameCount(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取行频（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机行频结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetAcquisitionLineRate(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置行频（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的行频
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机行频会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetAcquisitionLineRate(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取心跳信息（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机心跳信息结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_GetHeartBeatTimeout(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置心跳信息（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的心跳信息
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机心跳信息会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_CC_SetHeartBeatTimeout(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取网络包大小（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机网络包大小结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_GIGE_GetGevSCPSPacketSize(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置网络包大小（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的网络包大小
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机网络包大小会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_GIGE_SetGevSCPSPacketSize(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取网络包发送间隔（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pstValue` - 返回给调用者有关相机网络包发送间隔结构体指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_GIGE_GetGevSCPD(
        handle: *mut c_void,
        pstValue: *mut crate::MVCC_INTVALUE,
    ) -> c_int;

    /// 设置网络包发送间隔（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nValue` - 想要设置的网络包发送间隔
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机网络包发送间隔会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_GIGE_SetGevSCPD(handle: *mut c_void, nValue: c_uint) -> c_int;

    /// 获取接收端 IP 地址（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pnIP` - 返回给调用者接收端 IP 地址（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 0xa9fe0102 表示 169.254.1.2
    #[deprecated]
    pub unsafe fn MV_GIGE_GetGevSCDA(handle: *mut c_void, pnIP: *mut c_uint) -> c_int;

    /// 设置接收端 IP 地址（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nIP` - 想要设置的接收端 IP 地址
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机接收端 IP 地址会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_GIGE_SetGevSCDA(handle: *mut c_void, nIP: c_uint) -> c_int;

    /// 获取发送端的端口号（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `pnPort` - 返回给调用者发送端的端口号（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_GIGE_GetGevSCSP(handle: *mut c_void, pnPort: *mut c_uint) -> c_int;

    /// 设置发送端的端口号（已废弃）
    ///
    /// # 参数
    /// * `handle` - 相机句柄
    /// * `nPort` - 想要设置的发送端的端口号
    ///
    /// # 返回值
    /// 成功返回 MV_OK，并且相机发送端的端口号会更改为相应值，失败返回错误码
    #[deprecated]
    pub unsafe fn MV_GIGE_SetGevSCSP(handle: *mut c_void, nPort: c_uint) -> c_int;

    /// 设置设备波特率（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `nBaudrate` - 设置的波特率值
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 数值参考 CameraParams.h 中宏定义，如#define MV_CAML_BAUDRATE_9600  0x00000001
    /// 该接口已弃用，建议改用 MV_CAML_SetDeviceBaudrate 接口
    #[deprecated]
    pub unsafe fn MV_CAML_SetDeviceBauderate(handle: *mut c_void, nBaudrate: c_uint) -> c_int;

    /// 获取设备波特率（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnCurrentBaudrate` - 波特率信息指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 数值参考 CameraParams.h 中宏定义，如#define MV_CAML_BAUDRATE_9600  0x00000001
    /// 该接口已弃用，建议改用 MV_CAML_GetDeviceBaudrate 接口
    #[deprecated]
    pub unsafe fn MV_CAML_GetDeviceBauderate(
        handle: *mut c_void,
        pnCurrentBaudrate: *mut c_uint,
    ) -> c_int;

    /// 获取设备与主机间连接支持的波特率（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pnBaudrateAblity` - 支持的波特率信息的指针（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 所支持波特率的或运算结果，单个数值参考 CameraParams.h 中宏定义，如 MV_CAML_BAUDRATE_9600  0x00000001
    /// 该接口已弃用，建议改用 MV_CAML_GetSupportBaudrates 接口
    #[deprecated]
    pub unsafe fn MV_CAML_GetSupportBauderates(
        handle: *mut c_void,
        pnBaudrateAblity: *mut c_uint,
    ) -> c_int;

    /// 保存图片，支持 Bmp 和 Jpeg（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstSaveParam` - 保存图片参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// MV_CC_SaveImageEx2 比 MV_CC_SaveImageEx 增加参数 handle，为了保证与其他接口的统一
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_SaveImageEx2(
        handle: *mut c_void,
        pstSaveParam: *mut crate::MV_SAVE_IMAGE_PARAM_EX,
    ) -> c_int;

    /// 保存图像到文件（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstSaveFileParam` - 保存图片文件参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 该接口支持 BMP/JPEG/PNG/TIFF
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_SaveImageToFile(
        handle: *mut c_void,
        pstSaveFileParam: *mut crate::MV_SAVE_IMG_TO_FILE_PARAM,
    ) -> c_int;

    /// 像素格式转换（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstCvtParam` - 像素格式转换参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 如果设备当前采集图像是 JPEG 压缩的格式，则不支持调用该接口进行转换
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_ConvertPixelType(
        handle: *mut c_void,
        pstCvtParam: *mut crate::MV_CC_PIXEL_CONVERT_PARAM,
    ) -> c_int;

    /// 设置 SDK 日志路径（已废弃）
    ///
    /// # 参数
    /// * `strSDKLogPath` - SDK 日志路径
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// V2.4.1 版本新增日志服务，开启服务之后该接口无效，默认日志服务为开启状态
    #[deprecated]
    pub unsafe fn MV_CC_SetSDKLogPath(strSDKLogPath: *const c_char) -> c_int;

    /// 显示一帧图像（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstDisplayInfo` - 图像信息
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 与设备类型无关，渲染模式为 D3D 时，支持的最大分辨率为 16384 * 163840
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_DisplayOneFrame(
        handle: *mut c_void,
        pstDisplayInfo: *mut crate::MV_DISPLAY_FRAME_INFO,
    ) -> c_int;

    /// 获取支持的传输层（已废弃）
    ///
    /// # 返回值
    /// 支持的传输层编号
    ///
    /// # 注意事项
    /// 返回是设备的传输层，比如（MV_GIGE_DEVICE | MV_USB_DEVICE | MV_GENTL_XOF_DEVICE 等），不包含采集卡的类型
    #[deprecated]
    pub unsafe fn MV_CC_EnumerateTls() -> c_int;

    /// 创建设备句柄，不生成日志（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄（输出参数）
    /// * `pstDevInfo` - 设备信息结构体
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 通过该接口创建句柄，调用 SDK 接口，不会默认生成 SDK 日志文件，
    /// 如果需要生成日志文件可以通过 MV_CC_CreateHandle 创建句柄，日志文件自动生成
    #[deprecated]
    pub unsafe fn MV_CC_CreateHandleWithoutLog(
        handle: *mut *mut c_void,
        pstDevInfo: *const MV_CC_DEVICE_INFO,
    ) -> c_int;

    /// 注册图像数据回调，RGB（已废弃）
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
    /// 该接口不支持 MV_CAMERALINK_DEVICE 类型的设备
    #[deprecated]
    pub unsafe fn MV_CC_RegisterImageCallBackForRGB(
        handle: *mut c_void,
        cbOutput: unsafe extern "C" fn(*mut c_uchar, *mut MV_FRAME_OUT_INFO_EX, *mut c_void),
        pUser: *mut c_void,
    ) -> c_int;

    /// 注册图像数据回调，BGR（已废弃）
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
    /// 该接口不支持 MV_CAMERALINK_DEVICE 类型的设备
    #[deprecated]
    pub unsafe fn MV_CC_RegisterImageCallBackForBGR(
        handle: *mut c_void,
        cbOutput: unsafe extern "C" fn(*mut c_uchar, *mut MV_FRAME_OUT_INFO_EX, *mut c_void),
        pUser: *mut c_void,
    ) -> c_int;

    /// 获取一帧 RGB 数据，此函数为查询式获取（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pData` - 图像数据接收指针（输出参数）
    /// * `nDataSize` - 接收缓存大小
    /// * `pstFrameInfo` - 图像信息结构体（输出参数）
    /// * `nMsec` - 等待超时时间
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 因为图像转换成 RGB24 格式有耗时，所以当数据帧率过高时该接口可能会导致丢帧。
    /// 该接口不支持 MV_CAMERALINK_DEVICE 设备。
    #[deprecated]
    pub unsafe fn MV_CC_GetImageForRGB(
        handle: *mut c_void,
        pData: *mut c_uchar,
        nDataSize: c_uint,
        pstFrameInfo: *mut MV_FRAME_OUT_INFO_EX,
        nMsec: c_int,
    ) -> c_int;

    /// 获取一帧 BGR 数据，此函数为查询式获取（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pData` - 图像数据接收指针（输出参数）
    /// * `nDataSize` - 接收缓存大小
    /// * `pstFrameInfo` - 图像信息结构体（输出参数）
    /// * `nMsec` - 等待超时时间
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 因为图像转换成 BGR24 格式有耗时，所以当数据帧率过高时该接口可能会导致丢帧。
    /// 该接口不支持 CameraLink 设备。
    #[deprecated]
    pub unsafe fn MV_CC_GetImageForBGR(
        handle: *mut c_void,
        pData: *mut c_uchar,
        nDataSize: c_uint,
        pstFrameInfo: *mut MV_FRAME_OUT_INFO_EX,
        nMsec: c_int,
    ) -> c_int;

    /// 打开获取或设置相机参数的 GUI 界面（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 限制：在同一线程中多相机同时调用该接口，只能打开当前一个 GUI 界面，
    /// 需要关闭当前相机 GUI 界面后，才可打开另一个相机的 GUI 界面（后续版本优化）
    /// 该接口仅支持 windows 平台
    #[deprecated]
    pub unsafe fn MV_CC_OpenParamsGUI(handle: *mut c_void) -> c_int;

    /// 保存 3D 点云数据，支持 PLY、CSV 和 OBJ 三种格式（已废弃）
    ///
    /// # 参数
    /// * `handle` - 设备句柄
    /// * `pstPointDataParam` - 保存点云数据参数结构体（输出参数）
    ///
    /// # 返回值
    /// 成功返回 MV_OK，失败返回错误码
    ///
    /// # 注意事项
    /// 目前支持 PixelType_Gvsp_Coord3D_ABC32、PixelType_Gvsp_Coord3D_ABC32f、
    /// PixelType_Gvsp_Coord3D_AB32、PixelType_Gvsp_Coord3D_AB32f、
    /// PixelType_Gvsp_Coord3D_AC32、PixelType_Gvsp_Coord3D_AC32f，
    /// 暂不支持其他 3D 格式
    #[allow(deprecated)]
    #[deprecated]
    pub unsafe fn MV_CC_SavePointCloudData(
        handle: *mut c_void,
        pstPointDataParam: *mut crate::MV_SAVE_POINT_CLOUD_PARAM,
    ) -> c_int;

} // unsafe extern "C"
