pub mod cxs;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Errorcode
{
    Success = 0,
    Failure,
    Waiting,
}

pub enum CxsStateType
{
    CxsStateNone = 0,
    CxsStateInitialized,
    CxsStateOfferSent,
    CxsStateRequestReceived,
    CxsStateAccepted,
    CxsStateUnfulfilled,
    CxsStateExpired,
    CxsStateRevoked,
}

#[repr(C)]
pub struct CxsStatus {
    pub handle: ::std::os::raw::c_int,
    pub status: ::std::os::raw::c_int,
    pub msg: *mut ::std::os::raw::c_char,
}
