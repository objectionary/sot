// Copyright (c) 2022-2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::Sodg;
use emap::Map;

impl<const N: usize> Sodg<N> {
    /// Make an empty [`Sodg`], with no vertices and no edges.
    ///
    /// # Panics
    ///
    /// May panic if vertices provided to alerts are absent (should never happen, though).
    #[must_use]
    pub fn empty(cap: usize) -> Self {
        Self {
            alive: Map::with_capacity_none(cap),
            edges: Map::with_capacity_some(cap, micromap::Map::new()),
            data: Map::with_capacity_none(cap),
            taken: Map::with_capacity_none(cap),
            next_v: 0,
        }
    }
}

#[test]
fn makes_an_empty_sodg() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(0);
    assert_eq!(1, g.len());
}
