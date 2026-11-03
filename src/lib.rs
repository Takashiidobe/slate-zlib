#![allow(non_camel_case_types, rustdoc::all)]

#[cfg(not(any(
    all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"),
    all(target_arch = "x86", target_os = "linux", target_env = "gnu"),
    all(target_arch = "aarch64", target_os = "linux", target_env = "gnu"),
    all(target_arch = "arm", target_os = "linux", target_env = "gnu"),
    all(target_arch = "x86_64", target_os = "linux", target_env = "musl"),
    all(target_arch = "aarch64", target_os = "linux", target_env = "musl"),
    all(target_arch = "x86_64", target_os = "macos"),
    all(target_arch = "aarch64", target_os = "macos"),
    all(target_arch = "x86_64", target_os = "freebsd"),
    all(target_arch = "aarch64", target_os = "freebsd"),
    all(target_arch = "x86_64", target_os = "android"),
    all(target_arch = "aarch64", target_os = "android"),
    all(target_arch = "x86_64", windows, target_env = "msvc"),
    all(target_arch = "x86", windows, target_env = "msvc"),
    all(target_arch = "aarch64", windows, target_env = "msvc"),
    all(target_arch = "arm", windows, target_env = "msvc")
)))]
compile_error!("this crate was not translated for this target");

#[macro_use]
mod __slate_macros;

#[path = "adler32.rs"]
mod __slate_unit_adler32;
#[path = "compress.rs"]
mod __slate_unit_compress;
#[path = "crc32.rs"]
mod __slate_unit_crc32;
#[path = "deflate.rs"]
mod __slate_unit_deflate;
#[path = "gzclose.rs"]
mod __slate_unit_gzclose;
#[path = "gzlib.rs"]
mod __slate_unit_gzlib;
#[path = "gzread.rs"]
mod __slate_unit_gzread;
#[path = "gzwrite.rs"]
mod __slate_unit_gzwrite;
#[path = "infback.rs"]
mod __slate_unit_infback;
#[path = "inffast.rs"]
mod __slate_unit_inffast;
#[path = "inflate.rs"]
mod __slate_unit_inflate;
#[path = "inftrees.rs"]
mod __slate_unit_inftrees;
#[path = "trees.rs"]
mod __slate_unit_trees;
#[path = "uncompr.rs"]
mod __slate_unit_uncompr;
#[path = "zutil.rs"]
mod __slate_unit_zutil;
