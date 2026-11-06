//! infback.c -- inflate using a call-back interface
//! Copyright (C) 1995-2026 Mark Adler
//! For conditions of distribution and use, see copyright notice in zlib.h
#[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
unsafe extern "C" {
    fn memcpy(arg0: *mut (), arg1: *const (), arg2: size_t) -> *mut ();
}

#[cfg(feature = "zlib-debug")]
unsafe extern "C" {
    static mut z_verbose: i32;
    fn fflush(_0: *mut slate_support::FILE) -> i32;
}

unsafe extern "C" {
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

use core::ffi::c_char;
use core::ffi::c_ulong;
#[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
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

// This code is largely copied from inflate.c.  Normally either infback.o or
// inflate.o would be linked into an application--not both.  The interface
// with inffast.c is retained so that optimized assembler-coded versions of
// inflate_fast() can be used with either inflate.c or infback.c.
/// strm provides memory allocation functions in zalloc and zfree, or
/// Z_NULL to use the library memory allocation functions.
///
/// windowBits is in the range 8..15, and window is a user-supplied
/// window and output buffer that is 2**windowBits bytes.
#[unsafe(no_mangle)]
extern "C-unwind" fn inflateBackInit_(
    strm: *mut z_stream_s,
    windowBits: i32,
    window: *mut u8,
    version: *const c_char,
    stream_size: i32,
) -> i32 {
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_os = "macos",
        windows
    ))]
    {
        if version.is_null()
            || ((unsafe { *version.add(0) }) as i32)
                != ((unsafe { *(c"1.3.2.1-motley".as_ptr() as *mut c_char).add(0) }) as i32)
            || stream_size != (std::mem::size_of::<z_stream_s>() as i32)
        {
            return -6;
        }
    }
    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_os = "macos",
        windows
    )))]
    {
        if version.is_null()
            || (((unsafe { *version.add(0) }) as u32) as i32)
                != (((unsafe { *(c"1.3.2.1-motley".as_ptr() as *mut c_char).add(0) }) as u32)
                    as i32)
            || stream_size != (std::mem::size_of::<z_stream_s>() as i32)
        {
            return -6;
        }
    }
    if strm.is_null() || window.is_null() || windowBits < 8 || windowBits > 15 {
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
    #[cfg(all(target_pointer_width = "64", unix))]
    let state: *mut inflate_state =
        ZALLOC__2!(strm, 1i32, std::mem::size_of::<inflate_state>() as c_ulong)
            as *mut inflate_state;
    #[cfg(target_pointer_width = "32")]
    let state: *mut inflate_state =
        ZALLOC__4!(strm, 1i32, std::mem::size_of::<inflate_state>() as u32) as *mut inflate_state;
    #[cfg(all(target_pointer_width = "64", windows))]
    let state: *mut inflate_state =
        ZALLOC__2!(strm, 1i32, std::mem::size_of::<inflate_state>() as u64) as *mut inflate_state;
    if state.is_null() {
        return -4;
    }
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
        (*state).dmax = 32768;
    }
    unsafe {
        (*state).wbits = windowBits as u32;
    }
    unsafe {
        (*state).wsize = 1u32 << windowBits;
    }
    unsafe {
        (*state).window = window;
    }
    unsafe {
        (*state).wnext = 0;
    }
    unsafe {
        (*state).whave = 0;
    }
    unsafe {
        (*state).sane = 1;
    }
    0
}

#[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
use slate_support::memcpy;

// Macros for inflateBack():
// Load returned state from inflate_fast()
// Set state from registers for inflate_fast()
// Clear the input bit accumulator
// Assure that some input is available.  If input is requested, but denied,
// then return a Z_BUF_ERROR from inflateBack().
// Get a byte of input into the bit accumulator, or return from inflateBack()
// with an error if there is no input available.
// Assure that there are at least n bits in the bit accumulator.  If there is
// not enough available input to do that, then return from inflateBack() with
// an error.
// Return the low n bits of the bit accumulator (n < 16)
// Remove n bits from the bit accumulator
// Remove zero to seven bits as needed to go to a byte boundary
// Assure that some output space is available, by writing out the window
// if it's full.  If the write fails, return from inflateBack() with a
// Z_BUF_ERROR.
/// strm provides the memory allocation functions and window buffer on input,
/// and provides information on the unused input on return.  For Z_DATA_ERROR
/// returns, strm will also provide an error message.
///
/// in() and out() are the call-back input and output functions.  When
/// inflateBack() needs more input, it calls in().  When inflateBack() has
/// filled the window with output, or when it completes with data in the
/// window, it calls out() to write out the data.  The application must not
/// change the provided input until in() is called again or inflateBack()
/// returns.  The application must not change the window/output buffer until
/// inflateBack() returns.
///
/// in() and out() are called with a descriptor parameter provided in the
/// inflateBack() call.  This parameter can be a structure that provides the
/// information required to do the read or write, as well as accumulated
/// information on the input and output such as totals and check values.
///
/// in() should return zero on failure.  out() should return non-zero on
/// failure.  If either in() or out() fails, than inflateBack() returns a
/// Z_BUF_ERROR.  strm->next_in can be checked for Z_NULL to see whether it
/// was in() or out() that caused in the error.  Otherwise,  inflateBack()
/// returns Z_STREAM_END on success, Z_DATA_ERROR for an deflate format
/// error, or Z_MEM_ERROR if it could not allocate memory for the state.
/// inflateBack() can also return Z_STREAM_ERROR if the input parameters
/// are not correct, i.e. strm is Z_NULL or the state was not initialized.
#[unsafe(no_mangle)]
extern "C-unwind" fn inflateBack(
    strm: *mut z_stream_s,
    r#in: Option<unsafe extern "C-unwind" fn(*mut (), *mut *mut u8) -> u32>,
    in_desc: *mut (),
    out: Option<unsafe extern "C-unwind" fn(*mut (), *mut u8, u32) -> i32>,
    out_desc: *mut (),
) -> i32 {
    // return code
    // permutation of code lengths
    let mut ret: i32 = unsafe { std::mem::zeroed() };
    // length to copy for repeats, bits to drop
    let mut len: u32 = unsafe { std::mem::zeroed() };
    // parent table entry
    let mut last: std::mem::MaybeUninit<__SlateRecord_inftrees_h_28> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_174: *mut __SlateRecord_inftrees_h_28 =
        std::ptr::addr_of_mut!(last) as *mut __SlateRecord_inftrees_h_28; // current decoding table entry
    let mut here: std::mem::MaybeUninit<__SlateRecord_inftrees_h_28> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_175: *mut __SlateRecord_inftrees_h_28 =
        std::ptr::addr_of_mut!(here) as *mut __SlateRecord_inftrees_h_28; // where to copy match bytes from
    let mut from: *mut u8 = unsafe { std::mem::zeroed() };
    // number of stored or match bytes to copy
    let mut copy: u32 = unsafe { std::mem::zeroed() };
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
        // Check that the strm exists and that the state was initialized
        if strm.is_null() || unsafe { (*strm).state }.is_null() {
            return -2;
        } else {
            state = (*strm).state as *mut inflate_state;
            // Reset the state
            (*strm).msg = std::ptr::null_mut::<c_char>();
            (*state).mode = 16191;
            (*state).last = 0;
            (*state).whave = 0;
            next = (*strm).next_in;
            have = if !next.is_null() { (*strm).avail_in } else { 0 };
            hold = 0;
            bits = 0;
            put = (*state).window;
            left = (*state).wsize;
            // Inflate until end of block marked as last
            '__join_3: {
                '__join_197: {
                    '__join_183: {
                        '__join_172: {
                            '__join_168: {
                                '__join_157: {
                                    '__join_147: {
                                        '__join_108: {
                                            '__join_64: {
                                                '__join_54: {
                                                    '__join_27: {
                                                        '__join_16: {
                                                            '__join_36: {
                                                                '__join_44: {
                                                                    '__join_72: {
                                                                        '__join_80: {
                                                                            '__join_116: {
                                                                                '__join_124: {
                                                                                    '__join_132: {
                                                                                        '__join_6: {
                                                                                            '__join_5: {
                                                                                                '__loop_203: loop {
                                                                                                    let __t1: u32 = (*state).mode as u32;
                                                                                                    if __t1 == 16191 {
if (*state).last != 0 {
BYTEBITS!(hold, bits);
(*state).mode = 16208;
continue '__loop_203;
} else {
// determine and dispatch block type
'__loop_194: loop {
if bits < 3 {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_197;
}
}
if true {
have = have.wrapping_sub(1);
let __v169: *mut u8 = next;
next = __v169.add(1);
hold = hold.wrapping_add((*__v169 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_194;
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
let __t0: u32 = BITS!(2i32, hold);
if __t0 == 0 {
#[cfg(feature = "zlib-debug")]
{
Tracev__2!(slate_support::stderr(), c"inflate:     stored block%s\n".as_ptr() as *mut c_char, if (*state).last != 0 { c" (last)".as_ptr() as *mut c_char } else { c"".as_ptr() as *mut c_char });
}
(*state).mode = 16193;
} else {
if __t0 == 1 {
inflate_fixed(state); // fixed block
#[cfg(feature = "zlib-debug")]
{
Tracev__2!(slate_support::stderr(), c"inflate:     fixed codes block%s\n".as_ptr() as *mut c_char, if (*state).last != 0 { c" (last)".as_ptr() as *mut c_char } else { c"".as_ptr() as *mut c_char });
}
(*state).mode = 16200;
// decode codes
} else {
if __t0 == 2 {
#[cfg(feature = "zlib-debug")]
{
Tracev__2!(slate_support::stderr(), c"inflate:     dynamic codes block%s\n".as_ptr() as *mut c_char, if (*state).last != 0 { c" (last)".as_ptr() as *mut c_char } else { c"".as_ptr() as *mut c_char });
}
(*state).mode = 16196;
} else {
(*strm).msg = c"invalid block type".as_ptr() as *mut c_char;
(*state).mode = 16209;
}
}
}
DROPBITS!(2i32, hold, bits);
continue '__loop_203;
}
} else {
if __t1 == 16193 {
BYTEBITS!(hold, bits); // get and verify stored block length
// go to byte boundary
'__loop_180: loop {
if bits < 32 {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_183;
}
}
if true {
have = have.wrapping_sub(1);
let __v160: *mut u8 = next;
next = __v160.add(1);
hold = hold.wrapping_add((*__v160 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_180;
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
continue '__loop_203;
} else {
(*state).length = (hold as u32) & 65535;
#[cfg(feature = "zlib-debug")]
{
Tracev__2!(slate_support::stderr(), c"inflate:       stored length %u\n".as_ptr() as *mut c_char, unsafe { (*state).length });
}
INITBITS!(hold, bits);
// copy stored block from input to output
while (*state).length != 0 {
copy = (*state).length;
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_172;
}
}
if true {
break;
}
}
loop {
if left == 0 {
put = (*state).window;
left = (*state).wsize;
(*state).whave = left;
if out.unwrap()(out_desc, put, left) != 0 {
break '__join_168;
}
}
if true {
break;
}
}
if copy > have {
copy = have;
}
if copy > left {
copy = left;
}
#[cfg(all(not(target_os = "macos"), target_pointer_width = "64", unix))]
{
std::ptr::copy_nonoverlapping(((next as *const ()) as *const core::ffi::c_void) as *const u8, ((put as *mut ()) as *mut core::ffi::c_void) as *mut u8, ((copy as c_ulong) as usize) as usize);
}
#[cfg(any(target_arch = "x86", all(target_arch = "arm", unix)))]
{
std::ptr::copy_nonoverlapping(((next as *const ()) as *const core::ffi::c_void) as *const u8, ((put as *mut ()) as *mut core::ffi::c_void) as *mut u8, (copy as usize) as usize);
}
#[cfg(target_os = "macos")]
{
std::ptr::copy_nonoverlapping(next as *const u8, put as *mut u8, (copy as c_ulong) as usize);
}
#[cfg(all(target_pointer_width = "64", windows))]
{
std::ptr::copy_nonoverlapping(((next as *const ()) as *const core::ffi::c_void) as *const u8, ((put as *mut ()) as *mut core::ffi::c_void) as *mut u8, ((copy as u64) as usize) as usize);
}
#[cfg(all(target_arch = "arm", windows))]
{
std::ptr::copy_nonoverlapping(next as *const u8, put as *mut u8, copy as usize);
}
have = have.wrapping_sub(copy);
let __v151: *mut u8 = next;
next = __v151.add(copy as usize);
left = left.wrapping_sub(copy);
let __v147: *mut u8 = put;
put = __v147.add(copy as usize);
let __v144: u32 = (*state).length;
(*state).length = __v144.wrapping_sub(copy);
}
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       stored end\n".as_ptr() as *mut c_char);
}
(*state).mode = 16191;
continue '__loop_203;
}
} else {
if __t1 == 16196 {
'__loop_154: loop {
if bits < 14 {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_157;
}
}
if true {
have = have.wrapping_sub(1);
let __v140: *mut u8 = next;
next = __v140.add(1);
hold = hold.wrapping_add((*__v140 as c_ulong) << bits);
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
// get dynamic table entries descriptor
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
continue '__loop_203;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       table sizes ok\n".as_ptr() as *mut c_char);
}
// get code length code lengths (not a typo)
(*state).have = 0;
while (*state).have < (*state).ncode {
'__loop_144: loop {
if bits < 3 {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_147;
}
}
if true {
have = have.wrapping_sub(1);
let __v131: *mut u8 = next;
next = __v131.add(1);
hold = hold.wrapping_add((*__v131 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_144;
}
}
}
} else {
if true {
break;
}
}
}
let __v123: u32 = (*state).have;
(*state).have = __v123.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).offset(((order.0[__v123 as usize] as u32) as i32) as isize) = BITS!(3i32, hold) as u16;
DROPBITS!(3i32, hold, bits);
}
while (*state).have < 19 {
let __v120: u32 = (*state).have;
(*state).have = __v120.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).offset(((order.0[__v120 as usize] as u32) as i32) as isize) = 0;
}
(*state).next = (*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28;
(*state).lencode = (*state).next as *const __SlateRecord_inftrees_h_28;
(*state).lenbits = 7;
ret = inflate_table(0u32, (*state).lens.as_mut_ptr() as *mut u16, 19u32, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).lenbits), (*state).work.as_mut_ptr() as *mut u16);
if ret != 0 {
(*strm).msg = c"invalid code lengths set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_203;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       code lengths ok\n".as_ptr() as *mut c_char);
}
// get length and distance code code lengths
(*state).have = 0;
'__join_94: {
'__join_102: {
'__loop_95: loop {
if (*state).have < unsafe { (*state).nlen }.wrapping_add((*state).ndist) {
'__loop_135: loop {
*__slate_slot_175 = *unsafe { (*state).lencode }.offset(BITS!(unsafe { (*state).lenbits }, hold) as isize);
if ((*__slate_slot_175).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_132;
}
}
if true {
have = have.wrapping_sub(1);
let __v116: *mut u8 = next;
next = __v116.add(1);
hold = hold.wrapping_add((*__v116 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_135;
}
}
}
}
}
if (((*__slate_slot_175).val as u32) as i32) < 16 {
DROPBITS__2!((*__slate_slot_175).bits, hold, bits);
let __v76: u32 = (*state).have;
(*state).have = __v76.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).add(__v76 as usize) = (*__slate_slot_175).val;
} else {
if (((*__slate_slot_175).val as u32) as i32) == 16 {
'__loop_105: loop {
if bits < (((((*__slate_slot_175).bits as u32) as i32) + 2) as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_108;
}
}
if true {
have = have.wrapping_sub(1);
let __v89: *mut u8 = next;
next = __v89.add(1);
hold = hold.wrapping_add((*__v89 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_105;
}
}
}
} else {
if true {
break;
}
}
}
DROPBITS__2!((*__slate_slot_175).bits, hold, bits);
if (*state).have == 0 {
break '__join_102;
} else {
len = *((*state).lens.as_mut_ptr() as *mut u16).offset(unsafe { (*state).have }.wrapping_sub(1) as isize) as u32;
copy = 3u32.wrapping_add(BITS!(2i32, hold));
DROPBITS!(2i32, hold, bits);
}
} else {
if (((*__slate_slot_175).val as u32) as i32) == 17 {
'__loop_113: loop {
if bits < (((((*__slate_slot_175).bits as u32) as i32) + 3) as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_116;
}
}
if true {
have = have.wrapping_sub(1);
let __v98: *mut u8 = next;
next = __v98.add(1);
hold = hold.wrapping_add((*__v98 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_113;
}
}
}
} else {
if true {
break;
}
}
}
DROPBITS__2!((*__slate_slot_175).bits, hold, bits);
len = 0;
copy = 3u32.wrapping_add(BITS!(3i32, hold));
DROPBITS!(3i32, hold, bits);
} else {
'__loop_121: loop {
if bits < (((((*__slate_slot_175).bits as u32) as i32) + 7) as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_124;
}
}
if true {
have = have.wrapping_sub(1);
let __v107: *mut u8 = next;
next = __v107.add(1);
hold = hold.wrapping_add((*__v107 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_121;
}
}
}
} else {
if true {
break;
}
}
}
DROPBITS__2!((*__slate_slot_175).bits, hold, bits);
len = 0;
copy = 11u32.wrapping_add(BITS!(7i32, hold));
DROPBITS!(7i32, hold, bits);
}
}
if unsafe { (*state).have }.wrapping_add(copy) > unsafe { (*state).nlen }.wrapping_add((*state).ndist) {
break;
} else {
loop {
let __v79: u32 = copy;
copy = __v79.wrapping_sub(1);
if __v79 != 0 {
let __v81: u32 = (*state).have;
(*state).have = __v81.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).add(__v81 as usize) = len as u16;
} else {
continue '__loop_95;
}
}
}
}
} else {
break '__join_94;
}
}
(*strm).msg = c"invalid bit length repeat".as_ptr() as *mut c_char;
(*state).mode = 16209;
break '__join_94;
}
(*strm).msg = c"invalid bit length repeat".as_ptr() as *mut c_char;
(*state).mode = 16209;
}
// handle error breaks in while
if ((*state).mode as u32) == 16209 {
continue '__loop_203;
} else {
// check for end-of-block code (better have one)
if ((*((*state).lens.as_mut_ptr() as *mut u16).add(256) as u32) as i32) == 0 {
(*strm).msg = c"invalid code -- missing end-of-block".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_203;
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
continue '__loop_203;
} else {
(*state).distcode = (*state).next as *const __SlateRecord_inftrees_h_28;
(*state).distbits = 6;
ret = inflate_table(2u32, ((*state).lens.as_mut_ptr() as *mut u16).offset((*state).nlen as isize), (*state).ndist, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).distbits), (*state).work.as_mut_ptr() as *mut u16);
if ret != 0 {
(*strm).msg = c"invalid distances set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_203;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       codes ok\n".as_ptr() as *mut c_char);
}
(*state).mode = 16200;
// fallthrough
}
}
}
}
}
}
}
#[cfg(all(feature = "zlib-debug", feature = "pkzip-bug"))]
{
Tracev!(slate_support::stderr(), c"inflate:       table sizes ok\n".as_ptr() as *mut c_char);
}
#[cfg(feature = "pkzip-bug")]
{
(*state).have = 0;
while (*state).have < (*state).ncode {
'__loop_1: loop {
if bits < 3 {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_147;
}
}
if true {
have = have.wrapping_sub(1);
let __v2: *mut u8 = next;
next = __v2.add(1);
hold = hold.wrapping_add((*__v2 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_1;
}
}
}
} else {
if true {
break;
}
}
}
let __v4: u32 = (*state).have;
(*state).have = __v4.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).offset(((order.0[__v4 as usize] as u32) as i32) as isize) = BITS!(3i32, hold) as u16;
DROPBITS!(3i32, hold, bits);
}
while (*state).have < 19 {
let __v6: u32 = (*state).have;
(*state).have = __v6.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).offset(((order.0[__v6 as usize] as u32) as i32) as isize) = 0;
}
(*state).next = (*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28;
(*state).lencode = (*state).next as *const __SlateRecord_inftrees_h_28;
(*state).lenbits = 7;
ret = inflate_table(0u32, (*state).lens.as_mut_ptr() as *mut u16, 19u32, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).lenbits), (*state).work.as_mut_ptr() as *mut u16);
if ret != 0 {
(*strm).msg = c"invalid code lengths set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_203;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       code lengths ok\n".as_ptr() as *mut c_char);
}
// get length and distance code code lengths
(*state).have = 0;
'__join_7: {
'__join_8: {
'__loop_9: loop {
if (*state).have < unsafe { (*state).nlen }.wrapping_add((*state).ndist) {
'__loop_10: loop {
*__slate_slot_175 = *unsafe { (*state).lencode }.offset(BITS!(unsafe { (*state).lenbits }, hold) as isize);
if ((*__slate_slot_175).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_132;
}
}
if true {
have = have.wrapping_sub(1);
let __v13: *mut u8 = next;
next = __v13.add(1);
hold = hold.wrapping_add((*__v13 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_10;
}
}
}
}
}
if (((*__slate_slot_175).val as u32) as i32) < 16 {
DROPBITS__2!((*__slate_slot_175).bits, hold, bits);
let __v14: u32 = (*state).have;
(*state).have = __v14.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).add(__v14 as usize) = (*__slate_slot_175).val;
} else {
if (((*__slate_slot_175).val as u32) as i32) == 16 {
'__loop_15: loop {
if bits < (((((*__slate_slot_175).bits as u32) as i32) + 2) as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_108;
}
}
if true {
have = have.wrapping_sub(1);
let __v16: *mut u8 = next;
next = __v16.add(1);
hold = hold.wrapping_add((*__v16 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_15;
}
}
}
} else {
if true {
break;
}
}
}
DROPBITS__2!((*__slate_slot_175).bits, hold, bits);
if (*state).have == 0 {
break '__join_8;
} else {
len = *((*state).lens.as_mut_ptr() as *mut u16).offset(unsafe { (*state).have }.wrapping_sub(1) as isize) as u32;
copy = 3u32.wrapping_add(BITS!(2i32, hold));
DROPBITS!(2i32, hold, bits);
}
} else {
if (((*__slate_slot_175).val as u32) as i32) == 17 {
'__loop_17: loop {
if bits < (((((*__slate_slot_175).bits as u32) as i32) + 3) as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_116;
}
}
if true {
have = have.wrapping_sub(1);
let __v18: *mut u8 = next;
next = __v18.add(1);
hold = hold.wrapping_add((*__v18 as c_ulong) << bits);
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
DROPBITS__2!((*__slate_slot_175).bits, hold, bits);
len = 0;
copy = 3u32.wrapping_add(BITS!(3i32, hold));
DROPBITS!(3i32, hold, bits);
} else {
'__loop_19: loop {
if bits < (((((*__slate_slot_175).bits as u32) as i32) + 7) as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_124;
}
}
if true {
have = have.wrapping_sub(1);
let __v21: *mut u8 = next;
next = __v21.add(1);
hold = hold.wrapping_add((*__v21 as c_ulong) << bits);
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
DROPBITS__2!((*__slate_slot_175).bits, hold, bits);
len = 0;
copy = 11u32.wrapping_add(BITS!(7i32, hold));
DROPBITS!(7i32, hold, bits);
}
}
if unsafe { (*state).have }.wrapping_add(copy) > unsafe { (*state).nlen }.wrapping_add((*state).ndist) {
break;
} else {
loop {
let __v22: u32 = copy;
copy = __v22.wrapping_sub(1);
if __v22 != 0 {
let __v23: u32 = (*state).have;
(*state).have = __v23.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).add(__v23 as usize) = len as u16;
} else {
continue '__loop_9;
}
}
}
}
} else {
break '__join_7;
}
}
(*strm).msg = c"invalid bit length repeat".as_ptr() as *mut c_char;
(*state).mode = 16209;
break '__join_7;
}
(*strm).msg = c"invalid bit length repeat".as_ptr() as *mut c_char;
(*state).mode = 16209;
}
// handle error breaks in while
if ((*state).mode as u32) == 16209 {
continue '__loop_203;
} else {
// check for end-of-block code (better have one)
if ((*((*state).lens.as_mut_ptr() as *mut u16).add(256) as u32) as i32) == 0 {
(*strm).msg = c"invalid code -- missing end-of-block".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_203;
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
continue '__loop_203;
} else {
(*state).distcode = (*state).next as *const __SlateRecord_inftrees_h_28;
(*state).distbits = 6;
ret = inflate_table(2u32, ((*state).lens.as_mut_ptr() as *mut u16).offset((*state).nlen as isize), (*state).ndist, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).distbits), (*state).work.as_mut_ptr() as *mut u16);
if ret != 0 {
(*strm).msg = c"invalid distances set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_203;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       codes ok\n".as_ptr() as *mut c_char);
}
(*state).mode = 16200;
// fallthrough
}
}
}
}
}
}
} else {
if __t1 != 16200 {
if __t1 == 16208 {
break '__join_6;
} else {
if __t1 == 16209 {
break '__join_5;
} else {
break '__loop_203;
}
}
}
}
}
}
                                                                                                    if have >= 6 && left >= 258 {
RESTORE!(strm, put, left, next, have, state, hold, bits);
inflate_fast(strm, (*state).wsize);
LOAD!(put, strm, left, next, have, hold, state, bits);
} else {
// use inflate_fast() if we have enough input and output
// get a literal, length, or end-of-block code
'__loop_83: loop {
*__slate_slot_175 = *unsafe { (*state).lencode }.offset(BITS!(unsafe { (*state).lenbits }, hold) as isize);
if ((*__slate_slot_175).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_80;
}
}
if true {
have = have.wrapping_sub(1);
let __v72: *mut u8 = next;
next = __v72.add(1);
hold = hold.wrapping_add((*__v72 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_83;
}
}
}
}
}
if (*__slate_slot_175).op != 0 && (((*__slate_slot_175).op as u32) as i32) & 240 == 0 {
*__slate_slot_174 = *__slate_slot_175;
'__loop_75: loop {
*__slate_slot_175 = *unsafe { (*state).lencode }.offset(((((*__slate_slot_174).val as u32) as i32) as u32).wrapping_add(BITS!((((*__slate_slot_174).bits as u32) as i32) + (((*__slate_slot_174).op as u32) as i32), hold) >> (((*__slate_slot_174).bits as u32) as i32)) as isize);
if (((((*__slate_slot_174).bits as u32) as i32) + (((*__slate_slot_175).bits as u32) as i32)) as u32) <= bits {
break;
} else {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_72;
}
}
if true {
have = have.wrapping_sub(1);
let __v63: *mut u8 = next;
next = __v63.add(1);
hold = hold.wrapping_add((*__v63 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_75;
}
}
}
}
}
DROPBITS__2!((*__slate_slot_174).bits, hold, bits);
}
DROPBITS__2!((*__slate_slot_175).bits, hold, bits);
(*state).length = (*__slate_slot_175).val as u32;
// process literal
if (((*__slate_slot_175).op as u32) as i32) == 0 {
#[cfg(feature = "zlib-debug")]
{
Tracevv__2!(slate_support::stderr(), if (((*__slate_slot_175).val as u32) as i32) >= 32 && (((*__slate_slot_175).val as u32) as i32) < 127 { c"inflate:         literal '%c'\n".as_ptr() as *mut c_char } else { c"inflate:         literal 0x%02x\n".as_ptr() as *mut c_char }, (*__slate_slot_175).val);
}
loop {
if left == 0 {
put = (*state).window;
left = (*state).wsize;
(*state).whave = left;
if out.unwrap()(out_desc, put, left) != 0 {
break '__join_64;
}
}
if true {
break;
}
}
let __v56: *mut u8 = put;
put = __v56.add(1);
*__v56 = (*state).length as u8;
left = left.wrapping_sub(1);
(*state).mode = 16200;
} else {
// process end of block
if (((*__slate_slot_175).op as u32) as i32) & 32 != 0 {
#[cfg(feature = "zlib-debug")]
{
Tracevv__3!(slate_support::stderr(), c"inflate:         end of block\n".as_ptr() as *mut c_char);
}
(*state).mode = 16191;
} else {
// invalid code
if (((*__slate_slot_175).op as u32) as i32) & 64 != 0 {
(*strm).msg = c"invalid literal/length code".as_ptr() as *mut c_char;
(*state).mode = 16209;
} else {
// length code -- get extra bits, if any
(*state).extra = ((*__slate_slot_175).op as u32) & 15;
if (*state).extra != 0 {
'__loop_51: loop {
if bits < ((*state).extra as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_54;
}
}
if true {
have = have.wrapping_sub(1);
let __v50: *mut u8 = next;
next = __v50.add(1);
hold = hold.wrapping_add((*__v50 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_51;
}
}
}
} else {
if true {
break;
}
}
}
let __v42: u32 = (*state).length;
let __v41: u32 = __v42.wrapping_add(BITS!(unsafe { (*state).extra }, hold));
(*state).length = __v41;
DROPBITS!(unsafe { (*state).extra }, hold, bits);
}
#[cfg(feature = "zlib-debug")]
{
Tracevv!(slate_support::stderr(), c"inflate:         length %u\n".as_ptr() as *mut c_char, unsafe { (*state).length });
}
// get distance code
'__loop_47: loop {
*__slate_slot_175 = *unsafe { (*state).distcode }.offset(BITS!(unsafe { (*state).distbits }, hold) as isize);
if ((*__slate_slot_175).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_44;
}
}
if true {
have = have.wrapping_sub(1);
let __v38: *mut u8 = next;
next = __v38.add(1);
hold = hold.wrapping_add((*__v38 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_47;
}
}
}
}
}
if (((*__slate_slot_175).op as u32) as i32) & 240 == 0 {
*__slate_slot_174 = *__slate_slot_175;
'__loop_39: loop {
*__slate_slot_175 = *unsafe { (*state).distcode }.offset(((((*__slate_slot_174).val as u32) as i32) as u32).wrapping_add(BITS!((((*__slate_slot_174).bits as u32) as i32) + (((*__slate_slot_174).op as u32) as i32), hold) >> (((*__slate_slot_174).bits as u32) as i32)) as isize);
if (((((*__slate_slot_174).bits as u32) as i32) + (((*__slate_slot_175).bits as u32) as i32)) as u32) <= bits {
break;
} else {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_36;
}
}
if true {
have = have.wrapping_sub(1);
let __v29: *mut u8 = next;
next = __v29.add(1);
hold = hold.wrapping_add((*__v29 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_39;
}
}
}
}
}
DROPBITS__2!((*__slate_slot_174).bits, hold, bits);
}
DROPBITS__2!((*__slate_slot_175).bits, hold, bits);
if (((*__slate_slot_175).op as u32) as i32) & 64 != 0 {
(*strm).msg = c"invalid distance code".as_ptr() as *mut c_char;
(*state).mode = 16209;
} else {
(*state).offset = (*__slate_slot_175).val as u32;
// get distance extra bits, if any
(*state).extra = ((*__slate_slot_175).op as u32) & 15;
if (*state).extra != 0 {
'__loop_24: loop {
if bits < ((*state).extra as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_27;
}
}
if true {
have = have.wrapping_sub(1);
let __v20: *mut u8 = next;
next = __v20.add(1);
hold = hold.wrapping_add((*__v20 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_24;
}
}
}
} else {
if true {
break;
}
}
}
let __v12: u32 = (*state).offset;
let __v11: u32 = __v12.wrapping_add(BITS!(unsafe { (*state).extra }, hold));
(*state).offset = __v11;
DROPBITS!(unsafe { (*state).extra }, hold, bits);
}
if (*state).offset > unsafe { (*state).wsize }.wrapping_sub(if (*state).whave < (*state).wsize { left } else { 0 }) {
(*strm).msg = c"invalid distance too far back".as_ptr() as *mut c_char;
(*state).mode = 16209;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracevv!(slate_support::stderr(), c"inflate:         distance %u\n".as_ptr() as *mut c_char, unsafe { (*state).offset });
}
// copy match from window to output
loop {
if left == 0 {
put = (*state).window;
left = (*state).wsize;
(*state).whave = left;
if out.unwrap()(out_desc, put, left) != 0 {
break '__join_16;
}
}
if true {
copy = unsafe { (*state).wsize }.wrapping_sub((*state).offset);
if copy < left {
from = put.add(copy as usize);
copy = left.wrapping_sub(copy);
} else {
from = put.offset(-((*state).offset as isize));
copy = left;
}
if copy > (*state).length {
copy = (*state).length;
}
let __v9: u32 = (*state).length;
(*state).length = __v9.wrapping_sub(copy);
left = left.wrapping_sub(copy);
loop {
let __v5: *mut u8 = from;
from = __v5.add(1);
let __v3: *mut u8 = put;
put = __v3.add(1);
*__v3 = *__v5;
let __v0: u32 = copy.wrapping_sub(1);
copy = __v0;
if __v0 == 0 {
break;
}
}
if (*state).length == 0 {
continue '__loop_203;
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
                                                                                                ret = -2; // can't happen, but makes compilers happy
                                                                                                break '__join_3;
                                                                                            }
                                                                                            ret =
                                                                                                -3;
                                                                                            break '__join_3;
                                                                                        }
                                                                                        ret = 1; // inflate stream terminated properly
                                                                                        break '__join_3;
                                                                                    }
                                                                                    next = std::ptr::null_mut::<u8>();
                                                                                    ret = -5;
                                                                                    break '__join_3;
                                                                                }
                                                                                next = std::ptr::null_mut::<u8>();
                                                                                ret = -5;
                                                                                break '__join_3;
                                                                            }
                                                                            next =
                                                                                std::ptr::null_mut::<
                                                                                    u8,
                                                                                >(
                                                                                );
                                                                            ret = -5;
                                                                            break '__join_3;
                                                                        }
                                                                        next = std::ptr::null_mut::<
                                                                            u8,
                                                                        >(
                                                                        );
                                                                        ret = -5;
                                                                        break '__join_3;
                                                                    }
                                                                    next =
                                                                        std::ptr::null_mut::<u8>();
                                                                    ret = -5;
                                                                    break '__join_3;
                                                                }
                                                                next = std::ptr::null_mut::<u8>();
                                                                ret = -5;
                                                                break '__join_3;
                                                            }
                                                            next = std::ptr::null_mut::<u8>();
                                                            ret = -5;
                                                            break '__join_3;
                                                        }
                                                        ret = -5;
                                                        break '__join_3;
                                                    }
                                                    next = std::ptr::null_mut::<u8>();
                                                    ret = -5;
                                                    break '__join_3;
                                                }
                                                next = std::ptr::null_mut::<u8>();
                                                ret = -5;
                                                break '__join_3;
                                            }
                                            ret = -5;
                                            break '__join_3;
                                        }
                                        next = std::ptr::null_mut::<u8>();
                                        ret = -5;
                                        break '__join_3;
                                    }
                                    next = std::ptr::null_mut::<u8>();
                                    ret = -5;
                                    break '__join_3;
                                }
                                next = std::ptr::null_mut::<u8>();
                                ret = -5;
                                break '__join_3;
                            }
                            ret = -5;
                            break '__join_3;
                        }
                        next = std::ptr::null_mut::<u8>();
                        ret = -5;
                        break '__join_3;
                    }
                    next = std::ptr::null_mut::<u8>();
                    ret = -5;
                    break '__join_3;
                }
                next = std::ptr::null_mut::<u8>();
                ret = -5;
            }
            if left < (*state).wsize {
                if out.unwrap()(
                    out_desc,
                    (*state).window,
                    unsafe { (*state).wsize }.wrapping_sub(left),
                ) != 0
                    && ret == 1
                {
                    ret = -5;
                }
            }
            (*strm).next_in = next;
            (*strm).avail_in = have;
            return ret;
        }
    }
    // Write leftover output and return unused input
}

static mut order: __SlateAlignLargeArray<[u16; 19]> = __SlateAlignLargeArray([
    16u16, 17u16, 18u16, 0u16, 8u16, 7u16, 9u16, 6u16, 10u16, 5u16, 11u16, 4u16, 12u16, 3u16,
    13u16, 2u16, 14u16, 1u16, 15u16,
]);

#[unsafe(no_mangle)]
extern "C-unwind" fn inflateBackEnd(strm: *mut z_stream_s) -> i32 {
    if strm.is_null() || unsafe { (*strm).state }.is_null() || (unsafe { (*strm).zfree }) == None {
        return -2;
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
