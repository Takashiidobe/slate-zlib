//! uncompr.c -- decompress a memory buffer
//! Copyright (C) 1995-2026 Jean-loup Gailly, Mark Adler
//! For conditions of distribution and use, see copyright notice in zlib.h
#[cfg(all(target_pointer_width = "64", windows))]
macro_rules! inflateInit__2 {
    ($strm:expr) => {
        unsafe {
            inflateInit_(
                $strm,
                c"1.3.2.1-motley".as_ptr(),
                std::mem::size_of::<z_stream_s>() as i32,
            )
        }
    };
}

#[cfg(target_pointer_width = "32")]
macro_rules! inflateInit__3 {
    ($strm:expr) => {
        unsafe {
            inflateInit_(
                $strm,
                c"1.3.2.1-motley".as_ptr(),
                std::mem::size_of::<z_stream_s>() as i32,
            )
        }
    };
}

#[cfg(all(target_pointer_width = "64", unix))]
macro_rules! inflateInit {
    ($strm:expr) => {
        unsafe {
            inflateInit_(
                $strm,
                c"1.3.2.1-motley".as_ptr(),
                std::mem::size_of::<z_stream_s>() as i32,
            )
        }
    };
}

unsafe extern "C" {
    fn inflate(arg0: *mut z_stream_s, arg1: i32) -> i32;
    fn inflateEnd(arg0: *mut z_stream_s) -> i32;
    fn inflateInit_(arg0: *mut z_stream_s, arg1: *const c_char, arg2: i32) -> i32;
}

use core::ffi::c_char;
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
///   Decompresses the source buffer into the destination buffer.  *sourceLen is
/// the byte length of the source buffer. Upon entry, *destLen is the total size
/// of the destination buffer, which must be large enough to hold the entire
/// uncompressed data. (The size of the uncompressed data must have been saved
/// previously by the compressor and transmitted to the decompressor by some
/// mechanism outside the scope of this compression library.) Upon exit,
/// *destLen is the size of the decompressed data and *sourceLen is the number
/// of source bytes consumed. Upon return, source + *sourceLen points to the
/// first unused input byte.
///
///   uncompress returns Z_OK if success, Z_MEM_ERROR if there was not enough
/// memory, Z_BUF_ERROR if there was not enough room in the output buffer, or
/// Z_DATA_ERROR if the input data was corrupted, including if the input data is
/// an incomplete zlib stream.
///
///   The _z versions of the functions take size_t length arguments.
#[unsafe(no_mangle)]
extern "C-unwind" fn uncompress2_z(
    mut dest: *mut u8,
    destLen: *mut size_t,
    source: *const u8,
    sourceLen: *mut size_t,
) -> i32 {
    let mut stream: z_stream_s = unsafe { std::mem::zeroed() };
    let mut err: i32 = 0;
    let max: u32 = 4294967295u32;
    let mut len: size_t = 0;
    let mut left: size_t = 0;
    #[cfg(target_pointer_width = "64")]
    {
        if sourceLen.is_null()
            || (unsafe { *sourceLen }) > 0 && source.is_null()
            || destLen.is_null()
            || (unsafe { *destLen }) > 0 && dest.is_null()
        {
            return -2;
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        if sourceLen.is_null()
            || ((unsafe { *sourceLen }) as u32) > 0 && source.is_null()
            || destLen.is_null()
            || ((unsafe { *destLen }) as u32) > 0 && dest.is_null()
        {
            return -2;
        }
    }
    len = unsafe { *sourceLen };
    left = unsafe { *destLen };
    #[cfg(target_pointer_width = "64")]
    {
        if left == 0 && dest.is_null() {
            dest = std::ptr::addr_of_mut!(stream.reserved) as *mut u8;
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        if (left as u32) == 0 && dest.is_null() {
            dest = std::ptr::addr_of_mut!(stream.reserved) as *mut u8;
        }
    }
    // next_out cannot be NULL
    stream.next_in = source as *mut u8;
    stream.avail_in = 0;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = std::ptr::null_mut::<()>();
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        err = inflateInit!(std::ptr::addr_of_mut!(stream));
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        err = inflateInit__2!(std::ptr::addr_of_mut!(stream));
    }
    #[cfg(target_pointer_width = "32")]
    {
        err = inflateInit__3!(std::ptr::addr_of_mut!(stream));
    }
    if err != 0 {
        return err;
    }
    stream.next_out = dest;
    stream.avail_out = 0;
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
                stream.avail_in = if len > (max as size_t) {
                    max
                } else {
                    len as u32
                };
                len = len.wrapping_sub(stream.avail_in as size_t);
            }
            #[cfg(target_pointer_width = "32")]
            {
                stream.avail_in = if (len as u32) > ((max as size_t) as u32) {
                    max
                } else {
                    len as u32
                };
                len = (len as u32).wrapping_sub(stream.avail_in) as size_t;
            }
        }
        err = unsafe { inflate(std::ptr::addr_of_mut!(stream), 0i32) };
        if err != 0 {
            break;
        }
    }
    // Set len and left to the unused input data and unused output space. Set
    // *sourceLen to the amount of input consumed. Set *destLen to the amount
    // of data produced.
    #[cfg(target_pointer_width = "64")]
    {
        len = len.wrapping_add(stream.avail_in as size_t);
        left = left.wrapping_add(stream.avail_out as size_t);
    }
    #[cfg(target_pointer_width = "64")]
    let __v10: size_t = unsafe { *sourceLen };
    #[cfg(target_pointer_width = "32")]
    {
        len = (len as u32).wrapping_add(stream.avail_in) as size_t;
        left = (left as u32).wrapping_add(stream.avail_out) as size_t;
    }
    unsafe {
        #[cfg(target_pointer_width = "64")]
        {
            *sourceLen = __v10.wrapping_sub(len);
        }
        #[cfg(target_pointer_width = "32")]
        {
            *sourceLen = (*sourceLen as u32).wrapping_sub(len as u32) as size_t;
        }
    }
    #[cfg(target_pointer_width = "64")]
    let __v13: size_t = unsafe { *destLen };
    unsafe {
        #[cfg(target_pointer_width = "64")]
        {
            *destLen = __v13.wrapping_sub(left);
        }
        #[cfg(target_pointer_width = "32")]
        {
            *destLen = (*destLen as u32).wrapping_sub(left as u32) as size_t;
        }
    }
    unsafe { inflateEnd(std::ptr::addr_of_mut!(stream)) };
    #[cfg(target_pointer_width = "64")]
    {
        if err == 1 {
            0
        } else {
            if err == 2 {
                -3
            } else {
                if err == -5 && len == 0 { -3 } else { err }
            }
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        if err == 1 {
            0
        } else {
            if err == 2 {
                -3
            } else {
                if err == -5 && (len as u32) == 0 {
                    -3
                } else {
                    err
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn uncompress2(
    dest: *mut u8,
    destLen: *mut c_ulong,
    source: *const u8,
    sourceLen: *mut c_ulong,
) -> i32 {
    let mut ret: i32 = 0;
    let mut got: size_t = (unsafe { *destLen }) as size_t;
    let mut used: size_t = (unsafe { *sourceLen }) as size_t;
    ret = uncompress2_z(
        dest,
        std::ptr::addr_of_mut!(got),
        source,
        std::ptr::addr_of_mut!(used),
    );
    unsafe {
        *sourceLen = used as c_ulong;
    }
    unsafe {
        *destLen = got as c_ulong;
    }
    ret
}

#[unsafe(no_mangle)]
extern "C-unwind" fn uncompress_z(
    dest: *mut u8,
    destLen: *mut size_t,
    source: *const u8,
    mut used: size_t,
) -> i32 {
    uncompress2_z(dest, destLen, source, std::ptr::addr_of_mut!(used))
}

#[unsafe(no_mangle)]
extern "C-unwind" fn uncompress(
    dest: *mut u8,
    destLen: *mut c_ulong,
    source: *const u8,
    mut used: c_ulong,
) -> i32 {
    uncompress2(dest, destLen, source, std::ptr::addr_of_mut!(used))
}
