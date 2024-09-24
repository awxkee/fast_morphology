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
use crate::op_type::MorphOp;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::ops::avx::{fast_morph_op_1d_avx, fast_morph_op_3d_avx, fast_morph_op_4d_avx};
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use crate::ops::neon::{fast_morph_op_1d_neon, fast_morph_op_3d_neon, fast_morph_op_4d_neon};
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::ops::sse::{fast_morph_op_1d_sse, fast_morph_op_3d_sse, fast_morph_op_4d_sse};
use colorutils_rs::{Rgb, Rgba};

#[inline]
fn fast_morph_op_1d_cpu<const OP_TYPE: u8>(slice: &[u8]) -> u8 {
    let op_type: MorphOp = OP_TYPE.into();
    let mut best_val = match op_type {
        MorphOp::Dilate => u8::MIN,
        MorphOp::Erode => u8::MAX,
    };
    for &element in slice {
        best_val = match op_type {
            MorphOp::Dilate => best_val.max(element),
            MorphOp::Erode => best_val.min(element),
        }
    }
    best_val
}

#[inline]
pub fn fast_morph_op_1d<const OP_TYPE: u8>() -> fn(&[u8]) -> u8 {
    let mut _dispatcher: fn(&[u8]) -> u8 = fast_morph_op_1d_cpu::<OP_TYPE>;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        _dispatcher = fast_morph_op_1d_neon::<OP_TYPE>;
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if std::arch::is_x86_feature_detected!("sse4.1") {
            _dispatcher = fast_morph_op_1d_sse::<OP_TYPE>;
        }
        if std::arch::is_x86_feature_detected!("avx2") {
            _dispatcher = fast_morph_op_1d_avx::<OP_TYPE>;
        }
    }
    _dispatcher
}

#[inline]
fn fast_morph_op_3d_cpu<const OP_TYPE: u8>(slice: &[Rgb<u8>]) -> Rgb<u8> {
    let op_type: MorphOp = OP_TYPE.into();
    let mut best_val = Rgb::dup(match op_type {
        MorphOp::Dilate => u8::MIN,
        MorphOp::Erode => u8::MAX,
    });
    for &element in slice {
        best_val = match op_type {
            MorphOp::Dilate => best_val.max_p(element),
            MorphOp::Erode => best_val.min_p(element),
        }
    }
    best_val
}

#[inline]
pub fn fast_morph_op_3d<const OP_TYPE: u8>() -> fn(&[Rgb<u8>]) -> Rgb<u8> {
    let mut _dispatcher: fn(&[Rgb<u8>]) -> Rgb<u8> = fast_morph_op_3d_cpu::<OP_TYPE>;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        _dispatcher = fast_morph_op_3d_neon::<OP_TYPE>;
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if std::arch::is_x86_feature_detected!("sse4.1") {
            _dispatcher = fast_morph_op_3d_sse::<OP_TYPE>;
        }
        if std::arch::is_x86_feature_detected!("avx2") {
            _dispatcher = fast_morph_op_3d_avx::<OP_TYPE>;
        }
    }
    _dispatcher
}

#[inline]
fn fast_morph_op_4d_cpu<const OP_TYPE: u8>(slice: &[Rgba<u8>]) -> Rgba<u8> {
    let op_type: MorphOp = OP_TYPE.into();
    let mut best_val = Rgba::dup(match op_type {
        MorphOp::Dilate => u8::MIN,
        MorphOp::Erode => u8::MAX,
    });
    for &element in slice {
        best_val = match op_type {
            MorphOp::Dilate => best_val.max_p(element),
            MorphOp::Erode => best_val.min_p(element),
        }
    }
    best_val
}

#[inline]
pub fn fast_morph_op_4d<const OP_TYPE: u8>() -> fn(&[Rgba<u8>]) -> Rgba<u8> {
    let mut _dispatcher: fn(&[Rgba<u8>]) -> Rgba<u8> = fast_morph_op_4d_cpu::<OP_TYPE>;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        _dispatcher = fast_morph_op_4d_neon::<OP_TYPE>;
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if std::arch::is_x86_feature_detected!("sse4.1") {
            _dispatcher = fast_morph_op_4d_sse::<OP_TYPE>;
        }
        if std::arch::is_x86_feature_detected!("avx2") {
            _dispatcher = fast_morph_op_4d_avx::<OP_TYPE>;
        }
    }
    _dispatcher
}
