use std::collections::HashMap;

pub fn run(data: &str) {
    let all: Vec<_> = data
        .split("\n\n")
        .into_iter()
        .filter(|y| !y.is_empty())
        .collect();

    let mut total = 0;
    let mut total_all_answered = 0;
    for group in all {
        let mut map: HashMap<char, u32> = HashMap::new();
        let people: Vec<_> = group.split("\n").collect();

        let people_len = people.len();

        for person in people {
            for question in person.chars() {
                *map.entry(question).or_default() += 1;
            }
        }

        total_all_answered += map.iter().filter(|y| *y.1 as usize == people_len).count();

        total += map.len();
    }

    println!("total = {} all answered = {}", total, total_all_answered);
}
