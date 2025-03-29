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
use crate::difference::MorphGradient;
use crate::filter_1d_padding::make_arena_1d;
use crate::morph_error::MorphError;
use crate::op_type::MorphOp;
use crate::ops::{morph_1d_common, ScanMax, ScanMin};
use crate::se_scan::ScanPoint;
use crate::{BorderMode, MorphExOp, MorphScalar};
use num_traits::{AsPrimitive, Bounded};

fn scan_kernel(kernel: &[u8]) -> Vec<ScanPoint> {
    assert_ne!(kernel.len() % 2, 0);
    let kernel_width = kernel.len();
    let half_width = kernel_width / 2;
    let mut scan_points = vec![];
    for (x, &item) in kernel.iter().enumerate() {
        if item != 0 {
            scan_points.push(ScanPoint::new(x as i32 - half_width as i32, 0));
        }
    }
    scan_points
}

trait Filter1DDelegate<T: Copy + Bounded + ScanMax + ScanMin> {
    fn morph_1d_common<const N: usize, const DILATE: bool>(
        src: &[T],
        dst: &mut [T],
        pad: usize,
        points: &[ScanPoint],
    );
}

impl Filter1DDelegate<u8> for u8 {
    fn morph_1d_common<const N: usize, const DILATE: bool>(
        src: &[u8],
        dst: &mut [u8],
        pad: usize,
        points: &[ScanPoint],
    ) {
        morph_1d_common::<u8, N, DILATE>(src, dst, pad, points);
    }
}

impl Filter1DDelegate<u16> for u16 {
    fn morph_1d_common<const N: usize, const DILATE: bool>(
        src: &[u16],
        dst: &mut [u16],
        pad: usize,
        points: &[ScanPoint],
    ) {
        morph_1d_common::<u16, N, DILATE>(src, dst, pad, points);
    }
}

impl Filter1DDelegate<f32> for f32 {
    fn morph_1d_common<const N: usize, const DILATE: bool>(
        src: &[f32],
        dst: &mut [f32],
        pad: usize,
        points: &[ScanPoint],
    ) {
        #[cfg(target_arch = "aarch64")]
        {
            use crate::ops::neon::morph_1d_neon_f32;
            morph_1d_neon_f32::<N, DILATE>(src, dst, pad, points);
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            morph_1d_common::<f32, N, DILATE>(src, dst, pad, points);
        }
    }
}

fn filter_1d_impl_base<
    T: Copy + 'static + ScanMax + ScanMin + Bounded + Filter1DDelegate<T> + Default,
    const N: usize,
>(
    src: &[T],
    dst: &mut [T],
    kernel: &[u8],
    op: MorphOp,
    border_mode: BorderMode,
    morph_scalar: MorphScalar,
) -> Result<(), MorphError>
where
    f64: AsPrimitive<T>,
{
    if kernel.len() % 2 == 0 {
        return Err(MorphError::KernelSizeMustBeOdd);
    }
    if src.len() != dst.len() {
        return Err(MorphError::SourceAndDestinationMustMatch);
    }
    if src.len() % N != 0 {
        return Err(MorphError::SamplesMustBeMultipleOfChannels);
    }
    let scanned_kernel = scan_kernel(kernel);
    if scanned_kernel.is_empty() {
        for (dst, src) in dst.iter_mut().zip(src.iter()) {
            *dst = *src;
        }
        return Ok(());
    }

    let pad = kernel.len() / 2;

    let arena_src = make_arena_1d::<T, N>(src, kernel.len(), border_mode, morph_scalar);

    match op {
        MorphOp::Dilate => T::morph_1d_common::<N, true>(&arena_src, dst, pad, &scanned_kernel),
        MorphOp::Erode => T::morph_1d_common::<N, false>(&arena_src, dst, pad, &scanned_kernel),
    }

    Ok(())
}

fn filter_1d_impl<
    T: Copy + 'static + ScanMax + ScanMin + Bounded + Filter1DDelegate<T> + Default + MorphGradient<T>,
    const N: usize,
>(
    src: &[T],
    dst: &mut [T],
    kernel: &[u8],
    op: MorphExOp,
    border_mode: BorderMode,
    morph_scalar: MorphScalar,
) -> Result<(), MorphError>
where
    f64: AsPrimitive<T>,
{
    match op {
        MorphExOp::Dilate => filter_1d_impl_base::<T, N>(
            src,
            dst,
            kernel,
            MorphOp::Dilate,
            border_mode,
            morph_scalar,
        ),
        MorphExOp::Erode => {
            filter_1d_impl_base::<T, N>(src, dst, kernel, MorphOp::Erode, border_mode, morph_scalar)
        }
        MorphExOp::Opening => {
            let mut transient = vec![T::default(); dst.len()];
            filter_1d_impl_base::<T, N>(
                src,
                &mut transient,
                kernel,
                MorphOp::Erode,
                border_mode,
                morph_scalar,
            )?;
            filter_1d_impl_base::<T, N>(
                &transient,
                dst,
                kernel,
                MorphOp::Dilate,
                border_mode,
                morph_scalar,
            )
        }
        MorphExOp::Closing => {
            let mut transient = vec![T::default(); dst.len()];
            filter_1d_impl_base::<T, N>(
                src,
                &mut transient,
                kernel,
                MorphOp::Dilate,
                border_mode,
                morph_scalar,
            )?;
            filter_1d_impl_base::<T, N>(
                &transient,
                dst,
                kernel,
                MorphOp::Erode,
                border_mode,
                morph_scalar,
            )
        }
        MorphExOp::Gradient => {
            let mut dilation = vec![T::default(); dst.len()];
            filter_1d_impl_base::<T, N>(
                src,
                &mut dilation,
                kernel,
                MorphOp::Dilate,
                border_mode,
                morph_scalar,
            )?;
            let mut erosion = vec![T::default(); dst.len()];
            filter_1d_impl_base::<T, N>(
                src,
                &mut erosion,
                kernel,
                MorphOp::Erode,
                border_mode,
                morph_scalar,
            )?;
            T::morph_gradient(&dilation, &erosion, dst);
            Ok(())
        }
        MorphExOp::TopHat => {
            let mut opened = vec![T::default(); dst.len()];
            filter_1d_impl::<T, N>(
                src,
                &mut opened,
                kernel,
                MorphExOp::Opening,
                border_mode,
                morph_scalar,
            )?;
            T::morph_gradient(src, &opened, dst);
            Ok(())
        }
        MorphExOp::BlackHat => {
            let mut closed = vec![T::default(); dst.len()];
            filter_1d_impl::<T, N>(
                src,
                &mut closed,
                kernel,
                MorphExOp::Closing,
                border_mode,
                morph_scalar,
            )?;
            T::morph_gradient(src, &closed, dst);
            Ok(())
        }
    }
}

pub fn morph_1d_f32(
    src: &[f32],
    dst: &mut [f32],
    kernel: &[u8],
    op: MorphExOp,
    border_mode: BorderMode,
    morph_scalar: MorphScalar,
) -> Result<(), MorphError> {
    filter_1d_impl::<f32, 1>(src, dst, kernel, op, border_mode, morph_scalar)
}

pub fn morph_1d_u16(
    src: &[u16],
    dst: &mut [u16],
    kernel: &[u8],
    op: MorphExOp,
    border_mode: BorderMode,
    morph_scalar: MorphScalar,
) -> Result<(), MorphError> {
    filter_1d_impl::<u16, 1>(src, dst, kernel, op, border_mode, morph_scalar)
}

pub fn morph_1d_u8(
    src: &[u8],
    dst: &mut [u8],
    kernel: &[u8],
    op: MorphExOp,
    border_mode: BorderMode,
    morph_scalar: MorphScalar,
) -> Result<(), MorphError> {
    filter_1d_impl::<u8, 1>(src, dst, kernel, op, border_mode, morph_scalar)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dilation() {
        let kernel = vec![0u8, 1u8, 1u8, 1u8, 1u8];
        let data = vec![
            0f32, 0., 0., 0., 0.5, 0.5, 1.5, 0., 1.5, 7., 9., 11., 15., 32., 0.75, 0.3, 0.25, 0.75,
            1., 9., 6.,
        ];
        let mut dst = vec![0.; data.len()];
        morph_1d_f32(
            &data,
            &mut dst,
            &kernel,
            MorphExOp::Dilate,
            BorderMode::Reflect,
            MorphScalar::new(0., 0., 0., 0.),
        )
        .unwrap();
        assert_eq!(dst[3], 0.5);
        assert_eq!(dst[7], 7.);
        assert_eq!(dst[17], 9.);
    }

    #[test]
    fn test_erosion() {
        let kernel = vec![0u8, 1u8, 1u8, 1u8, 1u8];
        let data = vec![
            0f32, 0., 0., 0., 0.5, 0.5, 1.5, 0., 1.5, 7., 9., 11., 15., 32., 0.75, 0.3, 0.25, 0.75,
            1., 9., 6.,
        ];
        let mut dst = vec![0.; data.len()];
        morph_1d_f32(
            &data,
            &mut dst,
            &kernel,
            MorphExOp::Erode,
            BorderMode::Reflect,
            MorphScalar::new(0., 0., 0., 0.),
        )
        .unwrap();
        assert_eq!(dst[3], 0.);
        assert_eq!(dst[7], 0.);
        assert_eq!(dst[12], 0.75);
        assert_eq!(dst[17], 0.25);
    }
}
