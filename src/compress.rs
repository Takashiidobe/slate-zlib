//! compress.c -- compress a memory buffer
//! Copyright (C) 1995-2026 Jean-loup Gailly, Mark Adler
//! For conditions of distribution and use, see copyright notice in zlib.h
#[cfg(all(target_pointer_width = "64", windows))]
macro_rules! deflateInit__3 {
    ($strm:expr, $level:expr) => {
        unsafe {
            deflateInit_(
                $strm,
                $level,
                c"1.3.2.1-motley".as_ptr(),
                std::mem::size_of::<z_stream_s>() as i32,
            )
        }
    };
}

#[cfg(target_pointer_width = "32")]
macro_rules! deflateInit {
    ($strm:expr, $level:expr) => {
        unsafe {
            deflateInit_(
                $strm,
                $level,
                c"1.3.2.1-motley".as_ptr(),
                std::mem::size_of::<z_stream_s>() as i32,
            )
        }
    };
}

#[cfg(all(target_pointer_width = "64", unix))]
macro_rules! deflateInit__2 {
    ($strm:expr, $level:expr) => {
        unsafe {
            deflateInit_(
                $strm,
                $level,
                c"1.3.2.1-motley".as_ptr(),
                std::mem::size_of::<z_stream_s>() as i32,
            )
        }
    };
}

unsafe extern "C" {
    fn deflate(arg0: *mut z_stream_s, arg1: i32) -> i32;
    fn deflateEnd(arg0: *mut z_stream_s) -> i32;
    fn deflateInit_(arg0: *mut z_stream_s, arg1: i32, arg2: *const c_char, arg3: i32) -> i32;
}

use core::ffi::c_char;
#[cfg(all(target_pointer_width = "64", unix))]
use core::ffi::c_long;
use core::ffi::c_ulong;
use slate_support::size_t;

#[repr(C)]
#[derive(Clone, Copy)]
struct internal_state {}

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

// @(#) $Id$
///   Compresses the source buffer into the destination buffer. The level
/// parameter has the same meaning as in deflateInit.  sourceLen is the byte
/// length of the source buffer. Upon entry, destLen is the total size of the
/// destination buffer, which must be at least 0.1% larger than sourceLen plus
/// 12 bytes. Upon exit, destLen is the actual size of the compressed buffer.
///
///   compress2 returns Z_OK if success, Z_MEM_ERROR if there was not enough
/// memory, Z_BUF_ERROR if there was not enough room in the output buffer,
/// Z_STREAM_ERROR if the level parameter is invalid.
///
///   The _z versions of the functions take size_t length arguments.
#[unsafe(no_mangle)]
extern "C-unwind" fn compress2_z(
    dest: *mut u8,
    destLen: *mut size_t,
    source: *const u8,
    mut sourceLen: size_t,
    level: i32,
) -> i32 {
    let mut stream: z_stream_s = unsafe { std::mem::zeroed() };
    let mut err: i32 = 0;
    let max: u32 = 4294967295u32;
    let mut left: size_t = 0;
    #[cfg(target_pointer_width = "64")]
    {
        if sourceLen > 0 && source.is_null()
            || destLen.is_null()
            || (unsafe { *destLen }) > 0 && dest.is_null()
        {
            return -2;
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        if (sourceLen as u32) > 0 && source.is_null()
            || destLen.is_null()
            || ((unsafe { *destLen }) as u32) > 0 && dest.is_null()
        {
            return -2;
        }
    }
    left = unsafe { *destLen };
    unsafe {
        *destLen = 0;
    }
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = std::ptr::null_mut::<()>();
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        err = deflateInit__2!(std::ptr::addr_of_mut!(stream), level);
    }
    #[cfg(target_pointer_width = "32")]
    {
        err = deflateInit!(std::ptr::addr_of_mut!(stream), level);
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        err = deflateInit__3!(std::ptr::addr_of_mut!(stream), level);
    }
    if err != 0 {
        return err;
    }
    stream.next_out = dest;
    stream.avail_out = 0;
    stream.next_in = source as *mut u8;
    stream.avail_in = 0;
    loop {
        if stream.avail_out == 0 {
            #[cfg(target_pointer_width = "64")]
            {
                stream.avail_out = if left > (max as size_t) {
                    max
                } else {
                    left as u32
                };
                left = left.wrapping_sub(stream.avail_out as size_t);
            }
            #[cfg(target_pointer_width = "32")]
            {
                stream.avail_out = if (left as u32) > ((max as size_t) as u32) {
                    max
                } else {
                    left as u32
                };
                left = (left as u32).wrapping_sub(stream.avail_out) as size_t;
            }
        }
        if stream.avail_in == 0 {
            #[cfg(target_pointer_width = "64")]
            {
                stream.avail_in = if sourceLen > (max as size_t) {
                    max
                } else {
                    sourceLen as u32
                };
                sourceLen = sourceLen.wrapping_sub(stream.avail_in as size_t);
            }
            #[cfg(target_pointer_width = "32")]
            {
                stream.avail_in = if (sourceLen as u32) > ((max as size_t) as u32) {
                    max
                } else {
                    sourceLen as u32
                };
                sourceLen = (sourceLen as u32).wrapping_sub(stream.avail_in) as size_t;
            }
        }
        err = unsafe {
            deflate(
                std::ptr::addr_of_mut!(stream),
                if sourceLen != 0 { 0i32 } else { 4i32 },
            )
        };
        if err != 0 {
            break;
        }
    }
    unsafe {
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            *destLen = (stream.next_out.offset_from(dest as *mut u8) as c_long) as size_t;
        }
        #[cfg(target_pointer_width = "32")]
        {
            *destLen = (stream.next_out.offset_from(dest as *mut u8) as i32) as size_t;
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            *destLen = (stream.next_out.offset_from(dest as *mut u8) as i64) as size_t;
        }
    }
    unsafe { deflateEnd(std::ptr::addr_of_mut!(stream)) };
    if err == 1 { 0 } else { err }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn compress2(
    dest: *mut u8,
    destLen: *mut c_ulong,
    source: *const u8,
    sourceLen: c_ulong,
    level: i32,
) -> i32 {
    let mut ret: i32 = 0;
    let mut got: size_t = (unsafe { *destLen }) as size_t;
    ret = compress2_z(
        dest,
        std::ptr::addr_of_mut!(got),
        source,
        sourceLen as size_t,
        level,
    );
    unsafe {
        *destLen = got as c_ulong;
    }
    ret
}

#[unsafe(no_mangle)]
extern "C-unwind" fn compress_z(
    dest: *mut u8,
    destLen: *mut size_t,
    source: *const u8,
    sourceLen: size_t,
) -> i32 {
    compress2_z(dest, destLen, source, sourceLen, -1i32)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn compress(
    dest: *mut u8,
    destLen: *mut c_ulong,
    source: *const u8,
    sourceLen: c_ulong,
) -> i32 {
    compress2(dest, destLen, source, sourceLen, -1i32)
}

#[cfg(target_pointer_width = "64")]
///   If the default memLevel or windowBits for deflateInit() is changed, then
/// this function needs to be updated.
#[unsafe(no_mangle)]
extern "C-unwind" fn compressBound_z(sourceLen: size_t) -> size_t {
    let bound: size_t = sourceLen
        .wrapping_add(sourceLen >> 12i32)
        .wrapping_add(sourceLen >> 14i32)
        .wrapping_add(sourceLen >> 25i32)
        .wrapping_add(13);
    if bound < sourceLen {
        (18446744073709551615 as size_t)
    } else {
        bound
    }
}

#[cfg(target_pointer_width = "32")]
///   If the default memLevel or windowBits for deflateInit() is changed, then
/// this function needs to be updated.
#[unsafe(no_mangle)]
extern "C-unwind" fn compressBound_z(sourceLen: size_t) -> size_t {
    let bound: size_t = (sourceLen as u32)
        .wrapping_add((sourceLen as u32) >> 12i32)
        .wrapping_add((sourceLen as u32) >> 14i32)
        .wrapping_add((sourceLen as u32) >> 25i32)
        .wrapping_add(13) as size_t;
    (if (bound as u32) < (sourceLen as u32) {
        4294967295u32
    } else {
        bound as u32
    }) as size_t
}

#[unsafe(no_mangle)]
extern "C-unwind" fn compressBound(sourceLen: c_ulong) -> c_ulong {
    let bound: size_t = compressBound_z(sourceLen as size_t);
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        if (bound as c_ulong) != (bound as c_ulong) {
            (18446744073709551615 as c_ulong)
        } else {
            bound as c_ulong
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        if (bound as c_ulong) != ((bound as u32) as c_ulong) {
            (4294967295 as c_ulong)
        } else {
            bound as c_ulong
        }
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        if ((bound as c_ulong) as size_t) != bound {
            (4294967295 as c_ulong)
        } else {
            bound as c_ulong
        }
    }
}
