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
pub fn morph_gradient_sse(dilation: &[u8], erosion: &[u8], dst: &mut [u8]) {
    unsafe {
        morph_gradient_sse_impl(dilation, erosion, dst);
    }
}
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn morph_gradient_sse_impl(dilation: &[u8], erosion: &[u8], dst: &mut [u8]) {
    if dilation.len() != erosion.len() || erosion.len() != dst.len() {
        panic!(
            "All array must match in size for gradient but received v0: {}, v1: {}, v2: {}",
            dilation.len(),
            erosion.len(),
            dst.len()
        );
    }
    let length = dilation.len();
    let mut _cx = 0usize;
    unsafe {
        while _cx + 64 < length {
            let v0_ptr = dilation.get_unchecked(_cx..).as_ptr();
            let v0_set = (
                _mm_loadu_si128(v0_ptr as *const __m128i),
                _mm_loadu_si128(v0_ptr.add(16) as *const __m128i),
                _mm_loadu_si128(v0_ptr.add(32) as *const __m128i),
                _mm_loadu_si128(v0_ptr.add(48) as *const __m128i),
            );
            let v1_ptr = erosion.get_unchecked(_cx..).as_ptr();
            let v1_set = (
                _mm_loadu_si128(v1_ptr as *const __m128i),
                _mm_loadu_si128(v1_ptr.add(16) as *const __m128i),
                _mm_loadu_si128(v1_ptr.add(32) as *const __m128i),
                _mm_loadu_si128(v1_ptr.add(48) as *const __m128i),
            );
            let result_set = (
                _mm_subs_epu8(v0_set.0, v1_set.0),
                _mm_subs_epu8(v0_set.1, v1_set.1),
                _mm_subs_epu8(v0_set.2, v1_set.2),
                _mm_subs_epu8(v0_set.3, v1_set.3),
            );
            let v_dst_ptr = dst.get_unchecked_mut(_cx..).as_mut_ptr();
            _mm_storeu_si128(v_dst_ptr as *mut __m128i, result_set.0);
            _mm_storeu_si128(v_dst_ptr.add(16) as *mut __m128i, result_set.1);
            _mm_storeu_si128(v_dst_ptr.add(32) as *mut __m128i, result_set.2);
            _mm_storeu_si128(v_dst_ptr.add(48) as *mut __m128i, result_set.3);
            _cx += 64;
        }
        while _cx + 32 < length {
            let v0_ptr = dilation.get_unchecked(_cx..).as_ptr();
            let v0_set = (
                _mm_loadu_si128(v0_ptr as *const __m128i),
                _mm_loadu_si128(v0_ptr.add(16) as *const __m128i),
            );
            let v1_ptr = erosion.get_unchecked(_cx..).as_ptr();
            let v1_set = (
                _mm_loadu_si128(v1_ptr as *const __m128i),
                _mm_loadu_si128(v1_ptr.add(16) as *const __m128i),
            );
            let result_set = (
                _mm_subs_epu8(v0_set.0, v1_set.0),
                _mm_subs_epu8(v0_set.1, v1_set.1),
            );
            let v_dst_ptr = dst.get_unchecked_mut(_cx..).as_mut_ptr();
            _mm_storeu_si128(v_dst_ptr as *mut __m128i, result_set.0);
            _mm_storeu_si128(v_dst_ptr.add(16) as *mut __m128i, result_set.1);
            _cx += 32;
        }
        while _cx + 16 < length {
            let v0_ptr = dilation.get_unchecked(_cx..).as_ptr();
            let v0_set = _mm_loadu_si128(v0_ptr as *const __m128i);
            let v1_ptr = erosion.get_unchecked(_cx..).as_ptr();
            let v1_set = _mm_loadu_si128(v1_ptr as *const __m128i);
            let result_set = _mm_subs_epu8(v0_set, v1_set);
            let v_dst_ptr = dst.get_unchecked_mut(_cx..).as_mut_ptr();
            _mm_storeu_si128(v_dst_ptr as *mut __m128i, result_set);
            _cx += 16;
        }
        while _cx < length {
            *dst.get_unchecked_mut(_cx) = dilation
                .get_unchecked(_cx)
                .saturating_sub(*erosion.get_unchecked(_cx));
            _cx += 1;
        }
    }
}
