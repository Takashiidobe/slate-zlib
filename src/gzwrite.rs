//! gzwrite.c -- zlib functions for writing gzip files
//! Copyright (C) 2004-2026 Mark Adler
//! For conditions of distribution and use, see copyright notice in zlib.h
macro_rules! deflateInit2 {
    ($strm:expr, $level:expr, $method:expr, $windowBits:expr, $memLevel:expr, $strategy:expr) => {
        unsafe {
            deflateInit2_(
                $strm,
                $level,
                $method,
                $windowBits,
                $memLevel,
                $strategy,
                c"1.3.2.1-motley".as_ptr(),
                std::mem::size_of::<z_stream_s>() as i32,
            )
        }
    };
}

#[cfg(all(windows, not(feature = "no-vsnprintf")))]
unsafe extern "C" {
    fn __stdio_common_vsprintf(
        arg0: u64,
        arg1: *mut c_char,
        arg2: size_t,
        arg3: *const c_char,
        arg4: *mut __crt_locale_pointers,
        arg5: *mut c_char,
    ) -> i32;
}

#[cfg(all(
    not(any(
        target_arch = "x86",
        all(target_arch = "aarch64", target_os = "macos"),
        windows
    )),
    not(feature = "no-vsnprintf")
))]
unsafe extern "C" {
    fn vsnprintf(
        arg0: *mut c_char,
        arg1: size_t,
        arg2: *const c_char,
        arg3: core::ffi::VaList<'static>,
    ) -> i32;
}

#[cfg(all(
    any(
        all(target_arch = "x86", unix),
        all(target_arch = "aarch64", target_os = "macos")
    ),
    not(feature = "no-vsnprintf")
))]
unsafe extern "C" {
    fn vsnprintf(arg0: *mut c_char, arg1: size_t, arg2: *const c_char, arg3: *mut c_char) -> i32;
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    #[link_name = "_close"]
    fn close(arg0: i32) -> i32;
    #[link_name = "_write"]
    fn write(arg0: i32, arg1: *const (), arg2: size_t) -> ssize_t;
    #[link_name = "_strerror"]
    fn strerror(arg0: i32) -> *mut c_char;
}

#[cfg(windows)]
unsafe extern "C" {
    fn close(arg0: i32) -> i32;
    fn write(arg0: i32, arg1: *const (), arg2: u32) -> i32;
}

unsafe extern "C" {
    fn deflate(arg0: *mut z_stream_s, arg1: i32) -> i32;
    fn deflateEnd(arg0: *mut z_stream_s) -> i32;
    fn deflateReset(arg0: *mut z_stream_s) -> i32;
    fn deflateParams(arg0: *mut z_stream_s, arg1: i32, arg2: i32) -> i32;
    fn deflateInit2_(
        arg0: *mut z_stream_s,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: *const c_char,
        arg7: i32,
    ) -> i32;
    fn gz_error(arg0: *mut __SlateRecord_gzguts_h_203, arg1: i32, arg2: *const c_char);
    fn gz_intmax() -> u32;
}

#[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
unsafe extern "C" {
    fn memcpy(arg0: *mut (), arg1: *const (), arg2: size_t) -> *mut ();
    fn memset(arg0: *mut (), arg1: i32, arg2: size_t) -> *mut ();
    fn strlen(arg0: *const c_char) -> size_t;
}

#[cfg(all(
    any(target_os = "macos", all(target_arch = "arm", windows)),
    not(feature = "no-vsnprintf")
))]
unsafe extern "C" {
    fn memmove(arg0: *mut (), arg1: *const (), arg2: size_t) -> *mut ();
}

#[cfg(all(target_arch = "arm", windows))]
unsafe extern "C" {
    fn strerror(arg0: i32) -> *mut c_char;
}

#[cfg(any(
    target_os = "macos",
    target_os = "android",
    all(target_arch = "arm", windows)
))]
unsafe extern "C" {
    fn malloc(arg0: size_t) -> *mut ();
    fn free(arg0: *mut ());
}

use core::ffi::c_char;
#[cfg(all(not(target_os = "macos"), target_pointer_width = "64", unix))]
use core::ffi::c_long;
use core::ffi::c_ulong;
#[cfg(unix)]
use slate_support::off_t;
use slate_support::ptrdiff_t;
use slate_support::size_t;

#[cfg(all(windows, not(feature = "no-vsnprintf")))]
#[repr(C)]
#[derive(Clone, Copy)]
struct __crt_locale_pointers {
    locinfo: *mut __crt_locale_data,
    mbcinfo: *mut __crt_multibyte_data,
}

#[cfg(all(windows, not(feature = "no-vsnprintf")))]
#[repr(C)]
#[derive(Clone, Copy)]
struct __crt_locale_data {}

#[cfg(all(windows, not(feature = "no-vsnprintf")))]
#[repr(C)]
#[derive(Clone, Copy)]
struct __crt_multibyte_data {}

#[cfg(any(all(target_pointer_width = "32", unix), target_os = "macos"))]
use slate_support::ssize_t;

#[repr(C)]
#[derive(Clone, Copy)]
struct internal_state {}

#[cfg(all(windows, not(feature = "no-vsnprintf")))]
fn __local_stdio_printf_options() -> *mut u64 {
    unsafe { std::ptr::addr_of_mut!(_OptionsStorage) }
}

#[cfg(all(windows, not(feature = "no-vsnprintf")))]
static mut _OptionsStorage: u64 = unsafe { std::mem::zeroed() };

#[cfg(all(windows, not(feature = "no-vsnprintf")))]
fn vsnprintf(
    _Buffer: *mut c_char,
    _BufferCount: size_t,
    _Format: *const c_char,
    _ArgList: *mut c_char,
) -> i32 {
    let _Result: i32 = unsafe {
        __stdio_common_vsprintf(
            *__local_stdio_printf_options() | 2u64,
            _Buffer,
            _BufferCount,
            _Format,
            std::ptr::null_mut::<__crt_locale_pointers>(),
            _ArgList,
        )
    };
    if _Result < 0 { -1 } else { _Result }
}

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

#[cfg(all(not(target_os = "macos"), unix))]
use slate_support::close;
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
#[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
use slate_support::memcpy;
#[cfg(all(
    not(any(target_os = "macos", all(target_arch = "arm", windows))),
    not(feature = "no-vsnprintf")
))]
use slate_support::memmove;
#[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
use slate_support::memset;
#[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
use slate_support::strerror;
#[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
use slate_support::strlen;
#[cfg(all(not(target_os = "macos"), unix))]
use slate_support::write;

/// Initialize state for writing a gzip file.  Mark initialization by setting
/// state->size to non-zero.  Return -1 on a memory allocation failure, or 0 on
/// success.
fn gz_init(state: *mut __SlateRecord_gzguts_h_203) -> i32 {
    let mut ret: i32 = 0;
    let strm: *mut z_stream_s = unsafe { std::ptr::addr_of_mut!((*state).strm) };
    // allocate input buffer (double size for gzprintf)
    unsafe {
        #[cfg(any(
            target_os = "linux",
            target_os = "freebsd",
            all(not(target_arch = "arm"), windows)
        ))]
        {
            (*state).r#in =
                (malloc((((*state).want << 1i32) as size_t) as usize) as *mut ()) as *mut u8;
        }
        #[cfg(any(
            target_os = "macos",
            target_os = "android",
            all(target_arch = "arm", windows)
        ))]
        {
            (*state).r#in = malloc(((*state).want << 1i32) as size_t) as *mut u8;
        }
    }
    if unsafe { (*state).r#in }.is_null() {
        unsafe { gz_error(state, -4i32, c"out of memory".as_ptr()) };
        return -1;
    }
    // only need output buffer and deflate state if compressing
    if (unsafe { (*state).direct }) == 0 {
        // allocate output buffer
        unsafe {
            #[cfg(any(
                target_os = "linux",
                target_os = "freebsd",
                all(not(target_arch = "arm"), windows)
            ))]
            {
                (*state).out = (malloc(((*state).want as size_t) as usize) as *mut ()) as *mut u8;
            }
            #[cfg(any(
                target_os = "macos",
                target_os = "android",
                all(target_arch = "arm", windows)
            ))]
            {
                (*state).out = malloc((*state).want as size_t) as *mut u8;
            }
        }
        if unsafe { (*state).out }.is_null() {
            #[cfg(any(
                target_os = "linux",
                target_os = "freebsd",
                all(not(target_arch = "arm"), windows)
            ))]
            {
                unsafe { free(((*state).r#in as *mut ()) as *mut core::ffi::c_void) };
            }
            #[cfg(any(
                target_os = "macos",
                target_os = "android",
                all(target_arch = "arm", windows)
            ))]
            {
                unsafe { free((*state).r#in as *mut ()) };
            }
            unsafe { gz_error(state, -4i32, c"out of memory".as_ptr()) };
            return -1;
        }
        // allocate deflate memory, set up for gzip compression
        unsafe {
            (*strm).zalloc = None;
        }
        unsafe {
            (*strm).zfree = None;
        }
        unsafe {
            (*strm).opaque = std::ptr::null_mut::<()>();
        }
        ret = deflateInit2!(strm, unsafe { (*state).level }, 8i32, 31i32, 8i32, unsafe {
            (*state).strategy
        });
        if ret != 0 {
            #[cfg(any(
                target_os = "linux",
                target_os = "freebsd",
                all(not(target_arch = "arm"), windows)
            ))]
            {
                unsafe { free(((*state).out as *mut ()) as *mut core::ffi::c_void) };
                unsafe { free(((*state).r#in as *mut ()) as *mut core::ffi::c_void) };
            }
            #[cfg(any(
                target_os = "macos",
                target_os = "android",
                all(target_arch = "arm", windows)
            ))]
            {
                unsafe { free((*state).out as *mut ()) };
                unsafe { free((*state).r#in as *mut ()) };
            }
            unsafe { gz_error(state, -4i32, c"out of memory".as_ptr()) };
            return -1;
        }
        unsafe {
            (*strm).next_in = std::ptr::null_mut::<u8>();
        }
    }
    // mark state as initialized
    unsafe {
        (*state).size = (*state).want;
    }
    // initialize write buffer if compressing
    if (unsafe { (*state).direct }) == 0 {
        unsafe {
            (*strm).avail_out = (*state).size;
        }
        unsafe {
            (*strm).next_out = (*state).out;
        }
        unsafe {
            (*state).x.next = (*strm).next_out;
        }
    }
    0
}

/// Compress whatever is at avail_in and next_in and write to the output file.
/// Return -1 if there is an error writing to the output file or if gz_init()
/// fails to allocate memory, otherwise 0.  flush is assumed to be a valid
/// deflate() flush value.  If flush is Z_FINISH, then the deflate() state is
/// reset to start a new gzip stream.  If gz->direct is true, then simply write
/// to the output file without compressing, and ignore flush.
fn gz_comp(state: *mut __SlateRecord_gzguts_h_203, flush: i32) -> i32 {
    // run deflate() on provided input until it produces no more output
    let mut ret: i32 = 0;
    let mut writ: i32 = 0;
    let mut have: u32 = 0;
    let mut put: u32 = 0;
    let max: u32 = 1073741824u32;
    let strm: *mut z_stream_s = unsafe { std::ptr::addr_of_mut!((*state).strm) };
    // allocate memory if this is the first time through
    let __v0: bool = (unsafe { (*state).size }) == 0 && gz_init(state) == -1;
    if __v0 {
        return -1;
    }
    // write directly if requested
    if (unsafe { (*state).direct }) != 0 {
        while (unsafe { (*strm).avail_in }) != 0 {
            unsafe {
                *slate_support::errno_location() = 0;
            }
            unsafe {
                (*state).again = 0;
            }
            put = if (unsafe { (*strm).avail_in }) > max {
                max
            } else {
                unsafe { (*strm).avail_in }
            };
            #[cfg(target_os = "macos")]
            {
                writ = (unsafe { write((*state).fd, (*strm).next_in as *const (), put as size_t) })
                    as i32;
            }
            #[cfg(all(not(target_os = "macos"), target_pointer_width = "64", unix))]
            {
                writ = (unsafe {
                    write(
                        (*state).fd as i32,
                        ((*strm).next_in as *const ()) as *const core::ffi::c_void,
                        (put as size_t) as usize,
                    ) as c_long
                }) as i32;
            }
            #[cfg(windows)]
            {
                writ = unsafe { write((*state).fd, (*strm).next_in as *const (), put) };
            }
            #[cfg(all(target_pointer_width = "32", unix))]
            {
                writ = (unsafe {
                    write(
                        (*state).fd as i32,
                        ((*strm).next_in as *const ()) as *const core::ffi::c_void,
                        (put as size_t) as usize,
                    ) as i32
                }) as i32;
            }
            if writ < 0 {
                #[cfg(any(target_os = "macos", target_os = "freebsd"))]
                let __v1: bool = (unsafe { *slate_support::errno_location() })
                    == (slate_support::EAGAIN as i32)
                    || (unsafe { *slate_support::errno_location() }) == 35;
                #[cfg(not(any(unix, target_arch = "arm")))]
                let __v1: bool = (unsafe { *slate_support::errno_location() })
                    == (slate_support::EAGAIN as i32)
                    || (unsafe { *slate_support::errno_location() })
                        == (slate_support::EWOULDBLOCK as i32);
                #[cfg(all(target_arch = "arm", windows))]
                let __v1: bool = (unsafe { *slate_support::errno_location() }) == 11
                    || (unsafe { *slate_support::errno_location() }) == 140;
                #[cfg(not(any(target_os = "linux", target_os = "android")))]
                {
                    if __v1 {
                        unsafe {
                            (*state).again = 1;
                        }
                    }
                }
                #[cfg(any(target_os = "linux", target_os = "android"))]
                {
                    if (unsafe { *slate_support::errno_location() })
                        == (slate_support::EAGAIN as i32)
                        || (unsafe { *slate_support::errno_location() }) == 11
                    {
                        unsafe {
                            (*state).again = 1;
                        }
                    }
                }
                unsafe { gz_error(state, -1i32, zstrerror!() as *const c_char) };
                return -1;
            }
            let __v3: u32 = unsafe { (*strm).avail_in };
            unsafe {
                (*strm).avail_in = __v3.wrapping_sub(writ as u32);
            }
            let __v6: *mut u8 = unsafe { (*strm).next_in };
            unsafe {
                (*strm).next_in = __v6.offset(writ as isize);
            }
        }
        return 0;
    }
    // check for a pending reset
    if (unsafe { (*state).reset }) != 0 {
        // don't start a new gzip member unless there is data to write and
        // we're not flushing
        if (unsafe { (*strm).avail_in }) == 0 && flush == 0 {
            return 0;
        }
        unsafe { deflateReset(strm) };
        unsafe {
            (*state).reset = 0;
        }
    }
    loop {
        // write out current buffer contents if full, or if flushing, but if
        // doing Z_FINISH then don't write until we get to Z_STREAM_END
        if (unsafe { (*strm).avail_out }) == 0 || flush != 0 && (flush != 4 || ret == 1) {
            while (unsafe { (*strm).next_out }) > unsafe { (*state).x.next } {
                unsafe {
                    *slate_support::errno_location() = 0;
                }
                unsafe {
                    (*state).again = 0;
                }
                put = if ((unsafe {
                    unsafe { (*strm).next_out }.offset_from((*state).x.next as *mut u8)
                }) as ptrdiff_t)
                    > ((max as i32) as ptrdiff_t)
                {
                    max
                } else {
                    ((unsafe {
                        unsafe { (*strm).next_out }.offset_from((*state).x.next as *mut u8)
                    }) as ptrdiff_t) as u32
                };
                #[cfg(target_os = "macos")]
                {
                    writ = (unsafe {
                        write((*state).fd, (*state).x.next as *const (), put as size_t)
                    }) as i32;
                }
                #[cfg(all(not(target_os = "macos"), target_pointer_width = "64", unix))]
                {
                    writ = (unsafe {
                        write(
                            (*state).fd as i32,
                            ((*state).x.next as *const ()) as *const core::ffi::c_void,
                            (put as size_t) as usize,
                        ) as c_long
                    }) as i32;
                }
                #[cfg(windows)]
                {
                    writ = unsafe { write((*state).fd, (*state).x.next as *const (), put) };
                }
                #[cfg(all(target_pointer_width = "32", unix))]
                {
                    writ = (unsafe {
                        write(
                            (*state).fd as i32,
                            ((*state).x.next as *const ()) as *const core::ffi::c_void,
                            (put as size_t) as usize,
                        ) as i32
                    }) as i32;
                }
                if writ < 0 {
                    #[cfg(any(target_os = "macos", target_os = "freebsd"))]
                    let __v9: bool = (unsafe { *slate_support::errno_location() })
                        == (slate_support::EAGAIN as i32)
                        || (unsafe { *slate_support::errno_location() }) == 35;
                    #[cfg(not(any(unix, target_arch = "arm")))]
                    let __v9: bool = (unsafe { *slate_support::errno_location() })
                        == (slate_support::EAGAIN as i32)
                        || (unsafe { *slate_support::errno_location() })
                            == (slate_support::EWOULDBLOCK as i32);
                    #[cfg(all(target_arch = "arm", windows))]
                    let __v9: bool = (unsafe { *slate_support::errno_location() }) == 11
                        || (unsafe { *slate_support::errno_location() }) == 140;
                    #[cfg(not(any(target_os = "linux", target_os = "android")))]
                    {
                        if __v9 {
                            unsafe {
                                (*state).again = 1;
                            }
                        }
                    }
                    #[cfg(any(target_os = "linux", target_os = "android"))]
                    {
                        if (unsafe { *slate_support::errno_location() })
                            == (slate_support::EAGAIN as i32)
                            || (unsafe { *slate_support::errno_location() }) == 11
                        {
                            unsafe {
                                (*state).again = 1;
                            }
                        }
                    }
                    unsafe { gz_error(state, -1i32, zstrerror!() as *const c_char) };
                    return -1;
                }
                let __v11: *mut u8 = unsafe { (*state).x.next };
                unsafe {
                    (*state).x.next = __v11.offset(writ as isize);
                }
            }
            if (unsafe { (*strm).avail_out }) == 0 {
                unsafe {
                    (*strm).avail_out = (*state).size;
                }
                unsafe {
                    (*strm).next_out = (*state).out;
                }
                unsafe {
                    (*state).x.next = (*state).out;
                }
            }
        }
        // compress
        have = unsafe { (*strm).avail_out };
        ret = unsafe { deflate(strm, flush) };
        if ret == -2 {
            unsafe {
                gz_error(
                    state,
                    -2i32,
                    c"internal error: deflate stream corrupt".as_ptr(),
                )
            };
            return -1;
        }
        have = have.wrapping_sub(unsafe { (*strm).avail_out });
        if have == 0 {
            break;
        }
    }
    // if that completed a deflate stream, allow another to start
    if flush == 4 {
        unsafe {
            (*state).reset = 1;
        }
    }
    // all done, no errors
    0
}

/// Compress state->skip (> 0) zeros to output.  Return -1 on a write error or
/// memory allocation failure by gz_comp(), or 0 on success. state->skip is
/// updated with the number of successfully written zeros, in case there is a
/// stall on a non-blocking write destination.
fn gz_zero(state: *mut __SlateRecord_gzguts_h_203) -> i32 {
    // compress state->skip zeros
    let mut first: i32 = 1;
    let mut ret: i32 = 0;
    let mut n: u32 = 0;
    let strm: *mut z_stream_s = unsafe { std::ptr::addr_of_mut!((*state).strm) };
    // consume whatever's left in the input buffer
    let __v0: bool = (unsafe { (*strm).avail_in }) != 0 && gz_comp(state, 0i32) == -1;
    if __v0 {
        return -1;
    }
    loop {
        #[cfg(unix)]
        {
            n = if GT_OFF!(unsafe { (*state).size })
                || ((unsafe { (*state).size }) as off_t) > unsafe { (*state).skip }
            {
                (unsafe { (*state).skip }) as u32
            } else {
                unsafe { (*state).size }
            };
        }
        #[cfg(windows)]
        {
            n = if GT_OFF!(unsafe { (*state).size })
                || ((unsafe { (*state).size }) as i64) > unsafe { (*state).skip }
            {
                (unsafe { (*state).skip }) as u32
            } else {
                unsafe { (*state).size }
            };
        }
        if first != 0 {
            #[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
            {
                unsafe {
                    std::ptr::write_bytes(
                        (((*state).r#in as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                        (0i32 as i32) as u8,
                        ((n as size_t) as usize) as usize,
                    )
                };
            }
            #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
            {
                unsafe {
                    std::ptr::write_bytes(
                        ((*state).r#in as *mut ()) as *mut u8,
                        0u8,
                        (n as size_t) as usize,
                    )
                };
            }
            first = 0;
        }
        unsafe {
            (*strm).avail_in = n;
        }
        unsafe {
            (*strm).next_in = (*state).r#in;
        }
        ret = gz_comp(state, 0i32);
        n = n.wrapping_sub(unsafe { (*strm).avail_in });
        unsafe {
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                (*state).x.pos = (*state).x.pos + (n as off_t);
            }
            #[cfg(all(target_pointer_width = "32", unix))]
            {
                (*state).x.pos = ((*state).x.pos as c_ulong).wrapping_add(n as c_ulong) as off_t;
            }
            #[cfg(windows)]
            {
                (*state).x.pos = (*state).x.pos + (n as i64);
            }
        }
        unsafe {
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                (*state).skip = (*state).skip - (n as off_t);
            }
            #[cfg(all(target_pointer_width = "32", unix))]
            {
                (*state).skip = ((*state).skip as c_ulong).wrapping_sub(n as c_ulong) as off_t;
            }
            #[cfg(windows)]
            {
                (*state).skip = (*state).skip - (n as i64);
            }
        }
        if ret == -1 {
            return -1;
        }
        if (unsafe { (*state).skip }) == 0 {
            break;
        }
    }
    0
}

/// Write len bytes from buf to file.  Return the number of bytes written.  If
/// the returned value is less than len, then there was an error. If the error
/// was a non-blocking stall, then the number of bytes consumed is returned.
/// For any other error, 0 is returned.
fn gz_write(state: *mut __SlateRecord_gzguts_h_203, mut buf: *const (), mut len: size_t) -> size_t {
    let put: size_t = len;
    let mut ret: i32 = 0;
    // if len is zero, avoid unnecessary operations
    if len == 0 {
        return 0;
    }
    // allocate memory if this is the first time through
    let __v0: bool = (unsafe { (*state).size }) == 0 && gz_init(state) == -1;
    if __v0 {
        return 0;
    }
    // check for seek request
    let __v1: bool = (unsafe { (*state).skip }) != 0 && gz_zero(state) == -1;
    if __v1 {
        return 0;
    }
    // for small len, copy to input buffer, otherwise compress directly
    if len < ((unsafe { (*state).size }) as size_t) {
        // copy to input buffer, compress when full
        loop {
            let mut have: u32 = 0;
            let mut copy: u32 = 0;
            if (unsafe { (*state).strm.avail_in }) == 0 {
                unsafe {
                    (*state).strm.next_in = (*state).r#in;
                }
            }
            have = ((unsafe {
                unsafe { (*state).strm.next_in }
                    .offset((*state).strm.avail_in as isize)
                    .offset_from((*state).r#in as *mut u8)
            }) as ptrdiff_t) as u32;
            copy = unsafe { (*state).size }.wrapping_sub(have);
            #[cfg(target_pointer_width = "64")]
            {
                if (copy as size_t) > len {
                    copy = len as u32;
                }
            }
            #[cfg(target_pointer_width = "32")]
            {
                if copy > (len as u32) {
                    copy = len as u32;
                }
            }
            #[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (buf as *const core::ffi::c_void) as *const u8,
                        ((unsafe { (*state).r#in }.add(have as usize) as *mut ())
                            as *mut core::ffi::c_void) as *mut u8,
                        ((copy as size_t) as usize) as usize,
                    )
                };
            }
            #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        buf as *const u8,
                        (unsafe { (*state).r#in }.add(have as usize) as *mut ()) as *mut u8,
                        (copy as size_t) as usize,
                    )
                };
            }
            let __v3: u32 = unsafe { (*state).strm.avail_in };
            unsafe {
                (*state).strm.avail_in = __v3.wrapping_add(copy);
            }
            unsafe {
                #[cfg(all(target_pointer_width = "64", unix))]
                {
                    (*state).x.pos = (*state).x.pos + (copy as off_t);
                }
                #[cfg(all(target_pointer_width = "32", unix))]
                {
                    (*state).x.pos =
                        ((*state).x.pos as c_ulong).wrapping_add(copy as c_ulong) as off_t;
                }
                #[cfg(windows)]
                {
                    (*state).x.pos = (*state).x.pos + (copy as i64);
                }
            }
            buf = (unsafe { (buf as *const c_char).add(copy as usize) }) as *const ();
            len = len.wrapping_sub(copy as size_t);
            if len == 0 {
                break;
            }
            if gz_comp(state, 0i32) == -1 {
                return if (unsafe { (*state).again }) != 0 {
                    put.wrapping_sub(len)
                } else {
                    0
                };
            }
        }
    } else {
        // consume whatever's left in the input buffer
        let __v11: bool = (unsafe { (*state).strm.avail_in }) != 0 && gz_comp(state, 0i32) == -1;
        if __v11 {
            return 0;
        }
        // directly compress user buffer to file
        unsafe {
            (*state).strm.next_in = buf as *mut u8;
        }
        loop {
            let mut n: u32 = 4294967295u32;
            #[cfg(target_pointer_width = "64")]
            {
                if (n as size_t) > len {
                    n = len as u32;
                }
            }
            #[cfg(target_pointer_width = "32")]
            {
                if n > (len as u32) {
                    n = len as u32;
                }
            }
            unsafe {
                (*state).strm.avail_in = n;
            }
            ret = gz_comp(state, 0i32);
            n = n.wrapping_sub(unsafe { (*state).strm.avail_in });
            unsafe {
                #[cfg(all(target_pointer_width = "64", unix))]
                {
                    (*state).x.pos = (*state).x.pos + (n as off_t);
                }
                #[cfg(all(target_pointer_width = "32", unix))]
                {
                    (*state).x.pos =
                        ((*state).x.pos as c_ulong).wrapping_add(n as c_ulong) as off_t;
                }
                #[cfg(windows)]
                {
                    (*state).x.pos = (*state).x.pos + (n as i64);
                }
            }
            len = len.wrapping_sub(n as size_t);
            if ret == -1 {
                return if (unsafe { (*state).again }) != 0 {
                    put.wrapping_sub(len)
                } else {
                    0
                };
            }
            if len == 0 {
                break;
            }
        }
    }
    // input was all buffered or compressed
    put
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzwrite(file: *mut gzFile_s, buf: *const (), len: u32) -> i32 {
    // get internal structure
    if file.is_null() {
        return 0;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    // check that we're writing and that there's no (serious) error
    if (unsafe { (*state).mode }) != 31153
        || (unsafe { (*state).err }) != 0 && (unsafe { (*state).again }) == 0
    {
        return 0;
    }
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    // since an int is returned, make sure len fits in one, otherwise return
    // with an error (this avoids a flaw in the interface)
    if (len as i32) < 0 {
        unsafe {
            gz_error(
                state,
                -3i32,
                c"requested length does not fit in int".as_ptr(),
            )
        };
        return 0;
    }
    // write len bytes from buf (the return value will fit in an int)
    gz_write(state, buf, len as size_t) as i32
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzfwrite(
    buf: *const (),
    size: size_t,
    nitems: size_t,
    file: *mut gzFile_s,
) -> size_t {
    let mut len: size_t = 0;
    // get internal structure
    if file.is_null() {
        return 0;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    // check that we're writing and that there's no (serious) error
    if (unsafe { (*state).mode }) != 31153
        || (unsafe { (*state).err }) != 0 && (unsafe { (*state).again }) == 0
    {
        return 0;
    }
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    // compute bytes to read -- error on overflow
    len = nitems.wrapping_mul(size);
    if size != 0 && len / size != nitems {
        unsafe { gz_error(state, -2i32, c"request does not fit in a size_t".as_ptr()) };
        return 0;
    }
    // write len bytes to buf, return the number of full items written
    #[cfg(target_pointer_width = "64")]
    let __v0: u64 = if len != 0 {
        gz_write(state, buf, len) / size
    } else {
        0
    };
    #[cfg(target_pointer_width = "32")]
    let __v0: u32 = if len != 0 {
        gz_write(state, buf, len) / size
    } else {
        0
    };
    __v0
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzputc(file: *mut gzFile_s, c: i32) -> i32 {
    let mut have: u32 = 0;
    let mut buf: [u8; 1] = [0u8; 1];
    let mut state: *mut __SlateRecord_gzguts_h_203 = unsafe { std::mem::zeroed() };
    // get internal structure
    if file.is_null() {
        return -1;
    }
    state = file as *mut __SlateRecord_gzguts_h_203;
    let strm: *mut z_stream_s = unsafe { std::ptr::addr_of_mut!((*state).strm) };
    // check that we're writing and that there's no (serious) error
    if (unsafe { (*state).mode }) != 31153
        || (unsafe { (*state).err }) != 0 && (unsafe { (*state).again }) == 0
    {
        return -1;
    }
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    // check for seek request
    let __v0: bool = (unsafe { (*state).skip }) != 0 && gz_zero(state) == -1;
    if __v0 {
        return -1;
    }
    // try writing to input buffer for speed (state->size == 0 if buffer not
    // initialized)
    if (unsafe { (*state).size }) != 0 {
        if (unsafe { (*strm).avail_in }) == 0 {
            unsafe {
                (*strm).next_in = (*state).r#in;
            }
        }
        have = ((unsafe {
            unsafe { (*strm).next_in }
                .offset((*strm).avail_in as isize)
                .offset_from((*state).r#in as *mut u8)
        }) as ptrdiff_t) as u32;
        if have < unsafe { (*state).size } {
            unsafe {
                *unsafe { (*state).r#in }.add(have as usize) = c as u8;
            }
            let __v2: u32 = unsafe { (*strm).avail_in };
            unsafe {
                (*strm).avail_in = __v2.wrapping_add(1);
            }
            unsafe {
                (*state).x.pos = (*state).x.pos + 1;
            }
            return c & 255;
        }
    }
    // no room in buffer or not initialized, use gz_write()
    buf[0] = c as u8;
    if gz_write(state, buf.as_mut_ptr() as *const (), (1 as size_t)) != 1 {
        return -1;
    }
    c & 255
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzputs(file: *mut gzFile_s, s: *const c_char) -> i32 {
    let mut len: size_t = 0;
    let mut put: size_t = 0;
    // get internal structure
    if file.is_null() {
        return -1;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    // check that we're writing and that there's no (serious) error
    if (unsafe { (*state).mode }) != 31153
        || (unsafe { (*state).err }) != 0 && (unsafe { (*state).again }) == 0
    {
        return -1;
    }
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    // write string
    #[cfg(all(not(target_os = "macos"), target_pointer_width = "64", unix))]
    {
        len = unsafe { strlen(s as *const core::ffi::c_char) as c_ulong };
    }
    #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
    {
        len = unsafe { strlen(s) };
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        len = unsafe { strlen(s as *const core::ffi::c_char) as u64 };
    }
    #[cfg(target_pointer_width = "64")]
    {
        if (len as i32) < 0 || ((len as u32) as size_t) != len {
            unsafe { gz_error(state, -2i32, c"string length does not fit in int".as_ptr()) };
            return -1;
        }
    }
    #[cfg(any(target_arch = "x86", all(target_arch = "arm", unix)))]
    {
        len = unsafe { strlen(s as *const core::ffi::c_char) as u32 };
    }
    #[cfg(target_pointer_width = "32")]
    {
        if (len as i32) < 0 || (len as u32) != (len as u32) {
            unsafe { gz_error(state, -2i32, c"string length does not fit in int".as_ptr()) };
            return -1;
        }
    }
    put = gz_write(state, s as *const (), len);
    if len != 0 && put == 0 { -1 } else { put as i32 }
}

#[cfg(not(feature = "no-vsnprintf"))]
/// If the second half of the input buffer is occupied, write out the contents.
/// If there is any input remaining due to a non-blocking stall on write, move
/// it to the start of the buffer. Return true if this did not open up the
/// second half of the buffer.  state->err should be checked after this to
/// handle a gz_comp() error.
fn gz_vacate(state: *mut __SlateRecord_gzguts_h_203) -> i32 {
    let strm: *mut z_stream_s = unsafe { std::ptr::addr_of_mut!((*state).strm) };
    if unsafe { (*strm).next_in }.is_null()
        || (unsafe { unsafe { (*strm).next_in }.offset((*strm).avail_in as isize) })
            <= unsafe { unsafe { (*state).r#in }.offset((*state).size as isize) }
    {
        return 0;
    }
    gz_comp(state, 0i32);
    if (unsafe { (*strm).avail_in }) == 0 {
        unsafe {
            (*strm).next_in = (*state).r#in;
        }
        return 0;
    }
    #[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
    {
        unsafe {
            std::ptr::copy(
                (((*strm).next_in as *const ()) as *const core::ffi::c_void) as *const u8,
                (((*state).r#in as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (((*strm).avail_in as size_t) as usize) as usize,
            )
        };
    }
    #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
    {
        unsafe {
            std::ptr::copy(
                ((*strm).next_in as *const ()) as *const u8,
                ((*state).r#in as *mut ()) as *mut u8,
                ((*strm).avail_in as size_t) as usize,
            )
        };
    }
    unsafe {
        (*strm).next_in = (*state).r#in;
    }
    ((unsafe { (*strm).avail_in }) > unsafe { (*state).size }) as i32
}

#[cfg(not(any(
    target_arch = "x86",
    all(target_arch = "aarch64", target_os = "macos"),
    windows
)))]
type __SlateParam_gzvprintf_2 = core::ffi::VaList<'static>;

#[cfg(any(
    target_arch = "x86",
    all(target_arch = "aarch64", target_os = "macos"),
    windows
))]
type __SlateParam_gzvprintf_2 = *mut c_char;

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzvprintf(
    file: *mut gzFile_s,
    format: *const c_char,
    mut va: __SlateParam_gzvprintf_2,
) -> i32 {
    #[cfg(not(feature = "no-vsnprintf"))]
    let mut len: i32 = 0;
    #[cfg(not(feature = "no-vsnprintf"))]
    let mut ret: i32 = 0;
    #[cfg(not(feature = "no-vsnprintf"))]
    let mut state: *mut __SlateRecord_gzguts_h_203 = unsafe { std::mem::zeroed() };
    #[cfg(not(feature = "no-vsnprintf"))]
    {
        if file.is_null() {
            return -2;
        }
        state = file as *mut __SlateRecord_gzguts_h_203;
    }
    #[cfg(not(feature = "no-vsnprintf"))]
    let strm: *mut z_stream_s = unsafe { std::ptr::addr_of_mut!((*state).strm) };
    #[cfg(not(feature = "no-vsnprintf"))]
    {
        if (unsafe { (*state).mode }) != 31153
            || (unsafe { (*state).err }) != 0 && (unsafe { (*state).again }) == 0
        {
            return -2;
        }
        unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    }
    #[cfg(not(feature = "no-vsnprintf"))]
    let __v0: bool = (unsafe { (*state).size }) == 0 && gz_init(state) == -1;
    #[cfg(not(feature = "no-vsnprintf"))]
    {
        if __v0 {
            return unsafe { (*state).err };
        }
    }
    #[cfg(not(feature = "no-vsnprintf"))]
    let __v1: bool = (unsafe { (*state).skip }) != 0 && gz_zero(state) == -1;
    #[cfg(not(feature = "no-vsnprintf"))]
    {
        if __v1 {
            return unsafe { (*state).err };
        }
        ret = gz_vacate(state);
        if (unsafe { (*state).err }) != 0 {
            if ret != 0 && (unsafe { (*state).again }) != 0 {
                // There was a non-blocking stall on write, resulting in the part
                // of the second half of the output buffer being occupied.  Return
                // a Z_BUF_ERROR to let the application know that this gzprintf()
                // needs to be retried.
                unsafe { gz_error(state, -5i32, c"stalled write on gzprintf".as_ptr()) };
            }
            if (unsafe { (*state).again }) == 0 {
                return unsafe { (*state).err };
            }
        }
        if (unsafe { (*strm).avail_in }) == 0 {
            unsafe {
                (*strm).next_in = (*state).r#in;
            }
        }
    }
    #[cfg(not(feature = "no-vsnprintf"))]
    let next: *mut c_char = (unsafe {
        unsafe { (*state).r#in }
            .offset(
                (unsafe { (*strm).next_in }.offset_from((*state).r#in as *mut u8) as ptrdiff_t)
                    as isize,
            )
            .offset((*strm).avail_in as isize)
    }) as *mut c_char;
    #[cfg(not(feature = "no-vsnprintf"))]
    {
        unsafe {
            *next.offset(unsafe { (*state).size }.wrapping_sub(1) as isize) = 0;
        }
    }
    #[cfg(all(
        not(any(
            target_arch = "x86",
            all(target_arch = "aarch64", target_os = "macos"),
            windows
        )),
        not(feature = "no-vsnprintf")
    ))]
    {
        len = unsafe { vsnprintf(next, (*state).size as size_t, format, va.clone()) };
    }
    #[cfg(all(
        any(
            all(target_arch = "x86", unix),
            all(target_arch = "aarch64", target_os = "macos")
        ),
        not(feature = "no-vsnprintf")
    ))]
    {
        len = unsafe { vsnprintf(next, (*state).size as size_t, format, va) };
    }
    #[cfg(all(windows, not(feature = "no-vsnprintf")))]
    {
        len = vsnprintf(next, (unsafe { (*state).size }) as size_t, format, va);
    }
    #[cfg(not(feature = "no-vsnprintf"))]
    {
        if len == 0
            || (len as u32) >= unsafe { (*state).size }
            || ((unsafe { *next.offset(unsafe { (*state).size }.wrapping_sub(1) as isize) }) as i32)
                != 0
        {
            return 0;
        }
    }
    #[cfg(not(feature = "no-vsnprintf"))]
    let __v3: u32 = unsafe { (*strm).avail_in };
    #[cfg(not(feature = "no-vsnprintf"))]
    {
        unsafe {
            (*strm).avail_in = __v3.wrapping_add(len as u32);
        }
        unsafe {
            #[cfg(unix)]
            {
                (*state).x.pos = (*state).x.pos + (len as off_t);
            }
            #[cfg(windows)]
            {
                (*state).x.pos = (*state).x.pos + (len as i64);
            }
        }
        ret = gz_vacate(state);
        if (unsafe { (*state).err }) != 0 && (unsafe { (*state).again }) == 0 {
            return unsafe { (*state).err };
        }
        { len }
    }
    #[cfg(all(
        not(any(
            target_arch = "x86",
            all(target_arch = "aarch64", target_os = "macos"),
            windows
        )),
        feature = "no-vsnprintf"
    ))]
    {
        va.clone();
    }
    #[cfg(feature = "no-vsnprintf")]
    {
        -2
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn gzprintf(
    file: *mut gzFile_s,
    mut format: *const c_char,
    mut __va_args: ...
) -> i32 {
    #[cfg(windows)]
    let mut va: *mut c_char = unsafe { std::mem::zeroed() };
    #[cfg(not(any(
        target_arch = "x86",
        all(target_arch = "aarch64", target_os = "macos"),
        windows
    )))]
    let mut va: core::ffi::VaList<'static> = unsafe { std::mem::zeroed() };
    let mut ret: i32 = 0;
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        unsafe {
            *std::ptr::addr_of_mut!(va) =
                core::mem::transmute::<core::ffi::VaList<'_>, *mut i8>(__va_args.clone());
        }
    }
    #[cfg(all(target_arch = "x86", windows))]
    {
        va = unsafe {
            (std::ptr::addr_of_mut!(format) as *mut c_char).offset(
                ((std::mem::size_of::<*const c_char>() as size_t)
                    .wrapping_add(std::mem::size_of::<i32>() as size_t)
                    .wrapping_sub(1)
                    & !(std::mem::size_of::<i32>() as size_t).wrapping_sub(1))
                    as isize,
            )
        };
    }
    #[cfg(all(target_arch = "arm", windows))]
    {
        va = unsafe {
            (std::ptr::addr_of_mut!(format) as *mut c_char).offset(
                ((std::mem::size_of::<*const c_char>() as size_t)
                    .wrapping_add(4)
                    .wrapping_sub(1)
                    & (4294967292 as size_t)) as isize,
            )
        };
    }
    #[cfg(any(
        all(target_arch = "x86", unix),
        all(target_arch = "aarch64", target_os = "macos")
    ))]
    let va: *mut c_char =
        unsafe { core::mem::transmute::<core::ffi::VaList<'_>, *mut i8>(__va_args.clone()) };
    #[cfg(any(
        target_arch = "x86",
        all(target_arch = "aarch64", target_os = "macos"),
        windows
    ))]
    {
        ret = gzvprintf(file, format, va);
    }
    #[cfg(windows)]
    {
        va = std::ptr::null_mut::<c_char>();
    }
    #[cfg(not(any(
        target_arch = "x86",
        all(target_arch = "aarch64", target_os = "macos"),
        windows
    )))]
    {
        va = unsafe {
            core::mem::transmute::<core::ffi::VaList<'_>, core::ffi::VaList<'static>>(
                __va_args.clone(),
            )
        };
        ret = gzvprintf(file, format, va.clone());
    }
    ret
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzflush(file: *mut gzFile_s, flush: i32) -> i32 {
    // get internal structure
    if file.is_null() {
        return -2;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    // check that we're writing and that there's no (serious) error
    if (unsafe { (*state).mode }) != 31153
        || (unsafe { (*state).err }) != 0 && (unsafe { (*state).again }) == 0
    {
        return -2;
    }
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    // check flush parameter
    if flush < 0 || flush > 4 {
        return -2;
    }
    // check for seek request
    let __v0: bool = (unsafe { (*state).skip }) != 0 && gz_zero(state) == -1;
    if __v0 {
        return unsafe { (*state).err };
    }
    // compress remaining data with requested flush
    gz_comp(state, flush);
    unsafe { (*state).err }
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzsetparams(file: *mut gzFile_s, level: i32, strategy: i32) -> i32 {
    let mut state: *mut __SlateRecord_gzguts_h_203 = unsafe { std::mem::zeroed() };
    // get internal structure
    if file.is_null() {
        return -2;
    }
    state = file as *mut __SlateRecord_gzguts_h_203;
    let strm: *mut z_stream_s = unsafe { std::ptr::addr_of_mut!((*state).strm) };
    // check that we're compressing and that there's no (serious) error
    if (unsafe { (*state).mode }) != 31153
        || (unsafe { (*state).err }) != 0 && (unsafe { (*state).again }) == 0
        || (unsafe { (*state).direct }) != 0
    {
        return -2;
    }
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    // if no change is requested, then do nothing
    if level == unsafe { (*state).level } && strategy == unsafe { (*state).strategy } {
        return 0;
    }
    // check for seek request
    let __v0: bool = (unsafe { (*state).skip }) != 0 && gz_zero(state) == -1;
    if __v0 {
        return unsafe { (*state).err };
    }
    // change compression parameters for subsequent input
    if (unsafe { (*state).size }) != 0 {
        // flush previous input with previous parameters before changing
        let __v1: bool = (unsafe { (*strm).avail_in }) != 0 && gz_comp(state, 5i32) == -1;
        if __v1 {
            return unsafe { (*state).err };
        }
        unsafe { deflateParams(strm, level, strategy) };
    }
    unsafe {
        (*state).level = level;
    }
    unsafe {
        (*state).strategy = strategy;
    }
    0
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzclose_w(file: *mut gzFile_s) -> i32 {
    let mut ret: i32 = 0;
    let mut state: *mut __SlateRecord_gzguts_h_203 = unsafe { std::mem::zeroed() };
    // get internal structure
    if file.is_null() {
        return -2;
    }
    state = file as *mut __SlateRecord_gzguts_h_203;
    // check that we're writing
    if (unsafe { (*state).mode }) != 31153 {
        return -2;
    }
    // check for seek request
    let __v0: bool = (unsafe { (*state).skip }) != 0 && gz_zero(state) == -1;
    if __v0 {
        ret = unsafe { (*state).err };
    }
    // flush, free memory, and close file
    if gz_comp(state, 4i32) == -1 {
        ret = unsafe { (*state).err };
    }
    if (unsafe { (*state).size }) != 0 {
        if (unsafe { (*state).direct }) == 0 {
            unsafe { deflateEnd(std::ptr::addr_of_mut!((*state).strm)) };
            #[cfg(any(
                target_os = "linux",
                target_os = "freebsd",
                all(not(target_arch = "arm"), windows)
            ))]
            {
                unsafe { free(((*state).out as *mut ()) as *mut core::ffi::c_void) };
            }
            #[cfg(any(
                target_os = "macos",
                target_os = "android",
                all(target_arch = "arm", windows)
            ))]
            {
                unsafe { free((*state).out as *mut ()) };
            }
        }
        #[cfg(any(
            target_os = "linux",
            target_os = "freebsd",
            all(not(target_arch = "arm"), windows)
        ))]
        {
            unsafe { free(((*state).r#in as *mut ()) as *mut core::ffi::c_void) };
        }
        #[cfg(any(
            target_os = "macos",
            target_os = "android",
            all(target_arch = "arm", windows)
        ))]
        {
            unsafe { free((*state).r#in as *mut ()) };
        }
    }
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        all(not(target_arch = "arm"), windows)
    ))]
    {
        unsafe { free(((*state).path as *mut ()) as *mut core::ffi::c_void) };
    }
    #[cfg(target_os = "android")]
    {
        unsafe { free((*state).path as *mut ()) };
    }
    #[cfg(all(not(target_os = "macos"), unix))]
    {
        if (unsafe { close((*state).fd as i32) as i32 }) == -1 {
            ret = -1;
        }
    }
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        unsafe { free((state as *mut ()) as *mut core::ffi::c_void) };
    }
    #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
    {
        unsafe { free((*state).path as *mut ()) };
    }
    #[cfg(any(target_os = "macos", windows))]
    {
        if (unsafe { close((*state).fd) }) == -1 {
            ret = -1;
        }
    }
    #[cfg(any(
        target_os = "macos",
        target_os = "android",
        all(target_arch = "arm", windows)
    ))]
    {
        unsafe { free(state as *mut ()) };
    }
    #[cfg(not(any(unix, target_arch = "arm")))]
    {
        unsafe { free((state as *mut ()) as *mut core::ffi::c_void) };
    }
    ret
}
