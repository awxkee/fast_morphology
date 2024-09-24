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
mod morph_row_op;
mod morph_row_rgb_op;
mod morph_row_rgba_op;
mod morph_rows_4_op;
mod morph_rows_rgb_4_op;
mod morph_rows_rgba_4_op;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub mod neon;
mod op;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod sse;
mod utils;

pub use morph_row_op::MorphFilterFlat2DRow;
pub use morph_row_rgb_op::MorphOpFilterRgb2DRow;
pub use morph_row_rgba_op::MorphOpFilterRgba2DRow;
pub use morph_rows_4_op::MorphFilterFlat2D4Rows;
pub use morph_rows_rgb_4_op::MorphOpFilterRgb2D4Rows;
pub use morph_rows_rgba_4_op::MorphOpFilterRgba2D4Rows;
