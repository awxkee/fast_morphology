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
use crate::ops::neon::op::fast_morph_op_4d_neon;
use crate::ops::neon::utils::vld4h_u8;
use crate::ops::utils::write_rgba_to_slice;
use crate::se_scan::ScanPoint;
use crate::unsafe_slice::UnsafeSlice;
use crate::ImageSize;
use colorutils_rs::Rgba;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
#[cfg(target_arch = "arm")]
use std::arch::arm::*;

#[derive(Clone)]
pub struct MorphOpFilterRgbaNeon2DRow<const OP_TYPE: u8> {}

impl<const OP_TYPE: u8> Default for MorphOpFilterRgbaNeon2DRow<OP_TYPE> {
    fn default() -> Self {
        MorphOpFilterRgbaNeon2DRow {}
    }
}

impl<const OP_TYPE: u8> MorthOpFilterFlat2DRow for MorphOpFilterRgbaNeon2DRow<OP_TYPE> {
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

        let decision_16 = match op_type {
            MorphOp::Dilate => vmaxq_u8,
            MorphOp::Erode => vminq_u8,
        };

        let decision_8 = match op_type {
            MorphOp::Dilate => vmax_u8,
            MorphOp::Erode => vmin_u8,
        };

        let det_op = match op_type {
            MorphOp::Dilate => vmaxvq_u8,
            MorphOp::Erode => vminvq_u8,
        };

        let det_op_8 = match op_type {
            MorphOp::Dilate => vmaxv_u8,
            MorphOp::Erode => vminv_u8,
        };

        if let Some(arena) = arena {
            let mut items0 =
                vec![Rgba::dup(base_val); analyzed_se.left_front.element_offsets.len()];

            let arena_stride = arena.width * 4;

            let src = &arena.arena;

            let dx = arena.pad_w as i32;
            let dy = arena.pad_h as i32;

            let d_size = ScanPoint::new(dx, dy);
            let filter_bounds = analyzed_se
                .left_front
                .filter_bounds
                .iter()
                .map(|&x| x + d_size)
                .collect::<Vec<_>>();

            for x in 0..width {
                let mut iter_index = 0usize;

                for filter in filter_bounds.iter() {
                    let filter_start_x = filter.x + x as i32;
                    let filter_start_y = filter.y + y as i32;
                    let filter_size = filter.size as usize;

                    let py = filter_start_y as usize;
                    let py0 = py * arena_stride;

                    let mut current_x = 0usize;

                    while current_x + 16 < filter_size {
                        let px = (filter_start_x + current_x as i32) as usize * 4;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = vld4q_u8(current0.as_ptr());

                        *items0.get_unchecked_mut(iter_index) = Rgba::new(
                            det_op(new_value0.0),
                            det_op(new_value0.1),
                            det_op(new_value0.2),
                            det_op(new_value0.3),
                        );
                        iter_index += 1;

                        current_x += 16;
                    }

                    while current_x + 8 < filter_size {
                        let px = (filter_start_x + current_x as i32) as usize * 4;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = vld4_u8(current0.as_ptr());

                        *items0.get_unchecked_mut(iter_index) = Rgba::new(
                            det_op_8(new_value0.0),
                            det_op_8(new_value0.1),
                            det_op_8(new_value0.2),
                            det_op_8(new_value0.3),
                        );
                        iter_index += 1;

                        current_x += 8;
                    }

                    while current_x + 4 < filter_size {
                        let px = (filter_start_x + current_x as i32) as usize * 4;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = vld4h_u8(current0.as_ptr(), base_val);

                        *items0.get_unchecked_mut(iter_index) = Rgba::new(
                            det_op_8(new_value0.0),
                            det_op_8(new_value0.1),
                            det_op_8(new_value0.2),
                            det_op_8(new_value0.3),
                        );
                        iter_index += 1;

                        current_x += 4;
                    }

                    while current_x < filter_size {
                        let px = (filter_start_x + current_x as i32) as usize * 4;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);

                        *items0.get_unchecked_mut(iter_index) =
                            Rgba::new(current0[0], current0[1], current0[2], current0[3]);
                        iter_index += 1;

                        current_x += 1;
                    }
                }

                let px = x * 4;
                write_rgba_to_slice(
                    dst,
                    y * stride + px,
                    fast_morph_op_4d_neon::<OP_TYPE>(items0.get_unchecked(..iter_index)),
                );
            }
        } else {
            let max_width = width as i32 - 1;
            let max_height = height as i32 - 1;
            for x in 0..width {
                let filter_bounds = &analyzed_se.left_front.filter_bounds;

                let mut rgb0 = Rgba::dup(base_val);

                for filter in filter_bounds.iter() {
                    let filter_start_x = filter.x + x as i32;
                    let filter_start_y = filter.y + y as i32;
                    if (filter_start_y + 3) < 0 {
                        continue;
                    }
                    if (filter_start_x + filter.size as i32) < 0 {
                        continue;
                    }
                    let mut filter_size = filter.size as usize;
                    if filter_size as i32 + filter_start_x >= width as i32 {
                        filter_size = (width as i32 - filter_start_x) as usize;
                    }

                    let py = filter_start_y;
                    let py0 = py.min(max_height).max(0) as usize * stride;

                    let mut current_x = 0usize;

                    let mut values_r_0 = vdupq_n_u8(rgb0.r);
                    let mut values_g_0 = vdupq_n_u8(rgb0.g);
                    let mut values_b_0 = vdupq_n_u8(rgb0.b);
                    let mut values_a_0 = vdupq_n_u8(rgb0.a);

                    while current_x + 16 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px =
                            (filter_start_x + current_x as i32).min(max_width).max(0) as usize * 4;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = vld4q_u8(current0.as_ptr());

                        values_r_0 = decision_16(values_r_0, new_value0.0);
                        values_g_0 = decision_16(values_g_0, new_value0.1);
                        values_b_0 = decision_16(values_b_0, new_value0.2);
                        values_a_0 = decision_16(values_a_0, new_value0.3);

                        current_x += 16;
                    }

                    let mut values_r8_0 = vdup_n_u8(det_op(values_r_0));
                    let mut values_g8_0 = vdup_n_u8(det_op(values_g_0));
                    let mut values_b8_0 = vdup_n_u8(det_op(values_b_0));
                    let mut values_a8_0 = vdup_n_u8(det_op(values_a_0));

                    while current_x + 8 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px =
                            (filter_start_x + current_x as i32).min(max_width).max(0) as usize * 4;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = vld4_u8(current0.as_ptr());

                        values_r8_0 = decision_8(values_r8_0, new_value0.0);
                        values_g8_0 = decision_8(values_g8_0, new_value0.1);
                        values_b8_0 = decision_8(values_b8_0, new_value0.2);
                        values_a8_0 = decision_8(values_a8_0, new_value0.3);

                        current_x += 8;
                    }

                    let mut values_r8_0 = vdup_n_u8(det_op_8(values_r8_0));
                    let mut values_g8_0 = vdup_n_u8(det_op_8(values_g8_0));
                    let mut values_b8_0 = vdup_n_u8(det_op_8(values_b8_0));
                    let mut values_a8_0 = vdup_n_u8(det_op_8(values_a8_0));

                    while current_x + 4 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px =
                            (filter_start_x + current_x as i32).min(max_width).max(0) as usize * 4;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = vld4h_u8(current0.as_ptr(), base_val);

                        values_r8_0 = decision_8(values_r8_0, new_value0.0);

                        values_g8_0 = decision_8(values_g8_0, new_value0.1);

                        values_b8_0 = decision_8(values_b8_0, new_value0.2);

                        values_a8_0 = decision_8(values_a8_0, new_value0.3);

                        current_x += 4;
                    }

                    let arr0: [u8; 8] = [
                        det_op_8(values_r8_0),
                        det_op_8(values_g8_0),
                        det_op_8(values_b8_0),
                        det_op_8(values_a8_0),
                        base_val,
                        base_val,
                        base_val,
                        base_val,
                    ];

                    let mut values0 = vld1_u8(arr0.as_ptr());

                    while current_x < filter_size {
                        let px =
                            (filter_start_x + current_x as i32).min(max_width).max(0) as usize * 4;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let arr0: [u8; 8] = [
                            current0[0],
                            current0[1],
                            current0[2],
                            current0[3],
                            base_val,
                            base_val,
                            base_val,
                            base_val,
                        ];
                        let new_value0 = vld1_u8(arr0.as_ptr());

                        values0 = decision_8(values0, new_value0);

                        current_x += 1;
                    }

                    let values0 = vget_lane_u32::<0>(vreinterpret_u32_u8(values0)).to_le_bytes();
                    rgb0 = Rgba::new(values0[0], values0[1], values0[2], values0[3]);
                }

                let px = x * 4;
                write_rgba_to_slice(dst, y * stride + px, rgb0);
            }
        }
    }
}
