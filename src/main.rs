mod grid;

use std::io::BufWriter;
use std::fs::File;
use grid::Grid;
use rand::{FromEntropy, Rng};
use rand::seq::{SliceRandom, IteratorRandom};
use rand_xorshift::XorShiftRng;

fn main() {
   let mut grid = Grid::new(12, 12);
   let mut rng = XorShiftRng::from_entropy();
   //binary_tree(&mut grid, &mut rng);
   //sidewinder(&mut grid, &mut rng);
   aldous_broder(&mut grid, &mut rng);
   let mut destination = BufWriter::new(File::create("maze.svg").unwrap());
   grid.write_as_svg(&mut destination).unwrap();
}

fn binary_tree<R: Rng>(grid: &mut Grid, rng: &mut R) {
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

fn sidewinder<R: Rng>(grid: &mut Grid, rng: &mut R) {
   let mut cur_run = vec![];
   for i in 0..grid.inner.len() {
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

fn aldous_broder<R: Rng>(grid: &mut Grid, rng: &mut R) {
   let mut neighbors = vec![];
   let mut visited = vec![false; grid.inner.len()];
   let mut cur_index = (0..grid.inner.len()).choose(rng).unwrap();
   visited[cur_index] = true;
   while visited.iter().any(|x| !x) {
      neighbors.clear();
      grid.neighbors(cur_index, &mut neighbors);
      let target = *neighbors.choose(rng).unwrap();
      if !visited[target] {
         if grid.has_neighbor_north(cur_index) && target == cur_index - grid.width {
            grid.connect_cell_north(cur_index);
         } else if target == cur_index + grid.width {
            grid.connect_cell_south(cur_index);
         } else if target == cur_index + 1 {
            grid.connect_cell_east(cur_index)
         } else {
            grid.connect_cell_west(cur_index);
         }
      }
      cur_index = target;
      visited[cur_index] = true;
   }
}
