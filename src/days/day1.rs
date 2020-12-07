pub fn run(data: &str) {
    let lines: Vec<_> = data
        .split("\n")
        .into_iter()
        .filter_map(|y| y.parse::<i32>().ok())
        .collect();

    println!("{}", xy(&lines));
    println!("{}", xyz(&lines));
}

fn xy(all: &[i32]) -> i32 {
    for i in 0..all.len() {
        let x = all[i].clone();
        for y in all {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    -1
}

fn xyz(all: &[i32]) -> i32 {
    for i in 0..all.len() {
        let x = all[i].clone();
        for i2 in 0..all.len() {
            let y = all[i2].clone();
            for i3 in 0..all.len() {
                let z = all[i3].clone();

                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }

    -1
}
