use std::{
    collections::{BTreeMap, HashMap},
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_9000() {
        let stack = HashMap::from([
            ("1".to_string(), vec!['Z', 'N']),
            ("2".to_string(), vec!['M', 'C', 'D']),
            ("3".to_string(), vec!['P']),
        ]);
        let input: Vec<String> = vec![
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];
        assert_eq!(get_result_9000(&stack, &input), "CMZ".to_string())
    }

    #[test]
    fn test_get_result_9001() {
        let stack = HashMap::from([
            ("1".to_string(), vec!['Z', 'N']),
            ("2".to_string(), vec!['M', 'C', 'D']),
            ("3".to_string(), vec!['P']),
        ]);
        let input: Vec<String> = vec![
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];
        assert_eq!(get_result_9001(&stack, &input), "MCD".to_string())
    }
}

fn get_result_9000(original_stack: &HashMap<String, Vec<char>>, input: &Vec<String>) -> String {
    let mut stack: BTreeMap<&str, Vec<char>> = BTreeMap::new();
    for (s, v) in original_stack {
        stack.insert(s, v.to_vec());
    }

    for line in input {
        let step: Vec<&str> = line.split(' ').collect();
        let amount = i32::from_str_radix(step[1], 10).expect("unable to parse number");
        let src = step[3];
        let dest = step[5];

        for _i in 0..amount {
            let item = stack.get_mut(src).unwrap().pop().unwrap();
            stack.get_mut(dest).unwrap().push(item);
        }
    }

    let mut result = String::new();
    for (k, v) in stack {
        result.push(v.last().unwrap().clone());
    }

    result
}

fn get_result_9001(original_stack: &HashMap<String, Vec<char>>, input: &Vec<String>) -> String {
    let mut stack: BTreeMap<&str, Vec<char>> = BTreeMap::new();
    for (s, v) in original_stack {
        stack.insert(s, v.to_vec());
    }

    for line in input {
        let step: Vec<&str> = line.split(' ').collect();
        let amount = i32::from_str_radix(step[1], 10).expect("unable to parse number") as usize;
        let src_stack = stack.get_mut(step[3]).unwrap();

        let mut items = src_stack.split_off(src_stack.len() - amount);
        stack.get_mut(step[5]).unwrap().append(&mut items);
    }

    let mut result = String::new();
    for (k, v) in stack {
        result.push(v.last().unwrap().clone());
    }

    result
}

fn get_file_content(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let content = get_file_content("/Users/owchoongseong/personal/adventofcode2022/day5/data.txt");

    let stack = HashMap::from([
        ("1".to_string(), vec!['R', 'G', 'J', 'B', 'T', 'V', 'Z']),
        ("2".to_string(), vec!['J', 'R', 'V', 'L']),
        ("3".to_string(), vec!['S', 'Q', 'F']),
        (
            "4".to_string(),
            vec!['Z', 'H', 'N', 'L', 'F', 'V', 'Q', 'G'],
        ),
        (
            "5".to_string(),
            vec!['R', 'Q', 'T', 'J', 'C', 'S', 'M', 'W'],
        ),
        ("6".to_string(), vec!['S', 'W', 'T', 'C', 'H', 'F']),
        ("7".to_string(), vec!['D', 'Z', 'C', 'V', 'F', 'N', 'J']),
        (
            "8".to_string(),
            vec!['L', 'G', 'Z', 'D', 'W', 'R', 'F', 'Q'],
        ),
        ("9".to_string(), vec!['J', 'B', 'W', 'V', 'P']),
    ]);

    let result_9000 = get_result_9000(&stack, &content);
    let result_9001 = get_result_9001(&stack, &content);
    println!("result_9000: {result_9000}; result_9001: {result_9001}");
}
