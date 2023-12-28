use crate::solution::Set;
use std::{fmt::Debug, fmt::Display, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
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

impl<T: PartialOrd + Display + Debug + Clone> Set<T> for TreeSet<T> {
    fn empty(&self) -> bool {
        match self.tree.as_ref() {
            Tree::Empty => true,
            _ => false,
        }
    }
    fn insert(&mut self, node: T) {
        let mut new_tree_vec = self.tree_vec.clone();
        let new_tree = self.tree.insert(node, &mut new_tree_vec);
        self.tree_vec = new_tree_vec;
        self.tree = Rc::new(new_tree);
    }

    fn print(&self) {
        self.tree.print();
        println!("");
        println!("Tree Vector: {:#?}", self.tree_vec);
    }

    fn elem(&self, val: &T) -> bool {
        self.tree_vec.iter().fold(false, |acc, tree| match tree {
            Tree::Fork(_, v, _) => acc || *val == *v,
            _ => false,
        })
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

impl<T: PartialOrd + Display + Clone> Tree<T> {
    fn insert(&self, node_val: T, mut tree_vec: &mut Vec<Tree<T>>) -> Self {
        match self {
            Tree::Fork(lt, v, rt) => {
                if node_val >= *v {
                    Tree::Fork(
                        Rc::clone(lt),
                        v.clone(),
                        Rc::new(rt.insert(node_val, &mut tree_vec)),
                    )
                } else {
                    Tree::Fork(
                        Rc::new(lt.insert(node_val, &mut tree_vec)),
                        v.clone(),
                        Rc::clone(rt),
                    )
                }
            }
            Tree::Empty => {
                let new_child = Tree::Fork(Rc::new(Tree::Empty), node_val, Rc::new(Tree::Empty));
                tree_vec.push(new_child.clone());
                new_child
            }
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
