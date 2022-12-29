use std::fs;

fn main() {
    let data = fs::read_to_string("/Users/owchoongseong/personal/adventofcode2022/day1/data.txt")
        .expect("Cannot find data.txt");

    let mut elves_total_calories = Vec::new();

    let mut total_calories_carried = 0;
    let mut food = data.split("\n").peekable();
    while let Some(f) = food.next() {
        if !f.is_empty() {
            let calories: u32 = f.parse().unwrap();
            total_calories_carried += calories;
        }

        if f.is_empty() || food.peek().is_none() {
            elves_total_calories.push(total_calories_carried);
            total_calories_carried = 0;
        }
    }

    let most_calories = elves_total_calories.iter().max().unwrap();
    println!("Highest amount of calories carried is {}", most_calories);

    elves_total_calories.sort_by(|a, b| b.cmp(a));
    let top3_calories: u32 = elves_total_calories.iter().take(3).sum();
    println!(
        "Total amount of calories carried by top 3 is {}",
        top3_calories
    );
}
