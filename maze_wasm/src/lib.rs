use maze_lib::grid::Grid;
use maze_lib::mazegen;
use rand::FromEntropy;
use std::io::Write;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_maze_and_give_me_svg(width: usize, height: usize, mazegen_algo: &str) -> String {
   console_error_panic_hook::set_once();
   let algo = match mazegen_algo {
      "BinaryTree" => mazegen::Algo::BinaryTree,
      "Sidewinder" => mazegen::Algo::Sidewinder,
      "AldousBroder" => mazegen::Algo::AldousBroder,
      "Wilson" => mazegen::Algo::Wilson,
      "HuntAndKill" => mazegen::Algo::HuntAndKill,
      "RecursiveBacktracker" => mazegen::Algo::RecursiveBacktracker,
      _ => panic!("Got a bad input value from JS"),
   };
   let mut result = Vec::new();
   let mut rng = rand_xorshift::XorShiftRng::from_entropy();
   let mut grid = Grid::new(width, height);
   mazegen::carve_maze(&mut grid, &mut rng, algo);
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
