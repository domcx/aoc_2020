pub fn run(data: &str) {
    let lines: Vec<_> = data
        .split("\n")
        .into_iter()
        .filter(|y| !y.is_empty())
        .collect();

    let height = lines.len();

    let map = map_trees(&lines);

    let t1 = count_trees(&map, height, 1, 1);
    let t2 = count_trees(&map, height, 3, 1);
    let t3 = count_trees(&map, height, 5, 1);
    let t4 = count_trees(&map, height, 7, 1);
    let t5 = count_trees(&map, height, 1, 2);

    println!(
        "{},{},{},{},{} {}",
        t1,
        t2,
        t3,
        t4,
        t5,
        t1 * t2 * t3 * t4 * t5
    );
}

fn map_trees(lines: &[&str]) -> Vec<Vec<i32>> {
    let mut map = Vec::new();

    for line in lines {
        let mut next = Vec::new();
        for spot in line.chars() {
            if spot == '#' {
                next.push(1)
            } else {
                next.push(0);
            }
        }

        map.push(next);
    }

    map
}

fn count_trees(map: &Vec<Vec<i32>>, height: usize, right: usize, down: usize) -> u64 {
    let mut x = right;
    let mut y = down;

    let mut trees = 0;

    while y < height {
        if map[y][x] == 1 {
            trees += 1;
        }

        x = (x + right) % map[y].len();
        y += down;
    }

    trees
}
