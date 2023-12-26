pub mod list_set;
pub mod tree_set;

fn main() {
    let mut tree: tree_set::TreeSet<&str> = tree_set::TreeSet::new();
    let mut list: list_set::ListSet<&str> = list_set::ListSet::new();
    tree.insert("Hello World");
    tree.insert("Za Warudo");
    tree.insert("MUDA MUDA MUDA");
    list.insert("cogito ergo sum");
    tree.print_tree();
    list.print_list();
}
