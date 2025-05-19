pub fn null_h(_: usize, _: usize, _: usize) -> usize {
   0
}

pub fn manhattan_h(i: usize, goal: usize, width: usize) -> usize {
   let i_row = i / width;
   let i_col = i % width;

   let goal_row = goal / width;
   let goal_col = goal % width;

   i_col.abs_diff(goal_col) + i_row.abs_diff(goal_row)
}
