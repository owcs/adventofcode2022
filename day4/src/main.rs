use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_range() {
        assert_eq!(contains_range("2-4", "6-8"), false);
        assert_eq!(contains_range("2-3", "4-5"), false);
        assert_eq!(contains_range("5-7", "7-9"), false);
        assert_eq!(contains_range("2-8", "3-7"), true);
        assert_eq!(contains_range("6-6", "4-6"), true);
        assert_eq!(contains_range("2-6", "4-8"), false);
    }

    #[test]
    fn test_overlap_range() {
        assert_eq!(overlap_range("2-4", "6-8"), false);
        assert_eq!(overlap_range("2-3", "4-5"), false);
        assert_eq!(overlap_range("5-7", "7-9"), true);
        assert_eq!(overlap_range("2-8", "3-7"), true);
        assert_eq!(overlap_range("6-6", "4-6"), true);
        assert_eq!(overlap_range("2-6", "4-8"), true);
    }

    #[test]
    fn test_get_result_range() {
        let input = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
            "50-80,58-79".to_string(),
            "80-90,80-96".to_string(),
            "7-63,5-63".to_string(),
        ];
        assert_eq!(get_result_range(&input), 5)
    }

    #[test]
    fn test_get_result_overlap() {
        let input = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        assert_eq!(get_result_overlap(&input), 4)
    }
}

fn overlap_range(range1_str: &str, range2_str: &str) -> bool {
    let (start_1, end_1) = range1_str
        .split_once('-')
        .map(|s| {
            (
                i32::from_str_radix(s.0, 10).expect("cannot parse number"),
                i32::from_str_radix(s.1, 10).expect("cannot parse nunber"),
            )
        })
        .unwrap();
    let (start_2, end_2) = range2_str
        .split_once('-')
        .map(|s| {
            (
                i32::from_str_radix(s.0, 10).expect("cannot parse number"),
                i32::from_str_radix(s.1, 10).expect("cannot parse nunber"),
            )
        })
        .unwrap();

    for i in start_1..=end_1 {
        if start_2 <= i && i <= end_2 {
            return true
        }
    }

    false
}

fn contains_range(range1: &str, range2: &str) -> bool {
    let (start_1, end_1) = range1
        .split_once('-')
        .map(|s| {
            (
                i32::from_str_radix(s.0, 10).expect("cannot parse number"),
                i32::from_str_radix(s.1, 10).expect("cannot parse nunber"),
            )
        })
        .unwrap();
    let (start_2, end_2) = range2
        .split_once('-')
        .map(|s| {
            (
                i32::from_str_radix(s.0, 10).expect("cannot parse number"),
                i32::from_str_radix(s.1, 10).expect("cannot parse nunber"),
            )
        })
        .unwrap();

    (start_2 <= start_1 && end_1 <= end_2) || (start_1 <= start_2 && end_2 <= end_1)
}

fn get_result_overlap(input: &Vec<String>) -> i32 {
    let mut count = 0;
    for line in input {
        let (range1, range2) = line.split_once(',').unwrap();
        if overlap_range(range1, range2) {
            count += 1
        } 
    }

    count
}

fn get_result_range(input: &Vec<String>) -> i32 {
    let mut count = 0;
    for line in input {
        let (range1, range2) = line.split_once(',').unwrap();
        if contains_range(range1, range2) {
            count += 1
        } 
    }

    count
}

fn get_file_content(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let content = get_file_content("/Users/owchoongseong/personal/adventofcode2022/day4/data.txt");
    let result_range = get_result_range(&content);
    let result_overlap = get_result_overlap(&content);
    println!("result (range): {result_range}, result (overlap): {result_overlap}");
}
