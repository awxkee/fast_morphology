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
use crate::morph_base::MorphNativeOp;
use crate::op_type::MorphOp;
use crate::unsafe_slice::UnsafeSlice;
use crate::ImageSize;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
#[cfg(target_arch = "arm")]
use std::arch::arm::*;

#[derive(Clone)]
pub(crate) struct MorphOpFilterNeon2DRowF32<const OP_TYPE: u8> {}

impl<const OP_TYPE: u8> Default for MorphOpFilterNeon2DRowF32<OP_TYPE> {
    fn default() -> Self {
        MorphOpFilterNeon2DRowF32 {}
    }
}

#[inline(always)]
unsafe fn xvld1q_f32_x4(a: *const f32) -> float32x4x4_t {
    let v0 = vld1q_f32(a);
    let v1 = vld1q_f32(a.add(4));
    let v2 = vld1q_f32(a.add(8));
    let v3 = vld1q_f32(a.add(12));
    float32x4x4_t(v0, v1, v2, v3)
}

#[inline(always)]
unsafe fn xvld1q_f32_x2(a: *const f32) -> float32x4x2_t {
    let v0 = vld1q_f32(a);
    let v1 = vld1q_f32(a.add(4));
    float32x4x2_t(v0, v1)
}

#[inline(always)]
unsafe fn xvst1q_f32_x2(a: *mut f32, b: float32x4x2_t) {
    vst1q_f32(a, b.0);
    vst1q_f32(a.add(4), b.1);
}

#[inline(always)]
unsafe fn xvst1q_f32_x4(a: *mut f32, b: float32x4x4_t) {
    vst1q_f32(a, b.0);
    vst1q_f32(a.add(4), b.1);
    vst1q_f32(a.add(8), b.2);
    vst1q_f32(a.add(12), b.3);
}

impl<T, const OP_TYPE: u8> MorthOpFilterFlat2DRow<T> for MorphOpFilterNeon2DRowF32<OP_TYPE>
where
    T: Copy + 'static,
{
    unsafe fn dispatch_row(
        &self,
        arena: &Arena<T>,
        dst: &UnsafeSlice<T>,
        image_size: ImageSize,
        analyzed_se: AnalyzedSe,
        y: usize,
    ) {
        let width = image_size.width;

        let op_type: MorphOp = OP_TYPE.into();
        let stride = width * arena.components;

        let decision = match op_type {
            MorphOp::Dilate => vmaxq_f32,
            MorphOp::Erode => vminq_f32,
        };

        let decision_half = match op_type {
            MorphOp::Dilate => vmax_f32,
            MorphOp::Erode => vmin_f32,
        };

        let src: &Vec<f32> = std::mem::transmute(&arena.arena);
        let dst: &UnsafeSlice<f32> = std::mem::transmute(dst);

        let dx = arena.pad_w as i32;
        let dy = arena.pad_h as i32;

        let arena_stride = arena.width * arena.components;

        let offsets = analyzed_se
            .left_front
            .element_offsets
            .iter()
            .map(|&x| {
                src.get_unchecked(
                    ((x.y + dy + y as i32) as usize * arena_stride
                        + (x.x + dx) as usize * arena.components)..,
                )
            })
            .collect::<Vec<_>>();

        let total_width = width * arena.components;
        let length = analyzed_se.left_front.element_offsets.len();

        let mut cx = 0usize;

        while cx + 16 < total_width {
            let mut rows = xvld1q_f32_x4((*offsets.get_unchecked(0).get_unchecked(cx..)).as_ptr());

            for i in 1..length {
                let new_rows =
                    xvld1q_f32_x4((*offsets.get_unchecked(i)).get_unchecked(cx..).as_ptr());
                rows.0 = decision(rows.0, new_rows.0);
                rows.1 = decision(rows.1, new_rows.1);
                rows.2 = decision(rows.2, new_rows.2);
                rows.3 = decision(rows.3, new_rows.3);
            }

            xvst1q_f32_x4(dst.slice.as_ptr().add(y * stride + cx) as *mut f32, rows);

            cx += 16;
        }

        while cx + 8 < total_width {
            let mut rows = xvld1q_f32_x2((*offsets.get_unchecked(0).get_unchecked(cx..)).as_ptr());

            for i in 1..length {
                let new_rows =
                    xvld1q_f32_x2((*offsets.get_unchecked(i)).get_unchecked(cx..).as_ptr());
                rows.0 = decision(rows.0, new_rows.0);
                rows.1 = decision(rows.1, new_rows.1);
            }

            xvst1q_f32_x2(dst.slice.as_ptr().add(y * stride + cx) as *mut f32, rows);

            cx += 8;
        }

        while cx + 4 < total_width {
            let mut rows = vld1q_f32((*offsets.get_unchecked(0).get_unchecked(cx..)).as_ptr());

            for i in 1..length {
                let new_row = vld1q_f32((*offsets.get_unchecked(i)).get_unchecked(cx..).as_ptr());
                rows = decision(rows, new_row);
            }

            vst1q_f32(dst.slice.as_ptr().add(y * stride + cx) as *mut f32, rows);

            cx += 4;
        }

        while cx + 2 < total_width {
            let mut rows = vld1_f32((*offsets.get_unchecked(0).get_unchecked(cx..)).as_ptr());

            for i in 1..length {
                let new_row = vld1_f32((*offsets.get_unchecked(i)).get_unchecked(cx..).as_ptr());
                rows = decision_half(rows, new_row);
            }

            vst1_f32(dst.slice.as_ptr().add(y * stride + cx) as *mut f32, rows);

            cx += 2;
        }

        for x in cx..total_width {
            let mut k0 = *(*offsets.get_unchecked(0)).get_unchecked(x);

            for i in 1..length {
                k0 = k0.op::<OP_TYPE>(*(*offsets.get_unchecked(i)).get_unchecked(x));
            }
            dst.write(y * stride + x, k0);
        }
    }
}
