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
pub unsafe fn _mm_hmax_epu8(v: __m128i) -> u8 {
    let mut vmax = v;
    vmax = _mm_max_epu8(vmax, _mm_alignr_epi8::<1>(vmax, vmax));
    vmax = _mm_max_epu8(vmax, _mm_alignr_epi8::<2>(vmax, vmax));
    vmax = _mm_max_epu8(vmax, _mm_alignr_epi8::<4>(vmax, vmax));
    vmax = _mm_max_epu8(vmax, _mm_alignr_epi8::<8>(vmax, vmax));
    _mm_extract_epi8(vmax) as u8
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_hmin_epu8(v: __m128i) -> u8 {
    let mut vmax = v;
    vmax = _mm_min_epu8(vmax, _mm_alignr_epi8::<1>(vmax, vmax));
    vmax = _mm_min_epu8(vmax, _mm_alignr_epi8::<2>(vmax, vmax));
    vmax = _mm_min_epu8(vmax, _mm_alignr_epi8::<4>(vmax, vmax));
    vmax = _mm_min_epu8(vmax, _mm_alignr_epi8::<8>(vmax, vmax));
    _mm_extract_epi8(vmax) as u8
}
