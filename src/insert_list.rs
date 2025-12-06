use std::{cmp::Ordering, ops::{Index, IndexMut, RangeInclusive}};

pub struct InsertList<T>(Vec<T>);

impl<T> InsertList<T> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn split_off(&mut self, idx: usize) -> Self {
        Self(self.0.split_off(idx))
    }

    pub fn remove(&mut self, idx: usize) -> T {
        self.0.remove(idx)
    }

    pub fn insert(&mut self, val: T, cmp: impl Fn(&T, &T) -> Ordering) {
        // let mut idx = 0;
        // let len = self.len();
        // while idx < len {
        //     if cmp(&self.0[idx], &val) == Ordering::Less {
        //         self.0.insert(idx, val);
        //         break;
        //     }
        //     idx += 1;
        // }

        self.0.insert(self.0.partition_point(|elem| cmp(elem, &val) == Ordering::Less), val);
    }

    pub fn with_cmp(mut list: Vec<T>, cmp: impl Fn(&T, &T) -> Ordering) -> Self {
        let mut res = Self(vec![]);
        for elem in list.drain(0..) {
            res.insert(elem, &cmp);
        }
        res
    }
}

impl<T: std::cmp::Ord> From<Vec<T>> for InsertList<T> {
    fn from(mut value: Vec<T>) -> Self {
        value.sort();
        Self(value)
    }
}

impl InsertList<RangeInclusive<i64>> {
    pub fn merge(&mut self) {
        let len = self.len();
        if len == 1 { return; }

        let mut curr = len-1;
        while curr > 0 {
            let mut idx = curr-1;

            loop {
                if self.0[curr].start() <= self.0[idx].end() {
                    let inner = self.0.remove(curr);
                    self.0[idx] = *(self.0[idx].start())..=*(inner.end().max(self.0[idx].end()));
                    break;
                }
                if idx == 0 { break; }

                idx -= 1;
            }

            curr -= 1;
        }
    }

    pub fn new(mut list: Vec<RangeInclusive<i64>>) -> Self {
        let len = list.len();
        if len == 0 { return Self(list); }

        list.sort_by(|a,b| a.start().cmp(b.start()));

        let mut res = Self(list);
        res.merge();

        res
    }
}

impl<T> Index<usize> for InsertList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for InsertList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
