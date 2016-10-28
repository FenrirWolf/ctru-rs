/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum NFC_OpType { NFC_OpType_1 = 1, NFC_OpType_NFCTag = 2, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum NFC_TagState {
    NFC_TagState_Uninitialized = 0,
    NFC_TagState_ScanningStopped = 1,
    NFC_TagState_Scanning = 2,
    NFC_TagState_InRange = 3,
    NFC_TagState_OutOfRange = 4,
    NFC_TagState_DataReady = 5,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum Enum_Unnamed1 {
    NFC_amiiboFlag_Setup = 16,
    NFC_amiiboFlag_AppDataSetup = 32,
}
#[repr(C)]
#[derive(Copy)]
pub struct NFC_TagInfo {
    pub id_offset_size: u16_,
    pub unk_x2: u8_,
    pub unk_x3: u8_,
    pub id: [u8_; 40usize],
}
impl ::core::clone::Clone for NFC_TagInfo {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for NFC_TagInfo {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct NFC_AmiiboSettings {
    pub mii: [u8_; 96usize],
    pub nickname: [u16_; 11usize],
    pub flags: u8_,
    pub countrycodeid: u8_,
    pub setupdate_year: u16_,
    pub setupdate_month: u8_,
    pub setupdate_day: u8_,
    pub unk_x7c: [u8_; 44usize],
}
impl ::core::clone::Clone for NFC_AmiiboSettings {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for NFC_AmiiboSettings {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct NFC_AmiiboConfig {
    pub lastwritedate_year: u16_,
    pub lastwritedate_month: u8_,
    pub lastwritedate_day: u8_,
    pub write_counter: u16_,
    pub val_x6: u16_,
    pub val_x8: u8_,
    pub val_x9: u8_,
    pub val_xa: u16_,
    pub val_xc: u8_,
    pub pagex4_byte3: u8_,
    pub appdata_size: u8_,
    pub zeros: [u8_; 49usize],
}
impl ::core::clone::Clone for NFC_AmiiboConfig {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for NFC_AmiiboConfig {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct NFC_AppDataInitStruct {
    pub data_x0: [u8_; 12usize],
    pub data_xc: [u8_; 48usize],
}
impl ::core::clone::Clone for NFC_AppDataInitStruct {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for NFC_AppDataInitStruct {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct NFC_AppDataWriteStruct {
    pub id: [u8_; 10usize],
    pub id_size: u8_,
    pub unused_xb: [u8_; 21usize],
}
impl ::core::default::Default for NFC_AppDataWriteStruct {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}
extern "C" {
    pub fn nfcInit(type_: NFC_OpType) -> Result;
    pub fn nfcExit();
    pub fn nfcGetSessionHandle() -> Handle;
    pub fn nfcStartScanning(inval: u16_) -> Result;
    pub fn nfcStopScanning();
    pub fn nfcLoadAmiiboData() -> Result;
    pub fn nfcResetTagScanState() -> Result;
    pub fn nfcUpdateStoredAmiiboData() -> Result;
    pub fn nfcGetTagState(state: *mut NFC_TagState) -> Result;
    pub fn nfcGetTagInfo(out: *mut NFC_TagInfo) -> Result;
    pub fn nfcOpenAppData(amiibo_appid: u32_) -> Result;
    pub fn nfcInitializeWriteAppData(amiibo_appid: u32_,
                                     buf: *const ::libc::c_void, size: size_t)
     -> Result;
    pub fn nfcReadAppData(buf: *mut ::libc::c_void, size: size_t) -> Result;
    pub fn nfcWriteAppData(buf: *const ::libc::c_void, size: size_t,
                           taginfo: *mut NFC_TagInfo) -> Result;
    pub fn nfcGetAmiiboSettings(out: *mut NFC_AmiiboSettings) -> Result;
    pub fn nfcGetAmiiboConfig(out: *mut NFC_AmiiboConfig) -> Result;
}
use ::types::*;
