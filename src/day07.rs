//use std::{borrow::BorrowMut, cell::RefCell, env::current_exe, fmt, fs, rc::Rc, vec};

//use trees::{tr, Tree};

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 7");
    log::debug!("..or maybe not :-(");
    // let dirs = read_directory_tree();
    //dbg!(&dirs);
    // log::info!("Part 1: {}", find_first_distinct_characters(&input, 4) + 1);
    // log::info!("Part 2: {}", find_first_distinct_characters(&input, 14) + 1);
}

// #[derive(Debug, Clone)]
// struct File {
//     name: String,
//     size: i32,
// }

// type DirectoryHandle = Rc<RefCell<Directory>>;

// #[derive(Debug, Clone)]
// struct Directory {
//     name: String,
//     path: String,
//     files: Vec<File>,
// }

// impl fmt::Debug for Directory {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Directory")
//             .field("name", &self.name)
//             .field("path", &self.path)
//             .field("dirs", &self.dirs)
//             .field("files", &self.files)
//             .finish()
//     }
// }

// fn read_directory_tree() -> Directory {
//     let mut root = Directory {
//         parent: None,
//         name: "".to_owned(),
//         path: "/".to_owned(),
//         dirs: vec![],
//         files: vec![],
//     };
//     let input = fs::read_to_string("./inputs/day07.txt").unwrap();
//     let lines = input.split("\n");
//     let mut current_directory = &mut root;
//     for line in lines {
//         if line.starts_with("$ cd") {
//             let directory_name = &line[5..];
//             match directory_name {
//                 "/" => current_directory = &mut root,
//                 ".." => current_directory = &mut root, //*current_directory.parent.unwrap(),
//                 _ => {
//                     let mut maybe_child = current_directory
//                         .dirs
//                         .iter()
//                         .find(|v| v.name == directory_name);
//                     match maybe_child {
//                         Some(child) => current_directory = &mut child,
//                         None => {
//                             let new_dir = Directory {
//                                 parent: None,
//                                 name: directory_name.to_owned(),
//                                 path: "/".to_owned(),
//                                 dirs: vec![],
//                                 files: vec![],
//                             };
//                             current_directory.dirs.push(new_dir);
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     root
// }

// fn read_directory_tree() -> Tree<Directory> {
//     let input = fs::read_to_string("./inputs/day7.txt").unwrap();
//     let lines = input.split("\n");
//     let mut directory_tree = tr(Directory {
//         name: "".to_owned(),
//         path: "".to_owned(),
//         files: vec![],
//     });
//     let mut current_directory = directory_tree.root_mut();
//     let mut current_path: Vec<&str> = vec![];
//     for line in lines {
//         if line.starts_with("$ cd") {
//             current_path = cd(current_path.clone(), line);
//             current_directory = directory_tree.root_mut();
//             for dir in current_path.iter() {
//                 let maybe_directory = current_directory
//                     .iter()
//                     .find(|it| it.data.name == dir.clone().to_owned());
//                 match maybe_directory {
//                     Some(new_dir) => current_directory = new_dir.borrow_mut(),
//                     None => (),
//                 }
//             }
//             dbg!(&current_directory);
//         } else if line.starts_with("$ ls") {
//         }
//         //dbg!(&line);
//     }
//     directory_tree
// }

// fn cd<'a>(mut current_path: Vec<&'a str>, cd_command: &'a str) -> Vec<&'a str> {
//     let directory_name = &cd_command[5..];
//     match directory_name {
//         "/" => {
//             current_path.clear();
//         }
//         ".." => {
//             current_path.pop();
//         }
//         _ => {
//             current_path.push(directory_name);
//         }
//     }
//     return current_path;
// }
