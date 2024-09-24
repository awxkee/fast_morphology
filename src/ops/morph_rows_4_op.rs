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
pub struct MorphFilterFlat2D4Rows<const OP_TYPE: u8> {}

impl<const OP_TYPE: u8> Default for MorphFilterFlat2D4Rows<OP_TYPE> {
    fn default() -> Self {
        MorphFilterFlat2D4Rows {}
    }
}

impl<const OP_TYPE: u8> MorthOpFilterFlat2DRow for MorphFilterFlat2D4Rows<OP_TYPE> {
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
        let base_val = match op_type {
            MorphOp::Dilate => u8::MIN,
            MorphOp::Erode => u8::MAX,
        };
        let stride = width;

        let size = analyzed_se.left_front.element_offsets.len();

        let mut allocated_stack0 = SmartAllocator::new(base_val, size);
        let mut allocated_stack1 = SmartAllocator::new(base_val, size);
        let mut allocated_stack2 = SmartAllocator::new(base_val, size);
        let mut allocated_stack3 = SmartAllocator::new(base_val, size);
        let mut allocated_stack4 = SmartAllocator::new(base_val, size);
        let mut allocated_stack5 = SmartAllocator::new(base_val, size);

        let items0 = allocated_stack0.as_mut_slice();
        let items1 = allocated_stack1.as_mut_slice();
        let items2 = allocated_stack2.as_mut_slice();
        let items3 = allocated_stack3.as_mut_slice();
        let items4 = allocated_stack4.as_mut_slice();
        let items5 = allocated_stack5.as_mut_slice();

        let minmax_resolver = fast_morph_op_1d::<OP_TYPE>();

        if let Some(arena) = arena {
            let dx = arena.pad_w as i32;
            let dy = arena.pad_h as i32;

            let src = &arena.arena;

            let arena_width = arena.width;

            for x in 0..width {
                for (index, &filter) in analyzed_se.left_front.element_offsets.iter().enumerate() {
                    let filter_start_x0 = filter.x + x as i32 + dx;
                    let filter_start_y0 = filter.y + y as i32 + dy;

                    let img_index =
                        filter_start_x0 as usize + filter_start_y0 as usize * arena_width;
                    let new_value0 = *src.get_unchecked(img_index);
                    *items0.get_unchecked_mut(index) = new_value0;

                    let filter_start_y1 = filter_start_y0 + 1;
                    let img_index =
                        filter_start_x0 as usize + filter_start_y1 as usize * arena_width;
                    let new_value1 = *src.get_unchecked(img_index);
                    *items1.get_unchecked_mut(index) = new_value1;

                    let filter_start_y2 = filter_start_y1 + 1;
                    let img_index =
                        filter_start_x0 as usize + filter_start_y2 as usize * arena_width;
                    let new_value2 = *src.get_unchecked(img_index);
                    *items2.get_unchecked_mut(index) = new_value2;

                    let filter_start_y3 = filter_start_y2 + 1;
                    let img_index =
                        filter_start_x0 as usize + filter_start_y3 as usize * arena_width;
                    let new_value3 = *src.get_unchecked(img_index);
                    *items3.get_unchecked_mut(index) = new_value3;

                    let filter_start_y4 = filter_start_y3 + 1;
                    let img_index =
                        filter_start_x0 as usize + filter_start_y4 as usize * arena_width;
                    let new_value4 = *src.get_unchecked(img_index);
                    *items4.get_unchecked_mut(index) = new_value4;

                    let filter_start_y5 = filter_start_y4 + 1;
                    let img_index =
                        filter_start_x0 as usize + filter_start_y5 as usize * arena_width;
                    let new_value5 = *src.get_unchecked(img_index);
                    *items5.get_unchecked_mut(index) = new_value5;
                }

                dst.write(y * stride + x, minmax_resolver(
                    items0));
                dst.write((y + 1) * stride + x, minmax_resolver(items1));
                dst.write((y + 2) * stride + x, minmax_resolver(items2));
                dst.write((y + 3) * stride + x, minmax_resolver(items3));
                dst.write((y + 4) * stride + x, minmax_resolver(items4));
                dst.write((y + 5) * stride + x, minmax_resolver(items5));
            }
        } else {
            let max_width = width as i32 - 1;
            let max_height = height as i32 - 1;

            for x in 0..width {
                let mut iter_index = 0usize;

                for &filter in analyzed_se.left_front.filter_bounds.iter() {
                    let filter_start_x = filter.x + x as i32;
                    let filter_start_y = filter.y + y as i32;
                    if filter_start_y < -3 {
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
                    let py1 = (py + 1).min(max_height).max(0) as usize * stride;
                    let py2 = (py + 2).min(max_height).max(0) as usize * stride;
                    let py3 = (py + 3).min(max_height).max(0) as usize * stride;
                    let py4 = (py + 4).min(max_height).max(0) as usize * stride;
                    let py5 = (py + 5).min(max_height).max(0) as usize * stride;

                    while current_x < filter_size {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize;
                        let base_offset0 = py0 + px;
                        let base_offset1 = py1 + px;
                        let base_offset2 = py2 + px;
                        let base_offset3 = py3 + px;
                        let base_offset4 = py4 + px;
                        let base_offset5 = py5 + px;
                        let new_value0 = *src.get_unchecked(base_offset0);
                        *items0.get_unchecked_mut(iter_index) = new_value0;
                        let new_value1 = *src.get_unchecked(base_offset1);
                        *items1.get_unchecked_mut(iter_index) = new_value1;
                        let new_value2 = *src.get_unchecked(base_offset2);
                        *items2.get_unchecked_mut(iter_index) = new_value2;
                        let new_value3 = *src.get_unchecked(base_offset3);
                        *items3.get_unchecked_mut(iter_index) = new_value3;
                        let new_value4 = *src.get_unchecked(base_offset4);
                        *items4.get_unchecked_mut(iter_index) = new_value4;
                        let new_value5 = *src.get_unchecked(base_offset5);
                        *items5.get_unchecked_mut(iter_index) = new_value5;
                        iter_index += 1;
                        current_x += 1;
                    }
                }

                dst.write(
                    y * stride + x,
                    minmax_resolver(items0.get_unchecked(..iter_index)),
                );
                dst.write(
                    (y + 1) * stride + x,
                    minmax_resolver(items1.get_unchecked(..iter_index)),
                );
                dst.write(
                    (y + 2) * stride + x,
                    minmax_resolver(items2.get_unchecked(..iter_index)),
                );
                dst.write(
                    (y + 3) * stride + x,
                    minmax_resolver(items3.get_unchecked(..iter_index)),
                );
                dst.write(
                    (y + 4) * stride + x,
                    minmax_resolver(items4.get_unchecked(..iter_index)),
                );
                dst.write(
                    (y + 5) * stride + x,
                    minmax_resolver(items5.get_unchecked(..iter_index)),
                );
            }
        }
    }
}
