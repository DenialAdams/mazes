#![feature(duration_float)]

mod grid;
mod mazegen;
mod pathfinding;

use grid::Grid;
use mazegen::{aldous_broder, binary_tree, sidewinder};
use rand::FromEntropy;
use rand_xorshift::XorShiftRng;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use rand::SeedableRng;

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

fn main() {
   let mut grid = Grid::new(75, 75);
   //let mut rng = XorShiftRng::from_entropy();
   let mut rng = XorShiftRng::seed_from_u64(1);
   // binary_tree(&mut grid, &mut rng);
   //sidewinder(&mut grid, &mut rng);
   let mut start_time = Instant::now();
   aldous_broder(&mut grid, &mut rng);
   println!("mazegen elapsed: {}", start_time.elapsed().as_float_secs());
   start_time = Instant::now();
   let path = pathfinding::a_star(&grid, pathfinding::null_h).unwrap();
   println!("uniform cost search elapsed: {}", start_time.elapsed().as_float_secs());
   start_time = Instant::now();
   let path = pathfinding::a_star(&grid, pathfinding::manhattan_h).unwrap();
   println!("astar elapsed: {}", start_time.elapsed().as_float_secs());
   let mut destination = BufWriter::new(File::create("maze.svg").unwrap());
   writeln!(
      destination,
      "<svg viewBox=\"-3 -3 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
      (grid.width * 3) + 6,
      (grid.height * 3) + 6
   )
   .unwrap();
   grid.write_grid_as_svg(&mut destination).unwrap();
   for (i, x) in path.1.into_iter().enumerate() {
      let row = i / grid.width;
      let col = i % grid.width;

      let upper_left_y = row * 3;
      let upper_left_x = col * 3;
      use pathfinding::DiagStatus;
      match x {
         DiagStatus::Unexplored => {}
         DiagStatus::Generated => {
            writeln!(
               destination,
               "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" style=\"stroke-width:0px;stroke:#ededed;fill:#ffff00\" />",
               upper_left_x, upper_left_y, 3, 3
            ).unwrap();
         }
         DiagStatus::Expanded => {
            writeln!(
               destination,
               "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" style=\"stroke-width:0px;stroke:#ededed;fill:#ff8c00\" />",
               upper_left_x, upper_left_y, 3, 3
            ).unwrap();
         }
      }
   }
   for i in path.0.into_iter() {
      let row = i / grid.width;
      let col = i % grid.width;

      let upper_left_y = row * 3;
      let upper_left_x = col * 3;
      writeln!(
         destination,
         "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" style=\"stroke-width:0px;stroke:#ededed;fill:#ff0000\" />",
         upper_left_x, upper_left_y, 3, 3
      )
      .unwrap();
   }
   grid.write_maze_as_svg(&mut destination).unwrap();
   writeln!(destination, "</svg>").unwrap();
   //println!("{:?}", pathfinding::uniform_cost(&grid).unwrap());

   //let app = App::new(GuiState {}, AppConfig::default());
   //let window = Window::new(WindowCreateOptions::default(), css::native()).unwrap();
   //app.run(window).unwrap();
}
