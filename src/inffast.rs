//! inffast.c -- fast decoding
//! Copyright (C) 1995-2026 Mark Adler
//! For conditions of distribution and use, see copyright notice in zlib.h
#[cfg(feature = "zlib-debug")]
unsafe extern "C" {
    static mut z_verbose: i32;
    fn fflush(_0: *mut slate_support::FILE) -> i32;
}

use core::ffi::c_char;
use core::ffi::c_ulong;
use slate_support::ptrdiff_t;

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

/// Decode literal, length, and distance codes and write out the resulting
/// literal and match bytes until either not enough input or output is
/// available, an end-of-block is encountered, or a data error is encountered.
/// When large enough input and output buffers are supplied to inflate(), for
/// example, a 16K input buffer and a 64K output buffer, more than 95% of the
/// inflate execution time is spent in this routine.
///
/// Entry assumptions:
///
///      state->mode == LEN
///      strm->avail_in >= 6
///      strm->avail_out >= 258
///      start >= strm->avail_out
///      state->bits < 8
///
/// On return, state->mode is one of:
///
///      LEN -- ran out of enough output space or enough available input
///      TYPE -- reached end of block code, inflate() to interpret next block
///      BAD -- error in block data
///
/// Notes:
///
///  - The maximum input bits used by a length/distance pair is 15 bits for the
///    length code, 5 bits for the length extra, 15 bits for the distance code,
///    and 13 bits for the distance extra.  This totals 48 bits, or six bytes.
///    Therefore if strm->avail_in >= 6, then there is enough input to avoid
///    checking for available input while decoding.
///
///  - The maximum bytes that a single length/distance pair can output is 258
///    bytes, which is the maximum length that can be coded.  inflate_fast()
///    requires strm->avail_out >= 258 for each loop to avoid checking for
///    output space.
#[unsafe(no_mangle)]
extern "C-unwind" fn inflate_fast(strm: *mut z_stream_s, start: u32) {
    // where to copy match from
    let mut from: *mut u8 = unsafe { std::mem::zeroed() };
    // match distance
    let mut dist: u32 = unsafe { std::mem::zeroed() };
    // match length, unused bytes
    let mut len: u32 = unsafe { std::mem::zeroed() };
    // code bits, operation, extra bits, or
    //
    //  window position, window bytes to copy
    let mut op: u32 = unsafe { std::mem::zeroed() };
    // retrieved table entry
    let mut here: *const __SlateRecord_inftrees_h_28 = unsafe { std::mem::zeroed() };
    // mask for first level of distance codes
    let mut dmask: u32 = unsafe { std::mem::zeroed() };
    // mask for first level of length codes
    let mut lmask: u32 = unsafe { std::mem::zeroed() };
    // local strm->distcode
    let mut dcode: *const __SlateRecord_inftrees_h_28 = unsafe { std::mem::zeroed() };
    // local strm->lencode
    let mut lcode: *const __SlateRecord_inftrees_h_28 = unsafe { std::mem::zeroed() };
    // local strm->bits
    let mut bits: u32 = unsafe { std::mem::zeroed() };
    // local strm->hold
    #[cfg(all(target_pointer_width = "64", unix))]
    let mut hold: u64 = unsafe { std::mem::zeroed() };
    #[cfg(any(target_pointer_width = "32", windows))]
    let mut hold: u32 = unsafe { std::mem::zeroed() };
    // allocated sliding window, if wsize != 0
    let mut window: *mut u8 = unsafe { std::mem::zeroed() };
    // window write index
    let mut wnext: u32 = unsafe { std::mem::zeroed() };
    // valid bytes in the window
    let mut whave: u32 = unsafe { std::mem::zeroed() };
    // window size or zero if not using window
    let mut wsize: u32 = unsafe { std::mem::zeroed() };
    #[cfg(feature = "inflate-strict")]
    let mut dmax: u32 = unsafe { std::mem::zeroed() };
    // while out < end, enough space available
    let mut end: *mut u8 = unsafe { std::mem::zeroed() };
    // inflate()'s initial strm->next_out
    let mut beg: *mut u8 = unsafe { std::mem::zeroed() };
    // local strm->next_out
    let mut out: *mut u8 = unsafe { std::mem::zeroed() };
    // have enough input while in < last
    let mut last: *mut u8 = unsafe { std::mem::zeroed() };
    // local strm->next_in
    let mut r#in: *mut u8 = unsafe { std::mem::zeroed() };
    let mut state: *mut inflate_state = unsafe { std::mem::zeroed() };
    unsafe {
        // copy state to local variables
        state = (*strm).state as *mut inflate_state;
        r#in = (*strm).next_in;
        last = r#in.offset(unsafe { (*strm).avail_in }.wrapping_sub(5) as isize);
        out = (*strm).next_out;
        beg = out.offset(-(start.wrapping_sub((*strm).avail_out) as isize));
        end = out.offset(unsafe { (*strm).avail_out }.wrapping_sub(257) as isize);
        #[cfg(feature = "inflate-strict")]
        {
            dmax = (*state).dmax;
        }
        wsize = (*state).wsize;
        whave = (*state).whave;
        wnext = (*state).wnext;
        window = (*state).window;
        hold = (*state).hold;
        bits = (*state).bits;
        lcode = (*state).lencode;
        dcode = (*state).distcode;
        lmask = (1u32 << (*state).lenbits).wrapping_sub(1);
        dmask = (1u32 << (*state).distbits).wrapping_sub(1);
        // decode literals and length/distances until end-of-block or not enough
        // input data or output space
        '__join_0: {
            '__join_41: {
                '__join_32: {
                    #[cfg(feature = "inflate-strict")]
                    {
                        '__join_47: {
                            '__loop_66: loop {
                                if bits < 15 {
                                    let __v180: *mut u8 = r#in;
                                    r#in = __v180.add(1);
                                    hold = hold.wrapping_add((*__v180 as c_ulong) << bits);
                                    bits = bits.wrapping_add(8);
                                    let __v173: *mut u8 = r#in;
                                    r#in = __v173.add(1);
                                    hold = hold.wrapping_add((*__v173 as c_ulong) << bits);
                                    bits = bits.wrapping_add(8);
                                }
                                here = lcode.add((hold & (lmask as c_ulong)) as usize);
                                '__join_1: {
                                    '__join_2: {
                                        loop {
                                            op = (*here).bits as u32;
                                            hold >>= op;
                                            bits = bits.wrapping_sub(op);
                                            op = (*here).op as u32;
                                            if op == 0 {
                                                break '__join_2;
                                            } else {
                                                if op & 16 != 0 {
                                                    break;
                                                } else {
                                                    if op & 64 == 0 {
                                                        // 2nd level length code
                                                        here = lcode
                                                            .offset(((*here).val as i32) as isize)
                                                            .add(
                                                                (hold
                                                                    & ((1u32 << op).wrapping_sub(1)
                                                                        as c_ulong))
                                                                    as usize,
                                                            );
                                                    } else {
                                                        break '__loop_66;
                                                    }
                                                }
                                            }
                                        }
                                        // length base
                                        len = (*here).val as u32;
                                        op &= 15; // number of extra bits
                                        if op != 0 {
                                            if bits < op {
                                                let __v160: *mut u8 = r#in;
                                                r#in = __v160.add(1);
                                                hold =
                                                    hold.wrapping_add((*__v160 as c_ulong) << bits);
                                                bits = bits.wrapping_add(8);
                                            }
                                            len = len.wrapping_add(
                                                (hold as u32) & (1u32 << op).wrapping_sub(1),
                                            );
                                            hold >>= op;
                                            bits = bits.wrapping_sub(op);
                                        }
                                        #[cfg(feature = "zlib-debug")]
                                        {
                                            Tracevv!(
                                                slate_support::stderr(),
                                                c"inflate:         length %u\n".as_ptr()
                                                    as *mut c_char,
                                                len
                                            );
                                        }
                                        if bits < 15 {
                                            let __v147: *mut u8 = r#in;
                                            r#in = __v147.add(1);
                                            hold = hold.wrapping_add((*__v147 as c_ulong) << bits);
                                            bits = bits.wrapping_add(8);
                                            let __v140: *mut u8 = r#in;
                                            r#in = __v140.add(1);
                                            hold = hold.wrapping_add((*__v140 as c_ulong) << bits);
                                            bits = bits.wrapping_add(8);
                                        }
                                        here = dcode.add((hold & (dmask as c_ulong)) as usize);
                                        loop {
                                            op = (*here).bits as u32;
                                            hold >>= op;
                                            bits = bits.wrapping_sub(op);
                                            op = (*here).op as u32;
                                            if op & 16 != 0 {
                                                break;
                                            } else {
                                                if op & 64 == 0 {
                                                    // 2nd level distance code
                                                    here = dcode
                                                        .offset(((*here).val as i32) as isize)
                                                        .add(
                                                            (hold
                                                                & ((1u32 << op).wrapping_sub(1)
                                                                    as c_ulong))
                                                                as usize,
                                                        );
                                                } else {
                                                    break '__join_47;
                                                }
                                            }
                                        }
                                        // distance base
                                        dist = (*here).val as u32;
                                        op &= 15; // number of extra bits
                                        if bits < op {
                                            let __v127: *mut u8 = r#in;
                                            r#in = __v127.add(1);
                                            hold = hold.wrapping_add((*__v127 as c_ulong) << bits);
                                            bits = bits.wrapping_add(8);
                                            if bits < op {
                                                let __v120: *mut u8 = r#in;
                                                r#in = __v120.add(1);
                                                hold =
                                                    hold.wrapping_add((*__v120 as c_ulong) << bits);
                                                bits = bits.wrapping_add(8);
                                            }
                                        }
                                        dist = dist.wrapping_add(
                                            (hold as u32) & (1u32 << op).wrapping_sub(1),
                                        );
                                        if dist > dmax {
                                            break '__join_41;
                                        } else {
                                            hold >>= op;
                                            bits = bits.wrapping_sub(op);
                                            #[cfg(feature = "zlib-debug")]
                                            {
                                                Tracevv!(
                                                    slate_support::stderr(),
                                                    c"inflate:         distance %u\n".as_ptr()
                                                        as *mut c_char,
                                                    dist
                                                );
                                            }
                                            op = (out.offset_from(beg as *mut u8) as ptrdiff_t)
                                                as u32; // max distance in output
                                            if dist > op {
                                                // see if copy from window
                                                op = dist.wrapping_sub(op); // distance back in window
                                                if op > whave {
                                                    if (*state).sane != 0 {
                                                        break '__join_32;
                                                    } else {
                                                        #[cfg(feature = "inflate-allow-invalid")]
                                                        {
                                                            if len <= op.wrapping_sub(whave) {
                                                                loop {
                                                                    let __v85: *mut u8 = out;
                                                                    out = __v85.add(1);
                                                                    *__v85 = 0;
                                                                    let __v82: u32 =
                                                                        len.wrapping_sub(1);
                                                                    len = __v82;
                                                                    if __v82 == 0 {
                                                                        break '__join_1;
                                                                    }
                                                                }
                                                            } else {
                                                                len = len.wrapping_sub(
                                                                    op.wrapping_sub(whave),
                                                                );
                                                                loop {
                                                                    let __v79: *mut u8 = out;
                                                                    out = __v79.add(1);
                                                                    *__v79 = 0;
                                                                    let __v76: u32 =
                                                                        op.wrapping_sub(1);
                                                                    op = __v76;
                                                                    if __v76 <= whave {
                                                                        break;
                                                                    }
                                                                }
                                                                if op == 0 {
                                                                    from = out.sub(dist as usize);
                                                                    loop {
                                                                        let __v75: *mut u8 = from;
                                                                        from = __v75.add(1);
                                                                        let __v73: *mut u8 = out;
                                                                        out = __v73.add(1);
                                                                        *__v73 = *__v75;
                                                                        let __v70: u32 =
                                                                            len.wrapping_sub(1);
                                                                        len = __v70;
                                                                        if __v70 == 0 {
                                                                            break '__join_1;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                from = window;
                                                if wnext == 0 {
                                                    // very common case
                                                    let __v39: *mut u8 = from;
                                                    from = __v39
                                                        .offset(wsize.wrapping_sub(op) as isize);
                                                    if op < len {
                                                        // some from window
                                                        len = len.wrapping_sub(op);
                                                        loop {
                                                            let __v35: *mut u8 = from;
                                                            from = __v35.add(1);
                                                            let __v33: *mut u8 = out;
                                                            out = __v33.add(1);
                                                            *__v33 = *__v35;
                                                            let __v30: u32 = op.wrapping_sub(1);
                                                            op = __v30;
                                                            if __v30 == 0 {
                                                                break;
                                                            }
                                                        }
                                                        from = out.sub(dist as usize); // rest from output
                                                    }
                                                } else {
                                                    if wnext < op {
                                                        // wrap around window
                                                        let __v59: *mut u8 = from;
                                                        from = __v59.offset(
                                                            wsize
                                                                .wrapping_add(wnext)
                                                                .wrapping_sub(op)
                                                                as isize,
                                                        );
                                                        op = op.wrapping_sub(wnext);
                                                        if op < len {
                                                            // some from end of window
                                                            len = len.wrapping_sub(op);
                                                            loop {
                                                                let __v53: *mut u8 = from;
                                                                from = __v53.add(1);
                                                                let __v51: *mut u8 = out;
                                                                out = __v51.add(1);
                                                                *__v51 = *__v53;
                                                                let __v48: u32 = op.wrapping_sub(1);
                                                                op = __v48;
                                                                if __v48 == 0 {
                                                                    break;
                                                                }
                                                            }
                                                            from = window;
                                                            if wnext < len {
                                                                // some from start of window
                                                                op = wnext;
                                                                len = len.wrapping_sub(op);
                                                                loop {
                                                                    let __v45: *mut u8 = from;
                                                                    from = __v45.add(1);
                                                                    let __v43: *mut u8 = out;
                                                                    out = __v43.add(1);
                                                                    *__v43 = *__v45;
                                                                    let __v40: u32 =
                                                                        op.wrapping_sub(1);
                                                                    op = __v40;
                                                                    if __v40 == 0 {
                                                                        break;
                                                                    }
                                                                }
                                                                from = out.sub(dist as usize); // rest from output
                                                            }
                                                        }
                                                    } else {
                                                        // contiguous in window
                                                        let __v69: *mut u8 = from;
                                                        from =
                                                            __v69.offset(
                                                                wnext.wrapping_sub(op) as isize
                                                            );
                                                        if op < len {
                                                            // some from window
                                                            len = len.wrapping_sub(op);
                                                            loop {
                                                                let __v65: *mut u8 = from;
                                                                from = __v65.add(1);
                                                                let __v63: *mut u8 = out;
                                                                out = __v63.add(1);
                                                                *__v63 = *__v65;
                                                                let __v60: u32 = op.wrapping_sub(1);
                                                                op = __v60;
                                                                if __v60 == 0 {
                                                                    break;
                                                                }
                                                            }
                                                            from = out.sub(dist as usize); // rest from output
                                                        }
                                                    }
                                                }
                                                while len > 2 {
                                                    let __v29: *mut u8 = from;
                                                    from = __v29.add(1);
                                                    let __v27: *mut u8 = out;
                                                    out = __v27.add(1);
                                                    *__v27 = *__v29;
                                                    let __v25: *mut u8 = from;
                                                    from = __v25.add(1);
                                                    let __v23: *mut u8 = out;
                                                    out = __v23.add(1);
                                                    *__v23 = *__v25;
                                                    let __v21: *mut u8 = from;
                                                    from = __v21.add(1);
                                                    let __v19: *mut u8 = out;
                                                    out = __v19.add(1);
                                                    *__v19 = *__v21;
                                                    len = len.wrapping_sub(3);
                                                }
                                                if len != 0 {
                                                    let __v15: *mut u8 = from;
                                                    from = __v15.add(1);
                                                    let __v13: *mut u8 = out;
                                                    out = __v13.add(1);
                                                    *__v13 = *__v15;
                                                    if len > 1 {
                                                        let __v11: *mut u8 = from;
                                                        from = __v11.add(1);
                                                        let __v9: *mut u8 = out;
                                                        out = __v9.add(1);
                                                        *__v9 = *__v11;
                                                        break '__join_1;
                                                    } else {
                                                        break '__join_1;
                                                    }
                                                } else {
                                                    break '__join_1;
                                                }
                                            } else {
                                                from = out.sub(dist as usize); // copy direct from output
                                                loop {
                                                    // minimum length is three
                                                    let __v107: *mut u8 = from;
                                                    from = __v107.add(1);
                                                    let __v105: *mut u8 = out;
                                                    out = __v105.add(1);
                                                    *__v105 = *__v107;
                                                    let __v103: *mut u8 = from;
                                                    from = __v103.add(1);
                                                    let __v101: *mut u8 = out;
                                                    out = __v101.add(1);
                                                    *__v101 = *__v103;
                                                    let __v99: *mut u8 = from;
                                                    from = __v99.add(1);
                                                    let __v97: *mut u8 = out;
                                                    out = __v97.add(1);
                                                    *__v97 = *__v99;
                                                    len = len.wrapping_sub(3);
                                                    if len <= 2 {
                                                        break;
                                                    }
                                                }
                                                if len != 0 {
                                                    let __v93: *mut u8 = from;
                                                    from = __v93.add(1);
                                                    let __v91: *mut u8 = out;
                                                    out = __v91.add(1);
                                                    *__v91 = *__v93;
                                                    if len > 1 {
                                                        let __v89: *mut u8 = from;
                                                        from = __v89.add(1);
                                                        let __v87: *mut u8 = out;
                                                        out = __v87.add(1);
                                                        *__v87 = *__v89;
                                                        break '__join_1;
                                                    } else {
                                                        break '__join_1;
                                                    }
                                                } else {
                                                    break '__join_1;
                                                }
                                            }
                                        }
                                    }
                                    // literal
                                    #[cfg(feature = "zlib-debug")]
                                    {
                                        Tracevv__2!(
                                            slate_support::stderr(),
                                            if ((*here).val as i32) >= 32
                                                && ((*here).val as i32) < 127
                                            {
                                                c"inflate:         literal '%c'\n".as_ptr()
                                                    as *mut c_char
                                            } else {
                                                c"inflate:         literal 0x%02x\n".as_ptr()
                                                    as *mut c_char
                                            },
                                            unsafe { (*here).val }
                                        );
                                    }
                                    let __v7: *mut u8 = out;
                                    out = __v7.add(1);
                                    *__v7 = (*here).val as u8;
                                }
                                if !(r#in < last && out < end) {
                                    break '__join_0;
                                }
                            }
                            if op & 32 != 0 {
                                // end-of-block
                                #[cfg(feature = "zlib-debug")]
                                {
                                    Tracevv__3!(
                                        slate_support::stderr(),
                                        c"inflate:         end of block\n".as_ptr() as *mut c_char
                                    );
                                }
                                (*state).mode = 16191;
                                break '__join_0;
                            } else {
                                (*strm).msg =
                                    c"invalid literal/length code".as_ptr() as *mut c_char;
                                (*state).mode = 16209;
                                break '__join_0;
                            }
                        }
                        (*strm).msg = c"invalid distance code".as_ptr() as *mut c_char;
                        (*state).mode = 16209;
                        break '__join_0;
                    }
                    #[cfg(not(feature = "inflate-strict"))]
                    {
                        '__loop_64: loop {
                            if bits < 15 {
                                let __v1: *mut u8 = r#in;
                                r#in = __v1.add(1);
                                hold = hold.wrapping_add((*__v1 as c_ulong) << bits);
                                bits = bits.wrapping_add(8);
                                let __v2: *mut u8 = r#in;
                                r#in = __v2.add(1);
                                hold = hold.wrapping_add((*__v2 as c_ulong) << bits);
                                bits = bits.wrapping_add(8);
                            }
                            here = lcode.add((hold & (lmask as c_ulong)) as usize);
                            '__join_3: {
                                '__join_4: {
                                    loop {
                                        op = (*here).bits as u32;
                                        hold >>= op;
                                        bits = bits.wrapping_sub(op);
                                        op = (*here).op as u32;
                                        if op == 0 {
                                            break '__join_4;
                                        } else {
                                            if op & 16 != 0 {
                                                break;
                                            } else {
                                                if op & 64 == 0 {
                                                    // 2nd level length code
                                                    here = lcode
                                                        .offset(((*here).val as i32) as isize)
                                                        .add(
                                                            (hold
                                                                & ((1u32 << op).wrapping_sub(1)
                                                                    as c_ulong))
                                                                as usize,
                                                        );
                                                } else {
                                                    break '__loop_64;
                                                }
                                            }
                                        }
                                    }
                                    // length base
                                    len = (*here).val as u32;
                                    op &= 15; // number of extra bits
                                    if op != 0 {
                                        if bits < op {
                                            let __v6: *mut u8 = r#in;
                                            r#in = __v6.add(1);
                                            hold = hold.wrapping_add((*__v6 as c_ulong) << bits);
                                            bits = bits.wrapping_add(8);
                                        }
                                        len = len.wrapping_add(
                                            (hold as u32) & (1u32 << op).wrapping_sub(1),
                                        );
                                        hold >>= op;
                                        bits = bits.wrapping_sub(op);
                                    }
                                    #[cfg(feature = "zlib-debug")]
                                    {
                                        Tracevv!(
                                            slate_support::stderr(),
                                            c"inflate:         length %u\n".as_ptr() as *mut c_char,
                                            len
                                        );
                                    }
                                    if bits < 15 {
                                        let __v8: *mut u8 = r#in;
                                        r#in = __v8.add(1);
                                        hold = hold.wrapping_add((*__v8 as c_ulong) << bits);
                                        bits = bits.wrapping_add(8);
                                        let __v10: *mut u8 = r#in;
                                        r#in = __v10.add(1);
                                        hold = hold.wrapping_add((*__v10 as c_ulong) << bits);
                                        bits = bits.wrapping_add(8);
                                    }
                                    here = dcode.add((hold & (dmask as c_ulong)) as usize);
                                    loop {
                                        op = (*here).bits as u32;
                                        hold >>= op;
                                        bits = bits.wrapping_sub(op);
                                        op = (*here).op as u32;
                                        if op & 16 != 0 {
                                            break;
                                        } else {
                                            if op & 64 == 0 {
                                                // 2nd level distance code
                                                here = dcode
                                                    .offset(((*here).val as i32) as isize)
                                                    .add(
                                                        (hold
                                                            & ((1u32 << op).wrapping_sub(1)
                                                                as c_ulong))
                                                            as usize,
                                                    );
                                            } else {
                                                break '__join_32;
                                            }
                                        }
                                    }
                                    // distance base
                                    dist = (*here).val as u32;
                                    op &= 15; // number of extra bits
                                    if bits < op {
                                        let __v12: *mut u8 = r#in;
                                        r#in = __v12.add(1);
                                        hold = hold.wrapping_add((*__v12 as c_ulong) << bits);
                                        bits = bits.wrapping_add(8);
                                        if bits < op {
                                            let __v14: *mut u8 = r#in;
                                            r#in = __v14.add(1);
                                            hold = hold.wrapping_add((*__v14 as c_ulong) << bits);
                                            bits = bits.wrapping_add(8);
                                        }
                                    }
                                    dist = dist
                                        .wrapping_add((hold as u32) & (1u32 << op).wrapping_sub(1));
                                    hold >>= op;
                                    bits = bits.wrapping_sub(op);
                                    #[cfg(feature = "zlib-debug")]
                                    {
                                        Tracevv!(
                                            slate_support::stderr(),
                                            c"inflate:         distance %u\n".as_ptr()
                                                as *mut c_char,
                                            dist
                                        );
                                    }
                                    op = (out.offset_from(beg as *mut u8) as ptrdiff_t) as u32; // max distance in output
                                    if dist > op {
                                        // see if copy from window
                                        op = dist.wrapping_sub(op); // distance back in window
                                        if op > whave {
                                            if (*state).sane != 0 {
                                                break '__join_41;
                                            } else {
                                                #[cfg(feature = "inflate-allow-invalid")]
                                                {
                                                    if len <= op.wrapping_sub(whave) {
                                                        loop {
                                                            let __v16: *mut u8 = out;
                                                            out = __v16.add(1);
                                                            *__v16 = 0;
                                                            let __v17: u32 = len.wrapping_sub(1);
                                                            len = __v17;
                                                            if __v17 == 0 {
                                                                break '__join_3;
                                                            }
                                                        }
                                                    } else {
                                                        len = len
                                                            .wrapping_sub(op.wrapping_sub(whave));
                                                        loop {
                                                            let __v18: *mut u8 = out;
                                                            out = __v18.add(1);
                                                            *__v18 = 0;
                                                            let __v20: u32 = op.wrapping_sub(1);
                                                            op = __v20;
                                                            if __v20 <= whave {
                                                                break;
                                                            }
                                                        }
                                                        if op == 0 {
                                                            from = out.sub(dist as usize);
                                                            loop {
                                                                let __v22: *mut u8 = from;
                                                                from = __v22.add(1);
                                                                let __v24: *mut u8 = out;
                                                                out = __v24.add(1);
                                                                *__v24 = *__v22;
                                                                let __v26: u32 =
                                                                    len.wrapping_sub(1);
                                                                len = __v26;
                                                                if __v26 == 0 {
                                                                    break '__join_3;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        from = window;
                                        if wnext == 0 {
                                            // very common case
                                            let __v28: *mut u8 = from;
                                            from = __v28.offset(wsize.wrapping_sub(op) as isize);
                                            if op < len {
                                                // some from window
                                                len = len.wrapping_sub(op);
                                                loop {
                                                    let __v31: *mut u8 = from;
                                                    from = __v31.add(1);
                                                    let __v32: *mut u8 = out;
                                                    out = __v32.add(1);
                                                    *__v32 = *__v31;
                                                    let __v34: u32 = op.wrapping_sub(1);
                                                    op = __v34;
                                                    if __v34 == 0 {
                                                        break;
                                                    }
                                                }
                                                from = out.sub(dist as usize); // rest from output
                                            }
                                        } else {
                                            if wnext < op {
                                                // wrap around window
                                                let __v36: *mut u8 = from;
                                                from = __v36.offset(
                                                    wsize.wrapping_add(wnext).wrapping_sub(op)
                                                        as isize,
                                                );
                                                op = op.wrapping_sub(wnext);
                                                if op < len {
                                                    // some from end of window
                                                    len = len.wrapping_sub(op);
                                                    loop {
                                                        let __v37: *mut u8 = from;
                                                        from = __v37.add(1);
                                                        let __v38: *mut u8 = out;
                                                        out = __v38.add(1);
                                                        *__v38 = *__v37;
                                                        let __v41: u32 = op.wrapping_sub(1);
                                                        op = __v41;
                                                        if __v41 == 0 {
                                                            break;
                                                        }
                                                    }
                                                    from = window;
                                                    if wnext < len {
                                                        // some from start of window
                                                        op = wnext;
                                                        len = len.wrapping_sub(op);
                                                        loop {
                                                            let __v42: *mut u8 = from;
                                                            from = __v42.add(1);
                                                            let __v44: *mut u8 = out;
                                                            out = __v44.add(1);
                                                            *__v44 = *__v42;
                                                            let __v46: u32 = op.wrapping_sub(1);
                                                            op = __v46;
                                                            if __v46 == 0 {
                                                                break;
                                                            }
                                                        }
                                                        from = out.sub(dist as usize); // rest from output
                                                    }
                                                }
                                            } else {
                                                // contiguous in window
                                                let __v47: *mut u8 = from;
                                                from =
                                                    __v47.offset(wnext.wrapping_sub(op) as isize);
                                                if op < len {
                                                    // some from window
                                                    len = len.wrapping_sub(op);
                                                    loop {
                                                        let __v49: *mut u8 = from;
                                                        from = __v49.add(1);
                                                        let __v50: *mut u8 = out;
                                                        out = __v50.add(1);
                                                        *__v50 = *__v49;
                                                        let __v52: u32 = op.wrapping_sub(1);
                                                        op = __v52;
                                                        if __v52 == 0 {
                                                            break;
                                                        }
                                                    }
                                                    from = out.sub(dist as usize); // rest from output
                                                }
                                            }
                                        }
                                        while len > 2 {
                                            let __v54: *mut u8 = from;
                                            from = __v54.add(1);
                                            let __v55: *mut u8 = out;
                                            out = __v55.add(1);
                                            *__v55 = *__v54;
                                            let __v56: *mut u8 = from;
                                            from = __v56.add(1);
                                            let __v57: *mut u8 = out;
                                            out = __v57.add(1);
                                            *__v57 = *__v56;
                                            let __v58: *mut u8 = from;
                                            from = __v58.add(1);
                                            let __v61: *mut u8 = out;
                                            out = __v61.add(1);
                                            *__v61 = *__v58;
                                            len = len.wrapping_sub(3);
                                        }
                                        if len != 0 {
                                            let __v62: *mut u8 = from;
                                            from = __v62.add(1);
                                            let __v64: *mut u8 = out;
                                            out = __v64.add(1);
                                            *__v64 = *__v62;
                                            if len > 1 {
                                                let __v66: *mut u8 = from;
                                                from = __v66.add(1);
                                                let __v67: *mut u8 = out;
                                                out = __v67.add(1);
                                                *__v67 = *__v66;
                                                break '__join_3;
                                            } else {
                                                break '__join_3;
                                            }
                                        } else {
                                            break '__join_3;
                                        }
                                    } else {
                                        from = out.sub(dist as usize); // copy direct from output
                                        loop {
                                            // minimum length is three
                                            let __v68: *mut u8 = from;
                                            from = __v68.add(1);
                                            let __v71: *mut u8 = out;
                                            out = __v71.add(1);
                                            *__v71 = *__v68;
                                            let __v72: *mut u8 = from;
                                            from = __v72.add(1);
                                            let __v74: *mut u8 = out;
                                            out = __v74.add(1);
                                            *__v74 = *__v72;
                                            let __v77: *mut u8 = from;
                                            from = __v77.add(1);
                                            let __v78: *mut u8 = out;
                                            out = __v78.add(1);
                                            *__v78 = *__v77;
                                            len = len.wrapping_sub(3);
                                            if len <= 2 {
                                                break;
                                            }
                                        }
                                        if len != 0 {
                                            let __v80: *mut u8 = from;
                                            from = __v80.add(1);
                                            let __v81: *mut u8 = out;
                                            out = __v81.add(1);
                                            *__v81 = *__v80;
                                            if len > 1 {
                                                let __v83: *mut u8 = from;
                                                from = __v83.add(1);
                                                let __v84: *mut u8 = out;
                                                out = __v84.add(1);
                                                *__v84 = *__v83;
                                                break '__join_3;
                                            } else {
                                                break '__join_3;
                                            }
                                        } else {
                                            break '__join_3;
                                        }
                                    }
                                }
                                // literal
                                #[cfg(feature = "zlib-debug")]
                                {
                                    Tracevv__2!(
                                        slate_support::stderr(),
                                        if ((*here).val as i32) >= 32 && ((*here).val as i32) < 127
                                        {
                                            c"inflate:         literal '%c'\n".as_ptr()
                                                as *mut c_char
                                        } else {
                                            c"inflate:         literal 0x%02x\n".as_ptr()
                                                as *mut c_char
                                        },
                                        unsafe { (*here).val }
                                    );
                                }
                                let __v86: *mut u8 = out;
                                out = __v86.add(1);
                                *__v86 = (*here).val as u8;
                            }
                            if !(r#in < last && out < end) {
                                break '__join_0;
                            }
                        }
                        if op & 32 != 0 {
                            // end-of-block
                            #[cfg(feature = "zlib-debug")]
                            {
                                Tracevv__3!(
                                    slate_support::stderr(),
                                    c"inflate:         end of block\n".as_ptr() as *mut c_char
                                );
                            }
                            (*state).mode = 16191;
                            break '__join_0;
                        } else {
                            (*strm).msg = c"invalid literal/length code".as_ptr() as *mut c_char;
                            (*state).mode = 16209;
                            break '__join_0;
                        }
                    }
                }
                #[cfg(feature = "inflate-strict")]
                {
                    (*strm).msg = c"invalid distance too far back".as_ptr() as *mut c_char;
                }
                #[cfg(not(feature = "inflate-strict"))]
                {
                    (*strm).msg = c"invalid distance code".as_ptr() as *mut c_char;
                }
                (*state).mode = 16209;
                break '__join_0;
            }
            (*strm).msg = c"invalid distance too far back".as_ptr() as *mut c_char;
            (*state).mode = 16209;
        }
        // return unused bytes (on entry, bits < 8, so in won't go too far back)
        len = bits >> 3i32;
        let __v5: *mut u8 = r#in;
        r#in = __v5.sub(len as usize);
        bits = bits.wrapping_sub(len << 3i32);
        hold &= (1u32 << bits).wrapping_sub(1) as c_ulong;
        // update state and return
        (*strm).next_in = r#in;
        (*strm).next_out = out;
        #[cfg(target_pointer_width = "64")]
        {
            (*strm).avail_in = (if r#in < last {
                5 + (last.offset_from(r#in as *mut u8) as ptrdiff_t)
            } else {
                5 - (r#in.offset_from(last as *mut u8) as ptrdiff_t)
            }) as u32;
            (*strm).avail_out = (if out < end {
                257 + (end.offset_from(out as *mut u8) as ptrdiff_t)
            } else {
                257 - (out.offset_from(end as *mut u8) as ptrdiff_t)
            }) as u32;
        }
        #[cfg(target_pointer_width = "32")]
        {
            (*strm).avail_in = (if r#in < last {
                5 + ((last.offset_from(r#in as *mut u8) as ptrdiff_t) as i32)
            } else {
                5 - ((r#in.offset_from(last as *mut u8) as ptrdiff_t) as i32)
            }) as u32;
            (*strm).avail_out = (if out < end {
                257 + ((end.offset_from(out as *mut u8) as ptrdiff_t) as i32)
            } else {
                257 - ((out.offset_from(end as *mut u8) as ptrdiff_t) as i32)
            }) as u32;
        }
        (*state).hold = hold;
        (*state).bits = bits;
        return;
    }
}

// inflate_fast() speedups that turned out slower (on a PowerPC G3 750CXe):
// - Using bit fields for code structure
// - Different op definition to avoid & for extra bits (do & for table bits)
// - Three separate decoding do-loops for direct, window, and wnext == 0
// - Special case for distance > 1 copies to do overlapped load and store copy
// - Explicit branch predictions (based on measured branch probabilities)
// - Deferring match copy and interspersed it with decoding subsequent codes
// - Swapping literal/length else
// - Swapping window/direct else
// - Larger unrolled copy loops (three is about right)
// - Moving len -= 3 statement into middle of loop
