/* Bootstrapped */

mod filesystem;

use crate::utils::*;
use filesystem::*;
use std::rc::Rc;

fn visit(dir: &Rc<DirectoryContents>, values: &mut Vec<i64>) {
    let dir_size = dir.size.borrow();
    let parent_name = if let Some(parent) = dir.get_parent() {
        parent.name.clone()
    } else {
        "None".to_string()
    };
    println!("Dir: {} (Parent: {}) Size: {}", dir.name, parent_name, dir_size);
    if *dir_size <= 100_000 {
        values.push(*dir_size)
    }

    for child in dir.children.borrow().values() {
        visit(child, values);
    }
}

pub struct Solution {
    root: DirRef,
    current_dir: DirRef
}

impl Solution {
    pub fn new() -> Self {
        let root = Directory::new("/");
        let mut sol = Self {
            root: root.get_ref(),
            current_dir: root.get_ref()
        };
        sol.process_input("day07/input.txt");
        sol
    }

    fn process_cmd(&mut self, cmd: &str) {
        let breakdown: Vec<&str> = cmd.split(" ").collect();
        let start = breakdown[0];
        let file = breakdown[1];

        match start {
            "dir" => {
                let parent = Rc::downgrade(&self.current_dir);
                self.current_dir.add_directory(file, parent);
            },
            "cd" => {
                self.current_dir = self.current_dir.get_child(file);
            }
            _ => {
                let size = start.parse::<i64>().unwrap();
                self.current_dir.add_file(file, size);
            }
        }
    }
}

impl Solve for Solution {
    fn process_input(&mut self, path: &str) {
        let mut raw = read_file(path);
        raw = raw.replace("$ ", "");
        let mut cmd_list: Vec<&str> = raw.split("\n").collect();
        let cmds: Vec<&str> = cmd_list.drain(2..).collect();

        for cmd in cmds {
            match cmd {
                "cd .." => {
                    self.current_dir = self.current_dir.get_parent().unwrap();
                },
                "ls" => {},
                _ => self.process_cmd(cmd)
            }
        }
    }

    fn part1(&mut self) {
        let mut total: Vec<i64> = vec![];
        self.root.total_size();
        visit(&self.root, &mut total);
    }

    fn part2(&mut self) {
        
    }
}