//! gzclose.c -- zlib gzclose() function
//! Copyright (C) 2004, 2010 Mark Adler
//! For conditions of distribution and use, see copyright notice in zlib.h
unsafe extern "C" {
    fn gzclose_r(arg0: *mut gzFile_s) -> i32;
}

#[cfg(not(feature = "no-gzcompress"))]
unsafe extern "C" {
    fn gzclose_w(arg0: *mut gzFile_s) -> i32;
}

#[cfg(not(feature = "no-gzcompress"))]
use core::ffi::c_char;
#[cfg(not(feature = "no-gzcompress"))]
use core::ffi::c_ulong;
#[cfg(unix)]
use slate_support::off_t;

#[cfg(not(feature = "no-gzcompress"))]
#[repr(C)]
#[derive(Clone, Copy)]
struct internal_state {}

#[cfg(not(feature = "no-gzcompress"))]
#[repr(C)]
#[derive(Clone, Copy)]
struct z_stream_s {
    next_in: *mut u8,
    avail_in: u32,
    total_in: c_ulong,
    next_out: *mut u8,
    avail_out: u32,
    total_out: c_ulong,
    msg: *mut c_char,
    state: *mut internal_state,
    zalloc: Option<unsafe extern "C-unwind" fn(*mut (), u32, u32) -> *mut ()>,
    zfree: Option<unsafe extern "C-unwind" fn(*mut (), *mut ())>,
    opaque: *mut (),
    data_type: i32,
    adler: c_ulong,
    reserved: c_ulong,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct gzFile_s {
    have: u32,
    next: *mut u8,
    #[cfg(unix)]
    pos: off_t,
    #[cfg(windows)]
    pos: i64,
}

#[cfg(not(feature = "no-gzcompress"))]
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_gzguts_h_203 {
    x: gzFile_s,
    mode: i32,
    fd: i32,
    path: *mut c_char,
    size: u32,
    want: u32,
    r#in: *mut u8,
    out: *mut u8,
    direct: i32,
    junk: i32,
    how: i32,
    again: i32,
    #[cfg(unix)]
    start: off_t,
    #[cfg(windows)]
    start: i64,
    eof: i32,
    past: i32,
    level: i32,
    strategy: i32,
    reset: i32,
    #[cfg(unix)]
    skip: off_t,
    #[cfg(windows)]
    skip: i64,
    err: i32,
    msg: *mut c_char,
    strm: z_stream_s,
}

#[cfg(not(feature = "no-gzcompress"))]
/// gzclose() is in a separate file so that it is linked in only if it is used.
/// That way the other gzclose functions can be used instead to avoid linking in
/// unneeded compression or decompression routines.
#[unsafe(no_mangle)]
extern "C-unwind" fn gzclose(file: *mut gzFile_s) -> i32 {
    if file.is_null() {
        return -2;
    }
    let __v0: i32 = if (unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).mode }) == 7247 {
        unsafe { gzclose_r(file) }
    } else {
        unsafe { gzclose_w(file) }
    };
    __v0
}

#[cfg(feature = "no-gzcompress")]
/// gzclose() is in a separate file so that it is linked in only if it is used.
/// That way the other gzclose functions can be used instead to avoid linking in
/// unneeded compression or decompression routines.
#[unsafe(no_mangle)]
extern "C-unwind" fn gzclose(file: *mut gzFile_s) -> i32 {
    unsafe { gzclose_r(file) }
}
