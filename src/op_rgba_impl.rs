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
use crate::arena::{make_arena, PREFERRED_KERNEL_SIZE_FOR_ARENA};
use crate::filter_op_declare::MorthOpFilterFlat2DRow;
use crate::filter_rgba::{MorthFilterRgba2D4Rows, MorthFilterRgba2DRow};
use crate::op_type::MorphOp;
use crate::se_scan::scan_se;
use crate::structuring_element::KernelShape;
use crate::unsafe_slice::UnsafeSlice;
use crate::{BorderMode, ImageSize, MorphologyThreadingPolicy};
use std::sync::Arc;

pub(crate) unsafe fn make_morphology_rgba<const OP_TYPE: u8>(
    src: &[u8],
    dst: &mut [u8],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    if src.len() != dst.len() {
        return Err("Source slice size and destination must match"
            .parse()
            .unwrap());
    }

    let kernel_width = structuring_element_size.width;
    let kernel_height = structuring_element_size.height;
    if kernel_height * kernel_width != structuring_element.len() {
        return Err(format!(
            "Structuring element expected to be {} but it was {}",
            kernel_height * kernel_width,
            structuring_element.len()
        ));
    }

    let width = image_size.width;
    let height = image_size.height;

    if src.len() != width * height * 4 {
        return Err(format!(
            "Image size expected to be {} but it was {}",
            width * height * 4,
            src.len()
        ));
    }

    let analyzed_se = scan_se(structuring_element, structuring_element_size);

    if analyzed_se.is_empty {
        for (src, dst) in src.iter().zip(dst.iter_mut()) {
            *dst = *src;
        }
        return Ok(());
    }

    let morph_op: MorphOp = OP_TYPE.into();

    let filter = Arc::new(MorthFilterRgba2DRow::new(morph_op));
    let filter_4_rows = Arc::new(MorthFilterRgba2D4Rows::new(morph_op));

    let arena = if structuring_element_size.width < PREFERRED_KERNEL_SIZE_FOR_ARENA
        && structuring_element_size.height < PREFERRED_KERNEL_SIZE_FOR_ARENA
        || border_mode != BorderMode::Clamp
    {
        let padded = make_arena::<4>(
            src,
            width as u32,
            height as u32,
            structuring_element_size,
            border_mode,
        );
        Some(padded)
    } else {
        None
    };

    let arena_arc = Arc::new(arena);

    if let Some(pool) = threading_policy.get_pool(image_size) {
        pool.scope(|scope| {
            let unsafe_slice = UnsafeSlice::new(dst);

            let mut yy = 0usize;

            for y in (0..height.saturating_sub(4)).step_by(4) {
                let cloned_se = analyzed_se.clone();
                let cloned_filter = filter_4_rows.clone();
                let cloned_arena = arena_arc.clone();
                scope.spawn(move |_| {
                    cloned_filter.dispatch_row(
                        src,
                        &unsafe_slice,
                        image_size,
                        cloned_se,
                        y,
                        &cloned_arena,
                    );
                });

                yy = y;
            }

            for y in yy..height {
                let cloned_se = analyzed_se.clone();
                let cloned_filter = filter.clone();
                let cloned_arena = arena_arc.clone();
                scope.spawn(move |_| {
                    cloned_filter.dispatch_row(
                        src,
                        &unsafe_slice,
                        image_size,
                        cloned_se,
                        y,
                        &cloned_arena,
                    );
                });
            }
        })
    } else {
        let mut yy = 0usize;

        let unsafe_slice = UnsafeSlice::new(dst);

        for y in (0..height.saturating_sub(4)).step_by(4) {
            let cloned_se = analyzed_se.clone();
            let cloned_filter = filter_4_rows.clone();
            let cloned_arena = arena_arc.clone();
            cloned_filter.dispatch_row(src, &unsafe_slice, image_size, cloned_se, y, &cloned_arena);

            yy = y;
        }

        for y in yy..height {
            let cloned_filter = filter.clone();
            let cloned_arena = arena_arc.clone();
            cloned_filter.dispatch_row(
                src,
                &unsafe_slice,
                image_size,
                analyzed_se.clone(),
                y,
                &cloned_arena,
            );
        }
    }

    Ok(())
}
