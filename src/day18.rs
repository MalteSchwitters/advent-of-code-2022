use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs;

pub fn solve_puzzle() {
    log::debug!("-------------");
    log::debug!("Solving Day 18");
    let verticies = parse_input("./inputs/day18.txt");
    log::info!("Surface Count: {}", get_surfaces_count(&verticies));
}

fn get_surfaces_count(verticies: &Vec<Vertex>) -> usize {
    let system = create_coordinate_system(verticies);
    let test_vectors = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let mut count = 0;
    for (index, vertex) in verticies.iter().enumerate() {
        for vector in test_vectors {
            if test_vector(&system, &vertex, &vector) {
                count += 1;
            }
        }
        let mut completion = index as f32 / verticies.len() as f32 * 100.0;
        completion = (completion * 100.0).round() / 100.0;
        log::debug!("{}% complete, {} surfaces found", completion, count);
    }
    count
}

fn test_vector(system: &CoordinateSystem, vertex: &Vertex, vector: &(i32, i32, i32)) -> bool {
    let x = vertex.x + vector.0;
    let y = vertex.y + vector.1;
    let z = vertex.z + vector.2;

    if system.get(x, y, z).is_some() {
        return false;
    }
    test_reachable_from_outside(system, &(x, y, z))
}

fn test_reachable_from_outside(system: &CoordinateSystem, point: &(i32, i32, i32)) -> bool {
    let test_vectors = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let mut open: LinkedList<Vertex> = LinkedList::from([Vertex {
        x: point.0,
        y: point.1,
        z: point.2,
    }]);
    let mut closed: LinkedList<Vertex> = LinkedList::new();
    let mut x;
    let mut y;
    let mut z;
    while let Some(position) = open.pop_front() {
        closed.push_back(position);
        if system.is_out_of_bounds(position.x, position.y, position.z) {
            return true;
        }
        for vector in test_vectors {
            x = position.x + vector.0;
            y = position.y + vector.1;
            z = position.z + vector.2;

            let vert = Vertex { x: x, y: y, z: z };
            if closed.contains(&vert) {
                continue;
            }
            if open.contains(&vert) {
                continue;
            }
            if system.get(x, y, z).is_some() {
                closed.push_back(vert);
            } else {
                open.push_back(vert);
            }
        }
    }
    // all nodes have reached an end
    false
}

fn create_coordinate_system(verticies: &Vec<Vertex>) -> CoordinateSystem {
    let system = CoordinateSystem::new(&verticies);
    system
}

fn parse_input(file: &str) -> Vec<Vertex> {
    let input = fs::read_to_string(file).unwrap();
    input
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| Vertex {
            x: v[0],
            y: v[1],
            z: v[2],
        })
        .collect()
}

struct CoordinateSystem {
    system: HashMap<i32, HashMap<i32, HashMap<i32, char>>>,
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
    z_bounds: (i32, i32),
}

impl CoordinateSystem {
    fn new(verticies: &Vec<Vertex>) -> CoordinateSystem {
        let mut system = CoordinateSystem {
            system: HashMap::new(),
            x_bounds: (0, 0),
            y_bounds: (0, 0),
            z_bounds: (0, 0),
        };
        system.render(verticies);
        system
    }

    fn render(&mut self, verticies: &Vec<Vertex>) {
        let x_values = verticies.iter().map(|v| v.x);
        let y_values = verticies.iter().map(|v| v.y);
        let z_values = verticies.iter().map(|v| v.z);
        self.x_bounds = (x_values.clone().min().unwrap(), x_values.max().unwrap());
        self.y_bounds = (y_values.clone().min().unwrap(), y_values.max().unwrap());
        self.z_bounds = (z_values.clone().min().unwrap(), z_values.max().unwrap());

        for vert in verticies {
            if !self.system.contains_key(&vert.x) {
                self.system.insert(vert.x, HashMap::new());
            }
            let x = self.system.get_mut(&vert.x).unwrap();

            if !x.contains_key(&vert.y) {
                x.insert(vert.y, HashMap::new());
            }
            let xy = x.get_mut(&vert.y).unwrap();
            xy.insert(vert.z, '#');
        }
    }

    fn is_out_of_bounds(&self, x: i32, y: i32, z: i32) -> bool {
        x < self.x_bounds.0
            || x > self.x_bounds.1
            || y < self.y_bounds.0
            || y > self.y_bounds.1
            || z < self.z_bounds.0
            || z > self.z_bounds.1
    }

    fn get(&self, x: i32, y: i32, z: i32) -> Option<char> {
        let maybe_x_axis = self.system.get(&x);
        if let Some(x_axis) = maybe_x_axis {
            let maybe_y_axis = x_axis.get(&y);
            if let Some(y_axis) = maybe_y_axis {
                return y_axis.get(&z).map(|xyz| xyz.clone());
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vertex {
    x: i32,
    y: i32,
    z: i32,
}
