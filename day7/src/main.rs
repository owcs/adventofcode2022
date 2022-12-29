use std::{
    cell::RefCell,
    fs,
    io::{BufRead, BufReader},
    path::Path,
    rc::Rc,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_directory_size() {
        let terminal_output = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];
        let directory_sizes = parse_terminal_output(&terminal_output);
        assert_eq!(
            get_directory_size(directory_sizes),
            vec![
                ("/".to_string(), 48381165),
                ("d".to_string(), 24933642),
                ("a".to_string(), 94853),
                ("e".to_string(), 584)
            ]
        );
    }

    #[test]
    fn test_get_result_1() {
        let terminal_output = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];
        let directory_sizes = parse_terminal_output(&terminal_output);
        let all = get_directory_size(directory_sizes);
        assert_eq!(
            get_result_1(&all),
            95437
        );
    }

    #[test]
    fn test_get_result_2() {
        let terminal_output = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];
        let directory_sizes = parse_terminal_output(&terminal_output);
        let all = get_directory_size(directory_sizes);
        assert_eq!(get_result_2(&all), ("d".to_string(), 24933642));
    }
}

struct Directory {
    name: String,
    total_size: usize,
    children: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    pub fn new(name: String) -> Directory {
        return Directory {
            name,
            total_size: 0,
            children: vec![],
            parent: None,
        };
    }

    pub fn add_sub_dir(&mut self, new_node: Rc<RefCell<Directory>>) {
        self.children.push(new_node);
    }

    pub fn print(&self, level: usize) {
        let dir_name = &self.name;
        let total_size = &self.total_size;
        let tab = "\t".repeat(level);
        println!("{tab}{dir_name} = {total_size}");

        let next_level = level + 1;
        for dir in &self.children {
            dir.borrow().print(next_level);
        }
    }

    pub fn get_size_recursive(&self) -> usize {
        let mut total_size = self.total_size;
        for dir in &self.children {
            total_size += dir.borrow().get_size_recursive();
        }
        total_size
    }
}

static CMD_CD: &str = "$ cd";
static CMD_LS: &str = "$ ls";
static DIR_PREV: &str = "..";
static DIR_ROOT: &str = "/";
static LS_DIR: &str = "dir";

fn parse_terminal_output(lines: &Vec<String>) -> Rc<RefCell<Directory>> {
    let root = Rc::new(RefCell::new(Directory::new(DIR_ROOT.to_string())));
    let mut current = Rc::clone(&root);
    let mut is_directory_result = false;

    for line in lines {
        if line.starts_with(CMD_CD) {
            let cmd_arg = line[CMD_CD.len()..line.len()].trim();
            if cmd_arg == DIR_PREV {
                let current_clone = Rc::clone(&current);
                current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
            } else if cmd_arg != DIR_ROOT {
                let sub_dir = Rc::clone(
                    current
                        .borrow()
                        .children
                        .iter()
                        .find(|d| d.borrow().name == cmd_arg)
                        .unwrap(),
                );
                current = sub_dir;
            }
            is_directory_result = false;
        } else if line.starts_with(CMD_LS) {
            is_directory_result = true;
        } else if is_directory_result {
            if line.starts_with(LS_DIR) {
                let dir_name = line[LS_DIR.len()..line.len()].trim().clone().to_string();
                let child_dir = Rc::new(RefCell::new(Directory::new(dir_name)));
                child_dir.borrow_mut().parent = Some(Rc::clone(&current));
                current.borrow_mut().add_sub_dir(child_dir);
            } else {
                let (size_str, _filename) = line.split_once(' ').unwrap();
                let size: usize = size_str.parse().unwrap();
                current.borrow_mut().total_size += size;
            }
        }
    }

    // root.borrow().print(0);
    root
}

fn get_directory_size(directory_sizes: Rc<RefCell<Directory>>) -> Vec<(String, usize)> {
    let mut result = vec![];
    let mut next: Vec<Rc<RefCell<Directory>>> = vec![directory_sizes];

    while let Some(dir) = next.pop() {
        let value = dir.borrow();
        let dir_name = value.name.clone();
        let size = value.get_size_recursive();
        result.push((dir_name, size));

        for child in &value.children {
            let child_clone = Rc::clone(&child);
            next.push(child_clone);
        }
    }

    result
}

fn get_result_1(all: &Vec<(String, usize)>) -> usize {
    let mut total = 0;

    for e in all {
        if e.1 <= 100_000 {
            total += e.1;
        }
    }

    total
}

fn get_result_2(all: &Vec<(String, usize)>) -> (String, usize) {
    let min: usize = 30_000_000;
    let free = 70_000_000 - all[0].1;

    let mut result = vec![];
    
    for e in all {
        if e.1 + free >= min {
            result.push((e.0.clone(), e.1));
        }
    }

    result.sort_by(|a, b| a.1.cmp(&b.1));
    let smallest = result.first().unwrap();
    
    (smallest.0.clone(), smallest.1)
}

fn get_file_content(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let content = get_file_content("/Users/owchoongseong/personal/adventofcode2022/day7/data.txt");
    let directory_sizes = parse_terminal_output(&content);
    let all = get_directory_size(directory_sizes);
    let result_1 = get_result_1(&all);
    let result_2 = get_result_2(&all);
    println!("result_1: {result_1} result_2: {:?}", result_2);
}
