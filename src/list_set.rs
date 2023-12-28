use crate::solution::Set;
use std::fmt::Display;

pub struct ListSet<T> {
    list: Vec<T>,
}

impl<T> Iterator for ListSet<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl<T: Display + PartialEq> Set<T> for ListSet<T> {
    fn empty(&self) -> bool {
        return self.list.is_empty();
    }

    fn insert(&mut self, val: T) {
        if !self.list.contains(&val) {
            self.list.push(val);
        }
    }

    fn print(&self) {
        print!("[");
        self.list.iter().for_each(|entry| print!("{}, ", entry));
        print!("]");
    }
}

impl<T> ListSet<T> {
    pub fn new() -> Self {
        return ListSet {
            list: Vec::<T>::new(),
        };
    }
}
