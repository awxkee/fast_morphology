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
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_load_deinterleave_rgb(ptr: *const u8) -> (__m128i, __m128i, __m128i) {
    let row0 = _mm_loadu_si128(ptr as *const __m128i);
    let row1 = _mm_loadu_si128(ptr.add(16) as *const __m128i);
    let row2 = _mm_loadu_si128(ptr.add(32) as *const __m128i);
    _mm_deinterleave_rgb(row0, row1, row2)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_load_deinterleave_rgba(ptr: *const u8) -> (__m128i, __m128i, __m128i, __m128i) {
    let row0 = _mm_loadu_si128(ptr as *const __m128i);
    let row1 = _mm_loadu_si128(ptr.add(16) as *const __m128i);
    let row2 = _mm_loadu_si128(ptr.add(32) as *const __m128i);
    let row3 = _mm_loadu_si128(ptr.add(48) as *const __m128i);
    _mm_deinterleave_rgba(row0, row1, row2, row3)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_load_deinterleave_half_rgba(
    ptr: *const u8,
    fill: u8,
) -> (__m128i, __m128i, __m128i, __m128i) {
    let row0 = _mm_loadu_si128(ptr as *const __m128i);
    let row1 = _mm_loadu_si128(ptr.add(16) as *const __m128i);
    let v_fill = _mm_set1_epi8(fill as i8);
    _mm_deinterleave_rgba(row0, row1, v_fill, v_fill)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_load_deinterleave_quart_rgba(
    ptr: *const u8,
    fill: u8,
) -> (__m128i, __m128i, __m128i, __m128i) {
    let row0 = _mm_loadu_si128(ptr as *const __m128i);
    let v_fill = _mm_set1_epi8(fill as i8);
    _mm_deinterleave_rgba(row0, v_fill, v_fill, v_fill)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_load_deinterleave_half_rgb(
    ptr: *const u8,
    fill: u8,
) -> (__m128i, __m128i, __m128i) {
    let row0 = _mm_loadu_si128(ptr as *const __m128i);
    let upper_fix = _mm_set_epi8(
        fill as i8, fill as i8, fill as i8, fill as i8, fill as i8, fill as i8, fill as i8,
        fill as i8, 0, 0, 0, 0, 0, 0, 0, 0,
    );
    let row1 = _mm_or_si128(_mm_loadu_si64(ptr.add(16)), upper_fix);
    let row2 = _mm_set1_epi8(fill as i8);
    _mm_deinterleave_rgb(row0, row1, row2)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_load_deinterleave_quart_rgb(
    ptr: *const u8,
    fill: u8,
) -> (__m128i, __m128i, __m128i) {
    let mut row0 = _mm_loadu_si64(ptr);
    let loaded_val = (ptr.add(8) as *mut u32).read_unaligned();
    row0 = _mm_insert_epi32::<2>(row0, loaded_val as i32);
    let def = _mm_set1_epi8(fill as i8);
    _mm_deinterleave_rgb(row0, def, def)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_deinterleave_rgb(
    rgb0: __m128i,
    rgb1: __m128i,
    rgb2: __m128i,
) -> (__m128i, __m128i, __m128i) {
    #[rustfmt::skip]
    let idx = _mm_setr_epi8(0, 3, 6, 9,
                                    12, 15, 2, 5, 8,
                                    11, 14, 1, 4, 7,
                                    10, 13);

    let r6b5g5_0 = _mm_shuffle_epi8(rgb0, idx);
    let g6r5b5_1 = _mm_shuffle_epi8(rgb1, idx);
    let b6g5r5_2 = _mm_shuffle_epi8(rgb2, idx);

    #[rustfmt::skip]
    let mask010 = _mm_setr_epi8(0, 0, 0, 0,
                                        0, 0, -1, -1, -1,
                                        -1, -1, 0, 0, 0,
                                        0, 0);

    #[rustfmt::skip]
    let mask001 = _mm_setr_epi8(0, 0, 0, 0, 0,
                                    0, 0, 0, 0, 0, 0,
                                    -1, -1, -1, -1, -1);

    let b2g2b1 = _mm_blendv_epi8(b6g5r5_2, g6r5b5_1, mask001);
    let b2b0b1 = _mm_blendv_epi8(b2g2b1, r6b5g5_0, mask010);

    let r0r1b1 = _mm_blendv_epi8(r6b5g5_0, g6r5b5_1, mask010);
    let r0r1r2 = _mm_blendv_epi8(r0r1b1, b6g5r5_2, mask001);

    let g1r1g0 = _mm_blendv_epi8(g6r5b5_1, r6b5g5_0, mask001);
    let g1g2g0 = _mm_blendv_epi8(g1r1g0, b6g5r5_2, mask010);

    let g0g1g2 = _mm_alignr_epi8::<11>(g1g2g0, g1g2g0);
    let b0b1b2 = _mm_alignr_epi8::<6>(b2b0b1, b2b0b1);

    (r0r1r2, g0g1g2, b0b1b2)
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_deinterleave_rgba(
    rgba0: __m128i,
    rgba1: __m128i,
    rgba2: __m128i,
    rgba3: __m128i,
) -> (__m128i, __m128i, __m128i, __m128i) {
    let t0 = _mm_unpacklo_epi8(rgba0, rgba1); // r1 R1 g1 G1 b1 B1 a1 A1 r2 R2 g2 G2 b2 B2 a2 A2
    let t1 = _mm_unpackhi_epi8(rgba0, rgba1);
    let t2 = _mm_unpacklo_epi8(rgba2, rgba3); // r4 R4 g4 G4 b4 B4 a4 A4 r5 R5 g5 G5 b5 B5 a5 A5
    let t3 = _mm_unpackhi_epi8(rgba2, rgba3);

    let t4 = _mm_unpacklo_epi16(t0, t2); // r1 R1 r4 R4 g1 G1 G4 g4 G4 b1 B1 b4 B4 a1 A1 a4 A4
    let t5 = _mm_unpackhi_epi16(t0, t2);
    let t6 = _mm_unpacklo_epi16(t1, t3);
    let t7 = _mm_unpackhi_epi16(t1, t3);

    let l1 = _mm_unpacklo_epi32(t4, t6); // r1 R1 r4 R4 g1 G1 G4 g4 G4 b1 B1 b4 B4 a1 A1 a4 A4
    let l2 = _mm_unpackhi_epi32(t4, t6);
    let l3 = _mm_unpacklo_epi32(t5, t7);
    let l4 = _mm_unpackhi_epi32(t5, t7);

    #[rustfmt::skip]
    let shuffle = _mm_setr_epi8(0, 4, 8, 12,
                                        1, 5, 9, 13,
                                        2, 6, 10, 14,
                                        3, 7, 11, 15,
    );

    let r1 = _mm_shuffle_epi8(_mm_unpacklo_epi32(l1, l3), shuffle);
    let r2 = _mm_shuffle_epi8(_mm_unpackhi_epi32(l1, l3), shuffle);
    let r3 = _mm_shuffle_epi8(_mm_unpacklo_epi32(l2, l4), shuffle);
    let r4 = _mm_shuffle_epi8(_mm_unpackhi_epi32(l2, l4), shuffle);

    (r1, r2, r3, r4)
}
