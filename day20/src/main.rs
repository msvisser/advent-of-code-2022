use aocf::Aoc;

fn mix(numbers: &[isize], index_list: &mut Vec<usize>) {
    for i in 0..numbers.len() {
        let offset = numbers[i].rem_euclid(numbers.len() as isize - 1);
        let pos = index_list.iter().position(|&v| v == i).unwrap() as isize;

        for m in 0..offset {
            let from = (pos + m) % numbers.len() as isize;
            let to = (pos + m + 1) % numbers.len() as isize;
            index_list.swap(from as usize, to as usize);
        }
    }
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2022))
        .day(Some(20))
        .init()
        .unwrap();

    // Get input data (don't force)
    let input = aoc.get_input(false).unwrap();
    // let input = "1\n2\n-3\n3\n-2\n0\n4";

    let mut numbers = Vec::new();
    for line in input.lines() {
        let num = line.parse::<isize>().unwrap();
        numbers.push(num);
    }

    {
        let mut index_list: Vec<usize> = (0..numbers.len()).collect();
        mix(&numbers, &mut index_list);

        let actual_list: Vec<_> = index_list.iter().map(|&ind| numbers[ind]).collect();
        let zero_pos = actual_list.iter().position(|&v| v == 0).unwrap();
        let pos1 = (zero_pos + 1000) % numbers.len();
        let pos2 = (zero_pos + 2000) % numbers.len();
        let pos3 = (zero_pos + 3000) % numbers.len();
        println!("{}", actual_list[pos1] + actual_list[pos2] + actual_list[pos3]);
    }

    {
        let part2_numbers: Vec<_> = numbers.iter().map(|&n| n * 811589153).collect();
        let mut index_list: Vec<usize> = (0..numbers.len()).collect();
        for _ in 0..10 {
            mix(&part2_numbers, &mut index_list);
        }

        let actual_list: Vec<_> = index_list.iter().map(|&ind| part2_numbers[ind]).collect();
        let zero_pos = actual_list.iter().position(|&v| v == 0).unwrap();
        let pos1 = (zero_pos + 1000) % numbers.len();
        let pos2 = (zero_pos + 2000) % numbers.len();
        let pos3 = (zero_pos + 3000) % numbers.len();
        println!("{}", actual_list[pos1] + actual_list[pos2] + actual_list[pos3]);
    }
}
