use maze_lib::grid::Grid;
use maze_lib::mazegen;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::time::Instant;

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

fn main() {
   let seed_string = "jitter";
   let mut rng = if seed_string.is_empty() {
      XorShiftRng::from_entropy()
   } else {
      let seed_u64 = fxhash::hash64(seed_string);
      XorShiftRng::seed_from_u64(seed_u64)
   };
   if std::env::args().any(|x| x == "--dead-ends") {
      const DEADEND_WIDTH: usize = 20;
      const DEADEND_HEIGHT: usize = 20;
      const DEADEND_SIZE: usize = DEADEND_WIDTH * DEADEND_HEIGHT;
      let mut grid = Grid::new(DEADEND_WIDTH, DEADEND_HEIGHT);
      const DEADEND_SAMPLES: usize = 100;
      let avg_fmt_width = format!("{}", DEADEND_SIZE).len();
      let mut averages = Vec::with_capacity(mazegen::ALGOS.len());
      for algo in mazegen::ALGOS.iter() {
         println!("Running {}...", algo);
         let mut deadend_counts: Vec<usize> = Vec::with_capacity(DEADEND_SAMPLES);
         for _ in 0..DEADEND_SAMPLES {
            grid.reset();
            mazegen::carve_maze(&mut grid, &mut rng, *algo);
            deadend_counts.push(grid.dead_ends().count());
         }
         let total_deadends: usize = deadend_counts.iter().sum();
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
   let mut grid = Grid::new(10_000, 10_000);
   // mazegen
   {
      let start_time = Instant::now();
      //mazegen::binary_tree(&mut grid, &mut rng);
      //mazegen::sidewinder(&mut grid, &mut rng);
      //mazegen::aldous_broder(&mut grid, &mut rng);
      //mazegen::wilson(&mut grid, &mut rng);
      //mazegen::hunt_and_kill(&mut grid, &mut rng);
      mazegen::recursive_backtracker(&mut grid, &mut rng);
      //mazegen::kruskal(&mut grid, &mut rng);
      //mazegen::recursive_division(&mut grid, &mut rng);
      //mazegen::eller(&mut grid, &mut rng);
      //mazegen::prim_simplified(&mut grid, &mut rng);
      println!("mazegen elapsed: {}", start_time.elapsed().as_secs_f64());
      println!("{} dead-ends", grid.dead_ends().count());
   }
   let start_time = Instant::now();
   //let pf_data = maze_lib::pathfinding::algos::a_star(&grid, maze_lib::pathfinding::heuristics::manhattan_h, 0, grid.size() - 1, false).unwrap();
   let pf_data = maze_lib::pathfinding::algos::dfs(&grid, 0, grid.size() - 1).unwrap();
   println!("pathfinding elapsed: {}", start_time.elapsed().as_secs_f64());
   println!(
      "nodes expanded, generated: {} {}",
      pf_data.nodes_expanded, pf_data.nodes_generated
   );
   println!("path length: {}", pf_data.path.len());
   // write the maze clean
   if false {
      let mut dest = init_svg("maze", &grid).unwrap();
      grid.write_maze_as_svg(&mut dest).unwrap();
      writeln!(dest, "</svg>").unwrap();
   }
}
