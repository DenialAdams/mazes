pub(crate) struct DisjointSet {
   tree: Vec<Node>,
}

struct Node {
   parent: usize,
   rank: usize,
}

impl DisjointSet {
   pub fn new(num_sets: usize) -> DisjointSet {
      let mut tree = Vec::with_capacity(num_sets);
      for i in 0..num_sets {
         tree.push(Node { parent: i, rank: 0 });
      }
      DisjointSet { tree }
   }

   // find with path compression
   pub fn find(&mut self, x: usize) -> usize {
      let current_parent = self.tree[x].parent;
      if current_parent != x {
         self.tree[x].parent = self.find(current_parent);
      }
      self.tree[x].parent
   }

   // union by rank
   pub fn union(&mut self, x: usize, y: usize) {
      let x_root = self.find(x);
      let y_root = self.find(y);

      if x_root == y_root {
         return;
      }

      let (x_root, y_root) = if self.tree[x_root].rank < self.tree[y_root].rank {
         (y_root, x_root)
      } else {
         (x_root, y_root)
      };

      self.tree[y_root].parent = x_root;
      if self.tree[x_root].rank == self.tree[y_root].rank {
         self.tree[x_root].rank += 1;
      }
   }
}
