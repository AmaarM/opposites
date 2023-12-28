use crate::solution::Set;
use std::{fmt::Debug, fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub enum Tree<T> {
    Fork(Rc<Tree<T>>, T, Rc<Tree<T>>),
    Empty,
}

#[derive(Debug)]
pub struct TreeSet<T> {
    tree: Rc<Tree<T>>,
    tree_vec: Vec<Tree<T>>,
}

impl<T> Iterator for TreeSet<T> {
    type Item = Tree<T>;

    fn next(&mut self) -> Option<Tree<T>> {
        self.tree_vec.pop()
    }
}

impl<T: PartialOrd + Copy + Display + Debug> Set<T> for TreeSet<T> {
    fn empty(&self) -> bool {
        match self.tree.as_ref() {
            Tree::Empty => true,
            _ => false,
        }
    }
    fn insert(&mut self, node: T) {
        let new_tree = self.tree.insert(node);
        self.tree_vec.clear();
        self.tree_vec.push(new_tree.clone());
        self.tree = Rc::new(new_tree);
    }

    fn print(&self) {
        self.tree.print();
        println!("");
        println!("Tree Vector: {:#?}", self.tree_vec);
    }
}

impl<T> TreeSet<T> {
    pub fn new() -> Self {
        return TreeSet {
            tree: Rc::new(Tree::Empty),
            tree_vec: Vec::<Tree<T>>::new(),
        };
    }
}

impl<T: PartialOrd + Copy + Display> Tree<T> {
    fn insert(&self, node_val: T) -> Self {
        match self {
            Tree::Fork(lt, v, rt) => {
                if node_val >= *v {
                    Tree::Fork(Rc::clone(lt), *v, Rc::new(rt.insert(node_val)))
                } else {
                    Tree::Fork(Rc::new(lt.insert(node_val)), *v, Rc::clone(rt))
                }
            }
            Tree::Empty => Tree::Fork(Rc::new(Tree::Empty), node_val, Rc::new(Tree::Empty)),
        }
    }

    fn print(&self) {
        match self {
            Tree::Fork(lt, v, rt) => {
                print!("({} ", *v);
                lt.print();
                rt.print();
                print!(")");
            }
            Tree::Empty => {
                print!("Empty ");
            }
        }
    }
}
