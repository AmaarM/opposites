use std::fs;

pub fn read_file(file_name: String) -> String {
    fs::read_to_string(file_name).unwrap_or("".to_string())
}

pub fn implode(list: Vec<char>) -> String {
    let mut word = String::new();
    list.iter().for_each(|c| word.push(c.clone()));
    word
}

pub fn explode(word: String) -> Vec<char> {
    let mut list = Vec::<char>::new();
    word.chars().for_each(|c| list.push(c.clone()));
    list
}

pub fn prefixes_suffixes<T: Clone>(list: Vec<T>) -> Vec<(Vec<T>, Vec<T>)> {
    let (_, _, mut sofar) = list.iter().fold(
        (
            Vec::<T>::new(),
            list.clone(),
            Vec::<(Vec<T>, Vec<T>)>::new(),
        ),
        |(mut pre, suf, mut sofar), _| match (pre.as_slice(), suf.as_slice()) {
            (pre_slice, []) => {
                let (_, rest) = pre_slice.split_first().unwrap();
                sofar.push((list.clone(), rest.to_vec()));
                (pre.clone(), rest.to_vec(), sofar)
            }
            ([], suf_slice) => {
                let (val, rest) = suf_slice.split_first().unwrap();
                pre.push(val.clone());
                sofar.push((pre.clone(), rest.to_vec()));
                (pre, rest.to_vec(), sofar)
            }
            (_, suf_slice) => {
                let (val, rest) = suf_slice.split_first().unwrap();
                pre.push(val.clone());
                sofar.push((pre.clone(), rest.to_vec()));
                (pre, rest.to_vec(), sofar)
            }
        },
    );

    sofar.push((vec![], list));
    sofar
}

pub fn make_initial_list<T>(word: String) -> Vec<String> {
    let pre_suf = prefixes_suffixes::<char>(explode(word));
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    let mut all_words: Vec<String> = Vec::<String>::new();
    pre_suf.iter().for_each(|(pre, suf)| {
        let new_list = match !vowels.contains(pre.last().unwrap_or(&'z')) {
            true => {
                let mut prefix_without_list = pre.clone();
                prefix_without_list.pop();
                let pre_word = prefix_without_list.iter().collect::<String>();
                let mut new_words = Vec::<String>::new();
                vowels.iter().for_each(|vowel| {
                    let mut temp_pre = pre_word.clone();
                    temp_pre.push(vowel.clone());
                    temp_pre.push_str(suf.clone().iter().collect::<String>().as_str());
                    new_words.push(temp_pre);
                });
                new_words
            }
            false => Vec::<String>::new(),
        };
        new_list
            .iter()
            .for_each(|new_word| all_words.push(new_word.clone()));
    });

    all_words
}

pub fn possible_words<T>(word: String) -> Vec<String> {
    let initial_list = make_initial_list::<String>(word);
    println!("{:#?}", initial_list);
    initial_list
        .into_iter()
        .filter(|word| word.len() > 6)
        .collect()
}
