//! trees.c -- output deflated data using Huffman coding
//! Copyright (C) 1995-2026 Jean-loup Gailly
//! detect_data_type() function provided freely by Cosmin Truta, 2006
//! For conditions of distribution and use, see copyright notice in zlib.h
#[cfg(feature = "zlib-debug")]
macro_rules! MAX_DIST {
    ($s:expr) => {
        unsafe { (*$s).w_size }.wrapping_sub(262u32)
    };
}

#[cfg(all(any(target_os = "linux", target_os = "macos"), feature = "zlib-debug"))]
macro_rules! Tracecv {
    ($c:expr, $x__0:expr, $x__1:expr, $x__2:expr) => {
        if (unsafe { z_verbose }) > 1 && ($c) != 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char, $x__2) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! Tracecv__2 {
    ($c:expr, $x__0:expr, $x__1:expr, $x__2:expr, $x__3:expr, $x__4:expr, $x__5:expr, $x__6:expr) => {
        if (unsafe { z_verbose }) > 1 && $c {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf(
                    $x__0,
                    ($x__1) as *const c_char,
                    $x__2,
                    $x__3,
                    $x__4,
                    (($x__5) as u32) as i32,
                    $x__6,
                )
            };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(all(
    not(any(target_os = "linux", target_os = "macos")),
    feature = "zlib-debug"
))]
macro_rules! Tracecv__3 {
    ($c:expr, $x__0:expr, $x__1:expr, $x__2:expr) => {
        let __v: bool = (unsafe { z_verbose }) > 1 && ($c) != 0;
        if __v {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char, $x__2) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! Tracev__4 {
    ($x__0:expr, $x__1:expr, $x__2:expr, $x__3:expr, $x__4:expr, $x__5:expr, $x__6:expr, $x__7:expr) => {
        if (unsafe { z_verbose }) > 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf(
                    $x__0,
                    ($x__1) as *const c_char,
                    $x__2,
                    $x__3,
                    $x__4,
                    $x__5,
                    $x__6,
                    $x__7,
                )
            };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! Tracev__3 {
    ($x__0:expr, $x__1:expr, $x__2:expr, $x__3:expr) => {
        if (unsafe { z_verbose }) > 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char, $x__2, $x__3) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! Tracev__5 {
    ($x__0:expr, $x__1:expr, $x__2:expr) => {
        if (unsafe { z_verbose }) > 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf($x__0, ($x__1) as *const c_char, (($x__2) as u32) as i32)
            };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! Tracev__6 {
    ($x__0:expr, $x__1:expr, $x__2:expr, $x__3:expr, $x__4:expr) => {
        if (unsafe { z_verbose }) > 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf(
                    $x__0,
                    ($x__1) as *const c_char,
                    $x__2,
                    (($x__3) as u32) as i32,
                    $x__4,
                )
            };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! Tracevv__4 {
    ($x__0:expr, $x__1:expr, $x__2:expr, $x__3:expr) => {
        if (unsafe { z_verbose }) > 1 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char, $x__2, $x__3) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

macro_rules! d_code {
    ($dist:expr) => {
        if ($dist) < 256 {
            ((unsafe { _dist_code.0[($dist) as usize] }) as u32) as i32
        } else {
            ((unsafe { _dist_code.0[256u32.wrapping_add(($dist) >> 7i32) as usize] }) as u32) as i32
        }
    };
}

macro_rules! pqremove {
    ($s:expr, $tree:expr, $top:expr) => {
        $top = unsafe { *((*$s).heap.as_mut_ptr() as *mut i32).add(1) };
        let __v: *mut internal_state = $s;
        let __v2: i32 = unsafe { (*__v).heap_len };
        unsafe {
            (*__v).heap_len = __v2 - 1;
        }
        unsafe {
            *((*$s).heap.as_mut_ptr() as *mut i32).add(1) =
                *((*$s).heap.as_mut_ptr() as *mut i32).offset(__v2 as isize);
        }
        pqdownheap($s, $tree, 1i32);
    };
}

macro_rules! put_short {
    ($s:expr, $w:expr) => {
        put_byte!($s, ((((($w) as u32) as i32) & 255) as i8) as u8);
        put_byte!($s, (((((($w) as u16) as u32) as i32) >> 8i32) as i8) as u8);
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! send_code__3 {
    ($s:expr, $c:expr, $tree:expr) => {
        if (unsafe { z_verbose }) > 2 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf(slate_support::stderr(), c"\ncd %3d ".as_ptr(), $c) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
        send_bits(
            $s,
            ((unsafe { (*$tree.offset(($c) as isize)).fc.code }) as u32) as i32,
            ((unsafe { (*$tree.offset(($c) as isize)).dl.len }) as u32) as i32,
        );
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! send_code {
    ($s:expr, $c:expr, $tree:expr) => {
        if (unsafe { z_verbose }) > 2 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf(slate_support::stderr(), c"\ncd %3d ".as_ptr(), $c) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
        send_bits(
            $s,
            ((unsafe {
                (*($tree.as_mut_ptr() as *mut ct_data_s).offset(($c) as isize))
                    .fc
                    .code
            }) as u32) as i32,
            ((unsafe {
                (*($tree.as_mut_ptr() as *mut ct_data_s).offset(($c) as isize))
                    .dl
                    .len
            }) as u32) as i32,
        );
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! send_code__6 {
    ($s:expr, $c:expr, $tree:expr) => {
        if (unsafe { z_verbose }) > 2 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf(slate_support::stderr(), c"\ncd %3d ".as_ptr(), $c) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
        send_bits(
            $s,
            ((unsafe { $tree[($c) as usize].fc.code }) as u32) as i32,
            ((unsafe { $tree[($c) as usize].dl.len }) as u32) as i32,
        );
    };
}

#[cfg(not(feature = "zlib-debug"))]
macro_rules! send_bits {
    ($s:expr, $value:expr, $length:expr) => {
        let len: i32 = $length;
        if (unsafe { (*$s).bi_valid }) > 16 - len {
            let val: i32 = ($value) as i32;
            let __v: *mut internal_state = $s;
            let __v2: u16 = unsafe { (*__v).bi_buf };
            let __v3: u16 = ((((__v2 as u32) as i32)
                | ((((val as i16) as u16) as u32) as i32) << unsafe { (*$s).bi_valid })
                as i16) as u16;
            unsafe {
                (*__v).bi_buf = __v3;
            }
            put_short!($s, unsafe { (*$s).bi_buf });
            unsafe {
                (*$s).bi_buf = ((((((val as i16) as u16) as u32) as i32) >> 16 - (*$s).bi_valid)
                    as i16) as u16;
            }
            let __v4: *mut internal_state = $s;
            unsafe {
                (*__v4).bi_valid = (*__v4).bi_valid + (len - 16);
            }
        } else {
            let __v7: *mut internal_state = $s;
            let __v8: u16 = unsafe { (*__v7).bi_buf };
            let __v9: u16 = ((((__v8 as u32) as i32)
                | (((($value) as u16) as u32) as i32) << unsafe { (*$s).bi_valid })
                as i16) as u16;
            unsafe {
                (*__v7).bi_buf = __v9;
            }
            let __v10: *mut internal_state = $s;
            unsafe {
                (*__v10).bi_valid = (*__v10).bi_valid + len;
            }
        }
    };
}

#[cfg(not(feature = "zlib-debug"))]
macro_rules! send_bits__2 {
    ($s:expr, $value:expr, $length:expr) => {
        let len: i32 = (($length) as u32) as i32;
        if (unsafe { (*$s).bi_valid }) > 16 - len {
            let val: i32 = ($value) as i32;
            let __v: *mut internal_state = $s;
            let __v2: u16 = unsafe { (*__v).bi_buf };
            let __v3: u16 = ((((__v2 as u32) as i32)
                | ((((val as i16) as u16) as u32) as i32) << unsafe { (*$s).bi_valid })
                as i16) as u16;
            unsafe {
                (*__v).bi_buf = __v3;
            }
            put_short!($s, unsafe { (*$s).bi_buf });
            unsafe {
                (*$s).bi_buf = ((((((val as i16) as u16) as u32) as i32) >> 16 - (*$s).bi_valid)
                    as i16) as u16;
            }
            let __v4: *mut internal_state = $s;
            unsafe {
                (*__v4).bi_valid = (*__v4).bi_valid + (len - 16);
            }
        } else {
            let __v7: *mut internal_state = $s;
            let __v8: u16 = unsafe { (*__v7).bi_buf };
            let __v9: u16 = ((((__v8 as u32) as i32)
                | (((($value) as u16) as u32) as i32) << unsafe { (*$s).bi_valid })
                as i16) as u16;
            unsafe {
                (*__v7).bi_buf = __v9;
            }
            let __v10: *mut internal_state = $s;
            unsafe {
                (*__v10).bi_valid = (*__v10).bi_valid + len;
            }
        }
    };
}

#[cfg(not(feature = "zlib-debug"))]
macro_rules! send_code__5 {
    ($s:expr, $c:expr, $tree:expr) => {
        send_bits__2!($s, unsafe { $tree[($c) as usize].fc.code }, unsafe {
            $tree[($c) as usize].dl.len
        });
    };
}

#[cfg(not(feature = "zlib-debug"))]
macro_rules! send_code__4 {
    ($s:expr, $c:expr, $tree:expr) => {
        send_bits__2!(
            $s,
            unsafe { (*$tree.offset(($c) as isize)).fc.code },
            unsafe { (*$tree.offset(($c) as isize)).dl.len }
        );
    };
}

#[cfg(not(feature = "zlib-debug"))]
macro_rules! send_code__2 {
    ($s:expr, $c:expr, $tree:expr) => {
        send_bits__2!(
            $s,
            unsafe {
                (*($tree.as_mut_ptr() as *mut ct_data_s).offset(($c) as isize))
                    .fc
                    .code
            },
            unsafe {
                (*($tree.as_mut_ptr() as *mut ct_data_s).offset(($c) as isize))
                    .dl
                    .len
            }
        );
    };
}

macro_rules! smaller {
    ($tree:expr, $n:expr, $m:expr, $depth:expr) => {
        (((unsafe { (*$tree.offset(($n) as isize)).fc.freq }) as u32) as i32)
            < (((unsafe { (*$tree.offset(($m) as isize)).fc.freq }) as u32) as i32)
            || (((unsafe { (*$tree.offset(($n) as isize)).fc.freq }) as u32) as i32)
                == (((unsafe { (*$tree.offset(($m) as isize)).fc.freq }) as u32) as i32)
                && (((unsafe { *($depth.as_mut_ptr() as *mut u8).offset(($n) as isize) }) as u32)
                    as i32)
                    <= (((unsafe { *($depth.as_mut_ptr() as *mut u8).offset(($m) as isize) })
                        as u32) as i32)
    };
}

#[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
unsafe extern "C" {
    fn memcpy(arg0: *mut (), arg1: *const (), arg2: size_t) -> *mut ();
}

#[cfg(feature = "zlib-debug")]
unsafe extern "C" {
    static mut z_verbose: i32;
    fn z_error(arg0: *mut c_char);
    fn fflush(_0: *mut slate_support::FILE) -> i32;
}

use core::ffi::c_char;
use core::ffi::c_long;
use core::ffi::c_ulong;
#[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
use slate_support::size_t;

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
struct ct_data_s {
    fc: __SlateRecord_deflate_h_81,
    dl: __SlateRecord_deflate_h_81_2,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct tree_desc_s {
    dyn_tree: *mut ct_data_s,
    max_code: i32,
    stat_desc: *const static_tree_desc_s,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_deflate_h_81 {
    freq: u16,
    code: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord_deflate_h_81_2 {
    dad: u16,
    len: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct internal_state {
    strm: *mut z_stream_s,
    status: i32,
    pending_buf: *mut u8,
    pending_buf_size: c_ulong,
    pending_out: *mut u8,
    pending: c_ulong,
    wrap: i32,
    gzhead: *mut gz_header_s,
    gzindex: c_ulong,
    method: u8,
    last_flush: i32,
    w_size: u32,
    w_bits: u32,
    w_mask: u32,
    window: *mut u8,
    window_size: c_ulong,
    prev: *mut u16,
    head: *mut u16,
    ins_h: u32,
    hash_size: u32,
    hash_bits: u32,
    hash_mask: u32,
    hash_shift: u32,
    block_start: c_long,
    match_length: u32,
    prev_match: u32,
    match_available: i32,
    strstart: u32,
    match_start: u32,
    lookahead: u32,
    prev_length: u32,
    max_chain_length: u32,
    max_lazy_match: u32,
    level: i32,
    strategy: i32,
    good_match: u32,
    nice_match: i32,
    dyn_ltree: [ct_data_s; 573],
    dyn_dtree: [ct_data_s; 61],
    bl_tree: [ct_data_s; 39],
    l_desc: tree_desc_s,
    d_desc: tree_desc_s,
    bl_desc: tree_desc_s,
    bl_count: [u16; 16],
    heap: [i32; 573],
    heap_len: i32,
    heap_max: i32,
    depth: [u8; 573],
    #[cfg(feature = "lit-mem")]
    d_buf: *mut u16,
    #[cfg(feature = "lit-mem")]
    l_buf: *mut u8,
    #[cfg(not(feature = "lit-mem"))]
    sym_buf: *mut u8,
    lit_bufsize: u32,
    sym_next: u32,
    sym_end: u32,
    opt_len: c_ulong,
    static_len: c_ulong,
    matches: u32,
    insert: u32,
    #[cfg(feature = "zlib-debug")]
    compressed_len: c_ulong,
    #[cfg(feature = "zlib-debug")]
    bits_sent: c_ulong,
    bi_buf: u16,
    bi_valid: i32,
    bi_used: i32,
    high_water: c_ulong,
    slid: i32,
}

//  ALGORITHM
//
//      The "deflation" process uses several Huffman trees. The more
//      common source values are represented by shorter bit sequences.
//
//      Each code tree is stored in a compressed form which is itself
// a Huffman encoding of the lengths of all the code strings (in
// ascending order by source values).  The actual code strings are
// reconstructed from the lengths in the inflate process, as described
// in the deflate specification.
//
//  REFERENCES
//
//      Deutsch, L.P.,"'Deflate' Compressed Data Format Specification".
//      Available in ftp.uu.net:/pub/archiving/zip/doc/deflate-1.1.doc
//
//      Storer, James A.
//          Data Compression:  Methods and Theory, pp. 49-50.
//          Computer Science Press, 1988.  ISBN 0-7167-8156-5.
//
//      Sedgewick, R.
//          Algorithms, p290.
//          Addison-Wesley, 1983. ISBN 0-201-06672-6.
// @(#) $Id$
// #define GEN_TREES_H
// Constants
// Bit length codes must not exceed MAX_BL_BITS bits
// end of block literal code
// repeat previous bit length 3-6 times (2 bits of repeat count)
// repeat a zero length 3-10 times  (3 bits of repeat count)
// repeat a zero length 11-138 times  (7 bits of repeat count)
/// extra bits for each length code
static mut extra_lbits: __SlateAlignLargeArray<[i32; 29]> = __SlateAlignLargeArray([
    0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 1i32, 1i32, 1i32, 1i32, 2i32, 2i32, 2i32, 2i32,
    3i32, 3i32, 3i32, 3i32, 4i32, 4i32, 4i32, 4i32, 5i32, 5i32, 5i32, 5i32, 0i32,
]);

#[cfg(target_arch = "x86_64")]
#[repr(C, align(16))]
struct __SlateAlignLargeArray<T>(T);

#[cfg(not(target_arch = "x86_64"))]
#[repr(C)]
struct __SlateAlignLargeArray<T>(T);

/// extra bits for each distance code
static mut extra_dbits: __SlateAlignLargeArray<[i32; 30]> = __SlateAlignLargeArray([
    0i32, 0i32, 0i32, 0i32, 1i32, 1i32, 2i32, 2i32, 3i32, 3i32, 4i32, 4i32, 5i32, 5i32, 6i32, 6i32,
    7i32, 7i32, 8i32, 8i32, 9i32, 9i32, 10i32, 10i32, 11i32, 11i32, 12i32, 12i32, 13i32, 13i32,
]);

/// extra bits for each bit length code
static mut extra_blbits: __SlateAlignLargeArray<[i32; 19]> = __SlateAlignLargeArray([
    0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32,
    2i32, 3i32, 7i32,
]);

static mut bl_order: __SlateAlignLargeArray<[u8; 19]> = __SlateAlignLargeArray([
    16u8, 17u8, 18u8, 0u8, 8u8, 7u8, 9u8, 6u8, 10u8, 5u8, 11u8, 4u8, 12u8, 3u8, 13u8, 2u8, 14u8,
    1u8, 15u8,
]);

static mut static_ltree: __SlateAlignLargeArray<[ct_data_s; 288]> = __SlateAlignLargeArray([
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 12u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 140u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 76u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 204u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 44u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 172u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 108u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 236u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 28u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 156u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 92u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 220u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 60u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 188u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 124u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 252u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 2u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 130u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 66u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 194u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 34u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 162u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 98u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 226u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 18u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 146u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 82u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 210u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 50u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 178u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 114u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 242u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 10u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 138u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 74u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 202u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 42u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 170u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 106u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 234u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 26u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 154u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 90u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 218u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 58u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 186u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 122u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 250u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 6u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 134u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 70u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 198u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 38u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 166u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 102u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 230u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 22u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 150u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 86u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 214u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 54u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 182u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 118u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 246u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 14u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 142u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 78u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 206u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 46u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 174u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 110u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 238u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 30u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 158u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 94u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 222u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 62u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 190u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 126u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 254u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 1u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 129u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 65u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 193u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 33u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 161u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 97u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 225u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 17u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 145u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 81u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 209u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 49u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 177u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 113u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 241u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 9u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 137u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 73u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 201u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 41u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 169u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 105u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 233u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 25u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 153u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 89u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 217u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 57u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 185u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 121u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 249u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 5u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 133u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 69u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 197u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 37u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 165u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 101u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 229u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 21u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 149u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 85u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 213u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 53u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 181u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 117u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 245u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 13u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 141u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 77u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 205u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 45u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 173u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 109u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 237u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 29u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 157u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 93u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 221u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 61u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 189u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 125u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 253u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 19u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 275u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 147u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 403u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 83u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 339u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 211u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 467u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 51u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 307u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 179u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 435u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 115u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 371u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 243u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 499u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 11u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 267u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 139u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 395u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 75u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 331u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 203u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 459u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 43u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 299u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 171u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 427u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 107u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 363u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 235u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 491u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 27u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 283u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 155u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 411u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 91u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 347u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 219u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 475u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 59u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 315u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 187u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 443u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 123u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 379u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 251u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 507u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 7u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 263u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 135u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 391u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 71u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 327u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 199u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 455u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 39u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 295u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 167u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 423u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 103u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 359u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 231u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 487u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 23u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 279u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 151u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 407u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 87u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 343u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 215u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 471u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 55u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 311u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 183u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 439u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 119u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 375u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 247u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 503u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 15u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 271u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 143u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 399u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 79u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 335u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 207u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 463u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 47u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 303u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 175u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 431u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 111u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 367u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 239u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 495u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 31u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 287u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 159u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 415u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 95u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 351u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 223u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 479u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 63u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 319u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 191u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 447u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 127u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 383u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 255u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 511u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 9u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 0u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 64u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 32u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 96u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 16u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 80u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 48u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 112u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 8u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 72u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 40u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 104u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 24u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 88u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 56u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 120u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 4u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 68u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 36u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 100u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 20u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 84u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 52u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 116u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 7u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 3u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 131u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 67u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 195u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 35u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 163u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 99u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 227u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 8u16 },
    },
]);

static mut static_dtree: __SlateAlignLargeArray<[ct_data_s; 30]> = __SlateAlignLargeArray([
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 0u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 16u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 8u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 24u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 4u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 20u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 12u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 28u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 2u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 18u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 10u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 26u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 6u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 22u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 14u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 30u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 1u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 17u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 9u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 25u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 5u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 21u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 13u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 29u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 3u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 19u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 11u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 27u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 7u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
    ct_data_s {
        fc: __SlateRecord_deflate_h_81 { freq: 23u16 },
        dl: __SlateRecord_deflate_h_81_2 { dad: 5u16 },
    },
]);

#[unsafe(no_mangle)]
static mut _dist_code: __SlateAlignLargeArray<[u8; 512]> = __SlateAlignLargeArray([
    0u8, 1u8, 2u8, 3u8, 4u8, 4u8, 5u8, 5u8, 6u8, 6u8, 6u8, 6u8, 7u8, 7u8, 7u8, 7u8, 8u8, 8u8, 8u8,
    8u8, 8u8, 8u8, 8u8, 8u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 9u8, 10u8, 10u8, 10u8, 10u8, 10u8,
    10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 10u8, 11u8, 11u8, 11u8, 11u8, 11u8,
    11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 11u8, 12u8, 12u8, 12u8, 12u8, 12u8,
    12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8,
    12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 12u8, 13u8, 13u8, 13u8, 13u8, 13u8,
    13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8,
    13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 13u8, 14u8, 14u8, 14u8, 14u8, 14u8,
    14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8,
    14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8,
    14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8,
    14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 14u8, 15u8, 15u8, 15u8, 15u8, 15u8,
    15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
    15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
    15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8,
    15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 15u8, 0u8, 0u8, 16u8, 17u8, 18u8,
    18u8, 19u8, 19u8, 20u8, 20u8, 20u8, 20u8, 21u8, 21u8, 21u8, 21u8, 22u8, 22u8, 22u8, 22u8, 22u8,
    22u8, 22u8, 22u8, 23u8, 23u8, 23u8, 23u8, 23u8, 23u8, 23u8, 23u8, 24u8, 24u8, 24u8, 24u8, 24u8,
    24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 25u8, 25u8, 25u8, 25u8, 25u8,
    25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 26u8, 26u8, 26u8, 26u8, 26u8,
    26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8,
    26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 27u8, 27u8, 27u8, 27u8, 27u8,
    27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8,
    27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 28u8, 28u8, 28u8, 28u8, 28u8,
    28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8,
    28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8,
    28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8,
    28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 28u8, 29u8, 29u8, 29u8, 29u8, 29u8,
    29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8,
    29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8,
    29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8,
    29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8, 29u8,
]);

#[unsafe(no_mangle)]
static mut _length_code: __SlateAlignLargeArray<[u8; 256]> = __SlateAlignLargeArray([
    0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 8u8, 9u8, 9u8, 10u8, 10u8, 11u8, 11u8, 12u8, 12u8,
    12u8, 12u8, 13u8, 13u8, 13u8, 13u8, 14u8, 14u8, 14u8, 14u8, 15u8, 15u8, 15u8, 15u8, 16u8, 16u8,
    16u8, 16u8, 16u8, 16u8, 16u8, 16u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 17u8, 18u8, 18u8,
    18u8, 18u8, 18u8, 18u8, 18u8, 18u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 19u8, 20u8, 20u8,
    20u8, 20u8, 20u8, 20u8, 20u8, 20u8, 20u8, 20u8, 20u8, 20u8, 20u8, 20u8, 20u8, 20u8, 21u8, 21u8,
    21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 21u8, 22u8, 22u8,
    22u8, 22u8, 22u8, 22u8, 22u8, 22u8, 22u8, 22u8, 22u8, 22u8, 22u8, 22u8, 22u8, 22u8, 23u8, 23u8,
    23u8, 23u8, 23u8, 23u8, 23u8, 23u8, 23u8, 23u8, 23u8, 23u8, 23u8, 23u8, 23u8, 23u8, 24u8, 24u8,
    24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8,
    24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 24u8, 25u8, 25u8,
    25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8,
    25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 25u8, 26u8, 26u8,
    26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8,
    26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 26u8, 27u8, 27u8,
    27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8,
    27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 27u8, 28u8,
]);

static mut base_length: __SlateAlignLargeArray<[i32; 29]> = __SlateAlignLargeArray([
    0i32, 1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32, 10i32, 12i32, 14i32, 16i32, 20i32, 24i32,
    28i32, 32i32, 40i32, 48i32, 56i32, 64i32, 80i32, 96i32, 112i32, 128i32, 160i32, 192i32, 224i32,
    0i32,
]);

static mut base_dist: __SlateAlignLargeArray<[i32; 30]> = __SlateAlignLargeArray([
    0i32, 1i32, 2i32, 3i32, 4i32, 6i32, 8i32, 12i32, 16i32, 24i32, 32i32, 48i32, 64i32, 96i32,
    128i32, 192i32, 256i32, 384i32, 512i32, 768i32, 1024i32, 1536i32, 2048i32, 3072i32, 4096i32,
    6144i32, 8192i32, 12288i32, 16384i32, 24576i32,
]);

// The lengths of the bit length codes are sent in order of decreasing
// probability, to avoid transmitting the lengths for unused bit length codes.
// Local data. These are initialized only once.
// see definition of array dist_code below
#[repr(C)]
#[derive(Clone, Copy)]
struct static_tree_desc_s {
    /// static tree or NULL
    static_tree: *const ct_data_s,
    /// extra bits for each code or NULL
    extra_bits: *const i32,
    /// base index for extra_bits
    extra_base: i32,
    /// max number of elements in the tree
    elems: i32,
    /// max bit length for the codes
    max_length: i32,
}

#[cfg(target_pointer_width = "64")]
static mut static_l_desc: static_tree_desc_s = {
    let mut __t0: static_tree_desc_s = unsafe { std::mem::zeroed() };
    __t0.static_tree = unsafe { std::ptr::addr_of!(static_ltree.0) as *const ct_data_s };
    __t0.extra_bits = unsafe { std::ptr::addr_of!(extra_lbits.0) as *const i32 };
    __t0.extra_base = 257i32;
    __t0.elems = 286i32;
    __t0.max_length = 15i32;
    __t0
};

#[cfg(target_pointer_width = "32")]
static mut static_l_desc: static_tree_desc_s = static_tree_desc_s {
    static_tree: unsafe { std::ptr::addr_of!(static_ltree.0) as *const ct_data_s },
    extra_bits: unsafe { std::ptr::addr_of!(extra_lbits.0) as *const i32 },
    extra_base: 257i32,
    elems: 286i32,
    max_length: 15i32,
};

#[cfg(target_pointer_width = "64")]
static mut static_d_desc: static_tree_desc_s = {
    let mut __t0: static_tree_desc_s = unsafe { std::mem::zeroed() };
    __t0.static_tree = unsafe { std::ptr::addr_of!(static_dtree.0) as *const ct_data_s };
    __t0.extra_bits = unsafe { std::ptr::addr_of!(extra_dbits.0) as *const i32 };
    __t0.extra_base = 0i32;
    __t0.elems = 30i32;
    __t0.max_length = 15i32;
    __t0
};

#[cfg(target_pointer_width = "32")]
static mut static_d_desc: static_tree_desc_s = static_tree_desc_s {
    static_tree: unsafe { std::ptr::addr_of!(static_dtree.0) as *const ct_data_s },
    extra_bits: unsafe { std::ptr::addr_of!(extra_dbits.0) as *const i32 },
    extra_base: 0i32,
    elems: 30i32,
    max_length: 15i32,
};

#[cfg(target_pointer_width = "64")]
static mut static_bl_desc: static_tree_desc_s = {
    let mut __t0: static_tree_desc_s = unsafe { std::mem::zeroed() };
    __t0.static_tree = std::ptr::null::<ct_data_s>();
    __t0.extra_bits = unsafe { std::ptr::addr_of!(extra_blbits.0) as *const i32 };
    __t0.extra_base = 0i32;
    __t0.elems = 19i32;
    __t0.max_length = 7i32;
    __t0
};

#[cfg(target_pointer_width = "32")]
static mut static_bl_desc: static_tree_desc_s = static_tree_desc_s {
    static_tree: std::ptr::null::<ct_data_s>(),
    extra_bits: unsafe { std::ptr::addr_of!(extra_blbits.0) as *const i32 },
    extra_base: 0i32,
    elems: 19i32,
    max_length: 7i32,
};

#[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
use slate_support::memcpy;

// Output a short LSB first on the stream.
// IN assertion: there is enough room in pendingBuf.
/// Reverse the first len bits of a code, using straightforward code (a faster
/// method would use a table)
/// IN assertion: 1 <= len <= 15
fn bi_reverse(mut code: u32, mut len: i32) -> u32 {
    let mut res: u32 = 0;
    loop {
        res |= code & 1;
        code >>= 1i32;
        res <<= 1i32;
        let __v8: i32 = len - 1;
        len = __v8;
        if __v8 <= 0 {
            break;
        }
    }
    res >> 1i32
}

/// Flush the bit buffer, keeping at most 7 bits in it.
fn bi_flush(s: *mut internal_state) {
    if (unsafe { (*s).bi_valid }) == 16 {
        put_short!(s, unsafe { (*s).bi_buf });
        unsafe {
            (*s).bi_buf = 0;
        }
        unsafe {
            (*s).bi_valid = 0;
        }
    } else {
        if (unsafe { (*s).bi_valid }) >= 8 {
            put_byte!(s, (unsafe { (*s).bi_buf }) as u8);
            unsafe {
                (*s).bi_buf = (((((*s).bi_buf as u32) as i32) >> 8i32) as i16) as u16;
            }
            unsafe {
                (*s).bi_valid = (*s).bi_valid - 8;
            }
        }
    }
}

/// Flush the bit buffer and align the output on a byte boundary
fn bi_windup(s: *mut internal_state) {
    if (unsafe { (*s).bi_valid }) > 8 {
        put_short!(s, unsafe { (*s).bi_buf });
    } else {
        if (unsafe { (*s).bi_valid }) > 0 {
            put_byte!(s, (unsafe { (*s).bi_buf }) as u8);
        }
    }
    unsafe {
        (*s).bi_used = ((*s).bi_valid - 1 & 7) + 1;
    }
    unsafe {
        (*s).bi_buf = 0;
    }
    unsafe {
        (*s).bi_valid = 0;
    }
    #[cfg(feature = "zlib-debug")]
    {
        unsafe {
            (*s).bits_sent = unsafe { (*s).bits_sent }.wrapping_add(7) & !7;
        }
    }
}

/// Generate the codes for a given tree and bit counts (which need not be
/// optimal).
/// IN assertion: the array bl_count contains the bit length statistics for
/// the given tree and the field len is set for all tree elements.
/// OUT assertion: the field code is set for all tree elements of non
///     zero code length.
fn gen_codes(tree: *mut ct_data_s, max_code: i32, bl_count: *mut u16) {
    let mut next_code: __SlateAlignLargeArray<[u16; 16]> = __SlateAlignLargeArray([0u16; 16]); // next code value for each bit length
    let mut code: u32 = 0; // running code value
    // The distribution counts are first used to generate the code values
    // without bit reversal.
    let mut bits: i32 = 1; // bit index
    let mut n: i32 = 0; // code index
    while bits <= 15 {
        code = code.wrapping_add(
            (((unsafe { *bl_count.offset((bits - 1) as isize) }) as u32) as i32) as u32,
        ) << 1i32;
        next_code.0[bits as usize] = code as u16;
        bits += 1;
    }
    #[cfg(feature = "zlib-debug")]
    {
        Assert!(
            code.wrapping_add((((unsafe { *bl_count.add(15) }) as u32) as i32) as u32)
                .wrapping_sub(1)
                == 32767u32,
            c"inconsistent bit counts".as_ptr() as *mut c_char
        );
        Tracev__2!(
            slate_support::stderr(),
            c"\ngen_codes: max_code %d ".as_ptr() as *mut c_char,
            max_code
        );
    }
    while n <= max_code {
        let len: i32 = ((unsafe { (*tree.offset(n as isize)).dl.len }) as u32) as i32;
        if len != 0 {
            // Now reverse the bits
            let __v4: *mut u16 =
                unsafe { (next_code.0.as_mut_ptr() as *mut u16).offset(len as isize) };
            let __v5: u16 = unsafe { *__v4 };
            unsafe {
                *__v4 = ((((__v5 as u32) as i32) + 1) as i16) as u16;
            }
            unsafe {
                (*tree.offset(n as isize)).fc.code = bi_reverse(__v5 as u32, len) as u16;
            }
            #[cfg(all(target_env = "gnu", feature = "zlib-debug"))]
            {
                Tracecv__2!(
                    tree != ((unsafe { std::ptr::addr_of!(static_ltree.0) as *const ct_data_s })
                        as *mut ct_data_s),
                    slate_support::stderr(),
                    c"\nn %3d %c l %2d c %4x (%x) ".as_ptr() as *mut c_char,
                    n,
                    if (unsafe { slate_support::isgraph(n as i32) }) != 0 {
                        n
                    } else {
                        32i32
                    },
                    len,
                    unsafe { (*tree.offset(n as isize)).fc.code },
                    ((next_code.0[len as usize] as u32) as i32) - 1
                );
            }
            #[cfg(all(not(target_env = "gnu"), feature = "zlib-debug"))]
            {
                Tracecv__2!(
                    tree != ((unsafe { std::ptr::addr_of!(static_ltree.0) as *const ct_data_s })
                        as *mut ct_data_s),
                    slate_support::stderr(),
                    c"\nn %3d %c l %2d c %4x (%x) ".as_ptr() as *mut c_char,
                    n,
                    if (unsafe { slate_support::isgraph(n) }) != 0 {
                        n
                    } else {
                        32i32
                    },
                    len,
                    unsafe { (*tree.offset(n as isize)).fc.code },
                    ((next_code.0[len as usize] as u32) as i32) - 1
                );
            }
        }
        n += 1;
    }
}

#[cfg(feature = "zlib-debug")]
/// Send a value on a given number of bits.
/// IN assertion: length <= 16 and value fits in length bits.
fn send_bits(s: *mut internal_state, value: i32, length: i32) {
    Tracevv__4!(
        slate_support::stderr(),
        c" l %2d v %4x ".as_ptr() as *mut c_char,
        length,
        value
    );
    Assert!(
        length > 0 && length <= 15,
        c"invalid length".as_ptr() as *mut c_char
    );
    let __v1: c_ulong = unsafe { (*s).bits_sent };
    unsafe {
        (*s).bits_sent = __v1.wrapping_add(length as c_ulong);
    }
    // If not enough room in bi_buf, use (valid) bits from bi_buf and
    // (16 - bi_valid) bits from value, leaving (width - (16 - bi_valid))
    // unused bits in value.
    if (unsafe { (*s).bi_valid }) > 16 - length {
        unsafe {
            (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                | ((((value as i16) as u16) as u32) as i32) << (*s).bi_valid)
                as i16) as u16;
        }
        put_short!(s, unsafe { (*s).bi_buf });
        unsafe {
            (*s).bi_buf =
                ((((((value as i16) as u16) as u32) as i32) >> 16 - (*s).bi_valid) as i16) as u16;
        }
        unsafe {
            (*s).bi_valid = (*s).bi_valid + (length - 16);
        }
    } else {
        unsafe {
            (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                | ((((value as i16) as u16) as u32) as i32) << (*s).bi_valid)
                as i16) as u16;
        }
        unsafe {
            (*s).bi_valid = (*s).bi_valid + length;
        }
    }
}

// the arguments must not have side effects
#[cfg(not(feature = "zlib-debug"))]
/// Send a value on a given number of bits.
/// IN assertion: length <= 16 and value fits in length bits.
/// Initialize the various 'constant' tables.
fn tr_static_init() {}

#[cfg(feature = "zlib-debug")]
/// Initialize the various 'constant' tables.
fn tr_static_init() {}

// Generate the file trees.h describing the static trees.
/// Initialize a new block.
fn init_block(s: *mut internal_state) {
    // Initialize the trees.
    let mut n: i32 = 0; // iterates over tree elements
    while n < 256 + 1 + 29 {
        unsafe {
            (*((*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s).offset(n as isize))
                .fc
                .freq = 0;
        }
        n += 1;
    }
    n = 0;
    while n < 30 {
        unsafe {
            (*((*s).dyn_dtree.as_mut_ptr() as *mut ct_data_s).offset(n as isize))
                .fc
                .freq = 0;
        }
        n += 1;
    }
    n = 0;
    while n < 19 {
        unsafe {
            (*((*s).bl_tree.as_mut_ptr() as *mut ct_data_s).offset(n as isize))
                .fc
                .freq = 0;
        }
        n += 1;
    }
    unsafe {
        (*((*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s).add(256))
            .fc
            .freq = 1;
    }
    unsafe {
        (*s).static_len = 0;
    }
    unsafe {
        (*s).opt_len = 0;
    }
    unsafe {
        (*s).matches = 0;
    }
    unsafe {
        (*s).sym_next = 0;
    }
}

/// Initialize the tree data structures for a new zlib stream.
#[unsafe(no_mangle)]
extern "C-unwind" fn _tr_init(s: *mut internal_state) {
    tr_static_init();
    unsafe {
        (*s).l_desc.dyn_tree = (*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s;
    }
    unsafe {
        (*s).l_desc.stat_desc = std::ptr::addr_of!(static_l_desc);
    }
    unsafe {
        (*s).d_desc.dyn_tree = (*s).dyn_dtree.as_mut_ptr() as *mut ct_data_s;
    }
    unsafe {
        (*s).d_desc.stat_desc = std::ptr::addr_of!(static_d_desc);
    }
    unsafe {
        (*s).bl_desc.dyn_tree = (*s).bl_tree.as_mut_ptr() as *mut ct_data_s;
    }
    unsafe {
        (*s).bl_desc.stat_desc = std::ptr::addr_of!(static_bl_desc);
    }
    unsafe {
        (*s).bi_buf = 0;
    }
    unsafe {
        (*s).bi_valid = 0;
    }
    unsafe {
        (*s).bi_used = 0;
    }
    #[cfg(feature = "zlib-debug")]
    {
        unsafe {
            (*s).compressed_len = 0;
        }
        unsafe {
            (*s).bits_sent = 0;
        }
    }
    // Initialize the first block of the first file:
    init_block(s);
}

// Index within the heap array of least frequent node in the Huffman tree
// Remove the smallest element from the heap and recreate the heap with
// one less element. Updates heap and heap_len.
// Compares to subtrees, using the tree depth as tie breaker when
// the subtrees have equal frequency. This minimizes the worst case length.
/// Restore the heap property by moving down the tree starting at node k,
/// exchanging a node with the smallest of its two sons if necessary, stopping
/// when the heap property is re-established (each father smaller than its
/// two sons).
fn pqdownheap(s: *mut internal_state, mut tree: *mut ct_data_s, mut k: i32) {
    let v: i32 = unsafe { *((*s).heap.as_mut_ptr() as *mut i32).offset(k as isize) };
    let mut j: i32 = k << 1i32; // left son of k
    while j <= unsafe { (*s).heap_len } {
        // Set j to the smallest of the two sons:
        if j < unsafe { (*s).heap_len }
            && smaller!(
                tree,
                unsafe { *((*s).heap.as_mut_ptr() as *mut i32).offset((j + 1) as isize) },
                unsafe { *((*s).heap.as_mut_ptr() as *mut i32).offset(j as isize) },
                (*s).depth
            )
        {
            j += 1;
        }
        // Exit if v is smaller than both sons
        if smaller!(
            tree,
            v,
            unsafe { *((*s).heap.as_mut_ptr() as *mut i32).offset(j as isize) },
            (*s).depth
        ) {
            break;
        }
        // Exchange v with the smallest son
        unsafe {
            *((*s).heap.as_mut_ptr() as *mut i32).offset(k as isize) =
                *((*s).heap.as_mut_ptr() as *mut i32).offset(j as isize);
        }
        k = j;
        // And continue down the tree, setting j to the left son of k
        j <<= 1i32;
    }
    unsafe {
        *((*s).heap.as_mut_ptr() as *mut i32).offset(k as isize) = v;
    }
}

/// Compute the optimal bit lengths for a tree and update the total bit length
/// for the current block.
/// IN assertion: the fields freq and dad are set, heap[heap_max] and
///    above are the tree nodes sorted by increasing frequency.
/// OUT assertions: the field len is set to the optimal bit length, the
///     array bl_count contains the frequencies for each bit length.
///     The length opt_len is updated; static_len is also updated if stree is
///     not null.
fn gen_bitlen(s: *mut internal_state, desc: *mut tree_desc_s) {
    let tree: *mut ct_data_s = unsafe { (*desc).dyn_tree };
    let max_code: i32 = unsafe { (*desc).max_code };
    let stree: *const ct_data_s = unsafe { (*(*desc).stat_desc).static_tree };
    let extra: *const i32 = unsafe { (*(*desc).stat_desc).extra_bits };
    let base: i32 = unsafe { (*(*desc).stat_desc).extra_base };
    let max_length: i32 = unsafe { (*(*desc).stat_desc).max_length };
    let mut h: i32 = 0; // heap index
    let mut n: i32 = 0;
    let mut m: i32 = 0; // iterate over the tree elements
    let mut bits: i32 = 0; // bit length
    let mut xbits: i32 = 0; // extra bits
    let mut f: u16 = 0; // frequency
    let mut overflow: i32 = 0; // number of elements with bit length too large
    while bits <= 15 {
        unsafe {
            *((*s).bl_count.as_mut_ptr() as *mut u16).offset(bits as isize) = 0;
        }
        bits += 1;
    }
    // In a first pass, compute the optimal bit lengths (which may
    // overflow in the case of the bit length tree).
    unsafe {
        (*tree.offset(
            *((*s).heap.as_mut_ptr() as *mut i32).offset((*s).heap_max as isize) as isize,
        ))
        .dl
        .len = 0;
    }
    // root of the heap
    h = (unsafe { (*s).heap_max }) + 1;
    while h < 2 * (256 + 1 + 29) + 1 {
        n = unsafe { *((*s).heap.as_mut_ptr() as *mut i32).offset(h as isize) };
        bits = (((unsafe {
            (*tree.offset((((*tree.offset(n as isize)).dl.dad as u32) as i32) as isize))
                .dl
                .len
        }) as u32) as i32)
            + 1;
        if bits > max_length {
            bits = max_length;
            overflow += 1;
        }
        unsafe {
            (*tree.offset(n as isize)).dl.len = (bits as i16) as u16;
        }
        // We overwrite tree[n].Dad which is no longer needed
        if n <= max_code {
            // not a leaf node
            let __v6: *mut u16 =
                unsafe { ((*s).bl_count.as_mut_ptr() as *mut u16).offset(bits as isize) };
            unsafe {
                *__v6 = ((((*__v6 as u32) as i32) + 1) as i16) as u16;
            }
            xbits = 0;
            if n >= base {
                xbits = unsafe { *extra.offset((n - base) as isize) };
            }
            f = unsafe { (*tree.offset(n as isize)).fc.freq };
            let __v10: c_ulong = unsafe { (*s).opt_len };
            unsafe {
                (*s).opt_len = __v10
                    .wrapping_add((f as c_ulong).wrapping_mul(((bits + xbits) as u32) as c_ulong));
            }
            if !stree.is_null() {
                let __v13: c_ulong = unsafe { (*s).static_len };
                unsafe {
                    (*s).static_len = __v13.wrapping_add((f as c_ulong).wrapping_mul(
                        (((((*stree.offset(n as isize)).dl.len as u32) as i32) + xbits) as u32)
                            as c_ulong,
                    ));
                }
            }
        }
        h += 1;
    }
    if overflow == 0 {
        return;
    }
    #[cfg(feature = "zlib-debug")]
    {
        Tracev!(
            slate_support::stderr(),
            c"\nbit length overflow\n".as_ptr() as *mut c_char
        );
    }
    // This happens for example on obj2 and pic of the Calgary corpus
    // Find the first bit length which could increase:
    loop {
        bits = max_length - 1;
        while (((unsafe { *((*s).bl_count.as_mut_ptr() as *mut u16).offset(bits as isize) }) as u32)
            as i32)
            == 0
        {
            bits -= 1;
        }
        let __v20: *mut u16 =
            unsafe { ((*s).bl_count.as_mut_ptr() as *mut u16).offset(bits as isize) };
        unsafe {
            *__v20 = ((((*__v20 as u32) as i32) - 1) as i16) as u16;
        }
        // move one leaf down the tree
        let __v23: *mut u16 =
            unsafe { ((*s).bl_count.as_mut_ptr() as *mut u16).offset((bits + 1) as isize) };
        unsafe {
            *__v23 = ((((*__v23 as u32) as i32) + 2) as i16) as u16;
        }
        // move one overflow item as its brother
        let __v26: *mut u16 =
            unsafe { ((*s).bl_count.as_mut_ptr() as *mut u16).offset(max_length as isize) };
        unsafe {
            *__v26 = ((((*__v26 as u32) as i32) - 1) as i16) as u16;
        }
        // The brother of the overflow item also moves one step up,
        // but this does not affect bl_count[max_length]
        overflow -= 2;
        if overflow <= 0 {
            break;
        }
    }
    // Now recompute all bit lengths, scanning in increasing frequency.
    // h is still equal to HEAP_SIZE. (It is simpler to reconstruct all
    // lengths instead of fixing only the wrong ones. This idea is taken
    // from 'ar' written by Haruhiko Okumura.)
    bits = max_length;
    while bits != 0 {
        n = ((unsafe { *((*s).bl_count.as_mut_ptr() as *mut u16).offset(bits as isize) }) as u32)
            as i32;
        while n != 0 {
            let __v33: i32 = h - 1;
            h = __v33;
            m = unsafe { *((*s).heap.as_mut_ptr() as *mut i32).offset(__v33 as isize) };
            if m <= max_code {
                if ((unsafe { (*tree.offset(m as isize)).dl.len }) as u32) != (bits as u32) {
                    #[cfg(feature = "zlib-debug")]
                    {
                        Tracev__6!(
                            slate_support::stderr(),
                            c"code %d bits %d->%d\n".as_ptr() as *mut c_char,
                            m,
                            unsafe { (*tree.offset(m as isize)).dl.len },
                            bits
                        );
                    }
                    let __v36: c_ulong = unsafe { (*s).opt_len };
                    unsafe {
                        (*s).opt_len = __v36.wrapping_add(
                            (bits as c_ulong)
                                .wrapping_sub(
                                    (((*tree.offset(m as isize)).dl.len as u32) as i32) as c_ulong,
                                )
                                .wrapping_mul(
                                    (((*tree.offset(m as isize)).fc.freq as u32) as i32) as c_ulong,
                                ),
                        );
                    }
                    unsafe {
                        (*tree.offset(m as isize)).dl.len = (bits as i16) as u16;
                    }
                }
                n -= 1;
            }
        }
        bits -= 1;
    }
}

/// Construct one Huffman tree and assigns the code bit strings and lengths.
/// Update the total bit length for the current block.
/// IN assertion: the field freq is set for all tree elements.
/// OUT assertions: the fields len and code are set to the optimal bit length
///     and corresponding code. The length opt_len is updated; static_len is
///     also updated if stree is not null. The field max_code is set.
fn build_tree(s: *mut internal_state, desc: *mut tree_desc_s) {
    let tree: *mut ct_data_s = unsafe { (*desc).dyn_tree };
    let stree: *const ct_data_s = unsafe { (*(*desc).stat_desc).static_tree };
    let elems: i32 = unsafe { (*(*desc).stat_desc).elems };
    let mut n: i32 = 0;
    let mut m: i32 = 0; // iterate over heap elements
    let mut max_code: i32 = -1; // largest code with non zero frequency
    let mut node: i32 = 0; // new node being created
    // Construct the initial heap, with least frequent element in
    // heap[SMALLEST]. The sons of heap[n] are heap[2*n] and heap[2*n + 1].
    // heap[0] is not used.
    unsafe {
        (*s).heap_len = 0;
    }
    unsafe {
        (*s).heap_max = 2 * (256 + 1 + 29) + 1;
    }
    while n < elems {
        if (((unsafe { (*tree.offset(n as isize)).fc.freq }) as u32) as i32) != 0 {
            let __v1: i32 = n;
            max_code = __v1;
            let __v4: i32 = (unsafe { (*s).heap_len }) + 1;
            unsafe {
                (*s).heap_len = __v4;
            }
            unsafe {
                *((*s).heap.as_mut_ptr() as *mut i32).offset(__v4 as isize) = __v1;
            }
            unsafe {
                *((*s).depth.as_mut_ptr() as *mut u8).offset(n as isize) = 0;
            }
        } else {
            unsafe {
                (*tree.offset(n as isize)).dl.len = 0;
            }
        }
        n += 1;
    }
    // The pkzip format requires that at least one distance code exists,
    // and that at least one bit should be sent even if there is only one
    // possible code. So to avoid special checks later on we force at least
    // two codes of non zero frequency.
    while (unsafe { (*s).heap_len }) < 2 {
        let __v8: i32 = if max_code < 2 {
            let __v10: i32 = max_code + 1;
            max_code = __v10;
            __v10
        } else {
            0
        };
        let __v13: i32 = (unsafe { (*s).heap_len }) + 1;
        unsafe {
            (*s).heap_len = __v13;
        }
        let __v14: i32 = __v8;
        unsafe {
            *((*s).heap.as_mut_ptr() as *mut i32).offset(__v13 as isize) = __v14;
        }
        node = __v14;
        unsafe {
            (*tree.offset(node as isize)).fc.freq = 1;
        }
        unsafe {
            *((*s).depth.as_mut_ptr() as *mut u8).offset(node as isize) = 0;
        }
        let __v16: c_ulong = unsafe { (*s).opt_len };
        unsafe {
            (*s).opt_len = __v16.wrapping_sub(1);
        }
        if !stree.is_null() {
            let __v19: c_ulong = unsafe { (*s).static_len };
            unsafe {
                (*s).static_len = __v19.wrapping_sub(
                    (((*stree.offset(node as isize)).dl.len as u32) as i32) as c_ulong,
                );
            }
        }
        // node is 0 or 1 so it does not have extra bits
    }
    unsafe {
        (*desc).max_code = max_code;
    }
    // The elements heap[heap_len/2 + 1 .. heap_len] are leaves of the tree,
    // establish sub-heaps of increasing lengths:
    n = (unsafe { (*s).heap_len }) / 2;
    while n >= 1 {
        pqdownheap(s, tree, n);
        n -= 1;
    }
    // Construct the Huffman tree by repeatedly combining the least two
    // frequent nodes.
    node = elems; // next internal node of the tree
    loop {
        pqremove!(s, tree, n);
        // n = node of least frequency
        m = unsafe { *((*s).heap.as_mut_ptr() as *mut i32).add(1) }; // m = node of next least frequency
        let __v30: i32 = (unsafe { (*s).heap_max }) - 1;
        unsafe {
            (*s).heap_max = __v30;
        }
        unsafe {
            *((*s).heap.as_mut_ptr() as *mut i32).offset(__v30 as isize) = n;
        }
        // keep the nodes sorted by frequency
        let __v33: i32 = (unsafe { (*s).heap_max }) - 1;
        unsafe {
            (*s).heap_max = __v33;
        }
        unsafe {
            *((*s).heap.as_mut_ptr() as *mut i32).offset(__v33 as isize) = m;
        }
        // Create a new node father of n and m
        unsafe {
            (*tree.offset(node as isize)).fc.freq = (((((*tree.offset(n as isize)).fc.freq as u32)
                as i32)
                + (((*tree.offset(m as isize)).fc.freq as u32) as i32))
                as i16) as u16;
        }
        unsafe {
            *((*s).depth.as_mut_ptr() as *mut u8).offset(node as isize) =
                (((if ((*((*s).depth.as_mut_ptr() as *mut u8).offset(n as isize) as u32) as i32)
                    >= ((*((*s).depth.as_mut_ptr() as *mut u8).offset(m as isize) as u32) as i32)
                {
                    (*((*s).depth.as_mut_ptr() as *mut u8).offset(n as isize) as u32) as i32
                } else {
                    (*((*s).depth.as_mut_ptr() as *mut u8).offset(m as isize) as u32) as i32
                }) + 1) as i8) as u8;
        }
        let __v34: u16 = (node as i16) as u16;
        unsafe {
            (*tree.offset(m as isize)).dl.dad = __v34;
        }
        unsafe {
            (*tree.offset(n as isize)).dl.dad = __v34;
        }
        // and insert the new node in the heap
        let __v35: i32 = node;
        node = __v35 + 1;
        unsafe {
            *((*s).heap.as_mut_ptr() as *mut i32).add(1) = __v35;
        }
        pqdownheap(s, tree, 1i32);
        if (unsafe { (*s).heap_len }) < 2 {
            break;
        }
    }
    let __v40: i32 = (unsafe { (*s).heap_max }) - 1;
    unsafe {
        (*s).heap_max = __v40;
    }
    unsafe {
        *((*s).heap.as_mut_ptr() as *mut i32).offset(__v40 as isize) =
            *((*s).heap.as_mut_ptr() as *mut i32).add(1);
    }
    // At this point, the fields freq and dad are set. We can now
    // generate the bit lengths.
    gen_bitlen(s, desc);
    // The field len is now set, we can generate the bit codes
    gen_codes(tree, max_code, unsafe {
        (*s).bl_count.as_mut_ptr() as *mut u16
    });
}

/// Scan a literal or distance tree to determine the frequencies of the codes
/// in the bit length tree.
fn scan_tree(s: *mut internal_state, tree: *mut ct_data_s, max_code: i32) {
    let mut n: i32 = 0; // iterates over all tree elements
    let mut prevlen: i32 = -1; // last emitted length
    let mut curlen: i32 = 0; // length of current code
    let mut nextlen: i32 = ((unsafe { (*tree.add(0)).dl.len }) as u32) as i32; // length of next code
    let mut count: i32 = 0; // repeat count of the current code
    let mut max_count: i32 = 7; // max repeat count
    let mut min_count: i32 = 4; // min repeat count
    if nextlen == 0 {
        max_count = 138;
        min_count = 3;
    }
    unsafe {
        (*tree.offset((max_code + 1) as isize)).dl.len = 65535u16;
    }
    // guard
    while n <= max_code {
        '__slate_continue_2: {
            curlen = nextlen;
            nextlen = ((unsafe { (*tree.offset((n + 1) as isize)).dl.len }) as u32) as i32;
            let __v1: i32 = count + 1;
            count = __v1;
            if __v1 < max_count && curlen == nextlen {
                break '__slate_continue_2;
            } else {
                if count < min_count {
                    let __v3: *mut ct_data_s = unsafe {
                        ((*s).bl_tree.as_mut_ptr() as *mut ct_data_s).offset(curlen as isize)
                    };
                    unsafe {
                        (*__v3).fc.freq = (((((*__v3).fc.freq as u32) as i32)
                            + ((((count as i16) as u16) as u32) as i32))
                            as i16) as u16;
                    }
                } else {
                    if curlen != 0 {
                        if curlen != prevlen {
                            let __v6: *mut ct_data_s = unsafe {
                                ((*s).bl_tree.as_mut_ptr() as *mut ct_data_s)
                                    .offset(curlen as isize)
                            };
                            unsafe {
                                (*__v6).fc.freq =
                                    (((((*__v6).fc.freq as u32) as i32) + 1) as i16) as u16;
                            }
                        }
                        let __v9: *mut ct_data_s =
                            unsafe { ((*s).bl_tree.as_mut_ptr() as *mut ct_data_s).add(16) };
                        unsafe {
                            (*__v9).fc.freq =
                                (((((*__v9).fc.freq as u32) as i32) + 1) as i16) as u16;
                        }
                    } else {
                        if count <= 10 {
                            let __v12: *mut ct_data_s =
                                unsafe { ((*s).bl_tree.as_mut_ptr() as *mut ct_data_s).add(17) };
                            unsafe {
                                (*__v12).fc.freq =
                                    (((((*__v12).fc.freq as u32) as i32) + 1) as i16) as u16;
                            }
                        } else {
                            let __v15: *mut ct_data_s =
                                unsafe { ((*s).bl_tree.as_mut_ptr() as *mut ct_data_s).add(18) };
                            unsafe {
                                (*__v15).fc.freq =
                                    (((((*__v15).fc.freq as u32) as i32) + 1) as i16) as u16;
                            }
                        }
                    }
                }
            }
            count = 0;
            prevlen = curlen;
            if nextlen == 0 {
                max_count = 138;
                min_count = 3;
            } else {
                if curlen == nextlen {
                    max_count = 6;
                    min_count = 3;
                } else {
                    max_count = 7;
                    min_count = 4;
                }
            }
        }
        n += 1;
    }
}

/// Send a literal or distance tree in compressed form, using the codes in
/// bl_tree.
fn send_tree(s: *mut internal_state, tree: *mut ct_data_s, max_code: i32) {
    let mut n: i32 = 0; // iterates over all tree elements
    let mut prevlen: i32 = -1; // last emitted length
    let mut curlen: i32 = 0; // length of current code
    let mut nextlen: i32 = ((unsafe { (*tree.add(0)).dl.len }) as u32) as i32; // length of next code
    let mut count: i32 = 0; // repeat count of the current code
    let mut max_count: i32 = 7; // max repeat count
    let mut min_count: i32 = 4; // min repeat count
    // tree[max_code + 1].Len = -1;
    //
    // guard already set
    if nextlen == 0 {
        max_count = 138;
        min_count = 3;
    }
    while n <= max_code {
        '__slate_continue_2: {
            curlen = nextlen;
            nextlen = ((unsafe { (*tree.offset((n + 1) as isize)).dl.len }) as u32) as i32;
            let __v1: i32 = count + 1;
            count = __v1;
            if __v1 < max_count && curlen == nextlen {
                break '__slate_continue_2;
            } else {
                if count < min_count {
                    loop {
                        #[cfg(not(feature = "zlib-debug"))]
                        {
                            send_code__2!(s, curlen, (*s).bl_tree);
                        }
                        #[cfg(feature = "zlib-debug")]
                        {
                            send_code!(s, curlen, (*s).bl_tree);
                        }
                        let __v23: i32 = count - 1;
                        count = __v23;
                        if __v23 == 0 {
                            break;
                        }
                    }
                } else {
                    if curlen != 0 {
                        if curlen != prevlen {
                            #[cfg(not(feature = "zlib-debug"))]
                            {
                                send_code__2!(s, curlen, (*s).bl_tree);
                            }
                            #[cfg(feature = "zlib-debug")]
                            {
                                send_code!(s, curlen, (*s).bl_tree);
                            }
                            count -= 1;
                        }
                        #[cfg(not(feature = "zlib-debug"))]
                        {
                            send_code__2!(s, 16i32, (*s).bl_tree);
                        }
                        #[cfg(not(feature = "zlib-debug"))]
                        let len: i32 = 2;
                        #[cfg(not(feature = "zlib-debug"))]
                        {
                            if (unsafe { (*s).bi_valid }) > 16 - len {
                                let val: i32 = count - 3;
                                unsafe {
                                    (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                                        | ((((val as i16) as u16) as u32) as i32) << (*s).bi_valid)
                                        as i16)
                                        as u16;
                                }
                                put_short!(s, unsafe { (*s).bi_buf });
                                unsafe {
                                    (*s).bi_buf = ((((((val as i16) as u16) as u32) as i32)
                                        >> 16 - (*s).bi_valid)
                                        as i16)
                                        as u16;
                                }
                                unsafe {
                                    (*s).bi_valid = (*s).bi_valid + (len - 16);
                                }
                            } else {
                                unsafe {
                                    (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                                        | (((((count - 3) as i16) as u16) as u32) as i32)
                                            << (*s).bi_valid)
                                        as i16)
                                        as u16;
                                }
                                unsafe {
                                    (*s).bi_valid = (*s).bi_valid + len;
                                }
                            }
                        }
                        #[cfg(feature = "zlib-debug")]
                        {
                            Assert!(count >= 3 && count <= 6, c" 3_6?".as_ptr() as *mut c_char);
                            send_code!(s, 16i32, (*s).bl_tree);
                            send_bits(s, count - 3, 2i32);
                        }
                    } else {
                        if count <= 10 {
                            #[cfg(not(feature = "zlib-debug"))]
                            {
                                send_code__2!(s, 17i32, (*s).bl_tree);
                            }
                            #[cfg(not(feature = "zlib-debug"))]
                            let len: i32 = 3;
                            #[cfg(not(feature = "zlib-debug"))]
                            {
                                if (unsafe { (*s).bi_valid }) > 16 - len {
                                    let val: i32 = count - 3;
                                    unsafe {
                                        (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                                            | ((((val as i16) as u16) as u32) as i32)
                                                << (*s).bi_valid)
                                            as i16)
                                            as u16;
                                    }
                                    put_short!(s, unsafe { (*s).bi_buf });
                                    unsafe {
                                        (*s).bi_buf = ((((((val as i16) as u16) as u32) as i32)
                                            >> 16 - (*s).bi_valid)
                                            as i16)
                                            as u16;
                                    }
                                    unsafe {
                                        (*s).bi_valid = (*s).bi_valid + (len - 16);
                                    }
                                } else {
                                    unsafe {
                                        (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                                            | (((((count - 3) as i16) as u16) as u32) as i32)
                                                << (*s).bi_valid)
                                            as i16)
                                            as u16;
                                    }
                                    unsafe {
                                        (*s).bi_valid = (*s).bi_valid + len;
                                    }
                                }
                            }
                            #[cfg(feature = "zlib-debug")]
                            {
                                send_code!(s, 17i32, (*s).bl_tree);
                                send_bits(s, count - 3, 3i32);
                            }
                        } else {
                            #[cfg(not(feature = "zlib-debug"))]
                            {
                                send_code__2!(s, 18i32, (*s).bl_tree);
                            }
                            #[cfg(not(feature = "zlib-debug"))]
                            let len: i32 = 7;
                            #[cfg(not(feature = "zlib-debug"))]
                            {
                                if (unsafe { (*s).bi_valid }) > 16 - len {
                                    let val: i32 = count - 11;
                                    unsafe {
                                        (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                                            | ((((val as i16) as u16) as u32) as i32)
                                                << (*s).bi_valid)
                                            as i16)
                                            as u16;
                                    }
                                    put_short!(s, unsafe { (*s).bi_buf });
                                    unsafe {
                                        (*s).bi_buf = ((((((val as i16) as u16) as u32) as i32)
                                            >> 16 - (*s).bi_valid)
                                            as i16)
                                            as u16;
                                    }
                                    unsafe {
                                        (*s).bi_valid = (*s).bi_valid + (len - 16);
                                    }
                                } else {
                                    unsafe {
                                        (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                                            | (((((count - 11) as i16) as u16) as u32) as i32)
                                                << (*s).bi_valid)
                                            as i16)
                                            as u16;
                                    }
                                    unsafe {
                                        (*s).bi_valid = (*s).bi_valid + len;
                                    }
                                }
                            }
                            #[cfg(feature = "zlib-debug")]
                            {
                                send_code!(s, 18i32, (*s).bl_tree);
                                send_bits(s, count - 11, 7i32);
                            }
                        }
                    }
                }
            }
            count = 0;
            prevlen = curlen;
            if nextlen == 0 {
                max_count = 138;
                min_count = 3;
            } else {
                if curlen == nextlen {
                    max_count = 6;
                    min_count = 3;
                } else {
                    max_count = 7;
                    min_count = 4;
                }
            }
        }
        n += 1;
    }
}

/// Construct the Huffman tree for the bit lengths and return the index in
/// bl_order of the last bit length code to send.
fn build_bl_tree(s: *mut internal_state) -> i32 {
    // Determine the number of bit length codes to send. The pkzip format
    // requires that at least 4 bit length codes be sent. (appnote.txt says
    // 3 but the actual value used is 4.)
    let mut max_blindex: i32 = 19 - 1; // index of last bit length code of non zero freq
    // Determine the bit length frequencies for literal and distance trees
    scan_tree(
        s,
        unsafe { (*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s },
        unsafe { (*s).l_desc.max_code },
    );
    scan_tree(
        s,
        unsafe { (*s).dyn_dtree.as_mut_ptr() as *mut ct_data_s },
        unsafe { (*s).d_desc.max_code },
    );
    // Build the bit length tree:
    build_tree(s, unsafe { std::ptr::addr_of_mut!((*s).bl_desc) });
    // opt_len now includes the length of the tree representations, except the
    // lengths of the bit lengths codes and the 5 + 5 + 4 bits for the counts.
    while max_blindex >= 3 {
        if (((unsafe {
            (*((*s).bl_tree.as_mut_ptr() as *mut ct_data_s)
                .offset(((bl_order.0[max_blindex as usize] as u32) as i32) as isize))
            .dl
            .len
        }) as u32) as i32)
            != 0
        {
            break;
        }
        max_blindex -= 1;
    }
    // Update opt_len to include the bit length tree and counts
    let __v4: c_ulong = unsafe { (*s).opt_len };
    unsafe {
        (*s).opt_len = __v4.wrapping_add(
            (3 as c_ulong)
                .wrapping_mul((max_blindex as c_ulong).wrapping_add(1))
                .wrapping_add(5)
                .wrapping_add(5)
                .wrapping_add(4),
        );
    }
    #[cfg(feature = "zlib-debug")]
    {
        Tracev__3!(
            slate_support::stderr(),
            c"\ndyn trees: dyn %lu, stat %lu".as_ptr() as *mut c_char,
            unsafe { (*s).opt_len },
            unsafe { (*s).static_len }
        );
    }
    max_blindex
}

/// Send the header for a block using dynamic Huffman trees: the counts, the
/// lengths of the bit length codes, the literal tree and the distance tree.
/// IN assertion: lcodes >= 257, dcodes >= 1, blcodes >= 4.
fn send_all_trees(s: *mut internal_state, lcodes: i32, dcodes: i32, blcodes: i32) {
    #[cfg(not(feature = "zlib-debug"))]
    {
        {
            let len: i32 = 5;
            if (unsafe { (*s).bi_valid }) > 16 - len {
                let val: i32 = lcodes - 257;
                unsafe {
                    (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                        | ((((val as i16) as u16) as u32) as i32) << (*s).bi_valid)
                        as i16) as u16;
                }
                put_short!(s, unsafe { (*s).bi_buf });
                unsafe {
                    (*s).bi_buf = ((((((val as i16) as u16) as u32) as i32) >> 16 - (*s).bi_valid)
                        as i16) as u16;
                }
                unsafe {
                    (*s).bi_valid = (*s).bi_valid + (len - 16);
                }
            } else {
                unsafe {
                    (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                        | (((((lcodes - 257) as i16) as u16) as u32) as i32) << (*s).bi_valid)
                        as i16) as u16;
                }
                unsafe {
                    (*s).bi_valid = (*s).bi_valid + len;
                }
            }
        }
        {
            let len: i32 = 5;
            if (unsafe { (*s).bi_valid }) > 16 - len {
                let val: i32 = dcodes - 1;
                unsafe {
                    (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                        | ((((val as i16) as u16) as u32) as i32) << (*s).bi_valid)
                        as i16) as u16;
                }
                put_short!(s, unsafe { (*s).bi_buf });
                unsafe {
                    (*s).bi_buf = ((((((val as i16) as u16) as u32) as i32) >> 16 - (*s).bi_valid)
                        as i16) as u16;
                }
                unsafe {
                    (*s).bi_valid = (*s).bi_valid + (len - 16);
                }
            } else {
                unsafe {
                    (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                        | (((((dcodes - 1) as i16) as u16) as u32) as i32) << (*s).bi_valid)
                        as i16) as u16;
                }
                unsafe {
                    (*s).bi_valid = (*s).bi_valid + len;
                }
            }
        }
    }
    #[cfg(not(feature = "zlib-debug"))]
    let len: i32 = 4;
    #[cfg(not(feature = "zlib-debug"))]
    {
        if (unsafe { (*s).bi_valid }) > 16 - len {
            let val: i32 = blcodes - 4;
            unsafe {
                (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                    | ((((val as i16) as u16) as u32) as i32) << (*s).bi_valid)
                    as i16) as u16;
            }
            put_short!(s, unsafe { (*s).bi_buf });
            unsafe {
                (*s).bi_buf =
                    ((((((val as i16) as u16) as u32) as i32) >> 16 - (*s).bi_valid) as i16) as u16;
            }
            unsafe {
                (*s).bi_valid = (*s).bi_valid + (len - 16);
            }
        } else {
            unsafe {
                (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                    | (((((blcodes - 4) as i16) as u16) as u32) as i32) << (*s).bi_valid)
                    as i16) as u16;
            }
            unsafe {
                (*s).bi_valid = (*s).bi_valid + len;
            }
        }
    }
    #[cfg(feature = "zlib-debug")]
    {
        Assert!(
            lcodes >= 257 && dcodes >= 1 && blcodes >= 4,
            c"not enough codes".as_ptr() as *mut c_char
        );
        Assert!(
            lcodes <= 256 + 1 + 29 && dcodes <= 30 && blcodes <= 19,
            c"too many codes".as_ptr() as *mut c_char
        );
        Tracev!(
            slate_support::stderr(),
            c"\nbl counts: ".as_ptr() as *mut c_char
        );
        send_bits(s, lcodes - 257, 5i32);
        send_bits(s, dcodes - 1, 5i32);
        send_bits(s, blcodes - 4, 4i32);
    }
    // index in bl_order
    for rank in 0..blcodes {
        #[cfg(not(feature = "zlib-debug"))]
        {
            send_bits!(
                s,
                unsafe {
                    (*((*s).bl_tree.as_mut_ptr() as *mut ct_data_s)
                        .offset(((bl_order.0[rank as usize] as u32) as i32) as isize))
                    .dl
                    .len
                },
                3i32
            );
        }
        #[cfg(feature = "zlib-debug")]
        {
            Tracev__5!(
                slate_support::stderr(),
                c"\nbl code %2d ".as_ptr() as *mut c_char,
                unsafe { bl_order.0[rank as usize] }
            );
            send_bits(
                s,
                ((unsafe {
                    (*((*s).bl_tree.as_mut_ptr() as *mut ct_data_s)
                        .offset(((bl_order.0[rank as usize] as u32) as i32) as isize))
                    .dl
                    .len
                }) as u32) as i32,
                3i32,
            );
        }
    }
    #[cfg(feature = "zlib-debug")]
    {
        Tracev__2!(
            slate_support::stderr(),
            c"\nbl tree: sent %lu".as_ptr() as *mut c_char,
            unsafe { (*s).bits_sent }
        );
    }
    send_tree(
        s,
        unsafe { (*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s },
        lcodes - 1,
    ); // literal tree
    #[cfg(feature = "zlib-debug")]
    {
        Tracev__2!(
            slate_support::stderr(),
            c"\nlit tree: sent %lu".as_ptr() as *mut c_char,
            unsafe { (*s).bits_sent }
        );
    }
    send_tree(
        s,
        unsafe { (*s).dyn_dtree.as_mut_ptr() as *mut ct_data_s },
        dcodes - 1,
    ); // distance tree
    #[cfg(feature = "zlib-debug")]
    {
        Tracev__2!(
            slate_support::stderr(),
            c"\ndist tree: sent %lu".as_ptr() as *mut c_char,
            unsafe { (*s).bits_sent }
        );
    }
}

/// Send a stored block
#[unsafe(no_mangle)]
extern "C-unwind" fn _tr_stored_block(
    s: *mut internal_state,
    buf: *mut c_char,
    stored_len: c_ulong,
    last: i32,
) {
    #[cfg(not(feature = "zlib-debug"))]
    let len: i32 = 3;
    #[cfg(not(feature = "zlib-debug"))]
    {
        if (unsafe { (*s).bi_valid }) > 16 - len {
            let val: i32 = 0i32 + last;
            unsafe {
                (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                    | ((((val as i16) as u16) as u32) as i32) << (*s).bi_valid)
                    as i16) as u16;
            }
            put_short!(s, unsafe { (*s).bi_buf });
            unsafe {
                (*s).bi_buf =
                    ((((((val as i16) as u16) as u32) as i32) >> 16 - (*s).bi_valid) as i16) as u16;
            }
            unsafe {
                (*s).bi_valid = (*s).bi_valid + (len - 16);
            }
        } else {
            unsafe {
                (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                    | (((((0i32 + last) as i16) as u16) as u32) as i32) << (*s).bi_valid)
                    as i16) as u16;
            }
            unsafe {
                (*s).bi_valid = (*s).bi_valid + len;
            }
        }
    }
    #[cfg(feature = "zlib-debug")]
    {
        send_bits(s, 0i32 + last, 3i32);
    }
    bi_windup(s); // align on byte boundary
    put_short!(s, stored_len as u16);
    put_short!(s, !stored_len as u16);
    if stored_len != 0 {
        #[cfg(all(not(target_os = "macos"), target_pointer_width = "64", unix))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    ((buf as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((unsafe { (*s).pending_buf }.offset((*s).pending as isize) as *mut ())
                        as *mut core::ffi::c_void) as *mut u8,
                    (stored_len as usize) as usize,
                )
            };
        }
        #[cfg(any(target_arch = "x86", all(target_arch = "arm", unix)))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    ((buf as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((unsafe { (*s).pending_buf }.offset((*s).pending as isize) as *mut ())
                        as *mut core::ffi::c_void) as *mut u8,
                    ((stored_len as u32) as usize) as usize,
                )
            };
        }
        #[cfg(target_os = "macos")]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    buf as *const u8,
                    (unsafe { (*s).pending_buf }.offset((*s).pending as isize) as *mut ())
                        as *mut u8,
                    stored_len as usize,
                )
            };
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    ((buf as *const ()) as *const core::ffi::c_void) as *const u8,
                    ((unsafe { (*s).pending_buf }.offset((*s).pending as isize) as *mut ())
                        as *mut core::ffi::c_void) as *mut u8,
                    ((stored_len as u64) as usize) as usize,
                )
            };
        }
        #[cfg(all(target_arch = "arm", windows))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    buf as *const u8,
                    (unsafe { (*s).pending_buf }.offset((*s).pending as isize) as *mut ())
                        as *mut u8,
                    (stored_len as u32) as usize,
                )
            };
        }
    }
    let __v31: c_ulong = unsafe { (*s).pending };
    unsafe {
        (*s).pending = __v31.wrapping_add(stored_len);
    }
    #[cfg(feature = "zlib-debug")]
    {
        unsafe {
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                (*s).compressed_len = unsafe { (*s).compressed_len }
                    .wrapping_add(3)
                    .wrapping_add(7)
                    & (18446744073709551608 as c_ulong);
            }
            #[cfg(any(target_pointer_width = "32", windows))]
            {
                (*s).compressed_len = unsafe { (*s).compressed_len }
                    .wrapping_add(3)
                    .wrapping_add(7)
                    & (4294967288 as c_ulong);
            }
        }
    }
    #[cfg(feature = "zlib-debug")]
    let __v16: c_ulong = unsafe { (*s).compressed_len };
    #[cfg(feature = "zlib-debug")]
    {
        unsafe {
            (*s).compressed_len = __v16.wrapping_add(stored_len.wrapping_add(4) << 3i32);
        }
    }
    #[cfg(feature = "zlib-debug")]
    let __v19: c_ulong = unsafe { (*s).bits_sent };
    #[cfg(feature = "zlib-debug")]
    {
        unsafe {
            (*s).bits_sent = __v19.wrapping_add((32 as c_ulong));
        }
    }
    #[cfg(feature = "zlib-debug")]
    let __v22: c_ulong = unsafe { (*s).bits_sent };
    #[cfg(feature = "zlib-debug")]
    {
        unsafe {
            (*s).bits_sent = __v22.wrapping_add(stored_len << 3i32);
        }
    }
}

/// Flush the bits in the bit buffer to pending output (leaves at most 7 bits)
#[unsafe(no_mangle)]
extern "C-unwind" fn _tr_flush_bits(s: *mut internal_state) {
    bi_flush(s);
}

/// Send one empty static block to give enough lookahead for inflate.
/// This takes 10 bits, of which 7 may remain in the bit buffer.
#[unsafe(no_mangle)]
extern "C-unwind" fn _tr_align(s: *mut internal_state) {
    #[cfg(not(feature = "zlib-debug"))]
    let len: i32 = 3;
    #[cfg(not(feature = "zlib-debug"))]
    {
        if (unsafe { (*s).bi_valid }) > 16 - len {
            let val: i32 = 2i32;
            unsafe {
                (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                    | ((((val as i16) as u16) as u32) as i32) << (*s).bi_valid)
                    as i16) as u16;
            }
            put_short!(s, unsafe { (*s).bi_buf });
            unsafe {
                (*s).bi_buf =
                    ((((((val as i16) as u16) as u32) as i32) >> 16 - (*s).bi_valid) as i16) as u16;
            }
            unsafe {
                (*s).bi_valid = (*s).bi_valid + (len - 16);
            }
        } else {
            unsafe {
                (*s).bi_buf =
                    (((((*s).bi_buf as u32) as i32) | 2i32 << (*s).bi_valid) as i16) as u16;
            }
            unsafe {
                (*s).bi_valid = (*s).bi_valid + len;
            }
        }
        send_code__5!(s, 256i32, static_ltree.0);
    }
    #[cfg(feature = "zlib-debug")]
    {
        send_bits(s, 2i32, 3i32);
        send_code__6!(s, 256i32, static_ltree.0);
    }
    #[cfg(feature = "zlib-debug")]
    let __v1: c_ulong = unsafe { (*s).compressed_len };
    #[cfg(feature = "zlib-debug")]
    {
        unsafe {
            (*s).compressed_len = __v1.wrapping_add(10);
        }
    }
    bi_flush(s);
}

/// Send the block data compressed using the given Huffman trees
fn compress_block(
    s: *mut internal_state,
    mut ltree: *const ct_data_s,
    mut dtree: *const ct_data_s,
) {
    let mut dist: u32 = 0; // distance of matched string
    let mut lc: i32 = 0; // match length or unmatched char (if dist == 0)
    let mut sx: u32 = 0; // running index in symbol buffers
    let mut code: u32 = 0; // the code to send
    let mut extra: i32 = 0; // number of extra bits to send
    if (unsafe { (*s).sym_next }) != 0 {
        loop {
            #[cfg(not(feature = "lit-mem"))]
            let __v0: u32 = sx;
            #[cfg(not(feature = "lit-mem"))]
            {
                sx = __v0.wrapping_add(1);
                dist = ((((unsafe { *unsafe { (*s).sym_buf }.add(__v0 as usize) }) as u32) as i32)
                    & 255) as u32;
            }
            #[cfg(not(feature = "lit-mem"))]
            let __v2: u32 = sx;
            #[cfg(not(feature = "lit-mem"))]
            {
                sx = __v2.wrapping_add(1);
            }
            #[cfg(not(feature = "lit-mem"))]
            let __v4: u32 = (((((unsafe { *unsafe { (*s).sym_buf }.add(__v2 as usize) }) as u32)
                as i32)
                & 255) as u32)
                << 8i32;
            #[cfg(not(feature = "lit-mem"))]
            {
                dist = dist.wrapping_add(__v4);
            }
            #[cfg(feature = "lit-mem")]
            {
                dist = (unsafe { *unsafe { (*s).d_buf }.add(sx as usize) }) as u32;
            }
            let __v7: u32 = sx;
            sx = __v7.wrapping_add(1);
            #[cfg(not(feature = "lit-mem"))]
            {
                lc = ((unsafe { *unsafe { (*s).sym_buf }.add(__v7 as usize) }) as u32) as i32;
            }
            #[cfg(feature = "lit-mem")]
            {
                lc = ((unsafe { *unsafe { (*s).l_buf }.add(__v7 as usize) }) as u32) as i32;
            }
            if dist == 0 {
                #[cfg(feature = "zlib-debug")]
                {
                    send_code__3!(s, lc, ltree);
                }
                #[cfg(all(target_env = "gnu", feature = "zlib-debug"))]
                {
                    Tracecv!(
                        unsafe { slate_support::isgraph(lc as i32) },
                        slate_support::stderr(),
                        c" '%c' ".as_ptr() as *mut c_char,
                        lc
                    );
                }
                #[cfg(all(any(target_env = "musl", target_os = "macos"), feature = "zlib-debug"))]
                {
                    Tracecv!(
                        unsafe { slate_support::isgraph(lc) },
                        slate_support::stderr(),
                        c" '%c' ".as_ptr() as *mut c_char,
                        lc
                    );
                }
                #[cfg(all(
                    not(any(target_os = "linux", target_os = "macos")),
                    feature = "zlib-debug"
                ))]
                {
                    Tracecv__3!(
                        unsafe { slate_support::isgraph(lc) },
                        slate_support::stderr(),
                        c" '%c' ".as_ptr() as *mut c_char,
                        lc
                    );
                }
                #[cfg(not(feature = "zlib-debug"))]
                {
                    send_code__4!(s, lc, ltree);
                }
            } else {
                // Here, lc is the match length - MIN_MATCH
                code = (unsafe { _length_code.0[lc as usize] }) as u32;
                #[cfg(feature = "zlib-debug")]
                {
                    send_code__3!(s, code.wrapping_add(256).wrapping_add(1), ltree);
                }
                #[cfg(not(feature = "zlib-debug"))]
                {
                    send_code__4!(s, code.wrapping_add(256).wrapping_add(1), ltree);
                }
                // send length code
                extra = unsafe { extra_lbits.0[code as usize] };
                if extra != 0 {
                    lc -= unsafe { base_length.0[code as usize] };
                    #[cfg(feature = "zlib-debug")]
                    {
                        send_bits(s, lc, extra);
                    }
                    #[cfg(not(feature = "zlib-debug"))]
                    {
                        send_bits!(s, lc, extra);
                    }
                }
                dist = dist.wrapping_sub(1); // dist is now the match distance - 1
                code = d_code!(dist) as u32;
                #[cfg(feature = "zlib-debug")]
                {
                    Assert!(code < 30, c"bad d_code".as_ptr() as *mut c_char);
                    send_code__3!(s, code, dtree);
                }
                #[cfg(not(feature = "zlib-debug"))]
                {
                    send_code__4!(s, code, dtree);
                }
                // send the distance code
                extra = unsafe { extra_dbits.0[code as usize] };
                if extra != 0 {
                    dist = dist.wrapping_sub((unsafe { base_dist.0[code as usize] }) as u32);
                    #[cfg(feature = "zlib-debug")]
                    {
                        send_bits(s, dist as i32, extra);
                    }
                    #[cfg(not(feature = "zlib-debug"))]
                    {
                        send_bits!(s, dist as i32, extra);
                    }
                }
            }
            // literal or match pair ?
            // Check for no overlay of pending_buf on needed symbols
            #[cfg(all(not(feature = "lit-mem"), feature = "zlib-debug"))]
            {
                Assert!(
                    (unsafe { (*s).pending })
                        < (unsafe { (*s).lit_bufsize }.wrapping_add(sx) as c_ulong),
                    c"pendingBuf overflow".as_ptr() as *mut c_char
                );
            }
            #[cfg(all(feature = "lit-mem", feature = "zlib-debug"))]
            {
                Assert!(
                    (unsafe { (*s).pending })
                        < (2u32.wrapping_mul(unsafe { (*s).lit_bufsize }.wrapping_add(sx))
                            as c_ulong),
                    c"pendingBuf overflow".as_ptr() as *mut c_char
                );
            }
            if sx >= unsafe { (*s).sym_next } {
                break;
            }
        }
    }
    #[cfg(feature = "zlib-debug")]
    {
        send_code__3!(s, 256i32, ltree);
    }
    #[cfg(not(feature = "zlib-debug"))]
    {
        send_code__4!(s, 256i32, ltree);
    }
}

/// Check if the data type is TEXT or BINARY, using the following algorithm:
/// - TEXT if the two conditions below are satisfied:
///    a) There are no non-portable control characters belonging to the
///       "block list" (0..6, 14..25, 28..31).
///    b) There is at least one printable character belonging to the
///       "allow list" (9 {TAB}, 10 {LF}, 13 {CR}, 32..255).
/// - BINARY otherwise.
/// - The following partially-portable control characters form a
///   "gray list" that is ignored in this detection algorithm:
///   (7 {BEL}, 8 {BS}, 11 {VT}, 12 {FF}, 26 {SUB}, 27 {ESC}).
/// IN assertion: the fields Freq of dyn_ltree are set.
fn detect_data_type(s: *mut internal_state) -> i32 {
    // block_mask is the bit mask of block-listed bytes
    // set bits 0..6, 14..25, and 28..31
    // 0xf3ffc07f = binary 11110011111111111100000001111111
    let mut block_mask: c_ulong = 4093624447;
    // Check for non-textual ("block-listed") bytes.
    let mut n: i32 = 0;
    while n <= 31 {
        if block_mask & 1 != 0
            && (((unsafe {
                (*((*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s).offset(n as isize))
                    .fc
                    .freq
            }) as u32) as i32)
                != 0
        {
            return 0;
        }
        n += 1;
        block_mask >>= 1i32;
    }
    // Check for textual ("allow-listed") bytes.
    if (((unsafe {
        (*((*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s).add(9))
            .fc
            .freq
    }) as u32) as i32)
        != 0
        || (((unsafe {
            (*((*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s).add(10))
                .fc
                .freq
        }) as u32) as i32)
            != 0
        || (((unsafe {
            (*((*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s).add(13))
                .fc
                .freq
        }) as u32) as i32)
            != 0
    {
        return 1;
    }
    n = 32;
    while n < 256 {
        if (((unsafe {
            (*((*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s).offset(n as isize))
                .fc
                .freq
        }) as u32) as i32)
            != 0
        {
            return 1;
        }
        n += 1;
    }
    // There are no "block-listed" or "allow-listed" bytes:
    // this stream either is empty or has tolerated ("gray-listed") bytes only.
    0
}

/// Determine the best encoding for the current block: dynamic trees, static
/// trees or store, and write out the encoded block.
#[unsafe(no_mangle)]
extern "C-unwind" fn _tr_flush_block(
    s: *mut internal_state,
    buf: *mut c_char,
    stored_len: c_ulong,
    last: i32,
) {
    let mut opt_lenb: c_ulong = 0;
    let mut static_lenb: c_ulong = 0; // opt_len and static_len in bytes
    let mut max_blindex: i32 = 0; // index of last bit length code of non zero freq
    // Build the Huffman trees unless a stored block is forced
    if (unsafe { (*s).level }) > 0 {
        // Check if the file is binary or text
        if (unsafe { (*(*s).strm).data_type }) == 2 {
            unsafe {
                (*(*s).strm).data_type = detect_data_type(s);
            }
        }
        // Construct the literal and distance trees
        build_tree(s, unsafe { std::ptr::addr_of_mut!((*s).l_desc) });
        #[cfg(feature = "zlib-debug")]
        {
            Tracev__3!(
                slate_support::stderr(),
                c"\nlit data: dyn %lu, stat %lu".as_ptr() as *mut c_char,
                unsafe { (*s).opt_len },
                unsafe { (*s).static_len }
            );
        }
        build_tree(s, unsafe { std::ptr::addr_of_mut!((*s).d_desc) });
        #[cfg(feature = "zlib-debug")]
        {
            Tracev__3!(
                slate_support::stderr(),
                c"\ndist data: dyn %lu, stat %lu".as_ptr() as *mut c_char,
                unsafe { (*s).opt_len },
                unsafe { (*s).static_len }
            );
        }
        // At this point, opt_len and static_len are the total bit lengths of
        // the compressed block data, excluding the tree representations.
        // Build the bit length tree for the above two trees, and get the index
        // in bl_order of the last bit length code to send.
        max_blindex = build_bl_tree(s);
        // Determine the best encoding. Compute the block lengths in bytes.
        opt_lenb = unsafe { (*s).opt_len }.wrapping_add(3).wrapping_add(7) >> 3i32;
        static_lenb = unsafe { (*s).static_len }.wrapping_add(3).wrapping_add(7) >> 3i32;
        #[cfg(feature = "zlib-debug")]
        {
            Tracev__4!(
                slate_support::stderr(),
                c"\nopt %lu(%lu) stat %lu(%lu) stored %lu lit %u ".as_ptr() as *mut c_char,
                opt_lenb,
                unsafe { (*s).opt_len },
                static_lenb,
                unsafe { (*s).static_len },
                stored_len,
                (unsafe { (*s).sym_next }) / 3
            );
        }
        if static_lenb <= opt_lenb || (unsafe { (*s).strategy }) == 4 {
            opt_lenb = static_lenb;
        }
    } else {
        #[cfg(feature = "zlib-debug")]
        {
            Assert!(!buf.is_null(), c"lost buf".as_ptr() as *mut c_char);
        }
        let __v0: c_ulong = stored_len.wrapping_add(5);
        static_lenb = __v0;
        opt_lenb = __v0; // force a stored block
    }
    if stored_len.wrapping_add(4) <= opt_lenb && !buf.is_null() {
        // 4: two words for the lengths
        //
        // The test buf != NULL is only necessary if LIT_BUFSIZE > WSIZE.
        // Otherwise we can't have processed more than WSIZE input bytes since
        // the last block flush, because compression would have been
        // successful. If LIT_BUFSIZE <= WSIZE, it is never too late to
        // transform a block into a stored block.
        _tr_stored_block(s, buf, stored_len, last);
    } else {
        if static_lenb == opt_lenb {
            #[cfg(not(feature = "zlib-debug"))]
            let len: i32 = 3;
            #[cfg(not(feature = "zlib-debug"))]
            {
                if (unsafe { (*s).bi_valid }) > 16 - len {
                    let val: i32 = 2i32 + last;
                    unsafe {
                        (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                            | ((((val as i16) as u16) as u32) as i32) << (*s).bi_valid)
                            as i16) as u16;
                    }
                    put_short!(s, unsafe { (*s).bi_buf });
                    unsafe {
                        (*s).bi_buf = ((((((val as i16) as u16) as u32) as i32)
                            >> 16 - (*s).bi_valid) as i16)
                            as u16;
                    }
                    unsafe {
                        (*s).bi_valid = (*s).bi_valid + (len - 16);
                    }
                } else {
                    unsafe {
                        (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                            | (((((2i32 + last) as i16) as u16) as u32) as i32) << (*s).bi_valid)
                            as i16) as u16;
                    }
                    unsafe {
                        (*s).bi_valid = (*s).bi_valid + len;
                    }
                }
            }
            #[cfg(feature = "zlib-debug")]
            {
                send_bits(s, 2i32 + last, 3i32);
            }
            compress_block(
                s,
                unsafe { std::ptr::addr_of!(static_ltree.0) as *const ct_data_s },
                unsafe { std::ptr::addr_of!(static_dtree.0) as *const ct_data_s },
            );
            #[cfg(feature = "zlib-debug")]
            let __v2: c_ulong = unsafe { (*s).compressed_len };
            #[cfg(feature = "zlib-debug")]
            {
                unsafe {
                    (*s).compressed_len =
                        __v2.wrapping_add((3 as c_ulong).wrapping_add((*s).static_len));
                }
            }
        } else {
            #[cfg(not(feature = "zlib-debug"))]
            let len: i32 = 3;
            #[cfg(not(feature = "zlib-debug"))]
            {
                if (unsafe { (*s).bi_valid }) > 16 - len {
                    let val: i32 = 4i32 + last;
                    unsafe {
                        (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                            | ((((val as i16) as u16) as u32) as i32) << (*s).bi_valid)
                            as i16) as u16;
                    }
                    put_short!(s, unsafe { (*s).bi_buf });
                    unsafe {
                        (*s).bi_buf = ((((((val as i16) as u16) as u32) as i32)
                            >> 16 - (*s).bi_valid) as i16)
                            as u16;
                    }
                    unsafe {
                        (*s).bi_valid = (*s).bi_valid + (len - 16);
                    }
                } else {
                    unsafe {
                        (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                            | (((((4i32 + last) as i16) as u16) as u32) as i32) << (*s).bi_valid)
                            as i16) as u16;
                    }
                    unsafe {
                        (*s).bi_valid = (*s).bi_valid + len;
                    }
                }
            }
            #[cfg(feature = "zlib-debug")]
            {
                send_bits(s, 4i32 + last, 3i32);
            }
            send_all_trees(
                s,
                (unsafe { (*s).l_desc.max_code }) + 1,
                (unsafe { (*s).d_desc.max_code }) + 1,
                max_blindex + 1,
            );
            compress_block(
                s,
                (unsafe { (*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s }) as *const ct_data_s,
                (unsafe { (*s).dyn_dtree.as_mut_ptr() as *mut ct_data_s }) as *const ct_data_s,
            );
            #[cfg(feature = "zlib-debug")]
            let __v5: c_ulong = unsafe { (*s).compressed_len };
            #[cfg(feature = "zlib-debug")]
            {
                unsafe {
                    (*s).compressed_len =
                        __v5.wrapping_add((3 as c_ulong).wrapping_add((*s).opt_len));
                }
            }
        }
    }
    #[cfg(feature = "zlib-debug")]
    {
        Assert!(
            (unsafe { (*s).compressed_len }) == unsafe { (*s).bits_sent },
            c"bad compressed size".as_ptr() as *mut c_char
        );
    }
    // The above check is made mod 2^32, for files larger than 512 MB
    // and uLong implemented on 32 bits.
    init_block(s);
    if last != 0 {
        bi_windup(s);
        #[cfg(feature = "zlib-debug")]
        let __v8: c_ulong = unsafe { (*s).compressed_len };
        #[cfg(feature = "zlib-debug")]
        {
            unsafe {
                (*s).compressed_len = __v8.wrapping_add(7);
            }
        }
    }
    #[cfg(feature = "zlib-debug")]
    {
        Tracev__3!(
            slate_support::stderr(),
            c"\ncomprlen %lu(%lu) ".as_ptr() as *mut c_char,
            (unsafe { (*s).compressed_len }) >> 3i32,
            unsafe { (*s).compressed_len }
                .wrapping_sub((7 as c_ulong).wrapping_mul(last as c_ulong))
        );
    }
}

/// Save the match info and tally the frequency counts. Return true if
/// the current block must be flushed.
#[unsafe(no_mangle)]
extern "C-unwind" fn _tr_tally(s: *mut internal_state, mut dist: u32, lc: u32) -> i32 {
    #[cfg(not(feature = "lit-mem"))]
    let __v1: u32 = unsafe { (*s).sym_next };
    unsafe {
        #[cfg(not(feature = "lit-mem"))]
        {
            (*s).sym_next = __v1.wrapping_add(1);
        }
        #[cfg(feature = "lit-mem")]
        {
            *unsafe { (*s).d_buf }.offset((*s).sym_next as isize) = dist as u16;
        }
    }
    #[cfg(not(feature = "lit-mem"))]
    {
        unsafe {
            *unsafe { (*s).sym_buf }.add(__v1 as usize) = dist as u8;
        }
    }
    let __v4: u32 = unsafe { (*s).sym_next };
    unsafe {
        (*s).sym_next = __v4.wrapping_add(1);
    }
    unsafe {
        #[cfg(not(feature = "lit-mem"))]
        {
            *unsafe { (*s).sym_buf }.add(__v4 as usize) = (dist >> 8i32) as u8;
        }
        #[cfg(feature = "lit-mem")]
        {
            *unsafe { (*s).l_buf }.add(__v4 as usize) = lc as u8;
        }
    }
    #[cfg(not(feature = "lit-mem"))]
    let __v7: u32 = unsafe { (*s).sym_next };
    #[cfg(not(feature = "lit-mem"))]
    {
        unsafe {
            (*s).sym_next = __v7.wrapping_add(1);
        }
        unsafe {
            *unsafe { (*s).sym_buf }.add(__v7 as usize) = lc as u8;
        }
    }
    if dist == 0 {
        // lc is the unmatched char
        let __v9: *mut ct_data_s =
            unsafe { ((*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s).add(lc as usize) };
        unsafe {
            (*__v9).fc.freq = (((((*__v9).fc.freq as u32) as i32) + 1) as i16) as u16;
        }
    } else {
        let __v13: u32 = unsafe { (*s).matches };
        unsafe {
            (*s).matches = __v13.wrapping_add(1);
        }
        // Here, lc is the match length - MIN_MATCH
        dist = dist.wrapping_sub(1); // dist = match distance - 1
        #[cfg(feature = "zlib-debug")]
        {
            Assert!(
                (((dist as u16) as u32) as i32) < (((MAX_DIST!(s) as u16) as u32) as i32)
                    && (((lc as u16) as u32) as i32) <= 255i32
                    && ((((d_code!(dist) as i16) as u16) as u32) as i32) < 30,
                c"_tr_tally: bad match".as_ptr() as *mut c_char
            );
        }
        let __v17: *mut ct_data_s = unsafe {
            ((*s).dyn_ltree.as_mut_ptr() as *mut ct_data_s)
                .offset((((_length_code.0[lc as usize] as u32) as i32) + 256 + 1) as isize)
        };
        unsafe {
            (*__v17).fc.freq = (((((*__v17).fc.freq as u32) as i32) + 1) as i16) as u16;
        }
        let __v20: *mut ct_data_s = unsafe {
            ((*s).dyn_dtree.as_mut_ptr() as *mut ct_data_s).offset(d_code!(dist) as isize)
        };
        unsafe {
            (*__v20).fc.freq = (((((*__v20).fc.freq as u32) as i32) + 1) as i16) as u16;
        }
    }
    ((unsafe { (*s).sym_next }) == unsafe { (*s).sym_end }) as i32
}
