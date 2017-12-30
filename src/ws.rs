use libc::c_int;
use nanomsg_sys;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MessageType {
    Text = (nanomsg_sys::NN_WS_MSG_TYPE_TEXT) as isize,
    Binary = (nanomsg_sys::NN_WS_MSG_TYPE_BINARY) as isize,
}

impl MessageType {
    pub fn to_raw(&self) -> c_int {
        *self as c_int
    }
}
