use std::{fmt::Display, rc::Rc};

#[derive(Debug)]
pub enum Tree<T> {
    Fork(Rc<Tree<T>>, T, Rc<Tree<T>>),
    Empty,
}

#[derive(Debug)]
pub struct TreeSet<T> {
    tree: Rc<Tree<T>>,
}

impl<T: PartialOrd + Copy + Display> TreeSet<T> {
    pub fn new() -> Self {
        return TreeSet {
            tree: Rc::new(Tree::Empty),
        };
    }

    pub fn empty(&self) -> bool {
        match self.tree.as_ref() {
            Tree::Empty => true,
            _ => false,
        }
    }
    pub fn insert(&mut self, node: T) {
        let new_tree = self.tree.insert(node);
        self.tree = Rc::new(new_tree);
    }

    pub fn print_tree(&self) {
        self.tree.print();
        println!("");
    }
}

impl<T: PartialOrd + Copy + Display> Tree<T> {
    fn insert(&self, node_val: T) -> Self {
        match self {
            Tree::Fork(lt, v, rt) => {
                if node_val > *v {
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
