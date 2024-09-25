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
use crate::flat_se::{AnalyzedSe, FlatSe};
use crate::structuring_element::KernelShape;
use std::ops::Add;

pub(crate) unsafe fn scan_se(
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
) -> AnalyzedSe {
    let mut left_front = vec![];

    let kernel_width = structuring_element_size.width;
    let kernel_height = structuring_element_size.height;

    let half_kernel_width = kernel_width as i32 / 2;
    let half_kernel_height = kernel_height as i32 / 2;

    for y in 0..kernel_height {
        for x in 0..kernel_width {
            let item = *structuring_element.get_unchecked(y * kernel_height + x);
            if item != 0 {
                left_front.push(ScanPoint::new(
                    y as i32 - half_kernel_height,
                    x as i32 - half_kernel_width,
                ));
            }
        }
    }

    let iv_left: Vec<ScanPoint> = left_front.to_vec();

    AnalyzedSe::new(structuring_element.to_vec(), FlatSe::new(iv_left))
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ScanPoint {
    pub x: i32,
    pub y: i32,
}

impl Add<ScanPoint> for ScanPoint {
    type Output = ScanPoint;

    fn add(self, rhs: ScanPoint) -> Self::Output {
        ScanPoint::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct FilterBounds {
    pub x: i32,
    pub y: i32,
    pub size: u16,
}

impl FilterBounds {
    pub fn new(x: i32, y: i32, size: u16) -> FilterBounds {
        FilterBounds { x, y, size }
    }
}

impl Add<ScanPoint> for FilterBounds {
    type Output = FilterBounds;

    fn add(self, rhs: ScanPoint) -> Self::Output {
        FilterBounds::new(self.x + rhs.x, self.y + rhs.y, self.size)
    }
}

impl ScanPoint {
    pub fn new(x: i32, y: i32) -> ScanPoint {
        ScanPoint { x, y }
    }
}

impl Default for ScanPoint {
    fn default() -> Self {
        ScanPoint::new(0, 0)
    }
}
