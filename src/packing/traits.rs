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
use crate::packing::pack_gray_alpha::pack_gray_alpha_naive;
use crate::packing::pack_rgb::interleave_rgb_naive;
use crate::packing::pack_rgba::{interleave_rgba_naive, pack_rgba};
use crate::packing::unpack_gray_alpha::unpack_gray_alpha_naive;
use crate::packing::unpack_rgb::deinterleave_rgb_naive;
use crate::packing::unpack_rgba::deinterleave_rgba_naive;
use crate::packing::{
    pack_rgb, unpack_rgb, unpack_rgba, UnpackedGrayAlpha, UnpackedRgbImage, UnpackedRgbaImage,
};
use crate::ImageSize;

pub trait RgbPackable<T> {
    fn unpack(src: &[T], image_size: ImageSize) -> UnpackedRgbImage<T>;
    fn pack(unpacked_rgb_image: &UnpackedRgbImage<T>, dst: &mut [T], image_size: ImageSize);
}

impl RgbPackable<u8> for u8 {
    fn unpack(src: &[u8], image_size: ImageSize) -> UnpackedRgbImage<u8> {
        unpack_rgb(src, image_size)
    }

    fn pack(unpacked_rgb_image: &UnpackedRgbImage<u8>, dst: &mut [u8], image_size: ImageSize) {
        pack_rgb(unpacked_rgb_image, dst, image_size)
    }
}

impl RgbPackable<u16> for u16 {
    fn unpack(src: &[u16], image_size: ImageSize) -> UnpackedRgbImage<u16> {
        deinterleave_rgb_naive(src, image_size.width, image_size.height)
    }

    fn pack(unpacked_rgb_image: &UnpackedRgbImage<u16>, dst: &mut [u16], image_size: ImageSize) {
        interleave_rgb_naive(unpacked_rgb_image, dst, image_size.width, image_size.height)
    }
}

impl RgbPackable<f32> for f32 {
    fn unpack(src: &[f32], image_size: ImageSize) -> UnpackedRgbImage<f32> {
        deinterleave_rgb_naive(src, image_size.width, image_size.height)
    }

    fn pack(unpacked_rgb_image: &UnpackedRgbImage<f32>, dst: &mut [f32], image_size: ImageSize) {
        interleave_rgb_naive(unpacked_rgb_image, dst, image_size.width, image_size.height)
    }
}

pub trait RgbaPackable<T> {
    fn unpack(src: &[T], image_size: ImageSize) -> UnpackedRgbaImage<T>;
    fn pack(unpacked_rgb_image: &UnpackedRgbaImage<T>, dst: &mut [T], image_size: ImageSize);
}

impl RgbaPackable<u8> for u8 {
    fn pack(unpacked_rgb_image: &UnpackedRgbaImage<u8>, dst: &mut [u8], image_size: ImageSize) {
        pack_rgba(&unpacked_rgb_image, dst, image_size)
    }

    fn unpack(src: &[u8], image_size: ImageSize) -> UnpackedRgbaImage<u8> {
        unpack_rgba(src, image_size)
    }
}

impl RgbaPackable<u16> for u16 {
    fn pack(unpacked_rgb_image: &UnpackedRgbaImage<u16>, dst: &mut [u16], image_size: ImageSize) {
        interleave_rgba_naive(unpacked_rgb_image, dst, image_size.width, image_size.height)
    }

    fn unpack(src: &[u16], image_size: ImageSize) -> UnpackedRgbaImage<u16> {
        deinterleave_rgba_naive(src, image_size.width, image_size.height)
    }
}

impl RgbaPackable<f32> for f32 {
    fn pack(unpacked_rgb_image: &UnpackedRgbaImage<f32>, dst: &mut [f32], image_size: ImageSize) {
        interleave_rgba_naive(unpacked_rgb_image, dst, image_size.width, image_size.height)
    }

    fn unpack(src: &[f32], image_size: ImageSize) -> UnpackedRgbaImage<f32> {
        deinterleave_rgba_naive(src, image_size.width, image_size.height)
    }
}

pub trait GrayAlphaPackable<T> {
    fn unpack(src: &[T], image_size: ImageSize) -> UnpackedGrayAlpha<T>;
    fn pack(unpacked_rgb_image: &UnpackedGrayAlpha<T>, dst: &mut [T], image_size: ImageSize);
}

impl GrayAlphaPackable<u8> for u8 {
    fn pack(unpacked_rgb_image: &UnpackedGrayAlpha<u8>, dst: &mut [u8], image_size: ImageSize) {
        pack_gray_alpha_naive(
            &unpacked_rgb_image,
            dst,
            image_size.width,
            image_size.height,
        )
    }

    fn unpack(src: &[u8], image_size: ImageSize) -> UnpackedGrayAlpha<u8> {
        unpack_gray_alpha_naive(src, image_size.width, image_size.height)
    }
}

impl GrayAlphaPackable<u16> for u16 {
    fn pack(unpacked_rgb_image: &UnpackedGrayAlpha<u16>, dst: &mut [u16], image_size: ImageSize) {
        pack_gray_alpha_naive(
            &unpacked_rgb_image,
            dst,
            image_size.width,
            image_size.height,
        )
    }

    fn unpack(src: &[u16], image_size: ImageSize) -> UnpackedGrayAlpha<u16> {
        unpack_gray_alpha_naive(src, image_size.width, image_size.height)
    }
}

impl GrayAlphaPackable<f32> for f32 {
    fn pack(unpacked_rgb_image: &UnpackedGrayAlpha<f32>, dst: &mut [f32], image_size: ImageSize) {
        pack_gray_alpha_naive(
            &unpacked_rgb_image,
            dst,
            image_size.width,
            image_size.height,
        )
    }

    fn unpack(src: &[f32], image_size: ImageSize) -> UnpackedGrayAlpha<f32> {
        unpack_gray_alpha_naive(src, image_size.width, image_size.height)
    }
}
