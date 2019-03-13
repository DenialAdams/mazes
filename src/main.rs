#![feature(duration_float)]

mod grid;
mod mazegen;
mod pathfinding;

use grid::Grid;
use pathfinding::PathData;
use rand::FromEntropy;
use rand_xorshift::XorShiftRng;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::time::Instant;

/*

struct GuiState;

impl Layout for GuiState {
   fn layout(&self, info: LayoutInfo<Self>) -> Dom<Self> {
      if let Some((svg_cache, svg_layers)) = self.svg {
         Svg::with_layers(svg_layers).dom(&info.window, &svg_cache)
      } else {
         Button::with_label("Load SVG file").dom().with_callback(On::MouseUp, load_svg)
      }
   }
}

fn load_svg(app_state: &mut AppState<GuiState>, _: &mut CallbackInfo<GuiState>) -> UpdateScreen {
    let mut svg_cache = SvgCache::empty();
    let svg_layers = svg_cache.add_svg("maze.svg").unwrap();
    app_state.data.modify(|data| data.svg = Some((svg_cache, svg_layers)));
    Redraw
}

*/

fn init_svg(name: &'static str, grid: &Grid) -> io::Result<BufWriter<File>> {
   let mut destination = BufWriter::new(File::create(format!("{}.svg", name)).unwrap());
   writeln!(
      destination,
      "<svg viewBox=\"-3 -3 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
      (grid.width * 3) + 6,
      (grid.height * 3) + 6
   )?;
   Ok(destination)
}

fn write_path_and_maze_to_svg(name: &'static str, path_data: &PathData, grid: &Grid) -> io::Result<()> {
   let mut dest = init_svg(name, grid)?;
   pathfinding::write_diag_to_svg(&path_data.diag, grid.width, &mut dest)?;
   pathfinding::write_path_to_svg(&path_data.path, grid.width, &mut dest)?;
   grid.write_maze_as_svg(&mut dest)?;
   writeln!(dest, "</svg>")
}

fn main() {
   let mut rng = XorShiftRng::from_entropy();
   if std::env::args().any(|x| x == "--dead-ends") {
      const DEADEND_WIDTH: usize = 20;
      const DEADEND_HEIGHT: usize = 20;
      const DEADEND_SIZE: usize = DEADEND_WIDTH * DEADEND_HEIGHT;
      const DEADEND_SAMPLES: usize = 100;
      let avg_fmt_width = format!("{}", DEADEND_SIZE).len();
      let mut averages = Vec::with_capacity(mazegen::ALGOS.len());
      for algo in mazegen::ALGOS.iter() {
         println!("Running {}...", algo);
         let mut deadend_counts: Vec<usize> = Vec::with_capacity(DEADEND_SAMPLES);
         for _ in 0..DEADEND_SAMPLES {
            let mut grid = Grid::new(DEADEND_WIDTH, DEADEND_HEIGHT);
            mazegen::carve_maze(&mut grid, &mut rng, *algo);
            deadend_counts.push(grid.dead_ends().count());
         }
         let total_deadends = deadend_counts.iter().fold(0, |acc, x| acc + x);
         let avg_deadends = total_deadends as f64 / DEADEND_SAMPLES as f64;
         averages.push((*algo, avg_deadends));
      }
      println!();
      println!(
         "Average dead-ends per {}x{} maze ({} total cells):",
         DEADEND_WIDTH, DEADEND_HEIGHT, DEADEND_SIZE
      );
      println!();
      averages.sort_unstable_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
      for (algo, avg) in averages {
         let pct = avg * 100.0 / (DEADEND_SIZE as f64);
         println!(
            "{:>23} : {:>width$} ({}%)",
            format!("{}", algo),
            avg.round(),
            pct.round(),
            width = avg_fmt_width
         );
      }
      return;
   }
   let mut grid = Grid::new(12, 12);
   // mazegen
   {
      let start_time = Instant::now();
      //mazegen::binary_tree(&mut grid, &mut rng);
      //mazegen::sidewinder(&mut grid, &mut rng);
      //mazegen::aldous_broder(&mut grid, &mut rng);
      //mazegen::wilson(&mut grid, &mut rng);
      //mazegen::hunt_and_kill(&mut grid, &mut rng);
      mazegen::recursive_backtracker(&mut grid, &mut rng);
      println!("mazegen elapsed: {}", start_time.elapsed().as_float_secs());
      println!("{} dead-ends", grid.dead_ends().count());
   }
   // uniform cost search
   let ucs_path = {
      let start_time = Instant::now();
      let path = pathfinding::a_star(&grid, pathfinding::null_h).unwrap();
      println!("uniform cost search elapsed: {}", start_time.elapsed().as_float_secs());
      path
   };
   // a star
   let astar_path = {
      let start_time = Instant::now();
      let path = pathfinding::a_star(&grid, pathfinding::manhattan_h).unwrap();
      println!("astar elapsed: {}", start_time.elapsed().as_float_secs());
      path
   };
   // greedy best first
   let gbf_path = {
      let start_time = Instant::now();
      let path = pathfinding::greedy_best_first(&grid, pathfinding::manhattan_h).unwrap();
      println!("greedy best first elapsed: {}", start_time.elapsed().as_float_secs());
      path
   };
   // write the maze clean
   {
      let mut dest = init_svg("maze", &grid).unwrap();
      grid.write_maze_as_svg(&mut dest).unwrap();
      writeln!(dest, "</svg>").unwrap();
   }
   write_path_and_maze_to_svg("astar", &astar_path, &grid).unwrap();
   write_path_and_maze_to_svg("ucs", &ucs_path, &grid).unwrap();
   write_path_and_maze_to_svg("greedy_best_first", &gbf_path, &grid).unwrap();
}
