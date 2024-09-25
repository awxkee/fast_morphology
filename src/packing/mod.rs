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
pub mod avx;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod neon;
mod pack_rgb;
mod pack_rgba;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod sse;
mod traits;
mod unpack_rgb;
mod unpack_rgba;
mod unpacked_rgb_image;
mod unpacked_rgba_image;
mod unpack_gray_alpha;
mod unpacked_gray_alpha;
mod pack_gray_alpha;

pub use pack_rgb::pack_rgb;
pub use traits::{RgbPackable, RgbaPackable, GrayAlphaPackable};
pub use unpack_rgb::unpack_rgb;
pub use unpack_rgba::unpack_rgba;
pub use unpacked_rgb_image::UnpackedRgbImage;
pub use unpacked_rgba_image::UnpackedRgbaImage;
pub use unpacked_gray_alpha::UnpackedGrayAlpha;