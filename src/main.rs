mod grid;

use grid::Grid;
use rand::{FromEntropy, Rng};
use rand_xorshift::XorShiftRng;

fn main() {
   let mut grid = Grid::new(21, 21);
   binary_tree(&mut grid);
   println!("{}", grid);
}

fn binary_tree(grid: &mut Grid) {
   let mut rng = XorShiftRng::from_entropy();
   for i in 0..grid.inner.len() {
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
