use std::{collections::HashMap, path::PathBuf};

pub fn part_one(input: &str) -> Option<u32> {
    let mut path = PathBuf::new();
    let mut dir: HashMap<PathBuf, u32> = HashMap::new();
    input.lines().for_each(|ln| {
        if ln.starts_with("$ cd") {
            // CD
            match ln.trim_start_matches("$ cd ") {
                ".." => assert_eq!(path.pop(), true),
                sub_dir => path.push(sub_dir),
            }
        } else if ln.starts_with("$ ls") {
            // LS
        } else if ln.starts_with("dir") {
            // DIR
        } else {
            // FILE
            let size: u32 = ln.split_whitespace().next().unwrap().parse().unwrap();
            let mut temp_path = path.clone();

            if let Some(entry) = dir.get_mut(&temp_path) {
                *entry += size;
            } else {
                dir.insert(temp_path.clone(), size);
            }
            while temp_path.pop() {
                if let Some(entry) = dir.get_mut(&temp_path) {
                    *entry += size;
                } else {
                    dir.insert(temp_path.clone(), size);
                }
            }
        }
    });
    Some(dir.values().filter(|&&x| x <= 100_000).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut path = PathBuf::new();
    let mut dir: HashMap<PathBuf, u32> = HashMap::new();
    input.lines().for_each(|ln| {
        if ln.starts_with("$ cd") {
            // CD
            match ln.trim_start_matches("$ cd ") {
                ".." => assert_eq!(path.pop(), true),
                sub_dir => path.push(sub_dir),
            }
        } else if ln.starts_with("$ ls") {
            // LS
        } else if ln.starts_with("dir") {
            // DIR
        } else {
            // FILE
            let size: u32 = ln.split_whitespace().next().unwrap().parse().unwrap();
            let mut temp_path = path.clone();

            if let Some(entry) = dir.get_mut(&temp_path) {
                *entry += size;
            } else {
                dir.insert(temp_path.clone(), size);
            }
            while temp_path.pop() {
                if let Some(entry) = dir.get_mut(&temp_path) {
                    *entry += size;
                } else {
                    dir.insert(temp_path.clone(), size);
                }
            }
        }
    });
    let mut total_fs = PathBuf::new();
    total_fs.push("/");
    let total_fs = dir.get(&total_fs).unwrap();
    println!("{}", total_fs);
    let required = *total_fs - 40_000_000;
    println!("{}", required);
    let mut dir = dir.values().map(|x| *x).collect::<Vec<u32>>();
    dir.sort();
    Some(*dir.iter().find(|x| **x >= required).unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(
            part_one(
                "$ cd /
$ ls
dir a
$ cd a
$ ls
dir a
2 a.txt
$ cd a
$ ls
99999 a.txt"
            ),
            Some(9999)
        );
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
