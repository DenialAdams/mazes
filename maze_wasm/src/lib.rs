use maze_lib::grid::Grid;
use maze_lib::mazegen;
use rand::FromEntropy;
use std::io::Write;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_maze_and_give_me_svg(width: usize, height: usize) -> String {

   let mut result = Vec::new();
   let mut rng = rand_xorshift::XorShiftRng::from_entropy();
   let mut grid = Grid::new(width, height);
   mazegen::carve_maze(&mut grid, &mut rng, mazegen::Algo::RecursiveBacktracker);
   writeln!(
      result,
      "<svg viewBox=\"-3 -3 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
      (grid.width * 3) + 6,
      (grid.height * 3) + 6
   )
   .unwrap();
   grid.write_maze_as_svg(&mut result).unwrap();
   writeln!(result, "</svg>").unwrap();
   unsafe { String::from_utf8_unchecked(result) }
}
