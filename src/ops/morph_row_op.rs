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
use crate::unsafe_slice::UnsafeSlice;
use crate::ImageSize;

#[derive(Clone)]
pub struct MorphFilterFlat2DRow<const OP_TYPE: u8> {}

impl<const OP_TYPE: u8> Default for MorphFilterFlat2DRow<OP_TYPE> {
    fn default() -> Self {
        MorphFilterFlat2DRow {}
    }
}

impl<T, const OP_TYPE: u8> MorthOpFilterFlat2DRow<T> for MorphFilterFlat2DRow<OP_TYPE>
where
    T: 'static + Copy + MorphNativeOp<T>,
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
        let stride = image_size.width;

        let src = &arena.arena;

        let dx = arena.pad_w as i32;
        let dy = arena.pad_h as i32;

        let arena_width = arena.width;

        let offsets = analyzed_se
            .left_front
            .element_offsets
            .iter()
            .map(|&x| {
                src.get_unchecked(
                    ((x.y + dy + y as i32) as usize * arena_width + (x.x + dx) as usize)..,
                )
            })
            .collect::<Vec<_>>();

        let length = analyzed_se.left_front.element_offsets.iter().len();

        let mut _cx = 0usize;

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
