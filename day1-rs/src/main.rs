fn main() {
    let input = std::fs::read_to_string("../input/day1-test.txt").expect("reading input/...");
    let meals = input
        .split("\n\n")
        .map(|meal| {
            meal
                .lines()
                .map(|item| item.parse::<u64>().ok().unwrap())
                .collect::<Vec<_>>()
    })
        .collect::<Vec<_>>();
    println!("{meals:?}");
    let mut meal_sums = meals
        .iter()
        .map(|meal| meal.iter().sum::<u64>())
        .collect::<Vec<u64>>();
    println!("{meal_sums:?}");
    let part1 = meal_sums.iter().reduce(|a, b| if a > b { a } else  { b }).unwrap();
    println!("{part1:?}");
    meal_sums.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let part2 = meal_sums.into_iter().take(3).reduce(|a, b| a + b).unwrap();
    println!("{part2:?}");
}
