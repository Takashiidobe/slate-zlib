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
    let __slate_slot_258: *mut __SlateRecord_inftrees_h_28 =
        std::ptr::addr_of_mut!(last) as *mut __SlateRecord_inftrees_h_28; // current decoding table entry
    let mut here: std::mem::MaybeUninit<__SlateRecord_inftrees_h_28> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_259: *mut __SlateRecord_inftrees_h_28 =
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
                '__join_218: {
                    '__join_201: {
                        '__join_190: {
                            '__join_186: {
                                '__join_175: {
                                    '__join_159: {
                                        '__join_115: {
                                            '__join_67: {
                                                '__join_57: {
                                                    '__join_28: {
                                                        '__join_16: {
                                                            '__join_38: {
                                                                '__join_46: {
                                                                    '__join_76: {
                                                                        '__join_84: {
                                                                            '__join_125: {
                                                                                '__join_135: {
                                                                                    '__join_143: {
                                                                                        '__join_6: {
                                                                                            '__join_5: {
                                                                                                '__loop_225: loop {
                                                                                                    let __t1: u32 = (*state).mode as u32;
                                                                                                    if __t1 == 16191 {
if (*state).last != 0 {
loop {
hold >>= bits & 7;
bits = bits.wrapping_sub(bits & 7);
if true {
break;
}
}
(*state).mode = 16208;
continue '__loop_225;
} else {
// determine and dispatch block type
'__loop_215: loop {
if bits < 3 {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_218;
}
}
if true {
have = have.wrapping_sub(1);
let __v249: *mut u8 = next;
next = __v249.add(1);
hold = hold.wrapping_add((*__v249 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_215;
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
loop {
hold >>= 1i32;
bits = bits.wrapping_sub(1);
if true {
break;
}
}
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
loop {
hold >>= 2i32;
bits = bits.wrapping_sub(2);
if true {
continue '__loop_225;
}
}
}
} else {
if __t1 == 16193 {
loop {
hold >>= bits & 7;
bits = bits.wrapping_sub(bits & 7);
if true {
break;
}
}
// get and verify stored block length
// go to byte boundary
'__loop_198: loop {
if bits < 32 {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_201;
}
}
if true {
have = have.wrapping_sub(1);
let __v228: *mut u8 = next;
next = __v228.add(1);
hold = hold.wrapping_add((*__v228 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_198;
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
continue '__loop_225;
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
break '__join_190;
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
break '__join_186;
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
let __v219: *mut u8 = next;
next = __v219.add(copy as usize);
left = left.wrapping_sub(copy);
let __v215: *mut u8 = put;
put = __v215.add(copy as usize);
let __v212: u32 = (*state).length;
(*state).length = __v212.wrapping_sub(copy);
}
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       stored end\n".as_ptr() as *mut c_char);
}
(*state).mode = 16191;
continue '__loop_225;
}
} else {
if __t1 == 16196 {
'__loop_172: loop {
if bits < 14 {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_175;
}
}
if true {
have = have.wrapping_sub(1);
let __v208: *mut u8 = next;
next = __v208.add(1);
hold = hold.wrapping_add((*__v208 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_172;
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
loop {
hold >>= 5i32;
bits = bits.wrapping_sub(5);
if true {
break;
}
}
(*state).ndist = BITS!(5i32, hold).wrapping_add(1);
loop {
hold >>= 5i32;
bits = bits.wrapping_sub(5);
if true {
break;
}
}
(*state).ncode = BITS!(4i32, hold).wrapping_add(4);
loop {
hold >>= 4i32;
bits = bits.wrapping_sub(4);
if true {
break;
}
}
#[cfg(not(feature = "pkzip-bug"))]
{
if (*state).nlen > 286 || (*state).ndist > 30 {
(*strm).msg = c"too many length or distance symbols".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_225;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       table sizes ok\n".as_ptr() as *mut c_char);
}
// get code length code lengths (not a typo)
(*state).have = 0;
'__loop_152: while (*state).have < (*state).ncode {
'__loop_156: loop {
if bits < 3 {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_159;
}
}
if true {
have = have.wrapping_sub(1);
let __v187: *mut u8 = next;
next = __v187.add(1);
hold = hold.wrapping_add((*__v187 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_156;
}
}
}
} else {
if true {
break;
}
}
}
let __v179: u32 = (*state).have;
(*state).have = __v179.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).offset(((order.0[__v179 as usize] as u32) as i32) as isize) = BITS!(3i32, hold) as u16;
loop {
hold >>= 3i32;
bits = bits.wrapping_sub(3);
if true {
continue '__loop_152;
}
}
}
while (*state).have < 19 {
let __v172: u32 = (*state).have;
(*state).have = __v172.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).offset(((order.0[__v172 as usize] as u32) as i32) as isize) = 0;
}
(*state).next = (*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28;
(*state).lencode = (*state).next as *const __SlateRecord_inftrees_h_28;
(*state).lenbits = 7;
ret = inflate_table(0u32, (*state).lens.as_mut_ptr() as *mut u16, 19u32, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).lenbits), (*state).work.as_mut_ptr() as *mut u16);
if ret != 0 {
(*strm).msg = c"invalid code lengths set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_225;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       code lengths ok\n".as_ptr() as *mut c_char);
}
// get length and distance code code lengths
(*state).have = 0;
'__join_98: {
'__join_108: {
'__loop_99: loop {
if (*state).have < unsafe { (*state).nlen }.wrapping_add((*state).ndist) {
'__loop_146: loop {
*__slate_slot_259 = *unsafe { (*state).lencode }.offset(BITS!(unsafe { (*state).lenbits }, hold) as isize);
if ((*__slate_slot_259).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_143;
}
}
if true {
have = have.wrapping_sub(1);
let __v168: *mut u8 = next;
next = __v168.add(1);
hold = hold.wrapping_add((*__v168 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_146;
}
}
}
}
}
if (((*__slate_slot_259).val as u32) as i32) < 16 {
loop {
hold >>= ((*__slate_slot_259).bits as u32) as i32;
bits = bits.wrapping_sub((*__slate_slot_259).bits as u32);
if true {
break;
}
}
let __v100: u32 = (*state).have;
(*state).have = __v100.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).add(__v100 as usize) = (*__slate_slot_259).val;
} else {
if (((*__slate_slot_259).val as u32) as i32) == 16 {
'__loop_112: loop {
if bits < (((((*__slate_slot_259).bits as u32) as i32) + 2) as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_115;
}
}
if true {
have = have.wrapping_sub(1);
let __v125: *mut u8 = next;
next = __v125.add(1);
hold = hold.wrapping_add((*__v125 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_112;
}
}
}
} else {
if true {
break;
}
}
}
loop {
hold >>= ((*__slate_slot_259).bits as u32) as i32;
bits = bits.wrapping_sub((*__slate_slot_259).bits as u32);
if true {
break;
}
}
if (*state).have == 0 {
break '__join_108;
} else {
len = *((*state).lens.as_mut_ptr() as *mut u16).offset(unsafe { (*state).have }.wrapping_sub(1) as isize) as u32;
copy = 3u32.wrapping_add(BITS!(2i32, hold));
loop {
hold >>= 2i32;
bits = bits.wrapping_sub(2);
if true {
break;
}
}
}
} else {
if (((*__slate_slot_259).val as u32) as i32) == 17 {
'__loop_122: loop {
if bits < (((((*__slate_slot_259).bits as u32) as i32) + 3) as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_125;
}
}
if true {
have = have.wrapping_sub(1);
let __v142: *mut u8 = next;
next = __v142.add(1);
hold = hold.wrapping_add((*__v142 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_122;
}
}
}
} else {
if true {
break;
}
}
}
loop {
hold >>= ((*__slate_slot_259).bits as u32) as i32;
bits = bits.wrapping_sub((*__slate_slot_259).bits as u32);
if true {
break;
}
}
len = 0;
copy = 3u32.wrapping_add(BITS!(3i32, hold));
loop {
hold >>= 3i32;
bits = bits.wrapping_sub(3);
if true {
break;
}
}
} else {
'__loop_132: loop {
if bits < (((((*__slate_slot_259).bits as u32) as i32) + 7) as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_135;
}
}
if true {
have = have.wrapping_sub(1);
let __v159: *mut u8 = next;
next = __v159.add(1);
hold = hold.wrapping_add((*__v159 as c_ulong) << bits);
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
loop {
hold >>= ((*__slate_slot_259).bits as u32) as i32;
bits = bits.wrapping_sub((*__slate_slot_259).bits as u32);
if true {
break;
}
}
len = 0;
copy = 11u32.wrapping_add(BITS!(7i32, hold));
loop {
hold >>= 7i32;
bits = bits.wrapping_sub(7);
if true {
break;
}
}
}
}
if unsafe { (*state).have }.wrapping_add(copy) > unsafe { (*state).nlen }.wrapping_add((*state).ndist) {
break;
} else {
loop {
let __v107: u32 = copy;
copy = __v107.wrapping_sub(1);
if __v107 != 0 {
let __v109: u32 = (*state).have;
(*state).have = __v109.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).add(__v109 as usize) = len as u16;
} else {
continue '__loop_99;
}
}
}
}
} else {
break '__join_98;
}
}
(*strm).msg = c"invalid bit length repeat".as_ptr() as *mut c_char;
(*state).mode = 16209;
break '__join_98;
}
(*strm).msg = c"invalid bit length repeat".as_ptr() as *mut c_char;
(*state).mode = 16209;
}
// handle error breaks in while
if ((*state).mode as u32) == 16209 {
continue '__loop_225;
} else {
// check for end-of-block code (better have one)
if ((*((*state).lens.as_mut_ptr() as *mut u16).add(256) as u32) as i32) == 0 {
(*strm).msg = c"invalid code -- missing end-of-block".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_225;
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
continue '__loop_225;
} else {
(*state).distcode = (*state).next as *const __SlateRecord_inftrees_h_28;
(*state).distbits = 6;
ret = inflate_table(2u32, ((*state).lens.as_mut_ptr() as *mut u16).offset((*state).nlen as isize), (*state).ndist, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).distbits), (*state).work.as_mut_ptr() as *mut u16);
if ret != 0 {
(*strm).msg = c"invalid distances set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_225;
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
'__loop_1: while (*state).have < (*state).ncode {
'__loop_2: loop {
if bits < 3 {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_159;
}
}
if true {
have = have.wrapping_sub(1);
let __v4: *mut u8 = next;
next = __v4.add(1);
hold = hold.wrapping_add((*__v4 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_2;
}
}
}
} else {
if true {
break;
}
}
}
let __v6: u32 = (*state).have;
(*state).have = __v6.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).offset(((order.0[__v6 as usize] as u32) as i32) as isize) = BITS!(3i32, hold) as u16;
loop {
hold >>= 3i32;
bits = bits.wrapping_sub(3);
if true {
continue '__loop_1;
}
}
}
while (*state).have < 19 {
let __v7: u32 = (*state).have;
(*state).have = __v7.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).offset(((order.0[__v7 as usize] as u32) as i32) as isize) = 0;
}
(*state).next = (*state).codes.as_mut_ptr() as *mut __SlateRecord_inftrees_h_28;
(*state).lencode = (*state).next as *const __SlateRecord_inftrees_h_28;
(*state).lenbits = 7;
ret = inflate_table(0u32, (*state).lens.as_mut_ptr() as *mut u16, 19u32, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).lenbits), (*state).work.as_mut_ptr() as *mut u16);
if ret != 0 {
(*strm).msg = c"invalid code lengths set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_225;
} else {
#[cfg(feature = "zlib-debug")]
{
Tracev!(slate_support::stderr(), c"inflate:       code lengths ok\n".as_ptr() as *mut c_char);
}
// get length and distance code code lengths
(*state).have = 0;
'__join_8: {
'__join_9: {
'__loop_10: loop {
if (*state).have < unsafe { (*state).nlen }.wrapping_add((*state).ndist) {
'__loop_11: loop {
*__slate_slot_259 = *unsafe { (*state).lencode }.offset(BITS!(unsafe { (*state).lenbits }, hold) as isize);
if ((*__slate_slot_259).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_143;
}
}
if true {
have = have.wrapping_sub(1);
let __v12: *mut u8 = next;
next = __v12.add(1);
hold = hold.wrapping_add((*__v12 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_11;
}
}
}
}
}
if (((*__slate_slot_259).val as u32) as i32) < 16 {
loop {
hold >>= ((*__slate_slot_259).bits as u32) as i32;
bits = bits.wrapping_sub((*__slate_slot_259).bits as u32);
if true {
break;
}
}
let __v13: u32 = (*state).have;
(*state).have = __v13.wrapping_add(1);
*((*state).lens.as_mut_ptr() as *mut u16).add(__v13 as usize) = (*__slate_slot_259).val;
} else {
if (((*__slate_slot_259).val as u32) as i32) == 16 {
'__loop_14: loop {
if bits < (((((*__slate_slot_259).bits as u32) as i32) + 2) as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_115;
}
}
if true {
have = have.wrapping_sub(1);
let __v17: *mut u8 = next;
next = __v17.add(1);
hold = hold.wrapping_add((*__v17 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_14;
}
}
}
} else {
if true {
break;
}
}
}
loop {
hold >>= ((*__slate_slot_259).bits as u32) as i32;
bits = bits.wrapping_sub((*__slate_slot_259).bits as u32);
if true {
break;
}
}
if (*state).have == 0 {
break '__join_9;
} else {
len = *((*state).lens.as_mut_ptr() as *mut u16).offset(unsafe { (*state).have }.wrapping_sub(1) as isize) as u32;
copy = 3u32.wrapping_add(BITS!(2i32, hold));
loop {
hold >>= 2i32;
bits = bits.wrapping_sub(2);
if true {
break;
}
}
}
} else {
if (((*__slate_slot_259).val as u32) as i32) == 17 {
'__loop_18: loop {
if bits < (((((*__slate_slot_259).bits as u32) as i32) + 3) as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_125;
}
}
if true {
have = have.wrapping_sub(1);
let __v19: *mut u8 = next;
next = __v19.add(1);
hold = hold.wrapping_add((*__v19 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_18;
}
}
}
} else {
if true {
break;
}
}
}
loop {
hold >>= ((*__slate_slot_259).bits as u32) as i32;
bits = bits.wrapping_sub((*__slate_slot_259).bits as u32);
if true {
break;
}
}
len = 0;
copy = 3u32.wrapping_add(BITS!(3i32, hold));
loop {
hold >>= 3i32;
bits = bits.wrapping_sub(3);
if true {
break;
}
}
} else {
'__loop_20: loop {
if bits < (((((*__slate_slot_259).bits as u32) as i32) + 7) as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_135;
}
}
if true {
have = have.wrapping_sub(1);
let __v21: *mut u8 = next;
next = __v21.add(1);
hold = hold.wrapping_add((*__v21 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_20;
}
}
}
} else {
if true {
break;
}
}
}
loop {
hold >>= ((*__slate_slot_259).bits as u32) as i32;
bits = bits.wrapping_sub((*__slate_slot_259).bits as u32);
if true {
break;
}
}
len = 0;
copy = 11u32.wrapping_add(BITS!(7i32, hold));
loop {
hold >>= 7i32;
bits = bits.wrapping_sub(7);
if true {
break;
}
}
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
continue '__loop_10;
}
}
}
}
} else {
break '__join_8;
}
}
(*strm).msg = c"invalid bit length repeat".as_ptr() as *mut c_char;
(*state).mode = 16209;
break '__join_8;
}
(*strm).msg = c"invalid bit length repeat".as_ptr() as *mut c_char;
(*state).mode = 16209;
}
// handle error breaks in while
if ((*state).mode as u32) == 16209 {
continue '__loop_225;
} else {
// check for end-of-block code (better have one)
if ((*((*state).lens.as_mut_ptr() as *mut u16).add(256) as u32) as i32) == 0 {
(*strm).msg = c"invalid code -- missing end-of-block".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_225;
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
continue '__loop_225;
} else {
(*state).distcode = (*state).next as *const __SlateRecord_inftrees_h_28;
(*state).distbits = 6;
ret = inflate_table(2u32, ((*state).lens.as_mut_ptr() as *mut u16).offset((*state).nlen as isize), (*state).ndist, std::ptr::addr_of_mut!((*state).next), std::ptr::addr_of_mut!((*state).distbits), (*state).work.as_mut_ptr() as *mut u16);
if ret != 0 {
(*strm).msg = c"invalid distances set".as_ptr() as *mut c_char;
(*state).mode = 16209;
continue '__loop_225;
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
break '__loop_225;
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
'__loop_87: loop {
*__slate_slot_259 = *unsafe { (*state).lencode }.offset(BITS!(unsafe { (*state).lenbits }, hold) as isize);
if ((*__slate_slot_259).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_84;
}
}
if true {
have = have.wrapping_sub(1);
let __v96: *mut u8 = next;
next = __v96.add(1);
hold = hold.wrapping_add((*__v96 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_87;
}
}
}
}
}
if (*__slate_slot_259).op != 0 && (((*__slate_slot_259).op as u32) as i32) & 240 == 0 {
*__slate_slot_258 = *__slate_slot_259;
'__loop_79: loop {
*__slate_slot_259 = *unsafe { (*state).lencode }.offset(((((*__slate_slot_258).val as u32) as i32) as u32).wrapping_add(BITS!((((*__slate_slot_258).bits as u32) as i32) + (((*__slate_slot_258).op as u32) as i32), hold) >> (((*__slate_slot_258).bits as u32) as i32)) as isize);
if (((((*__slate_slot_258).bits as u32) as i32) + (((*__slate_slot_259).bits as u32) as i32)) as u32) <= bits {
break;
} else {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_76;
}
}
if true {
have = have.wrapping_sub(1);
let __v87: *mut u8 = next;
next = __v87.add(1);
hold = hold.wrapping_add((*__v87 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_79;
}
}
}
}
}
loop {
hold >>= ((*__slate_slot_258).bits as u32) as i32;
bits = bits.wrapping_sub((*__slate_slot_258).bits as u32);
if true {
break;
}
}
}
loop {
hold >>= ((*__slate_slot_259).bits as u32) as i32;
bits = bits.wrapping_sub((*__slate_slot_259).bits as u32);
if true {
break;
}
}
(*state).length = (*__slate_slot_259).val as u32;
// process literal
if (((*__slate_slot_259).op as u32) as i32) == 0 {
#[cfg(feature = "zlib-debug")]
{
Tracevv__2!(slate_support::stderr(), if (((*__slate_slot_259).val as u32) as i32) >= 32 && (((*__slate_slot_259).val as u32) as i32) < 127 { c"inflate:         literal '%c'\n".as_ptr() as *mut c_char } else { c"inflate:         literal 0x%02x\n".as_ptr() as *mut c_char }, (*__slate_slot_259).val);
}
loop {
if left == 0 {
put = (*state).window;
left = (*state).wsize;
(*state).whave = left;
if out.unwrap()(out_desc, put, left) != 0 {
break '__join_67;
}
}
if true {
break;
}
}
let __v72: *mut u8 = put;
put = __v72.add(1);
*__v72 = (*state).length as u8;
left = left.wrapping_sub(1);
(*state).mode = 16200;
} else {
// process end of block
if (((*__slate_slot_259).op as u32) as i32) & 32 != 0 {
#[cfg(feature = "zlib-debug")]
{
Tracevv__3!(slate_support::stderr(), c"inflate:         end of block\n".as_ptr() as *mut c_char);
}
(*state).mode = 16191;
} else {
// invalid code
if (((*__slate_slot_259).op as u32) as i32) & 64 != 0 {
(*strm).msg = c"invalid literal/length code".as_ptr() as *mut c_char;
(*state).mode = 16209;
} else {
// length code -- get extra bits, if any
(*state).extra = ((*__slate_slot_259).op as u32) & 15;
if (*state).extra != 0 {
'__loop_54: loop {
if bits < ((*state).extra as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_57;
}
}
if true {
have = have.wrapping_sub(1);
let __v66: *mut u8 = next;
next = __v66.add(1);
hold = hold.wrapping_add((*__v66 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_54;
}
}
}
} else {
if true {
break;
}
}
}
let __v58: u32 = (*state).length;
let __v57: u32 = __v58.wrapping_add(BITS!(unsafe { (*state).extra }, hold));
(*state).length = __v57;
loop {
hold >>= (*state).extra;
bits = bits.wrapping_sub((*state).extra as u32);
if true {
break;
}
}
}
#[cfg(feature = "zlib-debug")]
{
Tracevv!(slate_support::stderr(), c"inflate:         length %u\n".as_ptr() as *mut c_char, unsafe { (*state).length });
}
// get distance code
'__loop_49: loop {
*__slate_slot_259 = *unsafe { (*state).distcode }.offset(BITS!(unsafe { (*state).distbits }, hold) as isize);
if ((*__slate_slot_259).bits as u32) <= bits {
break;
} else {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_46;
}
}
if true {
have = have.wrapping_sub(1);
let __v50: *mut u8 = next;
next = __v50.add(1);
hold = hold.wrapping_add((*__v50 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_49;
}
}
}
}
}
if (((*__slate_slot_259).op as u32) as i32) & 240 == 0 {
*__slate_slot_258 = *__slate_slot_259;
'__loop_41: loop {
*__slate_slot_259 = *unsafe { (*state).distcode }.offset(((((*__slate_slot_258).val as u32) as i32) as u32).wrapping_add(BITS!((((*__slate_slot_258).bits as u32) as i32) + (((*__slate_slot_258).op as u32) as i32), hold) >> (((*__slate_slot_258).bits as u32) as i32)) as isize);
if (((((*__slate_slot_258).bits as u32) as i32) + (((*__slate_slot_259).bits as u32) as i32)) as u32) <= bits {
break;
} else {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_38;
}
}
if true {
have = have.wrapping_sub(1);
let __v41: *mut u8 = next;
next = __v41.add(1);
hold = hold.wrapping_add((*__v41 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_41;
}
}
}
}
}
loop {
hold >>= ((*__slate_slot_258).bits as u32) as i32;
bits = bits.wrapping_sub((*__slate_slot_258).bits as u32);
if true {
break;
}
}
}
loop {
hold >>= ((*__slate_slot_259).bits as u32) as i32;
bits = bits.wrapping_sub((*__slate_slot_259).bits as u32);
if true {
break;
}
}
if (((*__slate_slot_259).op as u32) as i32) & 64 != 0 {
(*strm).msg = c"invalid distance code".as_ptr() as *mut c_char;
(*state).mode = 16209;
} else {
(*state).offset = (*__slate_slot_259).val as u32;
// get distance extra bits, if any
(*state).extra = ((*__slate_slot_259).op as u32) & 15;
if (*state).extra != 0 {
'__loop_25: loop {
if bits < ((*state).extra as u32) {
loop {
if have == 0 {
have = r#in.unwrap()(in_desc, std::ptr::addr_of_mut!(next));
if have == 0 {
break '__join_28;
}
}
if true {
have = have.wrapping_sub(1);
let __v24: *mut u8 = next;
next = __v24.add(1);
hold = hold.wrapping_add((*__v24 as c_ulong) << bits);
bits = bits.wrapping_add(8);
if true {
continue '__loop_25;
}
}
}
} else {
if true {
break;
}
}
}
let __v16: u32 = (*state).offset;
let __v15: u32 = __v16.wrapping_add(BITS!(unsafe { (*state).extra }, hold));
(*state).offset = __v15;
loop {
hold >>= (*state).extra;
bits = bits.wrapping_sub((*state).extra as u32);
if true {
break;
}
}
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
continue '__loop_225;
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
