//! gzlib.c -- zlib functions common to reading and writing gzip files
//! Copyright (C) 2004-2026 Mark Adler
//! For conditions of distribution and use, see copyright notice in zlib.h
#[cfg(all(target_pointer_width = "32", windows))]
macro_rules! GT_OFF__8 {
    ($x:expr) => {{
        let __v: bool = (std::mem::size_of::<i32>() as u32) == (std::mem::size_of::<i64>() as u32)
            && ($x) > gz_intmax();
        __v
    }};
}

#[cfg(all(target_pointer_width = "64", windows))]
macro_rules! GT_OFF__5 {
    ($x:expr) => {{
        let __v: bool = (std::mem::size_of::<i32>() as u64) == (std::mem::size_of::<i64>() as u64)
            && ($x) > gz_intmax();
        __v
    }};
}

#[cfg(all(target_pointer_width = "32", unix))]
macro_rules! GT_OFF__7 {
    ($x:expr) => {{
        let __v: bool = (std::mem::size_of::<i32>() as u32)
            == (std::mem::size_of::<off_t>() as u32)
            && ($x) > gz_intmax();
        __v
    }};
}

#[cfg(all(target_pointer_width = "64", unix))]
macro_rules! GT_OFF__6 {
    ($x:expr) => {{
        let __v: bool = (std::mem::size_of::<i32>() as c_ulong)
            == (std::mem::size_of::<off_t>() as c_ulong)
            && ($x) > gz_intmax();
        __v
    }};
}

#[cfg(all(target_pointer_width = "64", windows))]
unsafe extern "C" {
    fn __va_start(arg0: *mut *mut c_char, ...);
}

#[cfg(windows)]
unsafe extern "C" {
    fn __stdio_common_vsprintf(
        arg0: u64,
        arg1: *mut c_char,
        arg2: size_t,
        arg3: *const c_char,
        arg4: *mut __crt_locale_pointers,
        arg5: *mut c_char,
    ) -> i32;
    fn wcstombs(arg0: *mut c_char, arg1: *const wchar_t, arg2: size_t) -> size_t;
    fn _wopen(arg0: *const wchar_t, arg1: i32, ...) -> i32;
    fn _lseeki64(arg0: i32, arg1: i64, arg2: i32) -> i64;
}

#[cfg(all(unix, feature = "no-vsnprintf"))]
unsafe extern "C" {
    fn sprintf(arg0: *mut c_char, arg1: *const c_char, ...) -> i32;
}

#[cfg(all(unix, not(feature = "no-vsnprintf")))]
unsafe extern "C" {
    fn snprintf(arg0: *mut c_char, arg1: size_t, arg2: *const c_char, ...) -> i32;
}

#[cfg(target_env = "gnu")]
unsafe extern "C" {
    fn lseek(arg0: i32, arg1: c_long, arg2: i32) -> c_long;
}

#[cfg(not(any(target_env = "gnu", target_os = "freebsd", windows)))]
unsafe extern "C" {
    fn lseek(arg0: i32, arg1: off_t, arg2: i32) -> off_t;
}

#[cfg(target_os = "freebsd")]
unsafe extern "C" {
    fn lseek(arg0: i32, arg1: i64, arg2: i32) -> i64;
}

#[cfg(feature = "no-vsnprintf")]
unsafe extern "C" {
    fn strcpy(arg0: *mut c_char, arg1: *const c_char) -> *mut c_char;
    fn strcat(arg0: *mut c_char, arg1: *const c_char) -> *mut c_char;
}

unsafe extern "C" {
    fn strlen(arg0: *const c_char) -> size_t;
    fn malloc(arg0: size_t) -> *mut ();
    fn free(arg0: *mut ());
    fn open(arg0: *const c_char, arg1: i32, ...) -> i32;
}

#[cfg(unix)]
unsafe extern "C" {
    fn fcntl(arg0: i32, arg1: i32, ...) -> i32;
}

use core::ffi::c_char;
#[cfg(target_env = "gnu")]
use core::ffi::c_long;
use core::ffi::c_ulong;
#[cfg(unix)]
use slate_support::off_t;
use slate_support::size_t;
#[cfg(windows)]
use slate_support::wchar_t;

#[cfg(windows)]
#[repr(C)]
#[derive(Clone, Copy)]
struct __crt_locale_pointers {
    locinfo: *mut __crt_locale_data,
    mbcinfo: *mut __crt_multibyte_data,
}

#[cfg(windows)]
#[repr(C)]
#[derive(Clone, Copy)]
struct __crt_locale_data {}

#[cfg(windows)]
#[repr(C)]
#[derive(Clone, Copy)]
struct __crt_multibyte_data {}

#[repr(C)]
#[derive(Clone, Copy)]
struct internal_state {}

#[cfg(windows)]
fn __local_stdio_printf_options() -> *mut u64 {
    unsafe { std::ptr::addr_of_mut!(_OptionsStorage) }
}

#[cfg(windows)]
static mut _OptionsStorage: u64 = unsafe { std::mem::zeroed() };

#[cfg(all(windows, feature = "no-vsnprintf"))]
fn _vsnprintf_l(
    _Buffer: *mut c_char,
    _BufferCount: size_t,
    _Format: *const c_char,
    _Locale: *mut __crt_locale_pointers,
    _ArgList: *mut c_char,
) -> i32 {
    let _Result: i32 = unsafe {
        __stdio_common_vsprintf(
            *__local_stdio_printf_options() | 1u64,
            _Buffer,
            _BufferCount,
            _Format,
            _Locale,
            _ArgList,
        )
    };
    if _Result < 0 { -1 } else { _Result }
}

#[cfg(all(target_pointer_width = "64", windows, feature = "no-vsnprintf"))]
const __SLATE_HOIST__vsprintf_l: size_t = (18446744073709551615 as size_t);

#[cfg(all(target_pointer_width = "32", windows, feature = "no-vsnprintf"))]
const __SLATE_HOIST__vsprintf_l: size_t = (4294967295 as size_t);

#[cfg(all(windows, feature = "no-vsnprintf"))]
fn _vsprintf_l(
    _Buffer: *mut c_char,
    _Format: *const c_char,
    _Locale: *mut __crt_locale_pointers,
    _ArgList: *mut c_char,
) -> i32 {
    _vsnprintf_l(
        _Buffer,
        __SLATE_HOIST__vsprintf_l,
        _Format,
        _Locale,
        _ArgList,
    )
}

#[cfg(all(windows, feature = "no-vsnprintf"))]
unsafe extern "C-unwind" fn sprintf(
    _Buffer: *mut c_char,
    _Format: *const c_char,
    mut __va_args: ...
) -> i32 {
    let mut _Result: i32 = 0;
    let mut _ArgList: *mut c_char = unsafe { std::mem::zeroed() };
    #[cfg(target_pointer_width = "64")]
    {
        unsafe {
            *std::ptr::addr_of_mut!(_ArgList) =
                core::mem::transmute::<core::ffi::VaList<'_>, *mut i8>(__va_args.clone());
        }
    }
    #[cfg(target_arch = "x86")]
    {
        _ArgList = unsafe {
            (std::ptr::addr_of!(_Format) as *mut c_char).offset(
                ((std::mem::size_of::<*const c_char>() as u32)
                    .wrapping_add(std::mem::size_of::<i32>() as u32)
                    .wrapping_sub(1)
                    & !(std::mem::size_of::<i32>() as u32).wrapping_sub(1))
                    as isize,
            )
        };
    }
    #[cfg(target_arch = "arm")]
    {
        _ArgList = unsafe {
            (std::ptr::addr_of!(_Format) as *mut c_char).offset(
                ((std::mem::size_of::<*const c_char>() as u32)
                    .wrapping_add(4)
                    .wrapping_sub(1)
                    & 4294967292u32) as isize,
            )
        };
    }
    _Result = _vsprintf_l(
        _Buffer,
        _Format,
        std::ptr::null_mut::<__crt_locale_pointers>(),
        _ArgList,
    );
    _ArgList = std::ptr::null_mut::<c_char>();
    _Result
}

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

#[cfg(all(windows, not(feature = "no-vsnprintf")))]
unsafe extern "C-unwind" fn snprintf(
    _Buffer: *mut c_char,
    _BufferCount: size_t,
    _Format: *const c_char,
    mut __va_args: ...
) -> i32 {
    let mut _Result: i32 = 0;
    let mut _ArgList: *mut c_char = unsafe { std::mem::zeroed() };
    #[cfg(target_pointer_width = "64")]
    {
        unsafe {
            *std::ptr::addr_of_mut!(_ArgList) =
                core::mem::transmute::<core::ffi::VaList<'_>, *mut i8>(__va_args.clone());
        }
        _Result = vsnprintf(_Buffer, _BufferCount as u64, _Format, _ArgList);
    }
    #[cfg(target_arch = "x86")]
    {
        _ArgList = unsafe {
            (std::ptr::addr_of!(_Format) as *mut c_char).offset(
                ((std::mem::size_of::<*const c_char>() as u32)
                    .wrapping_add(std::mem::size_of::<i32>() as u32)
                    .wrapping_sub(1)
                    & !(std::mem::size_of::<i32>() as u32).wrapping_sub(1))
                    as isize,
            )
        };
    }
    #[cfg(target_arch = "arm")]
    {
        _ArgList = unsafe {
            (std::ptr::addr_of!(_Format) as *mut c_char).offset(
                ((std::mem::size_of::<*const c_char>() as u32)
                    .wrapping_add(4)
                    .wrapping_sub(1)
                    & 4294967292u32) as isize,
            )
        };
    }
    #[cfg(target_pointer_width = "32")]
    {
        _Result = vsnprintf(_Buffer, _BufferCount as u32, _Format, _ArgList);
    }
    _ArgList = std::ptr::null_mut::<c_char>();
    _Result
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

/// Reset gzip file state
fn gz_reset(state: *mut __SlateRecord_gzguts_h_203) {
    unsafe {
        (*state).x.have = 0;
    }
    // no output data available
    if (unsafe { (*state).mode }) == 7247 {
        // for reading ...
        unsafe {
            (*state).eof = 0;
        }
        // not at end of file
        unsafe {
            (*state).past = 0;
        }
        // have not read past end yet
        unsafe {
            (*state).how = 0;
        }
        // look for gzip header
        unsafe {
            (*state).junk = -1;
        }
    // mark first member
    } else {
        unsafe {
            (*state).reset = 0;
        }
    }
    // for writing ...
    // no deflateReset pending
    unsafe {
        (*state).again = 0;
    }
    // no stalled i/o yet
    unsafe {
        (*state).skip = 0;
    }
    // no seek request pending
    gz_error(state, 0i32, std::ptr::null::<c_char>()); // clear error
    unsafe {
        (*state).x.pos = 0;
    }
    // no uncompressed data yet
    unsafe {
        (*state).strm.avail_in = 0;
    }
    // no input data yet
}

/// Open a gzip file either by name or file descriptor.
fn gz_open(path: *const (), fd: i32, mut mode: *const c_char) -> *mut gzFile_s {
    let mut len: size_t = 0;
    let mut oflag: i32 = 0;
    let mut exclusive: i32 = 0;
    // check input
    if path.is_null() || mode.is_null() {
        return std::ptr::null_mut::<gzFile_s>();
    }
    // allocate gzFile structure to return
    #[cfg(all(target_pointer_width = "64", windows))]
    let state: *mut __SlateRecord_gzguts_h_203 =
        (unsafe { malloc(std::mem::size_of::<__SlateRecord_gzguts_h_203>() as u64) })
            as *mut __SlateRecord_gzguts_h_203;
    #[cfg(target_pointer_width = "32")]
    let state: *mut __SlateRecord_gzguts_h_203 =
        (unsafe { malloc(std::mem::size_of::<__SlateRecord_gzguts_h_203>() as u32) })
            as *mut __SlateRecord_gzguts_h_203;
    #[cfg(all(target_pointer_width = "64", unix))]
    let state: *mut __SlateRecord_gzguts_h_203 =
        (unsafe { malloc(std::mem::size_of::<__SlateRecord_gzguts_h_203>() as c_ulong) })
            as *mut __SlateRecord_gzguts_h_203;
    if state.is_null() {
        return std::ptr::null_mut::<gzFile_s>();
    }
    unsafe {
        (*state).size = 0;
    }
    // no buffers allocated yet
    unsafe {
        (*state).want = 8192;
    }
    // requested buffer size
    unsafe {
        (*state).err = 0;
    }
    // no error yet
    unsafe {
        (*state).msg = std::ptr::null_mut::<c_char>();
    }
    // no error message yet
    // interpret mode
    unsafe {
        (*state).mode = 0;
    }
    unsafe {
        (*state).level = -1;
    }
    unsafe {
        (*state).strategy = 0;
    }
    unsafe {
        (*state).direct = 0;
    }
    while (unsafe { *mode }) != 0 {
        #[cfg(any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_os = "macos",
            windows
        ))]
        {
            if ((unsafe { *mode }) as i32) >= 48 && ((unsafe { *mode }) as i32) <= 57 {
                unsafe {
                    (*state).level = (*mode as i32) - 48;
                }
            } else {
                '__slate_break_0: {
                    #[cfg(all(windows, not(feature = "no-gzcompress")))]
                    {
                        match (unsafe { *mode }) as i32 {
                            114 => unsafe {
                                (*state).mode = 7247;
                            },
                            119 => unsafe {
                                (*state).mode = 31153;
                            },
                            97 => unsafe {
                                (*state).mode = 1;
                            },
                            43 => {
                                unsafe { free(state as *mut ()) }; // can't read and write at the same time
                                return std::ptr::null_mut::<gzFile_s>();
                            }
                            98 => {
                                break '__slate_break_0; // ignore -- will request binary anyway
                            }
                            120 => {
                                exclusive = 1;
                            }
                            102 => unsafe {
                                (*state).strategy = 1;
                            },
                            104 => unsafe {
                                (*state).strategy = 2;
                            },
                            82 => unsafe {
                                (*state).strategy = 3;
                            },
                            70 => unsafe {
                                (*state).strategy = 4;
                            },
                            71 => unsafe {
                                (*state).direct = -1;
                            },
                            84 => unsafe {
                                (*state).direct = 1;
                            },
                            _ => {
                                // could consider as an error, but just ignore
                            }
                        }
                    }
                    #[cfg(all(unix, not(feature = "no-gzcompress")))]
                    {
                        match (unsafe { *mode }) as i32 {
                            114 => unsafe {
                                (*state).mode = 7247;
                            },
                            119 => unsafe {
                                (*state).mode = 31153;
                            },
                            97 => unsafe {
                                (*state).mode = 1;
                            },
                            43 => {
                                unsafe { free(state as *mut ()) }; // can't read and write at the same time
                                return std::ptr::null_mut::<gzFile_s>();
                            }
                            98 => {
                                break '__slate_break_0; // ignore -- will request binary anyway
                            }
                            101 => {
                                #[cfg(any(target_os = "linux", target_os = "android"))]
                                {
                                    oflag |= 524288;
                                }
                                #[cfg(target_os = "macos")]
                                {
                                    oflag |= 16777216;
                                }
                                #[cfg(target_os = "freebsd")]
                                {
                                    oflag |= 1048576;
                                }
                            }
                            120 => {
                                exclusive = 1;
                            }
                            102 => unsafe {
                                (*state).strategy = 1;
                            },
                            104 => unsafe {
                                (*state).strategy = 2;
                            },
                            82 => unsafe {
                                (*state).strategy = 3;
                            },
                            70 => unsafe {
                                (*state).strategy = 4;
                            },
                            71 => unsafe {
                                (*state).direct = -1;
                            },
                            78 => {
                                #[cfg(any(target_os = "linux", target_os = "android"))]
                                {
                                    oflag |= 2048;
                                }
                                #[cfg(any(target_os = "macos", target_os = "freebsd"))]
                                {
                                    oflag |= 4;
                                }
                            }
                            84 => unsafe {
                                (*state).direct = 1;
                            },
                            _ => {
                                // could consider as an error, but just ignore
                            }
                        }
                    }
                    #[cfg(all(windows, feature = "no-gzcompress"))]
                    {
                        match (unsafe { *mode }) as i32 {
                            114 => unsafe {
                                (*state).mode = 7247;
                            },
                            43 => {
                                unsafe { free(state as *mut ()) }; // can't read and write at the same time
                                return std::ptr::null_mut::<gzFile_s>();
                            }
                            98 => {
                                break '__slate_break_0; // ignore -- will request binary anyway
                            }
                            120 => {
                                exclusive = 1;
                            }
                            102 => unsafe {
                                (*state).strategy = 1;
                            },
                            104 => unsafe {
                                (*state).strategy = 2;
                            },
                            82 => unsafe {
                                (*state).strategy = 3;
                            },
                            70 => unsafe {
                                (*state).strategy = 4;
                            },
                            71 => unsafe {
                                (*state).direct = -1;
                            },
                            84 => unsafe {
                                (*state).direct = 1;
                            },
                            _ => {
                                // could consider as an error, but just ignore
                            }
                        }
                    }
                    #[cfg(all(unix, feature = "no-gzcompress"))]
                    {
                        match (unsafe { *mode }) as i32 {
                            114 => unsafe {
                                (*state).mode = 7247;
                            },
                            43 => {
                                unsafe { free(state as *mut ()) }; // can't read and write at the same time
                                return std::ptr::null_mut::<gzFile_s>();
                            }
                            98 => {
                                break '__slate_break_0; // ignore -- will request binary anyway
                            }
                            101 => {
                                #[cfg(any(target_os = "linux", target_os = "android"))]
                                {
                                    oflag |= 524288;
                                }
                                #[cfg(target_os = "macos")]
                                {
                                    oflag |= 16777216;
                                }
                                #[cfg(target_os = "freebsd")]
                                {
                                    oflag |= 1048576;
                                }
                            }
                            120 => {
                                exclusive = 1;
                            }
                            102 => unsafe {
                                (*state).strategy = 1;
                            },
                            104 => unsafe {
                                (*state).strategy = 2;
                            },
                            82 => unsafe {
                                (*state).strategy = 3;
                            },
                            70 => unsafe {
                                (*state).strategy = 4;
                            },
                            71 => unsafe {
                                (*state).direct = -1;
                            },
                            78 => {
                                #[cfg(any(target_os = "linux", target_os = "android"))]
                                {
                                    oflag |= 2048;
                                }
                                #[cfg(any(target_os = "macos", target_os = "freebsd"))]
                                {
                                    oflag |= 4;
                                }
                            }
                            84 => unsafe {
                                (*state).direct = 1;
                            },
                            _ => {
                                // could consider as an error, but just ignore
                            }
                        }
                    }
                }
            }
        }
        #[cfg(not(any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_os = "macos",
            windows
        )))]
        {
            if (((unsafe { *mode }) as u32) as i32) >= 48
                && (((unsafe { *mode }) as u32) as i32) <= 57
            {
                unsafe {
                    (*state).level = ((*mode as u32) as i32) - 48;
                }
            } else {
                '__slate_break_1: {
                    #[cfg(not(feature = "no-gzcompress"))]
                    {
                        match ((unsafe { *mode }) as u32) as i32 {
                            114 => unsafe {
                                (*state).mode = 7247;
                            },
                            119 => unsafe {
                                (*state).mode = 31153;
                            },
                            97 => unsafe {
                                (*state).mode = 1;
                            },
                            43 => {
                                unsafe { free(state as *mut ()) }; // can't read and write at the same time
                                return std::ptr::null_mut::<gzFile_s>();
                            }
                            98 => {
                                break '__slate_break_1; // ignore -- will request binary anyway
                            }
                            101 => {
                                #[cfg(not(target_os = "freebsd"))]
                                {
                                    oflag |= 524288;
                                }
                                #[cfg(target_os = "freebsd")]
                                {
                                    oflag |= 1048576;
                                }
                            }
                            120 => {
                                exclusive = 1;
                            }
                            102 => unsafe {
                                (*state).strategy = 1;
                            },
                            104 => unsafe {
                                (*state).strategy = 2;
                            },
                            82 => unsafe {
                                (*state).strategy = 3;
                            },
                            70 => unsafe {
                                (*state).strategy = 4;
                            },
                            71 => unsafe {
                                (*state).direct = -1;
                            },
                            78 => {
                                #[cfg(not(target_os = "freebsd"))]
                                {
                                    oflag |= 2048;
                                }
                                #[cfg(target_os = "freebsd")]
                                {
                                    oflag |= 4;
                                }
                            }
                            84 => unsafe {
                                (*state).direct = 1;
                            },
                            _ => {
                                // could consider as an error, but just ignore
                            }
                        }
                    }
                    #[cfg(feature = "no-gzcompress")]
                    {
                        match ((unsafe { *mode }) as u32) as i32 {
                            114 => unsafe {
                                (*state).mode = 7247;
                            },
                            43 => {
                                unsafe { free(state as *mut ()) }; // can't read and write at the same time
                                return std::ptr::null_mut::<gzFile_s>();
                            }
                            98 => {
                                break '__slate_break_1; // ignore -- will request binary anyway
                            }
                            101 => {
                                #[cfg(not(target_os = "freebsd"))]
                                {
                                    oflag |= 524288;
                                }
                                #[cfg(target_os = "freebsd")]
                                {
                                    oflag |= 1048576;
                                }
                            }
                            120 => {
                                exclusive = 1;
                            }
                            102 => unsafe {
                                (*state).strategy = 1;
                            },
                            104 => unsafe {
                                (*state).strategy = 2;
                            },
                            82 => unsafe {
                                (*state).strategy = 3;
                            },
                            70 => unsafe {
                                (*state).strategy = 4;
                            },
                            71 => unsafe {
                                (*state).direct = -1;
                            },
                            78 => {
                                #[cfg(not(target_os = "freebsd"))]
                                {
                                    oflag |= 2048;
                                }
                                #[cfg(target_os = "freebsd")]
                                {
                                    oflag |= 4;
                                }
                            }
                            84 => unsafe {
                                (*state).direct = 1;
                            },
                            _ => {
                                // could consider as an error, but just ignore
                            }
                        }
                    }
                }
            }
        }
        let __v1: *const c_char = mode;
        mode = unsafe { __v1.add(1) };
    }
    // must provide an "r", "w", or "a"
    if (unsafe { (*state).mode }) == 0 {
        unsafe { free(state as *mut ()) };
        return std::ptr::null_mut::<gzFile_s>();
    }
    // direct is 0, 1 if "T", or -1 if "G" (last "G" or "T" wins)
    if (unsafe { (*state).mode }) == 7247 {
        if (unsafe { (*state).direct }) == 1 {
            // can't force a transparent read
            unsafe { free(state as *mut ()) };
            return std::ptr::null_mut::<gzFile_s>();
        }
        if (unsafe { (*state).direct }) == 0 {
            unsafe {
                (*state).direct = 1;
            }
        }
    // default when reading is auto-detect of gzip vs. transparent --
    // start with a transparent assumption in case of an empty file
    } else {
        if (unsafe { (*state).direct }) == -1 {
            // "G" has no meaning when writing -- disallow it
            unsafe { free(state as *mut ()) };
            return std::ptr::null_mut::<gzFile_s>();
        }
    }
    // if reading, direct == 1 for auto-detect, -1 for gzip only; if writing or
    // appending, direct == 0 for gzip, 1 for transparent (copy in to out)
    // save the path name for error messages
    #[cfg(windows)]
    {
        if fd == -2 {
            len = unsafe {
                wcstombs(
                    std::ptr::null_mut::<c_char>(),
                    path as *const wchar_t,
                    (0 as size_t),
                )
            };
        } else {
            len = (unsafe { strlen(path as *const c_char) }) as size_t;
        }
    }
    #[cfg(unix)]
    {
        len = (unsafe { strlen(path as *const c_char) }) as size_t;
    }
    unsafe {
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            (*state).path = malloc(len.wrapping_add(1) as u64) as *mut c_char;
        }
        #[cfg(target_pointer_width = "32")]
        {
            (*state).path = malloc((len as u32).wrapping_add(1)) as *mut c_char;
        }
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            (*state).path = malloc(len.wrapping_add(1) as c_ulong) as *mut c_char;
        }
    }
    if unsafe { (*state).path }.is_null() {
        unsafe { free(state as *mut ()) };
        return std::ptr::null_mut::<gzFile_s>();
    }
    #[cfg(windows)]
    {
        if fd == -2 {
            if len != 0 {
                #[cfg(target_pointer_width = "64")]
                {
                    unsafe { wcstombs((*state).path, path as *const wchar_t, len.wrapping_add(1)) };
                }
                #[cfg(target_pointer_width = "32")]
                {
                    unsafe {
                        wcstombs(
                            (*state).path,
                            path as *const wchar_t,
                            (len as u32).wrapping_add(1) as size_t,
                        )
                    };
                }
            } else {
                unsafe {
                    *(*state).path = 0;
                }
            }
        } else {
            #[cfg(all(target_pointer_width = "64", not(feature = "no-vsnprintf")))]
            {
                unsafe {
                    snprintf(
                        (*state).path,
                        len.wrapping_add(1) as u64,
                        c"%s".as_ptr(),
                        path as *const c_char,
                    )
                };
            }
            #[cfg(feature = "no-vsnprintf")]
            {
                unsafe { strcpy((*state).path, path as *const c_char) };
            }
            #[cfg(all(target_pointer_width = "32", not(feature = "no-vsnprintf")))]
            {
                unsafe {
                    snprintf(
                        (*state).path,
                        (len as u32).wrapping_add(1),
                        c"%s".as_ptr(),
                        path as *const c_char,
                    )
                };
            }
        }
    }
    #[cfg(all(target_pointer_width = "64", unix, not(feature = "no-vsnprintf")))]
    {
        unsafe {
            snprintf(
                (*state).path,
                len.wrapping_add(1) as c_ulong,
                c"%s".as_ptr(),
                path as *const c_char,
            )
        };
    }
    #[cfg(all(unix, feature = "no-vsnprintf"))]
    {
        unsafe { strcpy((*state).path, path as *const c_char) };
    }
    #[cfg(all(target_pointer_width = "32", unix, not(feature = "no-vsnprintf")))]
    {
        unsafe {
            snprintf(
                (*state).path,
                (len as u32).wrapping_add(1),
                c"%s".as_ptr(),
                path as *const c_char,
            )
        };
    }
    // compute the flags for open()
    #[cfg(windows)]
    {
        oflag |= 32768
            | if (unsafe { (*state).mode }) == 7247 {
                0
            } else {
                1 | 256
                    | if exclusive != 0 { 1024 } else { 0 }
                    | if (unsafe { (*state).mode }) == 31153 {
                        512
                    } else {
                        8
                    }
            };
    }
    #[cfg(target_env = "gnu")]
    {
        oflag |= if (unsafe { (*state).mode }) == 7247 {
            0
        } else {
            1 | 64
                | if exclusive != 0 { 128 } else { 0 }
                | if (unsafe { (*state).mode }) == 31153 {
                    512
                } else {
                    1024
                }
        };
    }
    #[cfg(not(any(
        all(not(target_env = "musl"), not(target_os = "android")),
        target_arch = "aarch64"
    )))]
    {
        oflag |= 32768
            | if (unsafe { (*state).mode }) == 7247 {
                0
            } else {
                1 | 64
                    | if exclusive != 0 { 128 } else { 0 }
                    | if (unsafe { (*state).mode }) == 31153 {
                        512
                    } else {
                        1024
                    }
            };
    }
    #[cfg(not(any(
        target_arch = "x86_64",
        all(not(target_env = "musl"), not(target_os = "android"))
    )))]
    {
        oflag |= 131072
            | if (unsafe { (*state).mode }) == 7247 {
                0
            } else {
                1 | 64
                    | if exclusive != 0 { 128 } else { 0 }
                    | if (unsafe { (*state).mode }) == 31153 {
                        512
                    } else {
                        1024
                    }
            };
    }
    #[cfg(any(target_os = "macos", target_os = "freebsd"))]
    {
        oflag |= if (unsafe { (*state).mode }) == 7247 {
            0
        } else {
            1 | 512
                | if exclusive != 0 { 2048 } else { 0 }
                | if (unsafe { (*state).mode }) == 31153 {
                    1024
                } else {
                    8
                }
        };
    }
    // open the file with the appropriate flags (or just use fd)
    if fd == -1 {
        unsafe {
            (*state).fd = open(path as *const c_char, oflag, 438i32);
        }
    } else {
        #[cfg(windows)]
        {
            if fd == -2 {
                unsafe {
                    (*state).fd = _wopen(path as *const wchar_t, oflag, 384i32);
                }
            } else {
                unsafe {
                    (*state).fd = fd;
                }
            }
        }
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            if oflag & 2048 != 0 {
                unsafe { fcntl(fd, 4i32, fcntl(fd, 3i32) | 2048) };
            }
            if oflag & 524288 != 0 {
                unsafe { fcntl(fd, 2i32, fcntl(fd, 1i32) | 524288) };
            }
        }
        #[cfg(any(target_os = "macos", target_os = "freebsd"))]
        {
            if oflag & 4 != 0 {
                unsafe { fcntl(fd, 4i32, fcntl(fd, 3i32) | 4) };
            }
        }
        #[cfg(target_os = "macos")]
        {
            if oflag & 16777216 != 0 {
                unsafe { fcntl(fd, 2i32, fcntl(fd, 1i32) | 16777216) };
            }
        }
        #[cfg(target_os = "freebsd")]
        {
            if oflag & 1048576 != 0 {
                unsafe { fcntl(fd, 2i32, fcntl(fd, 1i32) | 1048576) };
            }
        }
        #[cfg(unix)]
        {
            unsafe {
                (*state).fd = fd;
            }
        }
    }
    if (unsafe { (*state).fd }) == -1 {
        unsafe { free((*state).path as *mut ()) };
        unsafe { free(state as *mut ()) };
        return std::ptr::null_mut::<gzFile_s>();
    }
    if (unsafe { (*state).mode }) == 1 {
        #[cfg(windows)]
        {
            unsafe { _lseeki64((*state).fd, 0i64, 2i32) };
        }
        #[cfg(target_env = "gnu")]
        {
            unsafe { lseek((*state).fd, (0 as c_long), 2i32) };
        }
        #[cfg(not(any(target_env = "gnu", target_os = "freebsd", windows)))]
        {
            unsafe { lseek((*state).fd, (0 as off_t), 2i32) };
        }
        #[cfg(target_os = "freebsd")]
        {
            unsafe { lseek((*state).fd, 0i64, 2i32) };
        }
        // so gzoffset() is correct
        unsafe {
            (*state).mode = 31153;
        }
        // simplify later checks
    }
    // save the current position for rewinding (only if reading)
    if (unsafe { (*state).mode }) == 7247 {
        unsafe {
            #[cfg(windows)]
            {
                (*state).start = _lseeki64((*state).fd, 0i64, 1i32);
            }
            #[cfg(target_env = "gnu")]
            {
                (*state).start = lseek((*state).fd, (0 as c_long), 1i32) as off_t;
            }
            #[cfg(not(any(target_env = "gnu", target_os = "freebsd", windows)))]
            {
                (*state).start = lseek((*state).fd, (0 as off_t), 1i32);
            }
            #[cfg(target_os = "freebsd")]
            {
                (*state).start = lseek((*state).fd, 0i64, 1i32) as off_t;
            }
        }
        if (unsafe { (*state).start }) == -1 {
            unsafe {
                (*state).start = 0;
            }
        }
    }
    // initialize stream
    gz_reset(state);
    // return stream
    state as *mut gzFile_s
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzopen(path: *const c_char, mode: *const c_char) -> *mut gzFile_s {
    gz_open(path as *const (), -1i32, mode)
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzopen64(path: *const c_char, mode: *const c_char) -> *mut gzFile_s {
    gz_open(path as *const (), -1i32, mode)
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzdopen(fd: i32, mode: *const c_char) -> *mut gzFile_s {
    let mut path: *mut c_char = unsafe { std::mem::zeroed() }; // identifier for error messages
    let __v0: bool = if fd == -1 {
        true
    } else {
        #[cfg(all(target_pointer_width = "64", unix))]
        let __v1: *mut c_char =
            (unsafe {
                malloc((7 as c_ulong).wrapping_add(
                    (3 as c_ulong).wrapping_mul(std::mem::size_of::<i32>() as c_ulong),
                ))
            }) as *mut c_char;
        #[cfg(target_pointer_width = "32")]
        let __v1: *mut c_char = (unsafe {
            malloc(7u32.wrapping_add(3u32.wrapping_mul(std::mem::size_of::<i32>() as u32)))
        }) as *mut c_char;
        #[cfg(all(target_pointer_width = "64", windows))]
        let __v1: *mut c_char = (unsafe {
            malloc(7u64.wrapping_add(3u64.wrapping_mul(std::mem::size_of::<i32>() as u64)))
        }) as *mut c_char;
        path = __v1;
        __v1.is_null()
    };
    if __v0 {
        return std::ptr::null_mut::<gzFile_s>();
    }
    #[cfg(all(unix, feature = "no-vsnprintf"))]
    {
        unsafe { sprintf(path, c"<fd:%d>".as_ptr(), fd) };
    }
    #[cfg(all(windows, feature = "no-vsnprintf"))]
    {
        unsafe { sprintf(path, c"<fd:%d>".as_ptr(), fd) };
    }
    #[cfg(all(target_pointer_width = "64", unix, not(feature = "no-vsnprintf")))]
    {
        unsafe {
            snprintf(
                path,
                (7 as c_ulong).wrapping_add(
                    (3 as c_ulong).wrapping_mul(std::mem::size_of::<i32>() as c_ulong),
                ),
                c"<fd:%d>".as_ptr(),
                fd,
            )
        };
    }
    #[cfg(all(target_pointer_width = "32", unix, not(feature = "no-vsnprintf")))]
    {
        unsafe {
            snprintf(
                path,
                7u32.wrapping_add(3u32.wrapping_mul(std::mem::size_of::<i32>() as u32)),
                c"<fd:%d>".as_ptr(),
                fd,
            )
        };
    }
    #[cfg(all(target_pointer_width = "64", windows, not(feature = "no-vsnprintf")))]
    {
        unsafe {
            snprintf(
                path,
                7u64.wrapping_add(3u64.wrapping_mul(std::mem::size_of::<i32>() as u64)),
                c"<fd:%d>".as_ptr(),
                fd,
            )
        };
    }
    #[cfg(all(target_pointer_width = "32", windows, not(feature = "no-vsnprintf")))]
    {
        unsafe {
            snprintf(
                path,
                7u32.wrapping_add(3u32.wrapping_mul(std::mem::size_of::<i32>() as u32)),
                c"<fd:%d>".as_ptr(),
                fd,
            )
        };
    }
    let gz: *mut gzFile_s = gz_open(path as *const (), fd, mode);
    unsafe { free(path as *mut ()) };
    gz
}

#[cfg(windows)]
/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzopen_w(path: *const wchar_t, mode: *const c_char) -> *mut gzFile_s {
    gz_open(path as *const (), -2i32, mode)
}

#[cfg(unix)]
/// -- see zlib.h --
/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzbuffer(file: *mut gzFile_s, mut size: u32) -> i32 {
    // get internal structure and check integrity
    if file.is_null() {
        return -1;
    }
    if (unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).mode }) != 7247
        && (unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).mode }) != 31153
    {
        return -1;
    }
    // make sure we haven't already allocated memory
    if (unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).size }) != 0 {
        return -1;
    }
    // check and set requested size
    if size << 1i32 < size {
        return -1;
    }
    // need to be able to double it
    if size < 8 {
        size = 8;
    }
    // needed to behave well with flushing
    unsafe {
        (*(file as *mut __SlateRecord_gzguts_h_203)).want = size;
    }
    0
}

#[cfg(windows)]
/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzbuffer(file: *mut gzFile_s, mut size: u32) -> i32 {
    // get internal structure and check integrity
    if file.is_null() {
        return -1;
    }
    if (unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).mode }) != 7247
        && (unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).mode }) != 31153
    {
        return -1;
    }
    // make sure we haven't already allocated memory
    if (unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).size }) != 0 {
        return -1;
    }
    // check and set requested size
    if size << 1i32 < size {
        return -1;
    }
    // need to be able to double it
    if size < 8 {
        size = 8;
    }
    // needed to behave well with flushing
    unsafe {
        (*(file as *mut __SlateRecord_gzguts_h_203)).want = size;
    }
    0
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzrewind(file: *mut gzFile_s) -> i32 {
    // get internal structure
    if file.is_null() {
        return -1;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    // check that we're reading and that there's no error
    if (unsafe { (*state).mode }) != 7247
        || (unsafe { (*state).err }) != 0 && (unsafe { (*state).err }) != -5
    {
        return -1;
    }
    // back up and start over
    #[cfg(target_env = "gnu")]
    {
        if (unsafe { lseek((*state).fd, (*state).start as c_long, 0i32) }) == -1 {
            return -1;
        }
    }
    #[cfg(not(any(target_env = "gnu", target_os = "freebsd", windows)))]
    {
        if (unsafe { lseek((*state).fd, (*state).start, 0i32) }) == -1 {
            return -1;
        }
    }
    #[cfg(target_os = "freebsd")]
    {
        if (unsafe { lseek((*state).fd, (*state).start as i64, 0i32) }) == -1 {
            return -1;
        }
    }
    #[cfg(windows)]
    {
        if (unsafe { _lseeki64((*state).fd, (*state).start, 0i32) }) == -1 {
            return -1;
        }
    }
    gz_reset(state);
    0
}

#[cfg(unix)]
type __SlateRet_gzseek64 = off_t;

#[cfg(windows)]
type __SlateRet_gzseek64 = i64;

#[cfg(unix)]
type __SlateParam_gzseek64_1 = off_t;

#[cfg(windows)]
type __SlateParam_gzseek64_1 = i64;

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzseek64(
    file: *mut gzFile_s,
    mut offset: __SlateParam_gzseek64_1,
    whence: i32,
) -> __SlateRet_gzseek64 {
    let mut n: u32 = 0;
    #[cfg(unix)]
    let mut ret: off_t = 0;
    #[cfg(windows)]
    let mut ret: i64 = 0;
    // get internal structure and check integrity
    if file.is_null() {
        return -1;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    if (unsafe { (*state).mode }) != 7247 && (unsafe { (*state).mode }) != 31153 {
        return -1;
    }
    // check that there's no error
    if (unsafe { (*state).err }) != 0 && (unsafe { (*state).err }) != -5 {
        return -1;
    }
    // can only seek from start or relative to current position
    if whence != 0 && whence != 1 {
        return -1;
    }
    // normalize offset to a SEEK_CUR specification
    if whence == 0 {
        offset -= unsafe { (*state).x.pos };
    } else {
        offset += if (unsafe { (*state).past }) != 0 {
            0
        } else {
            unsafe { (*state).skip }
        };
        unsafe {
            (*state).skip = 0;
        }
    }
    // if within raw area while reading, just go there
    if (unsafe { (*state).mode }) == 7247
        && (unsafe { (*state).how }) == 1
        && (unsafe { (*state).x.pos }) + offset >= 0
    {
        #[cfg(target_env = "gnu")]
        {
            ret = (unsafe {
                lseek(
                    (*state).fd,
                    (offset - ((*state).x.have as off_t)) as c_long,
                    1i32,
                )
            }) as off_t;
        }
        #[cfg(not(any(target_env = "gnu", target_os = "freebsd", windows)))]
        {
            ret = unsafe { lseek((*state).fd, offset - ((*state).x.have as off_t), 1i32) };
        }
        #[cfg(target_os = "freebsd")]
        {
            ret = (unsafe {
                lseek(
                    (*state).fd,
                    (offset - ((*state).x.have as off_t)) as i64,
                    1i32,
                )
            }) as off_t;
        }
        #[cfg(windows)]
        {
            ret = unsafe {
                _lseeki64(
                    (*state).fd,
                    offset - (((*state).x.have as u64) as i64),
                    1i32,
                )
            };
        }
        if ret == -1 {
            return -1;
        }
        unsafe {
            (*state).x.have = 0;
        }
        unsafe {
            (*state).eof = 0;
        }
        unsafe {
            (*state).past = 0;
        }
        unsafe {
            (*state).skip = 0;
        }
        gz_error(state, 0i32, std::ptr::null::<c_char>());
        unsafe {
            (*state).strm.avail_in = 0;
        }
        unsafe {
            (*state).x.pos = (*state).x.pos + offset;
        }
        return unsafe { (*state).x.pos };
    }
    // calculate skip amount, rewinding if needed for back seek when reading
    if offset < 0 {
        if (unsafe { (*state).mode }) != 7247 {
            return -1;
        }
        // writing -- can't go backwards
        offset += unsafe { (*state).x.pos };
        if offset < 0 {
            return -1;
        }
        // before start of file!
        if gzrewind(file) == -1 {
            return -1;
        }
        // rewind, then skip to offset
    }
    // if reading, skip what's in output buffer (one less gzgetc() check)
    if (unsafe { (*state).mode }) == 7247 {
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            n = if GT_OFF__6!(unsafe { (*state).x.have })
                || ((unsafe { (*state).x.have }) as off_t) > offset
            {
                (offset as i32) as u32
            } else {
                unsafe { (*state).x.have }
            };
        }
        #[cfg(all(target_pointer_width = "32", unix))]
        {
            n = if GT_OFF__7!(unsafe { (*state).x.have })
                || ((unsafe { (*state).x.have }) as off_t) > offset
            {
                offset as u32
            } else {
                unsafe { (*state).x.have }
            };
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            n = if GT_OFF__5!(unsafe { (*state).x.have })
                || (((unsafe { (*state).x.have }) as u64) as i64) > offset
            {
                (offset as i32) as u32
            } else {
                unsafe { (*state).x.have }
            };
        }
        #[cfg(all(target_pointer_width = "32", windows))]
        {
            n = if GT_OFF__8!(unsafe { (*state).x.have })
                || (((unsafe { (*state).x.have }) as u64) as i64) > offset
            {
                (offset as i32) as u32
            } else {
                unsafe { (*state).x.have }
            };
        }
        let __v11: u32 = unsafe { (*state).x.have };
        unsafe {
            (*state).x.have = __v11.wrapping_sub(n);
        }
        let __v14: *mut u8 = unsafe { (*state).x.next };
        unsafe {
            (*state).x.next = __v14.add(n as usize);
        }
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
                (*state).x.pos = (*state).x.pos + ((n as u64) as i64);
            }
        }
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            offset -= n as off_t;
        }
        #[cfg(all(target_pointer_width = "32", unix))]
        {
            offset = (offset as c_ulong).wrapping_sub(n as c_ulong) as off_t;
        }
        #[cfg(windows)]
        {
            offset -= (n as u64) as i64;
        }
    }
    // request skip (if not zero)
    unsafe {
        (*state).skip = offset;
    }
    (unsafe { (*state).x.pos }) + offset
}

#[cfg(unix)]
type __SlateHoist_gzseek = off_t;

#[cfg(windows)]
type __SlateHoist_gzseek = i64;

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzseek(
    file: *mut gzFile_s,
    offset: __SlateHoist_gzseek,
    whence: i32,
) -> __SlateHoist_gzseek {
    let ret: __SlateHoist_gzseek = gzseek64(file, offset, whence);
    if ret == ret { ret } else { -1 }
}

#[cfg(unix)]
type __SlateHoist_gztell64 = off_t;

#[cfg(windows)]
type __SlateHoist_gztell64 = i64;

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gztell64(file: *mut gzFile_s) -> __SlateHoist_gztell64 {
    // get internal structure and check integrity
    if file.is_null() {
        return -1;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    if (unsafe { (*state).mode }) != 7247 && (unsafe { (*state).mode }) != 31153 {
        return -1;
    }
    // return position
    (unsafe { (*state).x.pos })
        + if (unsafe { (*state).past }) != 0 {
            0
        } else {
            unsafe { (*state).skip }
        }
}

#[cfg(unix)]
type __SlateHoist_gztell = off_t;

#[cfg(windows)]
type __SlateHoist_gztell = i64;

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gztell(file: *mut gzFile_s) -> __SlateHoist_gztell {
    let ret: __SlateHoist_gztell = gztell64(file);
    if ret == ret { ret } else { -1 }
}

#[cfg(unix)]
type __SlateRet_gzoffset64 = off_t;

#[cfg(windows)]
type __SlateRet_gzoffset64 = i64;

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzoffset64(file: *mut gzFile_s) -> __SlateRet_gzoffset64 {
    #[cfg(unix)]
    let mut offset: off_t = 0;
    #[cfg(windows)]
    let mut offset: i64 = 0;
    // get internal structure and check integrity
    if file.is_null() {
        return -1;
    }
    if (unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).mode }) != 7247
        && (unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).mode }) != 31153
    {
        return -1;
    }
    // compute and return effective offset in file
    #[cfg(target_env = "gnu")]
    {
        offset = (unsafe {
            lseek(
                (*(file as *mut __SlateRecord_gzguts_h_203)).fd,
                (0 as c_long),
                1i32,
            )
        }) as off_t;
    }
    #[cfg(not(any(target_env = "gnu", target_os = "freebsd", windows)))]
    {
        offset = unsafe {
            lseek(
                (*(file as *mut __SlateRecord_gzguts_h_203)).fd,
                (0 as off_t),
                1i32,
            )
        };
    }
    #[cfg(target_os = "freebsd")]
    {
        offset = (unsafe { lseek((*(file as *mut __SlateRecord_gzguts_h_203)).fd, 0i64, 1i32) })
            as off_t;
    }
    #[cfg(windows)]
    {
        offset = unsafe { _lseeki64((*(file as *mut __SlateRecord_gzguts_h_203)).fd, 0i64, 1i32) };
    }
    if offset == -1 {
        return -1;
    }
    if (unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).mode }) == 7247 {
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            offset -=
                (unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).strm.avail_in }) as off_t;
        }
        #[cfg(all(target_pointer_width = "32", unix))]
        {
            offset = (offset as c_ulong).wrapping_sub(
                (unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).strm.avail_in }) as c_ulong,
            ) as off_t;
        }
        #[cfg(windows)]
        {
            offset -= ((unsafe { (*(file as *mut __SlateRecord_gzguts_h_203)).strm.avail_in })
                as u64) as i64;
        }
    }
    // reading
    // don't count buffered input
    offset
}

#[cfg(unix)]
type __SlateHoist_gzoffset = off_t;

#[cfg(windows)]
type __SlateHoist_gzoffset = i64;

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzoffset(file: *mut gzFile_s) -> __SlateHoist_gzoffset {
    let ret: __SlateHoist_gzoffset = gzoffset64(file);
    if ret == ret { ret } else { -1 }
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzeof(file: *mut gzFile_s) -> i32 {
    // get internal structure and check integrity
    if file.is_null() {
        return 0;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    if (unsafe { (*state).mode }) != 7247 && (unsafe { (*state).mode }) != 31153 {
        return 0;
    }
    // return end-of-file state
    if (unsafe { (*state).mode }) == 7247 {
        unsafe { (*state).past }
    } else {
        0
    }
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzerror(file: *mut gzFile_s, errnum: *mut i32) -> *const c_char {
    // get internal structure and check integrity
    if file.is_null() {
        return std::ptr::null::<c_char>();
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    if (unsafe { (*state).mode }) != 7247 && (unsafe { (*state).mode }) != 31153 {
        return std::ptr::null::<c_char>();
    }
    // return error information
    if !errnum.is_null() {
        unsafe {
            *errnum = (*state).err;
        }
    }
    (if (unsafe { (*state).err }) == -4 {
        c"out of memory".as_ptr() as *mut c_char
    } else {
        if unsafe { (*state).msg }.is_null() {
            c"".as_ptr() as *mut c_char
        } else {
            unsafe { (*state).msg }
        }
    }) as *const c_char
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzclearerr(file: *mut gzFile_s) {
    // get internal structure and check integrity
    if file.is_null() {
        return;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    if (unsafe { (*state).mode }) != 7247 && (unsafe { (*state).mode }) != 31153 {
        return;
    }
    // clear error and end-of-file
    if (unsafe { (*state).mode }) == 7247 {
        unsafe {
            (*state).eof = 0;
        }
        unsafe {
            (*state).past = 0;
        }
    }
    gz_error(state, 0i32, std::ptr::null::<c_char>());
}

/// Create an error message in allocated memory and set state->err and
/// state->msg accordingly.  Free any previous error message already there.  Do
/// not try to free or allocate space if the error is Z_MEM_ERROR (out of
/// memory).  Simply save the error message as a static string.  If there is an
/// allocation failure constructing the error message, then convert the error to
/// out of memory.
#[unsafe(no_mangle)]
extern "C-unwind" fn gz_error(
    state: *mut __SlateRecord_gzguts_h_203,
    err: i32,
    msg: *const c_char,
) {
    // free previously allocated message and clear
    if !unsafe { (*state).msg }.is_null() {
        if (unsafe { (*state).err }) != -4 {
            unsafe { free((*state).msg as *mut ()) };
        }
        unsafe {
            (*state).msg = std::ptr::null_mut::<c_char>();
        }
    }
    // if fatal, set state->x.have to 0 so that the gzgetc() macro fails
    if err != 0 && err != -5 && (unsafe { (*state).again }) == 0 {
        unsafe {
            (*state).x.have = 0;
        }
    }
    // set error code, and if no message, then done
    unsafe {
        (*state).err = err;
    }
    if msg.is_null() {
        return;
    }
    // for an out of memory error, return literal string when requested
    if err == -4 {
        return;
    }
    // construct error message with path
    let __v0: *mut c_char = (unsafe {
        malloc(
            strlen((*state).path as *const c_char)
                .wrapping_add(strlen(msg))
                .wrapping_add(3),
        )
    }) as *mut c_char;
    unsafe {
        (*state).msg = __v0;
    }
    if __v0.is_null() {
        unsafe {
            (*state).err = -4;
        }
        return;
    }
    #[cfg(feature = "no-vsnprintf")]
    {
        unsafe { strcpy((*state).msg, (*state).path as *const c_char) };
        unsafe { strcat((*state).msg, c": ".as_ptr()) };
        unsafe { strcat((*state).msg, msg) };
    }
    #[cfg(all(unix, not(feature = "no-vsnprintf")))]
    {
        unsafe {
            snprintf(
                (*state).msg,
                strlen((*state).path as *const c_char)
                    .wrapping_add(strlen(msg))
                    .wrapping_add(3),
                c"%s%s%s".as_ptr(),
                (*state).path,
                c": ".as_ptr() as *mut c_char,
                msg,
            )
        };
    }
    #[cfg(all(windows, not(feature = "no-vsnprintf")))]
    {
        unsafe {
            snprintf(
                (*state).msg,
                strlen((*state).path as *const c_char)
                    .wrapping_add(strlen(msg))
                    .wrapping_add(3),
                c"%s%s%s".as_ptr(),
                (*state).path,
                c": ".as_ptr() as *mut c_char,
                msg,
            )
        };
    }
}

/// portably return maximum value for an int (when limits.h presumed not
/// available) -- we need to do this to cover cases where 2's complement not
/// used, since C standard permits 1's complement and sign-bit representations,
/// otherwise we could just use ((unsigned)-1) >> 1
#[unsafe(no_mangle)]
extern "C-unwind" fn gz_intmax() -> u32 {
    2147483647
}
