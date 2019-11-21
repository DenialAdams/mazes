use maze_lib::grid::Grid;
use maze_lib::{mazegen, pathfinding};
use rand::SeedableRng;
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

#[wasm_bindgen]
pub struct PfDataWasm {
   path: Box<[usize]>,
   diag: FinalizedDiagMapWasm,
   pub nodes_generated: usize,
   pub nodes_expanded: usize,
}

#[wasm_bindgen]
impl PfDataWasm {
   pub fn path(&self) -> Box<[usize]> {
      self.path.clone()
   }

   pub fn diag(&self) -> FinalizedDiagMapWasm {
      self.diag.clone()
   }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct FinalizedDiagMapWasm {
   inner: Box<[u8]>,
   generated_history: Box<[usize]>,
   expanded_history: Box<[usize]>,
   num_generated_history: Box<[u8]>,
}

#[wasm_bindgen]
impl FinalizedDiagMapWasm {
   pub fn inner(&self) -> Box<[u8]> {
      self.inner.clone()
   }

   pub fn generated_history(&self) -> Box<[usize]> {
      self.generated_history.clone()
   }

   pub fn expanded_history(&self) -> Box<[usize]> {
      self.expanded_history.clone()
   }

   pub fn num_generated_history(&self) -> Box<[u8]> {
      self.num_generated_history.clone()
   }
}

#[wasm_bindgen]
pub fn pathfind(start: usize, goal: usize, pathfind_algo: &str) -> PfDataWasm {
   let app = unsafe { MAZE_APP.as_ref().unwrap() };
   let pf_data = match pathfind_algo {
      "UniformCostSearch" => pathfinding::algos::a_star(&app.grid, pathfinding::heuristics::null_h, start, goal, false),
      "AStar" => pathfinding::algos::a_star(&app.grid, pathfinding::heuristics::manhattan_h, start, goal, false),
      "GreedyBestFirst" => {
         pathfinding::algos::a_star(&app.grid, pathfinding::heuristics::manhattan_h, start, goal, true)
      }
      "DepthFirstSearch" => pathfinding::algos::dfs(&app.grid, start, goal),
      _ => panic!("Got a bad pathfinding algo from JS"),
   }
   .unwrap();
   let mut pf_data_wasm: PfDataWasm = unsafe { std::mem::transmute(pf_data) };
   for i in pf_data_wasm.path.iter() {
      let i = *i as usize;
      pf_data_wasm.diag.inner[i] = DIAG_PATH;
   }
   pf_data_wasm
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
pub struct MazeCarveResults {
   maze_svg: String,
   pub num_deadends: usize,
}

#[wasm_bindgen]
impl MazeCarveResults {
   pub fn maze_svg(&self) -> String {
      self.maze_svg.clone()
   }
}

#[wasm_bindgen]
pub fn carve_maze(mazegen_algo: &str, seed_string: String) -> MazeCarveResults {
   let app = unsafe { MAZE_APP.as_mut().unwrap() };
   let algo = match mazegen_algo {
      "BinaryTree" => mazegen::Algo::BinaryTree,
      "Sidewinder" => mazegen::Algo::Sidewinder,
      "AldousBroder" => mazegen::Algo::AldousBroder,
      "Wilson" => mazegen::Algo::Wilson,
      "HuntAndKill" => mazegen::Algo::HuntAndKill,
      "RecursiveBacktracker" => mazegen::Algo::RecursiveBacktracker,
      "Kruskal" => mazegen::Algo::Kruskal,
      "RecursiveDivision" => mazegen::Algo::RecursiveDivision,
      "Eller" => mazegen::Algo::Eller,
      "PrimSimplified" => mazegen::Algo::PrimSimplified,
      "PrimTrue" => mazegen::Algo::PrimTrue,
      "Empty" => mazegen::Algo::Empty,
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
   MazeCarveResults {
      maze_svg: unsafe { String::from_utf8_unchecked(result) },
      num_deadends: app.grid.dead_ends().count(),
   }
}
