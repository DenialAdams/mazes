use maze_lib::grid::Grid;
use maze_lib::{mazegen, pathfinding};
use maze_lib::pathfinding::diagnostic_map::FinalizedDiagMap;
use rand::{FromEntropy, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::io::Write;

use wasm_bindgen::prelude::*;

pub struct MazeApp {
   grid: Grid,
}

static mut MAZE_APP: Option<MazeApp> = None;

#[wasm_bindgen]
pub fn app_init() {
   std::panic::set_hook(Box::new(console_error_panic_hook::hook));
   unsafe {
      MAZE_APP = Some(MazeApp {
         grid: Grid::new(12, 12),
      })
   };
}

const DIAG_PATH: u8 = 0x07;

pub struct FinalizedDiagMapWasm {
   pub inner: Box<[u8]>,
   pub generated_history: Box<[u32]>,
   pub expanded_history: Box<[u32]>,
   pub num_generated_history: Box<[u32]>,
}

#[wasm_bindgen]
pub fn pathfind(start: usize, goal: usize, pathfind_algo: &str) -> Box<[u8]> {
   let app = unsafe { MAZE_APP.as_ref().unwrap() };
   let pf_data = match pathfind_algo {
      "UniformCostSearch" => pathfinding::algos::a_star(&app.grid, pathfinding::heuristics::null_h, start, goal, false),
      "AStar" => pathfinding::algos::a_star(&app.grid, pathfinding::heuristics::manhattan_h, start, goal, false),
      "GreedyBestFirst" => pathfinding::algos::a_star(&app.grid, pathfinding::heuristics::manhattan_h, start, goal, true),
      "DepthFirstSearch" => pathfinding::algos::dfs(&app.grid, start, goal),
      _ => panic!("Got a bad pathfinding algo from JS"),
   }
   .unwrap();
   let mut pf_data_diag = unsafe { std::mem::transmute::<FinalizedDiagMap, FinalizedDiagMapWasm>(pf_data.diag) };
   for i in pf_data.path.iter() {
      pf_data_diag.inner[*i] = DIAG_PATH;
   }
   pf_data_diag.inner
}

#[wasm_bindgen]
pub fn djikstra(start: usize) -> Box<[u32]> {
   let app = unsafe { MAZE_APP.as_ref().unwrap() };
   let best_paths = pathfinding::algos::djikstra(&app.grid, start);
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
pub fn change_grid(width: usize, height: usize) -> String {
   let app = unsafe { MAZE_APP.as_mut().unwrap() };
   let mut result = Vec::new();
   app.grid = Grid::new(width, height);
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
   writeln!(result, "</svg>").unwrap();
   unsafe { String::from_utf8_unchecked(result) }
}

#[wasm_bindgen]
pub fn carve_maze(mazegen_algo: &str, seed_string: String) -> String {
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
   app.grid.reset();
   let mut rng = if seed_string.is_empty() {
      XorShiftRng::from_entropy()
   } else {
      let seed_u64 = fxhash::hash64(&seed_string);
      XorShiftRng::seed_from_u64(seed_u64)
   };
   mazegen::carve_maze(&mut app.grid, &mut rng, algo);
   writeln!(result, "<g id=\"g_maze\">").unwrap();
   app.grid.write_maze_as_svg(&mut result).unwrap();
   writeln!(result, "</g>").unwrap();
   unsafe { String::from_utf8_unchecked(result) }
}
