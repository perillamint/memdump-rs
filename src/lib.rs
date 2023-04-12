/*
 * SPDX-FileCopyrightText: 2022 perillamint
 *
 * SPDX-License-Identifier: MIT
 *
 * Lots of its code originated from https://github.com/pwnwriter/saika/blob/main/src/main.rs
 * and https://stackoverflow.com/questions/39488327/how-to-format-output-to-a-byte-array-with-no-std-and-no-allocator
 */

use core::fmt::Write;
use core::ptr::read_volatile;
use core::slice;
use core::str::from_utf8_unchecked;

struct Wrapper<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> Wrapper<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        Wrapper { buf, offset: 0 }
    }
}

impl<'a> core::fmt::Write for Wrapper<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();

        // Skip over already-copied data
        let remainder = &mut self.buf[self.offset..];
        // Check if there is space remaining (return error instead of panicking)
        if remainder.len() < bytes.len() {
            return Err(core::fmt::Error);
        }
        // Make the two slices the same length
        let remainder = &mut remainder[..bytes.len()];
        // Copy
        remainder.copy_from_slice(bytes);

        // Update offset to avoid overwriting
        self.offset += bytes.len();

        Ok(())
    }
}

/// This function accepts *u8 raw pointer and dumps it to the defmt log.
/// Due to its nature, it is unsafe.
pub unsafe fn memdump<F>(buf: *const u8, len: usize, printfn: F)
where
    F: Fn(&str),
{
    let mut offset = 0;
    while offset < len {
        let chunklen = if len - offset > 16 { 16 } else { len - offset };
        let chunk = slice::from_raw_parts(buf.add(offset), chunklen);
        offset += chunklen;

        // 88-byte length buffer;
        let mut outbuf = [0u8; 96];
        let mut bufwrap = Wrapper::new(&mut outbuf);

        write!(bufwrap, "{:018p} | ", chunk.as_ptr()).unwrap();

        for (i, item) in chunk.iter().enumerate().take(chunklen) {
            write!(bufwrap, "{:02x} ", read_volatile(item)).unwrap();
            if i % 8 == 7 {
                write!(bufwrap, " ").unwrap();
            }
        }

        for i in chunklen..16 {
            write!(bufwrap, "   ").unwrap();
            if i % 8 == 7 {
                write!(bufwrap, " ").unwrap();
            }
        }

        write!(bufwrap, " |").unwrap();
        for item in chunk.iter().take(chunklen) {
            let c = read_volatile(item) as char;
            if c.is_ascii_graphic() {
                write!(bufwrap, "{}", c).unwrap();
            } else {
                write!(bufwrap, ".").unwrap();
            }
        }
        write!(bufwrap, "|").unwrap();

        printfn(from_utf8_unchecked(&outbuf));
    }
}
