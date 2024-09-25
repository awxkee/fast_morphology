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
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[derive(Clone)]
pub struct MorphOpFilterSse2DRowF32<const OP_TYPE: u8> {}

impl<const OP_TYPE: u8> Default for MorphOpFilterSse2DRowF32<OP_TYPE> {
    fn default() -> Self {
        MorphOpFilterSse2DRowF32 {}
    }
}

impl<T, const OP_TYPE: u8> MorthOpFilterFlat2DRow<T> for MorphOpFilterSse2DRowF32<OP_TYPE>
where
    T: Copy + 'static + MorphNativeOp<T>,
{
    #[target_feature(enable = "sse4.1")]
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
        let stride = width;

        let decision = match op_type {
            MorphOp::Dilate => _mm_max_ps,
            MorphOp::Erode => _mm_min_ps,
        };

        let src: &Vec<f32> = std::mem::transmute(&arena.arena);
        let dst: &UnsafeSlice<f32> = std::mem::transmute(dst);

        let dx = arena.pad_w as i32;
        let dy = arena.pad_h as i32;

        let arena_stride = arena.width;

        let offsets = analyzed_se
            .left_front
            .element_offsets
            .iter()
            .map(|&x| {
                src.get_unchecked(
                    ((x.y + dy + y as i32) as usize * arena_stride + (x.x + dx) as usize)..,
                )
            })
            .collect::<Vec<_>>();

        let length = analyzed_se.left_front.element_offsets.iter().len();

        let mut _cx = 0usize;

        while _cx + 16 < width {
            let ptr0 = (*offsets.get_unchecked(0).get_unchecked(_cx..)).as_ptr();
            let mut row0 = _mm_loadu_ps(ptr0);
            let mut row1 = _mm_loadu_ps(ptr0.add(4));
            let mut row2 = _mm_loadu_ps(ptr0.add(8));
            let mut row3 = _mm_loadu_ps(ptr0.add(12));

            for i in 1..length {
                let ptr_d = (*offsets.get_unchecked(i)).get_unchecked(_cx..).as_ptr();
                let new_row0 = _mm_loadu_ps(ptr_d);
                let new_row1 = _mm_loadu_ps(ptr_d.add(4));
                let new_row2 = _mm_loadu_ps(ptr_d.add(8));
                let new_row3 = _mm_loadu_ps(ptr_d.add(12));
                row0 = decision(row0, new_row0);
                row1 = decision(row1, new_row1);
                row2 = decision(row2, new_row2);
                row3 = decision(row3, new_row3);
            }

            let v_dst = dst.slice.as_ptr().add(y * stride + _cx) as *mut f32;

            _mm_storeu_ps(v_dst, row0);
            _mm_storeu_ps(v_dst.add(4), row1);
            _mm_storeu_ps(v_dst.add(8), row2);
            _mm_storeu_ps(v_dst.add(12), row3);

            _cx += 16;
        }

        while _cx + 8 < width {
            let ptr0 = (*offsets.get_unchecked(0).get_unchecked(_cx..)).as_ptr();
            let mut row0 = _mm_loadu_ps(ptr0);
            let mut row1 = _mm_loadu_ps(ptr0.add(4));

            for i in 1..length {
                let ptr_d = (*offsets.get_unchecked(i)).get_unchecked(_cx..).as_ptr();
                let new_row0 = _mm_loadu_ps(ptr_d);
                let new_row1 = _mm_loadu_ps(ptr_d.add(4));
                row0 = decision(row0, new_row0);
                row1 = decision(row1, new_row1);
            }

            let v_dst = dst.slice.as_ptr().add(y * stride + _cx) as *mut f32;

            _mm_storeu_ps(v_dst, row0);
            _mm_storeu_ps(v_dst.add(8), row1);

            _cx += 8;
        }

        while _cx + 4 < width {
            let ptr0 = (*offsets.get_unchecked(0).get_unchecked(_cx..)).as_ptr();
            let mut row0 = _mm_loadu_ps(ptr0);

            for i in 1..length {
                let ptr_d = (*offsets.get_unchecked(i)).get_unchecked(_cx..).as_ptr();
                let new_row0 = _mm_loadu_ps(ptr_d);
                row0 = decision(row0, new_row0);
            }

            let v_dst = dst.slice.as_ptr().add(y * stride + _cx) as *mut f32;
            _mm_storeu_ps(v_dst, row0);

            _cx += 4;
        }

        while _cx + 2 < width {
            let ptr0 = (*offsets.get_unchecked(0).get_unchecked(_cx..)).as_ptr();
            let mut row0 = _mm_castsi128_ps(_mm_loadu_si64(ptr0 as *const u8));

            for i in 1..length {
                let ptr_d = (*offsets.get_unchecked(i)).get_unchecked(_cx..).as_ptr();
                let new_row0 = _mm_castsi128_ps(_mm_loadu_si64(ptr_d as *const u8));
                row0 = decision(row0, new_row0);
            }

            let v_dst = dst.slice.as_ptr().add(y * stride + _cx) as *mut u8;
            let v0 = _mm_castps_si128(row0);
            std::ptr::copy_nonoverlapping(&v0 as *const _ as *const u8, v_dst, 8);

            _cx += 2;
        }

        for x in _cx..width {
            let mut k0 = *(*offsets.get_unchecked(0)).get_unchecked(x);

            for i in 1..length {
                k0 = k0.op::<OP_TYPE>(*(*offsets.get_unchecked(i)).get_unchecked(x));
            }
            dst.write(y * stride + x, k0);
        }
    }
}
