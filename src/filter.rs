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
use crate::ImageSize;
use crate::op_type::MorphOp;
use crate::ops::neon::MorphOpFilterNeon2D4Rows;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use crate::ops::neon::MorphOpFilterNeon2DRow;
use crate::ops::{MorphFilterFlat2D4Rows, MorphFilterFlat2DRow};
use crate::unsafe_slice::UnsafeSlice;

pub struct MorthFilterFlat2DRow {
    pub(crate) handler: Box<dyn MorthOpFilterFlat2DRow + Sync + Send>,
}

impl MorthOpFilterFlat2DRow for MorthFilterFlat2DRow {
    unsafe fn dispatch_row(
        &self,
        src: &[u8],
        dst: &UnsafeSlice<u8>,
        image_size: ImageSize,
        analyzed_se: AnalyzedSe,
        y: usize,
        arena: &Option<Arena>,
    ) {
        self.handler
            .dispatch_row(src, dst, image_size, analyzed_se, y, arena)
    }
}

impl MorthFilterFlat2DRow {
    pub fn new(op: MorphOp) -> MorthFilterFlat2DRow {
        MorthFilterFlat2DRow {
            handler: match op {
                MorphOp::Dilate => {
                    let mut _result: Box<dyn MorthOpFilterFlat2DRow + Sync + Send> =
                        Box::new(MorphFilterFlat2DRow::<{ MorphOp::Dilate as u8 }>::default());
                    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
                    {
                        _result = Box::new(
                            MorphOpFilterNeon2DRow::<{ MorphOp::Dilate as u8 }>::default(),
                        );
                    }
                    _result
                }
                MorphOp::Erode => {
                    let mut _result: Box<dyn MorthOpFilterFlat2DRow + Sync + Send> =
                        Box::new(MorphFilterFlat2DRow::<{ MorphOp::Erode as u8 }>::default());
                    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
                    {
                        _result =
                            Box::new(MorphOpFilterNeon2DRow::<{ MorphOp::Erode as u8 }>::default());
                    }
                    _result
                }
            },
        }
    }
}

pub struct MorthFilterFlat2D4Rows {
    pub(crate) handler: Box<dyn MorthOpFilterFlat2DRow + Sync + Send>,
}

impl MorthFilterFlat2D4Rows {
    pub fn new(op: MorphOp) -> MorthFilterFlat2D4Rows {
        MorthFilterFlat2D4Rows {
            handler: match op {
                MorphOp::Dilate => {
                    let mut _result: Box<dyn MorthOpFilterFlat2DRow + Sync + Send> =
                        Box::new(MorphFilterFlat2D4Rows::<{ MorphOp::Dilate as u8 }>::default());
                    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
                    {
                        _result = Box::new(
                            MorphOpFilterNeon2D4Rows::<{ MorphOp::Dilate as u8 }>::default(),
                        );
                    }
                    _result
                }
                MorphOp::Erode => {
                    let mut _result: Box<dyn MorthOpFilterFlat2DRow + Sync + Send> =
                        Box::new(MorphFilterFlat2D4Rows::<{ MorphOp::Erode as u8 }>::default());
                    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
                    {
                        _result = Box::new(
                            MorphOpFilterNeon2D4Rows::<{ MorphOp::Erode as u8 }>::default(),
                        );
                    }
                    _result
                }
            },
        }
    }
}

impl MorthOpFilterFlat2DRow for MorthFilterFlat2D4Rows {
    unsafe fn dispatch_row(
        &self,
        src: &[u8],
        dst: &UnsafeSlice<u8>,
        image_size: ImageSize,
        analyzed_se: AnalyzedSe,
        y: usize,
        arena: &Option<Arena>,
    ) {
        self.handler
            .dispatch_row(src, dst, image_size, analyzed_se, y, arena);
    }
}
