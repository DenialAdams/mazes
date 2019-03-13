use crate::grid::Grid;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;

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
   let mut neighbors = vec![];
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
   let mut neighbors = vec![];
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
   let mut neighbors = vec![];
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
            if visited[i] {
               continue;
            }
            neighbors.clear();
            grid.neighbors(i, &mut neighbors);
            neighbors.retain(|i| visited[*i]);
            if neighbors.len() == 0 {
               continue;
            }

            let target = *neighbors.choose(rng).unwrap();
            grid.connect_neighbors(cur_index, target);
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
