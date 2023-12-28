use crate::utility::*;

pub trait Set<T> {
    fn empty(&self) -> bool;
    fn insert(&mut self, val: T) -> ();
    fn print(&self);
    fn elem(&self, val: &T) -> bool;
}

pub fn opposites_solution<T: Clone>(
    set: &mut dyn Set<String>,
    valid_list: &Vec<String>,
    word_list: &Vec<&str>,
) -> Vec<(String, String)> {
    let mut solution_list: Vec<(String, String)> = Vec::<(String, String)>::new();
    valid_list.iter().for_each(|val| set.insert(val.clone()));
    word_list
        .iter()
        .filter(|word| word.len() == 6)
        .for_each(|word| {
            let generated_words = possible_words(word.to_string());
            for generated_word in generated_words {
                if set.elem(&generated_word) {
                    solution_list.push((word.to_string(), generated_word));
                }
            }
        });

    solution_list
}
