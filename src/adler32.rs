//! adler32.c -- compute the Adler-32 checksum of a data stream
//! Copyright (C) 1995-2011, 2016 Mark Adler
//! For conditions of distribution and use, see copyright notice in zlib.h
macro_rules! DO16 {
    ($buf:expr, $adler:expr, $sum2:expr) => {
        DO8!($buf, 0i32, $adler, $sum2);
        DO8!($buf, 8i32, $adler, $sum2);
    };
}

macro_rules! DO1 {
    ($buf:expr, $i:expr, $adler:expr, $sum2:expr) => {
        let __v: c_ulong = $adler;
        let __v2: c_ulong =
            __v.wrapping_add(((unsafe { *$buf.offset(($i) as isize) }) as i32) as c_ulong);
        $adler = __v2;
        let __v3: c_ulong = $sum2;
        let __v4: c_ulong = __v3.wrapping_add($adler);
        $sum2 = __v4;
    };
}

macro_rules! DO2 {
    ($buf:expr, $i:expr, $adler:expr, $sum2:expr) => {
        DO1!($buf, $i, $adler, $sum2);
        DO1!($buf, ($i) + 1, $adler, $sum2);
    };
}

macro_rules! DO4 {
    ($buf:expr, $i:expr, $adler:expr, $sum2:expr) => {
        DO2!($buf, $i, $adler, $sum2);
        DO2!($buf, ($i) + 2, $adler, $sum2);
    };
}

macro_rules! DO8 {
    ($buf:expr, $i:expr, $adler:expr, $sum2:expr) => {
        DO4!($buf, $i, $adler, $sum2);
        DO4!($buf, ($i) + 4, $adler, $sum2);
    };
}

macro_rules! MOD28 {
    ($a:expr) => {
        let __v: c_ulong = $a;
        let __v2: c_ulong = __v % 65521;
        $a = __v2;
    };
}

#[cfg(all(target_pointer_width = "64", unix))]
macro_rules! MOD63 {
    ($a:expr) => {
        let __v: off_t = $a;
        let __v2: off_t = __v % 65521;
        $a = __v2;
    };
}

#[cfg(all(target_pointer_width = "32", unix))]
macro_rules! MOD63 {
    ($a:expr) => {
        let __v: off_t = $a;
        let __v2: off_t = ((__v as c_ulong) % 65521) as off_t;
        $a = __v2;
    };
}

#[cfg(windows)]
macro_rules! MOD63 {
    ($a:expr) => {
        let __v: i64 = $a;
        let __v2: i64 = __v % 65521;
        $a = __v2;
    };
}

macro_rules! MOD {
    ($a:expr) => {
        let __v: c_ulong = $a;
        let __v2: c_ulong = __v % 65521;
        $a = __v2;
    };
}

use core::ffi::c_long;
use core::ffi::c_ulong;
#[cfg(unix)]
use slate_support::off_t;
use slate_support::size_t;

// @(#) $Id$
// largest prime smaller than 65536
// NMAX is the largest n such that 255n(n+1)/2 + (n+1)(BASE-1) <= 2^32-1
/// use NO_DIVIDE if your processor does not do division in hardware --
/// try it both ways to see which is faster
#[unsafe(no_mangle)]
extern "C-unwind" fn adler32_z(mut adler: c_ulong, mut buf: *const u8, mut len: size_t) -> c_ulong {
    let mut sum2: c_ulong = 0;
    let mut n: u32 = 0;
    // split Adler-32 into component sums
    sum2 = adler >> 16i32 & 65535;
    adler &= 65535;
    // in case user likes doing a byte at a time, keep it fast
    if len == 1 {
        adler = adler.wrapping_add(((unsafe { *buf.add(0) }) as i32) as c_ulong);
        if adler >= 65521 {
            adler = adler.wrapping_sub(65521);
        }
        sum2 = sum2.wrapping_add(adler);
        if sum2 >= 65521 {
            sum2 = sum2.wrapping_sub(65521);
        }
        return adler | sum2 << 16i32;
    }
    // initial Adler-32 value (deferred check for len == 1 speed)
    if buf.is_null() {
        return 1;
    }
    // in case short lengths are provided, keep it somewhat fast
    if len < 16 {
        loop {
            let __v17: size_t = len;
            len = __v17.wrapping_sub(1);
            if __v17 == 0 {
                break;
            }
            let __v10: *const u8 = buf;
            buf = unsafe { __v10.add(1) };
            let __v12: u8 = unsafe { *__v10 };
            adler = adler.wrapping_add((__v12 as i32) as c_ulong);
            sum2 = sum2.wrapping_add(adler);
        }
        if adler >= 65521 {
            adler = adler.wrapping_sub(65521);
        }
        MOD28!(sum2); // only added so many BASE's
        return adler | sum2 << 16i32;
    }
    // do length NMAX blocks -- requires just one modulo operation
    while len >= 5552 {
        len = len.wrapping_sub(5552);
        n = 347u32; // NMAX is divisible by 16
        loop {
            DO16!(buf, adler, sum2);
            // 16 sums unrolled
            let __v90: *const u8 = buf;
            buf = unsafe { __v90.add(16) };
            let __v94: u32 = n.wrapping_sub(1);
            n = __v94;
            if __v94 == 0 {
                break;
            }
        }
        MOD!(adler);
        MOD!(sum2);
    }
    // do remaining bytes (less than NMAX, still just one modulo)
    if len != 0 {
        // avoid modulos if none remaining
        while len >= 16 {
            len = len.wrapping_sub(16);
            DO16!(buf, adler, sum2);
            let __v166: *const u8 = buf;
            buf = unsafe { __v166.add(16) };
        }
        loop {
            let __v176: size_t = len;
            len = __v176.wrapping_sub(1);
            if __v176 == 0 {
                break;
            }
            let __v169: *const u8 = buf;
            buf = unsafe { __v169.add(1) };
            let __v171: u8 = unsafe { *__v169 };
            adler = adler.wrapping_add((__v171 as i32) as c_ulong);
            sum2 = sum2.wrapping_add(adler);
        }
        MOD!(adler);
        MOD!(sum2);
    }
    // return recombined sums
    adler | sum2 << 16i32
}

#[unsafe(no_mangle)]
extern "C-unwind" fn adler32(adler: c_ulong, buf: *const u8, len: u32) -> c_ulong {
    adler32_z(adler, buf, len as size_t)
}

#[cfg(unix)]
type __SlateHoist_adler32_combine_ = off_t;

#[cfg(windows)]
type __SlateHoist_adler32_combine_ = i64;

fn adler32_combine_(
    adler1: c_ulong,
    adler2: c_ulong,
    mut len2: __SlateHoist_adler32_combine_,
) -> c_ulong {
    let mut sum1: c_ulong = 0;
    let mut sum2: c_ulong = 0;
    let mut rem: u32 = 0;
    // for negative len, return invalid adler32 as a clue for debugging
    if len2 < 0 {
        return 4294967295;
    }
    // the derivation of this formula is left as an exercise for the reader
    MOD63!(len2); // assumes len2 >= 0
    rem = len2 as u32;
    sum1 = adler1 & 65535;
    sum2 = (rem as c_ulong).wrapping_mul(sum1);
    MOD!(sum2);
    sum1 = sum1.wrapping_add((adler2 & 65535).wrapping_add(65521).wrapping_sub(1));
    sum2 = sum2.wrapping_add(
        (adler1 >> 16i32 & 65535)
            .wrapping_add(adler2 >> 16i32 & 65535)
            .wrapping_add(65521)
            .wrapping_sub(rem as c_ulong),
    );
    if sum1 >= 65521 {
        sum1 = sum1.wrapping_sub(65521);
    }
    if sum1 >= 65521 {
        sum1 = sum1.wrapping_sub(65521);
    }
    if sum2 >= (131042 as c_ulong) {
        sum2 = sum2.wrapping_sub((131042 as c_ulong));
    }
    if sum2 >= 65521 {
        sum2 = sum2.wrapping_sub(65521);
    }
    sum1 | sum2 << 16i32
}

#[cfg(unix)]
type __SlateHoist_adler32_combine = off_t;

#[cfg(windows)]
type __SlateHoist_adler32_combine = i64;

#[unsafe(no_mangle)]
extern "C-unwind" fn adler32_combine(
    adler1: c_ulong,
    adler2: c_ulong,
    len2: __SlateHoist_adler32_combine,
) -> c_ulong {
    adler32_combine_(adler1, adler2, len2)
}

#[cfg(unix)]
type __SlateHoist_adler32_combine64 = off_t;

#[cfg(windows)]
type __SlateHoist_adler32_combine64 = i64;

#[unsafe(no_mangle)]
extern "C-unwind" fn adler32_combine64(
    adler1: c_ulong,
    adler2: c_ulong,
    len2: __SlateHoist_adler32_combine64,
) -> c_ulong {
    adler32_combine_(adler1, adler2, len2)
}
