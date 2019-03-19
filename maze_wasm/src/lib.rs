use maze_lib::grid::Grid;
use maze_lib::{mazegen, pathfinding};
use rand::FromEntropy;
use rand_xorshift::XorShiftRng;
use std::io::Write;

use wasm_bindgen::prelude::*;

//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct MazeApp {
   rng: XorShiftRng,
   grid: Grid,
}

static mut MAZE_APP: Option<MazeApp> = None;

#[wasm_bindgen]
pub fn app_init() {
   std::panic::set_hook(Box::new(console_error_panic_hook::hook));
   unsafe {
      MAZE_APP = Some(MazeApp {
         rng: XorShiftRng::from_entropy(),
         grid: Grid::new(12, 12),
      })
   };
}

const DIAG_PATH: u8 = 0x07;

#[wasm_bindgen]
pub fn pathfind(start: usize, goal: usize, pathfind_algo: &str) -> Box<[u8]> {
   let app = unsafe { MAZE_APP.as_ref().unwrap() };
   let path_data = match pathfind_algo {
      "UniformCostSearch" => pathfinding::a_star(&app.grid, pathfinding::null_h, start, goal, false),
      "AStar" => pathfinding::a_star(&app.grid, pathfinding::manhattan_h, start, goal, false),
      "GreedyBestFirst" => pathfinding::a_star(&app.grid, pathfinding::manhattan_h, start, goal, true),
      _ => panic!("Got a bad pathfinding algo from JS"),
   }
   .unwrap();
   let mut diag = unsafe { std::mem::transmute::<_, Box<[u8]>>(path_data.diag) };
   for i in path_data.path.iter() {
      diag[*i] = DIAG_PATH;
   }
   diag
}

#[wasm_bindgen]
pub fn djikstra(start: usize) -> Box<[u32]> {
   let app = unsafe { MAZE_APP.as_ref().unwrap() };
   let best_paths = pathfinding::djikstra(&app.grid, start);
   let longest_path = *best_paths.iter().max().unwrap();
   let mut rgb_data = vec![0u32; best_paths.len()].into_boxed_slice();
   for (rgb, path_len) in rgb_data.iter_mut().zip(best_paths.iter()) {
      let intensity = (longest_path - *path_len) as f64 / longest_path as f64;
      let dark = (255.0 * intensity).round() as u32;
      let bright = 128 + (127.0 * intensity) as u32;
      *rgb = dark << 16 | bright << 8 | dark;
   }
   rgb_data
}

#[wasm_bindgen]
pub fn generate_maze_and_give_me_svg(width: usize, height: usize, mazegen_algo: &str) -> String {
   let app = unsafe { MAZE_APP.as_mut().unwrap() };
   let algo = match mazegen_algo {
      "BinaryTree" => mazegen::Algo::BinaryTree,
      "Sidewinder" => mazegen::Algo::Sidewinder,
      "AldousBroder" => mazegen::Algo::AldousBroder,
      "Wilson" => mazegen::Algo::Wilson,
      "HuntAndKill" => mazegen::Algo::HuntAndKill,
      "RecursiveBacktracker" => mazegen::Algo::RecursiveBacktracker,
      _ => panic!("Got a bad mazegen algo from JS"),
   };
   let mut result = Vec::new();
   app.grid = Grid::new(width, height);
   mazegen::carve_maze(&mut app.grid, &mut app.rng, algo);
   writeln!(
      result,
      "<svg viewBox=\"-3 -3 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
      (app.grid.width * 3) + 6,
      (app.grid.height * 3) + 6,
   )
   .unwrap();
   writeln!(result, "<g id=\"g_skele\">").unwrap();
   app.grid.write_skeleton_as_svg(&mut result).unwrap();
   writeln!(result, "</g>").unwrap();
   writeln!(result, "<g id=\"g_maze\">").unwrap();
   app.grid.write_maze_as_svg(&mut result).unwrap();
   writeln!(result, "</g>").unwrap();
   writeln!(result, "</svg>").unwrap();
   unsafe { String::from_utf8_unchecked(result) }
}
