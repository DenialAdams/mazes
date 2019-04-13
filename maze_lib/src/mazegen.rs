use crate::disjoint_set::DisjointSet;
use crate::grid::Grid;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;
use std::fmt;

#[derive(Copy, Clone)]
pub enum Algo {
   BinaryTree,
   Sidewinder,
   AldousBroder,
   Wilson,
   HuntAndKill,
   RecursiveBacktracker,
   Kruskal,
   RecursiveDivision,
}

impl fmt::Display for Algo {
   fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
      write!(
         fmt,
         "{}",
         match self {
            Algo::BinaryTree => "Binary Tree",
            Algo::Sidewinder => "Sidewinder",
            Algo::AldousBroder => "Aldous-Broder",
            Algo::Wilson => "Wilsons's",
            Algo::HuntAndKill => "Hunt and Kill",
            Algo::RecursiveBacktracker => "Recursive Backtracker",
            Algo::Kruskal => "Kruskal's",
            Algo::RecursiveDivision => "Recursive Division",
         }
      )
   }
}

pub const ALGOS: [Algo; 8] = [
   Algo::BinaryTree,
   Algo::Sidewinder,
   Algo::AldousBroder,
   Algo::Wilson,
   Algo::HuntAndKill,
   Algo::RecursiveBacktracker,
   Algo::Kruskal,
   Algo::RecursiveDivision,
];

pub fn carve_maze<R: Rng>(grid: &mut Grid, rng: &mut R, algo: Algo) {
   match algo {
      Algo::BinaryTree => binary_tree(grid, rng),
      Algo::Sidewinder => sidewinder(grid, rng),
      Algo::AldousBroder => aldous_broder(grid, rng),
      Algo::Wilson => wilson(grid, rng),
      Algo::HuntAndKill => hunt_and_kill(grid, rng),
      Algo::RecursiveBacktracker => recursive_backtracker(grid, rng),
      Algo::Kruskal => kruskal(grid, rng),
      Algo::RecursiveDivision => recursive_division(grid, rng),
   }
}

pub fn binary_tree<R: Rng>(grid: &mut Grid, rng: &mut R) {
   for i in 0..grid.size() {
      if grid.has_neighbor_north(i) && grid.has_neighbor_east(i) {
         if rng.gen_bool(0.5) {
            grid.connect_cell_north(i);
         } else {
            grid.connect_cell_east(i);
         }
      } else if grid.has_neighbor_north(i) {
         grid.connect_cell_north(i);
      } else if grid.has_neighbor_east(i) {
         grid.connect_cell_east(i);
      }
   }
}

pub fn sidewinder<R: Rng>(grid: &mut Grid, rng: &mut R) {
   let mut cur_run = vec![];
   for i in 0..grid.size() {
      if grid.has_neighbor_north(i) && grid.has_neighbor_east(i) {
         cur_run.push(i);
         if rng.gen_bool(0.5) {
            grid.connect_cell_north(*cur_run.choose(rng).unwrap());
            cur_run.clear();
         } else {
            grid.connect_cell_east(i);
         }
      } else if grid.has_neighbor_north(i) {
         cur_run.push(i);
         grid.connect_cell_north(*cur_run.choose(rng).unwrap());
         cur_run.clear();
      } else if grid.has_neighbor_east(i) {
         grid.connect_cell_east(i);
      }
   }
}

pub fn aldous_broder<R: Rng>(grid: &mut Grid, rng: &mut R) {
   let mut neighbors = Vec::with_capacity(4);
   let mut visited = vec![false; grid.size()];
   let mut cur_index = (0..grid.size()).choose(rng).unwrap();
   visited[cur_index] = true;
   while visited.iter().any(|x| !x) {
      neighbors.clear();
      grid.neighbors(cur_index, &mut neighbors);
      let target = *neighbors.choose(rng).unwrap();
      if !visited[target] {
         grid.connect_neighbors(cur_index, target);
      }
      cur_index = target;
      visited[cur_index] = true;
   }
}

pub fn wilson<R: Rng>(grid: &mut Grid, rng: &mut R) {
   let mut neighbors = Vec::with_capacity(4);
   let mut visited = vec![false; grid.size()];
   let mut walker_path: Vec<usize> = vec![(0..grid.size()).choose(rng).unwrap()];
   visited[0] = true;
   while visited.iter().any(|x| !x) {
      if visited[*walker_path.last().unwrap()] {
         for window in walker_path.windows(2) {
            visited[window[0]] = true;
            visited[window[1]] = true;
            grid.connect_neighbors(window[0], window[1]);
         }
         walker_path.clear();
         walker_path.push((0..grid.size()).choose(rng).unwrap());
      } else {
         neighbors.clear();
         grid.neighbors(*walker_path.last().unwrap(), &mut neighbors);
         let target = *neighbors.choose(rng).unwrap();
         if let Some(i) = walker_path.iter().rposition(|i| *i == target) {
            walker_path.truncate(i + 1);
         } else {
            walker_path.push(target);
         }
      }
   }
}

pub fn hunt_and_kill<R: Rng>(grid: &mut Grid, rng: &mut R) {
   let mut neighbors = Vec::with_capacity(4);
   let mut visited = vec![false; grid.size()];
   visited[0] = true;
   let mut cur_index = 0;
   'outer: loop {
      neighbors.clear();
      grid.neighbors(cur_index, &mut neighbors);
      neighbors.retain(|i| !visited[*i]);
      if neighbors.is_empty() {
         // HUNT
         for i in 0..grid.size() {
            // unvisited...
            if visited[i] {
               continue;
            }

            // ...with at least one visited neighbor
            neighbors.clear();
            grid.neighbors(i, &mut neighbors);
            neighbors.retain(|j| visited[*j]);
            if neighbors.is_empty() {
               continue;
            }

            // choose a visited neighbor, connect
            let target = *neighbors.choose(rng).unwrap();
            grid.connect_neighbors(i, target);
            cur_index = i;
            visited[i] = true;
            continue 'outer;
         }
         // didn't find any cells in hunt
         // (no unvisited cells)
         break;
      }
      let target = *neighbors.choose(rng).unwrap();
      grid.connect_neighbors(cur_index, target);
      cur_index = target;
      visited[cur_index] = true;
   }
}

pub fn recursive_backtracker<R: Rng>(grid: &mut Grid, rng: &mut R) {
   let mut neighbors = Vec::with_capacity(4);
   let mut stack = vec![0];
   let mut visited = vec![false; grid.size()];
   visited[0] = true;
   while !stack.is_empty() {
      neighbors.clear();
      grid.neighbors(*stack.last().unwrap(), &mut neighbors);
      neighbors.retain(|i| !visited[*i]);
      if neighbors.is_empty() {
         stack.pop();
      } else {
         let target = *neighbors.choose(rng).unwrap();
         grid.connect_neighbors(*stack.last().unwrap(), target);
         stack.push(target);
         visited[target] = true;
      }
   }
}

pub fn kruskal<R: Rng>(grid: &mut Grid, rng: &mut R) {
   let mut disjoint_set = DisjointSet::new(grid.size());
   let mut edges = Vec::with_capacity(grid.size() * 2);
   for i in 0..grid.size() {
      if grid.has_neighbor_south(i) {
         edges.push((i, i + grid.width))
      }
      if grid.has_neighbor_east(i) {
         edges.push((i, i + 1))
      }
   }
   edges.shuffle(rng);
   for edge in edges {
      if disjoint_set.find(edge.0) == disjoint_set.find(edge.1) {
         continue;
      }
      disjoint_set.union(edge.0, edge.1);
      grid.connect_neighbors(edge.0, edge.1);
   }
}

pub fn recursive_division<R: Rng>(grid: &mut Grid, rng: &mut R) {
   struct Rectangle {
      x: usize,
      y: usize,
      width: usize,
      height: usize,
   }
   // make the grid fully connected
   for i in 0..grid.size() {
      if grid.has_neighbor_south(i) {
         grid.connect_cell_south(i)
      }
      if grid.has_neighbor_east(i) {
         grid.connect_cell_east(i)
      }
   }

   let mut rects = vec![Rectangle {
      x: 0,
      y: 0,
      width: grid.width,
      height: grid.height,
   }];
   while let Some(rect) = rects.pop() {
      if rect.width <= 1 || rect.height <= 1 {
         continue;
      }

      // divide vertically
      if rect.height <= rect.width {
         let mid_x = rect.x + rect.width / 2;
         for i in rect.y..(rect.y + rect.height) {
            grid.disconnect_cell_west(i * grid.width + mid_x);
         }
         let random_i = (rect.y..(rect.y + rect.height)).choose(rng).unwrap();
         grid.connect_cell_west(random_i * grid.width + mid_x);
         // divide
         rects.push(Rectangle {
            x: rect.x,
            y: rect.y,
            width: rect.width / 2,
            height: rect.height,
         });
         rects.push(Rectangle {
            x: rect.x + rect.width / 2,
            y: rect.y,
            width: (rect.width / 2) + (rect.width % 2),
            height: rect.height,
         });
      } else {
         let mid_y = rect.y + rect.height / 2;
         for i in rect.x..(rect.x + rect.width) {
            grid.disconnect_cell_north(mid_y * grid.width + i);
         }
         let random_i = (rect.x..(rect.x + rect.width)).choose(rng).unwrap();
         grid.connect_cell_north(mid_y * grid.width + random_i);
         // divide
         rects.push(Rectangle {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height / 2,
         });
         rects.push(Rectangle {
            x: rect.x,
            y: rect.y + rect.height / 2,
            width: rect.width,
            height: (rect.height / 2) + (rect.height % 2),
         });
      }
   }
}
