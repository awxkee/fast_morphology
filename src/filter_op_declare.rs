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
use crate::flat_se::AnalyzedSe;
use crate::unsafe_slice::UnsafeSlice;
use crate::ImageSize;

#[derive(Clone)]
pub struct Arena<T> {
    pub arena: Vec<T>,
    pub width: usize,
    #[allow(dead_code)]
    pub height: usize,
    pub pad_w: usize,
    pub pad_h: usize,
}

impl<T> Arena<T> {
    pub fn new(
        arena: Vec<T>,
        arena_width: usize,
        arena_height: usize,
        arena_pad_w: usize,
        arena_pad_h: usize,
    ) -> Arena<T> {
        Arena {
            arena,
            width: arena_width,
            height: arena_height,
            pad_w: arena_pad_w,
            pad_h: arena_pad_h,
        }
    }
}

pub trait MorthOpFilterFlat2DRow<T> {
    unsafe fn dispatch_row(
        &self,
        arena: &Arena<T>,
        dst: &UnsafeSlice<T>,
        image_size: ImageSize,
        analyzed_se: AnalyzedSe,
        y: usize,
    );
}
