extern crate libc;

pub mod cxs;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum Errorcode
{
    Success = 0,
    Failure = 1,
    Waiting = 2,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum CxsStateType
{
    CxsStateNone = 0,
    CxsStateInitialized = 1,
    CxsStateOfferSent = 2,
    CxsStateRequestReceived = 3,
    CxsStateAccepted = 4,
    CxsStateUnfulfilled = 5,
    CxsStateExpired = 6,
    CxsStateRevoked = 7,
}

#[repr(C)]
pub struct CxsStatus {
    pub handle: ::std::os::raw::c_int,
    pub status: ::std::os::raw::c_int,
    pub msg: *mut ::std::os::raw::c_char,
}
