// // x AA -(2)-> DD     20 * (30 - 2) = 560
// //   AA -(2)-> BB     13 * (30 - 2) = 364
// //   AA -(3)-> JJ     22 * (30 - 3) = 594
// //   AA -(6)-> HH     22 * (30 - 6) = 528

// // x DD -(3)-> BB     13 * (30 - 5) = 325 ???
// //   DD -(2)-> CC      2 * (30 - 4) = 52
// //   DD -(5)-> HH     22 * (30 - 7) = 506
// //   DD -(4)-> JJ     21 * (30 - 6) = 504

// // x BB -(4)-> JJ     21 * (30 - 9) = 441
// // x JJ -(8)-> HH     22 * (30 - 17) = 286
// // x HH -(4)-> EE     3 * (30 - 21) = 27
// // x EE -(3)-> CC     2 * (30 - 24) = 12

// // 560 + 325 + 441 + 286 + 27 + 12 = 1651

// use regex::Regex;
// use std::collections::{HashMap, LinkedList};
// use std::hash::Hash;
// use std::ops::DivAssign;
// use std::{fs, path, vec};
// use trees::linked::fully::node::Link;

pub fn solve_puzzle() {
    //     log::debug!("-------------");
    //     log::debug!("Solving Day 16");
    //     let valves = parse_input("./inputs/day16_test.txt");
    //     log::info!("Part 1: {}", find_optimal_path(&valves));
}

// fn find_optimal_path(input: &Vec<Valve>) -> i32 {
//     // create a mapping from id to valve
//     let mut valves: HashMap<String, Valve> = HashMap::new();
//     for valve in input {
//         valves.insert(valve.id.clone(), valve.clone());
//     }

//     let optimal_path = find_optimal_path_from(
//         &valves,
//         &Path {
//             preasure_released: 0,
//             time_left: 30,
//             valves: vec!["AA".to_owned()],
//             opened_valves: vec![],
//         },
//     );
//     dbg!(&optimal_path);
//     optimal_path.preasure_released
// }

// fn find_optimal_path2(valves: &HashMap<String, Valve>, current_path: &Path) {
//     let current_valve = current_path
//         .valves
//         .last()
//         .and_then(|id| valves.get(id))
//         .unwrap();
// }

// fn get_next_weighted(valves: &HashMap<String, Valve>, path: &Path) {
//     let start = path.valves.last().and_then(|id| valves.get(id)).unwrap();
//     let mut paths: HashMap<String, Path> = HashMap::new();

//     let mut open: LinkedList<String> = LinkedList::new();
//     let mut closed: LinkedList<String> = LinkedList::new();
//     open.push_back(start.id.clone());
//     while let Some(node) = open.pop_front() {
//         closed.push_back(node.clone());
//         let reachable = valves.get(&node).unwrap().tunnels_to.clone();
//         for it in reachable {
//             let target = valves.get(it).unwrap();

//             if closed.contains(&it) {
//                 continue;
//             }
//             if open.contains(&it) {
//                 let cost = paths.get(it).unwrap().valves.len();
//                 let preasure = (path.time_left - cost - 1) * target.preasure;
//                 // if paths.get(&it).unwrap()
//                 // TODO check new weight
//                 //continue;
//             }
//             open.push_back(it.clone());
//         }
//     }
// }

// fn find_optimal_path_from(valves: &HashMap<String, Valve>, path: &Path) -> Path {
//     let current_valve = path.valves.last().and_then(|id| valves.get(id)).unwrap();
//     // get a vector of all valves reachable from here
//     let next_valves = current_valve
//         .tunnels_to
//         .iter()
//         // .filter(|id| !path.valves.contains(id))
//         .map(|id| valves.get(id).unwrap())
//         .collect::<Vec<&Valve>>();

//     // no time left, nothing more we can do
//     if path.time_left <= 0 {
//         dbg_path(&path);
//         return path.clone();
//     }

//     let mut optimal_path: Path = path.clone();

//     // we can either move to the next valve or open this valve first
//     // test path with opening the valve
//     if current_valve.preasure > 0 && !path.opened_valves.contains(&current_valve.id) {
//         // test path with opening the valve
//         let path_after_opening = path.copy_with_opened_valve(current_valve);
//         if path_after_opening.preasure_released > optimal_path.preasure_released {
//             optimal_path = path_after_opening.clone();
//         }

//         // test path with opening the valve and moving to another valve
//         for next in next_valves.iter() {
//             let path_to_valve_after_opening = path_after_opening.copy_with_new_valve(&next);
//             let optimal_path_with_valve =
//                 find_optimal_path_from(valves, &path_to_valve_after_opening);
//             if optimal_path_with_valve.preasure_released > optimal_path.preasure_released {
//                 optimal_path = optimal_path_with_valve;
//             }
//         }
//     }

//     // test path without opening the valve
//     for next in next_valves {
//         let path_to_valve_after_opening = path.copy_with_new_valve(&next);
//         let optimal_path_with_valve = find_optimal_path_from(valves, &path_to_valve_after_opening);
//         if optimal_path_with_valve.preasure_released > optimal_path.preasure_released {
//             optimal_path = optimal_path_with_valve;
//         }
//     }

//     optimal_path
// }

// fn dbg_path(path: &Path) {
//     let path_str = path
//         .valves
//         .iter()
//         .map(|valve| {
//             if path.opened_valves.contains(valve) {
//                 return valve.to_owned() + "*";
//             }
//             valve.to_owned()
//         })
//         .collect::<Vec<String>>()
//         .join(" -> ");
//     println!(
//         "{}, {} preasure released with {} minutes remaining",
//         path_str, path.preasure_released, path.time_left
//     );
// }

// fn parse_input(file: &str) -> Vec<Valve> {
//     let input = fs::read_to_string(file).unwrap();
//     let regex =
//         Regex::new(r"Valve ([A-Z]+) has flow rate=([0-9]+); tunnel[s]* lead[s]* to valve[s]* (.*)")
//             .expect("Invalid regex");

//     regex
//         .captures_iter(&input)
//         .map(|captures| Valve {
//             id: captures[1].to_owned(),
//             preasure: captures[2].to_string().parse::<i32>().unwrap(),
//             tunnels_to: captures[3]
//                 .split(", ")
//                 .map(|v| v.to_owned())
//                 .collect::<Vec<String>>(),
//         })
//         .collect::<Vec<Valve>>()
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct Path {
//     valves: Vec<String>,
//     opened_valves: Vec<String>,
//     preasure_released: i32,
//     time_left: i32,
// }

// impl Path {
//     fn move_to(&mut self, valves: &Vec<Valve>) {
//         for valve in valves {
//             self.valves.push(valve.id.clone());
//         }
//         let final_valve = valves.last().unwrap();
//         self.opened_valves.push(final_valve.id.clone());
//         self.time_left -= (valves.len() + 1) as i32;
//         self.preasure_released += final_valve.preasure * self.time_left;
//     }

//     fn copy_with_new_valve(&self, valve: &Valve) -> Path {
//         let mut copy = Path {
//             time_left: self.time_left,
//             preasure_released: self.preasure_released,
//             valves: self.valves.clone(),
//             opened_valves: self.opened_valves.clone(),
//         };
//         // subtract 1 minute for moving to the new valve
//         copy.time_left -= 1;
//         copy.valves.push(valve.id.clone());
//         copy
//     }

//     fn copy_with_opened_valve(&self, valve: &Valve) -> Path {
//         let mut copy = Path {
//             time_left: self.time_left,
//             preasure_released: self.preasure_released,
//             valves: self.valves.clone(),
//             opened_valves: self.opened_valves.clone(),
//         };
//         copy.opened_valves.push(valve.id.clone());
//         // subtract 1 minute for opening the valve
//         copy.time_left -= 1;
//         // valve will release preasure beginning in the next minute
//         copy.preasure_released += copy.time_left * valve.preasure;
//         copy
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct Valve {
//     id: String,
//     preasure: i32,
//     tunnels_to: Vec<String>,
// }
