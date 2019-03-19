use crate::grid::Grid;
use fxhash::FxHashMap;
use std::cmp::{Ord, Ordering, PartialOrd, Reverse};
use std::collections::BinaryHeap;
use std::io::{self, Write};

#[derive(PartialEq, Eq)]
struct Node {
   priority: usize,
   i: usize,
   path: Box<[usize]>,
}

impl PartialOrd for Node {
   fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl Ord for Node {
   fn cmp(&self, other: &Node) -> Ordering {
      self.priority.cmp(&other.priority)
   }
}

#[derive(PartialEq, Eq)]
struct DjikstraNode {
   i: usize,
   path: Box<[usize]>,
}

impl PartialOrd for DjikstraNode {
   fn partial_cmp(&self, other: &DjikstraNode) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl Ord for DjikstraNode {
   fn cmp(&self, other: &DjikstraNode) -> Ordering {
      self.path.len().cmp(&other.path.len())
   }
}

pub fn null_h(_: usize, _: usize, _: usize) -> usize {
   0
}

/// Calculate |x - y| while avoiding underflow
fn abs_diff(x: usize, y: usize) -> usize {
   if x > y {
      x - y
   } else {
      y - x
   }
}

pub fn manhattan_h(i: usize, goal: usize, width: usize) -> usize {
   let i_row = i / width;
   let i_col = i % width;

   let goal_row = goal / width;
   let goal_col = goal % width;

   abs_diff(i_col, goal_col) + abs_diff(i_row, goal_row)
}

pub const DIAG_UNEXPLORED: u8 = 0x00;
pub const DIAG_GENERATED: u8 = 0x01;
pub const DIAG_EXPANDED: u8 = 0x03;

#[derive(Copy, Clone)]
pub struct DiagStatus(pub u8);

pub struct PathData {
   pub path: Box<[usize]>,
   pub diag: Box<[DiagStatus]>,
   pub nodes_generated: u64,
   pub nodes_expanded: u64,
}

pub fn write_diag_to_svg<W: Write>(diag: &[DiagStatus], width: usize, dest: &mut W) -> io::Result<()> {
   for (i, x) in diag.iter().enumerate() {
      let row = i / width;
      let col = i % width;

      let upper_left_y = row * 3;
      let upper_left_x = col * 3;

      match x.0 {
         DIAG_UNEXPLORED => {}
         DIAG_GENERATED => {
            writeln!(
               dest,
               "<rect x=\"{}\" y=\"{}\" width=\"3\" height=\"3\" style=\"stroke-width:0.1px;stroke:#ffff00;fill:#ffff00\" />",
               upper_left_x, upper_left_y
            )?
         }
         DIAG_EXPANDED => {
            writeln!(
               dest,
               "<rect x=\"{}\" y=\"{}\" width=\"3\" height=\"3\" style=\"stroke-width:0.1px;stroke:#ff8c00;fill:#ff8c00\" />",
               upper_left_x, upper_left_y
            )?
         }
         _ => unreachable!(),
      }
   }
   Ok(())
}

pub fn write_path_to_svg<W: Write>(path: &[usize], width: usize, dest: &mut W) -> io::Result<()> {
   for i in path.iter() {
      let row = i / width;
      let col = i % width;

      let upper_left_y = row * 3;
      let upper_left_x = col * 3;
      writeln!(
         dest,
         "<rect x=\"{}\" y=\"{}\" width=\"3\" height=\"3\" style=\"stroke-width:0.1px;stroke:#ff0000;fill:#ff0000\" />",
         upper_left_x, upper_left_y
      )?
   }
   Ok(())
}

fn expand_node(
   cur_node: &Node,
   grid: &Grid,
   open: &mut BinaryHeap<Reverse<Node>>,
   priority: usize,
   diag_map: &mut [DiagStatus],
   nodes_generated: &mut u64,
) {
   let new_path_len = cur_node.path.len() + 1;
   // N
   if grid.has_neighbor_north(cur_node.i) && grid[cur_node.i].north_connected {
      let mut new_path = Vec::with_capacity(new_path_len);
      new_path.extend_from_slice(&cur_node.path);
      new_path.push(cur_node.i);
      open.push(Reverse(Node {
         priority,
         i: cur_node.i - grid.width,
         path: new_path.into_boxed_slice(),
      }));
      *nodes_generated += 1;
      diag_map[cur_node.i - grid.width].0 |= DIAG_GENERATED;
   }
   // S
   if grid.has_neighbor_south(cur_node.i) && grid[cur_node.i].south_connected {
      let mut new_path = Vec::with_capacity(new_path_len);
      new_path.extend_from_slice(&cur_node.path);
      new_path.push(cur_node.i);
      open.push(Reverse(Node {
         priority,
         i: cur_node.i + grid.width,
         path: new_path.into_boxed_slice(),
      }));
      *nodes_generated += 1;
      diag_map[cur_node.i + grid.width].0 |= DIAG_GENERATED;
   }
   // E
   if grid.has_neighbor_east(cur_node.i) && grid[cur_node.i].east_connected {
      let mut new_path = Vec::with_capacity(new_path_len);
      new_path.extend_from_slice(&cur_node.path);
      new_path.push(cur_node.i);
      open.push(Reverse(Node {
         priority,
         i: cur_node.i + 1,
         path: new_path.into_boxed_slice(),
      }));
      *nodes_generated += 1;
      diag_map[cur_node.i + 1].0 |= DIAG_GENERATED;
   }
   // W
   if grid.has_neighbor_west(cur_node.i) && grid[cur_node.i].west_connected {
      let mut new_path = Vec::with_capacity(new_path_len);
      new_path.extend_from_slice(&cur_node.path);
      new_path.push(cur_node.i);
      open.push(Reverse(Node {
         priority,
         i: cur_node.i - 1,
         path: new_path.into_boxed_slice(),
      }));
      *nodes_generated += 1;
      diag_map[cur_node.i - 1].0 |= DIAG_GENERATED;
   }
}

pub fn a_star<F>(grid: &Grid, h: F, start: usize, goal: usize, greedy: bool) -> Option<PathData>
where
   F: Fn(usize, usize, usize) -> usize,
{
   let mut nodes_generated = 0;
   let mut nodes_expanded = 0;
   let mut diag_map = vec![DiagStatus(DIAG_UNEXPLORED); grid.size()].into_boxed_slice();
   let mut open: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
   open.push(Reverse(Node {
      priority: h(start, goal, grid.width),
      i: start,
      path: vec![].into_boxed_slice(),
   }));
   let mut closed: FxHashMap<usize, usize> = FxHashMap::with_hasher(Default::default());
   while let Some(Reverse(cur_node)) = open.pop() {
      // if we've already reached this state in fewer actions (or the same number of actions),
      // we can not possibly do better
      if closed
         .get(&cur_node.i)
         .map(|x| *x <= cur_node.path.len())
         .unwrap_or(false)
      {
         continue;
      }

      if cur_node.i == goal {
         let mut final_path = cur_node.path.to_vec();
         final_path.push(goal);
         return Some(PathData {
            path: final_path.into_boxed_slice(),
            diag: diag_map,
            nodes_generated,
            nodes_expanded,
         });
      }

      let f = (cur_node.path.len() * !greedy as usize) + h(cur_node.i, goal, grid.width);
      expand_node(&cur_node, grid, &mut open, f, &mut diag_map, &mut nodes_generated);
      nodes_expanded += 1;
      diag_map[cur_node.i].0 = DIAG_EXPANDED;
      closed.insert(cur_node.i, cur_node.path.len());
   }
   None
}

pub fn djikstra(grid: &Grid, start: usize) -> Box<[usize]> {
   let mut best_paths = vec![std::usize::MAX; grid.size()].into_boxed_slice();
   let mut open: BinaryHeap<Reverse<DjikstraNode>> = BinaryHeap::new();
   open.push(Reverse(DjikstraNode {
      i: start,
      path: vec![].into_boxed_slice(),
   }));
   while let Some(Reverse(cur_node)) = open.pop() {
      // expand
      {
         let new_path_len = cur_node.path.len() + 1;
         // N
         if grid.has_neighbor_north(cur_node.i) && grid[cur_node.i].north_connected && best_paths[cur_node.i - grid.width] > new_path_len {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            open.push(Reverse(DjikstraNode {
               i: cur_node.i - grid.width,
               path: new_path.into_boxed_slice(),
            }));
         }
         // S
         if grid.has_neighbor_south(cur_node.i) && grid[cur_node.i].south_connected && best_paths[cur_node.i + grid.width] > new_path_len {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            open.push(Reverse(DjikstraNode {
               i: cur_node.i + grid.width,
               path: new_path.into_boxed_slice(),
            }));
         }
         // E
         if grid.has_neighbor_east(cur_node.i) && grid[cur_node.i].east_connected && best_paths[cur_node.i + 1] > new_path_len {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            open.push(Reverse(DjikstraNode {
               i: cur_node.i + 1,
               path: new_path.into_boxed_slice(),
            }));
         }
         // W
         if grid.has_neighbor_west(cur_node.i) && grid[cur_node.i].west_connected && best_paths[cur_node.i - 1] > new_path_len {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            open.push(Reverse(DjikstraNode {
               i: cur_node.i - 1,
               path: new_path.into_boxed_slice(),
            }));
         }
      }
      best_paths[cur_node.i] = cur_node.path.len();
   }
   best_paths
}
