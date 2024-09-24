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
use crate::se_scan::{FilterBounds, ScanPoint};

#[derive(Debug, Clone)]
pub(crate) struct FlatSe {
    /// Significant points { x, y }
    pub(crate) element_offsets: Vec<ScanPoint>,
    /// Precomputed filter bounds { x, y, horizontal_length }
    pub(crate) filter_bounds: Vec<FilterBounds>,
}

impl FlatSe {
    pub fn new(vec: Vec<ScanPoint>, filter_bounds_process: Vec<FilterBounds>) -> FlatSe {
        FlatSe {
            element_offsets: vec,
            filter_bounds: filter_bounds_process,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AnalyzedSe {
    #[allow(dead_code)]
    pub(crate) original_se: Vec<u8>,
    pub(crate) left_front: FlatSe,
    pub(crate) is_empty: bool,
}

impl AnalyzedSe {
    pub fn new(
        original_se: Vec<u8>,
        left_front: FlatSe,
    ) -> AnalyzedSe {
        let is_empty =
            left_front.element_offsets.is_empty() && left_front.element_offsets.is_empty();
        AnalyzedSe {
            original_se,
            left_front,
            is_empty,
        }
    }
}
