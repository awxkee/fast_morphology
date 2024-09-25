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
#![allow(clippy::too_many_arguments)]
extern crate core;

mod arena;
mod arena_roi;
mod border_mode;
#[cfg(feature = "image")]
mod dynamic_image;
mod filter;
mod filter_op_declare;
mod flat_se;
mod img_size;
mod morph_base;
mod morph_gray_alpha;
mod morph_rgb;
mod morph_rgba;
mod op;
mod op_f32;
mod op_impl;
mod op_type;
mod op_u16;
mod ops;
mod packing;
mod se_scan;
mod structuring_element;
mod thread_policy;
mod unsafe_slice;

pub use border_mode::BorderMode;
#[cfg(feature = "image")]
pub use dynamic_image::*;
pub use img_size::ImageSize;
pub use op::dilate;
pub use op::dilate_gray_alpha;
pub use op::dilate_rgb;
pub use op::dilate_rgba;
pub use op::erode;
pub use op::erode_gray_alpha;
pub use op::erode_rgb;
pub use op::erode_rgba;
pub use op::morphology;
pub use op::morphology_gray_alpha;
pub use op::morphology_rgb;
pub use op::morphology_rgba;
pub use op_f32::dilate_f32;
pub use op_f32::dilate_gray_alpha_f32;
pub use op_f32::dilate_rgb_f32;
pub use op_f32::dilate_rgba_f32;
pub use op_f32::erode_f32;
pub use op_f32::erode_gray_alpha_f32;
pub use op_f32::erode_rgb_f32;
pub use op_f32::erode_rgba_f32;
pub use op_f32::morphology_rgb_f32;
pub use op_f32::morphology_rgba_f32;
pub use op_type::MorphExOp;
pub use op_u16::dilate_gray_alpha_u16;
pub use op_u16::dilate_rgb_u16;
pub use op_u16::dilate_rgba_u16;
pub use op_u16::dilate_u16;
pub use op_u16::erode_gray_alpha_u16;
pub use op_u16::erode_rgb_u16;
pub use op_u16::erode_rgba_u16;
pub use op_u16::erode_u16;
pub use op_u16::morphology_gray_alpha_u16;
pub use op_u16::morphology_gray_u16;
pub use op_u16::morphology_rgb_u16;
pub use op_u16::morphology_rgba_u16;
pub use structuring_element::KernelShape;
pub use thread_policy::MorphologyThreadingPolicy;
