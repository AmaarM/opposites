use solution::opposites_solution;

pub mod list_set;
pub mod solution;
pub mod tree_set;
pub mod utility;

fn main() {
    let mut tree: tree_set::TreeSet<String> = tree_set::TreeSet::new();
    let mut list: list_set::ListSet<String> = list_set::ListSet::new();
    let binding = utility::read_file("words.txt".to_string());
    let words = binding.split("\n").collect::<Vec<&str>>();
    let sunday: Vec<&str> = vec![
        "defect", "biking", "strong", "simple", "salmon", "cloudy", "zither", "stanch", "stance",
        "defeat", "bikini", "clouds", "sarong", "gently", "simile", "saloon", "either", "stance",
        "seance", "gentle",
    ];
    let parsed_sunday = sunday
        .into_iter()
        .map(|word| word.to_string())
        .collect::<Vec<String>>();
    let solution_tree = opposites_solution::<String>(&mut tree, &parsed_sunday, &words);
    let solution_list = opposites_solution::<String>(&mut list, &parsed_sunday, &words);
    println!("{:#?}", solution_tree);
    println!("{:#?}", solution_list);
}
