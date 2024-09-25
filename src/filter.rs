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
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::ops::avx::MorphOpFilterAvx2DRow;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use crate::ops::neon::{
    MorphOpFilterNeon2DRow, MorphOpFilterNeon2DRowF32, MorphOpFilterNeon2DRowU16,
};
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::ops::sse::{MorphOpFilterSse2DRow, MorphOpFilterSse2DRowF32, MorphOpFilterSse2DRowU16};
use crate::ops::MorphFilterFlat2DRow;
use crate::unsafe_slice::UnsafeSlice;
use crate::ImageSize;

pub struct MorthFilterFlat2DRow<T>
where
    T: 'static,
{
    pub(crate) handler: Box<dyn MorthOpFilterFlat2DRow<T> + Sync + Send>,
}

impl<T> MorthOpFilterFlat2DRow<T> for MorthFilterFlat2DRow<T> {
    unsafe fn dispatch_row(
        &self,
        arena: &Arena<T>,
        dst: &UnsafeSlice<T>,
        image_size: ImageSize,
        analyzed_se: AnalyzedSe,
        y: usize,
    ) {
        self.handler
            .dispatch_row(arena, dst, image_size, analyzed_se, y)
    }
}

pub trait Row2DFilter<T> {
    fn get_filter(op: MorphOp) -> MorthFilterFlat2DRow<T>;
}

impl Row2DFilter<u8> for u8 {
    fn get_filter(op: MorphOp) -> MorthFilterFlat2DRow<u8> {
        MorthFilterFlat2DRow {
            handler: match op {
                MorphOp::Dilate => {
                    let mut _result: Box<dyn MorthOpFilterFlat2DRow<u8> + Sync + Send> =
                        Box::new(MorphFilterFlat2DRow::<{ MorphOp::Dilate as u8 }>::default());
                    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
                    {
                        _result = Box::new(
                            MorphOpFilterNeon2DRow::<{ MorphOp::Dilate as u8 }>::default(),
                        );
                    }
                    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                    {
                        if std::arch::is_x86_feature_detected!("sse4.1") {
                            _result = Box::new(
                                MorphOpFilterSse2DRow::<{ MorphOp::Dilate as u8 }>::default(),
                            );
                        }
                        if std::arch::is_x86_feature_detected!("avx2") {
                            _result = Box::new(
                                MorphOpFilterAvx2DRow::<{ MorphOp::Dilate as u8 }>::default(),
                            );
                        }
                    }
                    _result
                }
                MorphOp::Erode => {
                    let mut _result: Box<dyn MorthOpFilterFlat2DRow<u8> + Sync + Send> =
                        Box::new(MorphFilterFlat2DRow::<{ MorphOp::Erode as u8 }>::default());
                    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
                    {
                        _result =
                            Box::new(MorphOpFilterNeon2DRow::<{ MorphOp::Erode as u8 }>::default());
                    }
                    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                    {
                        if std::arch::is_x86_feature_detected!("sse4.1") {
                            _result = Box::new(
                                MorphOpFilterSse2DRow::<{ MorphOp::Erode as u8 }>::default(),
                            );
                        }
                        if std::arch::is_x86_feature_detected!("avx2") {
                            _result = Box::new(
                                MorphOpFilterAvx2DRow::<{ MorphOp::Erode as u8 }>::default(),
                            );
                        }
                    }
                    _result
                }
            },
        }
    }
}

impl Row2DFilter<f32> for f32 {
    fn get_filter(op: MorphOp) -> MorthFilterFlat2DRow<f32> {
        MorthFilterFlat2DRow {
            handler: match op {
                MorphOp::Dilate => {
                    let mut _result: Box<dyn MorthOpFilterFlat2DRow<f32> + Sync + Send> =
                        Box::new(MorphFilterFlat2DRow::<{ MorphOp::Dilate as u8 }>::default());
                    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                    {
                        if std::arch::is_x86_feature_detected!("sse4.1") {
                            _result = Box::new(
                                MorphOpFilterSse2DRowF32::<{ MorphOp::Dilate as u8 }>::default(),
                            );
                        }
                    }
                    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
                    {
                        _result = Box::new(
                            MorphOpFilterNeon2DRowF32::<{ MorphOp::Dilate as u8 }>::default(),
                        );
                    }
                    _result
                }
                MorphOp::Erode => {
                    let mut _result: Box<dyn MorthOpFilterFlat2DRow<f32> + Sync + Send> =
                        Box::new(MorphFilterFlat2DRow::<{ MorphOp::Erode as u8 }>::default());
                    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                    {
                        if std::arch::is_x86_feature_detected!("sse4.1") {
                            _result = Box::new(
                                MorphOpFilterSse2DRowF32::<{ MorphOp::Erode as u8 }>::default(),
                            );
                        }
                    }
                    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
                    {
                        _result = Box::new(
                            MorphOpFilterNeon2DRowF32::<{ MorphOp::Erode as u8 }>::default(),
                        );
                    }
                    _result
                }
            },
        }
    }
}

impl Row2DFilter<u16> for u16 {
    fn get_filter(op: MorphOp) -> MorthFilterFlat2DRow<u16> {
        MorthFilterFlat2DRow {
            handler: match op {
                MorphOp::Dilate => {
                    let mut _result: Box<dyn MorthOpFilterFlat2DRow<u16> + Sync + Send> =
                        Box::new(MorphFilterFlat2DRow::<{ MorphOp::Dilate as u8 }>::default());
                    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                    {
                        if std::arch::is_x86_feature_detected!("sse4.1") {
                            _result = Box::new(
                                MorphOpFilterSse2DRowU16::<{ MorphOp::Dilate as u8 }>::default(),
                            );
                        }
                    }
                    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
                    {
                        _result = Box::new(
                            MorphOpFilterNeon2DRowU16::<{ MorphOp::Dilate as u8 }>::default(),
                        );
                    }
                    _result
                }
                MorphOp::Erode => {
                    let mut _result: Box<dyn MorthOpFilterFlat2DRow<u16> + Sync + Send> =
                        Box::new(MorphFilterFlat2DRow::<{ MorphOp::Erode as u8 }>::default());
                    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                    {
                        if std::arch::is_x86_feature_detected!("sse4.1") {
                            _result = Box::new(
                                MorphOpFilterSse2DRowU16::<{ MorphOp::Erode as u8 }>::default(),
                            );
                        }
                    }
                    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
                    {
                        _result = Box::new(
                            MorphOpFilterNeon2DRowU16::<{ MorphOp::Erode as u8 }>::default(),
                        );
                    }
                    _result
                }
            },
        }
    }
}
