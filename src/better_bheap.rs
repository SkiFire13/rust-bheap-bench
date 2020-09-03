use std::mem::swap;
use std::vec::Vec;

use crate::hole::Hole;

#[derive(Clone)]
pub struct BinaryHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> BinaryHeap<T> {
    pub fn new() -> BinaryHeap<T> {
        BinaryHeap { data: vec![] }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop().map(|mut item| {
            if !self.is_empty() {
                swap(&mut item, &mut self.data[0]);
                unsafe {
                    self.sift_down_to_bottom(0);
                }
            }
            item
        })
    }

    pub fn push(&mut self, item: T) {
        let old_len = self.len();
        self.data.push(item);
        self.sift_up(0, old_len);
    }

    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let mut end = self.len();
        while end > 1 {
            end -= 1;
            self.data.swap(0, end);
            unsafe {
                self.sift_down_range_odd(0, end + (end & 1) - 1);
                if end > 1 && self.data.get_unchecked(end - 1) > self.data.get_unchecked((end - 2) / 2) {
                    // `[T]::swap_unchecked` when?
                    std::ptr::swap(
                        self.data.get_unchecked_mut(end - 1),
                        self.data.get_unchecked_mut((end - 2) / 2)
                    )
                }
            }
        }
        self.into_vec()
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    fn sift_up(&mut self, start: usize, pos: usize) -> usize {
        unsafe {
            let mut hole = Hole::new(&mut self.data, pos);

            while hole.pos() > start {
                let parent = (hole.pos() - 1) / 2;
                if hole.element() <= hole.get(parent) {
                    break;
                }
                hole.move_to(parent);
            }
            hole.pos()
        }
    }

    unsafe fn sift_down_range_odd(&mut self, pos: usize, end: usize) {
        debug_assert!(end % 2 == 1);
        let mut hole = Hole::new(&mut self.data, pos);
        let mut child = 2 * pos + 1;

        while child < end {
            let right = child + 1;
            child = child + (hole.get(child) <= hole.get(right)) as usize;
            if hole.element() >= hole.get(child) {
                break;
            }
            hole.move_to(child);
            child = 2 * hole.pos() + 1;
        }
    }

    unsafe fn sift_down_to_bottom(&mut self, mut pos: usize) {
        debug_assert!(self.len() != 0);
        let len = self.len();
        let end = len + (len & 1) - 1;
        let start = pos;

        let mut hole = Hole::new(&mut self.data, pos);
        let mut child = 2 * pos + 1;
        while child < end {
            let right = child + 1;
            child = child + (hole.get(child) <= hole.get(right)) as usize;
            hole.move_to(child);
            child = 2 * hole.pos() + 1;
        }
        if child == len - 1 {
            hole.move_to(child);
        }
        pos = hole.pos();
        drop(hole);

        self.sift_up(start, pos);
    }

    // This is used in PeekMut which I'm not currently implementing
    #[allow(dead_code)]
    unsafe fn sift_down(&mut self, pos: usize) {
        debug_assert!(self.len() != 0);
        let odd_len = self.len() + (self.len() & 1) - 1;
        self.sift_down_range_odd(pos, odd_len);
        if self.len() != odd_len {
            self.sift_up(0, odd_len);
        }
    }

    fn rebuild(&mut self) {
        unsafe {
            if self.len() > 1 {
                let mut n = self.len() / 2;
                let odd_len = self.len() + (self.len() & 1) - 1;
                while n > 0 {
                    n -= 1;
                    self.sift_down_range_odd(n, odd_len);
                }
                if self.len() != odd_len {
                    self.sift_up(0, odd_len);
                }
            }
        }
    }
}

impl<T: Ord> From<Vec<T>> for BinaryHeap<T> {
    fn from(vec: Vec<T>) -> BinaryHeap<T> {
        let mut heap = BinaryHeap { data: vec };
        heap.rebuild();
        heap
    }
}

impl<T> From<BinaryHeap<T>> for Vec<T> {
    fn from(heap: BinaryHeap<T>) -> Vec<T> {
        heap.data
    }
}

#[cfg(test)]
mod tests {
    use super::BinaryHeap;
    use rand::prelude::*;

    fn get_inputs() -> impl IntoIterator<Item = Vec<isize>> {
        static SIZES: &'static [usize] = &[
            1, 2, 3, 7, 8, 9, 15, 16, 17, 24, 31, 32, 33, 255, 256, 257, 315, 316,
        ];

        let mut rng = rand::thread_rng();

        SIZES.iter().flat_map(move |&size| {
            vec![
                (0..size as isize).collect(),
                (0..size as isize).rev().collect(),
                (1..size as isize).chain(std::iter::once(0)).collect(),
                (0..size as isize)
                    .step_by(2)
                    .chain((1..size as isize).step_by(2).rev())
                    .collect(),
                {
                    let mut vec = (0..size as isize).collect::<Vec<_>>();
                    vec.shuffle(&mut rng);
                    vec
                },
                [0, 1]
                    .iter()
                    .copied()
                    .cycle()
                    .take(size * 4)
                    .choose_multiple(&mut rng, size),
            ]
        })
    }

    #[test]
    fn build_heap() {
        fn is_heap<T: Ord>(slice: &[T]) -> bool {
            (0..slice.len() / 2).all(|i| {
                Some(&slice[i]) >= std::cmp::max(slice.get(2 * i + 1), slice.get(2 * i + 2))
            })
        }

        for input in get_inputs() {
            let bheap = BinaryHeap::from(input);
            assert!(is_heap(&bheap.data));
        }
    }

    #[test]
    fn into_sorted_vec() {
        for input in get_inputs() {
            let bheap = BinaryHeap::from(input);
            let vec = bheap.into_sorted_vec();
            for win in vec.windows(2) {
                if win[0] > win[1] {
                    dbg!(&vec, win[0], win[1]);
                }
                assert!(win[0] <= win[1]);
            }
        }
    }

    #[test]
    fn pop() {
        for input in get_inputs() {
            let mut bheap = BinaryHeap::from(input.clone());
            let vec = std::iter::from_fn(|| bheap.pop()).collect::<Vec<_>>();
            let mut sorted_input = input;
            sorted_input.sort_unstable_by(|a, b| a.cmp(b).reverse());
            assert_eq!(vec, sorted_input);
        }
    }
}
