use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/day7.txt").unwrap();
    println!("Answer: {}", solve(&input));
    println!("Answer: {}", solve_2(&input));
}

struct Dir {
    pub files: HashMap<String, u32>,
    pub sub_dirs: HashMap<String, Dir>,
    pub _name: String,
}

impl Dir {
    fn new(name: String) -> Self {
        Self {
            files: HashMap::new(),
            sub_dirs: HashMap::new(),
            _name: name,
        }
    }
}

fn solve(input: &str) -> u32 {
    let mut root = Dir::new("root".into());

    let mut commands = input.split("\n$ ").skip(1);
    add_contents(&mut root, &mut commands);
    let mut sizes = vec![];
    sum_size(&mut root, &mut sizes);
    sizes.iter().filter(|size| **size < 100000).sum()
}

fn solve_2(input: &str) -> u32 {
    let mut root = Dir::new("root".into());

    let mut commands = input.split("\n$ ").skip(1);
    add_contents(&mut root, &mut commands);
    let mut sizes = vec![];
    sum_size(&mut root, &mut sizes);
    sizes.sort();
    let unused_space = 70000000 - sizes.last().unwrap();
    let size_needed = 30000000 - unused_space;
    *sizes.iter().find(|item| **item > size_needed).unwrap()
}

fn add_contents<'a>(dir: &mut Dir, commands: &mut impl Iterator<Item = &'a str>) {
    while let Some(block) = commands.next() {
        let mut lines = block.lines();
        let cmd = lines.next().unwrap();
        if cmd == "ls" {
            for line in lines {
                if let Some(dir_name) = line.strip_prefix("dir ") {
                    dir.sub_dirs
                        .insert(dir_name.into(), Dir::new(dir_name.into()));
                } else {
                    let (size, file_name) = line.split_once(' ').unwrap();
                    dir.files.insert(file_name.into(), size.parse().unwrap());
                }
            }
        } else {
            let new_dir_name = cmd.strip_prefix("cd ").unwrap();
            if new_dir_name == ".." {
                return;
            }
            add_contents(dir.sub_dirs.get_mut(new_dir_name).unwrap(), commands);
        }
    }
}

fn sum_size(dir: &mut Dir, sum_valid: &mut Vec<u32>) -> u32 {
    let size_files = dir.files.values().sum::<u32>();
    let size_sub_dirs = dir
        .sub_dirs
        .values_mut()
        .map(|dir| sum_size(dir, sum_valid))
        .sum::<u32>();
    println!("Dir: {:?}", &dir._name);
    let tot_size: u32 = dbg!(size_files) + dbg!(size_sub_dirs);

    sum_valid.push(tot_size);
    dbg!(tot_size)
}

#[test]
fn test_example() {
    let input = r#"$ cd /
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
7214296 k"#;
    assert_eq!(solve(input), 95437)
}

#[test]
fn test_example_2() {
    let input = r#"$ cd /
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
7214296 k"#;
    assert_eq!(solve_2(input), 24933642)
}
