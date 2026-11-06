//! inflate.c -- zlib decompression
//! Copyright (C) 1995-2026 Mark Adler
//! For conditions of distribution and use, see copyright notice in zlib.h
#[cfg(not(feature = "no-gzip"))]
macro_rules! CRC2 {
    ($check:expr, $word:expr, $hbuf:expr) => {
        loop {
            $hbuf[0] = ($word) as u8;
            $hbuf[1] = (($word) >> 8i32) as u8;
            unsafe {
                $check = crc32($check, $hbuf.as_mut_ptr() as *const u8, 2u32);
            }
            if true {
                break;
            }
        }
    };
}

#[cfg(not(feature = "no-gzip"))]
macro_rules! CRC4 {
    ($check:expr, $word:expr, $hbuf:expr) => {
        loop {
            $hbuf[0] = ($word) as u8;
            $hbuf[1] = (($word) >> 8i32) as u8;
            $hbuf[2] = (($word) >> 16i32) as u8;
            $hbuf[3] = (($word) >> 24i32) as u8;
            unsafe {
                $check = crc32($check, $hbuf.as_mut_ptr() as *const u8, 4u32);
            }
            if true {
                break;
            }
        }
    };
}

#[cfg(all(feature = "zlib-debug", feature = "inflate-allow-invalid"))]
macro_rules! Trace {
    ($x__0:expr, $x__1:expr) => {
        if (unsafe { z_verbose }) >= 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(feature = "no-gzip")]
macro_rules! UPDATE_CHECK {
    ($check:expr, $buf:expr, $len:expr) => {
        unsafe { adler32($check, ($buf) as *const u8, $len) }
    };
}

#[cfg(all(target_pointer_width = "64", unix, not(feature = "no-gzip")))]
macro_rules! UPDATE_CHECK__2 {
    ($check:expr, $buf:expr, $len:expr, $state:expr) => {{
        let __v: u64 = if (unsafe { (*$state).flags }) != 0 {
            unsafe { crc32($check, ($buf) as *const u8, $len) }
        } else {
            unsafe { adler32($check, ($buf) as *const u8, $len) }
        };
        __v
    }};
}

#[cfg(all(any(target_pointer_width = "32", windows), not(feature = "no-gzip")))]
macro_rules! UPDATE_CHECK__2 {
    ($check:expr, $buf:expr, $len:expr, $state:expr) => {{
        let __v: u32 = if (unsafe { (*$state).flags }) != 0 {
            unsafe { crc32($check, ($buf) as *const u8, $len) }
        } else {
            unsafe { adler32($check, ($buf) as *const u8, $len) }
        };
        __v
    }};
}

macro_rules! ZSWAP32 {
    ($q:expr) => {
        (($q) >> 24i32 & 255)
            .wrapping_add(($q) >> 8i32 & 65280)
            .wrapping_add((($q) & 65280) << 8i32)
            .wrapping_add((($q) & 255) << 24i32)
    };
}

#[cfg(feature = "zlib-debug")]
unsafe extern "C" {
    static mut z_verbose: i32;
    fn fflush(_0: *mut slate_support::FILE) -> i32;
}

unsafe extern "C" {
    fn adler32(arg0: c_ulong, arg1: *const u8, arg2: u32) -> c_ulong;
    fn zcalloc(arg0: *mut (), arg1: u32, arg2: u32) -> *mut ();
    fn zcfree(arg0: *mut (), arg1: *mut ());
    fn inflate_table(
        arg0: u32,
        arg1: *mut u16,
        arg2: u32,
        arg3: *mut *mut __SlateRecord_inftrees_h_28,
        arg4: *mut u32,
        arg5: *mut u16,
    ) -> i32;
    fn inflate_fixed(arg0: *mut inflate_state);
    fn inflate_fast(arg0: *mut z_stream_s, arg1: u32);
}

#[cfg(not(feature = "no-gzip"))]
unsafe extern "C" {
    fn crc32(arg0: c_ulong, arg1: *const u8, arg2: u32) -> c_ulong;
}

#[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
unsafe extern "C" {
    fn memcpy(arg0: *mut (), arg1: *const (), arg2: size_t) -> *mut ();
    fn memset(arg0: *mut (), arg1: i32, arg2: size_t) -> *mut ();
}

use core::ffi::c_char;
use core::ffi::c_long;
use core::ffi::c_ulong;
use slate_support::ptrdiff_t;
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

#[repr(C)]
#[derive(Clone, Copy)]
struct gz_header_s {
    text: i32,
    time: c_ulong,
    xflags: i32,
    os: i32,
    extra: *mut u8,
    extra_len: u32,
    extra_max: u32,
    name: *mut u8,
    name_max: u32,
    comment: *mut u8,
    comm_max: u32,
    hcrc: i32,
    done: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord_inftrees_h_28 {
    op: u8,
    bits: u8,
    val: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct inflate_state {
    strm: *mut z_stream_s,
    mode: u32,
    last: i32,
    wrap: i32,
    havedict: i32,
    flags: i32,
    dmax: u32,
    check: c_ulong,
    total: c_ulong,
    head: *mut gz_header_s,
    wbits: u32,
    wsize: u32,
    whave: u32,
    wnext: u32,
    window: *mut u8,
    hold: c_ulong,
    bits: u32,
    length: u32,
    offset: u32,
    extra: u32,
    lencode: *const __SlateRecord_inftrees_h_28,
    distcode: *const __SlateRecord_inftrees_h_28,
    lenbits: u32,
    distbits: u32,
    ncode: u32,
    nlen: u32,
    ndist: u32,
    have: u32,
    next: *mut __SlateRecord_inftrees_h_28,
    lens: [u16; 320],
    work: [u16; 288],
    #[cfg(not(feature = "pkzip-bug"))]
    codes: [__SlateRecord_inftrees_h_28; 1444],
    #[cfg(feature = "pkzip-bug")]
    codes: [__SlateRecord_inftrees_h_28; 1448],
    sane: i32,
    back: i32,
    was: u32,
}

#[cfg(target_arch = "x86_64")]
#[repr(C, align(16))]
struct __SlateAlignLargeArray<T>(T);

#[cfg(not(target_arch = "x86_64"))]
#[repr(C)]
struct __SlateAlignLargeArray<T>(T);

// Change history:
//
// 1.2.beta0    24 Nov 2002
// - First version -- complete rewrite of inflate to simplify code, avoid
//   creation of window when not needed, minimize use of window when it is
//   needed, make inffast.c even faster, implement gzip decoding, and to
//   improve code readability and style over the previous zlib inflate code
//
// 1.2.beta1    25 Nov 2002
// - Use pointers for available input and output checking in inffast.c
// - Remove input and output counters in inffast.c
// - Change inffast.c entry and loop from avail_in >= 7 to >= 6
// - Remove unnecessary second byte pull from length extra in inffast.c
// - Unroll direct copy to three copies per loop in inffast.c
//
// 1.2.beta2    4 Dec 2002
// - Change external routine names to reduce potential conflicts
// - Correct filename to inffixed.h for fixed tables in inflate.c
// - Make hbuf[] unsigned char to match parameter type in inflate.c
// - Change strm->next_out[-state->offset] to *(strm->next_out - state->offset)
//   to avoid negation problem on Alphas (64 bit) in inflate.c
//
// 1.2.beta3    22 Dec 2002
// - Add comments on state->bits assertion in inffast.c
// - Add comments on op field in inftrees.h
// - Fix bug in reuse of allocated window after inflateReset()
// - Remove bit fields--back to byte structure for speed
// - Remove distance extra == 0 check in inflate_fast()--only helps for lengths
// - Change post-increments to pre-increments in inflate_fast(), PPC biased?
// - Add compile time option, POSTINC, to use post-increments instead (Intel?)
// - Make MATCH copy in inflate() much faster for when inflate_fast() not used
// - Use local copies of stream next and avail values, as well as local bit
//   buffer and bit count in inflate()--for speed when inflate_fast() not used
//
// 1.2.beta4    1 Jan 2003
// - Split ptr - 257 statements in inflate_table() to avoid compiler warnings
// - Move a comment on output buffer sizes from inffast.c to inflate.c
// - Add comments in inffast.c to introduce the inflate_fast() routine
// - Rearrange window copies in inflate_fast() for speed and simplification
// - Unroll last copy for window match in inflate_fast()
// - Use local copies of window variables in inflate_fast() for speed
// - Pull out common wnext == 0 case for speed in inflate_fast()
// - Make op and len in inflate_fast() unsigned for consistency
// - Add FAR to lcode and dcode declarations in inflate_fast()
// - Simplified bad distance check in inflate_fast()
// - Added inflateBackInit(), inflateBack(), and inflateBackEnd() in new
//   source file infback.c to provide a call-back interface to inflate for
//   programs like gzip and unzip -- uses window as output buffer to avoid
//   window copying
//
// 1.2.beta5    1 Jan 2003
// - Improved inflateBack() interface to allow the caller to provide initial
//   input in strm.
// - Fixed stored blocks bug in inflateBack()
//
// 1.2.beta6    4 Jan 2003
// - Added comments in inffast.c on effectiveness of POSTINC
// - Typecasting all around to reduce compiler warnings
// - Changed loops from while (1) or do {} while (1) to for (;;), again to
//   make compilers happy
// - Changed type of window in inflateBackInit() to unsigned char *
//
// 1.2.beta7    27 Jan 2003
// - Changed many types to unsigned or unsigned short to avoid warnings
// - Added inflateCopy() function
//
// 1.2.0        9 Mar 2003
// - Changed inflateBack() interface to provide separate opaque descriptors
//   for the in() and out() functions
// - Changed inflateBack() argument and in_func typedef to swap the length
//   and buffer address return values for the input function
// - Check next_in and next_out for Z_NULL on entry to inflate()
//
// The history for versions after 1.2.0 are in ChangeLog in zlib distribution.
fn inflateStateCheck(strm: *mut z_stream_s) -> i32 {
    if strm.is_null() || (unsafe { (*strm).zalloc }) == None || (unsafe { (*strm).zfree }) == None {
        return 1;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    if state.is_null()
        || (unsafe { (*state).strm }) != strm
        || ((unsafe { (*state).mode }) as u32) < 16180
        || ((unsafe { (*state).mode }) as u32) > 16211
    {
        return 1;
    }
    0
}

#[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
use slate_support::memcpy;
#[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
use slate_support::memset;

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateResetKeep(strm: *mut z_stream_s) -> i32 {
    if inflateStateCheck(strm) != 0 {
        return -2;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    unsafe {
        (*state).total = 0;
    }
    unsafe {
        (*strm).total_out = 0;
    }
    unsafe {
        (*strm).total_in = 0;
    }
    unsafe {
        (*strm).msg = std::ptr::null_mut::<c_char>();
    }
    unsafe {
        (*strm).data_type = 0;
    }
    if (unsafe { (*state).wrap }) != 0 {
        unsafe {
            (*strm).adler = ((*state).wrap & 1) as c_ulong;
        }
    }
    // to support ill-conceived Java test suite
    unsafe {
        (*state).mode = 16180;
    }
    unsafe {
        (*state).last = 0;
    }
    unsafe {
        (*state).havedict = 0;
    }
    unsafe {
        (*state).flags = -1;
    }
    unsafe {
        (*state).dmax = 32768;
    }
    unsafe {
        (*state).head = std::ptr::null_mut::<gz_header_s>();
    }
    unsafe {
        (*state).hold = 0;
    }
    unsafe {
        (*state).bits = 0;
    }
    let __v0: *mut __SlateRecord_inftrees_h_28 =
        unsafe { (*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28 };
    unsafe {
        (*state).next = __v0;
    }
    let __v1: *const __SlateRecord_inftrees_h_28 = __v0 as *const __SlateRecord_inftrees_h_28;
    unsafe {
        (*state).distcode = __v1;
    }
    unsafe {
        (*state).lencode = __v1;
    }
    unsafe {
        (*state).sane = 1;
    }
    unsafe {
        (*state).back = -1;
    }
    #[cfg(feature = "zlib-debug")]
    {
        Tracev!(
            slate_support::stderr(),
            c"inflate: reset\n".as_ptr() as *mut c_char
        );
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateReset(strm: *mut z_stream_s) -> i32 {
    if inflateStateCheck(strm) != 0 {
        return -2;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    unsafe {
        (*state).wsize = 0;
    }
    unsafe {
        (*state).whave = 0;
    }
    unsafe {
        (*state).wnext = 0;
    }
    inflateResetKeep(strm)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateReset2(strm: *mut z_stream_s, mut windowBits: i32) -> i32 {
    let mut wrap: i32 = 0;
    // get the state
    if inflateStateCheck(strm) != 0 {
        return -2;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    // extract wrap request from windowBits parameter
    if windowBits < 0 {
        if windowBits < -15 {
            return -2;
        }
        wrap = 0;
        windowBits = -windowBits;
    } else {
        wrap = (windowBits >> 4i32) + 5;
        #[cfg(not(feature = "no-gzip"))]
        {
            if windowBits < 48 {
                windowBits &= 15;
            }
        }
    }
    // set number of window bits, free window if different
    if windowBits != 0 && (windowBits < 8 || windowBits > 15) {
        return -2;
    }
    if !unsafe { (*state).window }.is_null() && (unsafe { (*state).wbits }) != (windowBits as u32) {
        ZFREE!(strm, unsafe { (*state).window });
        unsafe {
            (*state).window = std::ptr::null_mut::<u8>();
        }
    }
    // update state and reset the rest of it
    unsafe {
        (*state).wrap = wrap;
    }
    unsafe {
        (*state).wbits = windowBits as u32;
    }
    inflateReset(strm)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateInit2_(
    strm: *mut z_stream_s,
    windowBits: i32,
    version: *const c_char,
    stream_size: i32,
) -> i32 {
    let mut ret: i32 = 0;
    if version.is_null()
        || ((unsafe { *version.add(0) }) as i32)
            != ((unsafe { *(c"1.3.2.1-motley".as_ptr() as *mut c_char).add(0) }) as i32)
        || stream_size != (std::mem::size_of::<z_stream_s>() as i32)
    {
        return -6;
    }
    if strm.is_null() {
        return -2;
    }
    unsafe {
        (*strm).msg = std::ptr::null_mut::<c_char>();
    }
    // in case we return an error
    if (unsafe { (*strm).zalloc }) == None {
        unsafe {
            (*strm).zalloc = unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut (), u32, u32) -> *mut ()>,
                >(zcalloc as *const ())
            };
        }
        unsafe {
            (*strm).opaque = std::ptr::null_mut::<()>();
        }
    }
    if (unsafe { (*strm).zfree }) == None {
        unsafe {
            (*strm).zfree = unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut (), *mut ())>,
                >(zcfree as *const ())
            };
        }
    }
    let state: *mut inflate_state =
        ZALLOC__2!(strm, 1i32, std::mem::size_of::<inflate_state>() as size_t)
            as *mut inflate_state;
    if state.is_null() {
        return -4;
    }
    zmemzero!(state, std::mem::size_of::<inflate_state>() as size_t);
    #[cfg(feature = "zlib-debug")]
    {
        Tracev!(
            slate_support::stderr(),
            c"inflate: allocated\n".as_ptr() as *mut c_char
        );
    }
    unsafe {
        (*strm).state = state as *mut internal_state;
    }
    unsafe {
        (*state).strm = strm;
    }
    unsafe {
        (*state).window = std::ptr::null_mut::<u8>();
    }
    unsafe {
        (*state).mode = 16180;
    }
    // to pass state test in inflateReset2()
    ret = inflateReset2(strm, windowBits);
    if ret != 0 {
        ZFREE!(strm, state);
        unsafe {
            (*strm).state = std::ptr::null_mut::<internal_state>();
        }
    }
    ret
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateInit_(
    strm: *mut z_stream_s,
    version: *const c_char,
    stream_size: i32,
) -> i32 {
    inflateInit2_(strm, 15i32, version, stream_size)
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflatePrime(strm: *mut z_stream_s, bits: i32, mut value: i32) -> i32 {
    if inflateStateCheck(strm) != 0 {
        return -2;
    }
    if bits == 0 {
        return 0;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    if bits < 0 {
        unsafe {
            (*state).hold = 0;
        }
        unsafe {
            (*state).bits = 0;
        }
        return 0;
    }
    if bits > 16 || unsafe { (*state).bits }.wrapping_add(bits as u32) > 32 {
        return -2;
    }
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        value = ((value as c_long) & ((1 as c_long) << bits) - 1) as i32;
    }
    #[cfg(any(target_pointer_width = "32", windows))]
    {
        value = (value as c_long) & ((1 as c_long) << bits) - 1;
    }
    let __v3: c_ulong = unsafe { (*state).hold };
    unsafe {
        (*state).hold = __v3.wrapping_add((value as c_ulong) << (*state).bits);
    }
    let __v6: u32 = unsafe { (*state).bits };
    unsafe {
        (*state).bits = __v6.wrapping_add(bits as u32);
    }
    0
}

/// Update the window with the last wsize (normally 32K) bytes written before
/// returning.  If window does not exist yet, create it.  This is only called
/// when a window is already in use, or when output has been written during this
/// inflate call, but the end of the deflate stream has not been reached yet.
/// It is also called to create a window for dictionary data when a dictionary
/// is loaded.
///
/// Providing output buffers larger than 32K to inflate() should provide a speed
/// advantage, since only the last 32K of output is copied to the sliding window
/// upon return from inflate(), and since all distances after the first 32K of
/// output will fall in the output data, making match copies simpler and faster.
/// The advantage may be dependent on the size of the processor's data caches.
fn updatewindow(strm: *mut z_stream_s, end: *const u8, mut copy: u32) -> i32 {
    let mut dist: u32 = 0;
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    // if it hasn't been done already, allocate space for the window
    if unsafe { (*state).window }.is_null() {
        unsafe {
            (*state).window = ZALLOC!(
                strm,
                1u32 << (*state).wbits,
                std::mem::size_of::<u8>() as size_t
            ) as *mut u8;
        }
        if unsafe { (*state).window }.is_null() {
            return 1;
        }
    }
    // if window not in use yet, initialize
    if (unsafe { (*state).wsize }) == 0 {
        unsafe {
            (*state).wsize = 1u32 << (*state).wbits;
        }
        unsafe {
            (*state).wnext = 0;
        }
        unsafe {
            (*state).whave = 0;
        }
    }
    // copy state->wsize or less output bytes into the circular window
    if copy >= unsafe { (*state).wsize } {
        #[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    ((end.offset(-((*state).wsize as isize)) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    (((*state).window as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    (((*state).wsize as size_t) as usize) as usize,
                )
            };
        }
        #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    (end.offset(-((*state).wsize as isize)) as *const ()) as *const u8,
                    ((*state).window as *mut ()) as *mut u8,
                    ((*state).wsize as size_t) as usize,
                )
            };
        }
        unsafe {
            (*state).wnext = 0;
        }
        unsafe {
            (*state).whave = (*state).wsize;
        }
    } else {
        dist = unsafe { (*state).wsize }.wrapping_sub(unsafe { (*state).wnext });
        if dist > copy {
            dist = copy;
        }
        #[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    ((end.sub(copy as usize) as *const ()) as *const core::ffi::c_void)
                        as *const u8,
                    ((unsafe { (*state).window }.offset((*state).wnext as isize) as *mut ())
                        as *mut core::ffi::c_void) as *mut u8,
                    ((dist as size_t) as usize) as usize,
                )
            };
        }
        #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    (end.sub(copy as usize) as *const ()) as *const u8,
                    (unsafe { (*state).window }.offset((*state).wnext as isize) as *mut ())
                        as *mut u8,
                    (dist as size_t) as usize,
                )
            };
        }
        copy = copy.wrapping_sub(dist);
        if copy != 0 {
            #[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        ((end.sub(copy as usize) as *const ()) as *const core::ffi::c_void)
                            as *const u8,
                        (((*state).window as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                        ((copy as size_t) as usize) as usize,
                    )
                };
            }
            #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (end.sub(copy as usize) as *const ()) as *const u8,
                        ((*state).window as *mut ()) as *mut u8,
                        (copy as size_t) as usize,
                    )
                };
            }
            unsafe {
                (*state).wnext = copy;
            }
            unsafe {
                (*state).whave = (*state).wsize;
            }
        } else {
            let __v3: u32 = unsafe { (*state).wnext };
            unsafe {
                (*state).wnext = __v3.wrapping_add(dist);
            }
            if (unsafe { (*state).wnext }) == unsafe { (*state).wsize } {
                unsafe {
                    (*state).wnext = 0;
                }
            }
            if (unsafe { (*state).whave }) < unsafe { (*state).wsize } {
                let __v6: u32 = unsafe { (*state).whave };
                unsafe {
                    (*state).whave = __v6.wrapping_add(dist);
                }
            }
        }
    }
    0
}

// Macros for inflate():
// check function to use adler32() for zlib or crc32() for gzip
// check macros for header crc
// Load registers with state in inflate() for speed
// Restore state from registers in inflate()
// Clear the input bit accumulator
// Get a byte of input into the bit accumulator, or return from inflate()
// if there is no input available.
// Assure that there are at least n bits in the bit accumulator.  If there is
// not enough available input to do that, then return from inflate().
// Return the low n bits of the bit accumulator (n < 16)
// Remove n bits from the bit accumulator
// Remove zero to seven bits as needed to go to a byte boundary
// inflate() uses a state machine to process as much input data and generate as
// much output data as possible before returning.  The state machine is
// structured roughly as follows:
//
//  for (;;) switch (state) {
//  ...
//  case STATEn:
//      if (not enough input data or output space to make progress)
//          return;
//      ... make progress ...
//      state = STATEm;
//      break;
//  ...
//  }
//
// so when inflate() is called again, the same case is attempted again, and
// if the appropriate resources are provided, the machine proceeds to the
// next state.  The NEEDBITS() macro is usually the way the state evaluates
// whether it can proceed or should return.  NEEDBITS() does the return if
// the requested bits are not available.  The typical use of the BITS macros
// is:
//
//      NEEDBITS(n);
//      ... do something with BITS(n) ...
//      DROPBITS(n);
//
// where NEEDBITS(n) either returns from inflate() if there isn't enough
// input left to load n bits into the accumulator, or it continues.  BITS(n)
// gives the low n bits in the accumulator.  When done, DROPBITS(n) drops
// the low n bits off the accumulator.  INITBITS() clears the accumulator
// and sets the number of available bits to zero.  BYTEBITS() discards just
// enough bits to put the accumulator on a byte boundary.  After BYTEBITS()
// and a NEEDBITS(8), then BITS(8) would return the next byte in the stream.
//
// NEEDBITS(n) uses PULLBYTE() to get an available byte of input, or to return
// if there is no input available.  The decoding of variable length codes uses
// PULLBYTE() directly in order to pull just enough bytes to decode the next
// code, and no more.
//
// Some states loop until they get enough input, making sure that enough
// state information is maintained to continue the loop where it left off
// if NEEDBITS() returns in the loop.  For example, want, need, and keep
// would all have to actually be part of the saved state in case NEEDBITS()
// returns:
//
//  case STATEw:
//      while (want < need) {
//          NEEDBITS(n);
//          keep[want++] = BITS(n);
//          DROPBITS(n);
//      }
//      state = STATEx;
//  case STATEx:
//
// As shown above, if the next state is also the next case, then the break
// is omitted.
//
// A state may also return if there is not enough output space available to
// complete that state.  Those states are copying stored data, writing a
// literal byte, and copying a matching string.
//
// When returning, a "goto inf_leave" is used to update the total counters,
// update the check value, and determine whether any progress has been made
// during that inflate() call in order to return the proper return code.
// Progress is defined as a change in either strm->avail_in or strm->avail_out.
// When there is a window, goto inf_leave will update the window with the last
// output written.  If a goto inf_leave occurs in the middle of decompression
// and there is no window currently, goto inf_leave will create one and copy
// output to the window for the next call of inflate().
//
// In this implementation, the flush parameter of inflate() only affects the
// return code (per zlib.h).  inflate() always writes as much as possible to
// strm->next_out, given the space available and the provided input--the effect
// documented in zlib.h of Z_SYNC_FLUSH.  Furthermore, inflate() always defers
// the allocation of and copying into a sliding window until necessary, which
// provides the effect documented in zlib.h for Z_FINISH when the entire input
// stream available.  So the only thing the flush parameter actually does is:
// when flush is set to Z_FINISH, inflate() cannot return Z_OK.  Instead it
// will return Z_BUF_ERROR if it has not reached the end of the stream.
#[unsafe(no_mangle)]
extern "C-unwind" fn inflate(strm: *mut z_stream_s, flush: i32) -> i32 {
    #[cfg(not(feature = "no-gzip"))]
    let mut __v292: bool = unsafe { std::mem::zeroed() };
    #[cfg(not(feature = "no-gzip"))]
    let mut hbuf: [u8; 4] = unsafe { std::mem::zeroed() };
    // return code
    let mut ret: i32 = unsafe { std::mem::zeroed() };
    // length to copy for repeats, bits to drop
    let mut len: u32 = unsafe { std::mem::zeroed() };
    // parent table entry
    let mut last: std::mem::MaybeUninit<__SlateRecord_inftrees_h_28> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_342: *mut __SlateRecord_inftrees_h_28 =
        std::ptr::addr_of_mut!(last) as *mut __SlateRecord_inftrees_h_28; // current decoding table entry
    let mut here: std::mem::MaybeUninit<__SlateRecord_inftrees_h_28> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_343: *mut __SlateRecord_inftrees_h_28 =
        std::ptr::addr_of_mut!(here) as *mut __SlateRecord_inftrees_h_28; // where to copy match bytes from
    let mut from: *mut u8 = unsafe { std::mem::zeroed() };
    // number of stored or match bytes to copy
    let mut copy: u32 = unsafe { std::mem::zeroed() };
    // save starting available input and output
    let mut out: u32 = unsafe { std::mem::zeroed() };
    let mut r#in: u32 = unsafe { std::mem::zeroed() };
    // bits in bit buffer
    let mut bits: u32 = unsafe { std::mem::zeroed() };
    // bit buffer
    #[cfg(all(target_pointer_width = "64", unix))]
    let mut hold: u64 = unsafe { std::mem::zeroed() };
    #[cfg(any(target_pointer_width = "32", windows))]
    let mut hold: u32 = unsafe { std::mem::zeroed() };
    // available input and output
    let mut left: u32 = unsafe { std::mem::zeroed() };
    let mut have: u32 = unsafe { std::mem::zeroed() };
    // next output
    let mut put: *mut u8 = unsafe { std::mem::zeroed() };
    // next input
    let mut next: *mut u8 = unsafe { std::mem::zeroed() };
    let mut state: *mut inflate_state = unsafe { std::mem::zeroed() };
    unsafe {
        if inflateStateCheck(strm) != 0
            || unsafe { (*strm).next_out }.is_null()
            || unsafe { (*strm).next_in }.is_null() && (*strm).avail_in != 0
        {
            return -2;
        } else {
            state = (*strm).state as *mut inflate_state;
            if ((*state).mode as u32) == 16191 {
                (*state).mode = 16192;
            }
            // skip check
            LOAD!(put, strm, left, next, have, hold, state, bits);
            r#in = have;
            out = left;
            ret = 0;
            '__join_7: {
                '__join_185: {
                    '__join_11: {
                        '__join_12: {
                            '__join_13: {
                                '__join_10: {
                                    '__loop_319: loop {
                                        '__join_215: {
                                            '__join_227: {
                                                '__join_239: {
                                                    '__join_253: {
                                                        '__join_266: {
                                                            '__join_273: {
                                                                '__join_282: {
                                                                    '__join_194: {
                                                                        '__join_196: {
                                                                            '__join_199: {
                                                                                '__join_171: {
                                                                                    #[cfg(
                                                                                        feature = "no-gzip"
                                                                                    )]
                                                                                    {
                                                                                        '__join_108: {
                                                                                            let __t2: u32 = (*state).mode as u32;
                                                                                            if __t2 == 16180 {
if (*state).wrap == 0 {
(*state).mode = 16192;
continue '__loop_319;
} else {
'__loop_206: loop {
if bits < 16 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v254: *mut u8 = next;
next = __v254.add(1);
hold = hold.wrapping_add((*__v254 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_206;
}
}
}
} else {
if true {
break;
}
}
}
if ((BITS!(8i32, hold) << 8i32) as c_ulong).wrapping_add(hold >> 8i32) % 31 != 0 {
(*strm).msg = c"incorrect header check".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
if BITS!(4i32, hold) != 8 {
(*strm).msg = c"unknown compression method".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
DROPBITS!(4i32, hold, bits);
len = BITS!(4i32, hold).wrapping_add(8);
if (*state).wbits == 0 {
(*state).wbits = len;
}
if len > 15 || len > (*state).wbits {
(*strm).msg = c"invalid window size".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
(*state).dmax = 1u32 << len;
(*state).flags = 0;
// indicate zlib header
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:   zlib header ok\n".as_ptr() as *mut c_char);
}
#[cfg(all(target_pointer_width = "64", unix))]
let __v247: u64 = adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
#[cfg(any(target_pointer_width = "32", windows))]
let __v247: u32 = adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
(*state).check = __v247;
(*strm).adler = __v247;
(*state).mode = (if hold & 512 != 0 { 16189i32 } else { 16191i32 }) as u32;
INITBITS!(hold, bits);
continue '__loop_319;
}
}
}
}
} else {
if __t2 == 16189 {
'__loop_193: loop {
if bits < 32 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v244: *mut u8 = next;
next = __v244.add(1);
hold = hold.wrapping_add((*__v244 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_193;
}
}
}
} else {
if true {
break;
}
}
}
#[cfg(all(target_pointer_width = "64", unix))]
let __v237: u64 = ZSWAP32!(hold);
#[cfg(any(target_pointer_width = "32", windows))]
let __v237: u32 = ZSWAP32!(hold);
(*state).check = __v237;
(*strm).adler = __v237;
INITBITS!(hold, bits);
(*state).mode = 16190;
// fallthrough
break '__join_239;
} else {
if __t2 == 16190 {
break '__join_239;
} else {
if __t2 == 16191 {
break '__join_227;
} else {
if __t2 == 16192 {
break '__join_215;
} else {
if __t2 == 16193 {
BYTEBITS!(hold, bits); // go to byte boundary
'__loop_169: loop {
if bits < 32 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v3: *mut u8 = next;
next = __v3.add(1);
hold = hold.wrapping_add((*__v3 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_169;
}
}
}
} else {
if true {
break;
}
}
}
if hold & 65535 != hold >> 16i32 ^ 65535 {
(*strm).msg = c"invalid stored block lengths".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
(*state).length = (hold as u32) & 65535;
#[cfg(feature = "zlib-debug")]
{
Tracev__2!(slate_support::stderr(), c"inflate:       stored length %u\n".as_ptr() as *mut c_char, unsafe { (*state).length });
}
INITBITS!(hold, bits);
(*state).mode = 16194;
if flush == 6 {
break '__join_7;
} else {
// fallthrough
break '__join_266;
}
}
} else {
if __t2 == 16194 {
break '__join_266;
} else {
if __t2 == 16195 {
break '__join_253;
} else {
if __t2 == 16196 {
'__loop_152: loop {
if bits < 14 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v4: *mut u8 = next;
next = __v4.add(1);
hold = hold.wrapping_add((*__v4 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_152;
}
}
}
} else {
if true {
break;
}
}
}
(*state).nlen = BITS!(5i32, hold).wrapping_add(257);
DROPBITS!(5i32, hold, bits);
(*state).ndist = BITS!(5i32, hold).wrapping_add(1);
DROPBITS!(5i32, hold, bits);
(*state).ncode = BITS!(4i32, hold).wrapping_add(4);
DROPBITS!(4i32, hold, bits);
#[cfg(not(feature = "pkzip-bug"))]
{
if (*state).nlen > 286 || (*state).ndist > 30 {
(*strm).msg = c"too many length or distance symbols".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       table sizes ok\n".as_ptr() as *mut c_char);
}
(*state).have = 0;
(*state).mode = 16197;
// fallthrough
}
}
#[cfg(all(feature = "zlib-debug", feature = "pkzip-bug"))]
{
Tracev!(slate_support::stderr(), c"inflate:       table sizes ok\n".as_ptr() as *mut c_char);
}
#[cfg(feature = "pkzip-bug")]
{
(*state).have = 0;
(*state).mode = 16197;
}
} else {
if __t2 != 16197 {
if __t2 == 16198 {
break '__join_108;
} else {
if __t2 == 16199 {
break '__join_171;
} else {
if __t2 == 16200 {
break '__join_199;
} else {
if __t2 == 16201 {
break '__join_196;
} else {
if __t2 == 16202 {
break '__join_194;
} else {
if __t2 == 16203 {
break '__join_282;
} else {
if __t2 == 16204 {
break '__join_273;
} else {
if __t2 == 16205 {
if left == 0 {
break '__join_7;
} else {
let __v35: *mut u8 = put;
put = __v35.add(1);
*__v35 = (*state).length as u8;
left = left.wrapping_sub(1);
(*state).mode = 16200;
continue '__loop_319;
}
} else {
if __t2 == 16206 {
if (*state).wrap != 0 {
'__loop_19: loop {
if bits < 32 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v29: *mut u8 = next;
next = __v29.add(1);
hold = hold.wrapping_add((*__v29 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_19;
}
}
}
} else {
if true {
break;
}
}
}
out = out.wrapping_sub(left);
#[cfg(all(target_pointer_width = "64", unix))]
let __v19: u64 = (*strm).total_out;
#[cfg(any(target_pointer_width = "32", windows))]
let __v19: u32 = (*strm).total_out;
(*strm).total_out = __v19.wrapping_add(out as c_ulong);
#[cfg(all(target_pointer_width = "64", unix))]
let __v16: u64 = (*state).total;
#[cfg(any(target_pointer_width = "32", windows))]
let __v16: u32 = (*state).total;
(*state).total = __v16.wrapping_add(out as c_ulong);
if (*state).wrap & 4 != 0 && out != 0 {
#[cfg(all(target_pointer_width = "64", unix))]
let __v14: u64 = UPDATE_CHECK!(unsafe { (*state).check }, put.sub(out as usize), out);
#[cfg(any(target_pointer_width = "32", windows))]
let __v14: u32 = UPDATE_CHECK!(unsafe { (*state).check }, put.sub(out as usize), out);
(*state).check = __v14;
(*strm).adler = __v14;
}
out = left;
if (*state).wrap & 4 != 0 && ZSWAP32!(hold) != (*state).check {
(*strm).msg = c"incorrect data check".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
break '__join_13;
}
} else {
break '__join_12;
}
} else {
if __t2 == 16208 {
break '__join_11;
} else {
if __t2 == 16209 {
break '__join_10;
} else {
if __t2 == 16210 {
return -4;
} else {
if __t2 == 16211 {
break '__loop_319;
} else {
break '__loop_319;
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
                                                                                            while (*state).have < (*state).ncode {
'__loop_5: loop {
if bits < 3 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v195: *mut u8 = next;
next = __v195.add(1);
hold = hold.wrapping_add((*__v195 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_5;
}
}
}
} else {
if true {
break;
}
}
}
let __v187: u32 = (*state).have;
(*state).have = __v187.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).offset((order.0[__v187 as usize] as i32) as isize) = BITS!(3i32, hold) as u16;
DROPBITS!(3i32, hold, bits);
}
                                                                                            while (*state).have < 19 {
let __v184: u32 = (*state).have;
(*state).have = __v184.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).offset((order.0[__v184 as usize] as i32) as isize) = 0;
}
                                                                                            (*state).next = (*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28;
                                                                                            let __v182: *const __SlateRecord_inftrees_h_28 = (*state).next as *const __SlateRecord_inftrees_h_28;
                                                                                            (*state).distcode = __v182;
                                                                                            (*state).lencode = __v182;
                                                                                            (*state).lenbits = 7;
                                                                                            ret = inflate_table(0u32, (*state).lens.as_mut_ptr() as *mut u16, 19u32, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).lenbits), (*state).work.as_mut_ptr() as *mut u16);
                                                                                            if ret
                                                                                                != 0
                                                                                            {
                                                                                                (*strm).msg = c"invalid code lengths set".as_ptr() as *mut c_char;
                                                                                                (*state).mode = 16209;
                                                                                                continue '__loop_319;
                                                                                            } else {
                                                                                                #[cfg(feature = "zlib-debug")]
                                                                                                {
                                                                                                    Tracev!(slate_support::stderr(), c"inflate:       code lengths ok\n".as_ptr() as *mut c_char);
                                                                                                }
                                                                                                (*state).have = 0;
                                                                                                (*state).mode = 16198;
                                                                                                // fallthrough
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    '__join_172: {
                                                                                        '__join_58: {
                                                                                            #[cfg(not(feature = "no-gzip"))]
                                                                                            {
                                                                                                '__join_67: {
                                                                                                    '__join_79: {
                                                                                                        '__join_86: {
                                                                                                            '__join_106: {
                                                                                                                '__join_107: {
                                                                                                                    '__join_117: {
                                                                                                                        '__join_151: {
                                                                                                                            let __t1: u32 = (*state).mode as u32;
                                                                                                                            if __t1 == 16180 {
if (*state).wrap == 0 {
(*state).mode = 16192;
continue '__loop_319;
} else {
'__loop_314: loop {
if bits < 16 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v336: *mut u8 = next;
next = __v336.add(1);
hold = hold.wrapping_add((*__v336 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_314;
}
}
}
} else {
if true {
break;
}
}
}
if (*state).wrap & 2 != 0 && hold == 35615 {
// gzip header
if (*state).wbits == 0 {
(*state).wbits = 15;
}
(*state).check = crc32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
CRC2!((*state).check, hold, hbuf);
INITBITS!(hold, bits);
(*state).mode = 16181;
continue '__loop_319;
} else {
if !unsafe { (*state).head }.is_null() {
(*(*state).head).done = -1;
}
if (*state).wrap & 1 == 0 || ((BITS!(8i32, hold) << 8i32) as c_ulong).wrapping_add(hold >> 8i32) % 31 != 0 {
(*strm).msg = c"incorrect header check".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
// check if zlib header allowed
if BITS!(4i32, hold) != 8 {
(*strm).msg = c"unknown compression method".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
DROPBITS!(4i32, hold, bits);
len = BITS!(4i32, hold).wrapping_add(8);
if (*state).wbits == 0 {
(*state).wbits = len;
}
if len > 15 || len > (*state).wbits {
(*strm).msg = c"invalid window size".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
(*state).dmax = 1u32 << len;
(*state).flags = 0;
// indicate zlib header
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:   zlib header ok\n".as_ptr() as *mut c_char);
}
#[cfg(all(target_pointer_width = "64", unix))]
let __v329: u64 = adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
#[cfg(any(target_pointer_width = "32", windows))]
let __v329: u32 = adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
(*state).check = __v329;
(*strm).adler = __v329;
(*state).mode = (if hold & 512 != 0 { 16189i32 } else { 16191i32 }) as u32;
INITBITS!(hold, bits);
continue '__loop_319;
}
}
}
}
}
} else {
if __t1 == 16181 {
'__loop_295: loop {
if bits < 16 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v326: *mut u8 = next;
next = __v326.add(1);
hold = hold.wrapping_add((*__v326 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_295;
}
}
}
} else {
if true {
break;
}
}
}
(*state).flags = hold as i32;
if (*state).flags & 255 != 8 {
(*strm).msg = c"unknown compression method".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
if (*state).flags & 57344 != 0 {
(*strm).msg = c"unknown header flags set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
if !unsafe { (*state).head }.is_null() {
(*(*state).head).text = (hold >> 8i32 & 1) as i32;
}
if (*state).flags & 512 != 0 && (*state).wrap & 4 != 0 {
CRC2!((*state).check, hold, hbuf);
}
INITBITS!(hold, bits);
(*state).mode = 16182;
// fallthrough
break '__join_282;
}
}
} else {
if __t1 == 16182 {
break '__join_282;
} else {
if __t1 == 16183 {
break '__join_273;
} else {
if __t1 == 16184 {
break '__join_266;
} else {
if __t1 == 16185 {
break '__join_253;
} else {
if __t1 == 16186 {
break '__join_239;
} else {
if __t1 == 16187 {
break '__join_227;
} else {
if __t1 == 16188 {
break '__join_215;
} else {
if __t1 == 16189 {
'__loop_202: loop {
if bits < 32 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v253: *mut u8 = next;
next = __v253.add(1);
hold = hold.wrapping_add((*__v253 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_202;
}
}
}
} else {
if true {
break;
}
}
}
#[cfg(all(target_pointer_width = "64", unix))]
let __v246: u64 = ZSWAP32!(hold);
#[cfg(any(target_pointer_width = "32", windows))]
let __v246: u32 = ZSWAP32!(hold);
(*state).check = __v246;
(*strm).adler = __v246;
INITBITS!(hold, bits);
(*state).mode = 16190;
// fallthrough
break '__join_199;
} else {
if __t1 == 16190 {
break '__join_199;
} else {
if __t1 == 16191 {
break '__join_196;
} else {
if __t1 == 16192 {
break '__join_194;
} else {
if __t1 == 16193 {
BYTEBITS!(hold, bits); // go to byte boundary
'__loop_178: loop {
if bits < 32 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v233: *mut u8 = next;
next = __v233.add(1);
hold = hold.wrapping_add((*__v233 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_178;
}
}
}
} else {
if true {
break;
}
}
}
if hold & 65535 != hold >> 16i32 ^ 65535 {
(*strm).msg = c"invalid stored block lengths".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
(*state).length = (hold as u32) & 65535;
#[cfg(feature = "zlib-debug")]
{
Tracev__2!(slate_support::stderr(), c"inflate:       stored length %u\n".as_ptr() as *mut c_char, unsafe { (*state).length });
}
INITBITS!(hold, bits);
(*state).mode = 16194;
if flush == 6 {
break '__join_7;
} else {
// fallthrough
break '__join_172;
}
}
} else {
if __t1 == 16194 {
break '__join_172;
} else {
if __t1 == 16195 {
break '__join_171;
} else {
if __t1 == 16196 {
'__loop_161: loop {
if bits < 14 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v213: *mut u8 = next;
next = __v213.add(1);
hold = hold.wrapping_add((*__v213 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_161;
}
}
}
} else {
if true {
break;
}
}
}
(*state).nlen = BITS!(5i32, hold).wrapping_add(257);
DROPBITS!(5i32, hold, bits);
(*state).ndist = BITS!(5i32, hold).wrapping_add(1);
DROPBITS!(5i32, hold, bits);
(*state).ncode = BITS!(4i32, hold).wrapping_add(4);
DROPBITS!(4i32, hold, bits);
#[cfg(not(feature = "pkzip-bug"))]
{
if (*state).nlen > 286 || (*state).ndist > 30 {
(*strm).msg = c"too many length or distance symbols".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       table sizes ok\n".as_ptr() as *mut c_char);
}
(*state).have = 0;
(*state).mode = 16197;
// fallthrough
break '__join_151;
}
}
#[cfg(all(feature = "zlib-debug", feature = "pkzip-bug"))]
{
Tracev!(slate_support::stderr(), c"inflate:       table sizes ok\n".as_ptr() as *mut c_char);
}
#[cfg(feature = "pkzip-bug")]
{
(*state).have = 0;
(*state).mode = 16197;
break '__join_151;
}
} else {
if __t1 == 16197 {
break '__join_151;
} else {
if __t1 == 16198 {
break '__join_117;
} else {
if __t1 == 16199 {
break '__join_107;
} else {
if __t1 == 16200 {
break '__join_106;
} else {
if __t1 == 16201 {
break '__join_86;
} else {
if __t1 == 16202 {
break '__join_79;
} else {
if __t1 == 16203 {
break '__join_67;
} else {
if __t1 == 16204 {
break '__join_58;
} else {
if __t1 == 16205 {
if left == 0 {
break '__join_7;
} else {
let __v44: *mut u8 = put;
put = __v44.add(1);
*__v44 = (*state).length as u8;
left = left.wrapping_sub(1);
(*state).mode = 16200;
continue '__loop_319;
}
} else {
if __t1 == 16206 {
if (*state).wrap != 0 {
'__loop_28: loop {
if bits < 32 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v38: *mut u8 = next;
next = __v38.add(1);
hold = hold.wrapping_add((*__v38 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_28;
}
}
}
} else {
if true {
break;
}
}
}
out = out.wrapping_sub(left);
#[cfg(all(target_pointer_width = "64", unix))]
let __v28: u64 = (*strm).total_out;
#[cfg(any(target_pointer_width = "32", windows))]
let __v28: u32 = (*strm).total_out;
(*strm).total_out = __v28.wrapping_add(out as c_ulong);
#[cfg(all(target_pointer_width = "64", unix))]
let __v25: u64 = (*state).total;
#[cfg(any(target_pointer_width = "32", windows))]
let __v25: u32 = (*state).total;
(*state).total = __v25.wrapping_add(out as c_ulong);
if (*state).wrap & 4 != 0 && out != 0 {
#[cfg(all(target_pointer_width = "64", unix))]
let __v23: u64 = UPDATE_CHECK__2!(unsafe { (*state).check }, put.sub(out as usize), out, state);
#[cfg(any(target_pointer_width = "32", windows))]
let __v23: u32 = UPDATE_CHECK__2!(unsafe { (*state).check }, put.sub(out as usize), out, state);
(*state).check = __v23;
(*strm).adler = __v23;
}
out = left;
if (*state).wrap & 4 != 0 && (if (*state).flags != 0 { hold } else { ZSWAP32!(hold) }) != (*state).check {
(*strm).msg = c"incorrect data check".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
INITBITS!(hold, bits);
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:   check matches trailer\n".as_ptr() as *mut c_char);
}
}
}
(*state).mode = 16207;
// fallthrough
} else {
if __t1 != 16207 {
if __t1 == 16208 {
break '__join_11;
} else {
if __t1 == 16209 {
break '__join_10;
} else {
if __t1 == 16210 {
return -4;
} else {
if __t1 == 16211 {
break '__loop_319;
} else {
break '__loop_319;
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
                                                                                                                            if (*state).wrap != 0 && (*state).flags != 0 {
'__loop_17: loop {
if bits < 32 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v20: *mut u8 = next;
next = __v20.add(1);
hold = hold.wrapping_add((*__v20 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_17;
}
}
}
} else {
if true {
break;
}
}
}
if (*state).wrap & 4 != 0 && hold != (*state).total & 4294967295 {
(*strm).msg = c"incorrect length check".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
break '__join_13;
}
} else {
break '__join_12;
}
                                                                                                                        }
                                                                                                                        while (*state).have < (*state).ncode {
'__loop_154: loop {
if bits < 3 {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v204: *mut u8 = next;
next = __v204.add(1);
hold = hold.wrapping_add((*__v204 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_154;
}
}
}
} else {
if true {
break;
}
}
}
let __v196: u32 = (*state).have;
(*state).have = __v196.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).offset((order.0[__v196 as usize] as i32) as isize) = BITS!(3i32, hold) as u16;
DROPBITS!(3i32, hold, bits);
}
                                                                                                                        while (*state).have < 19 {
let __v193: u32 = (*state).have;
(*state).have = __v193.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).offset((order.0[__v193 as usize] as i32) as isize) = 0;
}
                                                                                                                        (*state).next = (*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28;
                                                                                                                        let __v191: *const __SlateRecord_inftrees_h_28 = (*state).next as *const __SlateRecord_inftrees_h_28;
                                                                                                                        (*state).distcode = __v191;
                                                                                                                        (*state).lencode = __v191;
                                                                                                                        (*state).lenbits = 7;
                                                                                                                        ret = inflate_table(0u32, (*state).lens.as_mut_ptr() as *mut u16, 19u32, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).lenbits), (*state).work.as_mut_ptr() as *mut u16);
                                                                                                                        if ret != 0 {
(*strm).msg = c"invalid code lengths set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       code lengths ok\n".as_ptr() as *mut c_char);
}
(*state).have = 0;
(*state).mode = 16198;
// fallthrough
}
                                                                                                                    }
                                                                                                                    '__join_116: {
                                                                                                                        '__join_124: {
                                                                                                                            '__loop_117: loop {
                                                                                                                                if (*state).have < unsafe { (*state).nlen }.wrapping_add((*state).ndist) {
'__loop_145: loop {
*__slate_slot_343 = *unsafe { (*state).lencode }.offset(BITS!(unsafe { (*state).lenbits }, hold) as isize);
if ((*__slate_slot_343).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v188: *mut u8 = next;
next = __v188.add(1);
hold = hold.wrapping_add((*__v188 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_145;
}
}
}
}
}
if ((*__slate_slot_343).val as i32) < 16 {
DROPBITS__2!((*__slate_slot_343).bits, hold, bits);
let __v148: u32 = (*state).have;
(*state).have = __v148.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).add(__v148 as usize) = (*__slate_slot_343).val;
} else {
if ((*__slate_slot_343).val as i32) == 16 {
'__loop_127: loop {
if bits < ((((*__slate_slot_343).bits as i32) + 2) as u32) {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v161: *mut u8 = next;
next = __v161.add(1);
hold = hold.wrapping_add((*__v161 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_127;
}
}
}
} else {
if true {
break;
}
}
}
DROPBITS__2!((*__slate_slot_343).bits, hold, bits);
if (*state).have == 0 {
break '__join_124;
} else {
len = *((*state).lens.as_mut_ptr() as *mut u16).offset(unsafe { (*state).have }.wrapping_sub(1) as isize) as u32;
copy = 3u32.wrapping_add(BITS!(2i32, hold));
DROPBITS!(2i32, hold, bits);
}
} else {
if ((*__slate_slot_343).val as i32) == 17 {
'__loop_132: loop {
if bits < ((((*__slate_slot_343).bits as i32) + 3) as u32) {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v170: *mut u8 = next;
next = __v170.add(1);
hold = hold.wrapping_add((*__v170 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_132;
}
}
}
} else {
if true {
break;
}
}
}
DROPBITS__2!((*__slate_slot_343).bits, hold, bits);
len = 0;
copy = 3u32.wrapping_add(BITS!(3i32, hold));
DROPBITS!(3i32, hold, bits);
} else {
'__loop_137: loop {
if bits < ((((*__slate_slot_343).bits as i32) + 7) as u32) {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v179: *mut u8 = next;
next = __v179.add(1);
hold = hold.wrapping_add((*__v179 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_137;
}
}
}
} else {
if true {
break;
}
}
}
DROPBITS__2!((*__slate_slot_343).bits, hold, bits);
len = 0;
copy = 11u32.wrapping_add(BITS!(7i32, hold));
DROPBITS!(7i32, hold, bits);
}
}
if unsafe { (*state).have }.wrapping_add(copy) > unsafe { (*state).nlen }.wrapping_add((*state).ndist) {
break;
} else {
loop {
let __v151: u32 = copy;
copy = __v151.wrapping_sub(1);
if __v151 != 0 {
let __v153: u32 = (*state).have;
(*state).have = __v153.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).add(__v153 as usize) = len as u16;
} else {
continue '__loop_117;
}
}
}
}
} else {
break '__join_116;
}
                                                                                                                            }
                                                                                                                            (*strm).msg = c"invalid bit length repeat".as_ptr() as *mut c_char;
                                                                                                                            (*state).mode = 16209;
                                                                                                                            break '__join_116;
                                                                                                                        }
                                                                                                                        (*strm).msg = c"invalid bit length repeat".as_ptr() as *mut c_char;
                                                                                                                        (*state).mode = 16209;
                                                                                                                    }
                                                                                                                    // handle error breaks in while
                                                                                                                    if ((*state).mode as u32) == 16209 {
continue '__loop_319;
} else {
// check for end-of-block code (better have one)
if (*((*state).lens.as_mut_ptr() as *mut u16).add(256) as i32) == 0 {
(*strm).msg = c"invalid code -- missing end-of-block".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
// build code tables -- note: do not change the lenbits or distbits
// values here (9 and 6) without reading the comments in inftrees.h
// concerning the ENOUGH constants, which depend on those values
(*state).next = (*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28;
(*state).lencode = (*state).next as *const __SlateRecord_inftrees_h_28;
(*state).lenbits = 9;
ret = inflate_table(1u32, (*state).lens.as_mut_ptr() as *mut u16, (*state).nlen, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).lenbits), (*state).work.as_mut_ptr() as *mut u16);
if ret != 0 {
(*strm).msg = c"invalid literal/lengths set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
(*state).distcode = (*state).next as *const __SlateRecord_inftrees_h_28;
(*state).distbits = 6;
ret = inflate_table(2u32, ((*state).lens.as_mut_ptr() as *mut u16).offset((*state).nlen as isize), (*state).ndist, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).distbits), (*state).work.as_mut_ptr() as *mut u16);
if ret != 0 {
(*strm).msg = c"invalid distances set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       codes ok\n".as_ptr() as *mut c_char);
}
(*state).mode = 16199;
if flush == 6 {
break '__join_7;
}
// fallthrough
}
}
}
}
                                                                                                                }
                                                                                                                (*state).mode = 16200;
                                                                                                                // fallthrough
                                                                                                            }
                                                                                                            if have >= 6 && left >= 258 {
RESTORE!(strm, put, left, next, have, state, hold, bits);
inflate_fast(strm, out);
LOAD!(put, strm, left, next, have, hold, state, bits);
if ((*state).mode as u32) == 16191 {
(*state).back = -1;
continue '__loop_319;
} else {
continue '__loop_319;
}
} else {
(*state).back = 0;
'__loop_102: loop {
*__slate_slot_343 = *unsafe { (*state).lencode }.offset(BITS!(unsafe { (*state).lenbits }, hold) as isize);
if ((*__slate_slot_343).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v144: *mut u8 = next;
next = __v144.add(1);
hold = hold.wrapping_add((*__v144 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_102;
}
}
}
}
}
if (*__slate_slot_343).op != 0 && ((*__slate_slot_343).op as i32) & 240 == 0 {
*__slate_slot_342 = *__slate_slot_343;
'__loop_97: loop {
*__slate_slot_343 = *unsafe { (*state).lencode }.offset((((*__slate_slot_342).val as i32) as u32).wrapping_add(BITS!(((*__slate_slot_342).bits as i32) + ((*__slate_slot_342).op as i32), hold) >> ((*__slate_slot_342).bits as i32)) as isize);
if ((((*__slate_slot_342).bits as i32) + ((*__slate_slot_343).bits as i32)) as u32) <= bits {
break;
} else {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v135: *mut u8 = next;
next = __v135.add(1);
hold = hold.wrapping_add((*__v135 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_97;
}
}
}
}
}
DROPBITS__2!((*__slate_slot_342).bits, hold, bits);
(*state).back = (*state).back + ((*__slate_slot_342).bits as i32);
}
DROPBITS__2!((*__slate_slot_343).bits, hold, bits);
(*state).back = (*state).back + ((*__slate_slot_343).bits as i32);
(*state).length = (*__slate_slot_343).val as u32;
if ((*__slate_slot_343).op as i32) == 0 {
#[cfg(feature = "zlib-debug")]
{
Tracevv__2!(slate_support::stderr(), if ((*__slate_slot_343).val as i32) >= 32 && ((*__slate_slot_343).val as i32) < 127 { c"inflate:         literal '%c'\n".as_ptr() as *mut c_char } else { c"inflate:         literal 0x%02x\n".as_ptr() as *mut c_char }, (*__slate_slot_343).val);
}
(*state).mode = 16205;
continue '__loop_319;
} else {
if ((*__slate_slot_343).op as i32) & 32 != 0 {
#[cfg(feature = "zlib-debug")]
{
Tracevv__3!(slate_support::stderr(), c"inflate:         end of block\n".as_ptr() as *mut c_char);
}
(*state).back = -1;
(*state).mode = 16191;
continue '__loop_319;
} else {
if ((*__slate_slot_343).op as i32) & 64 != 0 {
(*strm).msg = c"invalid literal/length code".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
(*state).extra = ((*__slate_slot_343).op as u32) & 15;
(*state).mode = 16201;
// fallthrough
}
}
}
}
                                                                                                        }
                                                                                                        if (*state).extra != 0 {
'__loop_83: loop {
if bits < ((*state).extra as u32) {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v120: *mut u8 = next;
next = __v120.add(1);
hold = hold.wrapping_add((*__v120 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_83;
}
}
}
} else {
if true {
break;
}
}
}
let __v112: u32 = (*state).length;
let __v111: u32 = __v112.wrapping_add(BITS!(unsafe { (*state).extra }, hold));
(*state).length = __v111;
DROPBITS!(unsafe { (*state).extra }, hold, bits);
(*state).back = ((*state).back as u32).wrapping_add((*state).extra) as i32;
}
                                                                                                        #[cfg(feature = "zlib-debug")]
{
Tracevv!(slate_support::stderr(), c"inflate:         length %u\n".as_ptr() as *mut c_char, unsafe { (*state).length });
}
                                                                                                        (*state).was = (*state).length;
                                                                                                        (*state).mode = 16202;
                                                                                                        // fallthrough
                                                                                                    }
                                                                                                    '__loop_79: loop {
                                                                                                        *__slate_slot_343 = *unsafe { (*state).distcode }.offset(BITS!(unsafe { (*state).distbits }, hold) as isize);
                                                                                                        if ((*__slate_slot_343).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v105: *mut u8 = next;
next = __v105.add(1);
hold = hold.wrapping_add((*__v105 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_79;
}
}
}
}
                                                                                                    }
                                                                                                    if ((*__slate_slot_343).op as i32) & 240 == 0 {
*__slate_slot_342 = *__slate_slot_343;
'__loop_74: loop {
*__slate_slot_343 = *unsafe { (*state).distcode }.offset((((*__slate_slot_342).val as i32) as u32).wrapping_add(BITS!(((*__slate_slot_342).bits as i32) + ((*__slate_slot_342).op as i32), hold) >> ((*__slate_slot_342).bits as i32)) as isize);
if ((((*__slate_slot_342).bits as i32) + ((*__slate_slot_343).bits as i32)) as u32) <= bits {
break;
} else {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v96: *mut u8 = next;
next = __v96.add(1);
hold = hold.wrapping_add((*__v96 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_74;
}
}
}
}
}
DROPBITS__2!((*__slate_slot_342).bits, hold, bits);
(*state).back = (*state).back + ((*__slate_slot_342).bits as i32);
}
                                                                                                    DROPBITS__2!((*__slate_slot_343).bits, hold, bits);
                                                                                                    (*state).back = (*state).back + ((*__slate_slot_343).bits as i32);
                                                                                                    if ((*__slate_slot_343).op as i32) & 64 != 0 {
(*strm).msg = c"invalid distance code".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
(*state).offset = (*__slate_slot_343).val as u32;
(*state).extra = ((*__slate_slot_343).op as u32) & 15;
(*state).mode = 16203;
// fallthrough
}
                                                                                                }
                                                                                                if (*state).extra != 0 {
'__loop_64: loop {
if bits < ((*state).extra as u32) {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v81: *mut u8 = next;
next = __v81.add(1);
hold = hold.wrapping_add((*__v81 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_64;
}
}
}
} else {
if true {
break;
}
}
}
let __v73: u32 = (*state).offset;
let __v72: u32 = __v73.wrapping_add(BITS!(unsafe { (*state).extra }, hold));
(*state).offset = __v72;
DROPBITS!(unsafe { (*state).extra }, hold, bits);
(*state).back = ((*state).back as u32).wrapping_add((*state).extra) as i32;
}
                                                                                            }
                                                                                            #[cfg(all(not(feature = "no-gzip"), feature = "inflate-strict"))]
                                                                                            {
                                                                                                if (*state).offset > (*state).dmax {
(*strm).msg = c"invalid distance too far back".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracevv!(slate_support::stderr(), c"inflate:         distance %u\n".as_ptr() as *mut c_char, unsafe { (*state).offset });
}
(*state).mode = 16204;
// fallthrough
}
                                                                                            }
                                                                                            #[cfg(all(not(feature = "no-gzip"), feature = "zlib-debug", not(feature = "inflate-strict")))]
                                                                                            {
                                                                                                Tracevv!(slate_support::stderr(), c"inflate:         distance %u\n".as_ptr() as *mut c_char, unsafe { (*state).offset });
                                                                                            }
                                                                                            #[cfg(all(not(feature = "no-gzip"), not(feature = "inflate-strict")))]
                                                                                            {
                                                                                                (*state).mode = 16204;
                                                                                            }
                                                                                            #[cfg(feature = "no-gzip")]
                                                                                            {
                                                                                                '__loop_108: loop {
                                                                                                    if (*state).have < unsafe { (*state).nlen }.wrapping_add((*state).ndist) {
'__loop_136: loop {
*__slate_slot_343 = *unsafe { (*state).lencode }.offset(BITS!(unsafe { (*state).lenbits }, hold) as isize);
if ((*__slate_slot_343).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v6: *mut u8 = next;
next = __v6.add(1);
hold = hold.wrapping_add((*__v6 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_136;
}
}
}
}
}
if ((*__slate_slot_343).val as i32) < 16 {
DROPBITS__2!((*__slate_slot_343).bits, hold, bits);
let __v139: u32 = (*state).have;
(*state).have = __v139.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).add(__v139 as usize) = (*__slate_slot_343).val;
} else {
if ((*__slate_slot_343).val as i32) == 16 {
'__loop_118: loop {
if bits < ((((*__slate_slot_343).bits as i32) + 2) as u32) {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v152: *mut u8 = next;
next = __v152.add(1);
hold = hold.wrapping_add((*__v152 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_118;
}
}
}
} else {
if true {
break;
}
}
}
DROPBITS__2!((*__slate_slot_343).bits, hold, bits);
if (*state).have == 0 {
break '__join_58;
} else {
len = *((*state).lens.as_mut_ptr() as *mut u16).offset(unsafe { (*state).have }.wrapping_sub(1) as isize) as u32;
copy = 3u32.wrapping_add(BITS!(2i32, hold));
DROPBITS!(2i32, hold, bits);
}
} else {
if ((*__slate_slot_343).val as i32) == 17 {
'__loop_123: loop {
if bits < ((((*__slate_slot_343).bits as i32) + 3) as u32) {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v7: *mut u8 = next;
next = __v7.add(1);
hold = hold.wrapping_add((*__v7 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_123;
}
}
}
} else {
if true {
break;
}
}
}
DROPBITS__2!((*__slate_slot_343).bits, hold, bits);
len = 0;
copy = 3u32.wrapping_add(BITS!(3i32, hold));
DROPBITS!(3i32, hold, bits);
} else {
'__loop_128: loop {
if bits < ((((*__slate_slot_343).bits as i32) + 7) as u32) {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v9: *mut u8 = next;
next = __v9.add(1);
hold = hold.wrapping_add((*__v9 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_128;
}
}
}
} else {
if true {
break;
}
}
}
DROPBITS__2!((*__slate_slot_343).bits, hold, bits);
len = 0;
copy = 11u32.wrapping_add(BITS!(7i32, hold));
DROPBITS!(7i32, hold, bits);
}
}
if unsafe { (*state).have }.wrapping_add(copy) > unsafe { (*state).nlen }.wrapping_add((*state).ndist) {
break;
} else {
loop {
let __v142: u32 = copy;
copy = __v142.wrapping_sub(1);
if __v142 != 0 {
let __v10: u32 = (*state).have;
(*state).have = __v10.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).add(__v10 as usize) = len as u16;
} else {
continue '__loop_108;
}
}
}
}
} else {
break '__join_172;
}
                                                                                                }
                                                                                                (*strm).msg = c"invalid bit length repeat".as_ptr() as *mut c_char;
                                                                                                (*state).mode = 16209;
                                                                                                break '__join_172;
                                                                                            }
                                                                                        }
                                                                                        #[cfg(not(feature = "no-gzip"))]
                                                                                        {
                                                                                            if left
                                                                                                == 0
                                                                                            {
                                                                                                break '__join_7;
                                                                                            } else {
                                                                                                copy = out.wrapping_sub(left);
                                                                                                if (*state).offset > copy {
// copy from window
copy = unsafe { (*state).offset }.wrapping_sub(copy);
if copy > (*state).whave {
if (*state).sane != 0 {
(*strm).msg = c"invalid distance too far back".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
#[cfg(all(feature = "zlib-debug", feature = "inflate-allow-invalid"))]
{
Trace!(slate_support::stderr(), c"inflate.c too far\n".as_ptr() as *mut c_char);
}
#[cfg(feature = "inflate-allow-invalid")]
{
copy = copy.wrapping_sub((*state).whave);
if copy > (*state).length {
copy = (*state).length;
}
if copy > left {
copy = left;
}
left = left.wrapping_sub(copy);
}
#[cfg(feature = "inflate-allow-invalid")]
let __v63: u32 = (*state).length;
#[cfg(feature = "inflate-allow-invalid")]
{
(*state).length = __v63.wrapping_sub(copy);
loop {
let __v61: *mut u8 = put;
put = __v61.add(1);
*__v61 = 0;
let __v58: u32 = copy.wrapping_sub(1);
copy = __v58;
if __v58 == 0 {
break;
}
}
if (*state).length == 0 {
(*state).mode = 16200;
continue '__loop_319;
} else {
continue '__loop_319;
}
}
}
} else {
#[cfg(feature = "inflate-allow-invalid")]
{
if copy > (*state).wnext {
copy = copy.wrapping_sub((*state).wnext);
from = unsafe { (*state).window }.offset(unsafe { (*state).wsize }.wrapping_sub(copy) as isize);
} else {
from = unsafe { (*state).window }.offset(unsafe { (*state).wnext }.wrapping_sub(copy) as isize);
}
if copy > (*state).length {
copy = (*state).length;
}
}
}
#[cfg(not(feature = "inflate-allow-invalid"))]
{
if copy > (*state).wnext {
copy = copy.wrapping_sub((*state).wnext);
from = unsafe { (*state).window }.offset(unsafe { (*state).wsize }.wrapping_sub(copy) as isize);
} else {
from = unsafe { (*state).window }.offset(unsafe { (*state).wnext }.wrapping_sub(copy) as isize);
}
if copy > (*state).length {
copy = (*state).length;
}
}
} else {
// copy from output
from = put.offset(-((*state).offset as isize));
copy = (*state).length;
}
                                                                                                if copy > left {
copy = left;
}
                                                                                                left = left.wrapping_sub(copy);
                                                                                                let __v52: u32 = (*state).length;
                                                                                                (*state).length = __v52.wrapping_sub(copy);
                                                                                                loop {
                                                                                                    let __v50: *mut u8 = from;
                                                                                                    from = __v50.add(1);
                                                                                                    let __v48: *mut u8 = put;
                                                                                                    put = __v48.add(1);
                                                                                                    *__v48 = *__v50;
                                                                                                    let __v45: u32 = copy.wrapping_sub(1);
                                                                                                    copy = __v45;
                                                                                                    if __v45 == 0 {
break;
}
                                                                                                }
                                                                                                if (*state).length == 0 {
(*state).mode = 16200;
continue '__loop_319;
} else {
continue '__loop_319;
}
                                                                                            }
                                                                                        }
                                                                                        #[cfg(feature = "no-gzip")]
                                                                                        {
                                                                                            (*strm).msg = c"invalid bit length repeat".as_ptr() as *mut c_char;
                                                                                            (*state).mode = 16209;
                                                                                        }
                                                                                    }
                                                                                    #[cfg(not(
                                                                                        feature = "no-gzip"
                                                                                    ))]
                                                                                    {
                                                                                        (*state)
                                                                                            .mode =
                                                                                            16195;
                                                                                    }
                                                                                    #[cfg(
                                                                                        feature = "no-gzip"
                                                                                    )]
                                                                                    {
                                                                                        if ((*state)
                                                                                            .mode
                                                                                            as u32)
                                                                                            == 16209
                                                                                        {
                                                                                            continue '__loop_319;
                                                                                        } else {
                                                                                            // check for end-of-block code (better have one)
                                                                                            if (*((*state).lens.as_mut_ptr() as *mut u16).add(256) as i32) == 0 {
(*strm).msg = c"invalid code -- missing end-of-block".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
// build code tables -- note: do not change the lenbits or distbits
// values here (9 and 6) without reading the comments in inftrees.h
// concerning the ENOUGH constants, which depend on those values
(*state).next = (*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28;
(*state).lencode = (*state).next as *const __SlateRecord_inftrees_h_28;
(*state).lenbits = 9;
ret = inflate_table(1u32, (*state).lens.as_mut_ptr() as *mut u16, (*state).nlen, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).lenbits), (*state).work.as_mut_ptr() as *mut u16);
if ret != 0 {
(*strm).msg = c"invalid literal/lengths set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
(*state).distcode = (*state).next as *const __SlateRecord_inftrees_h_28;
(*state).distbits = 6;
ret = inflate_table(2u32, ((*state).lens.as_mut_ptr() as *mut u16).offset((*state).nlen as isize), (*state).ndist, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).distbits), (*state).work.as_mut_ptr() as *mut u16);
if ret != 0 {
(*strm).msg = c"invalid distances set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       codes ok\n".as_ptr() as *mut c_char);
}
(*state).mode = 16199;
if flush == 6 {
break '__join_7;
}
// fallthrough
}
}
}
                                                                                        }
                                                                                    }
                                                                                }
                                                                                #[cfg(not(
                                                                                    feature = "no-gzip"
                                                                                ))]
                                                                                {
                                                                                    copy = (*state)
                                                                                        .length;
                                                                                    if copy != 0 {
                                                                                        if copy
                                                                                            > have
                                                                                        {
                                                                                            copy = have;
                                                                                        }
                                                                                        if copy
                                                                                            > left
                                                                                        {
                                                                                            copy = left;
                                                                                        }
                                                                                        if copy == 0
                                                                                        {
                                                                                            break '__join_7;
                                                                                        } else {
                                                                                            #[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
                                                                                            {
                                                                                                std::ptr::copy_nonoverlapping(((next as *const ()) as *const core::ffi::c_void) as *const u8, ((put as *mut ()) as *mut core::ffi::c_void) as *mut u8, ((copy as size_t) as usize) as usize);
                                                                                            }
                                                                                            #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
                                                                                            {
                                                                                                std::ptr::copy_nonoverlapping(next as *const u8, put as *mut u8, (copy as size_t) as usize);
                                                                                            }
                                                                                            have = have.wrapping_sub(copy);
                                                                                            let __v224: *mut u8 = next;
                                                                                            next = __v224.add(copy as usize);
                                                                                            left = left.wrapping_sub(copy);
                                                                                            let __v220: *mut u8 = put;
                                                                                            put = __v220.add(copy as usize);
                                                                                            let __v217: u32 = (*state).length;
                                                                                            (*state).length = __v217.wrapping_sub(copy);
                                                                                            continue '__loop_319;
                                                                                        }
                                                                                    } else {
                                                                                        #[cfg(feature = "zlib-debug")]
                                                                                        {
                                                                                            Tracev!(slate_support::stderr(), c"inflate:       stored end\n".as_ptr() as *mut c_char);
                                                                                        }
                                                                                        (*state)
                                                                                            .mode =
                                                                                            16191;
                                                                                        continue '__loop_319;
                                                                                    }
                                                                                }
                                                                                #[cfg(
                                                                                    feature = "no-gzip"
                                                                                )]
                                                                                {
                                                                                    (*state).mode =
                                                                                        16200;
                                                                                }
                                                                            }
                                                                            #[cfg(not(
                                                                                feature = "no-gzip"
                                                                            ))]
                                                                            {
                                                                                if (*state).havedict
                                                                                    == 0
                                                                                {
                                                                                    RESTORE!(
                                                                                        strm, put,
                                                                                        left, next,
                                                                                        have,
                                                                                        state,
                                                                                        hold, bits
                                                                                    );
                                                                                    return 2;
                                                                                } else {
                                                                                    #[cfg(all(target_pointer_width = "64", unix))]
let __v245: u64 = adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
                                                                                    #[cfg(any(target_pointer_width = "32", windows))]
let __v245: u32 = adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
                                                                                    (*state)
                                                                                        .check =
                                                                                        __v245;
                                                                                    (*strm).adler =
                                                                                        __v245;
                                                                                    (*state).mode =
                                                                                        16191;
                                                                                    // fallthrough
                                                                                }
                                                                            }
                                                                            #[cfg(
                                                                                feature = "no-gzip"
                                                                            )]
                                                                            {
                                                                                if have >= 6
                                                                                    && left >= 258
                                                                                {
                                                                                    RESTORE!(
                                                                                        strm, put,
                                                                                        left, next,
                                                                                        have,
                                                                                        state,
                                                                                        hold, bits
                                                                                    );
                                                                                    inflate_fast(
                                                                                        strm, out,
                                                                                    );
                                                                                    LOAD!(
                                                                                        put, strm,
                                                                                        left, next,
                                                                                        have, hold,
                                                                                        state,
                                                                                        bits
                                                                                    );
                                                                                    if ((*state)
                                                                                        .mode
                                                                                        as u32)
                                                                                        == 16191
                                                                                    {
                                                                                        (*state)
                                                                                            .back =
                                                                                            -1;
                                                                                        continue '__loop_319;
                                                                                    } else {
                                                                                        continue '__loop_319;
                                                                                    }
                                                                                } else {
                                                                                    (*state).back =
                                                                                        0;
                                                                                    '__loop_93: loop {
                                                                                        *__slate_slot_343 = *unsafe { (*state).lencode }.offset(BITS!(unsafe { (*state).lenbits }, hold) as isize);
                                                                                        if ((*__slate_slot_343).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v11: *mut u8 = next;
next = __v11.add(1);
hold = hold.wrapping_add((*__v11 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_93;
}
}
}
}
                                                                                    }
                                                                                    if (*__slate_slot_343).op != 0 && ((*__slate_slot_343).op as i32) & 240 == 0 {
*__slate_slot_342 = *__slate_slot_343;
'__loop_88: loop {
*__slate_slot_343 = *unsafe { (*state).lencode }.offset((((*__slate_slot_342).val as i32) as u32).wrapping_add(BITS!(((*__slate_slot_342).bits as i32) + ((*__slate_slot_342).op as i32), hold) >> ((*__slate_slot_342).bits as i32)) as isize);
if ((((*__slate_slot_342).bits as i32) + ((*__slate_slot_343).bits as i32)) as u32) <= bits {
break;
} else {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v126: *mut u8 = next;
next = __v126.add(1);
hold = hold.wrapping_add((*__v126 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_88;
}
}
}
}
}
DROPBITS__2!((*__slate_slot_342).bits, hold, bits);
(*state).back = (*state).back + ((*__slate_slot_342).bits as i32);
}
                                                                                    DROPBITS__2!((*__slate_slot_343).bits, hold, bits);
                                                                                    (*state).back = (*state).back + ((*__slate_slot_343).bits as i32);
                                                                                    (*state).length = (*__slate_slot_343).val as u32;
                                                                                    if ((*__slate_slot_343).op as i32) == 0 {
#[cfg(feature = "zlib-debug")]
{
Tracevv__2!(slate_support::stderr(), if ((*__slate_slot_343).val as i32) >= 32 && ((*__slate_slot_343).val as i32) < 127 { c"inflate:         literal '%c'\n".as_ptr() as *mut c_char } else { c"inflate:         literal 0x%02x\n".as_ptr() as *mut c_char }, (*__slate_slot_343).val);
}
(*state).mode = 16205;
continue '__loop_319;
} else {
if ((*__slate_slot_343).op as i32) & 32 != 0 {
#[cfg(feature = "zlib-debug")]
{
Tracevv__3!(slate_support::stderr(), c"inflate:         end of block\n".as_ptr() as *mut c_char);
}
(*state).back = -1;
(*state).mode = 16191;
continue '__loop_319;
} else {
if ((*__slate_slot_343).op as i32) & 64 != 0 {
(*strm).msg = c"invalid literal/length code".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_319;
} else {
(*state).extra = ((*__slate_slot_343).op as u32) & 15;
(*state).mode = 16201;
// fallthrough
}
}
}
                                                                                }
                                                                            }
                                                                        }
                                                                        #[cfg(not(
                                                                            feature = "no-gzip"
                                                                        ))]
                                                                        {
                                                                            if flush == 5
                                                                                || flush == 6
                                                                            {
                                                                                break '__join_7;
                                                                            }
                                                                        }
                                                                        #[cfg(feature = "no-gzip")]
                                                                        {
                                                                            if (*state).extra != 0 {
                                                                                '__loop_12: loop {
                                                                                    if bits
                                                                                        < ((*state)
                                                                                            .extra
                                                                                            as u32)
                                                                                    {
                                                                                        loop {
                                                                                            if have
                                                                                                == 0
                                                                                            {
                                                                                                break '__join_7;
                                                                                            } else {
                                                                                                have = have.wrapping_sub(1);
                                                                                                let __v13: *mut u8 = next;
                                                                                                next = __v13.add(1);
                                                                                                hold = hold.wrapping_add((*__v13 as c_ulong) << bits);
                                                                                                bits = bits.wrapping_add(8);
                                                                                                if true {
continue '__loop_12;
}
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        if true {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                let __v103: u32 =
                                                                                    (*state).length;
                                                                                let __v102: u32 = __v103.wrapping_add(BITS!(unsafe { (*state).extra }, hold));
                                                                                (*state).length =
                                                                                    __v102;
                                                                                DROPBITS!(
                                                                                    unsafe {
                                                                                        (*state)
                                                                                            .extra
                                                                                    },
                                                                                    hold,
                                                                                    bits
                                                                                );
                                                                                (*state).back = ((*state).back as u32).wrapping_add((*state).extra) as i32;
                                                                            }
                                                                        }
                                                                        #[cfg(all(
                                                                            feature = "no-gzip",
                                                                            feature = "zlib-debug"
                                                                        ))]
                                                                        {
                                                                            Tracevv!(slate_support::stderr(), c"inflate:         length %u\n".as_ptr() as *mut c_char, unsafe { (*state).length });
                                                                        }
                                                                        #[cfg(feature = "no-gzip")]
                                                                        {
                                                                            (*state).was =
                                                                                (*state).length;
                                                                            (*state).mode = 16202;
                                                                        }
                                                                    }
                                                                    #[cfg(not(
                                                                        feature = "no-gzip"
                                                                    ))]
                                                                    {
                                                                        if (*state).last != 0 {
                                                                            BYTEBITS!(hold, bits);
                                                                            (*state).mode = 16206;
                                                                            continue '__loop_319;
                                                                        } else {
                                                                            '__loop_190: loop {
                                                                                if bits < 3 {
                                                                                    loop {
                                                                                        if have == 0
                                                                                        {
                                                                                            break '__join_7;
                                                                                        } else {
                                                                                            have = have.wrapping_sub(1);
                                                                                            let __v242: *mut u8 = next;
                                                                                            next = __v242.add(1);
                                                                                            hold = hold.wrapping_add((*__v242 as c_ulong) << bits);
                                                                                            bits = bits.wrapping_add(8);
                                                                                            if true
                                                                                            {
                                                                                                continue '__loop_190;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    if true {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                            }
                                                                            (*state).last =
                                                                                BITS!(1i32, hold)
                                                                                    as i32;
                                                                            DROPBITS!(
                                                                                1i32, hold, bits
                                                                            );
                                                                            let __t0: u32 =
                                                                                BITS!(2i32, hold);
                                                                            if __t0 == 0 {
                                                                                #[cfg(
                                                                                    feature = "zlib-debug"
                                                                                )]
                                                                                {
                                                                                    Tracev__2!(slate_support::stderr(), c"inflate:     stored block%s\n".as_ptr() as *mut c_char, if (*state).last != 0 { c" (last)".as_ptr() as *mut c_char } else { c"".as_ptr() as *mut c_char });
                                                                                }
                                                                                (*state).mode =
                                                                                    16193;
                                                                            } else {
                                                                                if __t0 == 1 {
                                                                                    inflate_fixed(
                                                                                        state,
                                                                                    ); // fixed block
                                                                                    #[cfg(
                                                                                        feature = "zlib-debug"
                                                                                    )]
                                                                                    {
                                                                                        Tracev__2!(slate_support::stderr(), c"inflate:     fixed codes block%s\n".as_ptr() as *mut c_char, if (*state).last != 0 { c" (last)".as_ptr() as *mut c_char } else { c"".as_ptr() as *mut c_char });
                                                                                    }
                                                                                    (*state).mode =
                                                                                        16199;
                                                                                    // decode codes
                                                                                    if flush == 6 {
                                                                                        break '__join_185;
                                                                                    }
                                                                                } else {
                                                                                    if __t0 == 2 {
                                                                                        #[cfg(feature = "zlib-debug")]
                                                                                        {
                                                                                            Tracev__2!(slate_support::stderr(), c"inflate:     dynamic codes block%s\n".as_ptr() as *mut c_char, if (*state).last != 0 { c" (last)".as_ptr() as *mut c_char } else { c"".as_ptr() as *mut c_char });
                                                                                        }
                                                                                        (*state)
                                                                                            .mode =
                                                                                            16196;
                                                                                    } else {
                                                                                        (*strm).msg = c"invalid block type".as_ptr() as *mut c_char;
                                                                                        (*state)
                                                                                            .mode =
                                                                                            16209;
                                                                                    }
                                                                                }
                                                                            }
                                                                            DROPBITS!(
                                                                                2i32, hold, bits
                                                                            );
                                                                            continue '__loop_319;
                                                                        }
                                                                    }
                                                                    #[cfg(feature = "no-gzip")]
                                                                    {
                                                                        '__loop_70: loop {
                                                                            *__slate_slot_343 = *unsafe { (*state).distcode }.offset(BITS!(unsafe { (*state).distbits }, hold) as isize);
                                                                            if ((*__slate_slot_343)
                                                                                .bits
                                                                                as u32)
                                                                                <= bits
                                                                            {
                                                                                break;
                                                                            } else {
                                                                                loop {
                                                                                    if have == 0 {
                                                                                        break '__join_7;
                                                                                    } else {
                                                                                        have = have.wrapping_sub(1);
                                                                                        let __v15: *mut u8 = next;
                                                                                        next = __v15.add(1);
                                                                                        hold = hold.wrapping_add((*__v15 as c_ulong) << bits);
                                                                                        bits = bits.wrapping_add(8);
                                                                                        if true {
                                                                                            continue '__loop_70;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        if ((*__slate_slot_343).op
                                                                            as i32)
                                                                            & 240
                                                                            == 0
                                                                        {
                                                                            *__slate_slot_342 =
                                                                                *__slate_slot_343;
                                                                            '__loop_65: loop {
                                                                                *__slate_slot_343 = *unsafe { (*state).distcode }.offset((((*__slate_slot_342).val as i32) as u32).wrapping_add(BITS!(((*__slate_slot_342).bits as i32) + ((*__slate_slot_342).op as i32), hold) >> ((*__slate_slot_342).bits as i32)) as isize);
                                                                                if ((((*__slate_slot_342).bits as i32) + ((*__slate_slot_343).bits as i32)) as u32) <= bits {
break;
} else {
loop {
if have == 0 {
break '__join_7;
} else {
have = have.wrapping_sub(1);
let __v87: *mut u8 = next;
next = __v87.add(1);
hold = hold.wrapping_add((*__v87 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_65;
}
}
}
}
                                                                            }
                                                                            DROPBITS__2!(
                                                                                (*__slate_slot_342)
                                                                                    .bits,
                                                                                hold,
                                                                                bits
                                                                            );
                                                                            (*state).back = (*state).back + ((*__slate_slot_342).bits as i32);
                                                                        }
                                                                        DROPBITS__2!(
                                                                            (*__slate_slot_343)
                                                                                .bits,
                                                                            hold,
                                                                            bits
                                                                        );
                                                                        (*state).back = (*state)
                                                                            .back
                                                                            + ((*__slate_slot_343)
                                                                                .bits
                                                                                as i32);
                                                                        if ((*__slate_slot_343).op
                                                                            as i32)
                                                                            & 64
                                                                            != 0
                                                                        {
                                                                            (*strm).msg = c"invalid distance code".as_ptr() as *mut c_char;
                                                                            (*state).mode = 16209;
                                                                            continue '__loop_319;
                                                                        } else {
                                                                            (*state).offset =
                                                                                (*__slate_slot_343)
                                                                                    .val
                                                                                    as u32;
                                                                            (*state).extra =
                                                                                ((*__slate_slot_343)
                                                                                    .op
                                                                                    as u32)
                                                                                    & 15;
                                                                            (*state).mode = 16203;
                                                                            // fallthrough
                                                                        }
                                                                    }
                                                                }
                                                                #[cfg(not(feature = "no-gzip"))]
                                                                {
                                                                    '__loop_282: loop {
                                                                        if bits < 32 {
                                                                            loop {
                                                                                if have == 0 {
                                                                                    break '__join_7;
                                                                                } else {
                                                                                    have = have.wrapping_sub(1);
                                                                                    let __v317: *mut u8 = next;
                                                                                    next = __v317
                                                                                        .add(1);
                                                                                    hold = hold.wrapping_add((*__v317 as c_ulong) << bits);
                                                                                    bits = bits.wrapping_add(8);
                                                                                    if true {
                                                                                        continue '__loop_282;
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if true {
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                    if !unsafe { (*state).head }
                                                                        .is_null()
                                                                    {
                                                                        (*(*state).head).time =
                                                                            hold;
                                                                    }
                                                                    if (*state).flags & 512 != 0
                                                                        && (*state).wrap & 4 != 0
                                                                    {
                                                                        CRC4!(
                                                                            (*state).check,
                                                                            hold,
                                                                            hbuf
                                                                        );
                                                                    }
                                                                    INITBITS!(hold, bits);
                                                                    (*state).mode = 16183;
                                                                }
                                                                #[cfg(feature = "no-gzip")]
                                                                {
                                                                    if (*state).extra != 0 {
                                                                        '__loop_55: loop {
                                                                            if bits
                                                                                < ((*state).extra
                                                                                    as u32)
                                                                            {
                                                                                loop {
                                                                                    if have == 0 {
                                                                                        break '__join_7;
                                                                                    } else {
                                                                                        have = have.wrapping_sub(1);
                                                                                        let __v17: *mut u8 = next;
                                                                                        next = __v17.add(1);
                                                                                        hold = hold.wrapping_add((*__v17 as c_ulong) << bits);
                                                                                        bits = bits.wrapping_add(8);
                                                                                        if true {
                                                                                            continue '__loop_55;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                if true {
                                                                                    break;
                                                                                }
                                                                            }
                                                                        }
                                                                        let __v64: u32 =
                                                                            (*state).offset;
                                                                        let __v18: u32 = __v64
                                                                            .wrapping_add(BITS!(
                                                                                unsafe {
                                                                                    (*state).extra
                                                                                },
                                                                                hold
                                                                            ));
                                                                        (*state).offset = __v18;
                                                                        DROPBITS!(
                                                                            unsafe {
                                                                                (*state).extra
                                                                            },
                                                                            hold,
                                                                            bits
                                                                        );
                                                                        (*state).back =
                                                                            ((*state).back as u32)
                                                                                .wrapping_add(
                                                                                    (*state).extra,
                                                                                )
                                                                                as i32;
                                                                    }
                                                                }
                                                                #[cfg(all(
                                                                    feature = "no-gzip",
                                                                    feature = "inflate-strict"
                                                                ))]
                                                                {
                                                                    if (*state).offset
                                                                        > (*state).dmax
                                                                    {
                                                                        (*strm).msg = c"invalid distance too far back".as_ptr() as *mut c_char;
                                                                        (*state).mode = 16209;
                                                                        continue '__loop_319;
                                                                    } else {
                                                                        #[cfg(
                                                                            feature = "zlib-debug"
                                                                        )]
                                                                        {
                                                                            Tracevv!(slate_support::stderr(), c"inflate:         distance %u\n".as_ptr() as *mut c_char, unsafe { (*state).offset });
                                                                        }
                                                                        (*state).mode = 16204;
                                                                        // fallthrough
                                                                    }
                                                                }
                                                                #[cfg(all(
                                                                    feature = "no-gzip",
                                                                    feature = "zlib-debug",
                                                                    not(
                                                                        feature = "inflate-strict"
                                                                    )
                                                                ))]
                                                                {
                                                                    Tracevv!(slate_support::stderr(), c"inflate:         distance %u\n".as_ptr() as *mut c_char, unsafe { (*state).offset });
                                                                }
                                                                #[cfg(all(
                                                                    feature = "no-gzip",
                                                                    not(
                                                                        feature = "inflate-strict"
                                                                    )
                                                                ))]
                                                                {
                                                                    (*state).mode = 16204;
                                                                }
                                                            }
                                                            #[cfg(not(feature = "no-gzip"))]
                                                            {
                                                                '__loop_273: loop {
                                                                    if bits < 16 {
                                                                        loop {
                                                                            if have == 0 {
                                                                                break '__join_7;
                                                                            } else {
                                                                                have = have
                                                                                    .wrapping_sub(
                                                                                        1,
                                                                                    );
                                                                                let __v308: *mut u8 = next;
                                                                                next =
                                                                                    __v308.add(1);
                                                                                hold = hold
                                                                                    .wrapping_add(
                                                                                    (*__v308
                                                                                        as c_ulong)
                                                                                        << bits,
                                                                                );
                                                                                bits = bits
                                                                                    .wrapping_add(
                                                                                        8,
                                                                                    );
                                                                                if true {
                                                                                    continue '__loop_273;
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if true {
                                                                            break;
                                                                        }
                                                                    }
                                                                }
                                                                if !unsafe { (*state).head }
                                                                    .is_null()
                                                                {
                                                                    (*(*state).head).xflags =
                                                                        (hold & 255) as i32;
                                                                    (*(*state).head).os =
                                                                        (hold >> 8i32) as i32;
                                                                }
                                                                if (*state).flags & 512 != 0
                                                                    && (*state).wrap & 4 != 0
                                                                {
                                                                    CRC2!(
                                                                        (*state).check,
                                                                        hold,
                                                                        hbuf
                                                                    );
                                                                }
                                                                INITBITS!(hold, bits);
                                                                (*state).mode = 16184;
                                                            }
                                                            #[cfg(feature = "no-gzip")]
                                                            {
                                                                if left == 0 {
                                                                    break '__join_7;
                                                                } else {
                                                                    copy = out.wrapping_sub(left);
                                                                    if (*state).offset > copy {
                                                                        // copy from window
                                                                        copy = unsafe {
                                                                            (*state).offset
                                                                        }
                                                                        .wrapping_sub(copy);
                                                                        if copy > (*state).whave {
                                                                            if (*state).sane != 0 {
                                                                                (*strm).msg = c"invalid distance too far back".as_ptr() as *mut c_char;
                                                                                (*state).mode =
                                                                                    16209;
                                                                                continue '__loop_319;
                                                                            } else {
                                                                                #[cfg(all(
                                                                                    feature = "zlib-debug",
                                                                                    feature = "inflate-allow-invalid"
                                                                                ))]
                                                                                {
                                                                                    Trace!(slate_support::stderr(), c"inflate.c too far\n".as_ptr() as *mut c_char);
                                                                                }
                                                                                #[cfg(
                                                                                    feature = "inflate-allow-invalid"
                                                                                )]
                                                                                {
                                                                                    copy = copy.wrapping_sub((*state).whave);
                                                                                    if copy
                                                                                        > (*state)
                                                                                            .length
                                                                                    {
                                                                                        copy = (*state).length;
                                                                                    }
                                                                                    if copy > left {
                                                                                        copy = left;
                                                                                    }
                                                                                    left = left.wrapping_sub(copy);
                                                                                }
                                                                                #[cfg(feature = "inflate-allow-invalid")]
let __v54: u32 = (*state).length;
                                                                                #[cfg(
                                                                                    feature = "inflate-allow-invalid"
                                                                                )]
                                                                                {
                                                                                    (*state).length = __v54.wrapping_sub(copy);
                                                                                    loop {
                                                                                        let __v21: *mut u8 = put;
                                                                                        put = __v21
                                                                                            .add(1);
                                                                                        *__v21 = 0;
                                                                                        let __v49: u32 = copy.wrapping_sub(1);
                                                                                        copy =
                                                                                            __v49;
                                                                                        if __v49
                                                                                            == 0
                                                                                        {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    if (*state)
                                                                                        .length
                                                                                        == 0
                                                                                    {
                                                                                        (*state)
                                                                                            .mode =
                                                                                            16200;
                                                                                        continue '__loop_319;
                                                                                    } else {
                                                                                        continue '__loop_319;
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            #[cfg(
                                                                                feature = "inflate-allow-invalid"
                                                                            )]
                                                                            {
                                                                                if copy
                                                                                    > (*state).wnext
                                                                                {
                                                                                    copy = copy.wrapping_sub((*state).wnext);
                                                                                    from = unsafe { (*state).window }.offset(unsafe { (*state).wsize }.wrapping_sub(copy) as isize);
                                                                                } else {
                                                                                    from = unsafe { (*state).window }.offset(unsafe { (*state).wnext }.wrapping_sub(copy) as isize);
                                                                                }
                                                                                if copy
                                                                                    > (*state)
                                                                                        .length
                                                                                {
                                                                                    copy = (*state)
                                                                                        .length;
                                                                                }
                                                                            }
                                                                        }
                                                                        #[cfg(not(
                                                                            feature = "inflate-allow-invalid"
                                                                        ))]
                                                                        {
                                                                            if copy > (*state).wnext
                                                                            {
                                                                                copy = copy
                                                                                    .wrapping_sub(
                                                                                        (*state)
                                                                                            .wnext,
                                                                                    );
                                                                                from = unsafe {
                                                                                    (*state).window
                                                                                }
                                                                                .offset(
                                                                                    unsafe {
                                                                                        (*state)
                                                                                            .wsize
                                                                                    }
                                                                                    .wrapping_sub(
                                                                                        copy,
                                                                                    )
                                                                                        as isize,
                                                                                );
                                                                            } else {
                                                                                from = unsafe {
                                                                                    (*state).window
                                                                                }
                                                                                .offset(
                                                                                    unsafe {
                                                                                        (*state)
                                                                                            .wnext
                                                                                    }
                                                                                    .wrapping_sub(
                                                                                        copy,
                                                                                    )
                                                                                        as isize,
                                                                                );
                                                                            }
                                                                            if copy
                                                                                > (*state).length
                                                                            {
                                                                                copy =
                                                                                    (*state).length;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        // copy from output
                                                                        from = put.offset(
                                                                            -((*state).offset
                                                                                as isize),
                                                                        );
                                                                        copy = (*state).length;
                                                                    }
                                                                    if copy > left {
                                                                        copy = left;
                                                                    }
                                                                    left = left.wrapping_sub(copy);
                                                                    let __v43: u32 =
                                                                        (*state).length;
                                                                    (*state).length =
                                                                        __v43.wrapping_sub(copy);
                                                                    loop {
                                                                        let __v41: *mut u8 = from;
                                                                        from = __v41.add(1);
                                                                        let __v39: *mut u8 = put;
                                                                        put = __v39.add(1);
                                                                        *__v39 = *__v41;
                                                                        let __v36: u32 =
                                                                            copy.wrapping_sub(1);
                                                                        copy = __v36;
                                                                        if __v36 == 0 {
                                                                            break;
                                                                        }
                                                                    }
                                                                    if (*state).length == 0 {
                                                                        (*state).mode = 16200;
                                                                        continue '__loop_319;
                                                                    } else {
                                                                        continue '__loop_319;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        #[cfg(not(feature = "no-gzip"))]
                                                        {
                                                            if (*state).flags & 1024 != 0 {
                                                                '__loop_261: loop {
                                                                    if bits < 16 {
                                                                        loop {
                                                                            if have == 0 {
                                                                                break '__join_7;
                                                                            } else {
                                                                                have = have
                                                                                    .wrapping_sub(
                                                                                        1,
                                                                                    );
                                                                                let __v299: *mut u8 = next;
                                                                                next =
                                                                                    __v299.add(1);
                                                                                hold = hold
                                                                                    .wrapping_add(
                                                                                    (*__v299
                                                                                        as c_ulong)
                                                                                        << bits,
                                                                                );
                                                                                bits = bits
                                                                                    .wrapping_add(
                                                                                        8,
                                                                                    );
                                                                                if true {
                                                                                    continue '__loop_261;
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if true {
                                                                            break;
                                                                        }
                                                                    }
                                                                }
                                                                (*state).length = hold as u32;
                                                                if !unsafe { (*state).head }
                                                                    .is_null()
                                                                {
                                                                    (*(*state).head).extra_len =
                                                                        hold as u32;
                                                                }
                                                                if (*state).flags & 512 != 0
                                                                    && (*state).wrap & 4 != 0
                                                                {
                                                                    CRC2!(
                                                                        (*state).check,
                                                                        hold,
                                                                        hbuf
                                                                    );
                                                                }
                                                                INITBITS!(hold, bits);
                                                            } else {
                                                                if !unsafe { (*state).head }
                                                                    .is_null()
                                                                {
                                                                    (*(*state).head).extra =
                                                                        std::ptr::null_mut::<u8>();
                                                                }
                                                            }
                                                            (*state).mode = 16185;
                                                        }
                                                        #[cfg(feature = "no-gzip")]
                                                        {
                                                            (*state).mode = 16195;
                                                        }
                                                        // fallthrough
                                                    }
                                                    #[cfg(not(feature = "no-gzip"))]
                                                    {
                                                        if (*state).flags & 1024 != 0 {
                                                            copy = (*state).length;
                                                            if copy > have {
                                                                copy = have;
                                                            }
                                                            if copy != 0 {
                                                                if !unsafe { (*state).head }
                                                                    .is_null()
                                                                    && !unsafe {
                                                                        (*(*state).head).extra
                                                                    }
                                                                    .is_null()
                                                                {
                                                                    let __v291: u32 = unsafe {
                                                                        (*(*state).head).extra_len
                                                                    }
                                                                    .wrapping_sub((*state).length);
                                                                    len = __v291;
                                                                    __v292 = __v291
                                                                        < (*(*state).head)
                                                                            .extra_max;
                                                                } else {
                                                                    __v292 = false;
                                                                }
                                                                if __v292 {
                                                                    #[cfg(not(any(
                                                                        target_os = "macos",
                                                                        all(
                                                                            target_arch = "arm",
                                                                            windows
                                                                        )
                                                                    )))]
                                                                    {
                                                                        std::ptr::copy_nonoverlapping(((next as *const ()) as *const core::ffi::c_void) as *const u8, ((unsafe { (*(*state).head).extra }.add(len as usize) as *mut ()) as *mut core::ffi::c_void) as *mut u8, (((if len.wrapping_add(copy) > (*(*state).head).extra_max { unsafe { (*(*state).head).extra_max }.wrapping_sub(len) } else { copy }) as size_t) as usize) as usize);
                                                                    }
                                                                    #[cfg(any(
                                                                        target_os = "macos",
                                                                        all(
                                                                            target_arch = "arm",
                                                                            windows
                                                                        )
                                                                    ))]
                                                                    {
                                                                        std::ptr::copy_nonoverlapping(next as *const u8, (unsafe { (*(*state).head).extra }.add(len as usize) as *mut ()) as *mut u8, ((if len.wrapping_add(copy) > (*(*state).head).extra_max { unsafe { (*(*state).head).extra_max }.wrapping_sub(len) } else { copy }) as size_t) as usize);
                                                                    }
                                                                }
                                                                if (*state).flags & 512 != 0
                                                                    && (*state).wrap & 4 != 0
                                                                {
                                                                    (*state).check = crc32(
                                                                        (*state).check,
                                                                        next as *const u8,
                                                                        copy,
                                                                    );
                                                                }
                                                                have = have.wrapping_sub(copy);
                                                                let __v288: *mut u8 = next;
                                                                next = __v288.add(copy as usize);
                                                                let __v285: u32 = (*state).length;
                                                                (*state).length =
                                                                    __v285.wrapping_sub(copy);
                                                            }
                                                            if (*state).length != 0 {
                                                                break '__join_7;
                                                            }
                                                        }
                                                        (*state).length = 0;
                                                        (*state).mode = 16186;
                                                    }
                                                    #[cfg(feature = "no-gzip")]
                                                    {
                                                        copy = (*state).length;
                                                        if copy != 0 {
                                                            if copy > have {
                                                                copy = have;
                                                            }
                                                            if copy > left {
                                                                copy = left;
                                                            }
                                                            if copy == 0 {
                                                                break '__join_7;
                                                            } else {
                                                                #[cfg(not(any(
                                                                    target_os = "macos",
                                                                    all(
                                                                        target_arch = "arm",
                                                                        windows
                                                                    )
                                                                )))]
                                                                {
                                                                    std::ptr::copy_nonoverlapping(((next as *const ()) as *const core::ffi::c_void) as *const u8, ((put as *mut ()) as *mut core::ffi::c_void) as *mut u8, ((copy as size_t) as usize) as usize);
                                                                }
                                                                #[cfg(any(
                                                                    target_os = "macos",
                                                                    all(
                                                                        target_arch = "arm",
                                                                        windows
                                                                    )
                                                                ))]
                                                                {
                                                                    std::ptr::copy_nonoverlapping(
                                                                        next as *const u8,
                                                                        put as *mut u8,
                                                                        (copy as size_t) as usize,
                                                                    );
                                                                }
                                                                have = have.wrapping_sub(copy);
                                                                let __v215: *mut u8 = next;
                                                                next = __v215.add(copy as usize);
                                                                left = left.wrapping_sub(copy);
                                                                let __v211: *mut u8 = put;
                                                                put = __v211.add(copy as usize);
                                                                let __v208: u32 = (*state).length;
                                                                (*state).length =
                                                                    __v208.wrapping_sub(copy);
                                                                continue '__loop_319;
                                                            }
                                                        } else {
                                                            #[cfg(feature = "zlib-debug")]
                                                            {
                                                                Tracev!(
                                                                    slate_support::stderr(),
                                                                    c"inflate:       stored end\n"
                                                                        .as_ptr()
                                                                        as *mut c_char
                                                                );
                                                            }
                                                            (*state).mode = 16191;
                                                            continue '__loop_319;
                                                        }
                                                    }
                                                }
                                                #[cfg(not(feature = "no-gzip"))]
                                                {
                                                    if (*state).flags & 2048 != 0 {
                                                        if have == 0 {
                                                            break '__join_7;
                                                        } else {
                                                            copy = 0;
                                                            loop {
                                                                let __v283: u32 = copy;
                                                                copy = __v283.wrapping_add(1);
                                                                len = *next.add(__v283 as usize)
                                                                    as u32;
                                                                if !unsafe { (*state).head }
                                                                    .is_null()
                                                                    && !unsafe {
                                                                        (*(*state).head).name
                                                                    }
                                                                    .is_null()
                                                                    && (*state).length
                                                                        < (*(*state).head).name_max
                                                                {
                                                                    let __v280: u32 =
                                                                        (*state).length;
                                                                    (*state).length =
                                                                        __v280.wrapping_add(1);
                                                                    *unsafe {
                                                                        (*(*state).head).name
                                                                    }
                                                                    .add(__v280 as usize) =
                                                                        len as u8;
                                                                }
                                                                if !(len != 0 && copy < have) {
                                                                    break;
                                                                }
                                                            }
                                                            if (*state).flags & 512 != 0
                                                                && (*state).wrap & 4 != 0
                                                            {
                                                                (*state).check = crc32(
                                                                    (*state).check,
                                                                    next as *const u8,
                                                                    copy,
                                                                );
                                                            }
                                                            have = have.wrapping_sub(copy);
                                                            let __v276: *mut u8 = next;
                                                            next = __v276.add(copy as usize);
                                                            if len != 0 {
                                                                break '__join_7;
                                                            }
                                                        }
                                                    } else {
                                                        if !unsafe { (*state).head }.is_null() {
                                                            (*(*state).head).name =
                                                                std::ptr::null_mut::<u8>();
                                                        }
                                                    }
                                                    (*state).length = 0;
                                                    (*state).mode = 16187;
                                                }
                                                #[cfg(feature = "no-gzip")]
                                                {
                                                    if (*state).havedict == 0 {
                                                        RESTORE!(
                                                            strm, put, left, next, have, state,
                                                            hold, bits
                                                        );
                                                        return 2;
                                                    } else {
                                                        #[cfg(all(target_pointer_width = "64", unix))]
let __v236: u64 = adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
                                                        #[cfg(any(target_pointer_width = "32", windows))]
let __v236: u32 = adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
                                                        (*state).check = __v236;
                                                        (*strm).adler = __v236;
                                                        (*state).mode = 16191;
                                                        // fallthrough
                                                    }
                                                }
                                            }
                                            #[cfg(not(feature = "no-gzip"))]
                                            {
                                                if (*state).flags & 4096 != 0 {
                                                    if have == 0 {
                                                        break '__join_7;
                                                    } else {
                                                        copy = 0;
                                                        loop {
                                                            let __v274: u32 = copy;
                                                            copy = __v274.wrapping_add(1);
                                                            len = *next.add(__v274 as usize) as u32;
                                                            if !unsafe { (*state).head }.is_null()
                                                                && !unsafe {
                                                                    (*(*state).head).comment
                                                                }
                                                                .is_null()
                                                                && (*state).length
                                                                    < (*(*state).head).comm_max
                                                            {
                                                                let __v271: u32 = (*state).length;
                                                                (*state).length =
                                                                    __v271.wrapping_add(1);
                                                                *unsafe {
                                                                    (*(*state).head).comment
                                                                }
                                                                .add(__v271 as usize) = len as u8;
                                                            }
                                                            if !(len != 0 && copy < have) {
                                                                break;
                                                            }
                                                        }
                                                        if (*state).flags & 512 != 0
                                                            && (*state).wrap & 4 != 0
                                                        {
                                                            (*state).check = crc32(
                                                                (*state).check,
                                                                next as *const u8,
                                                                copy,
                                                            );
                                                        }
                                                        have = have.wrapping_sub(copy);
                                                        let __v267: *mut u8 = next;
                                                        next = __v267.add(copy as usize);
                                                        if len != 0 {
                                                            break '__join_7;
                                                        }
                                                    }
                                                } else {
                                                    if !unsafe { (*state).head }.is_null() {
                                                        (*(*state).head).comment =
                                                            std::ptr::null_mut::<u8>();
                                                    }
                                                }
                                                (*state).mode = 16188;
                                            }
                                            #[cfg(feature = "no-gzip")]
                                            {
                                                if flush == 5 || flush == 6 {
                                                    break '__join_7;
                                                }
                                            }
                                        }
                                        #[cfg(not(feature = "no-gzip"))]
                                        {
                                            if (*state).flags & 512 != 0 {
                                                '__loop_212: loop {
                                                    if bits < 16 {
                                                        loop {
                                                            if have == 0 {
                                                                break '__join_7;
                                                            } else {
                                                                have = have.wrapping_sub(1);
                                                                let __v263: *mut u8 = next;
                                                                next = __v263.add(1);
                                                                hold = hold.wrapping_add(
                                                                    (*__v263 as c_ulong) << bits,
                                                                );
                                                                bits = bits.wrapping_add(8);
                                                                if true {
                                                                    continue '__loop_212;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if true {
                                                            break;
                                                        }
                                                    }
                                                }
                                                if (*state).wrap & 4 != 0
                                                    && hold != (*state).check & 65535
                                                {
                                                    (*strm).msg = c"header crc mismatch".as_ptr()
                                                        as *mut c_char;
                                                    (*state).mode = 16209;
                                                    continue '__loop_319;
                                                } else {
                                                    INITBITS!(hold, bits);
                                                }
                                            }
                                            if !unsafe { (*state).head }.is_null() {
                                                (*(*state).head).hcrc = (*state).flags >> 9i32 & 1;
                                                (*(*state).head).done = 1;
                                            }
                                        }
                                        #[cfg(all(
                                            target_pointer_width = "64",
                                            unix,
                                            not(feature = "no-gzip")
                                        ))]
                                        let __v256: u64 =
                                            crc32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
                                        #[cfg(all(
                                            any(target_pointer_width = "32", windows),
                                            not(feature = "no-gzip")
                                        ))]
                                        let __v256: u32 =
                                            crc32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
                                        #[cfg(not(feature = "no-gzip"))]
                                        {
                                            (*state).check = __v256;
                                            (*strm).adler = __v256;
                                            (*state).mode = 16191;
                                        }
                                        #[cfg(feature = "no-gzip")]
                                        {
                                            if (*state).last != 0 {
                                                BYTEBITS!(hold, bits);
                                                (*state).mode = 16206;
                                            } else {
                                                '__loop_181: loop {
                                                    if bits < 3 {
                                                        loop {
                                                            if have == 0 {
                                                                break '__join_7;
                                                            } else {
                                                                have = have.wrapping_sub(1);
                                                                let __v22: *mut u8 = next;
                                                                next = __v22.add(1);
                                                                hold = hold.wrapping_add(
                                                                    (*__v22 as c_ulong) << bits,
                                                                );
                                                                bits = bits.wrapping_add(8);
                                                                if true {
                                                                    continue '__loop_181;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if true {
                                                            break;
                                                        }
                                                    }
                                                }
                                                (*state).last = BITS!(1i32, hold) as i32;
                                                DROPBITS!(1i32, hold, bits);
                                                let __t23: u32 = BITS!(2i32, hold);
                                                if __t23 == 0 {
                                                    #[cfg(feature = "zlib-debug")]
                                                    {
                                                        Tracev__2!(
                                                            slate_support::stderr(),
                                                            c"inflate:     stored block%s\n"
                                                                .as_ptr()
                                                                as *mut c_char,
                                                            if (*state).last != 0 {
                                                                c" (last)".as_ptr() as *mut c_char
                                                            } else {
                                                                c"".as_ptr() as *mut c_char
                                                            }
                                                        );
                                                    }
                                                    (*state).mode = 16193;
                                                } else {
                                                    if __t23 == 1 {
                                                        inflate_fixed(state); // fixed block
                                                        #[cfg(feature = "zlib-debug")]
                                                        {
                                                            Tracev__2!(slate_support::stderr(), c"inflate:     fixed codes block%s\n".as_ptr() as *mut c_char, if (*state).last != 0 { c" (last)".as_ptr() as *mut c_char } else { c"".as_ptr() as *mut c_char });
                                                        }
                                                        (*state).mode = 16199;
                                                        // decode codes
                                                        if flush == 6 {
                                                            break '__join_185;
                                                        }
                                                    } else {
                                                        if __t23 == 2 {
                                                            #[cfg(feature = "zlib-debug")]
                                                            {
                                                                Tracev__2!(slate_support::stderr(), c"inflate:     dynamic codes block%s\n".as_ptr() as *mut c_char, if (*state).last != 0 { c" (last)".as_ptr() as *mut c_char } else { c"".as_ptr() as *mut c_char });
                                                            }
                                                            (*state).mode = 16196;
                                                        } else {
                                                            (*strm).msg = c"invalid block type"
                                                                .as_ptr()
                                                                as *mut c_char;
                                                            (*state).mode = 16209;
                                                        }
                                                    }
                                                }
                                                DROPBITS!(2i32, hold, bits);
                                            }
                                        }
                                    }
                                    return -2;
                                }
                                ret = -3;
                                break '__join_7;
                            }
                            INITBITS!(hold, bits);
                            #[cfg(all(not(feature = "no-gzip"), feature = "zlib-debug"))]
                            {
                                Tracev!(
                                    slate_support::stderr(),
                                    c"inflate:   length matches trailer\n".as_ptr() as *mut c_char
                                );
                            }
                            #[cfg(all(feature = "no-gzip", feature = "zlib-debug"))]
                            {
                                Tracev!(
                                    slate_support::stderr(),
                                    c"inflate:   check matches trailer\n".as_ptr() as *mut c_char
                                );
                            }
                        }
                        (*state).mode = 16208;
                        // fallthrough
                    }
                    ret = 1;
                    break '__join_7;
                }
                DROPBITS!(2i32, hold, bits);
            }
            RESTORE!(strm, put, left, next, have, state, hold, bits);
            if (*state).wsize != 0
                || out != (*strm).avail_out
                    && ((*state).mode as u32) < 16209
                    && (((*state).mode as u32) < 16206 || flush != 4)
            {
                if updatewindow(
                    strm,
                    (*strm).next_out as *const u8,
                    out.wrapping_sub((*strm).avail_out),
                ) != 0
                {
                    (*state).mode = 16210;
                    return -4;
                }
            }
            r#in = r#in.wrapping_sub((*strm).avail_in);
            out = out.wrapping_sub((*strm).avail_out);
            #[cfg(all(target_pointer_width = "64", unix))]
            let __v8: u64 = (*strm).total_in;
            #[cfg(any(target_pointer_width = "32", windows))]
            let __v8: u32 = (*strm).total_in;
            (*strm).total_in = __v8.wrapping_add(r#in as c_ulong);
            #[cfg(all(target_pointer_width = "64", unix))]
            let __v5: u64 = (*strm).total_out;
            #[cfg(any(target_pointer_width = "32", windows))]
            let __v5: u32 = (*strm).total_out;
            (*strm).total_out = __v5.wrapping_add(out as c_ulong);
            #[cfg(all(target_pointer_width = "64", unix))]
            let __v2: u64 = (*state).total;
            #[cfg(any(target_pointer_width = "32", windows))]
            let __v2: u32 = (*state).total;
            (*state).total = __v2.wrapping_add(out as c_ulong);
            if (*state).wrap & 4 != 0 && out != 0 {
                #[cfg(all(target_pointer_width = "64", unix, not(feature = "no-gzip")))]
                let __v0: u64 = UPDATE_CHECK__2!(
                    unsafe { (*state).check },
                    unsafe { (*strm).next_out }.sub(out as usize),
                    out,
                    state
                );
                #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "no-gzip")))]
                let __v0: u32 = UPDATE_CHECK__2!(
                    unsafe { (*state).check },
                    unsafe { (*strm).next_out }.sub(out as usize),
                    out,
                    state
                );
                #[cfg(all(target_pointer_width = "64", unix, feature = "no-gzip"))]
                let __v0: u64 = UPDATE_CHECK!(
                    unsafe { (*state).check },
                    unsafe { (*strm).next_out }.sub(out as usize),
                    out
                );
                #[cfg(all(any(target_pointer_width = "32", windows), feature = "no-gzip"))]
                let __v0: u32 = UPDATE_CHECK!(
                    unsafe { (*state).check },
                    unsafe { (*strm).next_out }.sub(out as usize),
                    out
                );
                (*state).check = __v0;
                (*strm).adler = __v0;
            }
            (*strm).data_type = ((*state).bits as i32)
                + if (*state).last != 0 { 64 } else { 0 }
                + if ((*state).mode as u32) == 16191 {
                    128
                } else {
                    0
                }
                + if ((*state).mode as u32) == 16199 || ((*state).mode as u32) == 16194 {
                    256
                } else {
                    0
                };
            if (r#in == 0 && out == 0 || flush == 4) && ret == 0 {
                ret = -5;
            }
            return ret;
        }
    }
    // fallthrough
    // Return from inflate(), updating the total counts and the check value.
    // If there was no progress during the inflate() call, return a buffer
    // error.  Call updatewindow() to create and/or update the window state.
    // Note: a memory error from inflate() is non-recoverable.
}

static mut order: __SlateAlignLargeArray<[u16; 19]> = __SlateAlignLargeArray([
    16u16, 17u16, 18u16, 0u16, 8u16, 7u16, 9u16, 6u16, 10u16, 5u16, 11u16, 4u16, 12u16, 3u16,
    13u16, 2u16, 14u16, 1u16, 15u16,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateEnd(strm: *mut z_stream_s) -> i32 {
    if inflateStateCheck(strm) != 0 {
        return -2;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    if !unsafe { (*state).window }.is_null() {
        ZFREE!(strm, unsafe { (*state).window });
    }
    ZFREE!(strm, unsafe { (*strm).state });
    unsafe {
        (*strm).state = std::ptr::null_mut::<internal_state>();
    }
    #[cfg(feature = "zlib-debug")]
    {
        Tracev!(
            slate_support::stderr(),
            c"inflate: end\n".as_ptr() as *mut c_char
        );
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateGetDictionary(
    strm: *mut z_stream_s,
    dictionary: *mut u8,
    dictLength: *mut u32,
) -> i32 {
    // check state
    if inflateStateCheck(strm) != 0 {
        return -2;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    // copy dictionary
    if (unsafe { (*state).whave }) != 0 && !dictionary.is_null() {
        #[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    ((unsafe { (*state).window }.offset((*state).wnext as isize) as *const ())
                        as *const core::ffi::c_void) as *const u8,
                    ((dictionary as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    ((unsafe { (*state).whave }.wrapping_sub((*state).wnext) as size_t) as usize)
                        as usize,
                )
            };
            unsafe {
                std::ptr::copy_nonoverlapping(
                    (((*state).window as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((dictionary
                        .offset((*state).whave as isize)
                        .offset(-((*state).wnext as isize)) as *mut ())
                        as *mut core::ffi::c_void) as *mut u8,
                    (((*state).wnext as size_t) as usize) as usize,
                )
            };
        }
        #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    (unsafe { (*state).window }.offset((*state).wnext as isize) as *const ())
                        as *const u8,
                    dictionary as *mut u8,
                    (unsafe { (*state).whave }.wrapping_sub((*state).wnext) as size_t) as usize,
                )
            };
            unsafe {
                std::ptr::copy_nonoverlapping(
                    ((*state).window as *const ()) as *const u8,
                    (dictionary
                        .offset((*state).whave as isize)
                        .offset(-((*state).wnext as isize)) as *mut ())
                        as *mut u8,
                    ((*state).wnext as size_t) as usize,
                )
            };
        }
    }
    if !dictLength.is_null() {
        unsafe {
            *dictLength = (*state).whave;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateSetDictionary(
    strm: *mut z_stream_s,
    dictionary: *const u8,
    dictLength: u32,
) -> i32 {
    let mut dictid: c_ulong = 0;
    let mut ret: i32 = 0;
    // check state
    if inflateStateCheck(strm) != 0 {
        return -2;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    if (unsafe { (*state).wrap }) != 0 && ((unsafe { (*state).mode }) as u32) != 16190 {
        return -2;
    }
    // check for correct dictionary identifier
    if ((unsafe { (*state).mode }) as u32) == 16190 {
        dictid = unsafe { adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32) };
        dictid = unsafe { adler32(dictid, dictionary, dictLength) };
        if dictid != unsafe { (*state).check } {
            return -3;
        }
    }
    // copy dictionary to window using updatewindow(), which will amend the
    // existing dictionary if appropriate
    ret = updatewindow(
        strm,
        unsafe { dictionary.add(dictLength as usize) },
        dictLength,
    );
    if ret != 0 {
        unsafe {
            (*state).mode = 16210;
        }
        return -4;
    }
    unsafe {
        (*state).havedict = 1;
    }
    #[cfg(feature = "zlib-debug")]
    {
        Tracev!(
            slate_support::stderr(),
            c"inflate:   dictionary set\n".as_ptr() as *mut c_char
        );
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateGetHeader(strm: *mut z_stream_s, head: *mut gz_header_s) -> i32 {
    // check state
    if inflateStateCheck(strm) != 0 {
        return -2;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    if (unsafe { (*state).wrap }) & 2 == 0 {
        return -2;
    }
    // save header structure
    unsafe {
        (*state).head = head;
    }
    unsafe {
        (*head).done = 0;
    }
    0
}

/// Search buf[0..len-1] for the pattern: 0, 0, 0xff, 0xff.  Return when found
/// or when out of input.  When called, *have is the number of pattern bytes
/// found in order so far, in 0..3.  On return *have is updated to the new
/// state.  If on return *have equals four, then the pattern was found and the
/// return value is how many bytes were read including the last byte of the
/// pattern.  If *have is less than four, then the pattern has not been found
/// yet and the return value is len.  In the latter case, syncsearch() can be
/// called again with more data and the *have state.  *have is initialized to
/// zero for the first call.
fn syncsearch(have: *mut u32, buf: *const u8, len: u32) -> u32 {
    let mut got: u32 = unsafe { *have };
    let mut next: u32 = 0;
    while next < len && got < 4 {
        if ((unsafe { *buf.add(next as usize) }) as i32) == if got < 2 { 0 } else { 255 } {
            got = got.wrapping_add(1);
        } else {
            if (unsafe { *buf.add(next as usize) }) != 0 {
                got = 0;
            } else {
                got = 4u32.wrapping_sub(got);
            }
        }
        next = next.wrapping_add(1);
    }
    unsafe {
        *have = got;
    }
    next
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateSync(strm: *mut z_stream_s) -> i32 {
    let mut len: u32 = 0; // number of bytes to look at or looked at
    let mut flags: i32 = 0; // temporary to save header status
    let mut r#in: c_ulong = 0;
    let mut out: c_ulong = 0; // temporary to save total_in and total_out
    let mut buf: [u8; 4] = [0u8; 4]; // to restore bit buffer to byte string
    let mut state: *mut inflate_state = unsafe { std::mem::zeroed() };
    // check parameters
    if inflateStateCheck(strm) != 0 {
        return -2;
    }
    state = (unsafe { (*strm).state }) as *mut inflate_state;
    if (unsafe { (*strm).avail_in }) == 0 && (unsafe { (*state).bits }) < 8 {
        return -5;
    }
    // if first time, start search in bit buffer
    if ((unsafe { (*state).mode }) as u32) != 16211 {
        unsafe {
            (*state).mode = 16211;
        }
        unsafe {
            (*state).hold = (*state).hold >> ((*state).bits & 7);
        }
        let __v4: u32 = unsafe { (*state).bits };
        unsafe {
            (*state).bits = __v4.wrapping_sub((*state).bits & 7);
        }
        len = 0;
        while (unsafe { (*state).bits }) >= 8 {
            let __v6: u32 = len;
            len = __v6.wrapping_add(1);
            buf[__v6 as usize] = (unsafe { (*state).hold }) as u8;
            unsafe {
                (*state).hold = (*state).hold >> 8i32;
            }
            let __v12: u32 = unsafe { (*state).bits };
            unsafe {
                (*state).bits = __v12.wrapping_sub(8);
            }
        }
        unsafe {
            (*state).have = 0;
        }
        syncsearch(
            unsafe { std::ptr::addr_of_mut!((*state).have) },
            buf.as_mut_ptr() as *const u8,
            len,
        );
    }
    // search available input
    len = syncsearch(
        unsafe { std::ptr::addr_of_mut!((*state).have) },
        (unsafe { (*strm).next_in }) as *const u8,
        unsafe { (*strm).avail_in },
    );
    let __v16: u32 = unsafe { (*strm).avail_in };
    unsafe {
        (*strm).avail_in = __v16.wrapping_sub(len);
    }
    let __v19: *mut u8 = unsafe { (*strm).next_in };
    unsafe {
        (*strm).next_in = __v19.add(len as usize);
    }
    let __v22: c_ulong = unsafe { (*strm).total_in };
    unsafe {
        (*strm).total_in = __v22.wrapping_add(len as c_ulong);
    }
    // return no joy or set up to restart inflate() on a new block
    if (unsafe { (*state).have }) != 4 {
        return -3;
    }
    if (unsafe { (*state).flags }) == -1 {
        unsafe {
            (*state).wrap = 0;
        }
    } else {
        let __v26: i32 = (unsafe { (*state).wrap }) & !4;
        unsafe {
            (*state).wrap = __v26;
        }
    }
    // if no header yet, treat as raw
    // no point in computing a check value now
    flags = unsafe { (*state).flags };
    r#in = unsafe { (*strm).total_in };
    out = unsafe { (*strm).total_out };
    inflateReset(strm);
    unsafe {
        (*strm).total_in = r#in;
    }
    unsafe {
        (*strm).total_out = out;
    }
    unsafe {
        (*state).flags = flags;
    }
    unsafe {
        (*state).mode = 16191;
    }
    0
}

/// Returns true if inflate is currently at the end of a block generated by
/// Z_SYNC_FLUSH or Z_FULL_FLUSH. This function is used by one PPP
/// implementation to provide an additional safety check. PPP uses
/// Z_SYNC_FLUSH but removes the length bytes of the resulting empty stored
/// block. When decompressing, PPP checks that at the end of input packet,
/// inflate is waiting for these length bytes.
#[unsafe(no_mangle)]
extern "C-unwind" fn inflateSyncPoint(strm: *mut z_stream_s) -> i32 {
    if inflateStateCheck(strm) != 0 {
        return -2;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    (((unsafe { (*state).mode }) as u32) == 16193 && (unsafe { (*state).bits }) == 0) as i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateCopy(dest: *mut z_stream_s, source: *mut z_stream_s) -> i32 {
    let mut window: *mut u8 = unsafe { std::mem::zeroed() };
    // check input
    if inflateStateCheck(source) != 0 || dest.is_null() {
        return -2;
    }
    let state: *mut inflate_state = (unsafe { (*source).state }) as *mut inflate_state;
    // allocate space
    let copy: *mut inflate_state =
        ZALLOC__2!(source, 1i32, std::mem::size_of::<inflate_state>() as size_t)
            as *mut inflate_state;
    if copy.is_null() {
        return -4;
    }
    zmemzero!(copy, std::mem::size_of::<inflate_state>() as size_t);
    window = std::ptr::null_mut::<u8>();
    if !unsafe { (*state).window }.is_null() {
        window = ZALLOC!(
            source,
            1u32 << unsafe { (*state).wbits },
            std::mem::size_of::<u8>() as size_t
        ) as *mut u8;
        if window.is_null() {
            ZFREE!(source, copy);
            return -4;
        }
    }
    // copy state
    #[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((source as *const ()) as *const core::ffi::c_void) as *const u8,
                ((dest as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                ((std::mem::size_of::<z_stream_s>() as size_t) as usize) as usize,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((state as *const ()) as *const core::ffi::c_void) as *const u8,
                ((copy as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                ((std::mem::size_of::<inflate_state>() as size_t) as usize) as usize,
            )
        };
    }
    #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                source as *const u8,
                dest as *mut u8,
                std::mem::size_of::<z_stream_s>(),
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                state as *const u8,
                copy as *mut u8,
                std::mem::size_of::<inflate_state>(),
            )
        };
    }
    unsafe {
        (*copy).strm = dest;
    }
    #[cfg(not(feature = "pkzip-bug"))]
    {
        if (unsafe { (*state).lencode })
            >= ((unsafe { (*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28 })
                as *const __SlateRecord_inftrees_h_28)
            && (unsafe { (*state).lencode })
                <= ((unsafe {
                    ((*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28)
                        .offset(1444i32 as isize)
                        .sub(1)
                }) as *const __SlateRecord_inftrees_h_28)
        {
            unsafe {
                (*copy).lencode = ((*copy).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28)
                    .offset(
                        (unsafe { (*state).lencode }.offset_from(
                            ((*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28)
                                as *const __SlateRecord_inftrees_h_28,
                        ) as ptrdiff_t) as isize,
                    ) as *const __SlateRecord_inftrees_h_28;
            }
            unsafe {
                (*copy).distcode = ((*copy).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28)
                    .offset(
                        (unsafe { (*state).distcode }.offset_from(
                            ((*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28)
                                as *const __SlateRecord_inftrees_h_28,
                        ) as ptrdiff_t) as isize,
                    ) as *const __SlateRecord_inftrees_h_28;
            }
        }
    }
    #[cfg(feature = "pkzip-bug")]
    {
        if (unsafe { (*state).lencode })
            >= ((unsafe { (*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28 })
                as *const __SlateRecord_inftrees_h_28)
            && (unsafe { (*state).lencode })
                <= ((unsafe {
                    ((*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28)
                        .offset(1448i32 as isize)
                        .sub(1)
                }) as *const __SlateRecord_inftrees_h_28)
        {
            unsafe {
                (*copy).lencode = ((*copy).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28)
                    .offset(
                        (unsafe { (*state).lencode }.offset_from(
                            ((*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28)
                                as *const __SlateRecord_inftrees_h_28,
                        ) as ptrdiff_t) as isize,
                    ) as *const __SlateRecord_inftrees_h_28;
            }
            unsafe {
                (*copy).distcode = ((*copy).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28)
                    .offset(
                        (unsafe { (*state).distcode }.offset_from(
                            ((*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28)
                                as *const __SlateRecord_inftrees_h_28,
                        ) as ptrdiff_t) as isize,
                    ) as *const __SlateRecord_inftrees_h_28;
            }
        }
    }
    unsafe {
        (*copy).next = ((*copy).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28).offset(
            (unsafe { (*state).next }.offset_from(
                ((*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28)
                    as *mut __SlateRecord_inftrees_h_28,
            ) as ptrdiff_t) as isize,
        );
    }
    if !window.is_null() {
        #[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    (((*state).window as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((window as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                    (((*state).whave as size_t) as usize) as usize,
                )
            };
        }
        #[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    ((*state).window as *const ()) as *const u8,
                    window as *mut u8,
                    ((*state).whave as size_t) as usize,
                )
            };
        }
    }
    unsafe {
        (*copy).window = window;
    }
    unsafe {
        (*dest).state = copy as *mut internal_state;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateUndermine(strm: *mut z_stream_s, subvert: i32) -> i32 {
    if inflateStateCheck(strm) != 0 {
        return -2;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    unsafe {
        #[cfg(not(feature = "inflate-allow-invalid"))]
        {
            (*state).sane = 1;
        }
        #[cfg(feature = "inflate-allow-invalid")]
        {
            (*state).sane = (subvert == 0) as i32;
        }
    }
    #[cfg(not(feature = "inflate-allow-invalid"))]
    {
        -3
    }
    #[cfg(feature = "inflate-allow-invalid")]
    {
        0
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateValidate(strm: *mut z_stream_s, check: i32) -> i32 {
    if inflateStateCheck(strm) != 0 {
        return -2;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    if check != 0 && (unsafe { (*state).wrap }) != 0 {
        unsafe {
            (*state).wrap = (*state).wrap | 4;
        }
    } else {
        let __v5: i32 = (unsafe { (*state).wrap }) & !4;
        unsafe {
            (*state).wrap = __v5;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateMark(strm: *mut z_stream_s) -> c_long {
    if inflateStateCheck(strm) != 0 {
        return -(65536 as c_long);
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        ((((unsafe { (*state).back }) as c_ulong) << 16i32) as c_long)
            + ((if ((unsafe { (*state).mode }) as u32) == 16195 {
                unsafe { (*state).length }
            } else {
                if ((unsafe { (*state).mode }) as u32) == 16204 {
                    unsafe { (*state).was }.wrapping_sub(unsafe { (*state).length })
                } else {
                    0u32
                }
            }) as c_long)
    }
    #[cfg(any(target_pointer_width = "32", windows))]
    {
        ((((((unsafe { (*state).back }) as c_long) as c_ulong) << 16i32) as c_long) as c_ulong)
            .wrapping_add(
                (if ((unsafe { (*state).mode }) as u32) == 16195 {
                    unsafe { (*state).length }
                } else {
                    if ((unsafe { (*state).mode }) as u32) == 16204 {
                        unsafe { (*state).was }.wrapping_sub(unsafe { (*state).length })
                    } else {
                        0u32
                    }
                }) as c_ulong,
            ) as c_long
    }
}

#[cfg(all(target_pointer_width = "64", unix))]
const __SLATE_HOIST_inflateCodesUsed: c_ulong = (18446744073709551615 as c_ulong);

#[cfg(any(target_pointer_width = "32", windows))]
const __SLATE_HOIST_inflateCodesUsed: c_ulong = (4294967295 as c_ulong);

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateCodesUsed(strm: *mut z_stream_s) -> c_ulong {
    if inflateStateCheck(strm) != 0 {
        return __SLATE_HOIST_inflateCodesUsed;
    }
    let state: *mut inflate_state = (unsafe { (*strm).state }) as *mut inflate_state;
    ((unsafe {
        unsafe { (*state).next }.offset_from(
            ((*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28)
                as *mut __SlateRecord_inftrees_h_28,
        )
    }) as ptrdiff_t) as c_ulong
}
