pub fn transpose(input: &str) -> Vec<String> {
    let lines: Vec<&str> = input.lines().collect();

    let mut columns: Vec<String> = vec![String::new(); lines.first().map(|x| x.len()).unwrap()];

    lines.iter().for_each(|l| {
        for (i, c) in l.chars().enumerate() {
            columns[i].push(c);
        }
    });

    columns
}
