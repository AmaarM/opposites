use std::fmt::Display;

pub struct ListSet<T> {
    list: Vec<T>,
}

impl<T: PartialEq + Display> ListSet<T> {
    pub fn new() -> Self {
        return ListSet {
            list: Vec::<T>::new(),
        };
    }

    pub fn empty(&self) -> bool {
        return self.list.is_empty();
    }

    pub fn insert(&mut self, val: T) {
        if !self.list.contains(&val) {
            self.list.push(val);
        }
    }

    pub fn print_list(&self) {
        print!("[");
        self.list.iter().for_each(|entry| print!("{}, ", entry));
        print!("]");
    }
}
