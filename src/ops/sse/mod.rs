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
mod hminmax;
mod morph_op;
mod morph_op_4_rows;
mod morph_rgb;
mod morph_rgb_4_rows;
mod morph_rgba;
mod morph_rgba_4_rows;
mod op;
mod v_load;

pub use hminmax::{_mm_hmax_epu8, _mm_hmin_epu8};
pub use morph_op::MorphOpFilterSse2DRow;
pub use morph_op_4_rows::MorphOpFilterSse2D4Rows;
pub use morph_rgb::MorphOpFilterRgbSse2DRow;
pub use morph_rgb_4_rows::MorphOpFilterRgbSse2D4Rows;
pub use morph_rgba::MorphOpFilterRgbaSse2DRow;
pub use morph_rgba_4_rows::MorphOpFilterRgbaSse2D4Rows;
pub use op::{fast_morph_op_1d_sse, fast_morph_op_3d_sse, fast_morph_op_4d_sse};
pub use v_load::*;
