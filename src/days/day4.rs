use regex::Regex;
use std::collections::HashMap;

pub fn run(data: &str) {
    let lines: Vec<_> = data
        .split("\n\n")
        .into_iter()
        .filter(|y| !y.is_empty())
        .map(|y| y.replace("\n", " "))
        .collect();

    let re1 = Regex::new(r"^[0-9A-f]*$").unwrap();
    let re2 = Regex::new(r"^[0-9]{9}$").unwrap();

    let mut total = 0;
    let mut total2 = 0;
    for line in lines {
        let split_spaces: Vec<_> = line.split(" ").collect();
        if split_spaces.len() < 7 {
            continue;
        }

        let mut valid = true;
        let mut map = HashMap::new();
        for split in split_spaces {
            let mut pair = split.split(":");
            let key = pair.next().unwrap();
            let value = pair.next().unwrap();

            if key == "byr" && !map.contains_key(key) {
                let parsed = value.parse::<i32>().unwrap();

                if parsed >= 1920 && parsed <= 2002 {
                    map.insert(key, value);
                }
            } else if key == "iyr" && !map.contains_key(key) {
                let parsed = value.parse::<i32>().unwrap();

                if parsed >= 2010 && parsed <= 2020 {
                    map.insert(key, value);
                }
            } else if key == "eyr" && !map.contains_key(key) {
                let parsed = value.parse::<i32>().unwrap();

                if parsed >= 2020 && parsed <= 2030 {
                    map.insert(key, value);
                }
            } else if key == "hgt" && !map.contains_key(key) {
                if value.ends_with("cm") {
                    let parsed = value[0..value.len() - 2].parse::<i32>().unwrap();
                    if parsed >= 150 && parsed <= 193 {
                        map.insert(key, value);
                    }
                } else if value.ends_with("in") {
                    let parsed = value[0..value.len() - 2].parse::<i32>().unwrap();
                    if parsed >= 59 && parsed <= 76 {
                        map.insert(key, value);
                    }
                }
            } else if key == "hcl" && !map.contains_key(key) {
                if value.starts_with("#") && re1.is_match(&value[1..]) {
                    map.insert(key, value);
                }
            } else if key == "ecl" && !map.contains_key(key) {
                if value == "amb"
                    || value == "blu"
                    || value == "brn"
                    || value == "gry"
                    || value == "grn"
                    || value == "hzl"
                    || value == "oth"
                {
                    map.insert(key, value);
                }
            } else if key == "pid" && !map.contains_key(key) {
                if re2.is_match(value) {
                    map.insert(key, value);
                }
            } else if key == "cid" && !map.contains_key(key) {
                map.insert(key, value);
            } else {
                valid = false;
                break;
            }
        }

        if !valid {
            continue;
        }

        if map.contains_key("byr")
            && map.contains_key("iyr")
            && map.contains_key("eyr")
            && map.contains_key("hgt")
            && map.contains_key("hcl")
            && map.contains_key("ecl")
            && map.contains_key("pid")
        {
            total2 += 1;
        }

        total += 1;
    }

    println!("{} {}", total, total2);
}
