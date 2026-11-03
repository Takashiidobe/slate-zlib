//! gzread.c -- zlib functions for reading gzip files
//! Copyright (C) 2004-2026 Mark Adler
//! For conditions of distribution and use, see copyright notice in zlib.h
#[cfg(all(target_pointer_width = "64", windows))]
macro_rules! inflateInit2 {
    ($strm:expr, $windowBits:expr) => {
        unsafe {
            inflateInit2_(
                $strm,
                $windowBits,
                c"1.3.2.1-motley".as_ptr(),
                std::mem::size_of::<z_stream_s>() as i32,
            )
        }
    };
}

#[cfg(target_pointer_width = "32")]
macro_rules! inflateInit2__2 {
    ($strm:expr, $windowBits:expr) => {
        unsafe {
            inflateInit2_(
                $strm,
                $windowBits,
                c"1.3.2.1-motley".as_ptr(),
                std::mem::size_of::<z_stream_s>() as i32,
            )
        }
    };
}

#[cfg(all(target_pointer_width = "64", unix))]
macro_rules! inflateInit2__3 {
    ($strm:expr, $windowBits:expr) => {
        unsafe {
            inflateInit2_(
                $strm,
                $windowBits,
                c"1.3.2.1-motley".as_ptr(),
                std::mem::size_of::<z_stream_s>() as i32,
            )
        }
    };
}

unsafe extern "C" {
    fn close(arg0: i32) -> i32;
    fn inflate(arg0: *mut z_stream_s, arg1: i32) -> i32;
    fn inflateEnd(arg0: *mut z_stream_s) -> i32;
    fn inflateReset(arg0: *mut z_stream_s) -> i32;
    fn inflateInit2_(arg0: *mut z_stream_s, arg1: i32, arg2: *const c_char, arg3: i32) -> i32;
    fn memcpy(arg0: *mut (), arg1: *const (), arg2: size_t) -> *mut ();
    fn memchr(arg0: *const (), arg1: i32, arg2: size_t) -> *mut ();
    fn strerror(arg0: i32) -> *mut c_char;
    fn malloc(arg0: size_t) -> *mut ();
    fn free(arg0: *mut ());
    fn gz_error(arg0: *mut __SlateRecord_gzguts_h_203, arg1: i32, arg2: *const c_char);
    fn gz_intmax() -> u32;
}

#[cfg(unix)]
unsafe extern "C" {
    fn read(arg0: i32, arg1: *mut (), arg2: size_t) -> ssize_t;
}

#[cfg(windows)]
unsafe extern "C" {
    fn read(arg0: i32, arg1: *mut (), arg2: u32) -> i32;
}

use core::ffi::c_char;
#[cfg(all(target_pointer_width = "64", unix))]
use core::ffi::c_long;
use core::ffi::c_ulong;
#[cfg(unix)]
use slate_support::off_t;
use slate_support::size_t;
#[cfg(unix)]
use slate_support::ssize_t;

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

/// Use read() to load a buffer -- return -1 on error, otherwise 0.  Read from
/// state->fd, and update state->eof, state->err, and state->msg as appropriate.
/// This function needs to loop on read(), since read() is not guaranteed to
/// read the number of bytes requested, depending on the type of descriptor. It
/// also needs to loop to manage the fact that read() returns an int. If the
/// descriptor is non-blocking and read() returns with no data in order to avoid
/// blocking, then gz_load() will return 0 if some data has been read, or -1 if
/// no data has been read. Either way, state->again is set true to indicate a
/// non-blocking event. If errno is non-zero on return, then there was an error
/// signaled from read().  *have is set to the number of bytes read.
fn gz_load(state: *mut __SlateRecord_gzguts_h_203, buf: *mut u8, len: u32, have: *mut u32) -> i32 {
    let mut ret: i32 = 0;
    let mut get: u32 = 0;
    let max: u32 = 1073741824u32;
    unsafe {
        (*state).again = 0;
    }
    unsafe {
        *slate_support::errno_location() = 0;
    }
    unsafe {
        *have = 0;
    }
    loop {
        get = len.wrapping_sub(unsafe { *have });
        if get > max {
            get = max;
        }
        #[cfg(unix)]
        {
            ret = (unsafe {
                read(
                    (*state).fd,
                    buf.offset(*have as isize) as *mut (),
                    get as size_t,
                )
            }) as i32;
        }
        #[cfg(windows)]
        {
            ret = unsafe { read((*state).fd, buf.offset(*have as isize) as *mut (), get) };
        }
        if ret <= 0 {
            break;
        }
        let __v2: u32 = unsafe { *have };
        unsafe {
            *have = __v2.wrapping_add(ret as u32);
        }
        if (unsafe { *have }) >= len {
            break;
        }
    }
    if ret < 0 {
        #[cfg(any(target_os = "macos", target_os = "freebsd"))]
        let __v4: bool = (unsafe { *slate_support::errno_location() }) == 35
            || (unsafe { *slate_support::errno_location() }) == 35;
        #[cfg(windows)]
        let __v4: bool = (unsafe { *slate_support::errno_location() }) == 11
            || (unsafe { *slate_support::errno_location() }) == 140;
        #[cfg(not(any(target_os = "linux", target_os = "android")))]
        {
            if __v4 {
                unsafe {
                    (*state).again = 1;
                }
                if (unsafe { *have }) != 0 {
                    return 0;
                }
            }
        }
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            if (unsafe { *slate_support::errno_location() }) == 11
                || (unsafe { *slate_support::errno_location() }) == 11
            {
                unsafe {
                    (*state).again = 1;
                }
                if (unsafe { *have }) != 0 {
                    return 0;
                }
            }
        }
        unsafe { gz_error(state, -1i32, zstrerror!() as *const c_char) };
        return -1;
    }
    if ret == 0 {
        unsafe {
            (*state).eof = 1;
        }
    }
    0
}

/// Load up input buffer and set eof flag if last data loaded -- return -1 on
/// error, 0 otherwise.  Note that the eof flag is set when the end of the input
/// file is reached, even though there may be unused data in the buffer.  Once
/// that data has been used, no more attempts will be made to read the file.
/// If strm->avail_in != 0, then the current data is moved to the beginning of
/// the input buffer, and then the remainder of the buffer is loaded with the
/// available data from the input file.
fn gz_avail(state: *mut __SlateRecord_gzguts_h_203) -> i32 {
    let mut got: u32 = 0;
    let strm: *mut z_stream_s = unsafe { std::ptr::addr_of_mut!((*state).strm) };
    if (unsafe { (*state).err }) != 0 && (unsafe { (*state).err }) != -5 {
        return -1;
    }
    if (unsafe { (*state).eof }) == 0 {
        if (unsafe { (*strm).avail_in }) != 0 {
            // copy what's there to the start
            let mut p: *mut u8 = unsafe { (*state).r#in };
            let mut q: *const u8 = (unsafe { (*strm).next_in }) as *const u8;
            if q != (p as *const u8) {
                let mut n: u32 = unsafe { (*strm).avail_in };
                loop {
                    let __v0: *const u8 = q;
                    q = unsafe { __v0.add(1) };
                    let __v2: *mut u8 = p;
                    p = unsafe { __v2.add(1) };
                    unsafe {
                        *__v2 = *__v0;
                    }
                    let __v6: u32 = n.wrapping_sub(1);
                    n = __v6;
                    if __v6 == 0 {
                        break;
                    }
                }
            }
        }
        if gz_load(
            state,
            unsafe { unsafe { (*state).r#in }.offset((*strm).avail_in as isize) },
            unsafe { (*state).size }.wrapping_sub(unsafe { (*strm).avail_in }),
            std::ptr::addr_of_mut!(got),
        ) == -1
        {
            return -1;
        }
        let __v8: u32 = unsafe { (*strm).avail_in };
        unsafe {
            (*strm).avail_in = __v8.wrapping_add(got);
        }
        unsafe {
            (*strm).next_in = (*state).r#in;
        }
    }
    0
}

/// Look for gzip header, set up for inflate or copy.  state->x.have must be 0.
/// If this is the first time in, allocate required memory.  state->how will be
/// left unchanged if there is no more input data available, will be set to COPY
/// if there is no gzip header and direct copying will be performed, or it will
/// be set to GZIP for decompression.  If direct copying, then leftover input
/// data from the input buffer will be copied to the output buffer.  In that
/// case, all further file reads will be directly to either the output buffer or
/// a user buffer.  If decompressing, the inflate state will be initialized.
/// gz_look() will return 0 on success or -1 on failure.
fn gz_look(state: *mut __SlateRecord_gzguts_h_203) -> i32 {
    let strm: *mut z_stream_s = unsafe { std::ptr::addr_of_mut!((*state).strm) };
    // allocate read buffers and inflate memory
    if (unsafe { (*state).size }) == 0 {
        // allocate buffers
        unsafe {
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                (*state).r#in = malloc((*state).want as c_ulong) as *mut u8;
            }
            #[cfg(target_pointer_width = "32")]
            {
                (*state).r#in = malloc((*state).want) as *mut u8;
            }
            #[cfg(all(target_pointer_width = "64", windows))]
            {
                (*state).r#in = malloc((*state).want as u64) as *mut u8;
            }
        }
        unsafe {
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                (*state).out = malloc(((*state).want << 1i32) as c_ulong) as *mut u8;
            }
            #[cfg(target_pointer_width = "32")]
            {
                (*state).out = malloc((*state).want << 1i32) as *mut u8;
            }
            #[cfg(all(target_pointer_width = "64", windows))]
            {
                (*state).out = malloc(((*state).want << 1i32) as u64) as *mut u8;
            }
        }
        if unsafe { (*state).r#in }.is_null() || unsafe { (*state).out }.is_null() {
            unsafe { free((*state).out as *mut ()) };
            unsafe { free((*state).r#in as *mut ()) };
            unsafe { gz_error(state, -4i32, c"out of memory".as_ptr()) };
            return -1;
        }
        unsafe {
            (*state).size = (*state).want;
        }
        // allocate inflate memory
        unsafe {
            (*state).strm.zalloc = None;
        }
        unsafe {
            (*state).strm.zfree = None;
        }
        unsafe {
            (*state).strm.opaque = std::ptr::null_mut::<()>();
        }
        unsafe {
            (*state).strm.avail_in = 0;
        }
        unsafe {
            (*state).strm.next_in = std::ptr::null_mut::<u8>();
        }
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            if inflateInit2__3!(unsafe { std::ptr::addr_of_mut!((*state).strm) }, 31i32) != 0 {
                // gunzip
                unsafe { free((*state).out as *mut ()) };
                unsafe { free((*state).r#in as *mut ()) };
                unsafe {
                    (*state).size = 0;
                }
                unsafe { gz_error(state, -4i32, c"out of memory".as_ptr()) };
                return -1;
            }
        }
        #[cfg(target_pointer_width = "32")]
        {
            if inflateInit2__2!(unsafe { std::ptr::addr_of_mut!((*state).strm) }, 31i32) != 0 {
                // gunzip
                unsafe { free((*state).out as *mut ()) };
                unsafe { free((*state).r#in as *mut ()) };
                unsafe {
                    (*state).size = 0;
                }
                unsafe { gz_error(state, -4i32, c"out of memory".as_ptr()) };
                return -1;
            }
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            if inflateInit2!(unsafe { std::ptr::addr_of_mut!((*state).strm) }, 31i32) != 0 {
                // gunzip
                unsafe { free((*state).out as *mut ()) };
                unsafe { free((*state).r#in as *mut ()) };
                unsafe {
                    (*state).size = 0;
                }
                unsafe { gz_error(state, -4i32, c"out of memory".as_ptr()) };
                return -1;
            }
        }
    }
    // if transparent reading is disabled, which would only be at the start, or
    // if we're looking for a gzip member after the first one, which is not at
    // the start, then proceed directly to look for a gzip member next
    if (unsafe { (*state).direct }) == -1 || (unsafe { (*state).junk }) == 0 {
        unsafe { inflateReset(strm) };
        unsafe {
            (*state).how = 2;
        }
        unsafe {
            (*state).junk = ((*state).junk != -1) as i32;
        }
        unsafe {
            (*state).direct = 0;
        }
        return 0;
    }
    // otherwise we're at the start with auto-detect -- we check to see if the
    // first four bytes could be gzip header in order to decide whether or not
    // this will be a transparent read
    // load any header bytes into the input buffer -- if the input is empty,
    // then it's not an error as this is a transparent read of zero bytes
    if gz_avail(state) == -1 {
        return -1;
    }
    if (unsafe { (*strm).avail_in }) == 0
        || (unsafe { (*state).again }) != 0 && (unsafe { (*strm).avail_in }) < 4
    {
        return 0;
    }
    // if non-blocking input stalled before getting four bytes, then
    // return and wait until a later call has accumulated enough
    // see if this is (likely) gzip input -- if the first four bytes are
    // consistent with a gzip header, then go look for the first gzip member,
    // otherwise proceed to copy the input transparently
    if (unsafe { (*strm).avail_in }) > 3
        && (((unsafe { *unsafe { (*strm).next_in }.add(0) }) as u32) as i32) == 31
        && (((unsafe { *unsafe { (*strm).next_in }.add(1) }) as u32) as i32) == 139
        && (((unsafe { *unsafe { (*strm).next_in }.add(2) }) as u32) as i32) == 8
        && (((unsafe { *unsafe { (*strm).next_in }.add(3) }) as u32) as i32) < 32
    {
        unsafe { inflateReset(strm) };
        unsafe {
            (*state).how = 2;
        }
        unsafe {
            (*state).junk = 1;
        }
        unsafe {
            (*state).direct = 0;
        }
        return 0;
    }
    // doing raw i/o: copy any leftover input to output -- this assumes that
    // the output buffer is larger than the input buffer, which also assures
    // space for gzungetc()
    unsafe {
        (*state).x.next = (*state).out;
    }
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*strm).next_in as *const ()) as *const u8,
                ((*state).x.next as *mut ()) as *mut u8,
                ((*strm).avail_in as c_ulong) as usize,
            )
        };
    }
    #[cfg(target_pointer_width = "32")]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*strm).next_in as *const ()) as *const u8,
                ((*state).x.next as *mut ()) as *mut u8,
                (*strm).avail_in as usize,
            )
        };
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*strm).next_in as *const ()) as *const u8,
                ((*state).x.next as *mut ()) as *mut u8,
                ((*strm).avail_in as u64) as usize,
            )
        };
    }
    unsafe {
        (*state).x.have = (*strm).avail_in;
    }
    unsafe {
        (*strm).avail_in = 0;
    }
    unsafe {
        (*state).how = 1;
    }
    0
}

/// Decompress from input to the provided next_out and avail_out in the state.
/// On return, state->x.have and state->x.next point to the just decompressed
/// data. If the gzip stream completes, state->how is reset to LOOK to look for
/// the next gzip stream or raw data, once state->x.have is depleted. Returns 0
/// on success, -1 on failure. If EOF is reached when looking for more input to
/// complete the gzip member, then an unexpected end of file error is raised.
/// If there is no more input, but state->again is true, then EOF has not been
/// reached, and no error is raised.
fn gz_decomp(state: *mut __SlateRecord_gzguts_h_203) -> i32 {
    let mut ret: i32 = 0;
    let mut had: u32 = 0;
    let strm: *mut z_stream_s = unsafe { std::ptr::addr_of_mut!((*state).strm) };
    // fill output buffer up to end of deflate stream
    had = unsafe { (*strm).avail_out };
    loop {
        // get more input for inflate()
        let __v0: bool = (unsafe { (*strm).avail_in }) == 0 && gz_avail(state) == -1;
        if __v0 {
            ret = unsafe { (*state).err };
            break;
        }
        if (unsafe { (*strm).avail_in }) == 0 {
            if (unsafe { (*state).again }) == 0 {
                unsafe { gz_error(state, -5i32, c"unexpected end of file".as_ptr()) };
            }
            break;
        }
        // decompress and handle errors
        ret = unsafe { inflate(strm, 0i32) };
        if (unsafe { (*strm).avail_out }) < had {
            unsafe {
                (*state).junk = 0;
            }
        }
        // any decompressed data marks this as a real gzip stream
        if ret == -2 || ret == 2 {
            unsafe {
                gz_error(
                    state,
                    -2i32,
                    c"internal error: inflate stream corrupt".as_ptr(),
                )
            };
            break;
        }
        if ret == -4 {
            unsafe { gz_error(state, -4i32, c"out of memory".as_ptr()) };
            break;
        }
        if ret == -3 {
            // deflate stream invalid
            if (unsafe { (*state).junk }) == 1 {
                // trailing garbage is ok
                unsafe {
                    (*strm).avail_in = 0;
                }
                unsafe {
                    (*state).eof = 1;
                }
                unsafe {
                    (*state).how = 0;
                }
                ret = 0;
                break;
            }
            unsafe {
                gz_error(
                    state,
                    -3i32,
                    (if unsafe { (*strm).msg }.is_null() {
                        c"compressed data error".as_ptr() as *mut c_char
                    } else {
                        (*strm).msg
                    }) as *const c_char,
                )
            };
            break;
        }
        if !((unsafe { (*strm).avail_out }) != 0 && ret != 1) {
            break;
        }
    }
    // update available output
    unsafe {
        (*state).x.have = had.wrapping_sub((*strm).avail_out);
    }
    unsafe {
        (*state).x.next = unsafe { (*strm).next_out }.offset(-((*state).x.have as isize));
    }
    // if the gzip stream completed successfully, look for another
    if ret == 1 {
        unsafe {
            (*state).junk = 0;
        }
        unsafe {
            (*state).how = 0;
        }
        return 0;
    }
    // return decompression status
    if ret != 0 { -1 } else { 0 }
}

/// Fetch data and put it in the output buffer.  Assumes state->x.have is 0.
/// Data is either copied from the input file or decompressed from the input
/// file depending on state->how.  If state->how is LOOK, then a gzip header is
/// looked for to determine whether to copy or decompress.  Returns -1 on error,
/// otherwise 0.  gz_fetch() will leave state->how as COPY or GZIP unless the
/// end of the input file has been reached and all data has been processed.
fn gz_fetch(state: *mut __SlateRecord_gzguts_h_203) -> i32 {
    let strm: *mut z_stream_s = unsafe { std::ptr::addr_of_mut!((*state).strm) };
    loop {
        match unsafe { (*state).how } {
            0 => {
                if gz_look(state) == -1 {
                    return -1;
                }
                // -> LOOK, COPY (only if never GZIP), or GZIP
                if (unsafe { (*state).how }) == 0 {
                    return 0;
                }
            }
            1 => {
                if gz_load(
                    state,
                    unsafe { (*state).out },
                    (unsafe { (*state).size }) << 1i32,
                    unsafe { std::ptr::addr_of_mut!((*state).x.have) },
                ) == -1
                {
                    return -1;
                }
                // -> COPY
                unsafe {
                    (*state).x.next = (*state).out;
                }
                return 0;
            }
            2 => {
                unsafe {
                    (*strm).avail_out = (*state).size << 1i32;
                }
                // -> GZIP or LOOK (if end of gzip stream)
                unsafe {
                    (*strm).next_out = (*state).out;
                }
                if gz_decomp(state) == -1 {
                    return -1;
                }
            }
            _ => {
                unsafe { gz_error(state, -2i32, c"state corrupt".as_ptr()) };
                return -1;
            }
        }
        if !((unsafe { (*state).x.have }) == 0
            && ((unsafe { (*state).eof }) == 0 || (unsafe { (*strm).avail_in }) != 0))
        {
            break;
        }
    }
    0
}

/// Skip state->skip (> 0) uncompressed bytes of output.  Return -1 on error, 0
/// on success.
fn gz_skip(state: *mut __SlateRecord_gzguts_h_203) -> i32 {
    let mut n: u32 = 0;
    // skip over len bytes or reach end-of-file, whichever comes first
    loop {
        // skip over whatever is in output buffer
        if (unsafe { (*state).x.have }) != 0 {
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                n = if GT_OFF__3!(unsafe { (*state).x.have })
                    || ((unsafe { (*state).x.have }) as off_t) > unsafe { (*state).skip }
                {
                    ((unsafe { (*state).skip }) as i32) as u32
                } else {
                    unsafe { (*state).x.have }
                };
            }
            #[cfg(all(target_pointer_width = "32", unix))]
            {
                n = if GT_OFF__2!(unsafe { (*state).x.have })
                    || ((unsafe { (*state).x.have }) as off_t) > unsafe { (*state).skip }
                {
                    (unsafe { (*state).skip }) as u32
                } else {
                    unsafe { (*state).x.have }
                };
            }
            #[cfg(all(target_pointer_width = "64", windows))]
            {
                n = if GT_OFF__4!(unsafe { (*state).x.have })
                    || (((unsafe { (*state).x.have }) as u64) as i64) > unsafe { (*state).skip }
                {
                    ((unsafe { (*state).skip }) as i32) as u32
                } else {
                    unsafe { (*state).x.have }
                };
            }
            #[cfg(all(target_pointer_width = "32", windows))]
            {
                n = if GT_OFF!(unsafe { (*state).x.have })
                    || (((unsafe { (*state).x.have }) as u64) as i64) > unsafe { (*state).skip }
                {
                    ((unsafe { (*state).skip }) as i32) as u32
                } else {
                    unsafe { (*state).x.have }
                };
            }
            let __v2: u32 = unsafe { (*state).x.have };
            unsafe {
                (*state).x.have = __v2.wrapping_sub(n);
            }
            let __v5: *mut u8 = unsafe { (*state).x.next };
            unsafe {
                (*state).x.next = __v5.add(n as usize);
            }
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
                    (*state).x.pos = (*state).x.pos + ((n as u64) as i64);
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
                    (*state).skip = (*state).skip - ((n as u64) as i64);
                }
            }
        } else {
            if (unsafe { (*state).eof }) != 0 && (unsafe { (*state).strm.avail_in }) == 0 {
                break;
            } else {
                // get more output, looking for header if required
                if gz_fetch(state) == -1 {
                    return -1;
                }
            }
        }
        // output buffer empty -- return if we're at the end of the input
        // need more data to skip -- load up output buffer
        if (unsafe { (*state).skip }) == 0 {
            break;
        }
    }
    0
}

/// Read len bytes into buf from file, or less than len up to the end of the
/// input. Return the number of bytes read. If zero is returned, either the end
/// of file was reached, or there was an error. state->err must be consulted in
/// that case to determine which. If there was an error, but some uncompressed
/// bytes were read before the error, then that count is returned. The error is
/// still recorded, and so is deferred until the next call.
fn gz_read(state: *mut __SlateRecord_gzguts_h_203, mut buf: *mut (), mut len: size_t) -> size_t {
    // get len bytes to buf, or less than len if at the end
    let mut got: size_t = 0;
    let mut n: u32 = 0;
    let mut err: i32 = 0;
    // if len is zero, avoid unnecessary operations
    #[cfg(target_pointer_width = "64")]
    {
        if len == 0 {
            return 0;
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        if (len as u32) == 0 {
            return 0;
        }
    }
    // process a skip request
    let __v0: bool = (unsafe { (*state).skip }) != 0 && gz_skip(state) == -1;
    if __v0 {
        return 0;
    }
    '__slate_break_7: loop {
        '__slate_continue_7: {
            // set n to the maximum amount of len that fits in an unsigned int
            n = 4294967295u32;
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
            // first just try copying data from the output buffer
            if (unsafe { (*state).x.have }) != 0 {
                if (unsafe { (*state).x.have }) < n {
                    n = unsafe { (*state).x.have };
                }
                #[cfg(all(target_pointer_width = "64", unix))]
                {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            ((*state).x.next as *const ()) as *const u8,
                            buf as *mut u8,
                            (n as c_ulong) as usize,
                        )
                    };
                }
                #[cfg(target_pointer_width = "32")]
                {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            ((*state).x.next as *const ()) as *const u8,
                            buf as *mut u8,
                            n as usize,
                        )
                    };
                }
                #[cfg(all(target_pointer_width = "64", windows))]
                {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            ((*state).x.next as *const ()) as *const u8,
                            buf as *mut u8,
                            (n as u64) as usize,
                        )
                    };
                }
                let __v2: *mut u8 = unsafe { (*state).x.next };
                unsafe {
                    (*state).x.next = __v2.add(n as usize);
                }
                let __v5: u32 = unsafe { (*state).x.have };
                unsafe {
                    (*state).x.have = __v5.wrapping_sub(n);
                }
                if (unsafe { (*state).err }) != 0 {
                    err = -1;
                }
            // caught deferred error from gz_fetch()
            } else {
                if (unsafe { (*state).eof }) != 0 && (unsafe { (*state).strm.avail_in }) == 0 {
                    break '__slate_break_7;
                } else {
                    if (unsafe { (*state).how }) == 0 || n < (unsafe { (*state).size }) << 1i32 {
                        // get more output, looking for header if required
                        if gz_fetch(state) == -1 && (unsafe { (*state).x.have }) == 0 {
                            err = -1;
                        }
                        // if state->x.have != 0, error will be caught after copy
                        break '__slate_continue_7; // no progress yet -- go back to copy above
                        // the copy above assures that we will leave with space in the
                        // output buffer, allowing at least one gzungetc() to succeed
                    } else {
                        if (unsafe { (*state).how }) == 1 {
                            err = gz_load(state, buf as *mut u8, n, std::ptr::addr_of_mut!(n));
                        } else {
                            // state->how == GZIP
                            unsafe {
                                (*state).strm.avail_out = n;
                            }
                            unsafe {
                                (*state).strm.next_out = buf as *mut u8;
                            }
                            err = gz_decomp(state);
                            n = unsafe { (*state).x.have };
                            unsafe {
                                (*state).x.have = 0;
                            }
                        }
                    }
                }
            }
            // output buffer empty -- return if we're at the end of the input
            // need output data -- for small len or new stream load up our output
            // buffer, so that gzgetc() can be fast
            // large len -- read directly into user buffer
            // read directly
            // large len -- decompress directly into user buffer
            // update progress
            #[cfg(target_pointer_width = "64")]
            {
                len = len.wrapping_sub(n as size_t);
            }
            #[cfg(target_pointer_width = "32")]
            {
                len = (len as u32).wrapping_sub(n) as size_t;
            }
            buf = (unsafe { (buf as *mut c_char).add(n as usize) }) as *mut ();
            #[cfg(target_pointer_width = "64")]
            {
                got = got.wrapping_add(n as size_t);
            }
            #[cfg(target_pointer_width = "32")]
            {
                got = (got as u32).wrapping_add(n) as size_t;
            }
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
                    (*state).x.pos = (*state).x.pos + ((n as u64) as i64);
                }
            }
        }
        if !(len != 0 && err == 0) {
            break;
        }
    }
    // note read past eof
    if len != 0 && (unsafe { (*state).eof }) != 0 {
        unsafe {
            (*state).past = 1;
        }
    }
    // return number of bytes read into user buffer
    got
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzread(file: *mut gzFile_s, buf: *mut (), mut len: u32) -> i32 {
    // get internal structure and check that it's for reading
    if file.is_null() {
        return -1;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    if (unsafe { (*state).mode }) != 7247 {
        return -1;
    }
    // check that there was no (serious) error
    if (unsafe { (*state).err }) != 0
        && (unsafe { (*state).err }) != -5
        && (unsafe { (*state).again }) == 0
    {
        return -1;
    }
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    // since an int is returned, make sure len fits in one, otherwise return
    // with an error (this avoids a flaw in the interface)
    if (len as i32) < 0 {
        unsafe { gz_error(state, -2i32, c"request does not fit in an int".as_ptr()) };
        return -1;
    }
    // read len or fewer bytes to buf
    len = gz_read(state, buf, len as size_t) as u32;
    // check for an error
    if len == 0 {
        if (unsafe { (*state).err }) != 0 && (unsafe { (*state).err }) != -5 {
            return -1;
        }
        if (unsafe { (*state).again }) != 0 {
            // non-blocking input stalled after some input was read, but no
            // uncompressed bytes were produced -- let the application know
            // this isn't EOF
            unsafe { gz_error(state, -1i32, zstrerror!() as *const c_char) };
            return -1;
        }
    }
    // return the number of bytes read
    len as i32
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzfread(
    buf: *mut (),
    size: size_t,
    nitems: size_t,
    file: *mut gzFile_s,
) -> size_t {
    let mut len: size_t = 0;
    // get internal structure and check that it's for reading
    if file.is_null() {
        return 0;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    if (unsafe { (*state).mode }) != 7247 {
        return 0;
    }
    // check that there was no (serious) error
    if (unsafe { (*state).err }) != 0
        && (unsafe { (*state).err }) != -5
        && (unsafe { (*state).again }) == 0
    {
        return 0;
    }
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    // compute bytes to read -- error on overflow
    #[cfg(target_pointer_width = "64")]
    {
        len = nitems.wrapping_mul(size);
        if size != 0 && len / size != nitems {
            unsafe { gz_error(state, -2i32, c"request does not fit in a size_t".as_ptr()) };
            return 0;
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        len = (nitems as u32).wrapping_mul(size as u32) as size_t;
        if size != 0 && (len as u32) / (size as u32) != (nitems as u32) {
            unsafe { gz_error(state, -2i32, c"request does not fit in a size_t".as_ptr()) };
            return 0;
        }
    }
    // read len or fewer bytes to buf, return the number of full items read
    #[cfg(target_pointer_width = "64")]
    let __v0: u64 = if len != 0 {
        gz_read(state, buf, len) / size
    } else {
        0
    };
    #[cfg(target_pointer_width = "64")]
    {
        __v0
    }
    #[cfg(target_pointer_width = "32")]
    let __v1: u32 = if len != 0 {
        (gz_read(state, buf, len) as u32) / (size as u32)
    } else {
        0
    };
    #[cfg(target_pointer_width = "32")]
    {
        __v1 as size_t
    }
}

// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzgetc(file: *mut gzFile_s) -> i32 {
    let mut buf: [u8; 1] = [0u8; 1];
    // get internal structure and check that it's for reading
    if file.is_null() {
        return -1;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    if (unsafe { (*state).mode }) != 7247 {
        return -1;
    }
    // check that there was no (serious) error
    if (unsafe { (*state).err }) != 0
        && (unsafe { (*state).err }) != -5
        && (unsafe { (*state).again }) == 0
    {
        return -1;
    }
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    // try output buffer (no need to check for skip request)
    if (unsafe { (*state).x.have }) != 0 {
        let __v1: u32 = unsafe { (*state).x.have };
        unsafe {
            (*state).x.have = __v1.wrapping_sub(1);
        }
        unsafe {
            (*state).x.pos = (*state).x.pos + 1;
        }
        let __v7: *mut u8 = unsafe { (*state).x.next };
        unsafe {
            (*state).x.next = __v7.add(1);
        }
        return ((unsafe { *__v7 }) as u32) as i32;
    }
    // nothing there -- try gz_read()
    #[cfg(target_pointer_width = "64")]
    {
        if gz_read(state, buf.as_mut_ptr() as *mut (), (1 as size_t)) < 1 {
            -1
        } else {
            (buf[0] as u32) as i32
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        if (gz_read(state, buf.as_mut_ptr() as *mut (), (1 as size_t)) as u32) < 1 {
            -1
        } else {
            (buf[0] as u32) as i32
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn gzgetc_(file: *mut gzFile_s) -> i32 {
    gzgetc(file)
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzungetc(c: i32, file: *mut gzFile_s) -> i32 {
    // get internal structure and check that it's for reading
    if file.is_null() {
        return -1;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    if (unsafe { (*state).mode }) != 7247 {
        return -1;
    }
    // in case this was just opened, set up the input buffer
    if (unsafe { (*state).how }) == 0 && (unsafe { (*state).x.have }) == 0 {
        gz_look(state);
    }
    // check that there was no (serious) error
    if (unsafe { (*state).err }) != 0
        && (unsafe { (*state).err }) != -5
        && (unsafe { (*state).again }) == 0
    {
        return -1;
    }
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    // process a skip request
    let __v0: bool = (unsafe { (*state).skip }) != 0 && gz_skip(state) == -1;
    if __v0 {
        return -1;
    }
    // can't push EOF
    if c < 0 {
        return -1;
    }
    // if output buffer empty, put byte at end (allows more pushing)
    if (unsafe { (*state).x.have }) == 0 {
        unsafe {
            (*state).x.have = 1;
        }
        unsafe {
            (*state).x.next = unsafe { (*state).out }
                .offset(((*state).size << 1i32) as isize)
                .sub(1);
        }
        unsafe {
            *unsafe { (*state).x.next }.add(0) = (c as i8) as u8;
        }
        unsafe {
            (*state).x.pos = (*state).x.pos - 1;
        }
        unsafe {
            (*state).past = 0;
        }
        return c;
    }
    // if no room, give up (must have already done a gzungetc())
    if (unsafe { (*state).x.have }) == (unsafe { (*state).size }) << 1i32 {
        unsafe { gz_error(state, -3i32, c"out of room to push characters".as_ptr()) };
        return -1;
    }
    // slide output data if needed and insert byte before existing data
    if (unsafe { (*state).x.next }) == unsafe { (*state).out } {
        let mut src: *mut u8 = unsafe { unsafe { (*state).out }.offset((*state).x.have as isize) };
        let mut dest: *mut u8 =
            unsafe { unsafe { (*state).out }.offset(((*state).size << 1i32) as isize) };
        while src > unsafe { (*state).out } {
            let __v4: *mut u8 = src;
            let __v5: *mut u8 = unsafe { __v4.sub(1) };
            src = __v5;
            let __v6: *mut u8 = dest;
            let __v7: *mut u8 = unsafe { __v6.sub(1) };
            dest = __v7;
            unsafe {
                *__v7 = *__v5;
            }
        }
        unsafe {
            (*state).x.next = dest;
        }
    }
    let __v10: u32 = unsafe { (*state).x.have };
    unsafe {
        (*state).x.have = __v10.wrapping_add(1);
    }
    let __v13: *mut u8 = unsafe { (*state).x.next };
    unsafe {
        (*state).x.next = __v13.sub(1);
    }
    unsafe {
        *unsafe { (*state).x.next }.add(0) = (c as i8) as u8;
    }
    unsafe {
        (*state).x.pos = (*state).x.pos - 1;
    }
    unsafe {
        (*state).past = 0;
    }
    c
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzgets(file: *mut gzFile_s, mut buf: *mut c_char, len: i32) -> *mut c_char {
    let mut left: u32 = 0;
    let mut n: u32 = 0;
    let mut eol: *mut u8 = unsafe { std::mem::zeroed() };
    // check parameters, get internal structure, and check that it's for
    // reading
    if file.is_null() || buf.is_null() || len < 1 {
        return std::ptr::null_mut::<c_char>();
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    if (unsafe { (*state).mode }) != 7247 {
        return std::ptr::null_mut::<c_char>();
    }
    // check that there was no (serious) error
    if (unsafe { (*state).err }) != 0
        && (unsafe { (*state).err }) != -5
        && (unsafe { (*state).again }) == 0
    {
        return std::ptr::null_mut::<c_char>();
    }
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    // process a skip request
    let __v0: bool = (unsafe { (*state).skip }) != 0 && gz_skip(state) == -1;
    if __v0 {
        return std::ptr::null_mut::<c_char>();
    }
    // copy output up to a new line, len-1 bytes, or there is no more output,
    // whichever comes first
    let str: *mut c_char = buf;
    left = (len as u32).wrapping_sub(1);
    if left != 0 {
        loop {
            // assure that something is in the output buffer
            let __v1: bool = (unsafe { (*state).x.have }) == 0 && gz_fetch(state) == -1;
            if __v1 {
                break;
            }
            // error
            if (unsafe { (*state).x.have }) == 0 {
                // end of file
                unsafe {
                    (*state).past = 1;
                }
                // read past end
                break; // return what we have
            }
            // look for end-of-line in current output buffer
            n = if (unsafe { (*state).x.have }) > left {
                left
            } else {
                unsafe { (*state).x.have }
            };
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                eol = (unsafe { memchr((*state).x.next as *const (), 10i32, n as c_ulong) })
                    as *mut u8;
            }
            #[cfg(target_pointer_width = "32")]
            {
                eol = (unsafe { memchr((*state).x.next as *const (), 10i32, n) }) as *mut u8;
            }
            #[cfg(all(target_pointer_width = "64", windows))]
            {
                eol = (unsafe { memchr((*state).x.next as *const (), 10i32, n as u64) }) as *mut u8;
            }
            if !eol.is_null() {
                #[cfg(all(target_pointer_width = "64", unix))]
                {
                    n = ((((unsafe { eol.offset_from((*state).x.next as *mut u8) }) as c_long)
                        as i32) as u32)
                        .wrapping_add(1);
                }
                #[cfg(target_pointer_width = "32")]
                {
                    n = (((unsafe { eol.offset_from((*state).x.next as *mut u8) }) as i32) as u32)
                        .wrapping_add(1);
                }
                #[cfg(all(target_pointer_width = "64", windows))]
                {
                    n = ((((unsafe { eol.offset_from((*state).x.next as *mut u8) }) as i64) as i32)
                        as u32)
                        .wrapping_add(1);
                }
            }
            // copy through end-of-line, or remainder if not found
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        ((*state).x.next as *const ()) as *const u8,
                        buf as *mut u8,
                        (n as c_ulong) as usize,
                    )
                };
            }
            #[cfg(target_pointer_width = "32")]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        ((*state).x.next as *const ()) as *const u8,
                        buf as *mut u8,
                        n as usize,
                    )
                };
            }
            #[cfg(all(target_pointer_width = "64", windows))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        ((*state).x.next as *const ()) as *const u8,
                        buf as *mut u8,
                        (n as u64) as usize,
                    )
                };
            }
            let __v4: u32 = unsafe { (*state).x.have };
            unsafe {
                (*state).x.have = __v4.wrapping_sub(n);
            }
            let __v7: *mut u8 = unsafe { (*state).x.next };
            unsafe {
                (*state).x.next = __v7.add(n as usize);
            }
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
                    (*state).x.pos = (*state).x.pos + ((n as u64) as i64);
                }
            }
            left = left.wrapping_sub(n);
            let __v14: *mut c_char = buf;
            buf = unsafe { __v14.add(n as usize) };
            if !(left != 0 && eol.is_null()) {
                break;
            }
        }
    }
    // append a terminating zero to the string (we don't check for a zero in
    // the contents, let the user worry about that) -- return the terminated
    // string, or if nothing was read, NULL
    if buf == str {
        return std::ptr::null_mut::<c_char>();
    }
    unsafe {
        *buf.add(0) = 0;
    }
    str
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzdirect(file: *mut gzFile_s) -> i32 {
    // get internal structure
    if file.is_null() {
        return 0;
    }
    let state: *mut __SlateRecord_gzguts_h_203 = file as *mut __SlateRecord_gzguts_h_203;
    // if the state is not known, but we can find out, then do so (this is
    // mainly for right after a gzopen() or gzdopen())
    if (unsafe { (*state).mode }) == 7247
        && (unsafe { (*state).how }) == 0
        && (unsafe { (*state).x.have }) == 0
    {
        gz_look(state);
    }
    // return 1 if transparent, 0 if processing a gzip stream
    ((unsafe { (*state).direct }) == 1) as i32
}

/// -- see zlib.h --
#[unsafe(no_mangle)]
extern "C-unwind" fn gzclose_r(file: *mut gzFile_s) -> i32 {
    let mut ret: i32 = 0;
    let mut err: i32 = 0;
    let mut state: *mut __SlateRecord_gzguts_h_203 = unsafe { std::mem::zeroed() };
    // get internal structure and check that it's for reading
    if file.is_null() {
        return -2;
    }
    state = file as *mut __SlateRecord_gzguts_h_203;
    if (unsafe { (*state).mode }) != 7247 {
        return -2;
    }
    // free memory and close file
    if (unsafe { (*state).size }) != 0 {
        unsafe { inflateEnd(std::ptr::addr_of_mut!((*state).strm)) };
        unsafe { free((*state).out as *mut ()) };
        unsafe { free((*state).r#in as *mut ()) };
    }
    err = if (unsafe { (*state).err }) == -5 {
        -5
    } else {
        0
    };
    unsafe { gz_error(state, 0i32, std::ptr::null::<c_char>()) };
    unsafe { free((*state).path as *mut ()) };
    ret = unsafe { close((*state).fd) };
    unsafe { free(state as *mut ()) };
    if ret != 0 { -1 } else { err }
}
