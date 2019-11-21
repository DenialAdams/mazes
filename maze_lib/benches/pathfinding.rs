use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maze_lib::grid::Grid;
use maze_lib::mazegen;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

fn dfs_bench(c: &mut Criterion) {
   let seed_u64 = fxhash::hash64("criterion");
   let mut rng = XorShiftRng::seed_from_u64(seed_u64);
   let mut grid = Grid::new(1_000, 1_000);
   mazegen::recursive_backtracker(&mut grid, &mut rng);
   //let pf_data = maze_lib::pathfinding::algos::a_star(&grid, maze_lib::pathfinding::heuristics::manhattan_h, 0, grid.size() - 1, false).unwrap();
   c.bench_function("dfs", move |b| {
      b.iter(|| maze_lib::pathfinding::algos::dfs(&grid, black_box(0), black_box(grid.size() - 1)))
   });
}

criterion_group!(benches, dfs_bench);
criterion_main!(benches);
