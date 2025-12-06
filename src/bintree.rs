use std::ops::RangeInclusive;
use crate::insert_list::InsertList;

#[derive(Debug)]
pub struct BinTree<T> {
    val: T,
    left: Option<Box<BinTree<T>>>,
    right: Option<Box<BinTree<T>>>,
}

impl<T> BinTree<T> {
    pub fn new(mut list: InsertList<T>) -> Self {
        let len = list.len();
        
        match len {
            1 => return Self { val: list.remove(0), left: None, right: None },
            2 => return Self { 
                val: list.remove(1),
                left: Some(Box::new(Self { val: list.remove(0), left: None, right: None })),
                right: None,
            },
            3 =>return Self { 
                val: list.remove(1),
                left: Some(Box::new(Self { val: list.remove(0), left: None, right: None })),
                right: Some(Box::new(Self { val: list.remove(2), left: None, right: None })),
            },
            _ => {},
        }

        let mid = len/2;
        let mut val = list.split_off(mid);
        let right_list = val.split_off(1);

        let left = Some(Box::new(Self::new(list)));
        let right = Some(Box::new(Self::new(right_list)));

        Self { val: val.remove(0), left, right }
    }
}

pub type RangeTree = BinTree<RangeInclusive<i64>>;

impl RangeTree {
    pub fn check_fresh(&self, id: i64) -> bool {
        let start = self.val.start();
        if id >= *start {
            let end = self.val.end();
            if id <= *end {
                true
            } else {
                check_fresh(&self.right, id)
            }
        } else {
            check_fresh(&self.left, id)
        }
    }

    pub fn fresh_count(&self) -> usize {
        let curr = self.val.end() - self.val.start() + 1;
        let left = fresh_count(&self.left);
        let right = fresh_count(&self.right);

        curr as usize + left + right
    }
}

fn fresh_count(opt: &Option<Box<RangeTree>>) -> usize {
    match opt {
        None => 0,
        Some(tt) => tt.fresh_count(),
    }
}

fn check_fresh(opt: &Option<Box<RangeTree>>, id: i64) -> bool {
    match opt {
        None => false,
        Some(tt) => tt.check_fresh(id),
    }
}
