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
use crate::ops::op::fast_morph_op_1d;
use crate::ops::smart_allocator::SmartAllocator;
use crate::unsafe_slice::UnsafeSlice;
use crate::ImageSize;

#[derive(Clone)]
pub struct MorphFilterFlat2DRow<const OP_TYPE: u8> {}

impl<const OP_TYPE: u8> Default for MorphFilterFlat2DRow<OP_TYPE> {
    fn default() -> Self {
        MorphFilterFlat2DRow {}
    }
}

impl<const OP_TYPE: u8> MorthOpFilterFlat2DRow for MorphFilterFlat2DRow<OP_TYPE> {
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
        if let Some(arena) = arena {
            let minmax_resolver = fast_morph_op_1d::<OP_TYPE>();
            let op_type: MorphOp = OP_TYPE.into();
            let stride = image_size.width;

            let base_val = match op_type {
                MorphOp::Dilate => u8::MIN,
                MorphOp::Erode => u8::MAX,
            };

            let src = &arena.arena;

            let size = analyzed_se.left_front.element_offsets.len();
            let mut allocated_window_0 = SmartAllocator::new(base_val, size);
            let items = allocated_window_0.as_mut_slice();

            let dx = arena.pad_w as i32;
            let dy = arena.pad_h as i32;

            let arena_width = arena.width;

            for x in 0..width {
                let chunk_size = 4;
                let iterator_4 = analyzed_se
                    .left_front
                    .element_offsets
                    .chunks_exact(chunk_size);
                let rem = iterator_4.remainder();
                for (index, filter) in iterator_4.enumerate() {
                    let filter_start_x0 = filter[0].x + x as i32 + dx;
                    let filter_start_y0 = filter[0].y + y as i32 + dy;

                    let img_index =
                        filter_start_x0 as usize + filter_start_y0 as usize * arena_width;
                    let new_value0 = *src.get_unchecked(img_index);
                    *items.get_unchecked_mut(index * 4) = new_value0;

                    let filter_start_x1 = filter[1].x + x as i32 + dx;
                    let filter_start_y1 = filter[1].y + y as i32 + dy;

                    let img_index =
                        filter_start_x1 as usize + filter_start_y1 as usize * arena_width;
                    let new_value1 = *src.get_unchecked(img_index);
                    *items.get_unchecked_mut(index * 4 + 1) = new_value1;

                    let filter_start_x2 = filter[2].x + x as i32 + dx;
                    let filter_start_y2 = filter[2].y + y as i32 + dy;

                    let img_index =
                        filter_start_x2 as usize + filter_start_y2 as usize * arena_width;
                    let new_value2 = *src.get_unchecked(img_index);
                    *items.get_unchecked_mut(index * 4 + 2) = new_value2;

                    let filter_start_x3 = filter[3].x + x as i32 + dx;
                    let filter_start_y3 = filter[3].y + y as i32 + dy;

                    let img_index =
                        filter_start_x3 as usize + filter_start_y3 as usize * arena_width;
                    let new_value3 = *src.get_unchecked(img_index);
                    *items.get_unchecked_mut(index * 4 + 3) = new_value3;
                }

                for (index, &filter) in rem.iter().enumerate() {
                    let filter_start_x = filter.x + x as i32 + dx;
                    let filter_start_y = filter.y + y as i32 + dy;

                    let img_index = filter_start_x as usize + filter_start_y as usize * arena_width;
                    let new_value0 = *src.get_unchecked(img_index);
                    *items.get_unchecked_mut(index) = new_value0;
                }

                dst.write(y * stride + x, minmax_resolver(items));
            }
        } else {
            let op_type: MorphOp = OP_TYPE.into();
            let stride = width;
            let max_width = width as i32 - 1;
            let max_height = height as i32 - 1;

            let base_val = match op_type {
                MorphOp::Dilate => u8::MIN,
                MorphOp::Erode => u8::MAX,
            };

            for x in 0..width {
                let mut value0 = base_val;

                for &filter in analyzed_se.left_front.filter_bounds.iter() {
                    let filter_start_x = filter.x + x as i32;
                    let filter_start_y = filter.y + y as i32;
                    if filter_start_y < -1 {
                        continue;
                    }
                    if (filter_start_x + filter.size as i32) < 0 {
                        continue;
                    }
                    let mut filter_size = filter.size as usize;
                    if filter_size as i32 + filter_start_x >= width as i32 {
                        filter_size = (width as i32 - filter_start_x) as usize;
                    }

                    let mut current_x = 0usize;

                    let py = filter_start_y;
                    let py0 = py.min(max_height).max(0) as usize * stride;

                    while current_x < filter_size {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize;
                        let base_offset0 = py0 + px;
                        let new_value0 = *src.get_unchecked(base_offset0);
                        value0 = match op_type {
                            MorphOp::Dilate => new_value0.max(value0),
                            MorphOp::Erode => new_value0.min(value0),
                        };
                        current_x += 1;
                    }
                }

                dst.write(y * stride + x, value0);
            }
        }
    }
}
