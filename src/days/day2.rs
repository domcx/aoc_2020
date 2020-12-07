pub fn run(data: &str) {
    let lines: Vec<_> = data
        .split("\n")
        .into_iter()
        .filter(|y| !y.is_empty())
        .collect();

    let mut count1 = 0;
    let mut count2 = 0;
    for line in lines {
        let words: Vec<_> = line.split(" ").collect();

        let range: Vec<_> = words[0].split("-").collect();
        let n1 = range[0].parse::<usize>().unwrap();
        let n2 = range[1].parse::<usize>().unwrap();

        let letter = &words[1][..1];

        let matches = words[2].matches(letter).count();

        if matches >= n1 && matches <= n2 {
            count1 += 1;
        }

        let p1 = &words[2][n1 - 1..n1] == letter;
        let p2 = &words[2][n2 - 1..n2] == letter;

        if (p1 && !p2) || (!p1 && p2) {
            count2 += 1;
        }
    }

    println!("{} {}", count1, count2);
}
