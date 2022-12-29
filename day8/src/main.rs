use std::{
    collections::HashSet,
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

struct Map {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(input: &Vec<String>) -> Map {
        let width = input[0].len();
        let height = input.len();
        let mut data = Vec::with_capacity(width * height);

        for line in input {
            for ch in line.chars() {
                data.push(ch.to_digit(10).unwrap());
            }
        }

        Map {
            data,
            width,
            height,
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            let start = y * self.width;
            let end = start + self.width;
            println!("{:?}", &self.data[start..end]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_1() {
        let map_data = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let map = Map::new(&map_data);

        assert_eq!(get_result_1(&map), 21);
    }

    #[test]
    fn test_get_scenic_score() {
        let map_data = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];
        let map = Map::new(&map_data);

        assert_eq!(get_scenic_score(&map, 2, 1), 4);
        assert_eq!(get_scenic_score(&map, 2, 3), 8);
    }

    #[test]
    fn test_get_result_2() {
        let map_data = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let map = Map::new(&map_data);

        assert_eq!(get_result_2(&map), 8);
    }
}

fn get_result_1(map: &Map) -> usize {
    let mut seen: HashSet<usize> = HashSet::new();

    // left to right
    for y in 0..map.height {
        let mut h: i32 = -1;
        for x in 0..map.width {
            let idx: usize = get_index(map, x, y);
            let mh: i32 = get_height_from_map(map, idx);
            if mh > h {
                seen.insert(idx);
                h = mh;
            }
        }
    }

    // right to left
    for y in 0..map.height {
        let mut h: i32 = -1;
        for x in (0..map.width).rev() {
            let idx: usize = get_index(map, x, y);
            let mh: i32 = get_height_from_map(map, idx);
            if mh > h {
                seen.insert(idx);
                h = mh;
            }
        }
    }

    // top to bottom
    for x in 0..map.width {
        let mut h: i32 = -1;
        for y in 0..map.height {
            let idx: usize = get_index(map, x, y);
            let mh: i32 = get_height_from_map(map, idx);
            if mh > h {
                seen.insert(idx);
                h = mh;
            }
        }
    }

    // bottom to top
    for x in 0..map.width {
        let mut h: i32 = -1;
        for y in (0..map.height).rev() {
            let idx: usize = get_index(map, x, y);
            let mh: i32 = get_height_from_map(map, idx);
            if mh > h {
                seen.insert(idx);
                h = mh;
            }
        }
    }

    seen.len()
}

fn get_index(map: &Map, x: usize, y: usize) -> usize {
    return (y * map.height + x).try_into().unwrap();
}

fn get_height_from_map(map: &Map, idx: usize) -> i32 {
    return map.data[idx].try_into().unwrap();
}

fn get_scenic_score(map: &Map, pos_x: usize, pos_y: usize) -> u32 {
    // up
    let h = get_height_from_map(map, get_index(map, pos_x, pos_y));
    let mut up_seen = 0;
    for y in (0..pos_y).rev() {
        let mh: i32 = get_height_from_map(map, get_index(map, pos_x, y));
        match mh {
            mh if mh < h => {
                up_seen += 1;
            }
            mh if mh >= h => {
                up_seen += 1;
                break;
            }
            _ => break,
        }
    }

    // left
    let mut left_seen = 0;
    for x in (0..pos_x).rev() {
        let mh: i32 = get_height_from_map(map, get_index(map, x, pos_y));
        match mh {
            mh if mh < h => {
                left_seen += 1;
            }
            mh if mh >= h => {
                left_seen += 1;
                break;
            }
            _ => break,
        }
    }

    // right
    let mut right_seen = 0;
    for x in (pos_x + 1)..map.width {
        let mh: i32 = get_height_from_map(map, get_index(map, x, pos_y));
        match mh {
            mh if mh < h => {
                right_seen += 1;
            }
            mh if mh >= h => {
                right_seen += 1;
                break;
            }
            _ => break,
        }
    }

    // down
    let mut down_seen = 0;
    for y in (pos_y + 1)..map.height {
        let mh: i32 = get_height_from_map(map, get_index(map, pos_x, y));
        match mh {
            mh if mh < h => {
                down_seen += 1;
            }
            mh if mh >= h => {
                down_seen += 1;
                break;
            }
            _ => break,
        }
    }

    up_seen * down_seen * left_seen * right_seen
}

fn get_result_2(map: &Map) -> u32 {
    let mut best = 0;
    for y in 1..(map.height - 1) {
        for x in 1..(map.width - 1) {
            let score = get_scenic_score(map, x, y);
            if score > best {
                best = score;
            }
        }
    }
    best
}

fn get_file_content(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let content = get_file_content("/Users/owchoongseong/personal/adventofcode2022/day8/data.txt");
    let map = Map::new(&content);
    let result_1 = get_result_1(&map);
    let result_2 = get_result_2(&map);
    println!("result_1: {result_1}; result_2: {result_2}");
}
