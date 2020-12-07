type PointRange = (u32, u32);

fn partition(upper: bool, range: PointRange) -> PointRange {
    let middle = range.0 + ((range.1 - range.0) / 2);
    if upper {
        (middle + 1, range.1)
    } else {
        (range.0, middle)
    }
}

fn read_line(run: &str) -> (u32, u32, u32) {
    let mut columns = (0, 7);
    let mut row = (0, 127);

    for char in run.chars() {
        if char == 'L' || char == 'R' {
            let upper = char == 'R';
            columns = partition(upper, columns);
        } else {
            let upper = char == 'B';
            row = partition(upper, row);
        }
    }

    (row.0, columns.0, (row.0 * 8) + columns.0)
}

pub fn run(data: &str) {
    let mut all: Vec<_> = data
        .split("\n")
        .into_iter()
        .filter(|y| !y.is_empty())
        .map(|y| read_line(y))
        .collect();

    all.sort_by_key(|y| y.2);

    println!("high {:?}", all.last().unwrap());

    for i in 1..all.len() - 1 {
        if all[i + 1].2 - all[i].2 > 1 {
            println!("my seat {:?}", all[i].2 + 1);
        }
    }
}
