use std::{
    io::{self, Read, Write},
    ptr, slice,
};

use libc::{c_char, c_int, c_void};

use crate::ffi;

#[derive(Debug)]
pub(crate) struct Writer<W: Write> {
    pub(crate) writer: W,
    pub(crate) error: Option<io::Error>,
}

impl<W: Write> Writer<W> {
    pub(crate) fn new(writer: W) -> Self {
        Self {
            writer,
            error: None,
        }
    }

    pub(crate) unsafe extern "C" fn lua_writer(
        _state: *mut ffi::lua_State,
        p: *const c_void,
        sz: usize,
        ud: *mut c_void,
    ) -> c_int {
        let this = ud as *mut Self;
        match (*this)
            .writer
            .write_all(slice::from_raw_parts(p as *const u8, sz))
        {
            Ok(()) => 0,
            Err(e) => {
                (*this).error = Some(e);
                1
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Reader<R: Read> {
    pub(crate) reader: R,
    pub(crate) buffer: Vec<u8>,
    pub(crate) error: Option<io::Error>,
}

impl<R: Read> Reader<R> {
    const CHUNK_SIZE: usize = 8192;

    pub(crate) fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![0u8; Self::CHUNK_SIZE],
            error: None,
        }
    }

    pub(crate) unsafe extern "C" fn lua_reader(
        _state: *mut ffi::lua_State,
        ud: *mut c_void,
        size: *mut usize,
    ) -> *const c_char {
        let this = ud as *mut Self;
        match (*this).reader.read(&mut (*this).buffer) {
            Ok(n) => {
                *size = n;
                (*this).buffer.as_ptr() as *const c_char
            }
            Err(e) => {
                *size = 0;
                (*this).error = Some(e);
                ptr::null()
            }
        }
    }
}
