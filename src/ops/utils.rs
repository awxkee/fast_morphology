/*
 * Copyright (c) Radzivon Bartoshyk. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1.  Redistributions of source code must retain the above copyright notice, this
 * list of conditions and the following disclaimer.
 *
 * 2.  Redistributions in binary form must reproduce the above copyright notice,
 * this list of conditions and the following disclaimer in the documentation
 * and/or other materials provided with the distribution.
 *
 * 3.  Neither the name of the copyright holder nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
use crate::unsafe_slice::UnsafeSlice;
use colorutils_rs::{Rgb, Rgba};

#[inline]
pub fn rgba_from_slice(slice: &[u8]) -> Rgba<u8> {
    unsafe {
        let bytes = (slice.as_ptr() as *const u32)
            .read_unaligned()
            .to_le_bytes();
        Rgba::new(bytes[0], bytes[1], bytes[2], bytes[3])
    }
}

#[inline]
pub fn rgb_from_slice(slice: &[u8]) -> Rgb<u8> {
    unsafe {
        Rgb::new(
            *slice.get_unchecked(0),
            *slice.get_unchecked(1),
            *slice.get_unchecked(2),
        )
    }
}

#[inline]
pub fn write_rgba_to_slice(cell: &UnsafeSlice<u8>, pos: usize, pixel: Rgba<u8>) {
    unsafe {
        let ptr = cell.slice.as_ptr().add(pos) as *mut u32;
        let px = u32::from_le_bytes([pixel.r, pixel.g, pixel.b, pixel.a]);
        ptr.write_unaligned(px);
    }
}

#[inline]
pub fn write_rgb_to_slice(cell: &UnsafeSlice<u8>, pos: usize, pixel: Rgb<u8>) {
    unsafe {
        cell.write(pos, pixel.r);
        cell.write(pos + 1, pixel.g);
        cell.write(pos + 2, pixel.b);
    }
}
