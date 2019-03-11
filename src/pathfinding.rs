use crate::grid::Grid;
use fxhash::FxHashMap;
use std::cmp::{Ord, Ordering, PartialOrd, Reverse};
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Node {
   f: usize,
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
      self.f.cmp(&other.f)
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

pub fn a_star<F>(grid: &Grid, h: F) -> Option<Box<[usize]>>
where
   F: Fn(usize, usize, usize) -> usize,
{
   let mut nodes_generated = 0;
   let mut nodes_expanded = 0;
   let mut open: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
   let start = 0;
   let goal = grid.size() - 1;
   open.push(Reverse(Node {
      f: h(start, goal, grid.width),
      i: start,
      path: vec![].into_boxed_slice(),
   }));
   let mut closed: FxHashMap<usize, usize> = FxHashMap::with_hasher(Default::default());
   while let Some(cur_node) = open.pop() {
      let cur_node = cur_node.0;
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
         println!("{} nodes generated {} nodes expanded", nodes_generated, nodes_expanded);
         return Some(final_path.into_boxed_slice());
      }

      let f = cur_node.path.len() + h(cur_node.i, goal, grid.width);

      // Expand
      {
         // N
         if grid.has_neighbor_north(cur_node.i) && grid[cur_node.i].north_connected {
            let mut new_path = cur_node.path.to_vec();
            new_path.push(cur_node.i);
            open.push(Reverse(Node {
               f,
               i: cur_node.i - grid.width,
               path: new_path.into_boxed_slice(),
            }));
            nodes_generated += 1;
         }
         // S
         if grid.has_neighbor_south(cur_node.i) && grid[cur_node.i].south_connected {
            let mut new_path = cur_node.path.to_vec();
            new_path.push(cur_node.i);
            open.push(Reverse(Node {
               f,
               i: cur_node.i + grid.width,
               path: new_path.into_boxed_slice(),
            }));
            nodes_generated += 1;
         }
         // E
         if grid.has_neighbor_east(cur_node.i) && grid[cur_node.i].east_connected {
            let mut new_path = cur_node.path.to_vec();
            new_path.push(cur_node.i);
            open.push(Reverse(Node {
               f,
               i: cur_node.i + 1,
               path: new_path.into_boxed_slice(),
            }));
            nodes_generated += 1;
         }
         // W
         if grid.has_neighbor_west(cur_node.i) && grid[cur_node.i].west_connected {
            let mut new_path = cur_node.path.to_vec();
            new_path.push(cur_node.i);
            open.push(Reverse(Node {
               f,
               i: cur_node.i - 1,
               path: new_path.into_boxed_slice(),
            }));
            nodes_generated += 1;
         }
      }
      nodes_expanded += 1;
      closed.insert(cur_node.i, cur_node.path.len());
   }
   None
}
