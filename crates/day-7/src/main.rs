use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Default)]
struct Dir<'t> {
    dirs: HashMap<&'t str, Rc<RefCell<Dir<'t>>>>,
    files: HashMap<&'t str, u32>,
    parent: Option<Rc<RefCell<Dir<'t>>>>,
}

impl<'t> Dir<'t> {
    pub fn size(&self) -> u32 {
        let mut size = self.files.values().sum();

        for dir in self.dirs.values() {
            size += dir.borrow().size();
        }

        size
    }
}

fn parse_tree<'t>(data: &'t str) -> Dir<'t> {
    let root = Rc::new(RefCell::new(Dir::default()));
    let mut current_dir = Rc::clone(&root);
    let mut is_listing = false;

    for line in data.trim().lines() {
        if line.is_empty() {
            continue;
        }

        // Commands
        if line.starts_with('$') {
            is_listing = false;

            let mut args = line.split_whitespace();
            args.next(); // Remove $

            match args.next().unwrap() {
                "cd" => match args.next().unwrap() {
                    "/" => {
                        current_dir = Rc::clone(&root);
                    }
                    ".." => {
                        let prev_dir = {
                            let dir = current_dir.borrow();
                            Rc::clone(dir.parent.as_ref().expect("Missing parent dir!"))
                        };

                        current_dir = prev_dir;
                    }
                    to => {
                        let next_dir = {
                            let dir = current_dir.borrow();
                            Rc::clone(dir.dirs.get(to).expect("Missing child dir!"))
                        };

                        current_dir = next_dir;
                    }
                },
                "ls" => {
                    is_listing = true;
                }
                _ => {}
            }

            continue;
        }

        // Listing contents
        if is_listing {
            let mut parts = line.split_whitespace();
            let meta = parts.next().unwrap();
            let name = parts.next().unwrap();
            let mut dir = current_dir.borrow_mut();

            if meta == "dir" {
                dir.dirs.insert(
                    name,
                    Rc::new(RefCell::new(Dir {
                        parent: Some(Rc::clone(&current_dir)),
                        ..Dir::default()
                    })),
                );
            } else {
                dir.files.insert(name, meta.parse().unwrap());
            }
        }
    }

    root.take()
}

fn sum_tree(dir: &Dir) -> u32 {
    let mut sum = 0;
    let size = dir.size();

    if size < 100_000 {
        sum += size;
    }

    for nested_dir in dir.dirs.values() {
        sum += sum_tree(&nested_dir.borrow());
    }

    sum
}

fn sum_tree_closest(dir: &Dir, min_size: u32, current_size: u32) -> u32 {
    let size = dir.size();
    let mut next_size = current_size;

    if size >= min_size && (size < next_size || next_size == 0) {
        next_size = size;
    }

    for nested_dir in dir.dirs.values() {
        next_size = sum_tree_closest(&nested_dir.borrow(), min_size, next_size);
    }

    next_size
}

fn part_1() {
    let data = include_str!("input.txt");
    let time = std::time::Instant::now();
    let tree = parse_tree(data);

    println!("total size = {}", sum_tree(&tree));
    println!("{:?}", time.elapsed());
}

fn part_2() {
    let data = include_str!("input.txt");
    let time = std::time::Instant::now();
    let tree = parse_tree(data);

    let total_space = 70_000_000;
    let used_space = tree.size();
    let free_space = total_space - used_space;
    let find_space = 30_000_000 - free_space;

    println!("total space = {}", total_space);
    println!("used space  = {}", used_space);
    println!("free space  = {}", free_space);
    println!("min delete size = {}", find_space);
    println!("closest size = {}", sum_tree_closest(&tree, find_space, 0));

    println!("{:?}", time.elapsed());
}

fn main() {
    part_1();
    println!("-----");
    part_2();
}
