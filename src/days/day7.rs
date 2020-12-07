use std::collections::HashMap;

struct Holds {
    name: String,
    count: u64,
}

struct Rule {
    name: String,
    holds: Vec<Holds>,
}

fn all_names(map: &HashMap<String, Vec<Holds>>, name: String) -> Vec<String> {
    let mut lowest = Vec::new();
    for pair in map {
        if pair.1
            .iter().any(|y| y.name == name) {
            lowest.push(pair.0.clone());
        }
    }

    lowest
}

fn how_many_contain(map: &HashMap<String, Vec<Holds>>, name: String) -> Vec<String> {
    let mut all= Vec::new();
    let mut to_read: Vec<String> = all_names(map, name);

    while to_read.len() > 0 {
        let next = to_read.pop().unwrap();

        if !all.contains(&next) {
            all.push(next.clone());
        }

        to_read.extend(all_names(map, next));
    }

    all
}

fn depth_of(map: &HashMap<String, Vec<Holds>>, name: &str) -> u64 {
    let top = map.get(name).unwrap();

    let mut count = 0;
    for holds in top {
        let depth = depth_of(map, &holds.name) + 1;
        count += depth * holds.count;
    }

    count
}

pub fn run(data: &str) {
    let all: Vec<_> = data
        .split("\n")
        .into_iter()
        .filter(|y| !y.is_empty())
        .collect();

    let mut rules: HashMap<String, Vec<Holds>> = HashMap::new();

    for rule in all {
        let f = rule.find("bags").unwrap();
        let f_c = rule.find("contain").unwrap();
        let bag_name = (&rule[0..f - 1].trim()).to_string();
        let contains = &rule[f_c + 8..];

        if contains != "no other bags." {
            let mut all = Vec::new();
            if contains.find(",").is_some() {
                all.extend(contains.split(",").map(|y| y.trim()));
            } else {
                all.push(contains)
            }

            for rule in all {
                let count_idx = rule.find(" ").unwrap();
                let l_idx = rule.rfind(" ").unwrap();
                let count = rule[0..count_idx].parse::<u64>().unwrap();
                let name = (&rule[count_idx + 1..l_idx].trim()).to_string();

                rules
                    .entry(bag_name.clone())
                    .or_default()
                    .push(Holds { name, count });
            }
        } else {
            rules
                .entry(bag_name.clone())
                .or_default();
        }
    }

    let names = how_many_contain(&rules, "shiny gold".to_string());
    println!("{}", names.len());

    let counts = depth_of(&rules, "shiny gold");
    println!("{}", counts);
}
