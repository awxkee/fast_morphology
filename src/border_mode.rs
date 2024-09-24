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
use num_traits::{AsPrimitive, Euclid, FromPrimitive, Signed};
use std::ops::Index;

#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
/// Declares an edge handling mode
pub enum BorderMode {
    /// If kernel goes out of bounds it will be clipped to an edge and edge pixel replicated across filter
    #[default]
    Clamp,
    /// If filter goes out of bounds image will be replicated with rule `cdefgh|abcdefgh|abcdefg`
    Wrap,
    /// If filter goes out of bounds image will be replicated with rule `fedcba|abcdefgh|hgfedcb`
    Reflect,
    /// If filter goes out of bounds image will be replicated with rule `gfedcb|abcdefgh|gfedcba`
    Reflect101,
    /// If filter goes out of bounds image will be replaced with provided constant values
    Constant(OutOfBoundsConstant),
}

#[inline]
pub(crate) fn reflect_index<
    T: Copy
        + 'static
        + PartialOrd
        + PartialEq
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + Euclid
        + FromPrimitive
        + Signed
        + AsPrimitive<usize>,
>(
    i: T,
    n: T,
) -> usize
where
    i64: AsPrimitive<T>,
{
    let i = (i - n).rem_euclid(&(2i64.as_() * n));
    let i = (i - n).abs();
    i.as_()
}

#[inline(always)]
pub(crate) fn reflect_index_101<
    T: Copy
        + 'static
        + PartialOrd
        + PartialEq
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + Euclid
        + FromPrimitive
        + Signed
        + AsPrimitive<usize>
        + Ord,
>(
    i: T,
    n: T,
) -> usize
where
    i64: AsPrimitive<T>,
{
    if i < T::from_i32(0i32).unwrap() {
        let i = (i - n).rem_euclid(&(2i64.as_() * n));
        let i = (i - n).abs();
        return (i + T::from_i32(1).unwrap()).min(n).as_();
    }
    if i > n {
        let i = (i - n).rem_euclid(&(2i64.as_() * n));
        let i = (i - n).abs();
        return (i - T::from_i32(1i32).unwrap())
            .max(T::from_i32(0i32).unwrap())
            .as_();
    }
    i.as_()
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct OutOfBoundsConstant {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl OutOfBoundsConstant {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> OutOfBoundsConstant {
        OutOfBoundsConstant { r, g, b, a }
    }

    pub fn replicate(v: u8) -> OutOfBoundsConstant {
        OutOfBoundsConstant::new(v, v, v, v)
    }
}

impl Index<usize> for OutOfBoundsConstant {
    type Output = u8;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => {
                panic!("Index if {} is not exists in [OutOfBoundsConstant]", index)
            }
        }
    }
}
