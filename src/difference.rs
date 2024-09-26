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
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::avx::morph_gradient_avx;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use crate::neon::morph_gradient_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::sse::morph_gradient_sse;
use num_traits::SaturatingSub;
use std::ops::Sub;

pub trait MorphGradient<T> {
    fn morph_gradient(dilation: &[T], erosion: &[T], dst: &mut [T]);
}

fn make_morph_gradient_sat<T>(dilation: &[T], erosion: &[T], dst: &mut [T])
where
    T: SaturatingSub + Default + Clone + Copy,
{
    for ((dilation, erosion), dst) in dilation.iter().zip(erosion.iter()).zip(dst.iter_mut()) {
        *dst = dilation.saturating_sub(erosion);
    }
}

fn make_morph_gradient<T>(dilation: &[T], erosion: &[T], dst: &mut [T])
where
    T: Sub<Output = T> + Default + Clone + Copy,
{
    for ((dilation, erosion), dst) in dilation.iter().zip(erosion.iter()).zip(dst.iter_mut()) {
        *dst = *dilation - *erosion;
    }
}

impl MorphGradient<u8> for u8 {
    fn morph_gradient(dilation: &[u8], erosion: &[u8], dst: &mut [u8]) {
        let mut _dispatcher: fn(&[u8], &[u8], &mut [u8]) = make_morph_gradient_sat;
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        {
            _dispatcher = morph_gradient_neon;
        }
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if std::arch::is_x86_feature_detected!("sse4.1") {
                _dispatcher = morph_gradient_sse;
            }
            if std::arch::is_x86_feature_detected!("avx2") {
                _dispatcher = morph_gradient_avx;
            }
        }
        _dispatcher(dilation, erosion, dst)
    }
}

impl MorphGradient<u16> for u16 {
    fn morph_gradient(dilation: &[u16], erosion: &[u16], dst: &mut [u16]) {
        make_morph_gradient_sat(dilation, erosion, dst)
    }
}

impl MorphGradient<f32> for f32 {
    fn morph_gradient(dilation: &[f32], erosion: &[f32], dst: &mut [f32]) {
        make_morph_gradient(dilation, erosion, dst)
    }
}
