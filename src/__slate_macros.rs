#[cfg(feature = "zlib-debug")]
macro_rules! Assert {
    ($cond:expr, $msg:expr) => {
        if !$cond {
            unsafe { z_error($msg) };
        }
    };
}

macro_rules! BITS {
    ($n:expr, $hold:expr) => {
        (($hold) as u32) & (1u32 << $n).wrapping_sub(1)
    };
}

macro_rules! BYTEBITS {
    ($hold:expr, $bits:expr) => {
        loop {
            let __v: c_ulong = $hold;
            let __v2: c_ulong = __v >> (($bits) & 7);
            $hold = __v2;
            let __v3: u32 = $bits;
            let __v4: u32 = __v3.wrapping_sub(($bits) & 7);
            $bits = __v4;
            if true {
                break;
            }
        }
    };
}

macro_rules! DROPBITS {
    ($n:expr, $hold:expr, $bits:expr) => {
        loop {
            let __v: c_ulong = $hold;
            let __v2: c_ulong = __v >> $n;
            $hold = __v2;
            let __v3: u32 = $bits;
            let __v4: u32 = __v3.wrapping_sub(($n) as u32);
            $bits = __v4;
            if true {
                break;
            }
        }
    };
}

macro_rules! DROPBITS__2 {
    ($n:expr, $hold:expr, $bits:expr) => {
        loop {
            let __v: c_ulong = $hold;
            let __v2: c_ulong = __v >> (($n) as i32);
            $hold = __v2;
            let __v3: u32 = $bits;
            let __v4: u32 = __v3.wrapping_sub(($n) as u32);
            $bits = __v4;
            if true {
                break;
            }
        }
    };
}

#[cfg(unix)]
macro_rules! GT_OFF {
    ($x:expr) => {{
        let __v: bool = (std::mem::size_of::<i32>() as size_t)
            == (std::mem::size_of::<off_t>() as size_t)
            && ($x) > unsafe { gz_intmax() };
        __v
    }};
}

#[cfg(windows)]
macro_rules! GT_OFF {
    ($x:expr) => {{
        let __v: bool = (std::mem::size_of::<i32>() as size_t)
            == (std::mem::size_of::<i64>() as size_t)
            && ($x) > unsafe { gz_intmax() };
        __v
    }};
}

macro_rules! INITBITS {
    ($hold:expr, $bits:expr) => {
        loop {
            $hold = 0;
            $bits = 0;
            if true {
                break;
            }
        }
    };
}

macro_rules! LOAD {
    ($put:expr, $strm:expr, $left:expr, $next:expr, $have:expr, $hold:expr, $state:expr, $bits:expr) => {
        loop {
            $put = unsafe { (*$strm).next_out };
            $left = unsafe { (*$strm).avail_out };
            $next = unsafe { (*$strm).next_in };
            $have = unsafe { (*$strm).avail_in };
            $hold = unsafe { (*$state).hold };
            $bits = unsafe { (*$state).bits };
            if true {
                break;
            }
        }
    };
}

macro_rules! RESTORE {
    ($strm:expr, $put:expr, $left:expr, $next:expr, $have:expr, $state:expr, $hold:expr, $bits:expr) => {
        loop {
            unsafe {
                (*$strm).next_out = $put;
            }
            unsafe {
                (*$strm).avail_out = $left;
            }
            unsafe {
                (*$strm).next_in = $next;
            }
            unsafe {
                (*$strm).avail_in = $have;
            }
            unsafe {
                (*$state).hold = $hold;
            }
            unsafe {
                (*$state).bits = $bits;
            }
            if true {
                break;
            }
        }
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! Tracev {
    ($x__0:expr, $x__1:expr) => {
        if (unsafe { z_verbose }) > 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! Tracev__2 {
    ($x__0:expr, $x__1:expr, $x__2:expr) => {
        if (unsafe { z_verbose }) > 0 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char, $x__2) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! Tracevv {
    ($x__0:expr, $x__1:expr, $x__2:expr) => {
        if (unsafe { z_verbose }) > 1 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char, $x__2) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! Tracevv__2 {
    ($x__0:expr, $x__1:expr, $x__2:expr) => {
        if (unsafe { z_verbose }) > 1 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char, ($x__2) as i32) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

#[cfg(feature = "zlib-debug")]
macro_rules! Tracevv__3 {
    ($x__0:expr, $x__1:expr) => {
        if (unsafe { z_verbose }) > 1 {
            let _ = std::io::Write::flush(&mut std::io::stdout());
            unsafe { slate_support::fprintf($x__0, ($x__1) as *const c_char) };
            unsafe { fflush(std::ptr::null_mut()) };
        }
    };
}

macro_rules! ZALLOC {
    ($strm:expr, $items:expr, $size:expr) => {
        unsafe { unsafe { (*$strm).zalloc }.unwrap()((*$strm).opaque, $items, ($size) as u32) }
    };
}

macro_rules! ZALLOC__2 {
    ($strm:expr, $items:expr, $size:expr) => {
        unsafe {
            unsafe { (*$strm).zalloc }.unwrap()((*$strm).opaque, ($items) as u32, ($size) as u32)
        }
    };
}

macro_rules! ZFREE {
    ($strm:expr, $addr:expr) => {
        unsafe { unsafe { (*$strm).zfree }.unwrap()((*$strm).opaque, ($addr) as *mut ()) };
    };
}

macro_rules! put_byte {
    ($s:expr, $c:expr) => {
        let __v: *mut internal_state = $s;
        let __v2: c_ulong = unsafe { (*__v).pending };
        unsafe {
            (*__v).pending = __v2.wrapping_add(1);
        }
        unsafe {
            *unsafe { (*$s).pending_buf }.add(__v2 as usize) = ($c) as u8;
        }
    };
}

#[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
macro_rules! zmemzero {
    ($dest:expr, $len:expr) => {
        unsafe {
            std::ptr::write_bytes(
                ((($dest) as *mut ()) as *mut core::ffi::c_void) as *mut u8,
                (0i32 as i32) as u8,
                (($len) as usize) as usize,
            )
        };
    };
}

#[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
macro_rules! zmemzero {
    ($dest:expr, $len:expr) => {
        unsafe { std::ptr::write_bytes((($dest) as *mut ()) as *mut u8, 0u8, ($len) as usize) };
    };
}

#[cfg(not(any(target_os = "macos", all(target_arch = "arm", windows))))]
macro_rules! zstrerror {
    () => {
        unsafe { strerror(*slate_support::errno_location() as i32) as *mut c_char }
    };
}

#[cfg(any(target_os = "macos", all(target_arch = "arm", windows)))]
macro_rules! zstrerror {
    () => {
        unsafe { strerror(*slate_support::errno_location()) }
    };
}
