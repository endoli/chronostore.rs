// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{Entry, Summary};

/// A [`Summary`] that tracks the minimum and maximum values.
#[derive(Default)]
pub struct SimpleSummary<V: Copy + Default + PartialOrd> {
    /// The minimum value seen by this summary.
    pub min: V,
    /// The maximum value seen by this summary.
    pub max: V,
}

impl<V: Copy + Default + PartialOrd> Summary<V> for SimpleSummary<V> {
    fn batch_update(&mut self, entries: &[Entry<V>]) {
        for e in entries {
            self.update(e);
        }
    }

    fn update(&mut self, entry: &Entry<V>) {
        if entry.value > self.max {
            self.max = entry.value;
        }
        if entry.value < self.min {
            self.min = entry.value;
        }
    }
}
