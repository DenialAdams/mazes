use super::diagnostic_map::{DiagMap, FinalizedDiagMap, DIAG_EXPANDED, DIAG_GENERATED, DIAG_UNEXPLORED};
use super::heuristics::manhattan_h;
use crate::grid::Grid;
use std::cmp::{Ord, Ordering, PartialOrd, Reverse};
use std::collections::BinaryHeap;
use std::io::{self, Write};

#[derive(Clone, PartialEq, Eq)]
struct PriorityNode {
   priority: usize,
   i: usize,
   path: Vec<usize>,
}

impl PartialOrd for PriorityNode {
   fn partial_cmp(&self, other: &PriorityNode) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl Ord for PriorityNode {
   fn cmp(&self, other: &PriorityNode) -> Ordering {
      self
         .priority
         .cmp(&other.priority)
         .then(self.path.len().cmp(&other.path.len()))
   }
}

#[derive(Clone, PartialEq, Eq)]
struct Node {
   i: usize,
   path: Vec<usize>,
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
      path: vec![],
   }));
   let mut closed: Box<[usize]> = vec![std::usize::MAX; grid.size()].into_boxed_slice();
   let mut neighbors_to_generate = Vec::with_capacity(4);
   while let Some(Reverse(mut cur_node)) = open.pop() {
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

      let cur_path_len = cur_node.path.len() + 1;

      if closed[cur_node.i] <= cur_node.path.len() {
         continue;
      }

      // Expand
      {
         let new_path_len = cur_path_len + 1;
         cur_node.path.reserve_exact(1);
         cur_node.path.push(cur_node.i);
         // wrapping sub is ok because we only use
         // i.e. i_north when we know it is north connected
         let i_north = cur_node.i.wrapping_sub(grid.width);
         let i_south = cur_node.i + grid.width;
         let i_east = cur_node.i + 1;
         let i_west = cur_node.i.wrapping_sub(1);
         neighbors_to_generate.clear();
         // fill neighbors_to_generate with our neighbors
         {
            // N
            if grid[cur_node.i].north_connected && closed[i_north] > new_path_len {
               neighbors_to_generate.push(i_north);
            }
            // S
            if grid[cur_node.i].south_connected && closed[i_south] > new_path_len {
               neighbors_to_generate.push(i_south);
            }
            // E
            if grid[cur_node.i].east_connected && closed[i_east] > new_path_len {
               neighbors_to_generate.push(i_east);
            }
            // W
            if grid[cur_node.i].west_connected && closed[i_west] > new_path_len {
               neighbors_to_generate.push(i_west);
            }
         }
         // actually do expansion
         {
            let g = new_path_len * !greedy as usize;
            // first, we generate every node other than the first neighbor
            neighbors_to_generate.iter().skip(1).for_each(|i| {
               open.push(Reverse(PriorityNode {
                  priority: g + h(*i, goal, grid.width),
                  i: *i,
                  path: cur_node.path.clone(),
               }));
               nodes_generated += 1;
               diag_map.mark_generated(*i);
               closed[*i] = new_path_len;
            });
            // now, we generate the first neighbor
            // the vast vast majority of cells have only one neighbor
            // in mazes, so avoiding a clone is a huge optimization
            if let Some(i) = neighbors_to_generate.get(0) {
               open.push(Reverse(PriorityNode {
                  priority: g + h(*i, goal, grid.width),
                  i: *i,
                  path: cur_node.path,
               }));
               nodes_generated += 1;
               diag_map.mark_generated(*i);
               closed[*i] = new_path_len;
            }
         }
      }
      nodes_expanded += 1;
      diag_map.mark_expanded(cur_node.i);
   }
   None
}

pub fn dfs(grid: &Grid, start: usize, goal: usize) -> Option<PathData> {
   struct DfsNode {
      i: usize,
      path_len: usize,
   }

   let mut nodes_generated = 0;
   let mut nodes_expanded = 0;
   let mut diag_map = DiagMap::new(grid.size());
   let mut path = vec![];
   let mut stack: Vec<DfsNode> = vec![DfsNode {
      i: start,
      path_len: 0,
   }];
   while let Some(cur_node) = stack.pop() {
      path.truncate(cur_node.path_len);
      path.push(cur_node.i);
      if cur_node.i == goal {
         return Some(PathData {
            path: path.into_boxed_slice(),
            diag: diag_map.into(),
            nodes_generated,
            nodes_expanded,
         });
      }

      // Expand
      let stack_size_before_expansion = stack.len();
      let path_len = path.len();
      {
         // wrapping sub is ok because we only use
         // i.e. i_north when we know it is north connected
         let i_north = cur_node.i.wrapping_sub(grid.width);
         let i_south = cur_node.i + grid.width;
         let i_east = cur_node.i + 1;
         let i_west = cur_node.i.wrapping_sub(1);
         // fill neighbors_to_generate with our neighbors
         {
            // N
            if grid[cur_node.i].north_connected && diag_map[i_north] == DIAG_UNEXPLORED {
               stack.push(DfsNode {
                  i: i_north,
                  path_len,
               });
               nodes_generated += 1;
               diag_map.mark_generated(i_north);
            }
            // S
            if grid[cur_node.i].south_connected && diag_map[i_south] == DIAG_UNEXPLORED {
               stack.push(DfsNode {
                  i: i_south,
                  path_len,
               });
               nodes_generated += 1;
               diag_map.mark_generated(i_south);
            }
            // E
            if grid[cur_node.i].east_connected && diag_map[i_east] == DIAG_UNEXPLORED {
               stack.push(DfsNode {
                  i: i_east,
                  path_len,
               });
               nodes_generated += 1;
               diag_map.mark_generated(i_east);
            }
            // W
            if grid[cur_node.i].west_connected && diag_map[i_west] == DIAG_UNEXPLORED {
               stack.push(DfsNode {
                  i: i_west,
                  path_len,
               });
               nodes_generated += 1;
               diag_map.mark_generated(i_west);
            }
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
      });
   }
   None
}

pub fn djikstra(grid: &Grid, start: usize) -> Box<[usize]> {
   let mut best_paths = vec![std::usize::MAX; grid.size()].into_boxed_slice();
   let mut open: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
   let mut neighbors_to_generate = Vec::with_capacity(4);
   open.push(Reverse(Node { i: start, path: vec![] }));
   while let Some(Reverse(mut cur_node)) = open.pop() {
      let cur_path_len = cur_node.path.len() + 1;
      // expand
      {
         let new_path_len = cur_path_len + 1;
         cur_node.path.reserve_exact(1);
         cur_node.path.push(cur_node.i);
         // wrapping sub is ok because we only use
         // i.e. i_north when we know it is north connected
         let i_north = cur_node.i.wrapping_sub(grid.width);
         let i_south = cur_node.i + grid.width;
         let i_east = cur_node.i + 1;
         let i_west = cur_node.i.wrapping_sub(1);
         neighbors_to_generate.clear();
         // fill neighbors_to_generate with our neighbors
         {
            // N
            if grid[cur_node.i].north_connected && best_paths[i_north] > new_path_len {
               neighbors_to_generate.push(i_north);
            }
            // S
            if grid[cur_node.i].south_connected && best_paths[i_south] > new_path_len {
               neighbors_to_generate.push(i_south);
            }
            // E
            if grid[cur_node.i].east_connected && best_paths[i_east] > new_path_len {
               neighbors_to_generate.push(i_east);
            }
            // W
            if grid[cur_node.i].west_connected && best_paths[i_west] > new_path_len {
               neighbors_to_generate.push(i_west);
            }
         }
         // actually do expansion
         {
            // first, we generate every node other than the first neighbor
            neighbors_to_generate.iter().skip(1).for_each(|i| {
               open.push(Reverse(Node {
                  i: *i,
                  path: cur_node.path.clone(),
               }));
            });
            // now, we generate the first neighbor
            // the vast vast majority of cells have only one neighbor
            // in mazes, so avoiding a clone is a huge optimization
            if let Some(i) = neighbors_to_generate.get(0) {
               open.push(Reverse(Node {
                  i: *i,
                  path: cur_node.path,
               }));
            }
         }
      }
      best_paths[cur_node.i] = cur_path_len;
   }
   best_paths
}
