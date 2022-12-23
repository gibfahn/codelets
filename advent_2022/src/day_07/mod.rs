use std::collections::HashMap;

const INPUT: &str = include_str!("./input");

const TOTAL_SPACE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;

#[derive(Debug, Default)]
struct File {
    size: usize,
    is_dir: bool,
}

#[derive(Debug, Default)]
struct FileSystem {
    files: Vec<File>,
    path_map: HashMap<String, usize>,
}

impl FileSystem {
    fn from_str(input: &str) -> Self {
        let mut filesystem = FileSystem::default();
        let mut inode_counter = 0;
        let mut current_path = Vec::new();

        filesystem.files.push(File {
            size: 0,
            is_dir: true,
        });
        filesystem.path_map.insert(String::from("/"), inode_counter);
        inode_counter += 1;

        for line in input.lines() {
            if line.starts_with("$ ") {
                if let Some(dir) = line.strip_prefix("$ cd ") {
                    if dir == "/" {
                        current_path.clear();
                    } else if dir == ".." {
                        let _ = current_path.pop();
                    } else {
                        current_path.push(dir);
                    }
                }
            } else if let Some(dir) = line.strip_prefix("dir ") {
                let file_path = make_path(&current_path, Some(dir));

                filesystem.files.push(File {
                    size: 0,
                    is_dir: true,
                });
                filesystem.path_map.insert(file_path, inode_counter);

                inode_counter += 1;
            } else {
                let (size, filename) = line.split_once(' ').unwrap();
                let size: usize = size.parse().unwrap();
                let file_path = make_path(&current_path, Some(filename));

                filesystem.files.push(File {
                    size,
                    is_dir: false,
                });
                filesystem.path_map.insert(file_path, inode_counter);

                filesystem.inc_parent_sizes(current_path.clone(), size);

                inode_counter += 1;
            }
        }

        filesystem
    }

    fn inc_parent_sizes(&mut self, mut current_path: Vec<&str>, size: usize) {
        loop {
            let path = make_path(&current_path, None);
            let inode = *self.path_map.get(&path).unwrap();
            self.files[inode].size += size;

            if current_path.is_empty() {
                break;
            }
            let _ = current_path.pop();
        }
    }
}

fn make_path(current_path: &[&str], filename: Option<&str>) -> String {
    let mut path = format!("/{}", current_path.join("/"));
    if let Some(filename) = filename {
        if !path.ends_with('/') {
            path.push('/');
        }
        path.push_str(filename);
    }
    path
}

pub fn first() -> String {
    let filesystem = FileSystem::from_str(INPUT);
    find_less_than_100k(filesystem)
}

fn find_less_than_100k(filesystem: FileSystem) -> String {
    filesystem
        .files
        .iter()
        .filter(|f| f.is_dir && f.size < 100_000)
        .map(|f| f.size)
        .sum::<usize>()
        .to_string()
}

pub fn second() -> String {
    let filesystem = FileSystem::from_str(INPUT);
    let space_used = filesystem.files[*filesystem.path_map.get("/").unwrap()].size;
    let space_to_delete = space_used - (TOTAL_SPACE - NEEDED_SPACE);
    filesystem
        .files
        .iter()
        .filter(|f| f.is_dir)
        .map(|f| f.size)
        .filter(|size| size > &space_to_delete)
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(
            dbg!(first()),
            advent_of_code::solve(2022, 7, 1, INPUT).unwrap()
        );
        assert_eq!(
            dbg!(second()),
            advent_of_code::solve(2022, 7, 2, INPUT).unwrap()
        );
    }

    #[test]
    fn examples() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(
            dbg!(find_less_than_100k(FileSystem::from_str(input))),
            "95437"
        );
    }
}
