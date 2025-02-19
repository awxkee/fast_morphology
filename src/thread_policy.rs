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
use crate::ImageSize;
use rayon::ThreadPool;
use std::num::NonZeroUsize;
use std::thread::available_parallelism;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum MorphologyThreadingPolicy {
    Single,
    Fixed(NonZeroUsize),
    #[default]
    Adaptive,
}

impl MorphologyThreadingPolicy {
    pub fn thread_count(&self, for_size: ImageSize) -> usize {
        match self {
            MorphologyThreadingPolicy::Single => 1,
            MorphologyThreadingPolicy::Adaptive => (for_size.width * for_size.height / (256 * 256))
                .clamp(1, Self::available_parallelism()),
            MorphologyThreadingPolicy::Fixed(fixed) => fixed.get(),
        }
    }

    pub fn get_pool(&self, for_size: ImageSize) -> Option<ThreadPool> {
        if *self == MorphologyThreadingPolicy::Single {
            return None;
        }
        let threads_count = self.thread_count(for_size);
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads_count)
            .build()
            .ok()
    }

    fn available_parallelism() -> usize {
        available_parallelism()
            .unwrap_or_else(|_| NonZeroUsize::new(1).unwrap())
            .get()
            .max(1)
    }
}
