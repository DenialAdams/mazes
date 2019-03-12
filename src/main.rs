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
   let mut grid = Grid::new(12, 12);
   let mut rng = XorShiftRng::from_entropy();
   // mazegen
   {
      let start_time = Instant::now();
      //mazegen::binary_tree(&mut grid, &mut rng);
      //mazegen::sidewinder(&mut grid, &mut rng);
      //mazegen::aldous_broder(&mut grid, &mut rng);
      //mazegen::wilson(&mut grid, &mut rng);
      mazegen::hunt_and_kill(&mut grid, &mut rng);
      println!("mazegen elapsed: {}", start_time.elapsed().as_float_secs());
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
