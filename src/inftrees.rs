//! inftrees.c -- generate Huffman trees for efficient decoding
//! Copyright (C) 1995-2026 Mark Adler
//! For conditions of distribution and use, see copyright notice in zlib.h
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

#[cfg(target_arch = "x86_64")]
#[repr(C, align(16))]
struct __SlateAlignLargeArray<T>(T);

#[cfg(not(target_arch = "x86_64"))]
#[repr(C)]
struct __SlateAlignLargeArray<T>(T);

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_os = "macos",
    windows
))]
#[unsafe(no_mangle)]
static mut inflate_copyright: __SlateAlignLargeArray<[c_char; 49]> = __SlateAlignLargeArray([
    32i8, 105i8, 110i8, 102i8, 108i8, 97i8, 116i8, 101i8, 32i8, 49i8, 46i8, 51i8, 46i8, 50i8, 46i8,
    49i8, 32i8, 67i8, 111i8, 112i8, 121i8, 114i8, 105i8, 103i8, 104i8, 116i8, 32i8, 49i8, 57i8,
    57i8, 53i8, 45i8, 50i8, 48i8, 50i8, 54i8, 32i8, 77i8, 97i8, 114i8, 107i8, 32i8, 65i8, 100i8,
    108i8, 101i8, 114i8, 32i8, 0i8,
]);

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_os = "macos",
    windows
)))]
#[unsafe(no_mangle)]
static mut inflate_copyright: __SlateAlignLargeArray<[c_char; 49]> = __SlateAlignLargeArray([
    32u8, 105u8, 110u8, 102u8, 108u8, 97u8, 116u8, 101u8, 32u8, 49u8, 46u8, 51u8, 46u8, 50u8, 46u8,
    49u8, 32u8, 67u8, 111u8, 112u8, 121u8, 114u8, 105u8, 103u8, 104u8, 116u8, 32u8, 49u8, 57u8,
    57u8, 53u8, 45u8, 50u8, 48u8, 50u8, 54u8, 32u8, 77u8, 97u8, 114u8, 107u8, 32u8, 65u8, 100u8,
    108u8, 101u8, 114u8, 32u8, 0u8,
]);

// If you use the zlib library in a product, an acknowledgment is welcome
// in the documentation of your product. If for some reason you cannot
// include such an acknowledgment, I would appreciate that you keep this
// copyright string in the executable of your product.
/// Build a set of tables to decode the provided canonical Huffman code.
/// The code lengths are lens[0..codes-1].  The result starts at *table,
/// whose indices are 0..2^bits-1.  work is a writable array of at least
/// lens shorts, which is used as a work area.  type is the type of code
/// to be generated, CODES, LENS, or DISTS.  On return, zero is success,
/// -1 is an invalid code, and +1 means that ENOUGH isn't enough.  table
/// on return points to the next available entry's address.  bits is the
/// requested root table index bits, and on return it is the actual root
/// table index bits.  It will differ if the request is greater than the
/// longest code or if it is less than the shortest code.
#[unsafe(no_mangle)]
extern "C-unwind" fn inflate_table(
    r#type: u32,
    lens: *mut u16,
    codes: u32,
    table: *mut *mut __SlateRecord_inftrees_h_28,
    bits: *mut u32,
    work: *mut u16,
) -> i32 {
    // accumulate lengths for codes (assumes lens[] all in 0..MAXBITS)
    let mut len: u32 = 0; // a code's length in bits
    let mut sym: u32 = 0; // index of code symbols
    let mut min: u32 = 1;
    let mut max: u32 = 15; // minimum and maximum code lengths
    let mut root: u32 = 0; // number of index bits for root table
    let mut curr: u32 = 0; // number of index bits for current table
    // current bits to drop from code for index
    let mut drop: u32 = 0; // code bits to drop for sub-table
    // check for an over-subscribed or incomplete set of lengths
    let mut left: i32 = 1; // number of prefix codes available
    let mut used: u32 = 0; // code entries in table used
    // initialize state for loop
    // starting code
    let mut huff: u32 = 0; // Huffman code
    let mut incr: u32 = 0; // for incrementing code, index
    let mut fill: u32 = 0; // index for replicating entries
    // trigger new sub-table when len > root
    let mut low: u32 = 4294967295u32; // low bits for current root entry
    let mut mask: u32 = 0; // mask for low root bits
    let mut here: __SlateRecord_inftrees_h_28 = unsafe { std::mem::zeroed() }; // table entry for duplication
    let mut next: *mut __SlateRecord_inftrees_h_28 = unsafe { std::mem::zeroed() }; // next available space in table
    let mut base: *const u16 = std::ptr::null::<u16>(); // base value table to use
    let mut extra: *const u16 = std::ptr::null::<u16>(); // extra bits table to use
    let mut r#match: u32 = 0; // use base and extra for symbol >= match
    let mut count: __SlateAlignLargeArray<[u16; 16]> = __SlateAlignLargeArray([0u16; 16]); // number of codes of each length
    let mut offs: __SlateAlignLargeArray<[u16; 16]> = __SlateAlignLargeArray([0u16; 16]); // offsets in table for each length
    // Length codes 257..285 base
    // Length codes 257..285 extra
    // Distance codes 0..29 base
    // Distance codes 0..29 extra
    // Process a set of code lengths to create a canonical Huffman code.  The
    // code lengths are lens[0..codes-1].  Each length corresponds to the
    // symbols 0..codes-1.  The Huffman code is generated by first sorting the
    // symbols by length from short to long, and retaining the symbol order
    // for codes with equal lengths.  Then the code starts with all zero bits
    // for the first code of the shortest length, and the codes are integer
    // increments for the same length, and zeros are appended as the length
    // increases.  For the deflate format, these bits are stored backwards
    // from their more natural integer increment ordering, and so when the
    // decoding tables are built in the large loop below, the integer codes
    // are incremented backwards.
    //
    // This routine assumes, but does not check, that all of the entries in
    // lens[] are in the range 0..MAXBITS.  The caller must assure this.
    // 1..MAXBITS is interpreted as that code length.  zero means that that
    // symbol does not occur in this code.
    //
    // The codes are sorted by computing a count of codes for each length,
    // creating from that a table of starting indices for each length in the
    // sorted table, and then entering the symbols in order in the sorted
    // table.  The sorted table is work[], with that space being provided by
    // the caller.
    //
    // The length counts are used for other purposes as well, i.e. finding
    // the minimum and maximum length codes, determining if there are any
    // codes at all, checking for a valid set of lengths, and looking ahead
    // at length counts to determine sub-table sizes when building the
    // decoding tables.
    while len <= 15 {
        count.0[len as usize] = 0;
        len = len.wrapping_add(1);
    }
    while sym < codes {
        let __v3: *mut u16 = unsafe {
            (count.0.as_mut_ptr() as *mut u16).offset((*lens.add(sym as usize) as i32) as isize)
        };
        unsafe {
            *__v3 = ((*__v3 as i32) + 1) as u16;
        }
        sym = sym.wrapping_add(1);
    }
    // bound code lengths, force root to be within code lengths
    root = unsafe { *bits };
    while max >= 1 {
        if (count.0[max as usize] as i32) != 0 {
            break;
        }
        max = max.wrapping_sub(1);
    }
    if root > max {
        root = max;
    }
    if max == 0 {
        // no symbols to code at all
        here.op = 64; // invalid code marker
        here.bits = 1;
        here.val = 0;
        let __v13: *mut __SlateRecord_inftrees_h_28 = unsafe { *table };
        unsafe {
            *table = __v13.add(1);
        }
        unsafe {
            *__v13 = here;
        }
        // make a table to force an error
        let __v16: *mut __SlateRecord_inftrees_h_28 = unsafe { *table };
        unsafe {
            *table = __v16.add(1);
        }
        unsafe {
            *__v16 = here;
        }
        unsafe {
            *bits = 1;
        }
        return 0; // no symbols, but wait for decoding to report error
    }
    while min < max {
        if (count.0[min as usize] as i32) != 0 {
            break;
        }
        min = min.wrapping_add(1);
    }
    if root < min {
        root = min;
    }
    len = 1;
    while len <= 15 {
        left <<= 1i32;
        left -= count.0[len as usize] as i32;
        if left < 0 {
            return -1;
        }
        // over-subscribed
        len = len.wrapping_add(1);
    }
    if left > 0 && ((r#type as u32) == 0 || max != 1) {
        return -1;
    }
    // incomplete set
    // generate offsets into symbol table for each length for sorting
    offs.0[1] = 0;
    len = 1;
    while len < 15 {
        offs.0[len.wrapping_add(1) as usize] =
            ((offs.0[len as usize] as i32) + (count.0[len as usize] as i32)) as u16;
        len = len.wrapping_add(1);
    }
    // sort symbols by length, by symbol order within each length
    sym = 0;
    while sym < codes {
        if ((unsafe { *lens.add(sym as usize) }) as i32) != 0 {
            let __v31: *mut u16 = unsafe {
                (offs.0.as_mut_ptr() as *mut u16).offset((*lens.add(sym as usize) as i32) as isize)
            };
            let __v32: u16 = unsafe { *__v31 };
            unsafe {
                *__v31 = ((__v32 as i32) + 1) as u16;
            }
            unsafe {
                *work.offset((__v32 as i32) as isize) = sym as u16;
            }
        }
        sym = sym.wrapping_add(1);
    }
    // Create and fill in decoding tables.  In this loop, the table being
    // filled is at next and has curr index bits.  The code being used is huff
    // with length len.  That code is converted to an index by dropping drop
    // bits off of the bottom.  For codes where len is less than drop + curr,
    // those top drop + curr - len bits are incremented through all values to
    // fill the table with replicated entries.
    //
    // root is the number of index bits for the root table.  When len exceeds
    // root, sub-tables are created pointed to by the root entry with an index
    // of the low root bits of huff.  This is saved in low to check for when a
    // new sub-table should be started.  drop is zero when the root table is
    // being filled, and drop is root when sub-tables are being filled.
    //
    // When a new sub-table is needed, it is necessary to look ahead in the
    // code lengths to determine what size sub-table is needed.  The length
    // counts are used for this, and so count[] is decremented as codes are
    // entered in the tables.
    //
    // used keeps track of how many table entries have been allocated from the
    // provided *table space.  It is checked for LENS and DIST tables against
    // the constants ENOUGH_LENS and ENOUGH_DISTS to guard against changes in
    // the initial root table size constants.  See the comments in inftrees.h
    // for more information.
    //
    // sym increments through all symbols, and the loop terminates when
    // all codes of length max, i.e. all codes, have been processed.  This
    // routine permits incomplete codes, so another loop after this one fills
    // in the rest of the decoding tables with invalid code markers.
    // set up for code type
    match r#type as u32 {
        0 => {
            r#match = 20;
        }
        1 => {
            base = unsafe { std::ptr::addr_of!(lbase.0) as *const u16 };
            extra = unsafe { std::ptr::addr_of!(lext.0) as *const u16 };
            r#match = 257;
        }
        2 => {
            base = unsafe { std::ptr::addr_of!(dbase.0) as *const u16 };
            extra = unsafe { std::ptr::addr_of!(dext.0) as *const u16 };
        }
        _ => {}
    }
    sym = 0; // starting code symbol
    len = min; // starting code length
    next = unsafe { *table }; // current table to fill in
    curr = root; // current table index bits
    used = 1u32 << root; // use root table entries
    mask = used.wrapping_sub(1); // mask for comparing low
    // check available table space
    #[cfg(not(feature = "pkzip-bug"))]
    {
        if (r#type as u32) == 1 && used > 852 || (r#type as u32) == 2 && used > 592 {
            return 1;
        }
    }
    #[cfg(feature = "pkzip-bug")]
    {
        if (r#type as u32) == 1 && used > 854 || (r#type as u32) == 2 && used > 594 {
            return 1;
        }
    }
    // process all codes and make table entries
    loop {
        // create table entry
        here.bits = len.wrapping_sub(drop) as u8;
        if (((unsafe { *work.add(sym as usize) }) as i32) as u32).wrapping_add(1) < r#match {
            here.op = 0;
            here.val = unsafe { *work.add(sym as usize) };
        } else {
            if (((unsafe { *work.add(sym as usize) }) as i32) as u32) >= r#match {
                here.op = (unsafe {
                    *extra.offset(
                        ((*work.add(sym as usize) as i32) as u32).wrapping_sub(r#match) as isize,
                    )
                }) as u8;
                here.val = unsafe {
                    *base.offset(
                        ((*work.add(sym as usize) as i32) as u32).wrapping_sub(r#match) as isize,
                    )
                };
            } else {
                here.op = 96u8; // end of block
                here.val = 0;
            }
        }
        // replicate for those indices with low len bits equal to huff
        incr = 1u32 << len.wrapping_sub(drop);
        fill = 1u32 << curr;
        min = fill; // save offset to next table
        loop {
            fill = fill.wrapping_sub(incr);
            unsafe {
                *next.offset((huff >> drop).wrapping_add(fill) as isize) = here;
            }
            if fill == 0 {
                break;
            }
        }
        // backwards increment the len-bit code huff
        incr = 1u32 << len.wrapping_sub(1);
        while huff & incr != 0 {
            incr >>= 1i32;
        }
        if incr != 0 {
            huff &= incr.wrapping_sub(1);
            huff = huff.wrapping_add(incr);
        } else {
            huff = 0;
        }
        // go to next symbol, update count, len
        sym = sym.wrapping_add(1);
        let __v50: *mut u16 = unsafe { (count.0.as_mut_ptr() as *mut u16).add(len as usize) };
        let __v52: u16 = (((unsafe { *__v50 }) as i32) - 1) as u16;
        unsafe {
            *__v50 = __v52;
        }
        if (__v52 as i32) == 0 {
            if len == max {
                break;
            }
            len = (unsafe { *lens.offset((*work.add(sym as usize) as i32) as isize) }) as u32;
        }
        // create new sub-table if needed
        if len > root && huff & mask != low {
            // if first time, transition to sub-tables
            if drop == 0 {
                drop = root;
            }
            // increment past last table
            let __v54: *mut __SlateRecord_inftrees_h_28 = next;
            next = unsafe { __v54.add(min as usize) }; // here min is 1 << curr
            // determine length of next table
            curr = len.wrapping_sub(drop);
            left = 1i32 << curr;
            while curr.wrapping_add(drop) < max {
                left -= count.0[curr.wrapping_add(drop) as usize] as i32;
                if left <= 0 {
                    break;
                }
                curr = curr.wrapping_add(1);
                left <<= 1i32;
            }
            // check for enough space
            used = used.wrapping_add(1u32 << curr);
            #[cfg(not(feature = "pkzip-bug"))]
            {
                if (r#type as u32) == 1 && used > 852 || (r#type as u32) == 2 && used > 592 {
                    return 1;
                }
            }
            #[cfg(feature = "pkzip-bug")]
            {
                if (r#type as u32) == 1 && used > 854 || (r#type as u32) == 2 && used > 594 {
                    return 1;
                }
            }
            // point entry in root table to sub-table
            low = huff & mask;
            unsafe {
                (*unsafe { *table }.add(low as usize)).op = curr as u8;
            }
            unsafe {
                (*unsafe { *table }.add(low as usize)).bits = root as u8;
            }
            unsafe {
                (*unsafe { *table }.add(low as usize)).val =
                    (next.offset_from(*table as *mut __SlateRecord_inftrees_h_28) as ptrdiff_t)
                        as u16;
            }
        }
    }
    // fill in remaining table entry if code is incomplete (guaranteed to have
    // at most one remaining entry, since if the code is incomplete, the
    // maximum code length that was allowed to get this far is one bit)
    if huff != 0 {
        here.op = 64; // invalid code marker
        here.bits = len.wrapping_sub(drop) as u8;
        here.val = 0;
        unsafe {
            *next.add(huff as usize) = here;
        }
    }
    // set return parameters
    let __v66: *mut __SlateRecord_inftrees_h_28 = unsafe { *table };
    unsafe {
        *table = __v66.add(used as usize);
    }
    unsafe {
        *bits = root;
    }
    0
}

static mut lbase: __SlateAlignLargeArray<[u16; 31]> = __SlateAlignLargeArray([
    3u16, 4u16, 5u16, 6u16, 7u16, 8u16, 9u16, 10u16, 11u16, 13u16, 15u16, 17u16, 19u16, 23u16,
    27u16, 31u16, 35u16, 43u16, 51u16, 59u16, 67u16, 83u16, 99u16, 115u16, 131u16, 163u16, 195u16,
    227u16, 258u16, 0u16, 0u16,
]);

static mut lext: __SlateAlignLargeArray<[u16; 31]> = __SlateAlignLargeArray([
    16u16, 16u16, 16u16, 16u16, 16u16, 16u16, 16u16, 16u16, 17u16, 17u16, 17u16, 17u16, 18u16,
    18u16, 18u16, 18u16, 19u16, 19u16, 19u16, 19u16, 20u16, 20u16, 20u16, 20u16, 21u16, 21u16,
    21u16, 21u16, 16u16, 68u16, 193u16,
]);

static mut dbase: __SlateAlignLargeArray<[u16; 32]> = __SlateAlignLargeArray([
    1u16, 2u16, 3u16, 4u16, 5u16, 7u16, 9u16, 13u16, 17u16, 25u16, 33u16, 49u16, 65u16, 97u16,
    129u16, 193u16, 257u16, 385u16, 513u16, 769u16, 1025u16, 1537u16, 2049u16, 3073u16, 4097u16,
    6145u16, 8193u16, 12289u16, 16385u16, 24577u16, 0u16, 0u16,
]);

static mut dext: __SlateAlignLargeArray<[u16; 32]> = __SlateAlignLargeArray([
    16u16, 16u16, 16u16, 16u16, 17u16, 17u16, 18u16, 18u16, 19u16, 19u16, 20u16, 20u16, 21u16,
    21u16, 22u16, 22u16, 23u16, 23u16, 24u16, 24u16, 25u16, 25u16, 26u16, 26u16, 27u16, 27u16,
    28u16, 28u16, 29u16, 29u16, 64u16, 64u16,
]);

static mut lenfix: __SlateAlignLargeArray<[__SlateRecord_inftrees_h_28; 512]> =
    __SlateAlignLargeArray([
        __SlateRecord_inftrees_h_28 {
            op: 96u8,
            bits: 7u8,
            val: 0u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 80u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 16u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 8u8,
            val: 115u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 31u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 112u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 48u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 192u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 10u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 96u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 32u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 160u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 0u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 128u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 64u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 224u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 6u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 88u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 24u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 144u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 59u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 120u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 56u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 208u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 17u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 104u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 40u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 176u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 8u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 136u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 72u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 240u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 4u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 84u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 20u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 21u8,
            bits: 8u8,
            val: 227u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 43u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 116u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 52u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 200u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 13u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 100u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 36u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 168u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 4u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 132u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 68u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 232u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 8u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 92u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 28u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 152u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 7u8,
            val: 83u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 124u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 60u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 216u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 23u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 108u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 44u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 184u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 12u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 140u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 76u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 248u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 3u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 82u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 18u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 21u8,
            bits: 8u8,
            val: 163u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 35u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 114u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 50u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 196u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 11u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 98u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 34u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 164u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 2u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 130u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 66u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 228u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 7u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 90u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 26u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 148u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 7u8,
            val: 67u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 122u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 58u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 212u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 19u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 106u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 42u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 180u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 10u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 138u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 74u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 244u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 5u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 86u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 22u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 64u8,
            bits: 8u8,
            val: 0u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 51u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 118u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 54u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 204u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 15u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 102u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 38u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 172u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 6u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 134u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 70u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 236u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 9u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 94u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 30u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 156u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 7u8,
            val: 99u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 126u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 62u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 220u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 27u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 110u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 46u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 188u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 14u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 142u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 78u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 252u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 96u8,
            bits: 7u8,
            val: 0u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 81u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 17u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 21u8,
            bits: 8u8,
            val: 131u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 31u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 113u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 49u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 194u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 10u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 97u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 33u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 162u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 1u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 129u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 65u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 226u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 6u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 89u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 25u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 146u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 59u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 121u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 57u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 210u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 17u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 105u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 41u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 178u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 9u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 137u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 73u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 242u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 4u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 85u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 21u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 8u8,
            val: 258u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 43u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 117u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 53u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 202u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 13u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 101u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 37u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 170u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 5u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 133u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 69u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 234u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 8u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 93u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 29u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 154u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 7u8,
            val: 83u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 125u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 61u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 218u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 23u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 109u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 45u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 186u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 13u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 141u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 77u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 250u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 3u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 83u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 19u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 21u8,
            bits: 8u8,
            val: 195u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 35u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 115u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 51u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 198u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 11u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 99u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 35u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 166u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 3u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 131u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 67u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 230u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 7u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 91u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 27u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 150u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 7u8,
            val: 67u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 123u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 59u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 214u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 19u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 107u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 43u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 182u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 11u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 139u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 75u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 246u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 5u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 87u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 23u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 64u8,
            bits: 8u8,
            val: 0u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 51u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 119u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 55u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 206u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 15u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 103u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 39u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 174u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 7u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 135u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 71u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 238u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 9u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 95u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 31u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 158u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 7u8,
            val: 99u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 127u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 63u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 222u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 27u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 111u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 47u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 190u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 15u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 143u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 79u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 254u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 96u8,
            bits: 7u8,
            val: 0u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 80u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 16u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 8u8,
            val: 115u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 31u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 112u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 48u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 193u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 10u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 96u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 32u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 161u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 0u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 128u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 64u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 225u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 6u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 88u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 24u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 145u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 59u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 120u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 56u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 209u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 17u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 104u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 40u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 177u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 8u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 136u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 72u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 241u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 4u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 84u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 20u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 21u8,
            bits: 8u8,
            val: 227u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 43u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 116u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 52u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 201u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 13u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 100u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 36u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 169u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 4u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 132u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 68u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 233u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 8u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 92u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 28u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 153u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 7u8,
            val: 83u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 124u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 60u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 217u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 23u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 108u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 44u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 185u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 12u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 140u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 76u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 249u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 3u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 82u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 18u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 21u8,
            bits: 8u8,
            val: 163u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 35u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 114u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 50u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 197u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 11u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 98u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 34u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 165u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 2u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 130u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 66u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 229u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 7u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 90u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 26u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 149u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 7u8,
            val: 67u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 122u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 58u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 213u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 19u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 106u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 42u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 181u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 10u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 138u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 74u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 245u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 5u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 86u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 22u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 64u8,
            bits: 8u8,
            val: 0u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 51u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 118u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 54u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 205u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 15u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 102u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 38u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 173u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 6u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 134u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 70u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 237u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 9u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 94u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 30u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 157u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 7u8,
            val: 99u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 126u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 62u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 221u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 27u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 110u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 46u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 189u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 14u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 142u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 78u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 253u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 96u8,
            bits: 7u8,
            val: 0u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 81u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 17u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 21u8,
            bits: 8u8,
            val: 131u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 31u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 113u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 49u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 195u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 10u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 97u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 33u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 163u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 1u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 129u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 65u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 227u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 6u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 89u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 25u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 147u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 59u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 121u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 57u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 211u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 17u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 105u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 41u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 179u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 9u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 137u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 73u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 243u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 4u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 85u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 21u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 8u8,
            val: 258u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 43u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 117u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 53u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 203u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 13u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 101u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 37u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 171u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 5u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 133u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 69u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 235u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 8u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 93u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 29u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 155u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 7u8,
            val: 83u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 125u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 61u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 219u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 23u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 109u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 45u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 187u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 13u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 141u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 77u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 251u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 3u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 83u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 19u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 21u8,
            bits: 8u8,
            val: 195u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 35u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 115u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 51u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 199u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 11u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 99u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 35u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 167u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 3u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 131u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 67u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 231u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 7u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 91u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 27u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 151u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 7u8,
            val: 67u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 123u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 59u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 215u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 19u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 107u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 43u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 183u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 11u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 139u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 75u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 247u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 5u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 87u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 23u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 64u8,
            bits: 8u8,
            val: 0u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 7u8,
            val: 51u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 119u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 55u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 207u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 7u8,
            val: 15u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 103u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 39u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 175u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 7u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 135u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 71u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 239u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 7u8,
            val: 9u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 95u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 31u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 159u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 7u8,
            val: 99u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 127u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 63u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 223u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 7u8,
            val: 27u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 111u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 47u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 191u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 15u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 143u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 8u8,
            val: 79u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 0u8,
            bits: 9u8,
            val: 255u16,
        },
    ]);

static mut distfix: __SlateAlignLargeArray<[__SlateRecord_inftrees_h_28; 32]> =
    __SlateAlignLargeArray([
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 5u8,
            val: 1u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 23u8,
            bits: 5u8,
            val: 257u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 5u8,
            val: 17u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 27u8,
            bits: 5u8,
            val: 4097u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 5u8,
            val: 5u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 25u8,
            bits: 5u8,
            val: 1025u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 21u8,
            bits: 5u8,
            val: 65u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 29u8,
            bits: 5u8,
            val: 16385u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 5u8,
            val: 3u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 24u8,
            bits: 5u8,
            val: 513u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 5u8,
            val: 33u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 28u8,
            bits: 5u8,
            val: 8193u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 5u8,
            val: 9u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 26u8,
            bits: 5u8,
            val: 2049u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 22u8,
            bits: 5u8,
            val: 129u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 64u8,
            bits: 5u8,
            val: 0u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 5u8,
            val: 2u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 23u8,
            bits: 5u8,
            val: 385u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 19u8,
            bits: 5u8,
            val: 25u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 27u8,
            bits: 5u8,
            val: 6145u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 17u8,
            bits: 5u8,
            val: 7u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 25u8,
            bits: 5u8,
            val: 1537u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 21u8,
            bits: 5u8,
            val: 97u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 29u8,
            bits: 5u8,
            val: 24577u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 16u8,
            bits: 5u8,
            val: 4u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 24u8,
            bits: 5u8,
            val: 769u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 20u8,
            bits: 5u8,
            val: 49u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 28u8,
            bits: 5u8,
            val: 12289u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 18u8,
            bits: 5u8,
            val: 13u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 26u8,
            bits: 5u8,
            val: 3073u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 22u8,
            bits: 5u8,
            val: 193u16,
        },
        __SlateRecord_inftrees_h_28 {
            op: 64u8,
            bits: 5u8,
            val: 0u16,
        },
    ]);

/// Return state with length and distance decoding tables and index sizes set to
/// fixed code decoding.  Normally this returns fixed tables from inffixed.h.
/// If BUILDFIXED is defined, then instead this routine builds the tables the
/// first time it's called, and returns those tables the first time and
/// thereafter.  This reduces the size of the code by about 2K bytes, in
/// exchange for a little execution time.  However, BUILDFIXED should not be
/// used for threaded applications if atomics are not available, as it will
/// not be thread-safe.
#[unsafe(no_mangle)]
extern "C-unwind" fn inflate_fixed(state: *mut inflate_state) {
    unsafe {
        (*state).lencode = std::ptr::addr_of!(lenfix.0) as *const __SlateRecord_inftrees_h_28;
    }
    unsafe {
        (*state).lenbits = 9;
    }
    unsafe {
        (*state).distcode = std::ptr::addr_of!(distfix.0) as *const __SlateRecord_inftrees_h_28;
    }
    unsafe {
        (*state).distbits = 5;
    }
}
