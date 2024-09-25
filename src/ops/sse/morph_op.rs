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
pub struct MorphOpFilterSse2DRow<const OP_TYPE: u8> {}

impl<const OP_TYPE: u8> Default for MorphOpFilterSse2DRow<OP_TYPE> {
    fn default() -> Self {
        MorphOpFilterSse2DRow {}
    }
}

impl<T, const OP_TYPE: u8> MorthOpFilterFlat2DRow<T> for MorphOpFilterSse2DRow<OP_TYPE>
where
    T: Copy + 'static + MorphNativeOp<T>,
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
        let stride = width;

        let decision = match op_type {
            MorphOp::Dilate => _mm_max_epu8,
            MorphOp::Erode => _mm_min_epu8,
        };

        let src: &Vec<u8> = std::mem::transmute(&arena.arena);
        let dst: &UnsafeSlice<u8> = std::mem::transmute(dst);

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

        while _cx + 64 < width {
            let ptr0 = (*offsets.get_unchecked(0).get_unchecked(_cx..)).as_ptr();
            let mut row0 = _mm_loadu_si128(ptr0 as *const __m128i);
            let mut row1 = _mm_loadu_si128(ptr0.add(16) as *const __m128i);
            let mut row2 = _mm_loadu_si128(ptr0.add(32) as *const __m128i);
            let mut row3 = _mm_loadu_si128(ptr0.add(48) as *const __m128i);

            for i in 1..length {
                let ptr_d = (*offsets.get_unchecked(i)).get_unchecked(_cx..).as_ptr();
                let new_row0 = _mm_loadu_si128(ptr_d as *const __m128i);
                let new_row1 = _mm_loadu_si128(ptr_d.add(16) as *const __m128i);
                let new_row2 = _mm_loadu_si128(ptr_d.add(32) as *const __m128i);
                let new_row3 = _mm_loadu_si128(ptr_d.add(48) as *const __m128i);
                row0 = decision(row0, new_row0);
                row1 = decision(row1, new_row1);
                row2 = decision(row2, new_row2);
                row3 = decision(row3, new_row3);
            }

            let v_dst = dst.slice.as_ptr().add(y * stride + _cx) as *mut u8;

            _mm_storeu_si128(v_dst as *mut __m128i, row0);
            _mm_storeu_si128(v_dst.add(16) as *mut __m128i, row1);
            _mm_storeu_si128(v_dst.add(32) as *mut __m128i, row2);
            _mm_storeu_si128(v_dst.add(48) as *mut __m128i, row3);

            _cx += 64;
        }

        while _cx + 32 < width {
            let ptr0 = (*offsets.get_unchecked(0).get_unchecked(_cx..)).as_ptr();
            let mut row0 = _mm_loadu_si128(ptr0 as *const __m128i);
            let mut row1 = _mm_loadu_si128(ptr0.add(16) as *const __m128i);

            for i in 1..length {
                let ptr_d = (*offsets.get_unchecked(i)).get_unchecked(_cx..).as_ptr();
                let new_row0 = _mm_loadu_si128(ptr_d as *const __m128i);
                let new_row1 = _mm_loadu_si128(ptr_d.add(16) as *const __m128i);
                row0 = decision(row0, new_row0);
                row1 = decision(row1, new_row1);
            }

            let v_dst = dst.slice.as_ptr().add(y * stride + _cx) as *mut u8;

            _mm_storeu_si128(v_dst as *mut __m128i, row0);
            _mm_storeu_si128(v_dst.add(16) as *mut __m128i, row1);

            _cx += 32;
        }

        while _cx + 16 < width {
            let ptr0 = (*offsets.get_unchecked(0).get_unchecked(_cx..)).as_ptr();
            let mut row0 = _mm_loadu_si128(ptr0 as *const __m128i);

            for i in 1..length {
                let ptr_d = (*offsets.get_unchecked(i)).get_unchecked(_cx..).as_ptr();
                let new_row0 = _mm_loadu_si128(ptr_d as *const __m128i);
                row0 = decision(row0, new_row0);
            }

            let v_dst = dst.slice.as_ptr().add(y * stride + _cx) as *mut u8;
            _mm_storeu_si128(v_dst as *mut __m128i, row0);

            _cx += 16;
        }

        while _cx + 8 < width {
            let ptr0 = (*offsets.get_unchecked(0).get_unchecked(_cx..)).as_ptr();
            let mut row0 = _mm_loadu_si64(ptr0);

            for i in 1..length {
                let ptr_d = (*offsets.get_unchecked(i)).get_unchecked(_cx..).as_ptr();
                let new_row0 = _mm_loadu_si64(ptr_d);
                row0 = decision(row0, new_row0);
            }

            let v_dst = dst.slice.as_ptr().add(y * stride + _cx) as *mut u8;
            std::ptr::copy_nonoverlapping(&row0 as *const _ as *const u8, v_dst, 8);

            _cx += 8;
        }

        for x in (_cx..width.saturating_sub(4)).step_by(4) {
            let mut k0 = *(*offsets.get_unchecked(0)).get_unchecked(x);
            let mut k1 = *(*offsets.get_unchecked(0)).get_unchecked(x + 1);
            let mut k2 = *(*offsets.get_unchecked(0)).get_unchecked(x + 2);
            let mut k3 = *(*offsets.get_unchecked(0)).get_unchecked(x + 3);

            for i in 1..length {
                k0 = k0.op::<OP_TYPE>(*(*offsets.get_unchecked(i)).get_unchecked(x));
                k1 = k1.op::<OP_TYPE>(*(*offsets.get_unchecked(i)).get_unchecked(x + 1));
                k2 = k2.op::<OP_TYPE>(*(*offsets.get_unchecked(i)).get_unchecked(x + 2));
                k3 = k3.op::<OP_TYPE>(*(*offsets.get_unchecked(i)).get_unchecked(x + 3));
            }

            let dst_offset = y * stride + x;

            dst.write(dst_offset, k0);
            dst.write(dst_offset + 1, k1);
            dst.write(dst_offset + 2, k2);
            dst.write(dst_offset + 3, k3);
            _cx = x;
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
