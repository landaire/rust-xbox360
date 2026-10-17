//! Ordinals of `xam.xex` exports that this crate does not import, for
//! callers that resolve them at run time. Named as Xenia names the exports.

use crate::Ordinal;

/// `NetDll_XnpGetSecAssocList`
pub const XNP_GET_SEC_ASSOC_LIST: Ordinal = Ordinal::new(0x81).unwrap();
/// `NetDll_XnpGetKeyList`
pub const XNP_GET_KEY_LIST: Ordinal = Ordinal::new(0x82).unwrap();
/// `NetDll_XnpGetQosLookupList`
pub const XNP_GET_QOS_LOOKUP_LIST: Ordinal = Ordinal::new(0x83).unwrap();
/// `NetDll_XnpToolSetCallbacks`
pub const XNP_TOOL_SET_CALLBACKS: Ordinal = Ordinal::new(0x8b).unwrap();
/// `NetDll_XnpToolIpProxyInject`
pub const XNP_TOOL_IP_PROXY_INJECT: Ordinal = Ordinal::new(0x8c).unwrap();
/// `NetDll_XnpEthernetInterceptXmitAsIp`
pub const XNP_ETHERNET_INTERCEPT_XMIT_AS_IP: Ordinal = Ordinal::new(0x8e).unwrap();
/// `XamMediaVerificationCreate`
pub const MEDIA_VERIFICATION_CREATE: Ordinal = Ordinal::new(0x9bb).unwrap();
/// `XamMediaVerificationClose`
pub const MEDIA_VERIFICATION_CLOSE: Ordinal = Ordinal::new(0x9bc).unwrap();
/// `XamMediaVerificationVerify`
pub const MEDIA_VERIFICATION_VERIFY: Ordinal = Ordinal::new(0x9bd).unwrap();
/// `XamMediaVerificationFailedBlocks`
pub const MEDIA_VERIFICATION_FAILED_BLOCKS: Ordinal = Ordinal::new(0x9be).unwrap();
/// `XamMediaVerificationInject`
pub const MEDIA_VERIFICATION_INJECT: Ordinal = Ordinal::new(0x9bf).unwrap();
/// `XamShowBadDiscErrorUI`
pub const SHOW_BAD_DISC_ERROR_UI: Ordinal = Ordinal::new(0x9c0).unwrap();
