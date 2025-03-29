/*
 * Copyright (c) Radzivon Bartoshyk 3/2025. All rights reserved.
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
use crate::se_scan::ScanPoint;
use num_traits::Bounded;

pub(crate) trait ScanMax {
    fn s_max(self, other: Self) -> Self;
}

pub(crate) trait ScanMin {
    fn s_min(self, other: Self) -> Self;
}

macro_rules! define_scan_max {
    ($basic_type: ident) => {
        impl ScanMax for $basic_type {
            #[inline(always)]
            fn s_max(self, other: Self) -> Self {
                self.max(other)
            }
        }
    };
}

macro_rules! define_scan_min {
    ($basic_type: ident) => {
        impl ScanMin for $basic_type {
            #[inline(always)]
            fn s_min(self, other: Self) -> Self {
                self.min(other)
            }
        }
    };
}

define_scan_max!(u8);
define_scan_max!(u16);
define_scan_max!(u32);
define_scan_max!(f32);

define_scan_min!(u8);
define_scan_min!(u16);
define_scan_min!(u32);
define_scan_min!(f32);

pub(crate) fn morph_1d_common<
    T: Copy + Bounded + ScanMax + ScanMin,
    const N: usize,
    const DILATE: bool,
>(
    src: &[T],
    dst: &mut [T],
    pad: usize,
    points: &[ScanPoint],
) {
    let mut offsets = vec![0usize; points.len()];
    for (offset, point) in offsets.iter_mut().zip(points.iter()) {
        *offset = ((point.x as isize + pad as isize) as usize) * N;
    }
    if DILATE {
        for (index, dst) in dst.iter_mut().enumerate() {
            let mut v_max = T::min_value();
            for &offset in offsets.iter() {
                let shifted = src[index + offset];
                v_max = v_max.s_max(shifted);
            }
            *dst = v_max;
        }
    } else {
        for (index, dst) in dst.iter_mut().enumerate() {
            let mut v_min = T::max_value();
            for &offset in offsets.iter() {
                let shifted = src[index + offset];
                v_min = v_min.s_min(shifted);
            }
            *dst = v_min;
        }
    }
}
