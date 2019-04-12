use crate::grid::Grid;
use super::diagnostic_map::{DiagMap, FinalizedDiagMap, DIAG_UNEXPLORED, DIAG_GENERATED, DIAG_EXPANDED};
use super::heuristics::manhattan_h;
use std::cmp::{Ord, Ordering, PartialOrd, Reverse};
use std::collections::BinaryHeap;
use std::io::{self, Write};

#[derive(Clone, PartialEq, Eq)]
struct PriorityNode {
   priority: usize,
   i: usize,
   path: Box<[usize]>,
}

impl PartialOrd for PriorityNode {
   fn partial_cmp(&self, other: &PriorityNode) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl Ord for PriorityNode {
   fn cmp(&self, other: &PriorityNode) -> Ordering {
      self.priority.cmp(&other.priority)
   }
}

#[derive(Clone, PartialEq, Eq)]
struct Node {
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
      self.path.len().cmp(&other.path.len())
   }
}

pub struct PathData {
   pub path: Box<[usize]>,
   pub diag: FinalizedDiagMap,
   pub nodes_generated: usize,
   pub nodes_expanded: usize,
}

pub fn write_diag_to_svg<W: Write>(diag: &DiagMap, width: usize, dest: &mut W) -> io::Result<()> {
   for (i, x) in diag.inner.iter().enumerate() {
      let row = i / width;
      let col = i % width;

      let upper_left_y = row * 3;
      let upper_left_x = col * 3;

      match *x {
         DIAG_UNEXPLORED => {}
         DIAG_GENERATED => {
            writeln!(
               dest,
               "<rect x=\"{}\" y=\"{}\" width=\"3\" height=\"3\" style=\"stroke-width:0.1px;stroke:#ffff00;fill:#ffff00\"/>",
               upper_left_x, upper_left_y
            )?
         }
         DIAG_EXPANDED => {
            writeln!(
               dest,
               "<rect x=\"{}\" y=\"{}\" width=\"3\" height=\"3\" style=\"stroke-width:0.1px;stroke:#ff8c00;fill:#ff8c00\"/>",
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
         "<rect x=\"{}\" y=\"{}\" width=\"3\" height=\"3\" style=\"stroke-width:0.1px;stroke:#ff0000;fill:#ff0000\"/>",
         upper_left_x, upper_left_y
      )?
   }
   Ok(())
}

#[inline(never)]
pub fn a_star<F>(grid: &Grid, h: F, start: usize, goal: usize, greedy: bool) -> Option<PathData>
where
   F: Fn(usize, usize, usize) -> usize,
{
   let mut nodes_generated = 0;
   let mut nodes_expanded = 0;
   let mut diag_map = DiagMap::new(grid.size());
   let mut open: BinaryHeap<Reverse<PriorityNode>> = BinaryHeap::new();
   open.push(Reverse(PriorityNode {
      priority: h(start, goal, grid.width),
      i: start,
      path: vec![].into_boxed_slice(),
   }));
   let mut closed: Box<[usize]> = vec![std::usize::MAX; grid.size()].into_boxed_slice();
   while let Some(Reverse(cur_node)) = open.pop() {
      if cur_node.i == goal {
         let mut final_path = cur_node.path.to_vec();
         final_path.push(goal);
         return Some(PathData {
            path: final_path.into_boxed_slice(),
            diag: diag_map.into(),
            nodes_generated,
            nodes_expanded,
         });
      }

      // Expand
      {
         // wrapping sub is ok because we only use
         // i.e. i_north when we know it is north connected
         let new_path_len = cur_node.path.len() + 1;
         let i_north = cur_node.i.wrapping_sub(grid.width);
         let i_south = cur_node.i + grid.width;
         let i_east = cur_node.i + 1;
         let i_west = cur_node.i.wrapping_sub(1);
         let g = new_path_len * !greedy as usize;
         // N
         if grid[cur_node.i].north_connected && closed[i_north] > new_path_len {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            let i_north = cur_node.i - grid.width;
            open.push(Reverse(PriorityNode {
               priority: g + h(i_north, goal, grid.width),
               i: i_north,
               path: new_path.into_boxed_slice(),
            }));
            nodes_generated += 1;
            diag_map.mark_generated(i_north);
         }
         // S
         if grid[cur_node.i].south_connected && closed[i_south] > new_path_len {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            let i_south = cur_node.i + grid.width;
            open.push(Reverse(PriorityNode {
               priority: g + h(i_south, goal, grid.width),
               i: i_south,
               path: new_path.into_boxed_slice(),
            }));
            nodes_generated += 1;
            diag_map.mark_generated(i_south);
         }
         // E
         if grid[cur_node.i].east_connected && closed[i_east] > new_path_len {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            let i_east = cur_node.i + 1;
            open.push(Reverse(PriorityNode {
               priority: g + h(i_east, goal, grid.width),
               i: i_east,
               path: new_path.into_boxed_slice(),
            }));
            nodes_generated += 1;
            diag_map.mark_generated(i_east);
         }
         // W
         if grid[cur_node.i].west_connected && closed[i_west] > new_path_len {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            let i_west = cur_node.i - 1;
            open.push(Reverse(PriorityNode {
               priority: g + h(i_west, goal, grid.width),
               i: i_west,
               path: new_path.into_boxed_slice(),
            }));
            nodes_generated += 1;
            diag_map.mark_generated(i_west);
         }
      }
      nodes_expanded += 1;
      diag_map.mark_expanded(cur_node.i);
      closed[cur_node.i] = cur_node.path.len();
   }
   None
}

pub fn dfs(grid: &Grid, start: usize, goal: usize) -> Option<PathData> {
   let mut nodes_generated = 0;
   let mut nodes_expanded = 0;
   let mut diag_map = DiagMap::new(grid.size());
   let mut stack: Vec<Node> = vec![Node {
      i: start,
      path: vec![].into_boxed_slice(),
   }];
   while let Some(cur_node) = stack.pop() {
      if cur_node.i == goal {
         let mut final_path = Vec::with_capacity(cur_node.path.len() + 1);
         final_path.extend_from_slice(&cur_node.path);
         final_path.push(goal);
         return Some(PathData {
            path: final_path.into_boxed_slice(),
            diag: diag_map.into(),
            nodes_generated,
            nodes_expanded,
         });
      }

      // Expand
      let stack_size_before_expansion = stack.len();
      {
         // wrapping sub is ok because we only use
         // i.e. i_north when we know it is north connected
         let new_path_len = cur_node.path.len() + 1;
         let i_north = cur_node.i.wrapping_sub(grid.width);
         let i_south = cur_node.i + grid.width;
         let i_east = cur_node.i + 1;
         let i_west = cur_node.i.wrapping_sub(1);
         // N
         if grid[cur_node.i].north_connected && diag_map[i_north] == DIAG_UNEXPLORED {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            stack.push(Node {
               i: i_north,
               path: new_path.into_boxed_slice(),
            });
            nodes_generated += 1;
            diag_map.mark_generated(i_north);
         }
         // S
         if grid[cur_node.i].south_connected && diag_map[i_south] == DIAG_UNEXPLORED {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            stack.push(Node {
               i: i_south,
               path: new_path.into_boxed_slice(),
            });
            nodes_generated += 1;
            diag_map.mark_generated(i_south);
         }
         // E
         if grid[cur_node.i].east_connected && diag_map[i_east] == DIAG_UNEXPLORED {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            stack.push(Node {
               i: i_east,
               path: new_path.into_boxed_slice(),
            });
            nodes_generated += 1;
            diag_map.mark_generated(i_east);
         }
         // W
         if grid[cur_node.i].west_connected && diag_map[i_west] == DIAG_UNEXPLORED {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            stack.push(Node {
               i: i_west,
               path: new_path.into_boxed_slice(),
            });
            nodes_generated += 1;
            diag_map.mark_generated(i_west);
         }
      }
      nodes_expanded += 1;
      diag_map.mark_expanded(cur_node.i);
      let new_stack_len = stack.len();
      let newly_added_elems = &mut stack[stack_size_before_expansion..new_stack_len];
      newly_added_elems.sort_unstable_by(|a, b| {
         // note we compare b to a (not a to b) in order to obtain a reverse sort,
         // such that the elements with the smallest h are at the top of the slice
         manhattan_h(b.i, goal, grid.width).cmp(&manhattan_h(a.i, goal, grid.width))
      })
   }
   None
}

pub fn djikstra(grid: &Grid, start: usize) -> Box<[usize]> {
   let mut best_paths = vec![std::usize::MAX; grid.size()].into_boxed_slice();
   let mut open: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
   open.push(Reverse(Node {
      i: start,
      path: vec![].into_boxed_slice(),
   }));
   while let Some(Reverse(cur_node)) = open.pop() {
      // expand
      {
         let new_path_len = cur_node.path.len() + 1;
         // N
         if grid[cur_node.i].north_connected && best_paths[cur_node.i - grid.width] > new_path_len {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            open.push(Reverse(Node {
               i: cur_node.i - grid.width,
               path: new_path.into_boxed_slice(),
            }));
         }
         // S
         if grid[cur_node.i].south_connected && best_paths[cur_node.i + grid.width] > new_path_len {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            open.push(Reverse(Node {
               i: cur_node.i + grid.width,
               path: new_path.into_boxed_slice(),
            }));
         }
         // E
         if grid[cur_node.i].east_connected && best_paths[cur_node.i + 1] > new_path_len {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            open.push(Reverse(Node {
               i: cur_node.i + 1,
               path: new_path.into_boxed_slice(),
            }));
         }
         // W
         if grid[cur_node.i].west_connected && best_paths[cur_node.i - 1] > new_path_len {
            let mut new_path = Vec::with_capacity(new_path_len);
            new_path.extend_from_slice(&cur_node.path);
            new_path.push(cur_node.i);
            open.push(Reverse(Node {
               i: cur_node.i - 1,
               path: new_path.into_boxed_slice(),
            }));
         }
      }
      best_paths[cur_node.i] = cur_node.path.len();
   }
   best_paths
}
