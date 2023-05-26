#![allow(non_camel_case_types)]

use libc::{c_int, c_char, c_void};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct xcb_extension_t {
    pub name: *const c_char,
    pub global_id: c_int,
}

#[repr(C)]
pub struct xcb_protocol_request_t {
    pub count: usize,
    pub ext: *mut xcb_extension_t,
    pub opcode: u8,
    pub isvoid: u8,
}

pub type xcb_connection_t = c_void;
