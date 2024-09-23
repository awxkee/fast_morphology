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
use crate::filter_op_declare::{Arena, MorthOpFilterFlat2DRow};
use crate::flat_se::AnalyzedSe;
use crate::op_type::MorphOp;
use crate::ops::utils::{rgba_from_slice, write_rgba_to_slice};
use crate::unsafe_slice::UnsafeSlice;
use colorutils_rs::Rgba;
use crate::ImageSize;
use crate::ops::neon::fast_morph_op_4d_neon;

#[derive(Clone)]
pub struct MorphOpFilterRgba2DRow<const OP_TYPE: u8> {}

impl<const OP_TYPE: u8> Default for MorphOpFilterRgba2DRow<OP_TYPE> {
    fn default() -> Self {
        MorphOpFilterRgba2DRow {}
    }
}

impl<const OP_TYPE: u8> MorthOpFilterFlat2DRow for MorphOpFilterRgba2DRow<OP_TYPE> {
    unsafe fn dispatch_row(
        &self,
        src: &[u8],
        dst: &UnsafeSlice<u8>,
        image_size: ImageSize,
        analyzed_se: AnalyzedSe,
        y: usize,
        arena: &Option<Arena>,
    ) {
        let width = image_size.width;
        let height = image_size.height;
        let op_type: MorphOp = OP_TYPE.into();
        let stride = width * 4;

        let base_val = match op_type {
            MorphOp::Dilate => u8::MIN,
            MorphOp::Erode => u8::MAX,
        };

        if let Some(arena) = arena {
            let mut items0 =
                vec![Rgba::dup(base_val); analyzed_se.left_front.element_offsets.len()];

            let arena_stride = arena.width * 4;

            let src = &arena.arena;

            let dx = arena.pad_w as i32;
            let dy = arena.pad_h as i32;

            for x in 0..width {
                let mut iter_index = 0usize;

                for &filter in analyzed_se.left_front.filter_bounds.iter() {
                    let filter_start_x = filter.x + x as i32 + dx;
                    let filter_start_y = filter.y + y as i32 + dy;

                    let filter_size = filter.size as usize;

                    let py = filter_start_y as usize;
                    let py0 = py * arena_stride;

                    let mut current_x = 0usize;

                    while current_x < filter_size {
                        let px = (filter_start_x + current_x as i32) as usize * 4;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = rgba_from_slice(current0);
                        *items0.get_unchecked_mut(iter_index) = new_value0;
                        iter_index += 1;

                        current_x += 1;
                    }
                }

                let px = x * 4;
                let offset0 = y * stride + px;
                write_rgba_to_slice(dst, offset0, fast_morph_op_4d_neon::<OP_TYPE>(items0.get_unchecked(..iter_index)));
            }
        } else {
            let max_width = width as i32 - 1;
            let max_height = height as i32 - 1;

            for x in 0..width {
                let mut values0 = Rgba::dup(base_val);

                for &filter in analyzed_se.left_front.filter_bounds.iter() {
                    let filter_start_x = filter.x + x as i32;
                    let filter_start_y = filter.y + y as i32;
                    if filter_start_y < 0 {
                        continue;
                    }
                    if (filter_start_x + filter.size as i32) < 0 {
                        continue;
                    }
                    let mut filter_size = if filter_start_x < 0 {
                        (filter.size as i32 - filter.x.abs()) as usize
                    } else {
                        filter.size as usize
                    };
                    if filter_size as i32 + filter_start_x >= width as i32 {
                        filter_size = (width as i32 - filter_start_x) as usize;
                    }

                    let py = filter_start_y;
                    let py0 = py.min(max_height).max(0) as usize * stride;

                    let mut current_x = 0usize;

                    while current_x < filter_size {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize * 4;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = rgba_from_slice(current0);
                        values0 = match op_type {
                            MorphOp::Dilate => values0.max_p(new_value0),
                            MorphOp::Erode => values0.min_p(new_value0),
                        };

                        current_x += 1;
                    }
                }

                let px = x * 4;
                let offset0 = y * stride + px;
                write_rgba_to_slice(dst, offset0, values0);
            }
        }
    }
}
