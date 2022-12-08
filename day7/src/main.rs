use std::{borrow::BorrowMut, cell::RefCell, ops::Deref, rc::Rc, thread::current};

use regex::Regex;

#[derive(Debug)]
pub enum Command {
    ChangeDirectory { target: String },
    ListCurrentDirectory { output: Vec<String> },
}

impl Command {
    pub fn from(command: &str, option: Option<&str>) -> Self {
        match command {
            "cd" => Command::ChangeDirectory {
                target: String::from(option.unwrap()),
            },
            "ls" => Command::ListCurrentDirectory { output: Vec::new() },
            _ => panic!("Unsupported command: {}", command),
        }
    }

    pub fn add_output(&mut self, line: &str) {
        match self {
            Command::ListCurrentDirectory { output } => {
                output.push(String::from(line));
            }
            _ => println!(
                "WARN: Attempted to add output to a cd command. Output: {}",
                line
            ),
        }
    }
}

pub struct File {
    name: String,
    size: u32,
    pub parent: Option<Rc<RefCell<File>>>,
    children: Option<Vec<Rc<RefCell<File>>>>,
}

impl File {
    pub fn new_directory(name: &str) -> Self {
        File {
            name: String::from(name),
            size: 0,
            parent: None,
            children: Some(Vec::new()),
        }
    }

    pub fn new_file(name: &str, size: u32) -> Self {
        File {
            name: String::from(name),
            size,
            parent: None,
            children: None,
        }
    }

    pub fn add_child(&mut self, parent: Rc<RefCell<File>>, mut child: File) -> &mut Self {
        child.parent = Some(parent);

        if let Some(children) = &mut self.children {
            children.push(Rc::new(RefCell::new(child)))
        } else {
            println!(
                "WARN: Attempted to add a child to a file. File: {}, Child: {}",
                self.name, child.name
            );
        }

        self
    }

    // returns index of child with given name, or None if no child has that name
    pub fn find_child_by_name(&self, name: &str) -> Option<Rc<RefCell<File>>> {
        if let Some(children) = &self.children {
            children
                .iter()
                .find(|child| child.borrow().name == name)
                .map(|child| child.clone())
        } else {
            None
        }
    }

    pub fn get_size(&self) -> u32 {
        let size_of_children: u32 = if let Some(children) = &self.children {
            children.iter().map(|child| child.borrow().get_size()).sum()
        } else {
            0
        };

        size_of_children + self.size
    }

    pub fn is_directory(&self) -> bool {
        self.children.is_some()
    }
}

fn input_to_commands(input: String) -> Vec<Command> {
    let mut commands = Vec::new();

    let mut current_command_index: usize = 0;

    let command_pattern = Regex::new(r"\$ (?P<command>\w+)\s*(?P<option>.*)").unwrap();

    for line in input.lines() {
        let pattern_matches = command_pattern.captures(line.trim());
        if let Some(captures) = pattern_matches {
            let command = captures.name("command").unwrap().as_str(); // required
            let option = captures.name("option").map(|val| val.as_str()); // optional

            current_command_index = commands.len();

            commands.push(Command::from(command, option));
        } else {
            let current_command = &mut commands[current_command_index];

            current_command.add_output(line);
        }
    }

    commands
}

fn execute_commands(commands: &Vec<Command>) -> Rc<RefCell<File>> {
    let root = Rc::new(RefCell::new(File::new_directory("/")));
    let mut current_file = root.clone();

    for command in commands {
        match command {
            Command::ChangeDirectory { target } if target == "/" => (),
            Command::ChangeDirectory { target } if target == ".." => {
                let parent = current_file.borrow().parent.as_ref().unwrap().clone();

                current_file = parent;
            }
            Command::ChangeDirectory { target } => {
                let new_directory = current_file.borrow().find_child_by_name(target).unwrap();

                current_file = new_directory;
            }
            Command::ListCurrentDirectory { output } => {
                for line in output {
                    let line_split: Vec<&str> = line.trim().split(" ").collect();

                    if let [first, second, ..] = &line_split[..] {
                        if *first == "dir" {
                            current_file
                                .deref()
                                .borrow_mut()
                                .add_child(current_file.clone(), File::new_directory(second));
                        } else {
                            let size: u32 = first.parse().unwrap();
                            current_file
                                .deref()
                                .borrow_mut()
                                .add_child(current_file.clone(), File::new_file(second, size));
                        }
                    } else {
                        panic!("Invalid line: {}", line);
                    }
                }
                // todo: for line in output, get the sizes
            }
        }
    }

    root
}

// for each dir sized < 100_000, sum them
fn find_size_p1(current_node: Rc<RefCell<File>>) -> u32 {
    // only interested in directories:
    if !current_node.borrow().is_directory() {
        return 0;
    }

    let my_size = current_node.borrow().get_size();

    let nested_dir_sizes: u32 = current_node
        .borrow()
        .children
        .as_ref()
        .unwrap()
        .iter()
        .map(|child| find_size_p1(child.clone()))
        .sum();

    if my_size < 100_000 {
        my_size + nested_dir_sizes
    } else {
        nested_dir_sizes
    }
}

// returns a vec of (file_size, file_name)
// only returns directories that are large enough to free enough space
fn find_candidates_to_delete(root: Rc<RefCell<File>>, need_to_delete: u32) -> Vec<(u32, String)> {
    if !root.borrow().is_directory() {
        return Vec::new();
    }

    let my_size = root.borrow().get_size();

    let mut child_candidates: Vec<(u32, String)> = root
        .deref()
        .borrow()
        .children
        .as_ref()
        .unwrap()
        .iter()
        .filter(|child| child.borrow().is_directory())
        .map(|child_dir| find_candidates_to_delete(child_dir.clone(), need_to_delete))
        .collect::<Vec<Vec<(u32, String)>>>()
        .concat();

    let mut to_concat = if my_size >= need_to_delete {
        vec![(my_size, root.borrow().name.to_string())]
    } else {
        Vec::new()
    };

    child_candidates.append(&mut to_concat);

    child_candidates
}

fn find_directory_to_delete(root: Rc<RefCell<File>>) -> (u32, String) {
    const TOTAL_DISK: u32 = 70_000_000;
    const DISK_NEEDED: u32 = 30_000_000;

    let current_used: u32 = root.borrow().get_size();

    let available_disk = TOTAL_DISK - current_used;
    let need_to_delete: u32 = DISK_NEEDED - available_disk;

    let mut candidates = find_candidates_to_delete(root, need_to_delete);

    candidates.sort_by(|(size1, _), (size2, _)| size1.cmp(&size2));

    (candidates[0].0.clone(), candidates[0].1.clone())
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let commands = input_to_commands(input);

    let root = execute_commands(&commands);

    let size = find_size_p1(root.clone());

    println!("sum of < 100_000: {}", size);

    let (size, directory) = find_directory_to_delete(root);

    println!("Should delete: {} ({} bytes)", directory, size);
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = "$ cd /
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

    use crate::*;

    #[test]
    fn test_part1() {
        let commands = input_to_commands(String::from(TEST_INPUT));

        let root = execute_commands(&commands);

        let size = find_size_p1(root.clone());

        println!("sum of < 100_000: {}", size);

        assert_eq!(size, 95437);
    }

    #[test]
    fn test_part2() {
        let commands = input_to_commands(String::from(TEST_INPUT));

        let root = execute_commands(&commands);

        let (size, directory) = find_directory_to_delete(root);

        assert_eq!(directory, "d");
        assert_eq!(size, 24933642);
    }
}
