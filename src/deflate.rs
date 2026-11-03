//! deflate.c -- compress data using the deflation algorithm
//! Copyright (C) 1995-2026 Jean-loup Gailly and Mark Adler
//! For conditions of distribution and use, see copyright notice in zlib.h
#[cfg(all(target_pointer_width = "64", windows))]
macro_rules! CLEAR_HASH__2 {
    ($s:expr) => {
        loop {
            unsafe {
                *unsafe { (*$s).head }
                    .offset(unsafe { (*$s).hash_size }.wrapping_sub(1) as isize) = 0;
            }
            zmemzero!(
                unsafe { (*$s).head },
                (unsafe { (*$s).hash_size }.wrapping_sub(1) as u64)
                    .wrapping_mul(std::mem::size_of::<u16>() as u64)
            );
            unsafe {
                (*$s).slid = 0;
            }
            if true {
                break;
            }
        }
    };
}

#[cfg(target_pointer_width = "32")]
macro_rules! CLEAR_HASH__3 {
    ($s:expr) => {
        loop {
            unsafe {
                *unsafe { (*$s).head }
                    .offset(unsafe { (*$s).hash_size }.wrapping_sub(1) as isize) = 0;
            }
            zmemzero!(
                unsafe { (*$s).head },
                unsafe { (*$s).hash_size }
                    .wrapping_sub(1)
                    .wrapping_mul(std::mem::size_of::<u16>() as u32)
            );
            unsafe {
                (*$s).slid = 0;
            }
            if true {
                break;
            }
        }
    };
}

#[cfg(all(target_pointer_width = "64", unix))]
macro_rules! CLEAR_HASH {
    ($s:expr) => {
        loop {
            unsafe {
                *unsafe { (*$s).head }
                    .offset(unsafe { (*$s).hash_size }.wrapping_sub(1) as isize) = 0;
            }
            zmemzero!(
                unsafe { (*$s).head },
                (unsafe { (*$s).hash_size }.wrapping_sub(1) as c_ulong)
                    .wrapping_mul(std::mem::size_of::<u16>() as c_ulong)
            );
            unsafe {
                (*$s).slid = 0;
            }
            if true {
                break;
            }
        }
    };
}

macro_rules! ERR_MSG {
    ($err:expr) => {
        unsafe {
            z_errmsg[(if ($err) < -6 || ($err) > 2 {
                9i32
            } else {
                2 - $err
            }) as usize]
        }
    };
}

macro_rules! ERR_RETURN {
    ($strm:expr, $err:expr) => {{
        unsafe {
            (*$strm).msg = ERR_MSG!($err);
        }
        return $err;
    }};
}

#[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
macro_rules! FLUSH_BLOCK_ONLY__4 {
    ($s:expr, $last:expr) => {
        unsafe {
            _tr_flush_block(
                $s,
                if (*$s).block_start >= 0 {
                    unsafe { (*$s).window }.add(((*$s).block_start as u32) as usize) as *mut c_char
                } else {
                    std::ptr::null_mut::<c_char>()
                },
                (((*$s).strstart as c_long) - (*$s).block_start) as c_ulong,
                $last,
            )
        };
        unsafe {
            (*$s).block_start = (*$s).strstart as c_long;
        }
        flush_pending(unsafe { (*$s).strm });
        Tracev!(slate_support::stderr(), c"[FLUSH]".as_ptr() as *mut c_char);
    };
}

#[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
macro_rules! FLUSH_BLOCK__2 {
    ($s:expr, $last:expr) => {{
        FLUSH_BLOCK_ONLY__4!($s, $last);
        if (unsafe { (*(*$s).strm).avail_out }) == 0 {
            return (if ($last) != 0 { 2i32 } else { 0i32 }) as u32;
        }
    }};
}

#[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
macro_rules! FLUSH_BLOCK_ONLY {
    ($s:expr, $last:expr) => {
        unsafe {
            _tr_flush_block(
                $s,
                if (*$s).block_start >= 0 {
                    unsafe { (*$s).window }.add(((*$s).block_start as u32) as usize) as *mut c_char
                } else {
                    std::ptr::null_mut::<c_char>()
                },
                (((*$s).strstart as c_long) - (*$s).block_start) as c_ulong,
                $last,
            )
        };
        unsafe {
            (*$s).block_start = (*$s).strstart as c_long;
        }
        flush_pending(unsafe { (*$s).strm });
    };
}

#[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
macro_rules! FLUSH_BLOCK__3 {
    ($s:expr, $last:expr) => {{
        FLUSH_BLOCK_ONLY!($s, $last);
        if (unsafe { (*(*$s).strm).avail_out }) == 0 {
            return (if ($last) != 0 { 2i32 } else { 0i32 }) as u32;
        }
    }};
}

#[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
macro_rules! FLUSH_BLOCK_ONLY__2 {
    ($s:expr, $last:expr) => {
        unsafe {
            _tr_flush_block(
                $s,
                if (*$s).block_start >= 0 {
                    unsafe { (*$s).window }.add((((*$s).block_start as i32) as u32) as usize)
                        as *mut c_char
                } else {
                    std::ptr::null_mut::<c_char>()
                },
                (((*$s).strstart as c_long) - (*$s).block_start) as c_ulong,
                $last,
            )
        };
        unsafe {
            (*$s).block_start = (*$s).strstart as c_long;
        }
        flush_pending(unsafe { (*$s).strm });
        Tracev!(slate_support::stderr(), c"[FLUSH]".as_ptr() as *mut c_char);
    };
}

#[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
macro_rules! FLUSH_BLOCK {
    ($s:expr, $last:expr) => {{
        FLUSH_BLOCK_ONLY__2!($s, $last);
        if (unsafe { (*(*$s).strm).avail_out }) == 0 {
            return (if ($last) != 0 { 2i32 } else { 0i32 }) as u32;
        }
    }};
}

#[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
macro_rules! FLUSH_BLOCK_ONLY__3 {
    ($s:expr, $last:expr) => {
        unsafe {
            _tr_flush_block(
                $s,
                if (*$s).block_start >= 0 {
                    unsafe { (*$s).window }.add((((*$s).block_start as i32) as u32) as usize)
                        as *mut c_char
                } else {
                    std::ptr::null_mut::<c_char>()
                },
                (((*$s).strstart as c_long) - (*$s).block_start) as c_ulong,
                $last,
            )
        };
        unsafe {
            (*$s).block_start = (*$s).strstart as c_long;
        }
        flush_pending(unsafe { (*$s).strm });
    };
}

#[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
macro_rules! FLUSH_BLOCK__4 {
    ($s:expr, $last:expr) => {{
        FLUSH_BLOCK_ONLY__3!($s, $last);
        if (unsafe { (*(*$s).strm).avail_out }) == 0 {
            return (if ($last) != 0 { 2i32 } else { 0i32 }) as u32;
        }
    }};
}

#[cfg(not(feature = "no-gzip"))]
macro_rules! HCRC_UPDATE {
    ($beg:expr, $s:expr, $strm:expr) => {
        loop {
            if (unsafe { (*(*$s).gzhead).hcrc }) != 0 && (unsafe { (*$s).pending }) > $beg {
                unsafe {
                    (*$strm).adler = crc32_z(
                        (*$strm).adler,
                        unsafe { (*$s).pending_buf }.offset(($beg) as isize) as *const u8,
                        unsafe { (*$s).pending }.wrapping_sub($beg) as size_t,
                    );
                }
            }
            if true {
                break;
            }
        }
    };
}

#[cfg(feature = "fastest")]
macro_rules! INSERT_STRING__2 {
    ($s:expr, $str:expr, $match_head:expr) => {
        UPDATE_HASH__2!($s, (*$s).ins_h, unsafe {
            *unsafe { (*$s).window }.offset($str.wrapping_add(2u32) as isize)
        });
        let __v: u32 = (unsafe { *unsafe { (*$s).head }.offset((*$s).ins_h as isize) }) as u32;
        $match_head = __v;
        let __v2: u16 = ($str) as u16;
        unsafe {
            *unsafe { (*$s).head }.offset((*$s).ins_h as isize) = __v2;
        }
    };
}

#[cfg(not(feature = "fastest"))]
macro_rules! INSERT_STRING {
    ($s:expr, $str:expr, $match_head:expr) => {
        UPDATE_HASH__2!($s, (*$s).ins_h, unsafe {
            *unsafe { (*$s).window }.offset($str.wrapping_add(2u32) as isize)
        });
        let __v: u16 = unsafe { *unsafe { (*$s).head }.offset((*$s).ins_h as isize) };
        unsafe {
            *unsafe { (*$s).prev }.offset((($str) & (*$s).w_mask) as isize) = __v;
        }
        let __v2: u32 = __v as u32;
        $match_head = __v2;
        let __v3: u16 = ($str) as u16;
        unsafe {
            *unsafe { (*$s).head }.offset((*$s).ins_h as isize) = __v3;
        }
    };
}

macro_rules! MAX_DIST {
    ($s:expr) => {
        unsafe { (*$s).w_size }.wrapping_sub(262u32)
    };
}

macro_rules! MIN {
    ($a:expr, $b:expr) => {
        if ($a) > $b { $b } else { $a }
    };
}

macro_rules! MIN__2 {
    ($a:expr, $b:expr) => {
        if ($a) > (($b) as c_ulong) {
            ($b) as c_ulong
        } else {
            $a
        }
    };
}

macro_rules! RANK {
    ($f:expr) => {
        ($f) * 2 - if ($f) > 4 { 9 } else { 0 }
    };
}

macro_rules! TRY_FREE {
    ($s:expr, $p:expr) => {
        if !$p.is_null() {
            ZFREE!($s, $p);
        }
    };
}

macro_rules! TRY_FREE__2 {
    ($s:expr, $p:expr) => {
        if !$p.is_null() {
            ZFREE!($s, $p);
        }
    };
}

#[cfg(all(not(target_os = "freebsd"), unix, feature = "zlib-debug"))]
macro_rules! Tracev {
    ($x__0:expr, $x__1:expr) => {
        if (unsafe { z_verbose }) > 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(all(target_os = "freebsd", feature = "zlib-debug"))]
macro_rules! Tracev {
    ($x__0:expr, $x__1:expr) => {
        if (unsafe { z_verbose }) > 0 {
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char) };
        }
    };
}

#[cfg(all(windows, feature = "zlib-debug"))]
macro_rules! Tracev {
    ($x__0:expr, $x__1:expr) => {
        if (unsafe { z_verbose }) > 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(all(not(target_os = "freebsd"), unix, feature = "zlib-debug"))]
macro_rules! Tracevv__2 {
    ($x__0:expr, $x__1:expr, $x__2:expr) => {
        if (unsafe { z_verbose }) > 1 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf($x__0, ($x__1) as *const c_char, (($x__2) as u32) as i32)
            };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(all(target_os = "freebsd", feature = "zlib-debug"))]
macro_rules! Tracevv__2 {
    ($x__0:expr, $x__1:expr, $x__2:expr) => {
        if (unsafe { z_verbose }) > 1 {
            unsafe {
                slate_support::fprintf($x__0, ($x__1) as *const c_char, (($x__2) as u32) as i32)
            };
        }
    };
}

#[cfg(all(windows, feature = "zlib-debug"))]
macro_rules! Tracevv__2 {
    ($x__0:expr, $x__1:expr, $x__2:expr) => {
        if (unsafe { z_verbose }) > 1 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf($x__0, ($x__1) as *const c_char, (($x__2) as u32) as i32)
            };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

macro_rules! UPDATE_HASH {
    ($s:expr, $h:expr, $c:expr) => {
        unsafe {
            $h = (($h) << (*$s).hash_shift ^ (((($c) as u32) as i32) as u32)) & (*$s).hash_mask;
        }
    };
}

macro_rules! UPDATE_HASH__2 {
    ($s:expr, $h:expr, $c:expr) => {{
        let __v: u32 = ((unsafe { $h }) << unsafe { (*$s).hash_shift }
            ^ (((($c) as u32) as i32) as u32))
            & unsafe { (*$s).hash_mask };
        unsafe {
            $h = __v;
        }
        __v
    }};
}

macro_rules! ZALLOC {
    ($strm:expr, $items:expr, $size:expr) => {
        unsafe { unsafe { (*$strm).zalloc }.unwrap()((*$strm).opaque, $items, ($size) as u32) }
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! _tr_tally_dist__4 {
    ($s:expr, $distance:expr, $length:expr, $flush:expr) => {
        $flush = unsafe { _tr_tally($s, ($distance) as u32, $length) };
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! _tr_tally_dist__3 {
    ($s:expr, $distance:expr, $length:expr, $flush:expr) => {
        $flush = unsafe { _tr_tally($s, $distance, $length) };
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! _tr_tally_lit__2 {
    ($s:expr, $c:expr, $flush:expr) => {
        $flush = unsafe { _tr_tally($s, 0u32, ($c) as u32) };
    };
}

#[cfg(all(feature = "lit-mem", not(feature = "zlib-debug")))]
macro_rules! _tr_tally_dist__2 {
    ($s:expr, $distance:expr, $length:expr, $flush:expr) => {
        let len: u8 = ($length) as u8;
        let mut dist: u16 = ($distance) as u16;
        unsafe {
            *unsafe { (*$s).d_buf }.offset((*$s).sym_next as isize) = dist;
        }
        let __v: *mut internal_state = $s;
        let __v2: u32 = unsafe { (*__v).sym_next };
        unsafe {
            (*__v).sym_next = __v2.wrapping_add(1);
        }
        unsafe {
            *unsafe { (*$s).l_buf }.add(__v2 as usize) = len;
        }
        dist = ((((dist as u32) as i32) - 1) as i16) as u16;
        let __v6: *mut ct_data_s = unsafe {
            ((*$s).dyn_ltree.as_mut_ptr() as *mut ct_data_s).offset(
                (((*(std::ptr::addr_of!(_length_code) as *const u8)
                    .offset(((len as u32) as i32) as isize) as u32) as i32)
                    + 256
                    + 1) as isize,
            )
        };
        unsafe {
            (*__v6).fc.freq = (((((*__v6).fc.freq as u32) as i32) + 1) as i16) as u16;
        }
        let __v9: *mut ct_data_s = unsafe {
            ((*$s).dyn_dtree.as_mut_ptr() as *mut ct_data_s).offset(d_code__2!(dist) as isize)
        };
        unsafe {
            (*__v9).fc.freq = (((((*__v9).fc.freq as u32) as i32) + 1) as i16) as u16;
        }
        $flush = ((unsafe { (*$s).sym_next }) == unsafe { (*$s).sym_end }) as i32;
    };
}

#[cfg(all(feature = "lit-mem", not(feature = "zlib-debug")))]
macro_rules! _tr_tally_lit__3 {
    ($s:expr, $c:expr, $flush:expr) => {
        let cc: u8 = $c;
        unsafe {
            *unsafe { (*$s).d_buf }.offset((*$s).sym_next as isize) = 0;
        }
        let __v: *mut internal_state = $s;
        let __v2: u32 = unsafe { (*__v).sym_next };
        unsafe {
            (*__v).sym_next = __v2.wrapping_add(1);
        }
        unsafe {
            *unsafe { (*$s).l_buf }.add(__v2 as usize) = cc;
        }
        let __v4: *mut ct_data_s = unsafe {
            ((*$s).dyn_ltree.as_mut_ptr() as *mut ct_data_s).offset(((cc as u32) as i32) as isize)
        };
        unsafe {
            (*__v4).fc.freq = (((((*__v4).fc.freq as u32) as i32) + 1) as i16) as u16;
        }
        $flush = ((unsafe { (*$s).sym_next }) == unsafe { (*$s).sym_end }) as i32;
    };
}

#[cfg(all(not(feature = "lit-mem"), not(feature = "zlib-debug")))]
macro_rules! _tr_tally_dist {
    ($s:expr, $distance:expr, $length:expr, $flush:expr) => {
        let len: u8 = ($length) as u8;
        let mut dist: u16 = ($distance) as u16;
        let __v: *mut internal_state = $s;
        let __v2: u32 = unsafe { (*__v).sym_next };
        unsafe {
            (*__v).sym_next = __v2.wrapping_add(1);
        }
        unsafe {
            *unsafe { (*$s).sym_buf }.add(__v2 as usize) = dist as u8;
        }
        let __v4: *mut internal_state = $s;
        let __v5: u32 = unsafe { (*__v4).sym_next };
        unsafe {
            (*__v4).sym_next = __v5.wrapping_add(1);
        }
        unsafe {
            *unsafe { (*$s).sym_buf }.add(__v5 as usize) =
                ((((dist as u32) as i32) >> 8i32) as i8) as u8;
        }
        let __v7: *mut internal_state = $s;
        let __v8: u32 = unsafe { (*__v7).sym_next };
        unsafe {
            (*__v7).sym_next = __v8.wrapping_add(1);
        }
        unsafe {
            *unsafe { (*$s).sym_buf }.add(__v8 as usize) = len;
        }
        dist = ((((dist as u32) as i32) - 1) as i16) as u16;
        let __v12: *mut ct_data_s = unsafe {
            ((*$s).dyn_ltree.as_mut_ptr() as *mut ct_data_s).offset(
                (((*(std::ptr::addr_of!(_length_code) as *const u8)
                    .offset(((len as u32) as i32) as isize) as u32) as i32)
                    + 256
                    + 1) as isize,
            )
        };
        unsafe {
            (*__v12).fc.freq = (((((*__v12).fc.freq as u32) as i32) + 1) as i16) as u16;
        }
        let __v15: *mut ct_data_s = unsafe {
            ((*$s).dyn_dtree.as_mut_ptr() as *mut ct_data_s).offset(d_code__2!(dist) as isize)
        };
        unsafe {
            (*__v15).fc.freq = (((((*__v15).fc.freq as u32) as i32) + 1) as i16) as u16;
        }
        $flush = ((unsafe { (*$s).sym_next }) == unsafe { (*$s).sym_end }) as i32;
    };
}

#[cfg(all(not(feature = "lit-mem"), not(feature = "zlib-debug")))]
macro_rules! _tr_tally_lit {
    ($s:expr, $c:expr, $flush:expr) => {
        let cc: u8 = $c;
        let __v: *mut internal_state = $s;
        let __v2: u32 = unsafe { (*__v).sym_next };
        unsafe {
            (*__v).sym_next = __v2.wrapping_add(1);
        }
        unsafe {
            *unsafe { (*$s).sym_buf }.add(__v2 as usize) = 0;
        }
        let __v4: *mut internal_state = $s;
        let __v5: u32 = unsafe { (*__v4).sym_next };
        unsafe {
            (*__v4).sym_next = __v5.wrapping_add(1);
        }
        unsafe {
            *unsafe { (*$s).sym_buf }.add(__v5 as usize) = 0;
        }
        let __v7: *mut internal_state = $s;
        let __v8: u32 = unsafe { (*__v7).sym_next };
        unsafe {
            (*__v7).sym_next = __v8.wrapping_add(1);
        }
        unsafe {
            *unsafe { (*$s).sym_buf }.add(__v8 as usize) = cc;
        }
        let __v10: *mut ct_data_s = unsafe {
            ((*$s).dyn_ltree.as_mut_ptr() as *mut ct_data_s).offset(((cc as u32) as i32) as isize)
        };
        unsafe {
            (*__v10).fc.freq = (((((*__v10).fc.freq as u32) as i32) + 1) as i16) as u16;
        }
        $flush = ((unsafe { (*$s).sym_next }) == unsafe { (*$s).sym_end }) as i32;
    };
}

#[cfg(not(feature = "zlib-debug"))]
macro_rules! d_code__2 {
    ($dist:expr) => {
        if ((($dist) as u32) as i32) < 256 {
            ((unsafe {
                *(std::ptr::addr_of!(_dist_code) as *const u8)
                    .offset(((($dist) as u32) as i32) as isize)
            }) as u32) as i32
        } else {
            ((unsafe {
                *(std::ptr::addr_of!(_dist_code) as *const u8)
                    .offset((256 + (((($dist) as u32) as i32) >> 7i32)) as isize)
            }) as u32) as i32
        }
    };
}

#[cfg(all(target_pointer_width = "64", windows))]
macro_rules! zmemzero__2 {
    ($dest:expr, $len:expr) => {
        unsafe {
            std::ptr::write_bytes(
                (($dest) as *mut ()) as *mut u8,
                0u8,
                (($len) as u64) as usize,
            )
        };
    };
}

#[cfg(all(target_pointer_width = "64", unix))]
macro_rules! zmemzero__3 {
    ($dest:expr, $len:expr) => {
        unsafe {
            std::ptr::write_bytes(
                (($dest) as *mut ()) as *mut u8,
                0u8,
                (($len) as c_ulong) as usize,
            )
        };
    };
}

unsafe extern "C" {
    static mut z_errmsg: [*mut c_char; 10];
    fn adler32(arg0: c_ulong, arg1: *const u8, arg2: u32) -> c_ulong;
    fn memcpy(arg0: *mut (), arg1: *const (), arg2: size_t) -> *mut ();
    fn memset(arg0: *mut (), arg1: i32, arg2: size_t) -> *mut ();
    fn zcalloc(arg0: *mut (), arg1: u32, arg2: u32) -> *mut ();
    fn zcfree(arg0: *mut (), arg1: *mut ());
    fn _tr_init(arg0: *mut internal_state);
    fn _tr_flush_block(arg0: *mut internal_state, arg1: *mut c_char, arg2: c_ulong, arg3: i32);
    fn _tr_flush_bits(arg0: *mut internal_state);
    fn _tr_align(arg0: *mut internal_state);
    fn _tr_stored_block(arg0: *mut internal_state, arg1: *mut c_char, arg2: c_ulong, arg3: i32);
}

#[cfg(all(target_os = "freebsd", feature = "zlib-debug"))]
unsafe extern "C" {
    static mut __isthreaded: i32;
}

#[cfg(feature = "zlib-debug")]
unsafe extern "C" {
    static mut z_verbose: i32;
    fn memcmp(arg0: *const (), arg1: *const (), arg2: size_t) -> i32;
    fn z_error(arg0: *mut c_char);
    fn _tr_tally(arg0: *mut internal_state, arg1: u32, arg2: u32) -> i32;
}

#[cfg(not(feature = "zlib-debug"))]
unsafe extern "C" {
    static mut _length_code: [u8; 0];
    static mut _dist_code: [u8; 0];
}

#[cfg(not(feature = "no-gzip"))]
unsafe extern "C" {
    fn crc32(arg0: c_ulong, arg1: *const u8, arg2: u32) -> c_ulong;
    fn crc32_z(arg0: c_ulong, arg1: *const u8, arg2: size_t) -> c_ulong;
}

#[cfg(all(not(target_os = "freebsd"), feature = "zlib-debug"))]
unsafe extern "C" {
    fn fflush(_0: *mut slate_support::FILE) -> i32;
}

use core::ffi::c_char;
use core::ffi::c_long;
use core::ffi::c_ulong;
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
struct static_tree_desc_s {}

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

// ALGORITHM
//
//     The "deflation" process depends on being able to identify portions
//     of the input text which are identical to earlier input (within a
//     sliding window trailing behind the input currently being processed).
//
//     The most straightforward technique turns out to be the fastest for
//     most input files: try all possible matches and select the longest.
//     The key feature of this algorithm is that insertions into the string
//     dictionary are very simple and thus fast, and deletions are avoided
//     completely. Insertions are performed at each input character, whereas
//     string matches are performed only when the previous match ends. So it
//     is preferable to spend more time in matches to allow very fast string
//     insertions and avoid deletions. The matching algorithm for small
//     strings is inspired from that of Rabin & Karp. A brute force approach
//     is used to find longer strings when a small match has been found.
//     A similar algorithm is used in comic (by Jan-Mark Wams) and freeze
//     (by Leonid Broukhis).
//        A previous version of this file used a more sophisticated algorithm
//     (by Fiala and Greene) which is guaranteed to run in linear amortized
//     time, but has a larger average cost, uses more memory and is patented.
//     However the F&G algorithm may be faster for some highly redundant
//     files if the parameter max_chain_length (described below) is too large.
//
// ACKNOWLEDGEMENTS
//
//     The idea of lazy evaluation of matches is due to Jan-Mark Wams, and
//     I found it in 'freeze' written by Leonid Broukhis.
//     Thanks to many people for bug reports and testing.
//
// REFERENCES
//
//     Deutsch, L.P.,"DEFLATE Compressed Data Format Specification".
//     Available at https://datatracker.ietf.org/doc/html/rfc1951
//
//     A description of the Rabin and Karp algorithm is given in the book
//        "Algorithms" by R. Sedgewick, Addison-Wesley, p252.
//
//     Fiala,E.R., and Greene,D.H.
//        Data Compression with Finite Windows, Comm.ACM, 32,4 (1989) 490-595
// @(#) $Id$
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_os = "macos",
    windows
))]
#[unsafe(no_mangle)]
static mut deflate_copyright: __SlateAlignLargeArray<[c_char; 70]> = __SlateAlignLargeArray([
    32i8, 100i8, 101i8, 102i8, 108i8, 97i8, 116i8, 101i8, 32i8, 49i8, 46i8, 51i8, 46i8, 50i8, 46i8,
    49i8, 32i8, 67i8, 111i8, 112i8, 121i8, 114i8, 105i8, 103i8, 104i8, 116i8, 32i8, 49i8, 57i8,
    57i8, 53i8, 45i8, 50i8, 48i8, 50i8, 54i8, 32i8, 74i8, 101i8, 97i8, 110i8, 45i8, 108i8, 111i8,
    117i8, 112i8, 32i8, 71i8, 97i8, 105i8, 108i8, 108i8, 121i8, 32i8, 97i8, 110i8, 100i8, 32i8,
    77i8, 97i8, 114i8, 107i8, 32i8, 65i8, 100i8, 108i8, 101i8, 114i8, 32i8, 0i8,
]);

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_os = "macos",
    windows
)))]
#[unsafe(no_mangle)]
static mut deflate_copyright: __SlateAlignLargeArray<[c_char; 70]> = __SlateAlignLargeArray([
    32u8, 100u8, 101u8, 102u8, 108u8, 97u8, 116u8, 101u8, 32u8, 49u8, 46u8, 51u8, 46u8, 50u8, 46u8,
    49u8, 32u8, 67u8, 111u8, 112u8, 121u8, 114u8, 105u8, 103u8, 104u8, 116u8, 32u8, 49u8, 57u8,
    57u8, 53u8, 45u8, 50u8, 48u8, 50u8, 54u8, 32u8, 74u8, 101u8, 97u8, 110u8, 45u8, 108u8, 111u8,
    117u8, 112u8, 32u8, 71u8, 97u8, 105u8, 108u8, 108u8, 121u8, 32u8, 97u8, 110u8, 100u8, 32u8,
    77u8, 97u8, 114u8, 107u8, 32u8, 65u8, 100u8, 108u8, 101u8, 114u8, 32u8, 0u8,
]);

#[cfg(target_arch = "x86_64")]
#[repr(C, align(16))]
struct __SlateAlignLargeArray<T>(T);

#[cfg(not(target_arch = "x86_64"))]
#[repr(C)]
struct __SlateAlignLargeArray<T>(T);

// If you use the zlib library in a product, an acknowledgment is welcome
// in the documentation of your product. If for some reason you cannot
// include such an acknowledgment, I would appreciate that you keep this
// copyright string in the executable of your product.
// Compression function. Returns the block state after the call.
// Local data
// Tail of hash chains
// Matches of length 3 are discarded if their distance exceeds TOO_FAR
/// Values for max_lazy_match, good_match and max_chain_length, depending on
/// the desired pack level (0..9). The values given below have been tuned to
/// exclude worst case performance for pathological files. Better values may be
/// found for specific files.
#[repr(C)]
#[derive(Clone, Copy)]
struct config_s {
    /// reduce lazy search above this match length
    good_length: u16,
    /// do not perform lazy search above this match length
    max_lazy: u16,
    /// quit search above this match length
    nice_length: u16,
    max_chain: u16,
    func: Option<unsafe extern "C-unwind" fn(*mut internal_state, i32) -> u32>,
}

#[cfg(not(feature = "fastest"))]
///      good lazy nice chain
///
/// 0
/// store only
/// 1
/// max speed, no lazy matches
/// 2
/// 3
/// 4
/// lazy matches
/// 5
/// 6
/// 7
/// 8
/// 9
/// max compression
static mut configuration_table: __SlateAlignLargeArray<[config_s; 10]> = __SlateAlignLargeArray([
    config_s {
        good_length: 0u16,
        max_lazy: 0u16,
        nice_length: 0u16,
        max_chain: 0u16,
        func: Some(deflate_stored),
    },
    config_s {
        good_length: 4u16,
        max_lazy: 4u16,
        nice_length: 8u16,
        max_chain: 4u16,
        func: Some(deflate_fast),
    },
    config_s {
        good_length: 4u16,
        max_lazy: 5u16,
        nice_length: 16u16,
        max_chain: 8u16,
        func: Some(deflate_fast),
    },
    config_s {
        good_length: 4u16,
        max_lazy: 6u16,
        nice_length: 32u16,
        max_chain: 32u16,
        func: Some(deflate_fast),
    },
    config_s {
        good_length: 4u16,
        max_lazy: 4u16,
        nice_length: 16u16,
        max_chain: 16u16,
        func: Some(deflate_slow),
    },
    config_s {
        good_length: 8u16,
        max_lazy: 16u16,
        nice_length: 32u16,
        max_chain: 32u16,
        func: Some(deflate_slow),
    },
    config_s {
        good_length: 8u16,
        max_lazy: 16u16,
        nice_length: 128u16,
        max_chain: 128u16,
        func: Some(deflate_slow),
    },
    config_s {
        good_length: 8u16,
        max_lazy: 32u16,
        nice_length: 128u16,
        max_chain: 256u16,
        func: Some(deflate_slow),
    },
    config_s {
        good_length: 32u16,
        max_lazy: 128u16,
        nice_length: 258u16,
        max_chain: 1024u16,
        func: Some(deflate_slow),
    },
    config_s {
        good_length: 32u16,
        max_lazy: 258u16,
        nice_length: 258u16,
        max_chain: 4096u16,
        func: Some(deflate_slow),
    },
]);

#[cfg(feature = "fastest")]
///      good lazy nice chain
///
/// 0
/// store only
/// 1
/// max speed, no lazy matches
static mut configuration_table: __SlateAlignLargeArray<[config_s; 2]> = __SlateAlignLargeArray([
    config_s {
        good_length: 0u16,
        max_lazy: 0u16,
        nice_length: 0u16,
        max_chain: 0u16,
        func: Some(deflate_stored),
    },
    config_s {
        good_length: 4u16,
        max_lazy: 4u16,
        nice_length: 8u16,
        max_chain: 4u16,
        func: Some(deflate_fast),
    },
]);

// Note: the deflate() code requires max_lazy >= MIN_MATCH and max_chain >= 4
// For deflate_fast() (levels <= 3) good is ignored and lazy has a different
// meaning.
// rank Z_BLOCK between Z_NO_FLUSH and Z_PARTIAL_FLUSH
// Update a hash value with the given input byte
// IN  assertion: all calls to UPDATE_HASH are made with consecutive input
//    characters, so that a running hash key can be computed from the previous
//    key instead of complete recalculation each time.
// Insert string str in the dictionary and set match_head to the previous head
// of the hash chain (the most recent string with same hash key). Return
// the previous length of the hash chain.
// If this file is compiled with -DFASTEST, the compression level is forced
// to 1, and no hash chains are maintained.
// IN  assertion: all calls to INSERT_STRING are made with consecutive input
//    characters and the first MIN_MATCH bytes of str are valid (except for
//    the last MIN_MATCH-1 bytes of the input file).
// Initialize the hash table (avoiding 64K overflow for 16 bit systems).
// prev[] will be initialized on the fly.
/// Slide the hash table when sliding the window down (could be avoided with 32
/// bit values at the expense of memory usage). We slide even when level == 0 to
/// keep the hash table consistent if we switch back to level > 0 later.
fn slide_hash(s: *mut internal_state) {
    let mut n: u32 = unsafe { (*s).hash_size };
    let mut m: u32 = 0;
    let mut p: *mut u16 = unsafe { std::mem::zeroed() };
    let wsize: u32 = unsafe { (*s).w_size };
    p = unsafe { unsafe { (*s).head }.add(n as usize) };
    loop {
        let __v0: *mut u16 = p;
        let __v1: *mut u16 = unsafe { __v0.sub(1) };
        p = __v1;
        m = (unsafe { *__v1 }) as u32;
        unsafe {
            *p = (if m >= wsize {
                m.wrapping_sub(wsize)
            } else {
                0u32
            }) as u16;
        }
        let __v4: u32 = n.wrapping_sub(1);
        n = __v4;
        if __v4 == 0 {
            break;
        }
    }
    #[cfg(not(feature = "fastest"))]
    {
        n = wsize;
        p = unsafe { unsafe { (*s).prev }.add(n as usize) };
        loop {
            let __v5: *mut u16 = p;
            let __v6: *mut u16 = unsafe { __v5.sub(1) };
            p = __v6;
            m = (unsafe { *__v6 }) as u32;
            unsafe {
                *p = (if m >= wsize {
                    m.wrapping_sub(wsize)
                } else {
                    0u32
                }) as u16;
            }
            // If n is not on any hash chain, prev[n] is garbage but
            // its value will never be used.
            let __v9: u32 = n.wrapping_sub(1);
            n = __v9;
            if __v9 == 0 {
                break;
            }
        }
    }
    unsafe {
        (*s).slid = 1;
    }
}

/// Read a new buffer from the current input stream, update the adler32
/// and total number of bytes read.  All deflate() input goes through
/// this function so some applications may wish to modify it to avoid
/// allocating a large strm->next_in buffer and copying from it.
/// (See also flush_pending()).
fn read_buf(strm: *mut z_stream_s, buf: *mut u8, size: u32) -> u32 {
    let mut len: u32 = unsafe { (*strm).avail_in };
    if len > size {
        len = size;
    }
    if len == 0 {
        return 0;
    }
    let __v1: u32 = unsafe { (*strm).avail_in };
    unsafe {
        (*strm).avail_in = __v1.wrapping_sub(len);
    }
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*strm).next_in as *const ()) as *const u8,
                buf as *mut u8,
                (len as c_ulong) as usize,
            )
        };
    }
    #[cfg(target_pointer_width = "32")]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*strm).next_in as *const ()) as *const u8,
                buf as *mut u8,
                len as usize,
            )
        };
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*strm).next_in as *const ()) as *const u8,
                buf as *mut u8,
                (len as u64) as usize,
            )
        };
    }
    if (unsafe { (*(*strm).state).wrap }) == 1 {
        unsafe {
            (*strm).adler = adler32((*strm).adler, buf as *const u8, len);
        }
    } else {
        #[cfg(not(feature = "no-gzip"))]
        {
            if (unsafe { (*(*strm).state).wrap }) == 2 {
                unsafe {
                    (*strm).adler = crc32((*strm).adler, buf as *const u8, len);
                }
            }
        }
    }
    let __v4: *mut u8 = unsafe { (*strm).next_in };
    unsafe {
        (*strm).next_in = __v4.add(len as usize);
    }
    let __v7: c_ulong = unsafe { (*strm).total_in };
    unsafe {
        (*strm).total_in = __v7.wrapping_add(len as c_ulong);
    }
    len
}

/// Fill the window when the lookahead becomes insufficient.
/// Updates strstart and lookahead.
///
/// IN assertion: lookahead < MIN_LOOKAHEAD
/// OUT assertions: strstart <= window_size-MIN_LOOKAHEAD
///    At least one byte has been read, or avail_in == 0; reads are
///    performed for at least two bytes (required for the zip translate_eol
///    option -- not supported here).
fn fill_window(s: *mut internal_state) {
    let mut n: u32 = 0;
    let mut more: u32 = 0; // Amount of free space at the end of the window.
    let wsize: u32 = unsafe { (*s).w_size };
    #[cfg(feature = "zlib-debug")]
    {
        Assert!(
            (unsafe { (*s).lookahead }) < 262u32,
            c"already enough lookahead".as_ptr() as *mut c_char
        );
    }
    loop {
        more = unsafe { (*s).window_size }
            .wrapping_sub((unsafe { (*s).lookahead }) as c_ulong)
            .wrapping_sub((unsafe { (*s).strstart }) as c_ulong) as u32;
        // Deal with !@#$% 64K limit:
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            if (std::mem::size_of::<i32>() as c_ulong) <= 2 {
                if more == 0 && (unsafe { (*s).strstart }) == 0 && (unsafe { (*s).lookahead }) == 0
                {
                    more = wsize;
                } else {
                    if more == 4294967295u32 {
                        // Very unlikely, but possible on 16 bit machine if
                        // strstart == 0 && lookahead == 1 (input done a byte at time)
                        more = more.wrapping_sub(1);
                    }
                }
            }
        }
        #[cfg(target_pointer_width = "32")]
        {
            if (std::mem::size_of::<i32>() as u32) <= 2 {
                if more == 0 && (unsafe { (*s).strstart }) == 0 && (unsafe { (*s).lookahead }) == 0
                {
                    more = wsize;
                } else {
                    if more == 4294967295u32 {
                        // Very unlikely, but possible on 16 bit machine if
                        // strstart == 0 && lookahead == 1 (input done a byte at time)
                        more = more.wrapping_sub(1);
                    }
                }
            }
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            if (std::mem::size_of::<i32>() as u64) <= 2 {
                if more == 0 && (unsafe { (*s).strstart }) == 0 && (unsafe { (*s).lookahead }) == 0
                {
                    more = wsize;
                } else {
                    if more == 4294967295u32 {
                        // Very unlikely, but possible on 16 bit machine if
                        // strstart == 0 && lookahead == 1 (input done a byte at time)
                        more = more.wrapping_sub(1);
                    }
                }
            }
        }
        // If the window is almost full and there is insufficient lookahead,
        // move the upper half to the lower one to make room in the upper half.
        if (unsafe { (*s).strstart }) >= wsize.wrapping_add(MAX_DIST!(s)) {
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (unsafe { (*s).window }.add(wsize as usize) as *const ()) as *const u8,
                        ((*s).window as *mut ()) as *mut u8,
                        (wsize.wrapping_sub(more) as c_ulong) as usize,
                    )
                };
            }
            #[cfg(target_pointer_width = "32")]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (unsafe { (*s).window }.add(wsize as usize) as *const ()) as *const u8,
                        ((*s).window as *mut ()) as *mut u8,
                        wsize.wrapping_sub(more) as usize,
                    )
                };
            }
            #[cfg(all(target_pointer_width = "64", windows))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (unsafe { (*s).window }.add(wsize as usize) as *const ()) as *const u8,
                        ((*s).window as *mut ()) as *mut u8,
                        (wsize.wrapping_sub(more) as u64) as usize,
                    )
                };
            }
            let __v3: u32 = unsafe { (*s).match_start };
            unsafe {
                (*s).match_start = __v3.wrapping_sub(wsize);
            }
            let __v6: u32 = unsafe { (*s).strstart };
            unsafe {
                (*s).strstart = __v6.wrapping_sub(wsize);
            }
            // we now have strstart >= MAX_DIST
            unsafe {
                (*s).block_start = (*s).block_start - (wsize as c_long);
            }
            if (unsafe { (*s).insert }) > unsafe { (*s).strstart } {
                unsafe {
                    (*s).insert = (*s).strstart;
                }
            }
            slide_hash(s);
            more = more.wrapping_add(wsize);
        }
        if (unsafe { (*(*s).strm).avail_in }) == 0 {
            break;
        }
        // If there was no sliding:
        //    strstart <= WSIZE+MAX_DIST-1 && lookahead <= MIN_LOOKAHEAD - 1 &&
        //    more == window_size - lookahead - strstart
        // => more >= window_size - (MIN_LOOKAHEAD-1 + WSIZE + MAX_DIST-1)
        // => more >= window_size - 2*WSIZE + 2
        // In the BIG_MEM or MMAP case (not yet supported),
        //   window_size == input_size + MIN_LOOKAHEAD  &&
        //   strstart + s->lookahead <= input_size => more >= MIN_LOOKAHEAD.
        // Otherwise, window_size == 2*WSIZE so more >= 2.
        // If there was sliding, more >= WSIZE. So in all cases, more >= 2.
        #[cfg(feature = "zlib-debug")]
        {
            Assert!(more >= 2, c"more < 2".as_ptr() as *mut c_char);
        }
        n = read_buf(
            unsafe { (*s).strm },
            unsafe {
                unsafe { (*s).window }
                    .offset((*s).strstart as isize)
                    .offset((*s).lookahead as isize)
            },
            more,
        );
        let __v15: u32 = unsafe { (*s).lookahead };
        unsafe {
            (*s).lookahead = __v15.wrapping_add(n);
        }
        // Initialize the hash value now that we have some input:
        if unsafe { (*s).lookahead }.wrapping_add(unsafe { (*s).insert }) >= 3 {
            let mut str: u32 = unsafe { (*s).strstart }.wrapping_sub(unsafe { (*s).insert });
            unsafe {
                (*s).ins_h = *unsafe { (*s).window }.add(str as usize) as u32;
            }
            UPDATE_HASH!(s, (*s).ins_h, unsafe {
                *unsafe { (*s).window }.offset(str.wrapping_add(1) as isize)
            });
            while (unsafe { (*s).insert }) != 0 {
                UPDATE_HASH!(s, (*s).ins_h, unsafe {
                    *unsafe { (*s).window }.offset(str.wrapping_add(3).wrapping_sub(1) as isize)
                });
                #[cfg(not(feature = "fastest"))]
                {
                    unsafe {
                        *unsafe { (*s).prev }.offset((str & (*s).w_mask) as isize) =
                            *unsafe { (*s).head }.offset((*s).ins_h as isize);
                    }
                }
                unsafe {
                    *unsafe { (*s).head }.offset((*s).ins_h as isize) = str as u16;
                }
                str = str.wrapping_add(1);
                let __v20: u32 = unsafe { (*s).insert };
                unsafe {
                    (*s).insert = __v20.wrapping_sub(1);
                }
                if unsafe { (*s).lookahead }.wrapping_add(unsafe { (*s).insert }) < 3 {
                    break;
                }
            }
        }
        // If the whole input has less than MIN_MATCH bytes, ins_h is garbage,
        // but this is not important since only literal bytes will be emitted.
        if !((unsafe { (*s).lookahead }) < 262u32 && (unsafe { (*(*s).strm).avail_in }) != 0) {
            break;
        }
    }
    // If the WIN_INIT bytes after the end of the current data have never been
    // written, then zero those bytes in order to avoid memory check reports of
    // the use of uninitialized (or uninitialised as Julian writes) bytes by
    // the longest match routines.  Update the high water mark for the next
    // time through here.  WIN_INIT is set to MAX_MATCH since the longest match
    // routines allow scanning to strstart + MAX_MATCH, ignoring lookahead.
    if (unsafe { (*s).high_water }) < unsafe { (*s).window_size } {
        let curr: c_ulong = ((unsafe { (*s).strstart }) as c_ulong)
            .wrapping_add((unsafe { (*s).lookahead }) as c_ulong);
        let mut init: c_ulong = 0;
        if (unsafe { (*s).high_water }) < curr {
            // Previous high water mark below current data -- zero WIN_INIT
            // bytes or up to end of window, whichever is less.
            init = unsafe { (*s).window_size }.wrapping_sub(curr);
            if init > 258 {
                init = 258;
            }
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                zmemzero__3!(
                    unsafe { unsafe { (*s).window }.add(curr as usize) },
                    init as u32
                );
            }
            #[cfg(target_pointer_width = "32")]
            {
                zmemzero!(
                    unsafe { unsafe { (*s).window }.add(curr as usize) },
                    init as u32
                );
            }
            #[cfg(all(target_pointer_width = "64", windows))]
            {
                zmemzero__2!(
                    unsafe { unsafe { (*s).window }.add(curr as usize) },
                    init as u32
                );
            }
            unsafe {
                (*s).high_water = curr.wrapping_add(init);
            }
        } else {
            if (unsafe { (*s).high_water }) < curr.wrapping_add(258) {
                // High water mark at or above current data, but below current data
                // plus WIN_INIT -- zero out to current data plus WIN_INIT, or up
                // to end of window, whichever is less.
                init = curr
                    .wrapping_add(258)
                    .wrapping_sub(unsafe { (*s).high_water });
                if init > unsafe { (*s).window_size }.wrapping_sub(unsafe { (*s).high_water }) {
                    init = unsafe { (*s).window_size }.wrapping_sub(unsafe { (*s).high_water });
                }
                #[cfg(all(target_pointer_width = "64", unix))]
                {
                    zmemzero__3!(
                        unsafe { unsafe { (*s).window }.offset((*s).high_water as isize) },
                        init as u32
                    );
                }
                #[cfg(target_pointer_width = "32")]
                {
                    zmemzero!(
                        unsafe { unsafe { (*s).window }.offset((*s).high_water as isize) },
                        init as u32
                    );
                }
                #[cfg(all(target_pointer_width = "64", windows))]
                {
                    zmemzero__2!(
                        unsafe { unsafe { (*s).window }.offset((*s).high_water as isize) },
                        init as u32
                    );
                }
                let __v24: c_ulong = unsafe { (*s).high_water };
                unsafe {
                    (*s).high_water = __v24.wrapping_add(init);
                }
            }
        }
    }
    #[cfg(feature = "zlib-debug")]
    {
        Assert!(
            ((unsafe { (*s).strstart }) as c_ulong)
                <= unsafe { (*s).window_size }.wrapping_sub((262 as c_ulong)),
            c"not enough room for search".as_ptr() as *mut c_char
        );
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflateInit_(
    strm: *mut z_stream_s,
    level: i32,
    version: *const c_char,
    stream_size: i32,
) -> i32 {
    return deflateInit2_(strm, level, 8i32, 15i32, 8i32, 0i32, version, stream_size);
    // To do: ignore strm->next_in if we use it as window
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflateInit2_(
    strm: *mut z_stream_s,
    mut level: i32,
    method: i32,
    mut windowBits: i32,
    memLevel: i32,
    strategy: i32,
    version: *const c_char,
    stream_size: i32,
) -> i32 {
    let mut wrap: i32 = 1;
    #[cfg(any(all(target_arch = "x86_64", unix), target_os = "macos"))]
    {
        if version.is_null()
            || ((unsafe { *version.add(0) }) as i32) != ((unsafe { my_version[0] }) as i32)
            || (stream_size as c_ulong) != (std::mem::size_of::<z_stream_s>() as c_ulong)
        {
            return -6;
        }
    }
    #[cfg(any(target_arch = "x86", all(target_arch = "arm", windows)))]
    {
        if version.is_null()
            || ((unsafe { *version.add(0) }) as i32) != ((unsafe { my_version[0] }) as i32)
            || (stream_size as u32) != (std::mem::size_of::<z_stream_s>() as u32)
        {
            return -6;
        }
    }
    #[cfg(all(not(target_os = "macos"), target_arch = "aarch64", unix))]
    {
        if version.is_null()
            || (((unsafe { *version.add(0) }) as u32) as i32)
                != (((unsafe { my_version[0] }) as u32) as i32)
            || (stream_size as c_ulong) != (std::mem::size_of::<z_stream_s>() as c_ulong)
        {
            return -6;
        }
    }
    #[cfg(all(target_arch = "arm", unix))]
    {
        if version.is_null()
            || (((unsafe { *version.add(0) }) as u32) as i32)
                != (((unsafe { my_version[0] }) as u32) as i32)
            || (stream_size as u32) != (std::mem::size_of::<z_stream_s>() as u32)
        {
            return -6;
        }
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        if version.is_null()
            || ((unsafe { *version.add(0) }) as i32) != ((unsafe { my_version[0] }) as i32)
            || ((stream_size as i64) as u64) != (std::mem::size_of::<z_stream_s>() as u64)
        {
            return -6;
        }
    }
    if strm.is_null() {
        return -2;
    }
    unsafe {
        (*strm).msg = std::ptr::null_mut::<c_char>();
    }
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
    #[cfg(not(feature = "fastest"))]
    {
        if level == -1 {
            level = 6;
        }
    }
    #[cfg(feature = "fastest")]
    {
        if level != 0 {
            level = 1;
        }
    }
    if windowBits < 0 {
        // suppress zlib wrapper
        wrap = 0;
        if windowBits < -15 {
            return -2;
        }
        windowBits = -windowBits;
    } else {
        #[cfg(not(feature = "no-gzip"))]
        {
            if windowBits > 15 {
                wrap = 2; // write gzip wrapper instead
                windowBits -= 16;
            }
        }
    }
    if memLevel < 1
        || memLevel > 9
        || method != 8
        || windowBits < 8
        || windowBits > 15
        || level < 0
        || level > 9
        || strategy < 0
        || strategy > 4
        || windowBits == 8 && wrap != 1
    {
        return -2;
    }
    if windowBits == 8 {
        windowBits = 9;
    }
    // until 256-byte window bug fixed
    #[cfg(all(target_pointer_width = "64", unix))]
    let s: *mut internal_state =
        ZALLOC__2!(strm, 1i32, std::mem::size_of::<internal_state>() as c_ulong)
            as *mut internal_state;
    #[cfg(target_pointer_width = "32")]
    let s: *mut internal_state =
        ZALLOC__4!(strm, 1i32, std::mem::size_of::<internal_state>() as u32) as *mut internal_state;
    #[cfg(all(target_pointer_width = "64", windows))]
    let s: *mut internal_state =
        ZALLOC__2!(strm, 1i32, std::mem::size_of::<internal_state>() as u64) as *mut internal_state;
    if s.is_null() {
        return -4;
    }
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        zmemzero!(s, std::mem::size_of::<internal_state>() as c_ulong);
    }
    #[cfg(target_pointer_width = "32")]
    {
        zmemzero!(s, std::mem::size_of::<internal_state>() as u32);
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        zmemzero!(s, std::mem::size_of::<internal_state>() as u64);
    }
    unsafe {
        (*strm).state = s;
    }
    unsafe {
        (*s).strm = strm;
    }
    unsafe {
        (*s).status = 42;
    }
    // to pass state test in deflateReset()
    unsafe {
        (*s).wrap = wrap;
    }
    unsafe {
        (*s).gzhead = std::ptr::null_mut::<gz_header_s>();
    }
    unsafe {
        (*s).w_bits = windowBits as u32;
    }
    unsafe {
        (*s).w_size = (1i32 << (*s).w_bits) as u32;
    }
    unsafe {
        (*s).w_mask = unsafe { (*s).w_size }.wrapping_sub(1);
    }
    unsafe {
        (*s).hash_bits = (memLevel as u32).wrapping_add(7);
    }
    unsafe {
        (*s).hash_size = (1i32 << (*s).hash_bits) as u32;
    }
    unsafe {
        (*s).hash_mask = unsafe { (*s).hash_size }.wrapping_sub(1);
    }
    unsafe {
        (*s).hash_shift = unsafe { (*s).hash_bits }.wrapping_add(3).wrapping_sub(1) / 3;
    }
    unsafe {
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            (*s).window = ZALLOC!(
                strm,
                unsafe { (*s).w_size },
                (2 as c_ulong).wrapping_mul(std::mem::size_of::<u8>() as c_ulong)
            ) as *mut u8;
        }
        #[cfg(target_pointer_width = "32")]
        {
            (*s).window = ZALLOC__3!(
                strm,
                unsafe { (*s).w_size },
                2u32.wrapping_mul(std::mem::size_of::<u8>() as u32)
            ) as *mut u8;
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            (*s).window = ZALLOC!(
                strm,
                unsafe { (*s).w_size },
                2u64.wrapping_mul(std::mem::size_of::<u8>() as u64)
            ) as *mut u8;
        }
    }
    unsafe {
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            (*s).prev = ZALLOC!(
                strm,
                unsafe { (*s).w_size },
                std::mem::size_of::<u16>() as c_ulong
            ) as *mut u16;
        }
        #[cfg(target_pointer_width = "32")]
        {
            (*s).prev = ZALLOC__3!(
                strm,
                unsafe { (*s).w_size },
                std::mem::size_of::<u16>() as u32
            ) as *mut u16;
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            (*s).prev = ZALLOC!(
                strm,
                unsafe { (*s).w_size },
                std::mem::size_of::<u16>() as u64
            ) as *mut u16;
        }
    }
    unsafe {
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            (*s).head = ZALLOC!(
                strm,
                unsafe { (*s).hash_size },
                std::mem::size_of::<u16>() as c_ulong
            ) as *mut u16;
        }
        #[cfg(target_pointer_width = "32")]
        {
            (*s).head = ZALLOC__3!(
                strm,
                unsafe { (*s).hash_size },
                std::mem::size_of::<u16>() as u32
            ) as *mut u16;
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            (*s).head = ZALLOC!(
                strm,
                unsafe { (*s).hash_size },
                std::mem::size_of::<u16>() as u64
            ) as *mut u16;
        }
    }
    unsafe {
        (*s).high_water = 0;
    }
    // nothing written to s->window yet
    unsafe {
        (*s).lit_bufsize = (1i32 << memLevel + 6) as u32;
    }
    // 16K elements by default
    // We overlay pending_buf and sym_buf. This works since the average size
    // for length/distance pairs over any compressed block is assured to be 31
    // bits or less.
    //
    // Analysis: The longest fixed codes are a length code of 8 bits plus 5
    // extra bits, for lengths 131 to 257. The longest fixed distance codes are
    // 5 bits plus 13 extra bits, for distances 16385 to 32768. The longest
    // possible fixed-codes length/distance pair is then 31 bits total.
    //
    // sym_buf starts one-fourth of the way into pending_buf. So there are
    // three bytes in sym_buf for every four bytes in pending_buf. Each symbol
    // in sym_buf is three bytes -- two for the distance and one for the
    // literal/length. As each symbol is consumed, the pointer to the next
    // sym_buf value to read moves forward three bytes. From that symbol, up to
    // 31 bits are written to pending_buf. The closest the written pending_buf
    // bits gets to the next sym_buf symbol to read is just before the last
    // code is written. At that time, 31*(n - 2) bits have been written, just
    // after 24*(n - 2) bits have been consumed from sym_buf. sym_buf starts at
    // 8*n bits into pending_buf. (Note that the symbol buffer fills when n - 1
    // symbols are written.) The closest the writing gets to what is unread is
    // then n + 14 bits. Here n is lit_bufsize, which is 16384 by default, and
    // can range from 128 to 32768.
    //
    // Therefore, at a minimum, there are 142 bits of space between what is
    // written and what is read in the overlain buffers, so the symbols cannot
    // be overwritten by the compressed data. That space is actually 139 bits,
    // due to the three-bit fixed-code block header.
    //
    // That covers the case where either Z_FIXED is specified, forcing fixed
    // codes, or when the use of fixed codes is chosen, because that choice
    // results in a smaller compressed block than dynamic codes. That latter
    // condition then assures that the above analysis also covers all dynamic
    // blocks. A dynamic-code block will only be chosen to be emitted if it has
    // fewer bits than a fixed-code block would for the same set of symbols.
    // Therefore its average symbol length is assured to be less than 31. So
    // the compressed data for a dynamic block also cannot overwrite the
    // symbols from which it is being constructed.
    unsafe {
        #[cfg(feature = "lit-mem")]
        {
            (*s).pending_buf = ZALLOC!(strm, unsafe { (*s).lit_bufsize }, 5i32) as *mut u8;
        }
        #[cfg(not(feature = "lit-mem"))]
        {
            (*s).pending_buf = ZALLOC!(strm, unsafe { (*s).lit_bufsize }, 4i32) as *mut u8;
        }
    }
    unsafe {
        (*s).pending_buf_size = ((*s).lit_bufsize as c_ulong).wrapping_mul(4);
    }
    if unsafe { (*s).window }.is_null()
        || unsafe { (*s).prev }.is_null()
        || unsafe { (*s).head }.is_null()
        || unsafe { (*s).pending_buf }.is_null()
    {
        unsafe {
            (*s).status = 666;
        }
        unsafe {
            (*strm).msg = ERR_MSG!(-4i32);
        }
        deflateEnd(strm);
        return -4;
    }
    unsafe {
        #[cfg(feature = "lit-mem")]
        {
            (*s).d_buf =
                unsafe { (*s).pending_buf }.offset(((*s).lit_bufsize << 1i32) as isize) as *mut u16;
        }
        #[cfg(not(feature = "lit-mem"))]
        {
            (*s).sym_buf = unsafe { (*s).pending_buf }.offset((*s).lit_bufsize as isize);
        }
    }
    unsafe {
        #[cfg(feature = "lit-mem")]
        {
            (*s).l_buf = unsafe { (*s).pending_buf }.offset(((*s).lit_bufsize << 2i32) as isize);
        }
        #[cfg(not(feature = "lit-mem"))]
        {
            (*s).sym_end = unsafe { (*s).lit_bufsize }.wrapping_sub(1).wrapping_mul(3);
        }
    }
    #[cfg(feature = "lit-mem")]
    {
        unsafe {
            (*s).sym_end = unsafe { (*s).lit_bufsize }.wrapping_sub(1);
        }
    }
    // We avoid equality with lit_bufsize*3 because of wraparound at 64K
    // on 16 bit machines and because stored blocks are restricted to
    // 64K-1 bytes.
    unsafe {
        (*s).level = level;
    }
    unsafe {
        (*s).strategy = strategy;
    }
    unsafe {
        (*s).method = (method as i8) as u8;
    }
    deflateReset(strm)
}

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_os = "macos",
    windows
))]
static mut my_version: [c_char; 15] = [
    49i8, 46i8, 51i8, 46i8, 50i8, 46i8, 49i8, 45i8, 109i8, 111i8, 116i8, 108i8, 101i8, 121i8, 0i8,
];

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_os = "macos",
    windows
)))]
static mut my_version: [c_char; 15] = [
    49u8, 46u8, 51u8, 46u8, 50u8, 46u8, 49u8, 45u8, 109u8, 111u8, 116u8, 108u8, 101u8, 121u8, 0u8,
];

/// Check for a valid deflate stream state. Return 0 if ok, 1 if not.
fn deflateStateCheck(strm: *mut z_stream_s) -> i32 {
    if strm.is_null() || (unsafe { (*strm).zalloc }) == None || (unsafe { (*strm).zfree }) == None {
        return 1;
    }
    let s: *mut internal_state = unsafe { (*strm).state };
    #[cfg(not(feature = "no-gzip"))]
    {
        if s.is_null()
            || (unsafe { (*s).strm }) != strm
            || (unsafe { (*s).status }) != 42
                && (unsafe { (*s).status }) != 57
                && (unsafe { (*s).status }) != 69
                && (unsafe { (*s).status }) != 73
                && (unsafe { (*s).status }) != 91
                && (unsafe { (*s).status }) != 103
                && (unsafe { (*s).status }) != 113
                && (unsafe { (*s).status }) != 666
        {
            return 1;
        }
    }
    #[cfg(feature = "no-gzip")]
    {
        if s.is_null()
            || (unsafe { (*s).strm }) != strm
            || (unsafe { (*s).status }) != 42
                && (unsafe { (*s).status }) != 69
                && (unsafe { (*s).status }) != 73
                && (unsafe { (*s).status }) != 91
                && (unsafe { (*s).status }) != 103
                && (unsafe { (*s).status }) != 113
                && (unsafe { (*s).status }) != 666
        {
            return 1;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflateSetDictionary(
    strm: *mut z_stream_s,
    mut dictionary: *const u8,
    mut dictLength: u32,
) -> i32 {
    let mut s: *mut internal_state = unsafe { std::mem::zeroed() };
    let mut str: u32 = 0;
    let mut n: u32 = 0;
    let mut wrap: i32 = 0;
    let mut avail: u32 = 0;
    if deflateStateCheck(strm) != 0 || dictionary.is_null() {
        return -2;
    }
    s = unsafe { (*strm).state };
    wrap = unsafe { (*s).wrap };
    if wrap == 2 || wrap == 1 && (unsafe { (*s).status }) != 42 || (unsafe { (*s).lookahead }) != 0
    {
        return -2;
    }
    // when using zlib wrappers, compute Adler-32 for provided dictionary
    if wrap == 1 {
        unsafe {
            (*strm).adler = adler32((*strm).adler, dictionary, dictLength);
        }
    }
    unsafe {
        (*s).wrap = 0;
    }
    // avoid computing Adler-32 in read_buf
    // if dictionary would fill window, just replace the history
    if dictLength >= unsafe { (*s).w_size } {
        if wrap == 0 {
            // already empty otherwise
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                CLEAR_HASH!(s);
            }
            #[cfg(target_pointer_width = "32")]
            {
                CLEAR_HASH__3!(s);
            }
            #[cfg(all(target_pointer_width = "64", windows))]
            {
                CLEAR_HASH__2!(s);
            }
            unsafe {
                (*s).strstart = 0;
            }
            unsafe {
                (*s).block_start = 0;
            }
            unsafe {
                (*s).insert = 0;
            }
        }
        let __v1: *const u8 = dictionary;
        dictionary = unsafe { __v1.offset(dictLength.wrapping_sub((*s).w_size) as isize) }; // use the tail
        dictLength = unsafe { (*s).w_size };
    }
    // insert dictionary into window and hash
    avail = unsafe { (*strm).avail_in };
    let next: *mut u8 = unsafe { (*strm).next_in };
    unsafe {
        (*strm).avail_in = dictLength;
    }
    unsafe {
        (*strm).next_in = dictionary as *mut u8;
    }
    fill_window(s);
    while (unsafe { (*s).lookahead }) >= 3 {
        str = unsafe { (*s).strstart };
        n = unsafe { (*s).lookahead }.wrapping_sub(2u32);
        loop {
            UPDATE_HASH!(s, (*s).ins_h, unsafe {
                *unsafe { (*s).window }.offset(str.wrapping_add(3).wrapping_sub(1) as isize)
            });
            #[cfg(not(feature = "fastest"))]
            {
                unsafe {
                    *unsafe { (*s).prev }.offset((str & (*s).w_mask) as isize) =
                        *unsafe { (*s).head }.offset((*s).ins_h as isize);
                }
            }
            unsafe {
                *unsafe { (*s).head }.offset((*s).ins_h as isize) = str as u16;
            }
            str = str.wrapping_add(1);
            let __v7: u32 = n.wrapping_sub(1);
            n = __v7;
            if __v7 == 0 {
                break;
            }
        }
        unsafe {
            (*s).strstart = str;
        }
        unsafe {
            (*s).lookahead = 2u32;
        }
        fill_window(s);
    }
    let __v10: u32 = unsafe { (*s).strstart };
    unsafe {
        (*s).strstart = __v10.wrapping_add((*s).lookahead);
    }
    unsafe {
        (*s).block_start = (*s).strstart as c_long;
    }
    unsafe {
        (*s).insert = (*s).lookahead;
    }
    unsafe {
        (*s).lookahead = 0;
    }
    let __v12: u32 = 2u32;
    unsafe {
        (*s).prev_length = __v12;
    }
    unsafe {
        (*s).match_length = __v12;
    }
    unsafe {
        (*s).match_available = 0;
    }
    unsafe {
        (*strm).next_in = next;
    }
    unsafe {
        (*strm).avail_in = avail;
    }
    unsafe {
        (*s).wrap = wrap;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflateGetDictionary(
    strm: *mut z_stream_s,
    dictionary: *mut u8,
    dictLength: *mut u32,
) -> i32 {
    let mut len: u32 = 0;
    if deflateStateCheck(strm) != 0 {
        return -2;
    }
    let s: *mut internal_state = unsafe { (*strm).state };
    len = unsafe { (*s).strstart }.wrapping_add(unsafe { (*s).lookahead });
    if len > unsafe { (*s).w_size } {
        len = unsafe { (*s).w_size };
    }
    if !dictionary.is_null() && len != 0 {
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    (unsafe { (*s).window }
                        .offset((*s).strstart as isize)
                        .offset((*s).lookahead as isize)
                        .sub(len as usize) as *const ()) as *const u8,
                    dictionary as *mut u8,
                    (len as c_ulong) as usize,
                )
            };
        }
        #[cfg(target_pointer_width = "32")]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    (unsafe { (*s).window }
                        .offset((*s).strstart as isize)
                        .offset((*s).lookahead as isize)
                        .sub(len as usize) as *const ()) as *const u8,
                    dictionary as *mut u8,
                    len as usize,
                )
            };
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    (unsafe { (*s).window }
                        .offset((*s).strstart as isize)
                        .offset((*s).lookahead as isize)
                        .sub(len as usize) as *const ()) as *const u8,
                    dictionary as *mut u8,
                    (len as u64) as usize,
                )
            };
        }
    }
    if !dictLength.is_null() {
        unsafe {
            *dictLength = len;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflateResetKeep(strm: *mut z_stream_s) -> i32 {
    if deflateStateCheck(strm) != 0 {
        return -2;
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
    // use zfree if we ever allocate msg dynamically
    unsafe {
        (*strm).data_type = 2;
    }
    let s: *mut internal_state = unsafe { (*strm).state };
    unsafe {
        (*s).pending = 0;
    }
    unsafe {
        (*s).pending_out = (*s).pending_buf;
    }
    if (unsafe { (*s).wrap }) < 0 {
        unsafe {
            (*s).wrap = -(*s).wrap;
        }
        // was made negative by deflate(..., Z_FINISH);
    }
    unsafe {
        #[cfg(not(feature = "no-gzip"))]
        {
            (*s).status = if (*s).wrap == 2 { 57 } else { 42 };
        }
        #[cfg(feature = "no-gzip")]
        {
            (*s).status = 42;
        }
    }
    #[cfg(all(target_pointer_width = "64", unix, not(feature = "no-gzip")))]
    let __v0: u64 = if (unsafe { (*s).wrap }) == 2 {
        unsafe { crc32((0 as c_ulong), std::ptr::null::<u8>(), 0u32) }
    } else {
        unsafe { adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32) }
    };
    #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "no-gzip")))]
    let __v0: u32 = if (unsafe { (*s).wrap }) == 2 {
        unsafe { crc32((0 as c_ulong), std::ptr::null::<u8>(), 0u32) }
    } else {
        unsafe { adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32) }
    };
    unsafe {
        #[cfg(not(feature = "no-gzip"))]
        {
            (*strm).adler = __v0;
        }
        #[cfg(feature = "no-gzip")]
        {
            (*strm).adler = adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
        }
    }
    unsafe {
        (*s).last_flush = -2;
    }
    unsafe { _tr_init(s) };
    0
}

/// Initialize the "longest match" routines for a new zlib stream
fn lm_init(s: *mut internal_state) {
    unsafe {
        (*s).window_size = (2 as c_ulong).wrapping_mul((*s).w_size as c_ulong);
    }
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        CLEAR_HASH!(s);
    }
    #[cfg(target_pointer_width = "32")]
    {
        CLEAR_HASH__3!(s);
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        CLEAR_HASH__2!(s);
    }
    // Set the default configuration parameters:
    unsafe {
        (*s).max_lazy_match = configuration_table.0[(*s).level as usize].max_lazy as u32;
    }
    unsafe {
        (*s).good_match = configuration_table.0[(*s).level as usize].good_length as u32;
    }
    unsafe {
        (*s).nice_match = (configuration_table.0[(*s).level as usize].nice_length as u32) as i32;
    }
    unsafe {
        (*s).max_chain_length = configuration_table.0[(*s).level as usize].max_chain as u32;
    }
    unsafe {
        (*s).strstart = 0;
    }
    unsafe {
        (*s).block_start = 0;
    }
    unsafe {
        (*s).lookahead = 0;
    }
    unsafe {
        (*s).insert = 0;
    }
    let __v1: u32 = 2u32;
    unsafe {
        (*s).prev_length = __v1;
    }
    unsafe {
        (*s).match_length = __v1;
    }
    unsafe {
        (*s).match_available = 0;
    }
    unsafe {
        (*s).ins_h = 0;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflateReset(strm: *mut z_stream_s) -> i32 {
    let ret: i32 = deflateResetKeep(strm);
    if ret == 0 {
        lm_init(unsafe { (*strm).state });
    }
    ret
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflateSetHeader(strm: *mut z_stream_s, head: *mut gz_header_s) -> i32 {
    if deflateStateCheck(strm) != 0 || (unsafe { (*(*strm).state).wrap }) != 2 {
        return -2;
    }
    unsafe {
        (*(*strm).state).gzhead = head;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflatePending(
    strm: *mut z_stream_s,
    pending: *mut u32,
    bits: *mut i32,
) -> i32 {
    if deflateStateCheck(strm) != 0 {
        return -2;
    }
    if !bits.is_null() {
        unsafe {
            *bits = (*(*strm).state).bi_valid;
        }
    }
    if !pending.is_null() {
        unsafe {
            *pending = (*(*strm).state).pending as u32;
        }
        if ((unsafe { *pending }) as c_ulong) != unsafe { (*(*strm).state).pending } {
            unsafe {
                *pending = 4294967295u32;
            }
            return -5;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflateUsed(strm: *mut z_stream_s, bits: *mut i32) -> i32 {
    if deflateStateCheck(strm) != 0 {
        return -2;
    }
    if !bits.is_null() {
        unsafe {
            *bits = (*(*strm).state).bi_used;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflatePrime(strm: *mut z_stream_s, mut bits: i32, mut value: i32) -> i32 {
    let mut put: i32 = 0;
    if deflateStateCheck(strm) != 0 {
        return -2;
    }
    let s: *mut internal_state = unsafe { (*strm).state };
    #[cfg(not(feature = "lit-mem"))]
    {
        if bits < 0
            || bits > 16
            || (unsafe { (*s).sym_buf })
                < unsafe { unsafe { (*s).pending_out }.offset(2i32 as isize) }
        {
            return -5;
        }
    }
    #[cfg(feature = "lit-mem")]
    {
        if bits < 0
            || bits > 16
            || ((unsafe { (*s).d_buf }) as *mut u8)
                < unsafe { unsafe { (*s).pending_out }.offset(2i32 as isize) }
        {
            return -5;
        }
    }
    loop {
        put = 16 - unsafe { (*s).bi_valid };
        if put > bits {
            put = bits;
        }
        unsafe {
            (*s).bi_buf = (((((*s).bi_buf as u32) as i32)
                | ((((((value & (1i32 << put) - 1) << (*s).bi_valid) as i16) as u16) as u32)
                    as i32)) as i16) as u16;
        }
        unsafe {
            (*s).bi_valid = (*s).bi_valid + put;
        }
        unsafe { _tr_flush_bits(s) };
        value >>= put;
        bits -= put;
        if bits == 0 {
            break;
        }
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflateParams(strm: *mut z_stream_s, mut level: i32, strategy: i32) -> i32 {
    if deflateStateCheck(strm) != 0 {
        return -2;
    }
    let s: *mut internal_state = unsafe { (*strm).state };
    #[cfg(not(feature = "fastest"))]
    {
        if level == -1 {
            level = 6;
        }
    }
    #[cfg(feature = "fastest")]
    {
        if level != 0 {
            level = 1;
        }
    }
    if level < 0 || level > 9 || strategy < 0 || strategy > 4 {
        return -2;
    }
    let func: Option<unsafe extern "C-unwind" fn(*mut internal_state, i32) -> u32> =
        unsafe { configuration_table.0[(*s).level as usize].func };
    if (strategy != unsafe { (*s).strategy }
        || func != unsafe { configuration_table.0[level as usize].func })
        && (unsafe { (*s).last_flush }) != -2
    {
        // Flush the last buffer:
        let err: i32 = deflate(strm, 5i32);
        if err == -2 {
            return err;
        }
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            if (unsafe { (*strm).avail_in }) != 0
                || ((unsafe { (*s).strstart }) as c_long) - unsafe { (*s).block_start }
                    + ((unsafe { (*s).lookahead }) as c_long)
                    != 0
            {
                return -5;
            }
        }
        #[cfg(any(target_pointer_width = "32", windows))]
        {
            if (unsafe { (*strm).avail_in }) != 0
                || ((unsafe { (*s).strstart }) as c_ulong)
                    .wrapping_sub((unsafe { (*s).block_start }) as c_ulong)
                    .wrapping_add((unsafe { (*s).lookahead }) as c_ulong)
                    != 0
            {
                return -5;
            }
        }
    }
    if (unsafe { (*s).level }) != level {
        if (unsafe { (*s).level }) == 0 && (unsafe { (*s).matches }) != 0 {
            if (unsafe { (*s).matches }) == 1 {
                slide_hash(s);
            } else {
                #[cfg(all(target_pointer_width = "64", unix))]
                {
                    CLEAR_HASH!(s);
                }
                #[cfg(target_pointer_width = "32")]
                {
                    CLEAR_HASH__3!(s);
                }
                #[cfg(all(target_pointer_width = "64", windows))]
                {
                    CLEAR_HASH__2!(s);
                }
            }
            unsafe {
                (*s).matches = 0;
            }
        }
        unsafe {
            (*s).level = level;
        }
        unsafe {
            (*s).max_lazy_match = configuration_table.0[level as usize].max_lazy as u32;
        }
        unsafe {
            (*s).good_match = configuration_table.0[level as usize].good_length as u32;
        }
        unsafe {
            (*s).nice_match = (configuration_table.0[level as usize].nice_length as u32) as i32;
        }
        unsafe {
            (*s).max_chain_length = configuration_table.0[level as usize].max_chain as u32;
        }
    }
    unsafe {
        (*s).strategy = strategy;
    }
    0
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflateTune(
    strm: *mut z_stream_s,
    good_length: i32,
    max_lazy: i32,
    nice_length: i32,
    max_chain: i32,
) -> i32 {
    if deflateStateCheck(strm) != 0 {
        return -2;
    }
    let s: *mut internal_state = unsafe { (*strm).state };
    unsafe {
        (*s).good_match = good_length as u32;
    }
    unsafe {
        (*s).max_lazy_match = max_lazy as u32;
    }
    unsafe {
        (*s).nice_match = nice_length;
    }
    unsafe {
        (*s).max_chain_length = max_chain as u32;
    }
    0
}

/// For the default windowBits of 15 and memLevel of 8, this function returns a
/// close to exact, as well as small, upper bound on the compressed size. This
/// is an expansion of ~0.03%, plus a small constant.
///
/// For any setting other than those defaults for windowBits and memLevel, one
/// of two worst case bounds is returned. This is at most an expansion of ~4% or
/// ~13%, plus a small constant.
///
/// Both the 0.03% and 4% derive from the overhead of stored blocks. The first
/// one is for stored blocks of 16383 bytes (memLevel == 8), whereas the second
/// is for stored blocks of 127 bytes (the worst case memLevel == 1). The
/// expansion results from five bytes of header for each stored block.
///
/// The larger expansion of 13% results from a window size less than or equal to
/// the symbols buffer size (windowBits <= memLevel + 7). In that case some of
/// the data being compressed may have slid out of the sliding window, impeding
/// a stored block from being emitted. Then the only choice is a fixed or
/// dynamic block, where a fixed block limits the maximum expansion to 9 bits
/// per 8-bit byte, plus 10 bits for every block. The smallest block size for
/// which this can occur is 255 (memLevel == 2).
///
/// Shifts are used to approximate divisions, for speed.
#[unsafe(no_mangle)]
extern "C-unwind" fn deflateBound_z(strm: *mut z_stream_s, sourceLen: size_t) -> size_t {
    let mut fixedlen: size_t = 0;
    let mut storelen: size_t = 0;
    let mut wraplen: size_t = 0;
    let mut bound: size_t = 0;
    // upper bound for fixed blocks with 9-bit literals and length 255
    // (memLevel == 2, which is the lowest that may not use stored blocks) --
    // ~13% overhead plus a small constant
    #[cfg(target_pointer_width = "64")]
    {
        fixedlen = sourceLen
            .wrapping_add(sourceLen >> 3i32)
            .wrapping_add(sourceLen >> 8i32)
            .wrapping_add(sourceLen >> 9i32)
            .wrapping_add(4);
        if fixedlen < sourceLen {
            fixedlen = (18446744073709551615 as size_t);
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        fixedlen = (sourceLen as u32)
            .wrapping_add((sourceLen as u32) >> 3i32)
            .wrapping_add((sourceLen as u32) >> 8i32)
            .wrapping_add((sourceLen as u32) >> 9i32)
            .wrapping_add(4) as size_t;
        if (fixedlen as u32) < (sourceLen as u32) {
            fixedlen = (4294967295 as size_t);
        }
    }
    // upper bound for stored blocks with length 127 (memLevel == 1) --
    // ~4% overhead plus a small constant
    #[cfg(target_pointer_width = "64")]
    {
        storelen = sourceLen
            .wrapping_add(sourceLen >> 5i32)
            .wrapping_add(sourceLen >> 7i32)
            .wrapping_add(sourceLen >> 11i32)
            .wrapping_add(7);
        if storelen < sourceLen {
            storelen = (18446744073709551615 as size_t);
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        storelen = (sourceLen as u32)
            .wrapping_add((sourceLen as u32) >> 5i32)
            .wrapping_add((sourceLen as u32) >> 7i32)
            .wrapping_add((sourceLen as u32) >> 11i32)
            .wrapping_add(7) as size_t;
        if (storelen as u32) < (sourceLen as u32) {
            storelen = (4294967295 as size_t);
        }
    }
    // if can't get parameters, return larger bound plus a wrapper
    if deflateStateCheck(strm) != 0 {
        #[cfg(target_pointer_width = "64")]
        {
            bound = if fixedlen > storelen {
                fixedlen
            } else {
                storelen
            };
            return if bound.wrapping_add(18) < bound {
                (18446744073709551615 as size_t)
            } else {
                bound.wrapping_add(18)
            };
        }
        #[cfg(target_pointer_width = "32")]
        {
            bound = (if (fixedlen as u32) > (storelen as u32) {
                fixedlen as u32
            } else {
                storelen as u32
            }) as size_t;
            return (if (bound as u32).wrapping_add(18) < (bound as u32) {
                4294967295u32
            } else {
                (bound as u32).wrapping_add(18)
            }) as size_t;
        }
    }
    // compute wrapper length
    let s: *mut internal_state = unsafe { (*strm).state };
    #[cfg(not(feature = "no-gzip"))]
    {
        match if (unsafe { (*s).wrap }) < 0 {
            -unsafe { (*s).wrap }
        } else {
            unsafe { (*s).wrap }
        } {
            0 => {
                wraplen = 0; // raw deflate
            }
            1 => {
                wraplen = (6 + if (unsafe { (*s).strstart }) != 0 {
                    4i32
                } else {
                    0i32
                }) as size_t; // zlib wrapper
            }
            2 => {
                wraplen = 18; // gzip wrapper
                if !unsafe { (*s).gzhead }.is_null() {
                    // user-supplied gzip header
                    let mut str: *mut u8 = unsafe { std::mem::zeroed() };
                    if !unsafe { (*(*s).gzhead).extra }.is_null() {
                        #[cfg(target_pointer_width = "64")]
                        {
                            wraplen = wraplen.wrapping_add(
                                2u32.wrapping_add(unsafe { (*(*s).gzhead).extra_len }) as size_t,
                            );
                        }
                        #[cfg(target_pointer_width = "32")]
                        {
                            wraplen = (wraplen as u32).wrapping_add(
                                2u32.wrapping_add(unsafe { (*(*s).gzhead).extra_len }),
                            ) as size_t;
                        }
                    }
                    str = unsafe { (*(*s).gzhead).name };
                    if !str.is_null() {
                        loop {
                            #[cfg(target_pointer_width = "64")]
                            {
                                wraplen = wraplen.wrapping_add(1);
                            }
                            #[cfg(target_pointer_width = "32")]
                            {
                                wraplen = (wraplen as u32).wrapping_add(1) as size_t;
                            }
                            let __v6: *mut u8 = str;
                            str = unsafe { __v6.add(1) };
                            if (unsafe { *__v6 }) == 0 {
                                break;
                            }
                        }
                    }
                    str = unsafe { (*(*s).gzhead).comment };
                    if !str.is_null() {
                        loop {
                            #[cfg(target_pointer_width = "64")]
                            {
                                wraplen = wraplen.wrapping_add(1);
                            }
                            #[cfg(target_pointer_width = "32")]
                            {
                                wraplen = (wraplen as u32).wrapping_add(1) as size_t;
                            }
                            let __v11: *mut u8 = str;
                            str = unsafe { __v11.add(1) };
                            if (unsafe { *__v11 }) == 0 {
                                break;
                            }
                        }
                    }
                    if (unsafe { (*(*s).gzhead).hcrc }) != 0 {
                        #[cfg(target_pointer_width = "64")]
                        {
                            wraplen = wraplen.wrapping_add(2);
                        }
                        #[cfg(target_pointer_width = "32")]
                        {
                            wraplen = (wraplen as u32).wrapping_add(2) as size_t;
                        }
                    }
                }
            }
            _ => {
                wraplen = 18; // for compiler happiness
            }
        }
    }
    #[cfg(feature = "no-gzip")]
    {
        match if (unsafe { (*s).wrap }) < 0 {
            -unsafe { (*s).wrap }
        } else {
            unsafe { (*s).wrap }
        } {
            0 => {
                wraplen = 0; // raw deflate
            }
            1 => {
                wraplen = (6 + if (unsafe { (*s).strstart }) != 0 {
                    4i32
                } else {
                    0i32
                }) as size_t; // zlib wrapper
            }
            _ => {
                wraplen = 18; // for compiler happiness
            }
        }
    }
    // if not default parameters, return one of the conservative bounds
    if (unsafe { (*s).w_bits }) != 15 || (unsafe { (*s).hash_bits }) != 15u32 {
        #[cfg(target_pointer_width = "64")]
        {
            bound = if (unsafe { (*s).w_bits }) <= unsafe { (*s).hash_bits }
                && (unsafe { (*s).level }) != 0
            {
                fixedlen
            } else {
                storelen
            };
            return if bound.wrapping_add(wraplen) < bound {
                (18446744073709551615 as size_t)
            } else {
                bound.wrapping_add(wraplen)
            };
        }
        #[cfg(target_pointer_width = "32")]
        {
            bound = (if (unsafe { (*s).w_bits }) <= unsafe { (*s).hash_bits }
                && (unsafe { (*s).level }) != 0
            {
                fixedlen as u32
            } else {
                storelen as u32
            }) as size_t;
            return (if (bound as u32).wrapping_add(wraplen as u32) < (bound as u32) {
                4294967295u32
            } else {
                (bound as u32).wrapping_add(wraplen as u32)
            }) as size_t;
        }
    }
    // default settings: return tight bound for that case -- ~0.03% overhead
    // plus a small constant
    #[cfg(target_pointer_width = "64")]
    {
        bound = sourceLen
            .wrapping_add(sourceLen >> 12i32)
            .wrapping_add(sourceLen >> 14i32)
            .wrapping_add(sourceLen >> 25i32)
            .wrapping_add(13)
            .wrapping_sub(6)
            .wrapping_add(wraplen);
        {
            if bound < sourceLen {
                (18446744073709551615 as size_t)
            } else {
                bound
            }
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        bound = (sourceLen as u32)
            .wrapping_add((sourceLen as u32) >> 12i32)
            .wrapping_add((sourceLen as u32) >> 14i32)
            .wrapping_add((sourceLen as u32) >> 25i32)
            .wrapping_add(13)
            .wrapping_sub(6)
            .wrapping_add(wraplen as u32) as size_t;
        {
            (if (bound as u32) < (sourceLen as u32) {
                4294967295u32
            } else {
                bound as u32
            }) as size_t
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflateBound(strm: *mut z_stream_s, sourceLen: c_ulong) -> c_ulong {
    let bound: size_t = deflateBound_z(strm, sourceLen as size_t);
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        if (bound as c_ulong) != (bound as c_ulong) {
            (18446744073709551615 as c_ulong)
        } else {
            bound as c_ulong
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        if (bound as c_ulong) != ((bound as u32) as c_ulong) {
            (4294967295 as c_ulong)
        } else {
            bound as c_ulong
        }
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        if ((bound as c_ulong) as size_t) != bound {
            (4294967295 as c_ulong)
        } else {
            bound as c_ulong
        }
    }
}

/// Put a short in the pending buffer. The 16-bit value is put in MSB order.
/// IN assertion: the stream state is correct and there is enough room in
/// pending_buf.
fn putShortMSB(s: *mut internal_state, b: u32) {
    put_byte!(s, (b >> 8i32) as u8);
    put_byte!(s, (b & 255) as u8);
}

/// Flush as much pending output as possible. All deflate() output, except for
/// some deflate_stored() output, goes through this function so some
/// applications may wish to modify it to avoid allocating a large
/// strm->next_out buffer and copying into it. (See also read_buf()).
fn flush_pending(strm: *mut z_stream_s) {
    let mut len: u32 = 0;
    let s: *mut internal_state = unsafe { (*strm).state };
    unsafe { _tr_flush_bits(s) };
    len = if (unsafe { (*s).pending }) > ((unsafe { (*strm).avail_out }) as c_ulong) {
        unsafe { (*strm).avail_out }
    } else {
        (unsafe { (*s).pending }) as u32
    };
    if len == 0 {
        return;
    }
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*s).pending_out as *const ()) as *const u8,
                ((*strm).next_out as *mut ()) as *mut u8,
                (len as c_ulong) as usize,
            )
        };
    }
    #[cfg(target_pointer_width = "32")]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*s).pending_out as *const ()) as *const u8,
                ((*strm).next_out as *mut ()) as *mut u8,
                len as usize,
            )
        };
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*s).pending_out as *const ()) as *const u8,
                ((*strm).next_out as *mut ()) as *mut u8,
                (len as u64) as usize,
            )
        };
    }
    let __v1: *mut u8 = unsafe { (*strm).next_out };
    unsafe {
        (*strm).next_out = __v1.add(len as usize);
    }
    let __v4: *mut u8 = unsafe { (*s).pending_out };
    unsafe {
        (*s).pending_out = __v4.add(len as usize);
    }
    let __v7: c_ulong = unsafe { (*strm).total_out };
    unsafe {
        (*strm).total_out = __v7.wrapping_add(len as c_ulong);
    }
    let __v10: u32 = unsafe { (*strm).avail_out };
    unsafe {
        (*strm).avail_out = __v10.wrapping_sub(len);
    }
    let __v13: c_ulong = unsafe { (*s).pending };
    unsafe {
        (*s).pending = __v13.wrapping_sub(len as c_ulong);
    }
    if (unsafe { (*s).pending }) == 0 {
        unsafe {
            (*s).pending_out = (*s).pending_buf;
        }
    }
}

// Update the header CRC with the bytes s->pending_buf[beg..s->pending - 1].
#[unsafe(no_mangle)]
extern "C-unwind" fn deflate(strm: *mut z_stream_s, flush: i32) -> i32 {
    let mut old_flush: i32 = 0; // value of flush param for previous deflate call
    #[cfg(not(feature = "no-gzip"))]
    let mut s: *mut internal_state = unsafe { std::mem::zeroed() };
    if deflateStateCheck(strm) != 0 || flush > 5 || flush < 0 {
        return -2;
    }
    #[cfg(not(feature = "no-gzip"))]
    {
        s = unsafe { (*strm).state };
    }
    #[cfg(feature = "no-gzip")]
    let s: *mut internal_state = unsafe { (*strm).state };
    if unsafe { (*strm).next_out }.is_null()
        || (unsafe { (*strm).avail_in }) != 0 && unsafe { (*strm).next_in }.is_null()
        || (unsafe { (*s).status }) == 666 && flush != 4
    {
        ERR_RETURN!(strm, -2i32);
    }
    if (unsafe { (*strm).avail_out }) == 0 {
        ERR_RETURN!(strm, -5i32);
    }
    old_flush = unsafe { (*s).last_flush };
    unsafe {
        (*s).last_flush = flush;
    }
    // Flush as much pending output as possible
    if (unsafe { (*s).pending }) != 0 {
        flush_pending(strm);
        if (unsafe { (*strm).avail_out }) == 0 {
            // Since avail_out is 0, deflate will be called again with
            // more output space, but possibly with both pending and
            // avail_in equal to zero. There won't be anything to do,
            // but this is not an error situation so make sure we
            // return OK instead of BUF_ERROR at next call of deflate:
            unsafe {
                (*s).last_flush = -1;
            }
            return 0;
        }
    // Make sure there is something to do and avoid duplicate consecutive
    // flushes. For repeated and useless calls with Z_FINISH, we keep
    // returning Z_STREAM_END instead of Z_BUF_ERROR.
    } else {
        if (unsafe { (*strm).avail_in }) == 0 && RANK!(flush) <= RANK!(old_flush) && flush != 4 {
            ERR_RETURN!(strm, -5i32);
        }
    }
    // User must not provide more input after the first FINISH:
    if (unsafe { (*s).status }) == 666 && (unsafe { (*strm).avail_in }) != 0 {
        ERR_RETURN!(strm, -5i32);
    }
    // Write the header
    if (unsafe { (*s).status }) == 42 && (unsafe { (*s).wrap }) == 0 {
        unsafe {
            (*s).status = 113;
        }
    }
    if (unsafe { (*s).status }) == 42 {
        // zlib header
        let mut header: u32 =
            8u32.wrapping_add(unsafe { (*s).w_bits }.wrapping_sub(8) << 4i32) << 8i32;
        let mut level_flags: u32 = 0;
        if (unsafe { (*s).strategy }) >= 2 || (unsafe { (*s).level }) < 2 {
            level_flags = 0;
        } else {
            if (unsafe { (*s).level }) < 6 {
                level_flags = 1;
            } else {
                if (unsafe { (*s).level }) == 6 {
                    level_flags = 2;
                } else {
                    level_flags = 3;
                }
            }
        }
        header |= level_flags << 6i32;
        if (unsafe { (*s).strstart }) != 0 {
            header |= 32;
        }
        header = header.wrapping_add(31u32.wrapping_sub(header % 31));
        putShortMSB(s, header);
        // Save the adler32 of the preset dictionary:
        if (unsafe { (*s).strstart }) != 0 {
            putShortMSB(s, ((unsafe { (*strm).adler }) >> 16i32) as u32);
            putShortMSB(s, ((unsafe { (*strm).adler }) & 65535) as u32);
        }
        unsafe {
            (*strm).adler = adler32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
        }
        unsafe {
            (*s).status = 113;
        }
        // Compression must start with an empty pending buffer
        flush_pending(strm);
        if (unsafe { (*s).pending }) != 0 {
            unsafe {
                (*s).last_flush = -1;
            }
            return 0;
        }
    }
    #[cfg(not(feature = "no-gzip"))]
    {
        if (unsafe { (*s).status }) == 57 {
            // gzip header
            unsafe {
                (*strm).adler = crc32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
            }
            put_byte!(s, 31i32);
            put_byte!(s, 139i32);
            put_byte!(s, 8i32);
            if unsafe { (*s).gzhead }.is_null() {
                put_byte!(s, 0i32);
                put_byte!(s, 0i32);
                put_byte!(s, 0i32);
                put_byte!(s, 0i32);
                put_byte!(s, 0i32);
                put_byte!(
                    s,
                    if (unsafe { (*s).level }) == 9 {
                        2i32
                    } else {
                        if (unsafe { (*s).strategy }) >= 2 || (unsafe { (*s).level }) < 2 {
                            4i32
                        } else {
                            0i32
                        }
                    }
                );
                #[cfg(all(not(target_os = "macos"), unix))]
                {
                    put_byte!(s, 3i32);
                }
                #[cfg(target_os = "macos")]
                {
                    put_byte!(s, 19i32);
                }
                #[cfg(windows)]
                {
                    put_byte!(s, 10i32);
                }
                unsafe {
                    (*s).status = 113;
                }
                // Compression must start with an empty pending buffer
                flush_pending(strm);
                if (unsafe { (*s).pending }) != 0 {
                    unsafe {
                        (*s).last_flush = -1;
                    }
                    return 0;
                }
            } else {
                put_byte!(
                    s,
                    (if (unsafe { (*(*s).gzhead).text }) != 0 {
                        1
                    } else {
                        0
                    }) + if (unsafe { (*(*s).gzhead).hcrc }) != 0 {
                        2
                    } else {
                        0
                    } + if unsafe { (*(*s).gzhead).extra }.is_null() {
                        0
                    } else {
                        4
                    } + if unsafe { (*(*s).gzhead).name }.is_null() {
                        0
                    } else {
                        8
                    } + if unsafe { (*(*s).gzhead).comment }.is_null() {
                        0
                    } else {
                        16
                    }
                );
                put_byte!(s, ((unsafe { (*(*s).gzhead).time }) & 255) as u8);
                put_byte!(s, ((unsafe { (*(*s).gzhead).time }) >> 8i32 & 255) as u8);
                put_byte!(s, ((unsafe { (*(*s).gzhead).time }) >> 16i32 & 255) as u8);
                put_byte!(s, ((unsafe { (*(*s).gzhead).time }) >> 24i32 & 255) as u8);
                put_byte!(
                    s,
                    if (unsafe { (*s).level }) == 9 {
                        2i32
                    } else {
                        if (unsafe { (*s).strategy }) >= 2 || (unsafe { (*s).level }) < 2 {
                            4i32
                        } else {
                            0i32
                        }
                    }
                );
                put_byte!(s, (unsafe { (*(*s).gzhead).os }) & 255);
                if !unsafe { (*(*s).gzhead).extra }.is_null() {
                    put_byte!(s, (unsafe { (*(*s).gzhead).extra_len }) & 255);
                    put_byte!(s, (unsafe { (*(*s).gzhead).extra_len }) >> 8i32 & 255);
                }
                if (unsafe { (*(*s).gzhead).hcrc }) != 0 {
                    unsafe {
                        (*strm).adler = crc32_z(
                            (*strm).adler,
                            (*s).pending_buf as *const u8,
                            (*s).pending as size_t,
                        );
                    }
                }
                unsafe {
                    (*s).gzindex = 0;
                }
                unsafe {
                    (*s).status = 69;
                }
            }
        }
        if (unsafe { (*s).status }) == 69 {
            if !unsafe { (*(*s).gzhead).extra }.is_null() {
                let mut beg: c_ulong = unsafe { (*s).pending }; // start of bytes to update crc
                let mut left: c_ulong = (((unsafe { (*(*s).gzhead).extra_len }) & 65535)
                    as c_ulong)
                    .wrapping_sub(unsafe { (*s).gzindex });
                while unsafe { (*s).pending }.wrapping_add(left) > unsafe { (*s).pending_buf_size }
                {
                    let copy: c_ulong =
                        unsafe { (*s).pending_buf_size }.wrapping_sub(unsafe { (*s).pending });
                    #[cfg(all(target_pointer_width = "64", unix))]
                    {
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                (unsafe { (*(*s).gzhead).extra }.offset((*s).gzindex as isize)
                                    as *const ()) as *const u8,
                                (unsafe { (*s).pending_buf }.offset((*s).pending as isize)
                                    as *mut ()) as *mut u8,
                                copy as usize,
                            )
                        };
                    }
                    #[cfg(target_pointer_width = "32")]
                    {
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                (unsafe { (*(*s).gzhead).extra }.offset((*s).gzindex as isize)
                                    as *const ()) as *const u8,
                                (unsafe { (*s).pending_buf }.offset((*s).pending as isize)
                                    as *mut ()) as *mut u8,
                                (copy as u32) as usize,
                            )
                        };
                    }
                    #[cfg(all(target_pointer_width = "64", windows))]
                    {
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                (unsafe { (*(*s).gzhead).extra }.offset((*s).gzindex as isize)
                                    as *const ()) as *const u8,
                                (unsafe { (*s).pending_buf }.offset((*s).pending as isize)
                                    as *mut ()) as *mut u8,
                                (copy as u64) as usize,
                            )
                        };
                    }
                    unsafe {
                        (*s).pending = (*s).pending_buf_size;
                    }
                    HCRC_UPDATE!(beg, s, strm);
                    let __v65: c_ulong = unsafe { (*s).gzindex };
                    unsafe {
                        (*s).gzindex = __v65.wrapping_add(copy);
                    }
                    flush_pending(strm);
                    if (unsafe { (*s).pending }) != 0 {
                        unsafe {
                            (*s).last_flush = -1;
                        }
                        return 0;
                    }
                    beg = 0;
                    left = left.wrapping_sub(copy);
                }
                #[cfg(all(target_pointer_width = "64", unix))]
                {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            (unsafe { (*(*s).gzhead).extra }.offset((*s).gzindex as isize)
                                as *const ()) as *const u8,
                            (unsafe { (*s).pending_buf }.offset((*s).pending as isize) as *mut ())
                                as *mut u8,
                            left as usize,
                        )
                    };
                }
                #[cfg(target_pointer_width = "32")]
                {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            (unsafe { (*(*s).gzhead).extra }.offset((*s).gzindex as isize)
                                as *const ()) as *const u8,
                            (unsafe { (*s).pending_buf }.offset((*s).pending as isize) as *mut ())
                                as *mut u8,
                            (left as u32) as usize,
                        )
                    };
                }
                #[cfg(all(target_pointer_width = "64", windows))]
                {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            (unsafe { (*(*s).gzhead).extra }.offset((*s).gzindex as isize)
                                as *const ()) as *const u8,
                            (unsafe { (*s).pending_buf }.offset((*s).pending as isize) as *mut ())
                                as *mut u8,
                            (left as u64) as usize,
                        )
                    };
                }
                let __v71: c_ulong = unsafe { (*s).pending };
                unsafe {
                    (*s).pending = __v71.wrapping_add(left);
                }
                HCRC_UPDATE!(beg, s, strm);
                unsafe {
                    (*s).gzindex = 0;
                }
            }
            unsafe {
                (*s).status = 73;
            }
        }
        if (unsafe { (*s).status }) == 73 {
            if !unsafe { (*(*s).gzhead).name }.is_null() {
                let mut beg: c_ulong = unsafe { (*s).pending }; // start of bytes to update crc
                let mut val: i32 = 0;
                loop {
                    if (unsafe { (*s).pending }) == unsafe { (*s).pending_buf_size } {
                        HCRC_UPDATE!(beg, s, strm);
                        flush_pending(strm);
                        if (unsafe { (*s).pending }) != 0 {
                            unsafe {
                                (*s).last_flush = -1;
                            }
                            return 0;
                        }
                        beg = 0;
                    }
                    let __v76: c_ulong = unsafe { (*s).gzindex };
                    unsafe {
                        (*s).gzindex = __v76.wrapping_add(1);
                    }
                    val = ((unsafe { *unsafe { (*(*s).gzhead).name }.add(__v76 as usize) }) as u32)
                        as i32;
                    put_byte!(s, val);
                    if val == 0 {
                        break;
                    }
                }
                HCRC_UPDATE!(beg, s, strm);
                unsafe {
                    (*s).gzindex = 0;
                }
            }
            unsafe {
                (*s).status = 91;
            }
        }
        if (unsafe { (*s).status }) == 91 {
            if !unsafe { (*(*s).gzhead).comment }.is_null() {
                let mut beg: c_ulong = unsafe { (*s).pending }; // start of bytes to update crc
                let mut val: i32 = 0;
                loop {
                    if (unsafe { (*s).pending }) == unsafe { (*s).pending_buf_size } {
                        HCRC_UPDATE!(beg, s, strm);
                        flush_pending(strm);
                        if (unsafe { (*s).pending }) != 0 {
                            unsafe {
                                (*s).last_flush = -1;
                            }
                            return 0;
                        }
                        beg = 0;
                    }
                    let __v85: c_ulong = unsafe { (*s).gzindex };
                    unsafe {
                        (*s).gzindex = __v85.wrapping_add(1);
                    }
                    val = ((unsafe { *unsafe { (*(*s).gzhead).comment }.add(__v85 as usize) })
                        as u32) as i32;
                    put_byte!(s, val);
                    if val == 0 {
                        break;
                    }
                }
                HCRC_UPDATE!(beg, s, strm);
            }
            unsafe {
                (*s).status = 103;
            }
        }
        if (unsafe { (*s).status }) == 103 {
            if (unsafe { (*(*s).gzhead).hcrc }) != 0 {
                if unsafe { (*s).pending }.wrapping_add(2) > unsafe { (*s).pending_buf_size } {
                    flush_pending(strm);
                    if (unsafe { (*s).pending }) != 0 {
                        unsafe {
                            (*s).last_flush = -1;
                        }
                        return 0;
                    }
                }
                put_byte!(s, ((unsafe { (*strm).adler }) & 255) as u8);
                put_byte!(s, ((unsafe { (*strm).adler }) >> 8i32 & 255) as u8);
                unsafe {
                    (*strm).adler = crc32((0 as c_ulong), std::ptr::null::<u8>(), 0u32);
                }
            }
            unsafe {
                (*s).status = 113;
            }
            // Compression must start with an empty pending buffer
            flush_pending(strm);
            if (unsafe { (*s).pending }) != 0 {
                unsafe {
                    (*s).last_flush = -1;
                }
                return 0;
            }
        }
    }
    // Start a new block or continue the current one.
    if (unsafe { (*strm).avail_in }) != 0
        || (unsafe { (*s).lookahead }) != 0
        || flush != 0 && (unsafe { (*s).status }) != 666
    {
        let mut bstate: u32 = 0;
        let __v98: u32 = if (unsafe { (*s).level }) == 0 {
            deflate_stored(s, flush) as u32
        } else {
            let __v99: u32 = if (unsafe { (*s).strategy }) == 2 {
                deflate_huff(s, flush) as u32
            } else {
                let __v100: u32 = if (unsafe { (*s).strategy }) == 3 {
                    deflate_rle(s, flush) as u32
                } else {
                    (unsafe {
                        unsafe { configuration_table.0[(*s).level as usize].func }.unwrap()(
                            s, flush,
                        )
                    }) as u32
                };
                __v100
            };
            __v99
        };
        bstate = __v98 as u32;
        if (bstate as u32) == 2 || (bstate as u32) == 3 {
            unsafe {
                (*s).status = 666;
            }
        }
        if (bstate as u32) == 0 || (bstate as u32) == 2 {
            if (unsafe { (*strm).avail_out }) == 0 {
                unsafe {
                    (*s).last_flush = -1;
                }
                // avoid BUF_ERROR next call, see above
            }
            return 0;
            // If flush != Z_NO_FLUSH && avail_out == 0, the next call
            // of deflate should use the same flush parameter to make sure
            // that the flush is complete. So we don't have to output an
            // empty block here, this will be done at next call. This also
            // ensures that for a very small output buffer, we emit at most
            // one empty block.
        }
        if (bstate as u32) == 1 {
            if flush == 1 {
                unsafe { _tr_align(s) };
            } else {
                if flush != 5 {
                    // FULL_FLUSH or SYNC_FLUSH
                    unsafe {
                        _tr_stored_block(s, std::ptr::null_mut::<c_char>(), (0 as c_ulong), 0i32)
                    };
                    // For a full flush, this empty block will be recognized
                    // as a special marker by inflate_sync().
                    if flush == 3 {
                        #[cfg(all(target_pointer_width = "64", unix))]
                        {
                            CLEAR_HASH!(s);
                        }
                        #[cfg(target_pointer_width = "32")]
                        {
                            CLEAR_HASH__3!(s);
                        }
                        #[cfg(all(target_pointer_width = "64", windows))]
                        {
                            CLEAR_HASH__2!(s);
                        }
                        // forget history
                        if (unsafe { (*s).lookahead }) == 0 {
                            unsafe {
                                (*s).strstart = 0;
                            }
                            unsafe {
                                (*s).block_start = 0;
                            }
                            unsafe {
                                (*s).insert = 0;
                            }
                        }
                    }
                }
            }
            flush_pending(strm);
            if (unsafe { (*strm).avail_out }) == 0 {
                unsafe {
                    (*s).last_flush = -1;
                }
                // avoid BUF_ERROR at next call, see above
                return 0;
            }
        }
    }
    if flush != 4 {
        return 0;
    }
    if (unsafe { (*s).wrap }) <= 0 {
        return 1;
    }
    // Write the trailer
    #[cfg(not(feature = "no-gzip"))]
    {
        if (unsafe { (*s).wrap }) == 2 {
            put_byte!(s, ((unsafe { (*strm).adler }) & 255) as u8);
            put_byte!(s, ((unsafe { (*strm).adler }) >> 8i32 & 255) as u8);
            put_byte!(s, ((unsafe { (*strm).adler }) >> 16i32 & 255) as u8);
            put_byte!(s, ((unsafe { (*strm).adler }) >> 24i32 & 255) as u8);
            put_byte!(s, ((unsafe { (*strm).total_in }) & 255) as u8);
            put_byte!(s, ((unsafe { (*strm).total_in }) >> 8i32 & 255) as u8);
            put_byte!(s, ((unsafe { (*strm).total_in }) >> 16i32 & 255) as u8);
            put_byte!(s, ((unsafe { (*strm).total_in }) >> 24i32 & 255) as u8);
        } else {
            putShortMSB(s, ((unsafe { (*strm).adler }) >> 16i32) as u32);
            putShortMSB(s, ((unsafe { (*strm).adler }) & 65535) as u32);
        }
    }
    #[cfg(feature = "no-gzip")]
    {
        putShortMSB(s, ((unsafe { (*strm).adler }) >> 16i32) as u32);
        putShortMSB(s, ((unsafe { (*strm).adler }) & 65535) as u32);
    }
    flush_pending(strm);
    // If avail_out is zero, the application will call deflate again
    // to flush the rest.
    if (unsafe { (*s).wrap }) > 0 {
        unsafe {
            (*s).wrap = -(*s).wrap;
        }
    }
    // write the trailer only once!
    if (unsafe { (*s).pending }) != 0 { 0 } else { 1 }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn deflateEnd(strm: *mut z_stream_s) -> i32 {
    let mut status: i32 = 0;
    if deflateStateCheck(strm) != 0 {
        return -2;
    }
    status = unsafe { (*(*strm).state).status };
    // Deallocate in reverse order of allocations:
    TRY_FREE__2!(strm, unsafe { (*(*strm).state).pending_buf });
    TRY_FREE!(strm, unsafe { (*(*strm).state).head });
    TRY_FREE!(strm, unsafe { (*(*strm).state).prev });
    TRY_FREE__2!(strm, unsafe { (*(*strm).state).window });
    ZFREE!(strm, unsafe { (*strm).state });
    unsafe {
        (*strm).state = std::ptr::null_mut::<internal_state>();
    }
    if status == 113 { -3 } else { 0 }
}

/// Copy the source state to the destination state.
/// To simplify the source, this is not supported for 16-bit MSDOS (which
/// doesn't have enough memory anyway to duplicate compression states).
#[unsafe(no_mangle)]
extern "C-unwind" fn deflateCopy(dest: *mut z_stream_s, source: *mut z_stream_s) -> i32 {
    if deflateStateCheck(source) != 0 || dest.is_null() {
        return -2;
    }
    let ss: *mut internal_state = unsafe { (*source).state };
    unsafe {
        std::ptr::copy_nonoverlapping(
            source as *const u8,
            dest as *mut u8,
            std::mem::size_of::<z_stream_s>(),
        )
    };
    #[cfg(all(target_pointer_width = "64", unix))]
    let ds: *mut internal_state =
        ZALLOC__2!(dest, 1i32, std::mem::size_of::<internal_state>() as c_ulong)
            as *mut internal_state;
    #[cfg(target_pointer_width = "32")]
    let ds: *mut internal_state =
        ZALLOC__4!(dest, 1i32, std::mem::size_of::<internal_state>() as u32) as *mut internal_state;
    #[cfg(all(target_pointer_width = "64", windows))]
    let ds: *mut internal_state =
        ZALLOC__2!(dest, 1i32, std::mem::size_of::<internal_state>() as u64) as *mut internal_state;
    if ds.is_null() {
        return -4;
    }
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        zmemzero!(ds, std::mem::size_of::<internal_state>() as c_ulong);
    }
    #[cfg(target_pointer_width = "32")]
    {
        zmemzero!(ds, std::mem::size_of::<internal_state>() as u32);
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        zmemzero!(ds, std::mem::size_of::<internal_state>() as u64);
    }
    unsafe {
        (*dest).state = ds;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(
            ss as *const u8,
            ds as *mut u8,
            std::mem::size_of::<internal_state>(),
        )
    };
    unsafe {
        (*ds).strm = dest;
    }
    unsafe {
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            (*ds).window = ZALLOC!(
                dest,
                unsafe { (*ds).w_size },
                (2 as c_ulong).wrapping_mul(std::mem::size_of::<u8>() as c_ulong)
            ) as *mut u8;
        }
        #[cfg(target_pointer_width = "32")]
        {
            (*ds).window = ZALLOC__3!(
                dest,
                unsafe { (*ds).w_size },
                2u32.wrapping_mul(std::mem::size_of::<u8>() as u32)
            ) as *mut u8;
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            (*ds).window = ZALLOC!(
                dest,
                unsafe { (*ds).w_size },
                2u64.wrapping_mul(std::mem::size_of::<u8>() as u64)
            ) as *mut u8;
        }
    }
    unsafe {
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            (*ds).prev = ZALLOC!(
                dest,
                unsafe { (*ds).w_size },
                std::mem::size_of::<u16>() as c_ulong
            ) as *mut u16;
        }
        #[cfg(target_pointer_width = "32")]
        {
            (*ds).prev = ZALLOC__3!(
                dest,
                unsafe { (*ds).w_size },
                std::mem::size_of::<u16>() as u32
            ) as *mut u16;
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            (*ds).prev = ZALLOC!(
                dest,
                unsafe { (*ds).w_size },
                std::mem::size_of::<u16>() as u64
            ) as *mut u16;
        }
    }
    unsafe {
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            (*ds).head = ZALLOC!(
                dest,
                unsafe { (*ds).hash_size },
                std::mem::size_of::<u16>() as c_ulong
            ) as *mut u16;
        }
        #[cfg(target_pointer_width = "32")]
        {
            (*ds).head = ZALLOC__3!(
                dest,
                unsafe { (*ds).hash_size },
                std::mem::size_of::<u16>() as u32
            ) as *mut u16;
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            (*ds).head = ZALLOC!(
                dest,
                unsafe { (*ds).hash_size },
                std::mem::size_of::<u16>() as u64
            ) as *mut u16;
        }
    }
    unsafe {
        #[cfg(feature = "lit-mem")]
        {
            (*ds).pending_buf = ZALLOC!(dest, unsafe { (*ds).lit_bufsize }, 5i32) as *mut u8;
        }
        #[cfg(not(feature = "lit-mem"))]
        {
            (*ds).pending_buf = ZALLOC!(dest, unsafe { (*ds).lit_bufsize }, 4i32) as *mut u8;
        }
    }
    if unsafe { (*ds).window }.is_null()
        || unsafe { (*ds).prev }.is_null()
        || unsafe { (*ds).head }.is_null()
        || unsafe { (*ds).pending_buf }.is_null()
    {
        deflateEnd(dest);
        return -4;
    }
    // following zmemcpy's do not work for 16-bit MSDOS
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).window as *const ()) as *const u8,
                ((*ds).window as *mut ()) as *mut u8,
                (*ss).high_water as usize,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).prev as *const ()) as *const u8,
                ((*ds).prev as *mut ()) as *mut u8,
                ((if (*ss).slid != 0
                    || unsafe { (*ss).strstart }.wrapping_sub((*ss).insert) > (*ds).w_size
                {
                    (*ds).w_size
                } else {
                    unsafe { (*ss).strstart }.wrapping_sub((*ss).insert)
                }) as c_ulong)
                    .wrapping_mul(std::mem::size_of::<u16>() as c_ulong) as usize,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).head as *const ()) as *const u8,
                ((*ds).head as *mut ()) as *mut u8,
                ((*ds).hash_size as c_ulong).wrapping_mul(std::mem::size_of::<u16>() as c_ulong)
                    as usize,
            )
        };
    }
    #[cfg(target_pointer_width = "32")]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).window as *const ()) as *const u8,
                ((*ds).window as *mut ()) as *mut u8,
                ((*ss).high_water as u32) as usize,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).prev as *const ()) as *const u8,
                ((*ds).prev as *mut ()) as *mut u8,
                if (*ss).slid != 0
                    || unsafe { (*ss).strstart }.wrapping_sub((*ss).insert) > (*ds).w_size
                {
                    (*ds).w_size
                } else {
                    unsafe { (*ss).strstart }.wrapping_sub((*ss).insert)
                }
                .wrapping_mul(std::mem::size_of::<u16>() as u32) as usize,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).head as *const ()) as *const u8,
                ((*ds).head as *mut ()) as *mut u8,
                unsafe { (*ds).hash_size }.wrapping_mul(std::mem::size_of::<u16>() as u32) as usize,
            )
        };
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).window as *const ()) as *const u8,
                ((*ds).window as *mut ()) as *mut u8,
                ((*ss).high_water as u64) as usize,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).prev as *const ()) as *const u8,
                ((*ds).prev as *mut ()) as *mut u8,
                ((if (*ss).slid != 0
                    || unsafe { (*ss).strstart }.wrapping_sub((*ss).insert) > (*ds).w_size
                {
                    (*ds).w_size
                } else {
                    unsafe { (*ss).strstart }.wrapping_sub((*ss).insert)
                }) as u64)
                    .wrapping_mul(std::mem::size_of::<u16>() as u64) as usize,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).head as *const ()) as *const u8,
                ((*ds).head as *mut ()) as *mut u8,
                ((*ds).hash_size as u64).wrapping_mul(std::mem::size_of::<u16>() as u64) as usize,
            )
        };
    }
    unsafe {
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            (*ds).pending_out = unsafe { (*ds).pending_buf }.offset(
                (unsafe { (*ss).pending_out }.offset_from((*ss).pending_buf as *mut u8) as c_long)
                    as isize,
            );
        }
        #[cfg(target_pointer_width = "32")]
        {
            (*ds).pending_out = unsafe { (*ds).pending_buf }.offset(
                (unsafe { (*ss).pending_out }.offset_from((*ss).pending_buf as *mut u8) as i32)
                    as isize,
            );
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            (*ds).pending_out = unsafe { (*ds).pending_buf }.offset(
                (unsafe { (*ss).pending_out }.offset_from((*ss).pending_buf as *mut u8) as i64)
                    as isize,
            );
        }
    }
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).pending_out as *const ()) as *const u8,
                ((*ds).pending_out as *mut ()) as *mut u8,
                (*ss).pending as usize,
            )
        };
    }
    #[cfg(target_pointer_width = "32")]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).pending_out as *const ()) as *const u8,
                ((*ds).pending_out as *mut ()) as *mut u8,
                ((*ss).pending as u32) as usize,
            )
        };
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).pending_out as *const ()) as *const u8,
                ((*ds).pending_out as *mut ()) as *mut u8,
                ((*ss).pending as u64) as usize,
            )
        };
    }
    unsafe {
        #[cfg(feature = "lit-mem")]
        {
            (*ds).d_buf = unsafe { (*ds).pending_buf }.offset(((*ds).lit_bufsize << 1i32) as isize)
                as *mut u16;
        }
        #[cfg(not(feature = "lit-mem"))]
        {
            (*ds).sym_buf = unsafe { (*ds).pending_buf }.offset((*ds).lit_bufsize as isize);
        }
    }
    #[cfg(feature = "lit-mem")]
    {
        unsafe {
            (*ds).l_buf = unsafe { (*ds).pending_buf }.offset(((*ds).lit_bufsize << 2i32) as isize);
        }
    }
    #[cfg(all(target_pointer_width = "64", unix, feature = "lit-mem"))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).d_buf as *const ()) as *const u8,
                ((*ds).d_buf as *mut ()) as *mut u8,
                ((*ss).sym_next as c_ulong).wrapping_mul(std::mem::size_of::<u16>() as c_ulong)
                    as usize,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).l_buf as *const ()) as *const u8,
                ((*ds).l_buf as *mut ()) as *mut u8,
                ((*ss).sym_next as c_ulong) as usize,
            )
        };
    }
    #[cfg(all(target_pointer_width = "32", feature = "lit-mem"))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).d_buf as *const ()) as *const u8,
                ((*ds).d_buf as *mut ()) as *mut u8,
                unsafe { (*ss).sym_next }.wrapping_mul(std::mem::size_of::<u16>() as u32) as usize,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).l_buf as *const ()) as *const u8,
                ((*ds).l_buf as *mut ()) as *mut u8,
                (*ss).sym_next as usize,
            )
        };
    }
    #[cfg(all(target_pointer_width = "64", windows, feature = "lit-mem"))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).d_buf as *const ()) as *const u8,
                ((*ds).d_buf as *mut ()) as *mut u8,
                ((*ss).sym_next as u64).wrapping_mul(std::mem::size_of::<u16>() as u64) as usize,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).l_buf as *const ()) as *const u8,
                ((*ds).l_buf as *mut ()) as *mut u8,
                ((*ss).sym_next as u64) as usize,
            )
        };
    }
    #[cfg(all(target_pointer_width = "64", unix, not(feature = "lit-mem")))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).sym_buf as *const ()) as *const u8,
                ((*ds).sym_buf as *mut ()) as *mut u8,
                ((*ss).sym_next as c_ulong) as usize,
            )
        };
    }
    #[cfg(all(target_pointer_width = "32", not(feature = "lit-mem")))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).sym_buf as *const ()) as *const u8,
                ((*ds).sym_buf as *mut ()) as *mut u8,
                (*ss).sym_next as usize,
            )
        };
    }
    #[cfg(all(target_pointer_width = "64", windows, not(feature = "lit-mem")))]
    {
        unsafe {
            std::ptr::copy_nonoverlapping(
                ((*ss).sym_buf as *const ()) as *const u8,
                ((*ds).sym_buf as *mut ()) as *mut u8,
                ((*ss).sym_next as u64) as usize,
            )
        };
    }
    unsafe {
        (*ds).l_desc.dyn_tree = (*ds).dyn_ltree.as_mut_ptr() as *mut ct_data_s;
    }
    unsafe {
        (*ds).d_desc.dyn_tree = (*ds).dyn_dtree.as_mut_ptr() as *mut ct_data_s;
    }
    unsafe {
        (*ds).bl_desc.dyn_tree = (*ds).bl_tree.as_mut_ptr() as *mut ct_data_s;
    }
    0
}

#[cfg(all(
    target_pointer_width = "64",
    unix,
    not(feature = "fastest"),
    not(feature = "zlib-debug")
))]
/// Set match_start to the longest match starting at the given string and
/// return its length. Matches shorter or equal to prev_length are discarded,
/// in which case the result is equal to prev_length and match_start is
/// garbage.
/// IN assertions: cur_match is the head of the hash chain for the current
///   string (strstart) and its distance is <= MAX_DIST, and prev_length >= 1
/// OUT assertion: the match length is not greater than s->lookahead.
fn longest_match(s: *mut internal_state, mut cur_match: u32) -> u32 {
    let mut chain_length: u32 = unsafe { (*s).max_chain_length }; // max hash chain length
    let mut scan: *mut u8 = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) }; // current string
    let mut r#match: *mut u8 = unsafe { std::mem::zeroed() }; // matched string
    let mut len: i32 = 0; // length of current match
    let mut best_len: i32 = (unsafe { (*s).prev_length }) as i32; // best match length so far
    let mut nice_match: i32 = unsafe { (*s).nice_match }; // stop if match long enough
    let limit: u32 = if (unsafe { (*s).strstart }) > MAX_DIST!(s) {
        unsafe { (*s).strstart }.wrapping_sub(MAX_DIST!(s))
    } else {
        0
    };
    // Stop when cur_match becomes <= limit. To simplify the code,
    // we prevent matches with the string of window index 0.
    let prev: *mut u16 = unsafe { (*s).prev };
    let wmask: u32 = unsafe { (*s).w_mask };
    let strend: *mut u8 = unsafe {
        unsafe { (*s).window }
            .offset((*s).strstart as isize)
            .add(258)
    };
    let mut scan_end1: u8 = unsafe { *scan.offset((best_len - 1) as isize) };
    let mut scan_end: u8 = unsafe { *scan.offset(best_len as isize) };
    // The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
    // It is easy to get rid of this optimization if necessary.
    // Do not waste too much time if we already have a good match:
    if (unsafe { (*s).prev_length }) >= unsafe { (*s).good_match } {
        chain_length >>= 2i32;
    }
    // Do not look for matches beyond the end of the input. This is necessary
    // to make deflate deterministic.
    if (nice_match as u32) > unsafe { (*s).lookahead } {
        nice_match = (unsafe { (*s).lookahead }) as i32;
    }
    loop {
        r#match = unsafe { unsafe { (*s).window }.add(cur_match as usize) };
        // Skip to next match if the match length cannot increase
        // or if the match length is less than 2.  Note that the checks below
        // for insufficient lookahead only occur occasionally for performance
        // reasons.  Therefore uninitialized memory will be accessed, and
        // conditional jumps will be made that depend on those values.
        // However the length of the match is limited to the lookahead, so
        // the output of deflate is not affected by the uninitialized values.
        let __v2: bool = if (((unsafe { *r#match.offset(best_len as isize) }) as u32) as i32)
            != ((scan_end as u32) as i32)
            || (((unsafe { *r#match.offset((best_len - 1) as isize) }) as u32) as i32)
                != ((scan_end1 as u32) as i32)
            || (((unsafe { *r#match }) as u32) as i32) != (((unsafe { *scan }) as u32) as i32)
        {
            true
        } else {
            let __v3: *mut u8 = r#match;
            let __v4: *mut u8 = unsafe { __v3.add(1) };
            r#match = __v4;
            (((unsafe { *__v4 }) as u32) as i32) != (((unsafe { *scan.add(1) }) as u32) as i32)
        };
        if !__v2 {
            // The check at best_len - 1 can be removed because it will be made
            // again later. (This heuristic is not always a win.)
            // It is not necessary to compare scan[2] and match[2] since they
            // are always equal when the other bytes match, given that
            // the hash keys are equal and that HASH_BITS >= 8.
            let __v6: *mut u8 = scan;
            scan = unsafe { __v6.add(2) };
            let __v8: *mut u8 = r#match;
            r#match = unsafe { __v8.add(1) };
            // We check for insufficient lookahead only every 8th comparison;
            // the 256th check will be made at strstart + 258.
            loop {
                let __v11: *mut u8 = scan;
                let __v12: *mut u8 = unsafe { __v11.add(1) };
                scan = __v12;
                let __v13: *mut u8 = r#match;
                let __v14: *mut u8 = unsafe { __v13.add(1) };
                r#match = __v14;
                let __v15: bool = if (((unsafe { *__v12 }) as u32) as i32)
                    == (((unsafe { *__v14 }) as u32) as i32)
                {
                    let __v16: *mut u8 = scan;
                    let __v17: *mut u8 = unsafe { __v16.add(1) };
                    scan = __v17;
                    let __v18: *mut u8 = r#match;
                    let __v19: *mut u8 = unsafe { __v18.add(1) };
                    r#match = __v19;
                    (((unsafe { *__v17 }) as u32) as i32) == (((unsafe { *__v19 }) as u32) as i32)
                } else {
                    false
                };
                let __v20: bool = if __v15 {
                    let __v21: *mut u8 = scan;
                    let __v22: *mut u8 = unsafe { __v21.add(1) };
                    scan = __v22;
                    let __v23: *mut u8 = r#match;
                    let __v24: *mut u8 = unsafe { __v23.add(1) };
                    r#match = __v24;
                    (((unsafe { *__v22 }) as u32) as i32) == (((unsafe { *__v24 }) as u32) as i32)
                } else {
                    false
                };
                let __v25: bool = if __v20 {
                    let __v26: *mut u8 = scan;
                    let __v27: *mut u8 = unsafe { __v26.add(1) };
                    scan = __v27;
                    let __v28: *mut u8 = r#match;
                    let __v29: *mut u8 = unsafe { __v28.add(1) };
                    r#match = __v29;
                    (((unsafe { *__v27 }) as u32) as i32) == (((unsafe { *__v29 }) as u32) as i32)
                } else {
                    false
                };
                let __v30: bool = if __v25 {
                    let __v31: *mut u8 = scan;
                    let __v32: *mut u8 = unsafe { __v31.add(1) };
                    scan = __v32;
                    let __v33: *mut u8 = r#match;
                    let __v34: *mut u8 = unsafe { __v33.add(1) };
                    r#match = __v34;
                    (((unsafe { *__v32 }) as u32) as i32) == (((unsafe { *__v34 }) as u32) as i32)
                } else {
                    false
                };
                let __v35: bool = if __v30 {
                    let __v36: *mut u8 = scan;
                    let __v37: *mut u8 = unsafe { __v36.add(1) };
                    scan = __v37;
                    let __v38: *mut u8 = r#match;
                    let __v39: *mut u8 = unsafe { __v38.add(1) };
                    r#match = __v39;
                    (((unsafe { *__v37 }) as u32) as i32) == (((unsafe { *__v39 }) as u32) as i32)
                } else {
                    false
                };
                let __v40: bool = if __v35 {
                    let __v41: *mut u8 = scan;
                    let __v42: *mut u8 = unsafe { __v41.add(1) };
                    scan = __v42;
                    let __v43: *mut u8 = r#match;
                    let __v44: *mut u8 = unsafe { __v43.add(1) };
                    r#match = __v44;
                    (((unsafe { *__v42 }) as u32) as i32) == (((unsafe { *__v44 }) as u32) as i32)
                } else {
                    false
                };
                let __v45: bool = if __v40 {
                    let __v46: *mut u8 = scan;
                    let __v47: *mut u8 = unsafe { __v46.add(1) };
                    scan = __v47;
                    let __v48: *mut u8 = r#match;
                    let __v49: *mut u8 = unsafe { __v48.add(1) };
                    r#match = __v49;
                    (((unsafe { *__v47 }) as u32) as i32) == (((unsafe { *__v49 }) as u32) as i32)
                } else {
                    false
                };
                if !(__v45 && scan < strend) {
                    break;
                }
            }
            len = 258 - (((unsafe { strend.offset_from(scan as *mut u8) }) as c_long) as i32);
            scan = unsafe { strend.sub(258) };
            if len > best_len {
                unsafe {
                    (*s).match_start = cur_match;
                }
                best_len = len;
                if len >= nice_match {
                    break;
                }
                scan_end1 = unsafe { *scan.offset((best_len - 1) as isize) };
                scan_end = unsafe { *scan.offset(best_len as isize) };
            }
        }
        let __v50: u32 = (unsafe { *prev.add((cur_match & wmask) as usize) }) as u32;
        cur_match = __v50;
        let __v51: bool = if __v50 > limit {
            let __v53: u32 = chain_length.wrapping_sub(1);
            chain_length = __v53;
            __v53 != 0
        } else {
            false
        };
        if !__v51 {
            break;
        }
    }
    if (best_len as u32) <= unsafe { (*s).lookahead } {
        return best_len as u32;
    }
    unsafe { (*s).lookahead }
}

#[cfg(all(
    target_pointer_width = "64",
    unix,
    feature = "fastest",
    not(feature = "zlib-debug")
))]
/// Optimized version for FASTEST only
fn longest_match(s: *mut internal_state, cur_match: u32) -> u32 {
    let mut scan: *mut u8 = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) }; // current string
    let mut r#match: *mut u8 = unsafe { std::mem::zeroed() }; // matched string
    let mut len: i32 = 0; // length of current match
    let strend: *mut u8 = unsafe {
        unsafe { (*s).window }
            .offset((*s).strstart as isize)
            .add(258)
    };
    // The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
    // It is easy to get rid of this optimization if necessary.
    r#match = unsafe { unsafe { (*s).window }.add(cur_match as usize) };
    // Return failure if the match length is less than 2:
    if (((unsafe { *r#match.add(0) }) as u32) as i32) != (((unsafe { *scan.add(0) }) as u32) as i32)
        || (((unsafe { *r#match.add(1) }) as u32) as i32)
            != (((unsafe { *scan.add(1) }) as u32) as i32)
    {
        return 2u32;
    }
    // The check at best_len - 1 can be removed because it will be made
    // again later. (This heuristic is not always a win.)
    // It is not necessary to compare scan[2] and match[2] since they
    // are always equal when the other bytes match, given that
    // the hash keys are equal and that HASH_BITS >= 8.
    let __v0: *mut u8 = scan;
    scan = unsafe { __v0.add(2) };
    let __v2: *mut u8 = r#match;
    r#match = unsafe { __v2.add(2) };
    // We check for insufficient lookahead only every 8th comparison;
    // the 256th check will be made at strstart + 258.
    loop {
        let __v5: *mut u8 = scan;
        let __v6: *mut u8 = unsafe { __v5.add(1) };
        scan = __v6;
        let __v7: *mut u8 = r#match;
        let __v8: *mut u8 = unsafe { __v7.add(1) };
        r#match = __v8;
        let __v9: bool =
            if (((unsafe { *__v6 }) as u32) as i32) == (((unsafe { *__v8 }) as u32) as i32) {
                let __v10: *mut u8 = scan;
                let __v11: *mut u8 = unsafe { __v10.add(1) };
                scan = __v11;
                let __v12: *mut u8 = r#match;
                let __v13: *mut u8 = unsafe { __v12.add(1) };
                r#match = __v13;
                (((unsafe { *__v11 }) as u32) as i32) == (((unsafe { *__v13 }) as u32) as i32)
            } else {
                false
            };
        let __v14: bool = if __v9 {
            let __v15: *mut u8 = scan;
            let __v16: *mut u8 = unsafe { __v15.add(1) };
            scan = __v16;
            let __v17: *mut u8 = r#match;
            let __v18: *mut u8 = unsafe { __v17.add(1) };
            r#match = __v18;
            (((unsafe { *__v16 }) as u32) as i32) == (((unsafe { *__v18 }) as u32) as i32)
        } else {
            false
        };
        let __v19: bool = if __v14 {
            let __v20: *mut u8 = scan;
            let __v21: *mut u8 = unsafe { __v20.add(1) };
            scan = __v21;
            let __v22: *mut u8 = r#match;
            let __v23: *mut u8 = unsafe { __v22.add(1) };
            r#match = __v23;
            (((unsafe { *__v21 }) as u32) as i32) == (((unsafe { *__v23 }) as u32) as i32)
        } else {
            false
        };
        let __v24: bool = if __v19 {
            let __v25: *mut u8 = scan;
            let __v26: *mut u8 = unsafe { __v25.add(1) };
            scan = __v26;
            let __v27: *mut u8 = r#match;
            let __v28: *mut u8 = unsafe { __v27.add(1) };
            r#match = __v28;
            (((unsafe { *__v26 }) as u32) as i32) == (((unsafe { *__v28 }) as u32) as i32)
        } else {
            false
        };
        let __v29: bool = if __v24 {
            let __v30: *mut u8 = scan;
            let __v31: *mut u8 = unsafe { __v30.add(1) };
            scan = __v31;
            let __v32: *mut u8 = r#match;
            let __v33: *mut u8 = unsafe { __v32.add(1) };
            r#match = __v33;
            (((unsafe { *__v31 }) as u32) as i32) == (((unsafe { *__v33 }) as u32) as i32)
        } else {
            false
        };
        let __v34: bool = if __v29 {
            let __v35: *mut u8 = scan;
            let __v36: *mut u8 = unsafe { __v35.add(1) };
            scan = __v36;
            let __v37: *mut u8 = r#match;
            let __v38: *mut u8 = unsafe { __v37.add(1) };
            r#match = __v38;
            (((unsafe { *__v36 }) as u32) as i32) == (((unsafe { *__v38 }) as u32) as i32)
        } else {
            false
        };
        let __v39: bool = if __v34 {
            let __v40: *mut u8 = scan;
            let __v41: *mut u8 = unsafe { __v40.add(1) };
            scan = __v41;
            let __v42: *mut u8 = r#match;
            let __v43: *mut u8 = unsafe { __v42.add(1) };
            r#match = __v43;
            (((unsafe { *__v41 }) as u32) as i32) == (((unsafe { *__v43 }) as u32) as i32)
        } else {
            false
        };
        if !(__v39 && scan < strend) {
            break;
        }
    }
    len = 258 - (((unsafe { strend.offset_from(scan as *mut u8) }) as c_long) as i32);
    if len < 3 {
        return 2u32;
    }
    unsafe {
        (*s).match_start = cur_match;
    }
    if (len as u32) <= unsafe { (*s).lookahead } {
        len as u32
    } else {
        unsafe { (*s).lookahead }
    }
}

#[cfg(all(
    target_pointer_width = "64",
    unix,
    not(feature = "fastest"),
    feature = "zlib-debug"
))]
/// Set match_start to the longest match starting at the given string and
/// return its length. Matches shorter or equal to prev_length are discarded,
/// in which case the result is equal to prev_length and match_start is
/// garbage.
/// IN assertions: cur_match is the head of the hash chain for the current
///   string (strstart) and its distance is <= MAX_DIST, and prev_length >= 1
/// OUT assertion: the match length is not greater than s->lookahead.
fn longest_match(s: *mut internal_state, mut cur_match: u32) -> u32 {
    let mut chain_length: u32 = unsafe { (*s).max_chain_length }; // max hash chain length
    let mut scan: *mut u8 = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) }; // current string
    let mut r#match: *mut u8 = unsafe { std::mem::zeroed() }; // matched string
    let mut len: i32 = 0; // length of current match
    let mut best_len: i32 = (unsafe { (*s).prev_length }) as i32; // best match length so far
    let mut nice_match: i32 = unsafe { (*s).nice_match }; // stop if match long enough
    let limit: u32 = if (unsafe { (*s).strstart }) > MAX_DIST!(s) {
        unsafe { (*s).strstart }.wrapping_sub(MAX_DIST!(s))
    } else {
        0
    };
    // Stop when cur_match becomes <= limit. To simplify the code,
    // we prevent matches with the string of window index 0.
    let prev: *mut u16 = unsafe { (*s).prev };
    let wmask: u32 = unsafe { (*s).w_mask };
    let strend: *mut u8 = unsafe {
        unsafe { (*s).window }
            .offset((*s).strstart as isize)
            .add(258)
    };
    let mut scan_end1: u8 = unsafe { *scan.offset((best_len - 1) as isize) };
    let mut scan_end: u8 = unsafe { *scan.offset(best_len as isize) };
    // The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
    // It is easy to get rid of this optimization if necessary.
    Assert!(
        (unsafe { (*s).hash_bits }) >= 8 && true,
        c"Code too clever".as_ptr() as *mut c_char
    );
    // Do not waste too much time if we already have a good match:
    if (unsafe { (*s).prev_length }) >= unsafe { (*s).good_match } {
        chain_length >>= 2i32;
    }
    // Do not look for matches beyond the end of the input. This is necessary
    // to make deflate deterministic.
    if (nice_match as u32) > unsafe { (*s).lookahead } {
        nice_match = (unsafe { (*s).lookahead }) as i32;
    }
    Assert!(
        ((unsafe { (*s).strstart }) as c_ulong)
            <= unsafe { (*s).window_size }.wrapping_sub((262 as c_ulong)),
        c"need lookahead".as_ptr() as *mut c_char
    );
    loop {
        Assert!(
            cur_match < unsafe { (*s).strstart },
            c"no future".as_ptr() as *mut c_char
        );
        r#match = unsafe { unsafe { (*s).window }.add(cur_match as usize) };
        // Skip to next match if the match length cannot increase
        // or if the match length is less than 2.  Note that the checks below
        // for insufficient lookahead only occur occasionally for performance
        // reasons.  Therefore uninitialized memory will be accessed, and
        // conditional jumps will be made that depend on those values.
        // However the length of the match is limited to the lookahead, so
        // the output of deflate is not affected by the uninitialized values.
        let __v2: bool = if (((unsafe { *r#match.offset(best_len as isize) }) as u32) as i32)
            != ((scan_end as u32) as i32)
            || (((unsafe { *r#match.offset((best_len - 1) as isize) }) as u32) as i32)
                != ((scan_end1 as u32) as i32)
            || (((unsafe { *r#match }) as u32) as i32) != (((unsafe { *scan }) as u32) as i32)
        {
            true
        } else {
            let __v3: *mut u8 = r#match;
            let __v4: *mut u8 = unsafe { __v3.add(1) };
            r#match = __v4;
            (((unsafe { *__v4 }) as u32) as i32) != (((unsafe { *scan.add(1) }) as u32) as i32)
        };
        if !__v2 {
            // The check at best_len - 1 can be removed because it will be made
            // again later. (This heuristic is not always a win.)
            // It is not necessary to compare scan[2] and match[2] since they
            // are always equal when the other bytes match, given that
            // the hash keys are equal and that HASH_BITS >= 8.
            let __v6: *mut u8 = scan;
            scan = unsafe { __v6.add(2) };
            let __v8: *mut u8 = r#match;
            r#match = unsafe { __v8.add(1) };
            Assert!(
                (((unsafe { *scan }) as u32) as i32) == (((unsafe { *r#match }) as u32) as i32),
                c"match[2]?".as_ptr() as *mut c_char
            );
            // We check for insufficient lookahead only every 8th comparison;
            // the 256th check will be made at strstart + 258.
            loop {
                let __v11: *mut u8 = scan;
                let __v12: *mut u8 = unsafe { __v11.add(1) };
                scan = __v12;
                let __v13: *mut u8 = r#match;
                let __v14: *mut u8 = unsafe { __v13.add(1) };
                r#match = __v14;
                let __v15: bool = if (((unsafe { *__v12 }) as u32) as i32)
                    == (((unsafe { *__v14 }) as u32) as i32)
                {
                    let __v16: *mut u8 = scan;
                    let __v17: *mut u8 = unsafe { __v16.add(1) };
                    scan = __v17;
                    let __v18: *mut u8 = r#match;
                    let __v19: *mut u8 = unsafe { __v18.add(1) };
                    r#match = __v19;
                    (((unsafe { *__v17 }) as u32) as i32) == (((unsafe { *__v19 }) as u32) as i32)
                } else {
                    false
                };
                let __v20: bool = if __v15 {
                    let __v21: *mut u8 = scan;
                    let __v22: *mut u8 = unsafe { __v21.add(1) };
                    scan = __v22;
                    let __v23: *mut u8 = r#match;
                    let __v24: *mut u8 = unsafe { __v23.add(1) };
                    r#match = __v24;
                    (((unsafe { *__v22 }) as u32) as i32) == (((unsafe { *__v24 }) as u32) as i32)
                } else {
                    false
                };
                let __v25: bool = if __v20 {
                    let __v26: *mut u8 = scan;
                    let __v27: *mut u8 = unsafe { __v26.add(1) };
                    scan = __v27;
                    let __v28: *mut u8 = r#match;
                    let __v29: *mut u8 = unsafe { __v28.add(1) };
                    r#match = __v29;
                    (((unsafe { *__v27 }) as u32) as i32) == (((unsafe { *__v29 }) as u32) as i32)
                } else {
                    false
                };
                let __v30: bool = if __v25 {
                    let __v31: *mut u8 = scan;
                    let __v32: *mut u8 = unsafe { __v31.add(1) };
                    scan = __v32;
                    let __v33: *mut u8 = r#match;
                    let __v34: *mut u8 = unsafe { __v33.add(1) };
                    r#match = __v34;
                    (((unsafe { *__v32 }) as u32) as i32) == (((unsafe { *__v34 }) as u32) as i32)
                } else {
                    false
                };
                let __v35: bool = if __v30 {
                    let __v36: *mut u8 = scan;
                    let __v37: *mut u8 = unsafe { __v36.add(1) };
                    scan = __v37;
                    let __v38: *mut u8 = r#match;
                    let __v39: *mut u8 = unsafe { __v38.add(1) };
                    r#match = __v39;
                    (((unsafe { *__v37 }) as u32) as i32) == (((unsafe { *__v39 }) as u32) as i32)
                } else {
                    false
                };
                let __v40: bool = if __v35 {
                    let __v41: *mut u8 = scan;
                    let __v42: *mut u8 = unsafe { __v41.add(1) };
                    scan = __v42;
                    let __v43: *mut u8 = r#match;
                    let __v44: *mut u8 = unsafe { __v43.add(1) };
                    r#match = __v44;
                    (((unsafe { *__v42 }) as u32) as i32) == (((unsafe { *__v44 }) as u32) as i32)
                } else {
                    false
                };
                let __v45: bool = if __v40 {
                    let __v46: *mut u8 = scan;
                    let __v47: *mut u8 = unsafe { __v46.add(1) };
                    scan = __v47;
                    let __v48: *mut u8 = r#match;
                    let __v49: *mut u8 = unsafe { __v48.add(1) };
                    r#match = __v49;
                    (((unsafe { *__v47 }) as u32) as i32) == (((unsafe { *__v49 }) as u32) as i32)
                } else {
                    false
                };
                if !(__v45 && scan < strend) {
                    break;
                }
            }
            Assert!(
                scan <= unsafe {
                    unsafe { (*s).window }
                        .add((unsafe { (*s).window_size }.wrapping_sub(1) as u32) as usize)
                },
                c"wild scan".as_ptr() as *mut c_char
            );
            len = 258 - (((unsafe { strend.offset_from(scan as *mut u8) }) as c_long) as i32);
            scan = unsafe { strend.sub(258) };
            if len > best_len {
                unsafe {
                    (*s).match_start = cur_match;
                }
                best_len = len;
                if len >= nice_match {
                    break;
                }
                scan_end1 = unsafe { *scan.offset((best_len - 1) as isize) };
                scan_end = unsafe { *scan.offset(best_len as isize) };
            }
        }
        let __v50: u32 = (unsafe { *prev.add((cur_match & wmask) as usize) }) as u32;
        cur_match = __v50;
        let __v51: bool = if __v50 > limit {
            let __v53: u32 = chain_length.wrapping_sub(1);
            chain_length = __v53;
            __v53 != 0
        } else {
            false
        };
        if !__v51 {
            break;
        }
    }
    if (best_len as u32) <= unsafe { (*s).lookahead } {
        return best_len as u32;
    }
    unsafe { (*s).lookahead }
}

#[cfg(all(
    target_pointer_width = "64",
    unix,
    feature = "fastest",
    feature = "zlib-debug"
))]
/// Optimized version for FASTEST only
fn longest_match(s: *mut internal_state, cur_match: u32) -> u32 {
    let mut scan: *mut u8 = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) }; // current string
    let mut r#match: *mut u8 = unsafe { std::mem::zeroed() }; // matched string
    let mut len: i32 = 0; // length of current match
    let strend: *mut u8 = unsafe {
        unsafe { (*s).window }
            .offset((*s).strstart as isize)
            .add(258)
    };
    // The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
    // It is easy to get rid of this optimization if necessary.
    Assert!(
        (unsafe { (*s).hash_bits }) >= 8 && true,
        c"Code too clever".as_ptr() as *mut c_char
    );
    Assert!(
        ((unsafe { (*s).strstart }) as c_ulong)
            <= unsafe { (*s).window_size }.wrapping_sub((262 as c_ulong)),
        c"need lookahead".as_ptr() as *mut c_char
    );
    Assert!(
        cur_match < unsafe { (*s).strstart },
        c"no future".as_ptr() as *mut c_char
    );
    r#match = unsafe { unsafe { (*s).window }.add(cur_match as usize) };
    // Return failure if the match length is less than 2:
    if (((unsafe { *r#match.add(0) }) as u32) as i32) != (((unsafe { *scan.add(0) }) as u32) as i32)
        || (((unsafe { *r#match.add(1) }) as u32) as i32)
            != (((unsafe { *scan.add(1) }) as u32) as i32)
    {
        return 2u32;
    }
    // The check at best_len - 1 can be removed because it will be made
    // again later. (This heuristic is not always a win.)
    // It is not necessary to compare scan[2] and match[2] since they
    // are always equal when the other bytes match, given that
    // the hash keys are equal and that HASH_BITS >= 8.
    let __v0: *mut u8 = scan;
    scan = unsafe { __v0.add(2) };
    let __v2: *mut u8 = r#match;
    r#match = unsafe { __v2.add(2) };
    Assert!(
        (((unsafe { *scan }) as u32) as i32) == (((unsafe { *r#match }) as u32) as i32),
        c"match[2]?".as_ptr() as *mut c_char
    );
    // We check for insufficient lookahead only every 8th comparison;
    // the 256th check will be made at strstart + 258.
    loop {
        let __v5: *mut u8 = scan;
        let __v6: *mut u8 = unsafe { __v5.add(1) };
        scan = __v6;
        let __v7: *mut u8 = r#match;
        let __v8: *mut u8 = unsafe { __v7.add(1) };
        r#match = __v8;
        let __v9: bool =
            if (((unsafe { *__v6 }) as u32) as i32) == (((unsafe { *__v8 }) as u32) as i32) {
                let __v10: *mut u8 = scan;
                let __v11: *mut u8 = unsafe { __v10.add(1) };
                scan = __v11;
                let __v12: *mut u8 = r#match;
                let __v13: *mut u8 = unsafe { __v12.add(1) };
                r#match = __v13;
                (((unsafe { *__v11 }) as u32) as i32) == (((unsafe { *__v13 }) as u32) as i32)
            } else {
                false
            };
        let __v14: bool = if __v9 {
            let __v15: *mut u8 = scan;
            let __v16: *mut u8 = unsafe { __v15.add(1) };
            scan = __v16;
            let __v17: *mut u8 = r#match;
            let __v18: *mut u8 = unsafe { __v17.add(1) };
            r#match = __v18;
            (((unsafe { *__v16 }) as u32) as i32) == (((unsafe { *__v18 }) as u32) as i32)
        } else {
            false
        };
        let __v19: bool = if __v14 {
            let __v20: *mut u8 = scan;
            let __v21: *mut u8 = unsafe { __v20.add(1) };
            scan = __v21;
            let __v22: *mut u8 = r#match;
            let __v23: *mut u8 = unsafe { __v22.add(1) };
            r#match = __v23;
            (((unsafe { *__v21 }) as u32) as i32) == (((unsafe { *__v23 }) as u32) as i32)
        } else {
            false
        };
        let __v24: bool = if __v19 {
            let __v25: *mut u8 = scan;
            let __v26: *mut u8 = unsafe { __v25.add(1) };
            scan = __v26;
            let __v27: *mut u8 = r#match;
            let __v28: *mut u8 = unsafe { __v27.add(1) };
            r#match = __v28;
            (((unsafe { *__v26 }) as u32) as i32) == (((unsafe { *__v28 }) as u32) as i32)
        } else {
            false
        };
        let __v29: bool = if __v24 {
            let __v30: *mut u8 = scan;
            let __v31: *mut u8 = unsafe { __v30.add(1) };
            scan = __v31;
            let __v32: *mut u8 = r#match;
            let __v33: *mut u8 = unsafe { __v32.add(1) };
            r#match = __v33;
            (((unsafe { *__v31 }) as u32) as i32) == (((unsafe { *__v33 }) as u32) as i32)
        } else {
            false
        };
        let __v34: bool = if __v29 {
            let __v35: *mut u8 = scan;
            let __v36: *mut u8 = unsafe { __v35.add(1) };
            scan = __v36;
            let __v37: *mut u8 = r#match;
            let __v38: *mut u8 = unsafe { __v37.add(1) };
            r#match = __v38;
            (((unsafe { *__v36 }) as u32) as i32) == (((unsafe { *__v38 }) as u32) as i32)
        } else {
            false
        };
        let __v39: bool = if __v34 {
            let __v40: *mut u8 = scan;
            let __v41: *mut u8 = unsafe { __v40.add(1) };
            scan = __v41;
            let __v42: *mut u8 = r#match;
            let __v43: *mut u8 = unsafe { __v42.add(1) };
            r#match = __v43;
            (((unsafe { *__v41 }) as u32) as i32) == (((unsafe { *__v43 }) as u32) as i32)
        } else {
            false
        };
        if !(__v39 && scan < strend) {
            break;
        }
    }
    Assert!(
        scan <= unsafe {
            unsafe { (*s).window }
                .add((unsafe { (*s).window_size }.wrapping_sub(1) as u32) as usize)
        },
        c"wild scan".as_ptr() as *mut c_char
    );
    len = 258 - (((unsafe { strend.offset_from(scan as *mut u8) }) as c_long) as i32);
    if len < 3 {
        return 2u32;
    }
    unsafe {
        (*s).match_start = cur_match;
    }
    if (len as u32) <= unsafe { (*s).lookahead } {
        len as u32
    } else {
        unsafe { (*s).lookahead }
    }
}

#[cfg(all(
    target_pointer_width = "32",
    not(feature = "fastest"),
    not(feature = "zlib-debug")
))]
/// Set match_start to the longest match starting at the given string and
/// return its length. Matches shorter or equal to prev_length are discarded,
/// in which case the result is equal to prev_length and match_start is
/// garbage.
/// IN assertions: cur_match is the head of the hash chain for the current
///   string (strstart) and its distance is <= MAX_DIST, and prev_length >= 1
/// OUT assertion: the match length is not greater than s->lookahead.
fn longest_match(s: *mut internal_state, mut cur_match: u32) -> u32 {
    let mut chain_length: u32 = unsafe { (*s).max_chain_length }; // max hash chain length
    let mut scan: *mut u8 = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) }; // current string
    let mut r#match: *mut u8 = unsafe { std::mem::zeroed() }; // matched string
    let mut len: i32 = 0; // length of current match
    let mut best_len: i32 = (unsafe { (*s).prev_length }) as i32; // best match length so far
    let mut nice_match: i32 = unsafe { (*s).nice_match }; // stop if match long enough
    let limit: u32 = if (unsafe { (*s).strstart }) > MAX_DIST!(s) {
        unsafe { (*s).strstart }.wrapping_sub(MAX_DIST!(s))
    } else {
        0
    };
    // Stop when cur_match becomes <= limit. To simplify the code,
    // we prevent matches with the string of window index 0.
    let prev: *mut u16 = unsafe { (*s).prev };
    let wmask: u32 = unsafe { (*s).w_mask };
    let strend: *mut u8 = unsafe {
        unsafe { (*s).window }
            .offset((*s).strstart as isize)
            .add(258)
    };
    let mut scan_end1: u8 = unsafe { *scan.offset((best_len - 1) as isize) };
    let mut scan_end: u8 = unsafe { *scan.offset(best_len as isize) };
    // The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
    // It is easy to get rid of this optimization if necessary.
    // Do not waste too much time if we already have a good match:
    if (unsafe { (*s).prev_length }) >= unsafe { (*s).good_match } {
        chain_length >>= 2i32;
    }
    // Do not look for matches beyond the end of the input. This is necessary
    // to make deflate deterministic.
    if (nice_match as u32) > unsafe { (*s).lookahead } {
        nice_match = (unsafe { (*s).lookahead }) as i32;
    }
    loop {
        r#match = unsafe { unsafe { (*s).window }.add(cur_match as usize) };
        // Skip to next match if the match length cannot increase
        // or if the match length is less than 2.  Note that the checks below
        // for insufficient lookahead only occur occasionally for performance
        // reasons.  Therefore uninitialized memory will be accessed, and
        // conditional jumps will be made that depend on those values.
        // However the length of the match is limited to the lookahead, so
        // the output of deflate is not affected by the uninitialized values.
        let __v2: bool = if (((unsafe { *r#match.offset(best_len as isize) }) as u32) as i32)
            != ((scan_end as u32) as i32)
            || (((unsafe { *r#match.offset((best_len - 1) as isize) }) as u32) as i32)
                != ((scan_end1 as u32) as i32)
            || (((unsafe { *r#match }) as u32) as i32) != (((unsafe { *scan }) as u32) as i32)
        {
            true
        } else {
            let __v3: *mut u8 = r#match;
            let __v4: *mut u8 = unsafe { __v3.add(1) };
            r#match = __v4;
            (((unsafe { *__v4 }) as u32) as i32) != (((unsafe { *scan.add(1) }) as u32) as i32)
        };
        if !__v2 {
            // The check at best_len - 1 can be removed because it will be made
            // again later. (This heuristic is not always a win.)
            // It is not necessary to compare scan[2] and match[2] since they
            // are always equal when the other bytes match, given that
            // the hash keys are equal and that HASH_BITS >= 8.
            let __v6: *mut u8 = scan;
            scan = unsafe { __v6.add(2) };
            let __v8: *mut u8 = r#match;
            r#match = unsafe { __v8.add(1) };
            // We check for insufficient lookahead only every 8th comparison;
            // the 256th check will be made at strstart + 258.
            loop {
                let __v11: *mut u8 = scan;
                let __v12: *mut u8 = unsafe { __v11.add(1) };
                scan = __v12;
                let __v13: *mut u8 = r#match;
                let __v14: *mut u8 = unsafe { __v13.add(1) };
                r#match = __v14;
                let __v15: bool = if (((unsafe { *__v12 }) as u32) as i32)
                    == (((unsafe { *__v14 }) as u32) as i32)
                {
                    let __v16: *mut u8 = scan;
                    let __v17: *mut u8 = unsafe { __v16.add(1) };
                    scan = __v17;
                    let __v18: *mut u8 = r#match;
                    let __v19: *mut u8 = unsafe { __v18.add(1) };
                    r#match = __v19;
                    (((unsafe { *__v17 }) as u32) as i32) == (((unsafe { *__v19 }) as u32) as i32)
                } else {
                    false
                };
                let __v20: bool = if __v15 {
                    let __v21: *mut u8 = scan;
                    let __v22: *mut u8 = unsafe { __v21.add(1) };
                    scan = __v22;
                    let __v23: *mut u8 = r#match;
                    let __v24: *mut u8 = unsafe { __v23.add(1) };
                    r#match = __v24;
                    (((unsafe { *__v22 }) as u32) as i32) == (((unsafe { *__v24 }) as u32) as i32)
                } else {
                    false
                };
                let __v25: bool = if __v20 {
                    let __v26: *mut u8 = scan;
                    let __v27: *mut u8 = unsafe { __v26.add(1) };
                    scan = __v27;
                    let __v28: *mut u8 = r#match;
                    let __v29: *mut u8 = unsafe { __v28.add(1) };
                    r#match = __v29;
                    (((unsafe { *__v27 }) as u32) as i32) == (((unsafe { *__v29 }) as u32) as i32)
                } else {
                    false
                };
                let __v30: bool = if __v25 {
                    let __v31: *mut u8 = scan;
                    let __v32: *mut u8 = unsafe { __v31.add(1) };
                    scan = __v32;
                    let __v33: *mut u8 = r#match;
                    let __v34: *mut u8 = unsafe { __v33.add(1) };
                    r#match = __v34;
                    (((unsafe { *__v32 }) as u32) as i32) == (((unsafe { *__v34 }) as u32) as i32)
                } else {
                    false
                };
                let __v35: bool = if __v30 {
                    let __v36: *mut u8 = scan;
                    let __v37: *mut u8 = unsafe { __v36.add(1) };
                    scan = __v37;
                    let __v38: *mut u8 = r#match;
                    let __v39: *mut u8 = unsafe { __v38.add(1) };
                    r#match = __v39;
                    (((unsafe { *__v37 }) as u32) as i32) == (((unsafe { *__v39 }) as u32) as i32)
                } else {
                    false
                };
                let __v40: bool = if __v35 {
                    let __v41: *mut u8 = scan;
                    let __v42: *mut u8 = unsafe { __v41.add(1) };
                    scan = __v42;
                    let __v43: *mut u8 = r#match;
                    let __v44: *mut u8 = unsafe { __v43.add(1) };
                    r#match = __v44;
                    (((unsafe { *__v42 }) as u32) as i32) == (((unsafe { *__v44 }) as u32) as i32)
                } else {
                    false
                };
                let __v45: bool = if __v40 {
                    let __v46: *mut u8 = scan;
                    let __v47: *mut u8 = unsafe { __v46.add(1) };
                    scan = __v47;
                    let __v48: *mut u8 = r#match;
                    let __v49: *mut u8 = unsafe { __v48.add(1) };
                    r#match = __v49;
                    (((unsafe { *__v47 }) as u32) as i32) == (((unsafe { *__v49 }) as u32) as i32)
                } else {
                    false
                };
                if !(__v45 && scan < strend) {
                    break;
                }
            }
            len = 258 - ((unsafe { strend.offset_from(scan as *mut u8) }) as i32);
            scan = unsafe { strend.sub(258) };
            if len > best_len {
                unsafe {
                    (*s).match_start = cur_match;
                }
                best_len = len;
                if len >= nice_match {
                    break;
                }
                scan_end1 = unsafe { *scan.offset((best_len - 1) as isize) };
                scan_end = unsafe { *scan.offset(best_len as isize) };
            }
        }
        let __v50: u32 = (unsafe { *prev.add((cur_match & wmask) as usize) }) as u32;
        cur_match = __v50;
        let __v51: bool = if __v50 > limit {
            let __v53: u32 = chain_length.wrapping_sub(1);
            chain_length = __v53;
            __v53 != 0
        } else {
            false
        };
        if !__v51 {
            break;
        }
    }
    if (best_len as u32) <= unsafe { (*s).lookahead } {
        return best_len as u32;
    }
    unsafe { (*s).lookahead }
}

#[cfg(all(
    target_pointer_width = "32",
    feature = "fastest",
    not(feature = "zlib-debug")
))]
/// Optimized version for FASTEST only
fn longest_match(s: *mut internal_state, cur_match: u32) -> u32 {
    let mut scan: *mut u8 = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) }; // current string
    let mut r#match: *mut u8 = unsafe { std::mem::zeroed() }; // matched string
    let mut len: i32 = 0; // length of current match
    let strend: *mut u8 = unsafe {
        unsafe { (*s).window }
            .offset((*s).strstart as isize)
            .add(258)
    };
    // The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
    // It is easy to get rid of this optimization if necessary.
    r#match = unsafe { unsafe { (*s).window }.add(cur_match as usize) };
    // Return failure if the match length is less than 2:
    if (((unsafe { *r#match.add(0) }) as u32) as i32) != (((unsafe { *scan.add(0) }) as u32) as i32)
        || (((unsafe { *r#match.add(1) }) as u32) as i32)
            != (((unsafe { *scan.add(1) }) as u32) as i32)
    {
        return 2u32;
    }
    // The check at best_len - 1 can be removed because it will be made
    // again later. (This heuristic is not always a win.)
    // It is not necessary to compare scan[2] and match[2] since they
    // are always equal when the other bytes match, given that
    // the hash keys are equal and that HASH_BITS >= 8.
    let __v0: *mut u8 = scan;
    scan = unsafe { __v0.add(2) };
    let __v2: *mut u8 = r#match;
    r#match = unsafe { __v2.add(2) };
    // We check for insufficient lookahead only every 8th comparison;
    // the 256th check will be made at strstart + 258.
    loop {
        let __v5: *mut u8 = scan;
        let __v6: *mut u8 = unsafe { __v5.add(1) };
        scan = __v6;
        let __v7: *mut u8 = r#match;
        let __v8: *mut u8 = unsafe { __v7.add(1) };
        r#match = __v8;
        let __v9: bool =
            if (((unsafe { *__v6 }) as u32) as i32) == (((unsafe { *__v8 }) as u32) as i32) {
                let __v10: *mut u8 = scan;
                let __v11: *mut u8 = unsafe { __v10.add(1) };
                scan = __v11;
                let __v12: *mut u8 = r#match;
                let __v13: *mut u8 = unsafe { __v12.add(1) };
                r#match = __v13;
                (((unsafe { *__v11 }) as u32) as i32) == (((unsafe { *__v13 }) as u32) as i32)
            } else {
                false
            };
        let __v14: bool = if __v9 {
            let __v15: *mut u8 = scan;
            let __v16: *mut u8 = unsafe { __v15.add(1) };
            scan = __v16;
            let __v17: *mut u8 = r#match;
            let __v18: *mut u8 = unsafe { __v17.add(1) };
            r#match = __v18;
            (((unsafe { *__v16 }) as u32) as i32) == (((unsafe { *__v18 }) as u32) as i32)
        } else {
            false
        };
        let __v19: bool = if __v14 {
            let __v20: *mut u8 = scan;
            let __v21: *mut u8 = unsafe { __v20.add(1) };
            scan = __v21;
            let __v22: *mut u8 = r#match;
            let __v23: *mut u8 = unsafe { __v22.add(1) };
            r#match = __v23;
            (((unsafe { *__v21 }) as u32) as i32) == (((unsafe { *__v23 }) as u32) as i32)
        } else {
            false
        };
        let __v24: bool = if __v19 {
            let __v25: *mut u8 = scan;
            let __v26: *mut u8 = unsafe { __v25.add(1) };
            scan = __v26;
            let __v27: *mut u8 = r#match;
            let __v28: *mut u8 = unsafe { __v27.add(1) };
            r#match = __v28;
            (((unsafe { *__v26 }) as u32) as i32) == (((unsafe { *__v28 }) as u32) as i32)
        } else {
            false
        };
        let __v29: bool = if __v24 {
            let __v30: *mut u8 = scan;
            let __v31: *mut u8 = unsafe { __v30.add(1) };
            scan = __v31;
            let __v32: *mut u8 = r#match;
            let __v33: *mut u8 = unsafe { __v32.add(1) };
            r#match = __v33;
            (((unsafe { *__v31 }) as u32) as i32) == (((unsafe { *__v33 }) as u32) as i32)
        } else {
            false
        };
        let __v34: bool = if __v29 {
            let __v35: *mut u8 = scan;
            let __v36: *mut u8 = unsafe { __v35.add(1) };
            scan = __v36;
            let __v37: *mut u8 = r#match;
            let __v38: *mut u8 = unsafe { __v37.add(1) };
            r#match = __v38;
            (((unsafe { *__v36 }) as u32) as i32) == (((unsafe { *__v38 }) as u32) as i32)
        } else {
            false
        };
        let __v39: bool = if __v34 {
            let __v40: *mut u8 = scan;
            let __v41: *mut u8 = unsafe { __v40.add(1) };
            scan = __v41;
            let __v42: *mut u8 = r#match;
            let __v43: *mut u8 = unsafe { __v42.add(1) };
            r#match = __v43;
            (((unsafe { *__v41 }) as u32) as i32) == (((unsafe { *__v43 }) as u32) as i32)
        } else {
            false
        };
        if !(__v39 && scan < strend) {
            break;
        }
    }
    len = 258 - ((unsafe { strend.offset_from(scan as *mut u8) }) as i32);
    if len < 3 {
        return 2u32;
    }
    unsafe {
        (*s).match_start = cur_match;
    }
    if (len as u32) <= unsafe { (*s).lookahead } {
        len as u32
    } else {
        unsafe { (*s).lookahead }
    }
}

#[cfg(all(
    target_pointer_width = "32",
    not(feature = "fastest"),
    feature = "zlib-debug"
))]
/// Set match_start to the longest match starting at the given string and
/// return its length. Matches shorter or equal to prev_length are discarded,
/// in which case the result is equal to prev_length and match_start is
/// garbage.
/// IN assertions: cur_match is the head of the hash chain for the current
///   string (strstart) and its distance is <= MAX_DIST, and prev_length >= 1
/// OUT assertion: the match length is not greater than s->lookahead.
fn longest_match(s: *mut internal_state, mut cur_match: u32) -> u32 {
    let mut chain_length: u32 = unsafe { (*s).max_chain_length }; // max hash chain length
    let mut scan: *mut u8 = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) }; // current string
    let mut r#match: *mut u8 = unsafe { std::mem::zeroed() }; // matched string
    let mut len: i32 = 0; // length of current match
    let mut best_len: i32 = (unsafe { (*s).prev_length }) as i32; // best match length so far
    let mut nice_match: i32 = unsafe { (*s).nice_match }; // stop if match long enough
    let limit: u32 = if (unsafe { (*s).strstart }) > MAX_DIST!(s) {
        unsafe { (*s).strstart }.wrapping_sub(MAX_DIST!(s))
    } else {
        0
    };
    // Stop when cur_match becomes <= limit. To simplify the code,
    // we prevent matches with the string of window index 0.
    let prev: *mut u16 = unsafe { (*s).prev };
    let wmask: u32 = unsafe { (*s).w_mask };
    let strend: *mut u8 = unsafe {
        unsafe { (*s).window }
            .offset((*s).strstart as isize)
            .add(258)
    };
    let mut scan_end1: u8 = unsafe { *scan.offset((best_len - 1) as isize) };
    let mut scan_end: u8 = unsafe { *scan.offset(best_len as isize) };
    // The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
    // It is easy to get rid of this optimization if necessary.
    Assert!(
        (unsafe { (*s).hash_bits }) >= 8 && true,
        c"Code too clever".as_ptr() as *mut c_char
    );
    // Do not waste too much time if we already have a good match:
    if (unsafe { (*s).prev_length }) >= unsafe { (*s).good_match } {
        chain_length >>= 2i32;
    }
    // Do not look for matches beyond the end of the input. This is necessary
    // to make deflate deterministic.
    if (nice_match as u32) > unsafe { (*s).lookahead } {
        nice_match = (unsafe { (*s).lookahead }) as i32;
    }
    Assert!(
        ((unsafe { (*s).strstart }) as c_ulong)
            <= unsafe { (*s).window_size }.wrapping_sub((262 as c_ulong)),
        c"need lookahead".as_ptr() as *mut c_char
    );
    loop {
        Assert!(
            cur_match < unsafe { (*s).strstart },
            c"no future".as_ptr() as *mut c_char
        );
        r#match = unsafe { unsafe { (*s).window }.add(cur_match as usize) };
        // Skip to next match if the match length cannot increase
        // or if the match length is less than 2.  Note that the checks below
        // for insufficient lookahead only occur occasionally for performance
        // reasons.  Therefore uninitialized memory will be accessed, and
        // conditional jumps will be made that depend on those values.
        // However the length of the match is limited to the lookahead, so
        // the output of deflate is not affected by the uninitialized values.
        let __v2: bool = if (((unsafe { *r#match.offset(best_len as isize) }) as u32) as i32)
            != ((scan_end as u32) as i32)
            || (((unsafe { *r#match.offset((best_len - 1) as isize) }) as u32) as i32)
                != ((scan_end1 as u32) as i32)
            || (((unsafe { *r#match }) as u32) as i32) != (((unsafe { *scan }) as u32) as i32)
        {
            true
        } else {
            let __v3: *mut u8 = r#match;
            let __v4: *mut u8 = unsafe { __v3.add(1) };
            r#match = __v4;
            (((unsafe { *__v4 }) as u32) as i32) != (((unsafe { *scan.add(1) }) as u32) as i32)
        };
        if !__v2 {
            // The check at best_len - 1 can be removed because it will be made
            // again later. (This heuristic is not always a win.)
            // It is not necessary to compare scan[2] and match[2] since they
            // are always equal when the other bytes match, given that
            // the hash keys are equal and that HASH_BITS >= 8.
            let __v6: *mut u8 = scan;
            scan = unsafe { __v6.add(2) };
            let __v8: *mut u8 = r#match;
            r#match = unsafe { __v8.add(1) };
            Assert!(
                (((unsafe { *scan }) as u32) as i32) == (((unsafe { *r#match }) as u32) as i32),
                c"match[2]?".as_ptr() as *mut c_char
            );
            // We check for insufficient lookahead only every 8th comparison;
            // the 256th check will be made at strstart + 258.
            loop {
                let __v11: *mut u8 = scan;
                let __v12: *mut u8 = unsafe { __v11.add(1) };
                scan = __v12;
                let __v13: *mut u8 = r#match;
                let __v14: *mut u8 = unsafe { __v13.add(1) };
                r#match = __v14;
                let __v15: bool = if (((unsafe { *__v12 }) as u32) as i32)
                    == (((unsafe { *__v14 }) as u32) as i32)
                {
                    let __v16: *mut u8 = scan;
                    let __v17: *mut u8 = unsafe { __v16.add(1) };
                    scan = __v17;
                    let __v18: *mut u8 = r#match;
                    let __v19: *mut u8 = unsafe { __v18.add(1) };
                    r#match = __v19;
                    (((unsafe { *__v17 }) as u32) as i32) == (((unsafe { *__v19 }) as u32) as i32)
                } else {
                    false
                };
                let __v20: bool = if __v15 {
                    let __v21: *mut u8 = scan;
                    let __v22: *mut u8 = unsafe { __v21.add(1) };
                    scan = __v22;
                    let __v23: *mut u8 = r#match;
                    let __v24: *mut u8 = unsafe { __v23.add(1) };
                    r#match = __v24;
                    (((unsafe { *__v22 }) as u32) as i32) == (((unsafe { *__v24 }) as u32) as i32)
                } else {
                    false
                };
                let __v25: bool = if __v20 {
                    let __v26: *mut u8 = scan;
                    let __v27: *mut u8 = unsafe { __v26.add(1) };
                    scan = __v27;
                    let __v28: *mut u8 = r#match;
                    let __v29: *mut u8 = unsafe { __v28.add(1) };
                    r#match = __v29;
                    (((unsafe { *__v27 }) as u32) as i32) == (((unsafe { *__v29 }) as u32) as i32)
                } else {
                    false
                };
                let __v30: bool = if __v25 {
                    let __v31: *mut u8 = scan;
                    let __v32: *mut u8 = unsafe { __v31.add(1) };
                    scan = __v32;
                    let __v33: *mut u8 = r#match;
                    let __v34: *mut u8 = unsafe { __v33.add(1) };
                    r#match = __v34;
                    (((unsafe { *__v32 }) as u32) as i32) == (((unsafe { *__v34 }) as u32) as i32)
                } else {
                    false
                };
                let __v35: bool = if __v30 {
                    let __v36: *mut u8 = scan;
                    let __v37: *mut u8 = unsafe { __v36.add(1) };
                    scan = __v37;
                    let __v38: *mut u8 = r#match;
                    let __v39: *mut u8 = unsafe { __v38.add(1) };
                    r#match = __v39;
                    (((unsafe { *__v37 }) as u32) as i32) == (((unsafe { *__v39 }) as u32) as i32)
                } else {
                    false
                };
                let __v40: bool = if __v35 {
                    let __v41: *mut u8 = scan;
                    let __v42: *mut u8 = unsafe { __v41.add(1) };
                    scan = __v42;
                    let __v43: *mut u8 = r#match;
                    let __v44: *mut u8 = unsafe { __v43.add(1) };
                    r#match = __v44;
                    (((unsafe { *__v42 }) as u32) as i32) == (((unsafe { *__v44 }) as u32) as i32)
                } else {
                    false
                };
                let __v45: bool = if __v40 {
                    let __v46: *mut u8 = scan;
                    let __v47: *mut u8 = unsafe { __v46.add(1) };
                    scan = __v47;
                    let __v48: *mut u8 = r#match;
                    let __v49: *mut u8 = unsafe { __v48.add(1) };
                    r#match = __v49;
                    (((unsafe { *__v47 }) as u32) as i32) == (((unsafe { *__v49 }) as u32) as i32)
                } else {
                    false
                };
                if !(__v45 && scan < strend) {
                    break;
                }
            }
            Assert!(
                scan <= unsafe {
                    unsafe { (*s).window }
                        .add((unsafe { (*s).window_size }.wrapping_sub(1) as u32) as usize)
                },
                c"wild scan".as_ptr() as *mut c_char
            );
            len = 258 - ((unsafe { strend.offset_from(scan as *mut u8) }) as i32);
            scan = unsafe { strend.sub(258) };
            if len > best_len {
                unsafe {
                    (*s).match_start = cur_match;
                }
                best_len = len;
                if len >= nice_match {
                    break;
                }
                scan_end1 = unsafe { *scan.offset((best_len - 1) as isize) };
                scan_end = unsafe { *scan.offset(best_len as isize) };
            }
        }
        let __v50: u32 = (unsafe { *prev.add((cur_match & wmask) as usize) }) as u32;
        cur_match = __v50;
        let __v51: bool = if __v50 > limit {
            let __v53: u32 = chain_length.wrapping_sub(1);
            chain_length = __v53;
            __v53 != 0
        } else {
            false
        };
        if !__v51 {
            break;
        }
    }
    if (best_len as u32) <= unsafe { (*s).lookahead } {
        return best_len as u32;
    }
    unsafe { (*s).lookahead }
}

#[cfg(all(
    target_pointer_width = "32",
    feature = "fastest",
    feature = "zlib-debug"
))]
/// Optimized version for FASTEST only
fn longest_match(s: *mut internal_state, cur_match: u32) -> u32 {
    let mut scan: *mut u8 = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) }; // current string
    let mut r#match: *mut u8 = unsafe { std::mem::zeroed() }; // matched string
    let mut len: i32 = 0; // length of current match
    let strend: *mut u8 = unsafe {
        unsafe { (*s).window }
            .offset((*s).strstart as isize)
            .add(258)
    };
    // The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
    // It is easy to get rid of this optimization if necessary.
    Assert!(
        (unsafe { (*s).hash_bits }) >= 8 && true,
        c"Code too clever".as_ptr() as *mut c_char
    );
    Assert!(
        ((unsafe { (*s).strstart }) as c_ulong)
            <= unsafe { (*s).window_size }.wrapping_sub((262 as c_ulong)),
        c"need lookahead".as_ptr() as *mut c_char
    );
    Assert!(
        cur_match < unsafe { (*s).strstart },
        c"no future".as_ptr() as *mut c_char
    );
    r#match = unsafe { unsafe { (*s).window }.add(cur_match as usize) };
    // Return failure if the match length is less than 2:
    if (((unsafe { *r#match.add(0) }) as u32) as i32) != (((unsafe { *scan.add(0) }) as u32) as i32)
        || (((unsafe { *r#match.add(1) }) as u32) as i32)
            != (((unsafe { *scan.add(1) }) as u32) as i32)
    {
        return 2u32;
    }
    // The check at best_len - 1 can be removed because it will be made
    // again later. (This heuristic is not always a win.)
    // It is not necessary to compare scan[2] and match[2] since they
    // are always equal when the other bytes match, given that
    // the hash keys are equal and that HASH_BITS >= 8.
    let __v0: *mut u8 = scan;
    scan = unsafe { __v0.add(2) };
    let __v2: *mut u8 = r#match;
    r#match = unsafe { __v2.add(2) };
    Assert!(
        (((unsafe { *scan }) as u32) as i32) == (((unsafe { *r#match }) as u32) as i32),
        c"match[2]?".as_ptr() as *mut c_char
    );
    // We check for insufficient lookahead only every 8th comparison;
    // the 256th check will be made at strstart + 258.
    loop {
        let __v5: *mut u8 = scan;
        let __v6: *mut u8 = unsafe { __v5.add(1) };
        scan = __v6;
        let __v7: *mut u8 = r#match;
        let __v8: *mut u8 = unsafe { __v7.add(1) };
        r#match = __v8;
        let __v9: bool =
            if (((unsafe { *__v6 }) as u32) as i32) == (((unsafe { *__v8 }) as u32) as i32) {
                let __v10: *mut u8 = scan;
                let __v11: *mut u8 = unsafe { __v10.add(1) };
                scan = __v11;
                let __v12: *mut u8 = r#match;
                let __v13: *mut u8 = unsafe { __v12.add(1) };
                r#match = __v13;
                (((unsafe { *__v11 }) as u32) as i32) == (((unsafe { *__v13 }) as u32) as i32)
            } else {
                false
            };
        let __v14: bool = if __v9 {
            let __v15: *mut u8 = scan;
            let __v16: *mut u8 = unsafe { __v15.add(1) };
            scan = __v16;
            let __v17: *mut u8 = r#match;
            let __v18: *mut u8 = unsafe { __v17.add(1) };
            r#match = __v18;
            (((unsafe { *__v16 }) as u32) as i32) == (((unsafe { *__v18 }) as u32) as i32)
        } else {
            false
        };
        let __v19: bool = if __v14 {
            let __v20: *mut u8 = scan;
            let __v21: *mut u8 = unsafe { __v20.add(1) };
            scan = __v21;
            let __v22: *mut u8 = r#match;
            let __v23: *mut u8 = unsafe { __v22.add(1) };
            r#match = __v23;
            (((unsafe { *__v21 }) as u32) as i32) == (((unsafe { *__v23 }) as u32) as i32)
        } else {
            false
        };
        let __v24: bool = if __v19 {
            let __v25: *mut u8 = scan;
            let __v26: *mut u8 = unsafe { __v25.add(1) };
            scan = __v26;
            let __v27: *mut u8 = r#match;
            let __v28: *mut u8 = unsafe { __v27.add(1) };
            r#match = __v28;
            (((unsafe { *__v26 }) as u32) as i32) == (((unsafe { *__v28 }) as u32) as i32)
        } else {
            false
        };
        let __v29: bool = if __v24 {
            let __v30: *mut u8 = scan;
            let __v31: *mut u8 = unsafe { __v30.add(1) };
            scan = __v31;
            let __v32: *mut u8 = r#match;
            let __v33: *mut u8 = unsafe { __v32.add(1) };
            r#match = __v33;
            (((unsafe { *__v31 }) as u32) as i32) == (((unsafe { *__v33 }) as u32) as i32)
        } else {
            false
        };
        let __v34: bool = if __v29 {
            let __v35: *mut u8 = scan;
            let __v36: *mut u8 = unsafe { __v35.add(1) };
            scan = __v36;
            let __v37: *mut u8 = r#match;
            let __v38: *mut u8 = unsafe { __v37.add(1) };
            r#match = __v38;
            (((unsafe { *__v36 }) as u32) as i32) == (((unsafe { *__v38 }) as u32) as i32)
        } else {
            false
        };
        let __v39: bool = if __v34 {
            let __v40: *mut u8 = scan;
            let __v41: *mut u8 = unsafe { __v40.add(1) };
            scan = __v41;
            let __v42: *mut u8 = r#match;
            let __v43: *mut u8 = unsafe { __v42.add(1) };
            r#match = __v43;
            (((unsafe { *__v41 }) as u32) as i32) == (((unsafe { *__v43 }) as u32) as i32)
        } else {
            false
        };
        if !(__v39 && scan < strend) {
            break;
        }
    }
    Assert!(
        scan <= unsafe {
            unsafe { (*s).window }
                .add((unsafe { (*s).window_size }.wrapping_sub(1) as u32) as usize)
        },
        c"wild scan".as_ptr() as *mut c_char
    );
    len = 258 - ((unsafe { strend.offset_from(scan as *mut u8) }) as i32);
    if len < 3 {
        return 2u32;
    }
    unsafe {
        (*s).match_start = cur_match;
    }
    if (len as u32) <= unsafe { (*s).lookahead } {
        len as u32
    } else {
        unsafe { (*s).lookahead }
    }
}

#[cfg(all(
    target_pointer_width = "64",
    windows,
    not(feature = "fastest"),
    not(feature = "zlib-debug")
))]
/// Set match_start to the longest match starting at the given string and
/// return its length. Matches shorter or equal to prev_length are discarded,
/// in which case the result is equal to prev_length and match_start is
/// garbage.
/// IN assertions: cur_match is the head of the hash chain for the current
///   string (strstart) and its distance is <= MAX_DIST, and prev_length >= 1
/// OUT assertion: the match length is not greater than s->lookahead.
fn longest_match(s: *mut internal_state, mut cur_match: u32) -> u32 {
    let mut chain_length: u32 = unsafe { (*s).max_chain_length }; // max hash chain length
    let mut scan: *mut u8 = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) }; // current string
    let mut r#match: *mut u8 = unsafe { std::mem::zeroed() }; // matched string
    let mut len: i32 = 0; // length of current match
    let mut best_len: i32 = (unsafe { (*s).prev_length }) as i32; // best match length so far
    let mut nice_match: i32 = unsafe { (*s).nice_match }; // stop if match long enough
    let limit: u32 = if (unsafe { (*s).strstart }) > MAX_DIST!(s) {
        unsafe { (*s).strstart }.wrapping_sub(MAX_DIST!(s))
    } else {
        0
    };
    // Stop when cur_match becomes <= limit. To simplify the code,
    // we prevent matches with the string of window index 0.
    let prev: *mut u16 = unsafe { (*s).prev };
    let wmask: u32 = unsafe { (*s).w_mask };
    let strend: *mut u8 = unsafe {
        unsafe { (*s).window }
            .offset((*s).strstart as isize)
            .add(258)
    };
    let mut scan_end1: u8 = unsafe { *scan.offset((best_len - 1) as isize) };
    let mut scan_end: u8 = unsafe { *scan.offset(best_len as isize) };
    // The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
    // It is easy to get rid of this optimization if necessary.
    // Do not waste too much time if we already have a good match:
    if (unsafe { (*s).prev_length }) >= unsafe { (*s).good_match } {
        chain_length >>= 2i32;
    }
    // Do not look for matches beyond the end of the input. This is necessary
    // to make deflate deterministic.
    if (nice_match as u32) > unsafe { (*s).lookahead } {
        nice_match = (unsafe { (*s).lookahead }) as i32;
    }
    loop {
        r#match = unsafe { unsafe { (*s).window }.add(cur_match as usize) };
        // Skip to next match if the match length cannot increase
        // or if the match length is less than 2.  Note that the checks below
        // for insufficient lookahead only occur occasionally for performance
        // reasons.  Therefore uninitialized memory will be accessed, and
        // conditional jumps will be made that depend on those values.
        // However the length of the match is limited to the lookahead, so
        // the output of deflate is not affected by the uninitialized values.
        let __v2: bool = if (((unsafe { *r#match.offset(best_len as isize) }) as u32) as i32)
            != ((scan_end as u32) as i32)
            || (((unsafe { *r#match.offset((best_len - 1) as isize) }) as u32) as i32)
                != ((scan_end1 as u32) as i32)
            || (((unsafe { *r#match }) as u32) as i32) != (((unsafe { *scan }) as u32) as i32)
        {
            true
        } else {
            let __v3: *mut u8 = r#match;
            let __v4: *mut u8 = unsafe { __v3.add(1) };
            r#match = __v4;
            (((unsafe { *__v4 }) as u32) as i32) != (((unsafe { *scan.add(1) }) as u32) as i32)
        };
        if !__v2 {
            // The check at best_len - 1 can be removed because it will be made
            // again later. (This heuristic is not always a win.)
            // It is not necessary to compare scan[2] and match[2] since they
            // are always equal when the other bytes match, given that
            // the hash keys are equal and that HASH_BITS >= 8.
            let __v6: *mut u8 = scan;
            scan = unsafe { __v6.add(2) };
            let __v8: *mut u8 = r#match;
            r#match = unsafe { __v8.add(1) };
            // We check for insufficient lookahead only every 8th comparison;
            // the 256th check will be made at strstart + 258.
            loop {
                let __v11: *mut u8 = scan;
                let __v12: *mut u8 = unsafe { __v11.add(1) };
                scan = __v12;
                let __v13: *mut u8 = r#match;
                let __v14: *mut u8 = unsafe { __v13.add(1) };
                r#match = __v14;
                let __v15: bool = if (((unsafe { *__v12 }) as u32) as i32)
                    == (((unsafe { *__v14 }) as u32) as i32)
                {
                    let __v16: *mut u8 = scan;
                    let __v17: *mut u8 = unsafe { __v16.add(1) };
                    scan = __v17;
                    let __v18: *mut u8 = r#match;
                    let __v19: *mut u8 = unsafe { __v18.add(1) };
                    r#match = __v19;
                    (((unsafe { *__v17 }) as u32) as i32) == (((unsafe { *__v19 }) as u32) as i32)
                } else {
                    false
                };
                let __v20: bool = if __v15 {
                    let __v21: *mut u8 = scan;
                    let __v22: *mut u8 = unsafe { __v21.add(1) };
                    scan = __v22;
                    let __v23: *mut u8 = r#match;
                    let __v24: *mut u8 = unsafe { __v23.add(1) };
                    r#match = __v24;
                    (((unsafe { *__v22 }) as u32) as i32) == (((unsafe { *__v24 }) as u32) as i32)
                } else {
                    false
                };
                let __v25: bool = if __v20 {
                    let __v26: *mut u8 = scan;
                    let __v27: *mut u8 = unsafe { __v26.add(1) };
                    scan = __v27;
                    let __v28: *mut u8 = r#match;
                    let __v29: *mut u8 = unsafe { __v28.add(1) };
                    r#match = __v29;
                    (((unsafe { *__v27 }) as u32) as i32) == (((unsafe { *__v29 }) as u32) as i32)
                } else {
                    false
                };
                let __v30: bool = if __v25 {
                    let __v31: *mut u8 = scan;
                    let __v32: *mut u8 = unsafe { __v31.add(1) };
                    scan = __v32;
                    let __v33: *mut u8 = r#match;
                    let __v34: *mut u8 = unsafe { __v33.add(1) };
                    r#match = __v34;
                    (((unsafe { *__v32 }) as u32) as i32) == (((unsafe { *__v34 }) as u32) as i32)
                } else {
                    false
                };
                let __v35: bool = if __v30 {
                    let __v36: *mut u8 = scan;
                    let __v37: *mut u8 = unsafe { __v36.add(1) };
                    scan = __v37;
                    let __v38: *mut u8 = r#match;
                    let __v39: *mut u8 = unsafe { __v38.add(1) };
                    r#match = __v39;
                    (((unsafe { *__v37 }) as u32) as i32) == (((unsafe { *__v39 }) as u32) as i32)
                } else {
                    false
                };
                let __v40: bool = if __v35 {
                    let __v41: *mut u8 = scan;
                    let __v42: *mut u8 = unsafe { __v41.add(1) };
                    scan = __v42;
                    let __v43: *mut u8 = r#match;
                    let __v44: *mut u8 = unsafe { __v43.add(1) };
                    r#match = __v44;
                    (((unsafe { *__v42 }) as u32) as i32) == (((unsafe { *__v44 }) as u32) as i32)
                } else {
                    false
                };
                let __v45: bool = if __v40 {
                    let __v46: *mut u8 = scan;
                    let __v47: *mut u8 = unsafe { __v46.add(1) };
                    scan = __v47;
                    let __v48: *mut u8 = r#match;
                    let __v49: *mut u8 = unsafe { __v48.add(1) };
                    r#match = __v49;
                    (((unsafe { *__v47 }) as u32) as i32) == (((unsafe { *__v49 }) as u32) as i32)
                } else {
                    false
                };
                if !(__v45 && scan < strend) {
                    break;
                }
            }
            len = 258 - (((unsafe { strend.offset_from(scan as *mut u8) }) as i64) as i32);
            scan = unsafe { strend.sub(258) };
            if len > best_len {
                unsafe {
                    (*s).match_start = cur_match;
                }
                best_len = len;
                if len >= nice_match {
                    break;
                }
                scan_end1 = unsafe { *scan.offset((best_len - 1) as isize) };
                scan_end = unsafe { *scan.offset(best_len as isize) };
            }
        }
        let __v50: u32 = (unsafe { *prev.add((cur_match & wmask) as usize) }) as u32;
        cur_match = __v50;
        let __v51: bool = if __v50 > limit {
            let __v53: u32 = chain_length.wrapping_sub(1);
            chain_length = __v53;
            __v53 != 0
        } else {
            false
        };
        if !__v51 {
            break;
        }
    }
    if (best_len as u32) <= unsafe { (*s).lookahead } {
        return best_len as u32;
    }
    unsafe { (*s).lookahead }
}

#[cfg(all(
    target_pointer_width = "64",
    windows,
    feature = "fastest",
    not(feature = "zlib-debug")
))]
/// Optimized version for FASTEST only
fn longest_match(s: *mut internal_state, cur_match: u32) -> u32 {
    let mut scan: *mut u8 = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) }; // current string
    let mut r#match: *mut u8 = unsafe { std::mem::zeroed() }; // matched string
    let mut len: i32 = 0; // length of current match
    let strend: *mut u8 = unsafe {
        unsafe { (*s).window }
            .offset((*s).strstart as isize)
            .add(258)
    };
    // The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
    // It is easy to get rid of this optimization if necessary.
    r#match = unsafe { unsafe { (*s).window }.add(cur_match as usize) };
    // Return failure if the match length is less than 2:
    if (((unsafe { *r#match.add(0) }) as u32) as i32) != (((unsafe { *scan.add(0) }) as u32) as i32)
        || (((unsafe { *r#match.add(1) }) as u32) as i32)
            != (((unsafe { *scan.add(1) }) as u32) as i32)
    {
        return 2u32;
    }
    // The check at best_len - 1 can be removed because it will be made
    // again later. (This heuristic is not always a win.)
    // It is not necessary to compare scan[2] and match[2] since they
    // are always equal when the other bytes match, given that
    // the hash keys are equal and that HASH_BITS >= 8.
    let __v0: *mut u8 = scan;
    scan = unsafe { __v0.add(2) };
    let __v2: *mut u8 = r#match;
    r#match = unsafe { __v2.add(2) };
    // We check for insufficient lookahead only every 8th comparison;
    // the 256th check will be made at strstart + 258.
    loop {
        let __v5: *mut u8 = scan;
        let __v6: *mut u8 = unsafe { __v5.add(1) };
        scan = __v6;
        let __v7: *mut u8 = r#match;
        let __v8: *mut u8 = unsafe { __v7.add(1) };
        r#match = __v8;
        let __v9: bool =
            if (((unsafe { *__v6 }) as u32) as i32) == (((unsafe { *__v8 }) as u32) as i32) {
                let __v10: *mut u8 = scan;
                let __v11: *mut u8 = unsafe { __v10.add(1) };
                scan = __v11;
                let __v12: *mut u8 = r#match;
                let __v13: *mut u8 = unsafe { __v12.add(1) };
                r#match = __v13;
                (((unsafe { *__v11 }) as u32) as i32) == (((unsafe { *__v13 }) as u32) as i32)
            } else {
                false
            };
        let __v14: bool = if __v9 {
            let __v15: *mut u8 = scan;
            let __v16: *mut u8 = unsafe { __v15.add(1) };
            scan = __v16;
            let __v17: *mut u8 = r#match;
            let __v18: *mut u8 = unsafe { __v17.add(1) };
            r#match = __v18;
            (((unsafe { *__v16 }) as u32) as i32) == (((unsafe { *__v18 }) as u32) as i32)
        } else {
            false
        };
        let __v19: bool = if __v14 {
            let __v20: *mut u8 = scan;
            let __v21: *mut u8 = unsafe { __v20.add(1) };
            scan = __v21;
            let __v22: *mut u8 = r#match;
            let __v23: *mut u8 = unsafe { __v22.add(1) };
            r#match = __v23;
            (((unsafe { *__v21 }) as u32) as i32) == (((unsafe { *__v23 }) as u32) as i32)
        } else {
            false
        };
        let __v24: bool = if __v19 {
            let __v25: *mut u8 = scan;
            let __v26: *mut u8 = unsafe { __v25.add(1) };
            scan = __v26;
            let __v27: *mut u8 = r#match;
            let __v28: *mut u8 = unsafe { __v27.add(1) };
            r#match = __v28;
            (((unsafe { *__v26 }) as u32) as i32) == (((unsafe { *__v28 }) as u32) as i32)
        } else {
            false
        };
        let __v29: bool = if __v24 {
            let __v30: *mut u8 = scan;
            let __v31: *mut u8 = unsafe { __v30.add(1) };
            scan = __v31;
            let __v32: *mut u8 = r#match;
            let __v33: *mut u8 = unsafe { __v32.add(1) };
            r#match = __v33;
            (((unsafe { *__v31 }) as u32) as i32) == (((unsafe { *__v33 }) as u32) as i32)
        } else {
            false
        };
        let __v34: bool = if __v29 {
            let __v35: *mut u8 = scan;
            let __v36: *mut u8 = unsafe { __v35.add(1) };
            scan = __v36;
            let __v37: *mut u8 = r#match;
            let __v38: *mut u8 = unsafe { __v37.add(1) };
            r#match = __v38;
            (((unsafe { *__v36 }) as u32) as i32) == (((unsafe { *__v38 }) as u32) as i32)
        } else {
            false
        };
        let __v39: bool = if __v34 {
            let __v40: *mut u8 = scan;
            let __v41: *mut u8 = unsafe { __v40.add(1) };
            scan = __v41;
            let __v42: *mut u8 = r#match;
            let __v43: *mut u8 = unsafe { __v42.add(1) };
            r#match = __v43;
            (((unsafe { *__v41 }) as u32) as i32) == (((unsafe { *__v43 }) as u32) as i32)
        } else {
            false
        };
        if !(__v39 && scan < strend) {
            break;
        }
    }
    len = 258 - (((unsafe { strend.offset_from(scan as *mut u8) }) as i64) as i32);
    if len < 3 {
        return 2u32;
    }
    unsafe {
        (*s).match_start = cur_match;
    }
    if (len as u32) <= unsafe { (*s).lookahead } {
        len as u32
    } else {
        unsafe { (*s).lookahead }
    }
}

#[cfg(all(
    target_pointer_width = "64",
    windows,
    not(feature = "fastest"),
    feature = "zlib-debug"
))]
/// Set match_start to the longest match starting at the given string and
/// return its length. Matches shorter or equal to prev_length are discarded,
/// in which case the result is equal to prev_length and match_start is
/// garbage.
/// IN assertions: cur_match is the head of the hash chain for the current
///   string (strstart) and its distance is <= MAX_DIST, and prev_length >= 1
/// OUT assertion: the match length is not greater than s->lookahead.
fn longest_match(s: *mut internal_state, mut cur_match: u32) -> u32 {
    let mut chain_length: u32 = unsafe { (*s).max_chain_length }; // max hash chain length
    let mut scan: *mut u8 = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) }; // current string
    let mut r#match: *mut u8 = unsafe { std::mem::zeroed() }; // matched string
    let mut len: i32 = 0; // length of current match
    let mut best_len: i32 = (unsafe { (*s).prev_length }) as i32; // best match length so far
    let mut nice_match: i32 = unsafe { (*s).nice_match }; // stop if match long enough
    let limit: u32 = if (unsafe { (*s).strstart }) > MAX_DIST!(s) {
        unsafe { (*s).strstart }.wrapping_sub(MAX_DIST!(s))
    } else {
        0
    };
    // Stop when cur_match becomes <= limit. To simplify the code,
    // we prevent matches with the string of window index 0.
    let prev: *mut u16 = unsafe { (*s).prev };
    let wmask: u32 = unsafe { (*s).w_mask };
    let strend: *mut u8 = unsafe {
        unsafe { (*s).window }
            .offset((*s).strstart as isize)
            .add(258)
    };
    let mut scan_end1: u8 = unsafe { *scan.offset((best_len - 1) as isize) };
    let mut scan_end: u8 = unsafe { *scan.offset(best_len as isize) };
    // The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
    // It is easy to get rid of this optimization if necessary.
    Assert!(
        (unsafe { (*s).hash_bits }) >= 8 && true,
        c"Code too clever".as_ptr() as *mut c_char
    );
    // Do not waste too much time if we already have a good match:
    if (unsafe { (*s).prev_length }) >= unsafe { (*s).good_match } {
        chain_length >>= 2i32;
    }
    // Do not look for matches beyond the end of the input. This is necessary
    // to make deflate deterministic.
    if (nice_match as u32) > unsafe { (*s).lookahead } {
        nice_match = (unsafe { (*s).lookahead }) as i32;
    }
    Assert!(
        ((unsafe { (*s).strstart }) as c_ulong)
            <= unsafe { (*s).window_size }.wrapping_sub((262 as c_ulong)),
        c"need lookahead".as_ptr() as *mut c_char
    );
    loop {
        Assert!(
            cur_match < unsafe { (*s).strstart },
            c"no future".as_ptr() as *mut c_char
        );
        r#match = unsafe { unsafe { (*s).window }.add(cur_match as usize) };
        // Skip to next match if the match length cannot increase
        // or if the match length is less than 2.  Note that the checks below
        // for insufficient lookahead only occur occasionally for performance
        // reasons.  Therefore uninitialized memory will be accessed, and
        // conditional jumps will be made that depend on those values.
        // However the length of the match is limited to the lookahead, so
        // the output of deflate is not affected by the uninitialized values.
        let __v2: bool = if (((unsafe { *r#match.offset(best_len as isize) }) as u32) as i32)
            != ((scan_end as u32) as i32)
            || (((unsafe { *r#match.offset((best_len - 1) as isize) }) as u32) as i32)
                != ((scan_end1 as u32) as i32)
            || (((unsafe { *r#match }) as u32) as i32) != (((unsafe { *scan }) as u32) as i32)
        {
            true
        } else {
            let __v3: *mut u8 = r#match;
            let __v4: *mut u8 = unsafe { __v3.add(1) };
            r#match = __v4;
            (((unsafe { *__v4 }) as u32) as i32) != (((unsafe { *scan.add(1) }) as u32) as i32)
        };
        if !__v2 {
            // The check at best_len - 1 can be removed because it will be made
            // again later. (This heuristic is not always a win.)
            // It is not necessary to compare scan[2] and match[2] since they
            // are always equal when the other bytes match, given that
            // the hash keys are equal and that HASH_BITS >= 8.
            let __v6: *mut u8 = scan;
            scan = unsafe { __v6.add(2) };
            let __v8: *mut u8 = r#match;
            r#match = unsafe { __v8.add(1) };
            Assert!(
                (((unsafe { *scan }) as u32) as i32) == (((unsafe { *r#match }) as u32) as i32),
                c"match[2]?".as_ptr() as *mut c_char
            );
            // We check for insufficient lookahead only every 8th comparison;
            // the 256th check will be made at strstart + 258.
            loop {
                let __v11: *mut u8 = scan;
                let __v12: *mut u8 = unsafe { __v11.add(1) };
                scan = __v12;
                let __v13: *mut u8 = r#match;
                let __v14: *mut u8 = unsafe { __v13.add(1) };
                r#match = __v14;
                let __v15: bool = if (((unsafe { *__v12 }) as u32) as i32)
                    == (((unsafe { *__v14 }) as u32) as i32)
                {
                    let __v16: *mut u8 = scan;
                    let __v17: *mut u8 = unsafe { __v16.add(1) };
                    scan = __v17;
                    let __v18: *mut u8 = r#match;
                    let __v19: *mut u8 = unsafe { __v18.add(1) };
                    r#match = __v19;
                    (((unsafe { *__v17 }) as u32) as i32) == (((unsafe { *__v19 }) as u32) as i32)
                } else {
                    false
                };
                let __v20: bool = if __v15 {
                    let __v21: *mut u8 = scan;
                    let __v22: *mut u8 = unsafe { __v21.add(1) };
                    scan = __v22;
                    let __v23: *mut u8 = r#match;
                    let __v24: *mut u8 = unsafe { __v23.add(1) };
                    r#match = __v24;
                    (((unsafe { *__v22 }) as u32) as i32) == (((unsafe { *__v24 }) as u32) as i32)
                } else {
                    false
                };
                let __v25: bool = if __v20 {
                    let __v26: *mut u8 = scan;
                    let __v27: *mut u8 = unsafe { __v26.add(1) };
                    scan = __v27;
                    let __v28: *mut u8 = r#match;
                    let __v29: *mut u8 = unsafe { __v28.add(1) };
                    r#match = __v29;
                    (((unsafe { *__v27 }) as u32) as i32) == (((unsafe { *__v29 }) as u32) as i32)
                } else {
                    false
                };
                let __v30: bool = if __v25 {
                    let __v31: *mut u8 = scan;
                    let __v32: *mut u8 = unsafe { __v31.add(1) };
                    scan = __v32;
                    let __v33: *mut u8 = r#match;
                    let __v34: *mut u8 = unsafe { __v33.add(1) };
                    r#match = __v34;
                    (((unsafe { *__v32 }) as u32) as i32) == (((unsafe { *__v34 }) as u32) as i32)
                } else {
                    false
                };
                let __v35: bool = if __v30 {
                    let __v36: *mut u8 = scan;
                    let __v37: *mut u8 = unsafe { __v36.add(1) };
                    scan = __v37;
                    let __v38: *mut u8 = r#match;
                    let __v39: *mut u8 = unsafe { __v38.add(1) };
                    r#match = __v39;
                    (((unsafe { *__v37 }) as u32) as i32) == (((unsafe { *__v39 }) as u32) as i32)
                } else {
                    false
                };
                let __v40: bool = if __v35 {
                    let __v41: *mut u8 = scan;
                    let __v42: *mut u8 = unsafe { __v41.add(1) };
                    scan = __v42;
                    let __v43: *mut u8 = r#match;
                    let __v44: *mut u8 = unsafe { __v43.add(1) };
                    r#match = __v44;
                    (((unsafe { *__v42 }) as u32) as i32) == (((unsafe { *__v44 }) as u32) as i32)
                } else {
                    false
                };
                let __v45: bool = if __v40 {
                    let __v46: *mut u8 = scan;
                    let __v47: *mut u8 = unsafe { __v46.add(1) };
                    scan = __v47;
                    let __v48: *mut u8 = r#match;
                    let __v49: *mut u8 = unsafe { __v48.add(1) };
                    r#match = __v49;
                    (((unsafe { *__v47 }) as u32) as i32) == (((unsafe { *__v49 }) as u32) as i32)
                } else {
                    false
                };
                if !(__v45 && scan < strend) {
                    break;
                }
            }
            Assert!(
                scan <= unsafe {
                    unsafe { (*s).window }
                        .add((unsafe { (*s).window_size }.wrapping_sub(1) as u32) as usize)
                },
                c"wild scan".as_ptr() as *mut c_char
            );
            len = 258 - (((unsafe { strend.offset_from(scan as *mut u8) }) as i64) as i32);
            scan = unsafe { strend.sub(258) };
            if len > best_len {
                unsafe {
                    (*s).match_start = cur_match;
                }
                best_len = len;
                if len >= nice_match {
                    break;
                }
                scan_end1 = unsafe { *scan.offset((best_len - 1) as isize) };
                scan_end = unsafe { *scan.offset(best_len as isize) };
            }
        }
        let __v50: u32 = (unsafe { *prev.add((cur_match & wmask) as usize) }) as u32;
        cur_match = __v50;
        let __v51: bool = if __v50 > limit {
            let __v53: u32 = chain_length.wrapping_sub(1);
            chain_length = __v53;
            __v53 != 0
        } else {
            false
        };
        if !__v51 {
            break;
        }
    }
    if (best_len as u32) <= unsafe { (*s).lookahead } {
        return best_len as u32;
    }
    unsafe { (*s).lookahead }
}

#[cfg(all(
    target_pointer_width = "64",
    windows,
    feature = "fastest",
    feature = "zlib-debug"
))]
/// Optimized version for FASTEST only
fn longest_match(s: *mut internal_state, cur_match: u32) -> u32 {
    let mut scan: *mut u8 = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) }; // current string
    let mut r#match: *mut u8 = unsafe { std::mem::zeroed() }; // matched string
    let mut len: i32 = 0; // length of current match
    let strend: *mut u8 = unsafe {
        unsafe { (*s).window }
            .offset((*s).strstart as isize)
            .add(258)
    };
    // The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
    // It is easy to get rid of this optimization if necessary.
    Assert!(
        (unsafe { (*s).hash_bits }) >= 8 && true,
        c"Code too clever".as_ptr() as *mut c_char
    );
    Assert!(
        ((unsafe { (*s).strstart }) as c_ulong)
            <= unsafe { (*s).window_size }.wrapping_sub((262 as c_ulong)),
        c"need lookahead".as_ptr() as *mut c_char
    );
    Assert!(
        cur_match < unsafe { (*s).strstart },
        c"no future".as_ptr() as *mut c_char
    );
    r#match = unsafe { unsafe { (*s).window }.add(cur_match as usize) };
    // Return failure if the match length is less than 2:
    if (((unsafe { *r#match.add(0) }) as u32) as i32) != (((unsafe { *scan.add(0) }) as u32) as i32)
        || (((unsafe { *r#match.add(1) }) as u32) as i32)
            != (((unsafe { *scan.add(1) }) as u32) as i32)
    {
        return 2u32;
    }
    // The check at best_len - 1 can be removed because it will be made
    // again later. (This heuristic is not always a win.)
    // It is not necessary to compare scan[2] and match[2] since they
    // are always equal when the other bytes match, given that
    // the hash keys are equal and that HASH_BITS >= 8.
    let __v0: *mut u8 = scan;
    scan = unsafe { __v0.add(2) };
    let __v2: *mut u8 = r#match;
    r#match = unsafe { __v2.add(2) };
    Assert!(
        (((unsafe { *scan }) as u32) as i32) == (((unsafe { *r#match }) as u32) as i32),
        c"match[2]?".as_ptr() as *mut c_char
    );
    // We check for insufficient lookahead only every 8th comparison;
    // the 256th check will be made at strstart + 258.
    loop {
        let __v5: *mut u8 = scan;
        let __v6: *mut u8 = unsafe { __v5.add(1) };
        scan = __v6;
        let __v7: *mut u8 = r#match;
        let __v8: *mut u8 = unsafe { __v7.add(1) };
        r#match = __v8;
        let __v9: bool =
            if (((unsafe { *__v6 }) as u32) as i32) == (((unsafe { *__v8 }) as u32) as i32) {
                let __v10: *mut u8 = scan;
                let __v11: *mut u8 = unsafe { __v10.add(1) };
                scan = __v11;
                let __v12: *mut u8 = r#match;
                let __v13: *mut u8 = unsafe { __v12.add(1) };
                r#match = __v13;
                (((unsafe { *__v11 }) as u32) as i32) == (((unsafe { *__v13 }) as u32) as i32)
            } else {
                false
            };
        let __v14: bool = if __v9 {
            let __v15: *mut u8 = scan;
            let __v16: *mut u8 = unsafe { __v15.add(1) };
            scan = __v16;
            let __v17: *mut u8 = r#match;
            let __v18: *mut u8 = unsafe { __v17.add(1) };
            r#match = __v18;
            (((unsafe { *__v16 }) as u32) as i32) == (((unsafe { *__v18 }) as u32) as i32)
        } else {
            false
        };
        let __v19: bool = if __v14 {
            let __v20: *mut u8 = scan;
            let __v21: *mut u8 = unsafe { __v20.add(1) };
            scan = __v21;
            let __v22: *mut u8 = r#match;
            let __v23: *mut u8 = unsafe { __v22.add(1) };
            r#match = __v23;
            (((unsafe { *__v21 }) as u32) as i32) == (((unsafe { *__v23 }) as u32) as i32)
        } else {
            false
        };
        let __v24: bool = if __v19 {
            let __v25: *mut u8 = scan;
            let __v26: *mut u8 = unsafe { __v25.add(1) };
            scan = __v26;
            let __v27: *mut u8 = r#match;
            let __v28: *mut u8 = unsafe { __v27.add(1) };
            r#match = __v28;
            (((unsafe { *__v26 }) as u32) as i32) == (((unsafe { *__v28 }) as u32) as i32)
        } else {
            false
        };
        let __v29: bool = if __v24 {
            let __v30: *mut u8 = scan;
            let __v31: *mut u8 = unsafe { __v30.add(1) };
            scan = __v31;
            let __v32: *mut u8 = r#match;
            let __v33: *mut u8 = unsafe { __v32.add(1) };
            r#match = __v33;
            (((unsafe { *__v31 }) as u32) as i32) == (((unsafe { *__v33 }) as u32) as i32)
        } else {
            false
        };
        let __v34: bool = if __v29 {
            let __v35: *mut u8 = scan;
            let __v36: *mut u8 = unsafe { __v35.add(1) };
            scan = __v36;
            let __v37: *mut u8 = r#match;
            let __v38: *mut u8 = unsafe { __v37.add(1) };
            r#match = __v38;
            (((unsafe { *__v36 }) as u32) as i32) == (((unsafe { *__v38 }) as u32) as i32)
        } else {
            false
        };
        let __v39: bool = if __v34 {
            let __v40: *mut u8 = scan;
            let __v41: *mut u8 = unsafe { __v40.add(1) };
            scan = __v41;
            let __v42: *mut u8 = r#match;
            let __v43: *mut u8 = unsafe { __v42.add(1) };
            r#match = __v43;
            (((unsafe { *__v41 }) as u32) as i32) == (((unsafe { *__v43 }) as u32) as i32)
        } else {
            false
        };
        if !(__v39 && scan < strend) {
            break;
        }
    }
    Assert!(
        scan <= unsafe {
            unsafe { (*s).window }
                .add((unsafe { (*s).window_size }.wrapping_sub(1) as u32) as usize)
        },
        c"wild scan".as_ptr() as *mut c_char
    );
    len = 258 - (((unsafe { strend.offset_from(scan as *mut u8) }) as i64) as i32);
    if len < 3 {
        return 2u32;
    }
    unsafe {
        (*s).match_start = cur_match;
    }
    if (len as u32) <= unsafe { (*s).lookahead } {
        len as u32
    } else {
        unsafe { (*s).lookahead }
    }
}

#[cfg(feature = "zlib-debug")]
/// Check that the match at match_start is indeed a match.
fn check_match(s: *mut internal_state, mut start: u32, r#match: u32, mut length: i32) {
    // check that the match is indeed a match
    let mut back: *mut u8 = unsafe { unsafe { (*s).window }.offset((r#match as i32) as isize) };
    let mut here: *mut u8 = unsafe { unsafe { (*s).window }.add(start as usize) };
    let mut len: u32 = length as u32;
    if r#match == 4294967295u32 {
        // match starts one byte before the current window -- just compare the
        // subsequent length-1 bytes
        let __v0: *mut u8 = back;
        back = unsafe { __v0.add(1) };
        let __v2: *mut u8 = here;
        here = unsafe { __v2.add(1) };
        len = len.wrapping_sub(1);
    }
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        if (unsafe { memcmp(back as *const (), here as *const (), len as c_ulong) }) != 0 {
            #[cfg(not(target_os = "freebsd"))]
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf(
                    slate_support::stderr(),
                    c" start %u, match %d, length %d\n".as_ptr(),
                    start,
                    r#match as i32,
                    length,
                )
            };
            #[cfg(not(target_os = "freebsd"))]
            {
                unsafe { fflush(std::ptr::null_mut()) };
            }
            loop {
                let __v6: *mut u8 = back;
                back = unsafe { __v6.add(1) };
                let __v8: *mut u8 = here;
                here = unsafe { __v8.add(1) };
                #[cfg(not(target_os = "freebsd"))]
                let _ = std::io::Write::flush(&mut std::io::stdout());
                unsafe {
                    slate_support::fprintf(
                        slate_support::stderr(),
                        c"(%02x %02x)".as_ptr(),
                        (*__v6 as u32) as i32,
                        (*__v8 as u32) as i32,
                    )
                };
                #[cfg(not(target_os = "freebsd"))]
                {
                    unsafe { fflush(std::ptr::null_mut()) };
                }
                let __v12: u32 = len.wrapping_sub(1);
                len = __v12;
                if __v12 == 0 {
                    break;
                }
            }
            unsafe { z_error(c"invalid match".as_ptr() as *mut c_char) };
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        if (unsafe { memcmp(back as *const (), here as *const (), len) }) != 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            #[cfg(unix)]
            {
                unsafe {
                    slate_support::fprintf(
                        slate_support::stderr(),
                        c" start %u, match %d, length %d\n".as_ptr(),
                        start,
                        r#match as i32,
                        length,
                    )
                };
            }
            #[cfg(windows)]
            {
                unsafe {
                    slate_support::fprintf(
                        slate_support::stderr(),
                        c" start %u, match %d, length %d\n".as_ptr(),
                        start,
                        r#match as i32,
                        length,
                    )
                };
            }
            unsafe { fflush(std::ptr::null_mut()) };
            loop {
                let __v1: *mut u8 = back;
                back = unsafe { __v1.add(1) };
                let __v3: *mut u8 = here;
                here = unsafe { __v3.add(1) };
                let _ = std::io::Write::flush(&mut std::io::stdout());
                #[cfg(unix)]
                {
                    unsafe {
                        slate_support::fprintf(
                            slate_support::stderr(),
                            c"(%02x %02x)".as_ptr(),
                            (*__v1 as u32) as i32,
                            (*__v3 as u32) as i32,
                        )
                    };
                }
                #[cfg(windows)]
                {
                    unsafe {
                        slate_support::fprintf(
                            slate_support::stderr(),
                            c"(%02x %02x)".as_ptr(),
                            (*__v1 as u32) as i32,
                            (*__v3 as u32) as i32,
                        )
                    };
                }
                unsafe { fflush(std::ptr::null_mut()) };
                let __v4: u32 = len.wrapping_sub(1);
                len = __v4;
                if __v4 == 0 {
                    break;
                }
            }
            unsafe { z_error(c"invalid match".as_ptr() as *mut c_char) };
        }
    }
    #[cfg(all(target_pointer_width = "64", windows))]
    {
        if (unsafe { memcmp(back as *const (), here as *const (), len as u64) }) != 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe {
                slate_support::fprintf(
                    slate_support::stderr(),
                    c" start %u, match %d, length %d\n".as_ptr(),
                    start,
                    r#match as i32,
                    length,
                )
            };
            unsafe { fflush(std::ptr::null_mut()) };
            loop {
                let __v5: *mut u8 = back;
                back = unsafe { __v5.add(1) };
                let __v7: *mut u8 = here;
                here = unsafe { __v7.add(1) };
                let _ = std::io::Write::flush(&mut std::io::stdout());
                unsafe {
                    slate_support::fprintf(
                        slate_support::stderr(),
                        c"(%02x %02x)".as_ptr(),
                        (*__v5 as u32) as i32,
                        (*__v7 as u32) as i32,
                    )
                };
                unsafe { fflush(std::ptr::null_mut()) };
                let __v9: u32 = len.wrapping_sub(1);
                len = __v9;
                if __v9 == 0 {
                    break;
                }
            }
            unsafe { z_error(c"invalid match".as_ptr() as *mut c_char) };
        }
    }
    if (unsafe { z_verbose }) > 1 {
        #[cfg(not(target_os = "freebsd"))]
        let _ = std::io::Write::flush(&mut std::io::stdout());
        #[cfg(unix)]
        {
            unsafe {
                slate_support::fprintf(
                    slate_support::stderr(),
                    c"\\[%d,%d]".as_ptr(),
                    start.wrapping_sub(r#match),
                    length,
                )
            };
        }
        #[cfg(windows)]
        {
            unsafe {
                slate_support::fprintf(
                    slate_support::stderr(),
                    c"\\[%d,%d]".as_ptr(),
                    start.wrapping_sub(r#match),
                    length,
                )
            };
        }
        #[cfg(not(target_os = "freebsd"))]
        {
            unsafe { fflush(std::ptr::null_mut()) };
        }
        loop {
            #[cfg(not(target_os = "freebsd"))]
            let __v13: u32 = start;
            #[cfg(not(target_os = "freebsd"))]
            {
                start = __v13.wrapping_add(1);
            }
            #[cfg(not(target_os = "freebsd"))]
            let _ = std::io::Write::flush(&mut std::io::stdout());
            #[cfg(not(target_os = "freebsd"))]
            {
                unsafe {
                    slate_support::putc(
                        (*unsafe { (*s).window }.add(__v13 as usize) as u32) as i32,
                        slate_support::stderr(),
                    )
                };
                unsafe { fflush(std::ptr::null_mut()) };
            }
            #[cfg(target_os = "freebsd")]
            let __v10: i32 = if (unsafe { __isthreaded }) == 0 {
                let __v14: u32 = start;
                start = __v14.wrapping_add(1);
                unsafe {
                    slate_support::__sputc(
                        (*unsafe { (*s).window }.add(__v14 as usize) as u32) as i32,
                        slate_support::stderr(),
                    )
                }
            } else {
                let __v16: u32 = start;
                start = __v16.wrapping_add(1);
                unsafe {
                    slate_support::putc(
                        (*unsafe { (*s).window }.add(__v16 as usize) as u32) as i32,
                        slate_support::stderr(),
                    )
                }
            };
            let __v17: i32 = length - 1;
            length = __v17;
            if __v17 == 0 {
                break;
            }
        }
    }
}

// Flush the current block, with given end-of-file flag.
// IN assertion: strstart is set to the end of the current match.
// Same but force premature exit if necessary.
// Maximum stored block length in deflate format (not including header).
// Minimum of a and b.
/// Copy without compression as much as possible from the input stream, return
/// the current block state.
///
/// In case deflateParams() is used to later switch to a non-zero compression
/// level, s->matches (otherwise unused when storing) keeps track of the number
/// of hash table slides to perform. If s->matches is 1, then one hash table
/// slide will be done when switching. If s->matches is 2, the maximum value
/// allowed here, then the hash table will be cleared, since two or more slides
/// is the same as a clear.
///
/// deflate_stored() is written to minimize the number of times an input byte is
/// copied. It is most efficient with large input and output buffers, which
/// maximizes the opportunities to have a single copy from next_in to next_out.
#[cfg_attr(
    all(not(target_os = "macos"), unix),
    unsafe(link_section = ".text.slate_distinct.deflate.deflate_stored")
)]
extern "C-unwind" fn deflate_stored(s: *mut internal_state, flush: i32) -> u32 {
    // Smallest worthy block size when not flushing or finishing. By default
    // this is 32K. This can be as small as 507 bytes for memLevel == 1. For
    // large input and output buffers, the stored block size will be larger.
    let mut min_block: u32 = MIN__2!(unsafe { (*s).pending_buf_size }.wrapping_sub(5), unsafe {
        (*s).w_size
    }) as u32;
    // Copy as many min_block or larger stored blocks directly to next_out as
    // possible. If flushing, copy the remaining available input to next_out as
    // stored blocks, if there is enough space.
    let mut last: i32 = 0;
    let mut len: u32 = 0;
    let mut left: u32 = 0;
    let mut have: u32 = 0;
    let mut used: u32 = unsafe { (*(*s).strm).avail_in };
    loop {
        // Set len to the maximum size block that we can copy directly with the
        // available input data and output space. Set left to how much of that
        // would be copied from what's left in the window.
        len = 65535; // maximum deflate stored block length
        have = ((unsafe { (*s).bi_valid }) as u32).wrapping_add(42) >> 3i32; // bytes in header
        if (unsafe { (*(*s).strm).avail_out }) < have {
            break;
        }
        // need room for header
        // maximum stored block length that will fit in avail_out:
        have = unsafe { (*(*s).strm).avail_out }.wrapping_sub(have);
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            left = ((((unsafe { (*s).strstart }) as c_long) - unsafe { (*s).block_start }) as i32)
                as u32;
        }
        #[cfg(any(target_pointer_width = "32", windows))]
        {
            left = ((unsafe { (*s).strstart }) as c_ulong)
                .wrapping_sub((unsafe { (*s).block_start }) as c_ulong) as u32;
        }
        // window bytes
        if (len as c_ulong)
            > (left as c_ulong).wrapping_add((unsafe { (*(*s).strm).avail_in }) as c_ulong)
        {
            len = left.wrapping_add(unsafe { (*(*s).strm).avail_in });
        }
        // limit len to the input
        if len > have {
            len = have;
        }
        // limit len to the output
        // If the stored block would be less than min_block in length, or if
        // unable to copy all of the available input when flushing, then try
        // copying to the window and the pending buffer instead. Also don't
        // write an empty block when flushing -- deflate() does that.
        if len < min_block
            && (len == 0 && flush != 4
                || flush == 0
                || len != left.wrapping_add(unsafe { (*(*s).strm).avail_in }))
        {
            break;
        }
        // Make a dummy stored block in pending to get the header bytes,
        // including any pending bits. This also updates the debugging counts.
        last = if flush == 4 && len == left.wrapping_add(unsafe { (*(*s).strm).avail_in }) {
            1
        } else {
            0
        };
        unsafe { _tr_stored_block(s, std::ptr::null_mut::<c_char>(), (0 as c_ulong), last) };
        // Replace the lengths in the dummy stored block with len.
        unsafe {
            *unsafe { (*s).pending_buf }.offset(unsafe { (*s).pending }.wrapping_sub(4) as isize) =
                len as u8;
        }
        unsafe {
            *unsafe { (*s).pending_buf }.offset(unsafe { (*s).pending }.wrapping_sub(3) as isize) =
                (len >> 8i32) as u8;
        }
        unsafe {
            *unsafe { (*s).pending_buf }.offset(unsafe { (*s).pending }.wrapping_sub(2) as isize) =
                !len as u8;
        }
        unsafe {
            *unsafe { (*s).pending_buf }.offset(unsafe { (*s).pending }.wrapping_sub(1) as isize) =
                (!len >> 8i32) as u8;
        }
        // Write the stored block header bytes.
        flush_pending(unsafe { (*s).strm });
        #[cfg(feature = "zlib-debug")]
        let __v2: c_ulong = unsafe { (*s).compressed_len };
        #[cfg(feature = "zlib-debug")]
        {
            unsafe {
                (*s).compressed_len = __v2.wrapping_add((len << 3i32) as c_ulong);
            }
        }
        #[cfg(feature = "zlib-debug")]
        let __v5: c_ulong = unsafe { (*s).bits_sent };
        #[cfg(feature = "zlib-debug")]
        {
            unsafe {
                (*s).bits_sent = __v5.wrapping_add((len << 3i32) as c_ulong);
            }
        }
        // Copy uncompressed bytes from the window to next_out.
        if left != 0 {
            if left > len {
                left = len;
            }
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (unsafe { (*s).window }.offset((*s).block_start as isize) as *const ())
                            as *const u8,
                        ((*(*s).strm).next_out as *mut ()) as *mut u8,
                        (left as c_ulong) as usize,
                    )
                };
            }
            #[cfg(target_pointer_width = "32")]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (unsafe { (*s).window }.offset((*s).block_start as isize) as *const ())
                            as *const u8,
                        ((*(*s).strm).next_out as *mut ()) as *mut u8,
                        left as usize,
                    )
                };
            }
            #[cfg(all(target_pointer_width = "64", windows))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (unsafe { (*s).window }.offset((*s).block_start as isize) as *const ())
                            as *const u8,
                        ((*(*s).strm).next_out as *mut ()) as *mut u8,
                        (left as u64) as usize,
                    )
                };
            }
            let __v7: *mut z_stream_s = unsafe { (*s).strm };
            let __v8: *mut u8 = unsafe { (*__v7).next_out };
            unsafe {
                (*__v7).next_out = __v8.add(left as usize);
            }
            let __v10: *mut z_stream_s = unsafe { (*s).strm };
            let __v11: u32 = unsafe { (*__v10).avail_out };
            unsafe {
                (*__v10).avail_out = __v11.wrapping_sub(left);
            }
            let __v13: *mut z_stream_s = unsafe { (*s).strm };
            let __v14: c_ulong = unsafe { (*__v13).total_out };
            unsafe {
                (*__v13).total_out = __v14.wrapping_add(left as c_ulong);
            }
            unsafe {
                #[cfg(all(target_pointer_width = "64", unix))]
                {
                    (*s).block_start = (*s).block_start + (left as c_long);
                }
                #[cfg(any(target_pointer_width = "32", windows))]
                {
                    (*s).block_start =
                        ((*s).block_start as c_ulong).wrapping_add(left as c_ulong) as c_long;
                }
            }
            len = len.wrapping_sub(left);
        }
        // Copy uncompressed bytes directly from next_in to next_out, updating
        // the check value.
        if len != 0 {
            read_buf(unsafe { (*s).strm }, unsafe { (*(*s).strm).next_out }, len);
            let __v21: *mut z_stream_s = unsafe { (*s).strm };
            let __v22: *mut u8 = unsafe { (*__v21).next_out };
            unsafe {
                (*__v21).next_out = __v22.add(len as usize);
            }
            let __v24: *mut z_stream_s = unsafe { (*s).strm };
            let __v25: u32 = unsafe { (*__v24).avail_out };
            unsafe {
                (*__v24).avail_out = __v25.wrapping_sub(len);
            }
            let __v27: *mut z_stream_s = unsafe { (*s).strm };
            let __v28: c_ulong = unsafe { (*__v27).total_out };
            unsafe {
                (*__v27).total_out = __v28.wrapping_add(len as c_ulong);
            }
        }
        if last != 0 {
            break;
        }
    }
    // Update the sliding window with the last s->w_size bytes of the copied
    // data, or append all of the copied data to the existing window if less
    // than s->w_size bytes were copied. Also update the number of bytes to
    // insert in the hash tables, in the event that deflateParams() switches to
    // a non-zero compression level.
    used = used.wrapping_sub(unsafe { (*(*s).strm).avail_in }); // number of input bytes directly copied
    if used != 0 {
        // If any input was used, then no unused input remains in the window,
        // therefore s->block_start == s->strstart.
        if used >= unsafe { (*s).w_size } {
            // supplant the previous history
            unsafe {
                (*s).matches = 2;
            }
            // clear hash
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (unsafe { (*(*s).strm).next_in }.offset(-((*s).w_size as isize))
                            as *const ()) as *const u8,
                        ((*s).window as *mut ()) as *mut u8,
                        ((*s).w_size as c_ulong) as usize,
                    )
                };
            }
            #[cfg(target_pointer_width = "32")]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (unsafe { (*(*s).strm).next_in }.offset(-((*s).w_size as isize))
                            as *const ()) as *const u8,
                        ((*s).window as *mut ()) as *mut u8,
                        (*s).w_size as usize,
                    )
                };
            }
            #[cfg(all(target_pointer_width = "64", windows))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (unsafe { (*(*s).strm).next_in }.offset(-((*s).w_size as isize))
                            as *const ()) as *const u8,
                        ((*s).window as *mut ()) as *mut u8,
                        ((*s).w_size as u64) as usize,
                    )
                };
            }
            unsafe {
                (*s).strstart = (*s).w_size;
            }
            unsafe {
                (*s).insert = (*s).strstart;
            }
        } else {
            if unsafe { (*s).window_size }.wrapping_sub((unsafe { (*s).strstart }) as c_ulong)
                <= (used as c_ulong)
            {
                // Slide the window down.
                let __v33: u32 = unsafe { (*s).strstart };
                unsafe {
                    (*s).strstart = __v33.wrapping_sub((*s).w_size);
                }
                #[cfg(all(target_pointer_width = "64", unix))]
                {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            (unsafe { (*s).window }.offset((*s).w_size as isize) as *const ())
                                as *const u8,
                            ((*s).window as *mut ()) as *mut u8,
                            ((*s).strstart as c_ulong) as usize,
                        )
                    };
                }
                #[cfg(target_pointer_width = "32")]
                {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            (unsafe { (*s).window }.offset((*s).w_size as isize) as *const ())
                                as *const u8,
                            ((*s).window as *mut ()) as *mut u8,
                            (*s).strstart as usize,
                        )
                    };
                }
                #[cfg(all(target_pointer_width = "64", windows))]
                {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            (unsafe { (*s).window }.offset((*s).w_size as isize) as *const ())
                                as *const u8,
                            ((*s).window as *mut ()) as *mut u8,
                            ((*s).strstart as u64) as usize,
                        )
                    };
                }
                if (unsafe { (*s).matches }) < 2 {
                    let __v36: u32 = unsafe { (*s).matches };
                    unsafe {
                        (*s).matches = __v36.wrapping_add(1);
                    }
                }
                // add a pending slide_hash()
                if (unsafe { (*s).insert }) > unsafe { (*s).strstart } {
                    unsafe {
                        (*s).insert = (*s).strstart;
                    }
                }
            }
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (unsafe { (*(*s).strm).next_in }.sub(used as usize) as *const ())
                            as *const u8,
                        (unsafe { (*s).window }.offset((*s).strstart as isize) as *mut ())
                            as *mut u8,
                        (used as c_ulong) as usize,
                    )
                };
            }
            #[cfg(target_pointer_width = "32")]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (unsafe { (*(*s).strm).next_in }.sub(used as usize) as *const ())
                            as *const u8,
                        (unsafe { (*s).window }.offset((*s).strstart as isize) as *mut ())
                            as *mut u8,
                        used as usize,
                    )
                };
            }
            #[cfg(all(target_pointer_width = "64", windows))]
            {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        (unsafe { (*(*s).strm).next_in }.sub(used as usize) as *const ())
                            as *const u8,
                        (unsafe { (*s).window }.offset((*s).strstart as isize) as *mut ())
                            as *mut u8,
                        (used as u64) as usize,
                    )
                };
            }
            let __v39: u32 = unsafe { (*s).strstart };
            unsafe {
                (*s).strstart = __v39.wrapping_add(used);
            }
            let __v42: u32 = unsafe { (*s).insert };
            let __v43: u32 = __v42.wrapping_add(MIN!(
                used,
                unsafe { (*s).w_size }.wrapping_sub(unsafe { (*s).insert })
            ));
            unsafe {
                (*s).insert = __v43;
            }
        }
        unsafe {
            (*s).block_start = (*s).strstart as c_long;
        }
    }
    if (unsafe { (*s).high_water }) < ((unsafe { (*s).strstart }) as c_ulong) {
        unsafe {
            (*s).high_water = (*s).strstart as c_ulong;
        }
    }
    // If the last block was written to next_out, then done.
    if last != 0 {
        unsafe {
            (*s).bi_used = 8;
        }
        return 3;
    }
    // If flushing and all input has been consumed, then done.
    if flush != 0
        && flush != 4
        && (unsafe { (*(*s).strm).avail_in }) == 0
        && ((unsafe { (*s).strstart }) as c_long) == unsafe { (*s).block_start }
    {
        return 1;
    }
    // Fill the window with any remaining input.
    have = unsafe { (*s).window_size }.wrapping_sub((unsafe { (*s).strstart }) as c_ulong) as u32;
    if (unsafe { (*(*s).strm).avail_in }) > have
        && (unsafe { (*s).block_start }) >= ((unsafe { (*s).w_size }) as c_long)
    {
        // Slide the window down.
        unsafe {
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                (*s).block_start = (*s).block_start - ((*s).w_size as c_long);
            }
            #[cfg(any(target_pointer_width = "32", windows))]
            {
                (*s).block_start =
                    ((*s).block_start as c_ulong).wrapping_sub((*s).w_size as c_ulong) as c_long;
            }
        }
        let __v48: u32 = unsafe { (*s).strstart };
        unsafe {
            (*s).strstart = __v48.wrapping_sub((*s).w_size);
        }
        #[cfg(all(target_pointer_width = "64", unix))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    (unsafe { (*s).window }.offset((*s).w_size as isize) as *const ()) as *const u8,
                    ((*s).window as *mut ()) as *mut u8,
                    ((*s).strstart as c_ulong) as usize,
                )
            };
        }
        #[cfg(target_pointer_width = "32")]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    (unsafe { (*s).window }.offset((*s).w_size as isize) as *const ()) as *const u8,
                    ((*s).window as *mut ()) as *mut u8,
                    (*s).strstart as usize,
                )
            };
        }
        #[cfg(all(target_pointer_width = "64", windows))]
        {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    (unsafe { (*s).window }.offset((*s).w_size as isize) as *const ()) as *const u8,
                    ((*s).window as *mut ()) as *mut u8,
                    ((*s).strstart as u64) as usize,
                )
            };
        }
        if (unsafe { (*s).matches }) < 2 {
            let __v51: u32 = unsafe { (*s).matches };
            unsafe {
                (*s).matches = __v51.wrapping_add(1);
            }
        }
        // add a pending slide_hash()
        have = have.wrapping_add(unsafe { (*s).w_size }); // more space now
        if (unsafe { (*s).insert }) > unsafe { (*s).strstart } {
            unsafe {
                (*s).insert = (*s).strstart;
            }
        }
    }
    if have > unsafe { (*(*s).strm).avail_in } {
        have = unsafe { (*(*s).strm).avail_in };
    }
    if have != 0 {
        read_buf(
            unsafe { (*s).strm },
            unsafe { unsafe { (*s).window }.offset((*s).strstart as isize) },
            have,
        );
        let __v56: u32 = unsafe { (*s).strstart };
        unsafe {
            (*s).strstart = __v56.wrapping_add(have);
        }
        let __v59: u32 = unsafe { (*s).insert };
        let __v60: u32 = __v59.wrapping_add(MIN!(
            have,
            unsafe { (*s).w_size }.wrapping_sub(unsafe { (*s).insert })
        ));
        unsafe {
            (*s).insert = __v60;
        }
    }
    if (unsafe { (*s).high_water }) < ((unsafe { (*s).strstart }) as c_ulong) {
        unsafe {
            (*s).high_water = (*s).strstart as c_ulong;
        }
    }
    // There was not enough avail_out to write a complete worthy or flushed
    // stored block to next_out. Write a stored block to pending instead, if we
    // have enough input for a worthy block, or if flushing and there is enough
    // room for the remaining input as a stored block in the pending buffer.
    have = ((unsafe { (*s).bi_valid }) as u32).wrapping_add(42) >> 3i32; // bytes in header
    // maximum stored block length that will fit in pending:
    have = MIN__2!(
        unsafe { (*s).pending_buf_size }.wrapping_sub(have as c_ulong),
        65535i32
    ) as u32;
    min_block = MIN!(have, unsafe { (*s).w_size });
    #[cfg(all(target_pointer_width = "64", unix))]
    {
        left =
            ((((unsafe { (*s).strstart }) as c_long) - unsafe { (*s).block_start }) as i32) as u32;
    }
    #[cfg(any(target_pointer_width = "32", windows))]
    {
        left = ((unsafe { (*s).strstart }) as c_ulong)
            .wrapping_sub((unsafe { (*s).block_start }) as c_ulong) as u32;
    }
    if left >= min_block
        || (left != 0 || flush == 4)
            && flush != 0
            && (unsafe { (*(*s).strm).avail_in }) == 0
            && left <= have
    {
        len = MIN!(left, have);
        last = if flush == 4 && (unsafe { (*(*s).strm).avail_in }) == 0 && len == left {
            1
        } else {
            0
        };
        unsafe {
            _tr_stored_block(
                s,
                ((*s).window as *mut c_char).offset((*s).block_start as isize),
                len as c_ulong,
                last,
            )
        };
        unsafe {
            #[cfg(all(target_pointer_width = "64", unix))]
            {
                (*s).block_start = (*s).block_start + (len as c_long);
            }
            #[cfg(any(target_pointer_width = "32", windows))]
            {
                (*s).block_start =
                    ((*s).block_start as c_ulong).wrapping_add(len as c_ulong) as c_long;
            }
        }
        flush_pending(unsafe { (*s).strm });
    }
    // We've done all we can with the available input and output.
    if last != 0 {
        unsafe {
            (*s).bi_used = 8;
        }
    }
    (if last != 0 { 2i32 } else { 0i32 }) as u32
}

/// Compress as much as possible from the input stream, return the current
/// block state.
/// This function does not perform lazy evaluation of matches and inserts
/// new strings in the dictionary only for unmatched strings or for short
/// matches. It is used only for the fast compression options.
#[cfg_attr(
    all(not(target_os = "macos"), unix),
    unsafe(link_section = ".text.slate_distinct.deflate.deflate_fast")
)]
extern "C-unwind" fn deflate_fast(s: *mut internal_state, flush: i32) -> u32 {
    let mut hash_head: u32 = 0; // head of the hash chain
    let mut bflush: i32 = 0; // set if current block must be flushed
    loop {
        // Make sure that we always have enough lookahead, except
        // at the end of the input file. We need MAX_MATCH bytes
        // for the next match, plus MIN_MATCH bytes to insert the
        // string following the next match.
        if (unsafe { (*s).lookahead }) < 262u32 {
            fill_window(s);
            if (unsafe { (*s).lookahead }) < 262u32 && flush == 0 {
                return 0;
            }
            if (unsafe { (*s).lookahead }) == 0 {
                break;
            }
            // flush the current block
        }
        // Insert the string window[strstart .. strstart + 2] in the
        // dictionary, and set hash_head to the head of the hash chain:
        hash_head = 0;
        if (unsafe { (*s).lookahead }) >= 3 {
            #[cfg(not(feature = "fastest"))]
            {
                INSERT_STRING!(s, unsafe { (*s).strstart }, hash_head);
            }
            #[cfg(feature = "fastest")]
            {
                INSERT_STRING__2!(s, unsafe { (*s).strstart }, hash_head);
            }
        }
        // Find the longest match, discarding those <= prev_length.
        // At this point we have always match_length < MIN_MATCH
        if hash_head != 0 && unsafe { (*s).strstart }.wrapping_sub(hash_head) <= MAX_DIST!(s) {
            // To simplify the code, we prevent matches with the string
            // of window index 0 (in particular we have to avoid a match
            // of the string with itself at the start of the input file).
            unsafe {
                (*s).match_length = longest_match(s, hash_head);
            }
            // longest_match() sets match_start
        }
        if (unsafe { (*s).match_length }) >= 3 {
            #[cfg(feature = "zlib-debug")]
            {
                check_match(
                    s,
                    unsafe { (*s).strstart },
                    unsafe { (*s).match_start },
                    (unsafe { (*s).match_length }) as i32,
                );
                _tr_tally_dist__3!(
                    s,
                    unsafe { (*s).strstart }.wrapping_sub(unsafe { (*s).match_start }),
                    unsafe { (*s).match_length }.wrapping_sub(3),
                    bflush
                );
            }
            #[cfg(all(not(feature = "lit-mem"), not(feature = "zlib-debug")))]
            {
                _tr_tally_dist!(
                    s,
                    unsafe { (*s).strstart }.wrapping_sub(unsafe { (*s).match_start }),
                    unsafe { (*s).match_length }.wrapping_sub(3),
                    bflush
                );
            }
            #[cfg(all(feature = "lit-mem", not(feature = "zlib-debug")))]
            {
                _tr_tally_dist__2!(
                    s,
                    unsafe { (*s).strstart }.wrapping_sub(unsafe { (*s).match_start }),
                    unsafe { (*s).match_length }.wrapping_sub(3),
                    bflush
                );
            }
            let __v6: u32 = unsafe { (*s).lookahead };
            unsafe {
                (*s).lookahead = __v6.wrapping_sub((*s).match_length);
            }
            #[cfg(not(feature = "fastest"))]
            {
                if (unsafe { (*s).match_length }) <= unsafe { (*s).max_lazy_match }
                    && (unsafe { (*s).lookahead }) >= 3
                {
                    let __v9: u32 = unsafe { (*s).match_length };
                    unsafe {
                        (*s).match_length = __v9.wrapping_sub(1);
                    }
                    // string at strstart already in table
                    loop {
                        let __v12: u32 = unsafe { (*s).strstart };
                        unsafe {
                            (*s).strstart = __v12.wrapping_add(1);
                        }
                        INSERT_STRING!(s, unsafe { (*s).strstart }, hash_head);
                        // strstart never exceeds WSIZE-MAX_MATCH, so there are
                        // always MIN_MATCH bytes ahead.
                        let __v20: u32 = unsafe { (*s).match_length };
                        let __v21: u32 = __v20.wrapping_sub(1);
                        unsafe {
                            (*s).match_length = __v21;
                        }
                        if __v21 == 0 {
                            break;
                        }
                    }
                    let __v23: u32 = unsafe { (*s).strstart };
                    unsafe {
                        (*s).strstart = __v23.wrapping_add(1);
                    }
                } else {
                    let __v26: u32 = unsafe { (*s).strstart };
                    unsafe {
                        (*s).strstart = __v26.wrapping_add((*s).match_length);
                    }
                    unsafe {
                        (*s).match_length = 0;
                    }
                    unsafe {
                        (*s).ins_h = *unsafe { (*s).window }.offset((*s).strstart as isize) as u32;
                    }
                    UPDATE_HASH!(s, (*s).ins_h, unsafe {
                        *unsafe { (*s).window }
                            .offset(unsafe { (*s).strstart }.wrapping_add(1) as isize)
                    });
                    // If lookahead < MIN_MATCH, ins_h is garbage, but it does not
                    // matter since it will be recomputed at next deflate call.
                }
            }
            #[cfg(feature = "fastest")]
            let __v8: u32 = unsafe { (*s).strstart };
            #[cfg(feature = "fastest")]
            {
                unsafe {
                    (*s).strstart = __v8.wrapping_add((*s).match_length);
                }
                unsafe {
                    (*s).match_length = 0;
                }
                unsafe {
                    (*s).ins_h = *unsafe { (*s).window }.offset((*s).strstart as isize) as u32;
                }
                UPDATE_HASH!(s, (*s).ins_h, unsafe {
                    *unsafe { (*s).window }
                        .offset(unsafe { (*s).strstart }.wrapping_add(1) as isize)
                });
            }
        } else {
            // No match, output a literal byte
            #[cfg(feature = "zlib-debug")]
            {
                Tracevv__2!(
                    slate_support::stderr(),
                    c"%c".as_ptr() as *mut c_char,
                    unsafe { *unsafe { (*s).window }.offset((*s).strstart as isize) }
                );
                _tr_tally_lit__2!(
                    s,
                    unsafe { *unsafe { (*s).window }.offset((*s).strstart as isize) },
                    bflush
                );
            }
            #[cfg(all(not(feature = "lit-mem"), not(feature = "zlib-debug")))]
            {
                _tr_tally_lit!(
                    s,
                    unsafe { *unsafe { (*s).window }.offset((*s).strstart as isize) },
                    bflush
                );
            }
            #[cfg(all(feature = "lit-mem", not(feature = "zlib-debug")))]
            {
                _tr_tally_lit__3!(
                    s,
                    unsafe { *unsafe { (*s).window }.offset((*s).strstart as isize) },
                    bflush
                );
            }
            let __v29: u32 = unsafe { (*s).lookahead };
            unsafe {
                (*s).lookahead = __v29.wrapping_sub(1);
            }
            let __v32: u32 = unsafe { (*s).strstart };
            unsafe {
                (*s).strstart = __v32.wrapping_add(1);
            }
        }
        if bflush != 0 {
            #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
            {
                FLUSH_BLOCK!(s, 0i32);
            }
            #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
            {
                FLUSH_BLOCK__2!(s, 0i32);
            }
            #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
            {
                FLUSH_BLOCK__4!(s, 0i32);
            }
            #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
            {
                FLUSH_BLOCK__3!(s, 0i32);
            }
        }
    }
    unsafe {
        (*s).insert = if (*s).strstart < 2u32 {
            (*s).strstart
        } else {
            2u32
        };
    }
    if flush == 4 {
        #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
        {
            FLUSH_BLOCK!(s, 1i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
        {
            FLUSH_BLOCK__2!(s, 1i32);
        }
        #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__4!(s, 1i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__3!(s, 1i32);
        }
        return 3;
    }
    if (unsafe { (*s).sym_next }) != 0 {
        #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
        {
            FLUSH_BLOCK!(s, 0i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
        {
            FLUSH_BLOCK__2!(s, 0i32);
        }
        #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__4!(s, 0i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__3!(s, 0i32);
        }
    }
    1
}

#[cfg(not(feature = "fastest"))]
/// Same as above, but achieves better compression. We use a lazy
/// evaluation for matches: a match is finally adopted only if there is
/// no better match at the next window position.
#[cfg_attr(
    all(not(target_os = "macos"), unix),
    unsafe(link_section = ".text.slate_distinct.deflate.deflate_slow")
)]
extern "C-unwind" fn deflate_slow(s: *mut internal_state, flush: i32) -> u32 {
    let mut hash_head: u32 = 0; // head of hash chain
    let mut bflush: i32 = 0; // set if current block must be flushed
    // Process the input block.
    loop {
        // Make sure that we always have enough lookahead, except
        // at the end of the input file. We need MAX_MATCH bytes
        // for the next match, plus MIN_MATCH bytes to insert the
        // string following the next match.
        if (unsafe { (*s).lookahead }) < 262u32 {
            fill_window(s);
            if (unsafe { (*s).lookahead }) < 262u32 && flush == 0 {
                return 0;
            }
            if (unsafe { (*s).lookahead }) == 0 {
                break;
            }
            // flush the current block
        }
        // Insert the string window[strstart .. strstart + 2] in the
        // dictionary, and set hash_head to the head of the hash chain:
        hash_head = 0;
        if (unsafe { (*s).lookahead }) >= 3 {
            INSERT_STRING!(s, unsafe { (*s).strstart }, hash_head);
        }
        // Find the longest match, discarding those <= prev_length.
        unsafe {
            (*s).prev_length = (*s).match_length;
        }
        unsafe {
            (*s).prev_match = (*s).match_start;
        }
        unsafe {
            (*s).match_length = 2u32;
        }
        if hash_head != 0
            && (unsafe { (*s).prev_length }) < unsafe { (*s).max_lazy_match }
            && unsafe { (*s).strstart }.wrapping_sub(hash_head) <= MAX_DIST!(s)
        {
            // To simplify the code, we prevent matches with the string
            // of window index 0 (in particular we have to avoid a match
            // of the string with itself at the start of the input file).
            unsafe {
                (*s).match_length = longest_match(s, hash_head);
            }
            // longest_match() sets match_start
            if (unsafe { (*s).match_length }) <= 5
                && ((unsafe { (*s).strategy }) == 1
                    || (unsafe { (*s).match_length }) == 3
                        && unsafe { (*s).strstart }.wrapping_sub(unsafe { (*s).match_start })
                            > 4096)
            {
                // If prev_match is also MIN_MATCH, match_start is garbage
                // but we will ignore the current match anyway.
                unsafe {
                    (*s).match_length = 2u32;
                }
            }
        }
        // If there was a match at the previous step and the current
        // match is not better, output the previous match:
        if (unsafe { (*s).prev_length }) >= 3
            && (unsafe { (*s).match_length }) <= unsafe { (*s).prev_length }
        {
            let max_insert: u32 = unsafe { (*s).strstart }
                .wrapping_add(unsafe { (*s).lookahead })
                .wrapping_sub(3);
            // Do not insert strings in hash table beyond this.
            #[cfg(feature = "zlib-debug")]
            {
                check_match(
                    s,
                    unsafe { (*s).strstart }.wrapping_sub(1),
                    unsafe { (*s).prev_match },
                    (unsafe { (*s).prev_length }) as i32,
                );
                _tr_tally_dist__3!(
                    s,
                    unsafe { (*s).strstart }
                        .wrapping_sub(1)
                        .wrapping_sub(unsafe { (*s).prev_match }),
                    unsafe { (*s).prev_length }.wrapping_sub(3),
                    bflush
                );
            }
            #[cfg(all(not(feature = "lit-mem"), not(feature = "zlib-debug")))]
            {
                _tr_tally_dist!(
                    s,
                    unsafe { (*s).strstart }
                        .wrapping_sub(1)
                        .wrapping_sub(unsafe { (*s).prev_match }),
                    unsafe { (*s).prev_length }.wrapping_sub(3),
                    bflush
                );
            }
            #[cfg(all(feature = "lit-mem", not(feature = "zlib-debug")))]
            {
                _tr_tally_dist__2!(
                    s,
                    unsafe { (*s).strstart }
                        .wrapping_sub(1)
                        .wrapping_sub(unsafe { (*s).prev_match }),
                    unsafe { (*s).prev_length }.wrapping_sub(3),
                    bflush
                );
            }
            // Insert in hash table all strings up to the end of the match.
            // strstart - 1 and strstart are already inserted. If there is not
            // enough lookahead, the last two strings are not inserted in
            // the hash table.
            let __v7: u32 = unsafe { (*s).lookahead };
            unsafe {
                (*s).lookahead = __v7.wrapping_sub(unsafe { (*s).prev_length }.wrapping_sub(1));
            }
            let __v10: u32 = unsafe { (*s).prev_length };
            unsafe {
                (*s).prev_length = __v10.wrapping_sub(2);
            }
            loop {
                let __v13: u32 = unsafe { (*s).strstart };
                let __v14: u32 = __v13.wrapping_add(1);
                unsafe {
                    (*s).strstart = __v14;
                }
                if __v14 <= max_insert {
                    INSERT_STRING!(s, unsafe { (*s).strstart }, hash_head);
                }
                let __v21: u32 = unsafe { (*s).prev_length };
                let __v22: u32 = __v21.wrapping_sub(1);
                unsafe {
                    (*s).prev_length = __v22;
                }
                if __v22 == 0 {
                    break;
                }
            }
            unsafe {
                (*s).match_available = 0;
            }
            unsafe {
                (*s).match_length = 2u32;
            }
            let __v24: u32 = unsafe { (*s).strstart };
            unsafe {
                (*s).strstart = __v24.wrapping_add(1);
            }
            if bflush != 0 {
                #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
                {
                    FLUSH_BLOCK!(s, 0i32);
                }
                #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
                {
                    FLUSH_BLOCK__2!(s, 0i32);
                }
                #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
                {
                    FLUSH_BLOCK__4!(s, 0i32);
                }
                #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
                {
                    FLUSH_BLOCK__3!(s, 0i32);
                }
            }
        } else {
            if (unsafe { (*s).match_available }) != 0 {
                // If there was no match at the previous position, output a
                // single literal. If there was a match but the current match
                // is longer, truncate the previous match to a single literal.
                #[cfg(feature = "zlib-debug")]
                {
                    Tracevv__2!(
                        slate_support::stderr(),
                        c"%c".as_ptr() as *mut c_char,
                        unsafe {
                            *unsafe { (*s).window }
                                .offset(unsafe { (*s).strstart }.wrapping_sub(1) as isize)
                        }
                    );
                    _tr_tally_lit__2!(
                        s,
                        unsafe {
                            *unsafe { (*s).window }
                                .offset(unsafe { (*s).strstart }.wrapping_sub(1) as isize)
                        },
                        bflush
                    );
                }
                #[cfg(all(not(feature = "lit-mem"), not(feature = "zlib-debug")))]
                {
                    _tr_tally_lit!(
                        s,
                        unsafe {
                            *unsafe { (*s).window }
                                .offset(unsafe { (*s).strstart }.wrapping_sub(1) as isize)
                        },
                        bflush
                    );
                }
                #[cfg(all(feature = "lit-mem", not(feature = "zlib-debug")))]
                {
                    _tr_tally_lit__3!(
                        s,
                        unsafe {
                            *unsafe { (*s).window }
                                .offset(unsafe { (*s).strstart }.wrapping_sub(1) as isize)
                        },
                        bflush
                    );
                }
                if bflush != 0 {
                    #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
                    {
                        FLUSH_BLOCK_ONLY__2!(s, 0i32);
                    }
                    #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
                    {
                        FLUSH_BLOCK_ONLY__4!(s, 0i32);
                    }
                    #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
                    {
                        FLUSH_BLOCK_ONLY__3!(s, 0i32);
                    }
                    #[cfg(all(
                        any(target_pointer_width = "32", windows),
                        not(feature = "zlib-debug")
                    ))]
                    {
                        FLUSH_BLOCK_ONLY!(s, 0i32);
                    }
                }
                let __v27: u32 = unsafe { (*s).strstart };
                unsafe {
                    (*s).strstart = __v27.wrapping_add(1);
                }
                let __v30: u32 = unsafe { (*s).lookahead };
                unsafe {
                    (*s).lookahead = __v30.wrapping_sub(1);
                }
                if (unsafe { (*(*s).strm).avail_out }) == 0 {
                    return 0;
                }
            } else {
                // There is no previous match to compare with, wait for
                // the next step to decide.
                unsafe {
                    (*s).match_available = 1;
                }
                let __v33: u32 = unsafe { (*s).strstart };
                unsafe {
                    (*s).strstart = __v33.wrapping_add(1);
                }
                let __v36: u32 = unsafe { (*s).lookahead };
                unsafe {
                    (*s).lookahead = __v36.wrapping_sub(1);
                }
            }
        }
    }
    #[cfg(feature = "zlib-debug")]
    {
        Assert!(flush != 0, c"no flush?".as_ptr() as *mut c_char);
    }
    if (unsafe { (*s).match_available }) != 0 {
        #[cfg(feature = "zlib-debug")]
        {
            Tracevv__2!(
                slate_support::stderr(),
                c"%c".as_ptr() as *mut c_char,
                unsafe {
                    *unsafe { (*s).window }
                        .offset(unsafe { (*s).strstart }.wrapping_sub(1) as isize)
                }
            );
            _tr_tally_lit__2!(
                s,
                unsafe {
                    *unsafe { (*s).window }
                        .offset(unsafe { (*s).strstart }.wrapping_sub(1) as isize)
                },
                bflush
            );
        }
        #[cfg(all(not(feature = "lit-mem"), not(feature = "zlib-debug")))]
        {
            _tr_tally_lit!(
                s,
                unsafe {
                    *unsafe { (*s).window }
                        .offset(unsafe { (*s).strstart }.wrapping_sub(1) as isize)
                },
                bflush
            );
        }
        #[cfg(all(feature = "lit-mem", not(feature = "zlib-debug")))]
        {
            _tr_tally_lit__3!(
                s,
                unsafe {
                    *unsafe { (*s).window }
                        .offset(unsafe { (*s).strstart }.wrapping_sub(1) as isize)
                },
                bflush
            );
        }
        unsafe {
            (*s).match_available = 0;
        }
    }
    unsafe {
        (*s).insert = if (*s).strstart < 2u32 {
            (*s).strstart
        } else {
            2u32
        };
    }
    if flush == 4 {
        #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
        {
            FLUSH_BLOCK!(s, 1i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
        {
            FLUSH_BLOCK__2!(s, 1i32);
        }
        #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__4!(s, 1i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__3!(s, 1i32);
        }
        return 3;
    }
    if (unsafe { (*s).sym_next }) != 0 {
        #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
        {
            FLUSH_BLOCK!(s, 0i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
        {
            FLUSH_BLOCK__2!(s, 0i32);
        }
        #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__4!(s, 0i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__3!(s, 0i32);
        }
    }
    1
}

/// For Z_RLE, simply look for runs of bytes, generate matches only of distance
/// one.  Do not maintain a hash table.  (It will be regenerated if this run of
/// deflate switches away from Z_RLE.)
fn deflate_rle(s: *mut internal_state, flush: i32) -> u32 {
    let mut bflush: i32 = 0; // set if current block must be flushed
    let mut prev: u32 = 0; // byte at distance one to match
    let mut scan: *mut u8 = unsafe { std::mem::zeroed() };
    loop {
        // Make sure that we always have enough lookahead, except
        // at the end of the input file. We need MAX_MATCH bytes
        // for the longest run, plus one for the unrolled loop.
        if (unsafe { (*s).lookahead }) <= 258 {
            fill_window(s);
            if (unsafe { (*s).lookahead }) <= 258 && flush == 0 {
                return 0;
            }
            if (unsafe { (*s).lookahead }) == 0 {
                break;
            }
            // flush the current block
        }
        // See how many times the previous byte repeats
        unsafe {
            (*s).match_length = 0;
        }
        if (unsafe { (*s).lookahead }) >= 3 && (unsafe { (*s).strstart }) > 0 {
            scan = unsafe { unsafe { (*s).window }.offset((*s).strstart as isize).sub(1) };
            prev = (unsafe { *scan }) as u32;
            let __v1: *mut u8 = scan;
            let __v2: *mut u8 = unsafe { __v1.add(1) };
            scan = __v2;
            let __v3: bool = if prev == ((((unsafe { *__v2 }) as u32) as i32) as u32) {
                let __v4: *mut u8 = scan;
                let __v5: *mut u8 = unsafe { __v4.add(1) };
                scan = __v5;
                prev == ((((unsafe { *__v5 }) as u32) as i32) as u32)
            } else {
                false
            };
            let __v6: bool = if __v3 {
                let __v7: *mut u8 = scan;
                let __v8: *mut u8 = unsafe { __v7.add(1) };
                scan = __v8;
                prev == ((((unsafe { *__v8 }) as u32) as i32) as u32)
            } else {
                false
            };
            if __v6 {
                // scan goes up to strend for length of run
                let strend: *mut u8 = unsafe {
                    unsafe { (*s).window }
                        .offset((*s).strstart as isize)
                        .add(258)
                };
                loop {
                    let __v10: *mut u8 = scan;
                    let __v11: *mut u8 = unsafe { __v10.add(1) };
                    scan = __v11;
                    let __v12: bool = if prev == ((((unsafe { *__v11 }) as u32) as i32) as u32) {
                        let __v13: *mut u8 = scan;
                        let __v14: *mut u8 = unsafe { __v13.add(1) };
                        scan = __v14;
                        prev == ((((unsafe { *__v14 }) as u32) as i32) as u32)
                    } else {
                        false
                    };
                    let __v15: bool = if __v12 {
                        let __v16: *mut u8 = scan;
                        let __v17: *mut u8 = unsafe { __v16.add(1) };
                        scan = __v17;
                        prev == ((((unsafe { *__v17 }) as u32) as i32) as u32)
                    } else {
                        false
                    };
                    let __v18: bool = if __v15 {
                        let __v19: *mut u8 = scan;
                        let __v20: *mut u8 = unsafe { __v19.add(1) };
                        scan = __v20;
                        prev == ((((unsafe { *__v20 }) as u32) as i32) as u32)
                    } else {
                        false
                    };
                    let __v21: bool = if __v18 {
                        let __v22: *mut u8 = scan;
                        let __v23: *mut u8 = unsafe { __v22.add(1) };
                        scan = __v23;
                        prev == ((((unsafe { *__v23 }) as u32) as i32) as u32)
                    } else {
                        false
                    };
                    let __v24: bool = if __v21 {
                        let __v25: *mut u8 = scan;
                        let __v26: *mut u8 = unsafe { __v25.add(1) };
                        scan = __v26;
                        prev == ((((unsafe { *__v26 }) as u32) as i32) as u32)
                    } else {
                        false
                    };
                    let __v27: bool = if __v24 {
                        let __v28: *mut u8 = scan;
                        let __v29: *mut u8 = unsafe { __v28.add(1) };
                        scan = __v29;
                        prev == ((((unsafe { *__v29 }) as u32) as i32) as u32)
                    } else {
                        false
                    };
                    let __v30: bool = if __v27 {
                        let __v31: *mut u8 = scan;
                        let __v32: *mut u8 = unsafe { __v31.add(1) };
                        scan = __v32;
                        prev == ((((unsafe { *__v32 }) as u32) as i32) as u32)
                    } else {
                        false
                    };
                    if !(__v30 && scan < strend) {
                        break;
                    }
                }
                unsafe {
                    #[cfg(all(target_pointer_width = "64", unix))]
                    {
                        (*s).match_length = 258u32.wrapping_sub(
                            ((strend.offset_from(scan as *mut u8) as c_long) as i32) as u32,
                        );
                    }
                    #[cfg(target_pointer_width = "32")]
                    {
                        (*s).match_length = 258u32
                            .wrapping_sub((strend.offset_from(scan as *mut u8) as i32) as u32);
                    }
                    #[cfg(all(target_pointer_width = "64", windows))]
                    {
                        (*s).match_length = 258u32.wrapping_sub(
                            ((strend.offset_from(scan as *mut u8) as i64) as i32) as u32,
                        );
                    }
                }
                if (unsafe { (*s).match_length }) > unsafe { (*s).lookahead } {
                    unsafe {
                        (*s).match_length = (*s).lookahead;
                    }
                }
            }
            #[cfg(feature = "zlib-debug")]
            {
                Assert!(
                    scan <= unsafe {
                        unsafe { (*s).window }
                            .add((unsafe { (*s).window_size }.wrapping_sub(1) as u32) as usize)
                    },
                    c"wild scan".as_ptr() as *mut c_char
                );
            }
        }
        // Emit match if have run of MIN_MATCH or longer, else emit literal
        if (unsafe { (*s).match_length }) >= 3 {
            #[cfg(feature = "zlib-debug")]
            {
                check_match(
                    s,
                    unsafe { (*s).strstart },
                    unsafe { (*s).strstart }.wrapping_sub(1),
                    (unsafe { (*s).match_length }) as i32,
                );
                _tr_tally_dist__4!(
                    s,
                    1i32,
                    unsafe { (*s).match_length }.wrapping_sub(3),
                    bflush
                );
            }
            #[cfg(all(not(feature = "lit-mem"), not(feature = "zlib-debug")))]
            {
                _tr_tally_dist!(
                    s,
                    1i32,
                    unsafe { (*s).match_length }.wrapping_sub(3),
                    bflush
                );
            }
            #[cfg(all(feature = "lit-mem", not(feature = "zlib-debug")))]
            {
                _tr_tally_dist__2!(
                    s,
                    1i32,
                    unsafe { (*s).match_length }.wrapping_sub(3),
                    bflush
                );
            }
            let __v34: u32 = unsafe { (*s).lookahead };
            unsafe {
                (*s).lookahead = __v34.wrapping_sub((*s).match_length);
            }
            let __v37: u32 = unsafe { (*s).strstart };
            unsafe {
                (*s).strstart = __v37.wrapping_add((*s).match_length);
            }
            unsafe {
                (*s).match_length = 0;
            }
        } else {
            // No match, output a literal byte
            #[cfg(feature = "zlib-debug")]
            {
                Tracevv__2!(
                    slate_support::stderr(),
                    c"%c".as_ptr() as *mut c_char,
                    unsafe { *unsafe { (*s).window }.offset((*s).strstart as isize) }
                );
                _tr_tally_lit__2!(
                    s,
                    unsafe { *unsafe { (*s).window }.offset((*s).strstart as isize) },
                    bflush
                );
            }
            #[cfg(all(not(feature = "lit-mem"), not(feature = "zlib-debug")))]
            {
                _tr_tally_lit!(
                    s,
                    unsafe { *unsafe { (*s).window }.offset((*s).strstart as isize) },
                    bflush
                );
            }
            #[cfg(all(feature = "lit-mem", not(feature = "zlib-debug")))]
            {
                _tr_tally_lit__3!(
                    s,
                    unsafe { *unsafe { (*s).window }.offset((*s).strstart as isize) },
                    bflush
                );
            }
            let __v40: u32 = unsafe { (*s).lookahead };
            unsafe {
                (*s).lookahead = __v40.wrapping_sub(1);
            }
            let __v43: u32 = unsafe { (*s).strstart };
            unsafe {
                (*s).strstart = __v43.wrapping_add(1);
            }
        }
        if bflush != 0 {
            #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
            {
                FLUSH_BLOCK!(s, 0i32);
            }
            #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
            {
                FLUSH_BLOCK__2!(s, 0i32);
            }
            #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
            {
                FLUSH_BLOCK__4!(s, 0i32);
            }
            #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
            {
                FLUSH_BLOCK__3!(s, 0i32);
            }
        }
    }
    unsafe {
        (*s).insert = 0;
    }
    if flush == 4 {
        #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
        {
            FLUSH_BLOCK!(s, 1i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
        {
            FLUSH_BLOCK__2!(s, 1i32);
        }
        #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__4!(s, 1i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__3!(s, 1i32);
        }
        return 3;
    }
    if (unsafe { (*s).sym_next }) != 0 {
        #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
        {
            FLUSH_BLOCK!(s, 0i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
        {
            FLUSH_BLOCK__2!(s, 0i32);
        }
        #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__4!(s, 0i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__3!(s, 0i32);
        }
    }
    1
}

/// For Z_HUFFMAN_ONLY, do not look for matches.  Do not maintain a hash table.
/// (It will be regenerated if this run of deflate switches away from Huffman.)
fn deflate_huff(s: *mut internal_state, flush: i32) -> u32 {
    let mut bflush: i32 = 0; // set if current block must be flushed
    loop {
        // Make sure that we have a literal to write.
        if (unsafe { (*s).lookahead }) == 0 {
            fill_window(s);
            if (unsafe { (*s).lookahead }) == 0 {
                if flush == 0 {
                    return 0;
                }
                break; // flush the current block
            }
        }
        // Output a literal byte
        unsafe {
            (*s).match_length = 0;
        }
        #[cfg(feature = "zlib-debug")]
        {
            Tracevv__2!(
                slate_support::stderr(),
                c"%c".as_ptr() as *mut c_char,
                unsafe { *unsafe { (*s).window }.offset((*s).strstart as isize) }
            );
            _tr_tally_lit__2!(
                s,
                unsafe { *unsafe { (*s).window }.offset((*s).strstart as isize) },
                bflush
            );
        }
        #[cfg(all(not(feature = "lit-mem"), not(feature = "zlib-debug")))]
        {
            _tr_tally_lit!(
                s,
                unsafe { *unsafe { (*s).window }.offset((*s).strstart as isize) },
                bflush
            );
        }
        #[cfg(all(feature = "lit-mem", not(feature = "zlib-debug")))]
        {
            _tr_tally_lit__3!(
                s,
                unsafe { *unsafe { (*s).window }.offset((*s).strstart as isize) },
                bflush
            );
        }
        let __v2: u32 = unsafe { (*s).lookahead };
        unsafe {
            (*s).lookahead = __v2.wrapping_sub(1);
        }
        let __v5: u32 = unsafe { (*s).strstart };
        unsafe {
            (*s).strstart = __v5.wrapping_add(1);
        }
        if bflush != 0 {
            #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
            {
                FLUSH_BLOCK!(s, 0i32);
            }
            #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
            {
                FLUSH_BLOCK__2!(s, 0i32);
            }
            #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
            {
                FLUSH_BLOCK__4!(s, 0i32);
            }
            #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
            {
                FLUSH_BLOCK__3!(s, 0i32);
            }
        }
    }
    unsafe {
        (*s).insert = 0;
    }
    if flush == 4 {
        #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
        {
            FLUSH_BLOCK!(s, 1i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
        {
            FLUSH_BLOCK__2!(s, 1i32);
        }
        #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__4!(s, 1i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__3!(s, 1i32);
        }
        return 3;
    }
    if (unsafe { (*s).sym_next }) != 0 {
        #[cfg(all(target_pointer_width = "64", unix, feature = "zlib-debug"))]
        {
            FLUSH_BLOCK!(s, 0i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), feature = "zlib-debug"))]
        {
            FLUSH_BLOCK__2!(s, 0i32);
        }
        #[cfg(all(target_pointer_width = "64", unix, not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__4!(s, 0i32);
        }
        #[cfg(all(any(target_pointer_width = "32", windows), not(feature = "zlib-debug")))]
        {
            FLUSH_BLOCK__3!(s, 0i32);
        }
    }
    1
}
