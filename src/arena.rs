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
use crate::arena_roi::copy_roi;
use crate::border_mode::{reflect_index, reflect_index_101, BorderMode};
use crate::filter_op_declare::Arena;
use crate::structuring_element::KernelShape;

/// Pads an image with chosen border strategy
pub fn make_arena<T, const COMPONENTS: usize>(
    image: &[T],
    width: u32,
    height: u32,
    kernel_size: KernelShape,
    border_mode: BorderMode,
) -> Arena<T>
where
    T: Default + Copy,
{
    let (kw, kh) = (kernel_size.width, kernel_size.height);

    let pad_w = kw / 2;
    let pad_h = kh / 2;

    let new_height = height as usize + 2 * pad_h;
    let new_width = width as usize + 2 * pad_w;

    let mut padded_image = vec![T::default(); new_height * new_width * COMPONENTS];

    let old_stride = width as usize * COMPONENTS;
    let new_stride = new_width * COMPONENTS;

    unsafe {
        copy_roi(
            padded_image.get_unchecked_mut(pad_h * new_stride + (pad_w * COMPONENTS)..),
            image,
            new_stride,
            old_stride,
            height as usize,
        );
    }

    match border_mode {
        BorderMode::Clamp => {
            for i in 0..pad_h {
                for j in 0..pad_w {
                    let y = i.saturating_sub(pad_h).min(height as usize - 1);
                    let x = j.saturating_sub(pad_w).min(width as usize - 1);
                    unsafe {
                        let v_dst = i * new_stride + j * COMPONENTS;
                        let v_src = y * old_stride + x * COMPONENTS;
                        for i in 0..COMPONENTS {
                            *padded_image.get_unchecked_mut(v_dst + i) =
                                *image.get_unchecked(v_src + i);
                        }
                    }
                }
            }

            for i in (height as usize + pad_h)..new_height {
                for j in (width as usize + pad_w)..new_width {
                    let y = i.saturating_sub(pad_h).min(height as usize - 1);
                    let x = j.saturating_sub(pad_w).min(width as usize - 1);
                    unsafe {
                        let v_dst = i * new_stride + j * COMPONENTS;
                        let v_src = y * old_stride + x * COMPONENTS;
                        for i in 0..COMPONENTS {
                            *padded_image.get_unchecked_mut(v_dst + i) =
                                *image.get_unchecked(v_src + i);
                        }
                    }
                }
            }
        }
        BorderMode::Wrap => {
            for i in 0..pad_h {
                for j in 0..pad_w {
                    let y = (i as i64 - pad_h as i64).rem_euclid(height as i64 - 1) as usize;
                    let x = (j as i64 - pad_w as i64).rem_euclid(width as i64 - 1) as usize;
                    unsafe {
                        let v_dst = i * new_stride + j * COMPONENTS;
                        let v_src = y * old_stride + x * COMPONENTS;
                        for i in 0..COMPONENTS {
                            *padded_image.get_unchecked_mut(v_dst + i) =
                                *image.get_unchecked(v_src + i);
                        }
                    }
                }
            }

            for i in (height as usize + pad_h)..new_height {
                for j in (width as usize + pad_w)..new_width {
                    let y = (i as i64 - pad_h as i64).rem_euclid(height as i64 - 1) as usize;
                    let x = (j as i64 - pad_w as i64).rem_euclid(width as i64 - 1) as usize;
                    unsafe {
                        let v_dst = i * new_stride + j * COMPONENTS;
                        let v_src = y * old_stride + x * COMPONENTS;
                        for i in 0..COMPONENTS {
                            *padded_image.get_unchecked_mut(v_dst + i) =
                                *image.get_unchecked(v_src + i);
                        }
                    }
                }
            }
        }
        BorderMode::Reflect => {
            for i in 0..pad_h {
                for j in 0..pad_w {
                    let y = reflect_index(i as i64 - pad_h as i64, height as i64 - 1);
                    let x = reflect_index(j as i64 - pad_w as i64, width as i64 - 1);
                    unsafe {
                        let v_dst = i * new_stride + j * COMPONENTS;
                        let v_src = y * old_stride + x * COMPONENTS;
                        for i in 0..COMPONENTS {
                            *padded_image.get_unchecked_mut(v_dst + i) =
                                *image.get_unchecked(v_src + i);
                        }
                    }
                }
            }

            for i in (height as usize + pad_h)..new_height {
                for j in (width as usize + pad_w)..new_width {
                    let y = reflect_index(i as i64 - pad_h as i64, height as i64 - 1);
                    let x = reflect_index(j as i64 - pad_w as i64, width as i64 - 1);
                    unsafe {
                        let v_dst = i * new_stride + j * COMPONENTS;
                        let v_src = y * old_stride + x * COMPONENTS;
                        for i in 0..COMPONENTS {
                            *padded_image.get_unchecked_mut(v_dst + i) =
                                *image.get_unchecked(v_src + i);
                        }
                    }
                }
            }
        }
        BorderMode::Reflect101 => {
            for i in 0..pad_h {
                for j in 0..pad_w {
                    let y = reflect_index_101(i as i64 - pad_h as i64, height as i64 - 1);
                    let x = reflect_index_101(j as i64 - pad_w as i64, width as i64 - 1);
                    unsafe {
                        let v_dst = i * new_stride + j * COMPONENTS;
                        let v_src = y * old_stride + x * COMPONENTS;
                        for i in 0..COMPONENTS {
                            *padded_image.get_unchecked_mut(v_dst + i) =
                                *image.get_unchecked(v_src + i);
                        }
                    }
                }
            }

            for i in (height as usize + pad_h)..new_height {
                for j in (width as usize + pad_w)..new_width {
                    let y = reflect_index_101(i as i64 - pad_h as i64, height as i64 - 1);
                    let x = reflect_index_101(j as i64 - pad_w as i64, width as i64 - 1);
                    unsafe {
                        let v_dst = i * new_stride + j * COMPONENTS;
                        let v_src = y * old_stride + x * COMPONENTS;
                        for i in 0..COMPONENTS {
                            *padded_image.get_unchecked_mut(v_dst + i) =
                                *image.get_unchecked(v_src + i);
                        }
                    }
                }
            }
        }
    }

    Arena::new(padded_image, new_width, new_height, pad_w, pad_h)
}
