use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(1))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();

    let line_sets: Vec<_> = input.split("\n\n").collect();
    let mut sums: Vec<usize> = line_sets.iter()
        .map(|set| set.lines()
            .map(|l| l.parse::<usize>().unwrap())
            .sum()
        )
        .collect();

    sums.sort_by(|a, b| b.cmp(a));

    println!("{}", sums[0]);
    println!("{}", sums[0..3].iter().sum::<usize>());
}
