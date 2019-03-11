use crate::grid::Grid;
use fxhash::FxHashSet;
use std::collections::BinaryHeap;
use std::cmp::{PartialOrd, Ord, Ordering, Reverse};

#[derive(PartialEq, Eq)]
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

pub fn uniform_cost(grid: &Grid) -> Option<Box<[usize]>> {
   let mut nodes_generated = 0;
   let mut nodes_expanded = 0;
   let mut open: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
   open.push(Reverse(Node {
      i: 0,
      path: vec![].into_boxed_slice(),
   }));
   let goal = grid.size() - 1;
   let mut closed: FxHashSet<usize> = FxHashSet::with_hasher(Default::default());
   while let Some(cur_node) = open.pop() {
      let cur_node = cur_node.0;
      // because this is a BFS,
      // if we've already reached this state
      // it was reached in fewer path (or the same number of path),
      // we can not possibly do better
      if closed.get(&cur_node.i).is_some() {
         continue;
      }
      if cur_node.i == goal {
         let mut final_path = cur_node.path.to_vec();
         final_path.push(goal);
         return Some(final_path.into_boxed_slice());
      }
      // Expand
      {
         // N
         if grid.has_neighbor_north(cur_node.i) && grid[cur_node.i].north_connected {
            let mut new_path = cur_node.path.to_vec();
            new_path.push(cur_node.i);
            open.push(Reverse(Node {
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
               i: cur_node.i - 1,
               path: new_path.into_boxed_slice(),
            }));
            nodes_generated += 1;
         }
      }
      nodes_expanded += 1;
      closed.insert(cur_node.i);
   }
   None
}
