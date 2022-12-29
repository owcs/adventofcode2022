use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

#[test]
fn test_get_compartments() {
    assert_eq!(
        get_compartments("vJrwpWtwJgWrhcsFMMfFFhFp"),
        ("vJrwpWtwJgWr", "hcsFMMfFFhFp")
    );
    assert_eq!(
        get_compartments("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")
    );
    assert_eq!(
        get_compartments("PmmdzqPrVvPwwTWBwg"),
        ("PmmdzqPrV", "vPwwTWBwg")
    );
}

fn get_compartments(rucksack: &str) -> (&str, &str) {
    return rucksack.split_at(rucksack.len() / 2);
}

#[test]
fn test_find_duplicate_in_rucksack() {
    assert_eq!(
        find_duplicate_in_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap(),
        'p'
    );
    assert_eq!(
        find_duplicate_in_rucksack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap(),
        'L'
    );
    assert_eq!(
        find_duplicate_in_rucksack("PmmdzqPrVvPwwTWBwg").unwrap(),
        'P'
    );
    assert_eq!(
        find_duplicate_in_rucksack("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap(),
        'v'
    );
    assert_eq!(find_duplicate_in_rucksack("ttgJtRGJQctTZtZT").unwrap(), 't');
    assert_eq!(
        find_duplicate_in_rucksack("CrZsJsPPZsGzwwsLwLmpwMDw").unwrap(),
        's'
    );
}

fn find_duplicate_in_rucksack(rucksack: &str) -> Option<char> {
    let (first, second) = get_compartments(rucksack);
    for ch in first.chars() {
        if second.find(ch) != None {
            return Some(ch);
        }
    }
    return None;
}

#[test]
fn test_get_priority() {
    assert_eq!(get_priority('p'), 16);
    assert_eq!(get_priority('L'), 38);
    assert_eq!(get_priority('P'), 42);
    assert_eq!(get_priority('v'), 22);
    assert_eq!(get_priority('t'), 20);
    assert_eq!(get_priority('s'), 19);
}

fn get_priority(item: char) -> u32 {
    match item {
        'a'..='z' => return (item as u32) - 96,
        'A'..='Z' => return (item as u32) - 38,
        _ => todo!("not implemented"),
    }
}

#[test]
fn test_get_result() {
    let input = vec![
        String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
        String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        String::from("PmmdzqPrVvPwwTWBwg"),
        String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
        String::from("ttgJtRGJQctTZtZT"),
        String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
    ];
    assert_eq!(get_result(&input), 157);
}

fn get_result(input: &Vec<String>) -> u32 {
    return input
        .iter()
        .map(|i| find_duplicate_in_rucksack(i).unwrap())
        .map(|i| get_priority(i))
        .sum();
}

#[test]
fn test_get_badge() {
    assert_eq!(
        get_badge(
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        )
        .unwrap(),
        'r'
    );
    assert_eq!(
        get_badge(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        )
        .unwrap(),
        'Z'
    )
}

fn get_badge(i1: &str, i2: &str, i3: &str) -> Option<char> {
    for ch in i1.chars() {
        if i2.find(ch) != None && i3.find(ch) != None {
            return Some(ch);
        }
    }
    None
}

#[test]
fn test_get_group_result() {
    let input = vec![
        String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
        String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        String::from("PmmdzqPrVvPwwTWBwg"),
        String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
        String::from("ttgJtRGJQctTZtZT"),
        String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
    ];
    assert_eq!(get_group_result(&input), 70);
}

fn get_group_result(input: &Vec<String>) -> u32 {
    input
        .chunks(3)
        .map(|c| get_badge(&c[0], &c[1], &c[2]).unwrap())
        .map(|i| get_priority(i))
        .sum()
}

fn get_file_content(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let content = get_file_content("/Users/owchoongseong/personal/adventofcode2022/day3/data.txt");
    let result = get_result(&content);
    let group_result = get_group_result(&content);
    println!("result: {result}; group result: {group_result}");
}
