//! zutil.c -- target dependent utility functions for the compression library
//! Copyright (C) 1995-2026 Jean-loup Gailly
//! For conditions of distribution and use, see copyright notice in zlib.h
macro_rules! ERR_MSG {
    ($err:expr) => {
        unsafe {
            z_errmsg.0[(if ($err) < -6 || ($err) > 2 {
                9i32
            } else {
                2 - $err
            }) as usize]
        }
    };
}

#[cfg(any(
    target_os = "macos",
    target_os = "android",
    all(target_arch = "arm", windows)
))]
unsafe extern "C" {
    fn malloc(arg0: size_t) -> *mut ();
    fn calloc(arg0: size_t, arg1: size_t) -> *mut ();
    fn free(arg0: *mut ());
}

#[cfg(all(
    any(target_os = "macos", all(target_arch = "arm", windows)),
    feature = "zlib-debug"
))]
unsafe extern "C" {
    fn exit(arg0: i32);
}

use core::ffi::c_char;
#[cfg(any(
    feature = "no-gzip",
    feature = "no-gzcompress",
    feature = "fastest",
    feature = "pkzip-bug",
    feature = "no-vsnprintf"
))]
use core::ffi::c_long;
use core::ffi::c_ulong;
#[cfg(unix)]
use slate_support::off_t;
use slate_support::size_t;

#[cfg(target_arch = "x86_64")]
#[repr(C, align(16))]
struct __SlateAlignLargeArray<T>(T);

#[cfg(not(target_arch = "x86_64"))]
#[repr(C)]
struct __SlateAlignLargeArray<T>(T);

#[cfg(feature = "zlib-debug")]
unsafe extern "C" {
    fn fflush(_0: *mut slate_support::FILE) -> i32;
}

#[unsafe(no_mangle)]
static mut z_errmsg: __SlateAlignLargeArray<[*mut c_char; 10]> = __SlateAlignLargeArray([
    b"need dictionary\0".as_ptr() as *mut c_char,
    b"stream end\0".as_ptr() as *mut c_char,
    b"\0".as_ptr() as *mut c_char,
    b"file error\0".as_ptr() as *mut c_char,
    b"stream error\0".as_ptr() as *mut c_char,
    b"data error\0".as_ptr() as *mut c_char,
    b"insufficient memory\0".as_ptr() as *mut c_char,
    b"buffer error\0".as_ptr() as *mut c_char,
    b"incompatible version\0".as_ptr() as *mut c_char,
    b"\0".as_ptr() as *mut c_char,
]);

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    all(not(target_arch = "arm"), windows)
))]
use slate_support::calloc;
#[cfg(all(
    not(any(target_os = "macos", all(target_arch = "arm", windows))),
    feature = "zlib-debug"
))]
use slate_support::exit;
#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    all(not(target_arch = "arm"), windows)
))]
use slate_support::free;
#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    all(not(target_arch = "arm"), windows)
))]
use slate_support::malloc;

// @(#) $Id$
// Z_NEED_DICT       2
// Z_STREAM_END      1
// Z_OK              0
// Z_ERRNO         (-1)
// Z_STREAM_ERROR  (-2)
// Z_DATA_ERROR    (-3)
// Z_MEM_ERROR     (-4)
// Z_BUF_ERROR     (-5)
// Z_VERSION_ERROR (-6)
#[unsafe(no_mangle)]
extern "C-unwind" fn zlibVersion() -> *const c_char {
    c"1.3.2.1-motley".as_ptr()
}

#[unsafe(no_mangle)]
extern "C-unwind" fn zlibCompileFlags() -> c_ulong {
    let mut flags: c_ulong = 0;
    match std::mem::size_of::<u32>() as i32 {
        2 => {}
        4 => {
            flags = flags.wrapping_add(1);
        }
        8 => {
            flags = flags.wrapping_add(2);
        }
        _ => {
            flags = flags.wrapping_add(3);
        }
    }
    match std::mem::size_of::<c_ulong>() as i32 {
        2 => {}
        4 => {
            flags = flags.wrapping_add((4 as c_ulong));
        }
        8 => {
            flags = flags.wrapping_add((8 as c_ulong));
        }
        _ => {
            flags = flags.wrapping_add((12 as c_ulong));
        }
    }
    match std::mem::size_of::<*mut ()>() as i32 {
        2 => {}
        4 => {
            flags = flags.wrapping_add((16 as c_ulong));
        }
        8 => {
            flags = flags.wrapping_add((32 as c_ulong));
        }
        _ => {
            flags = flags.wrapping_add((48 as c_ulong));
        }
    }
    #[cfg(unix)]
    {
        match std::mem::size_of::<off_t>() as i32 {
            2 => {}
            4 => {
                flags = flags.wrapping_add((64 as c_ulong));
            }
            8 => {
                flags = flags.wrapping_add((128 as c_ulong));
            }
            _ => {
                flags = flags.wrapping_add((192 as c_ulong));
            }
        }
    }
    #[cfg(windows)]
    {
        match std::mem::size_of::<i64>() as i32 {
            2 => {}
            4 => {
                flags = flags.wrapping_add((64 as c_ulong));
            }
            8 => {
                flags = flags.wrapping_add((128 as c_ulong));
            }
            _ => {
                flags = flags.wrapping_add((192 as c_ulong));
            }
        }
    }
    #[cfg(feature = "zlib-debug")]
    {
        flags = flags.wrapping_add((256 as c_ulong));
    }
    // #if defined(ASMV) || defined(ASMINF)
    //     flags += 1 << 9;
    // #endif
    #[cfg(feature = "no-gzcompress")]
    {
        flags = flags.wrapping_add((65536 as c_ulong));
    }
    #[cfg(feature = "no-gzip")]
    {
        flags = flags.wrapping_add((131072 as c_ulong));
    }
    #[cfg(feature = "pkzip-bug")]
    {
        flags = flags.wrapping_add((1048576 as c_ulong));
    }
    #[cfg(feature = "fastest")]
    {
        flags = flags.wrapping_add((2097152 as c_ulong));
    }
    #[cfg(feature = "no-vsnprintf")]
    {
        flags = flags.wrapping_add((134217728 as c_ulong));
    }
    flags
}

#[cfg(feature = "zlib-debug")]
#[unsafe(no_mangle)]
static mut z_verbose: i32 = 0;

#[cfg(feature = "zlib-debug")]
#[unsafe(no_mangle)]
extern "C-unwind" fn z_error(m: *mut c_char) {
    let _ = std::io::Write::flush(&mut std::io::stdout());
    unsafe { slate_support::fprintf(slate_support::stderr(), c"%s\n".as_ptr(), m) };
    unsafe { fflush(std::ptr::null_mut()) };
    #[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
    {
        unsafe { std::process::exit(1i32 as i32) };
    }
    #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
    {
        unsafe { std::process::exit(1i32) };
    }
}

/// exported to allow conversion of error code to string for compress() and
/// uncompress()
#[unsafe(no_mangle)]
extern "C-unwind" fn zError(err: i32) -> *const c_char {
    ERR_MSG!(err) as *const c_char
}

#[unsafe(no_mangle)]
#[cfg_attr(
    all(not(target_os = "macos"), unix),
    unsafe(link_section = ".text.slate_distinct.zutil.zcalloc")
)]
extern "C-unwind" fn zcalloc(opaque: *mut (), items: u32, size: u32) -> *mut () {
    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        all(not(target_arch = "arm"), windows)
    ))]
    let __v0: *mut () = if (std::mem::size_of::<u32>() as size_t) > 2 {
        unsafe { malloc((items.wrapping_mul(size) as size_t) as usize) as *mut () }
    } else {
        unsafe { calloc((items as size_t) as usize, (size as size_t) as usize) as *mut () }
    };
    #[cfg(any(
        target_os = "macos",
        target_os = "android",
        all(target_arch = "arm", windows)
    ))]
    let __v0: *mut () = if (std::mem::size_of::<u32>() as size_t) > 2 {
        unsafe { malloc(items.wrapping_mul(size) as size_t) }
    } else {
        unsafe { calloc(items as size_t, size as size_t) }
    };
    __v0
}

#[unsafe(no_mangle)]
#[cfg_attr(
    all(not(target_os = "macos"), unix),
    unsafe(link_section = ".text.slate_distinct.zutil.zcfree")
)]
extern "C-unwind" fn zcfree(opaque: *mut (), ptr: *mut ()) {
    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        all(not(target_arch = "arm"), windows)
    ))]
    {
        unsafe { free(ptr as *mut core::ffi::c_void) };
    }
    #[cfg(any(
        target_os = "macos",
        target_os = "android",
        all(target_arch = "arm", windows)
    ))]
    {
        unsafe { free(ptr) };
    }
}
