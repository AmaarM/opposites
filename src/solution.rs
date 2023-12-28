use crate::utility::*;

// TODO implement elem for both sets
pub trait Set<T> {
    fn empty(&self) -> bool;
    fn insert(&mut self, val: T) -> ();
    fn print(&self);
}

// TODO write loop to filter valid words from list of all possible words
pub fn opposites_solution<T: Clone>(set: &mut dyn Set<T>, valid_list: Vec<T>) -> Vec<(T, T)> {
    valid_list.iter().for_each(|val| set.insert(val.clone()));
    set.print();

    vec![(
        valid_list.clone().pop().unwrap(),
        valid_list.clone().pop().unwrap(),
    )]
}
