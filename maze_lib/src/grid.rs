use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Default)]
pub struct Cell {
   pub north_connected: bool,
   pub south_connected: bool,
   pub east_connected: bool,
   pub west_connected: bool,
}

impl Cell {
   pub fn num_connections(self) -> u8 {
      self.north_connected as u8 + self.south_connected as u8 + self.east_connected as u8 + self.west_connected as u8
   }
}

pub struct Grid {
   pub inner: Box<[Cell]>,
   pub width: usize,
   pub height: usize,
}

impl Display for Grid {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      // top
      write!(f, "+")?;
      for _ in 0..self.width {
         write!(f, "---+")?;
      }
      writeln!(f)?;

      let mut top_buf = String::with_capacity((self.width * 3) + 1);
      let mut bot_buf = String::with_capacity((self.width * 3) + 1);
      top_buf.push('|');
      bot_buf.push('+');
      for (i, cell) in self.inner.iter().enumerate() {
         if cell.east_connected {
            top_buf.push_str("    ");
         } else {
            top_buf.push_str("   |");
         }
         if cell.south_connected {
            bot_buf.push_str("   +")
         } else {
            bot_buf.push_str("---+")
         }

         // end of row
         if (i + 1) % self.width == 0 {
            f.write_str(&top_buf)?;
            writeln!(f)?;
            f.write_str(&bot_buf)?;
            writeln!(f)?;
            top_buf.clear();
            bot_buf.clear();
            top_buf.push('|');
            bot_buf.push('+');
         }
      }

      Ok(())
   }
}

impl Index<usize> for Grid {
   type Output = Cell;

   fn index(&self, index: usize) -> &Cell {
      &self.inner[index]
   }
}

impl IndexMut<usize> for Grid {
   fn index_mut(&mut self, index: usize) -> &mut Cell {
      &mut self.inner[index]
   }
}

impl Grid {
   pub fn new(width: usize, height: usize) -> Grid {
      Grid {
         inner: vec![Cell::default(); width * height].into_boxed_slice(),
         width,
         height,
      }
   }

   pub fn reset(&mut self) {
      for x in self.inner.iter_mut() {
         *x = Cell::default();
      }
   }

   pub fn dead_ends(&self) -> impl Iterator<Item = &Cell> {
      self.inner.iter().filter(|x| x.num_connections() == 1)
   }

   pub fn get(&mut self, index: usize) -> Option<&Cell> {
      self.inner.get(index)
   }

   pub fn get_mut(&mut self, index: usize) -> Option<&mut Cell> {
      self.inner.get_mut(index)
   }

   pub fn check_if_neighbors_and_connected(&self, i1: usize, i2: usize) -> bool {
      if i2 + self.width == i1 {
         self.inner[i1].north_connected
      } else if i2 == i1 + self.width {
         self.inner[i1].south_connected
      } else if i2 == i1 + 1 {
         self.inner[i1].east_connected
      } else if i2 + 1 == i1 {
         self.inner[i1].west_connected
      } else {
         false
      }
   }

   /// If the cells are not neighbors, an incorrect connection will be made
   pub fn connect_neighbors(&mut self, i1: usize, i2: usize) {
      if i2 + self.width == i1 {
         self.connect_cell_north(i1);
      } else if i2 == i1 + self.width {
         self.connect_cell_south(i1);
      } else if i2 == i1 + 1 {
         self.connect_cell_east(i1);
      } else {
         self.connect_cell_west(i1);
      }
   }

   pub fn has_neighbor_north(&self, index: usize) -> bool {
      index >= self.width
   }

   pub fn has_neighbor_south(&self, index: usize) -> bool {
      index < (self.width * (self.height - 1))
   }

   pub fn has_neighbor_east(&self, index: usize) -> bool {
      index % self.width != (self.width - 1)
   }

   pub fn has_neighbor_west(&self, index: usize) -> bool {
      index % self.width != 0
   }

   pub fn neighbors(&self, index: usize, buf: &mut Vec<usize>) {
      if self.has_neighbor_north(index) {
         buf.push(index - self.width);
      }
      if self.has_neighbor_south(index) {
         buf.push(index + self.width);
      }
      if self.has_neighbor_east(index) {
         buf.push(index + 1);
      }
      if self.has_neighbor_west(index) {
         buf.push(index - 1);
      }
   }

   pub fn connect_cell_north(&mut self, index: usize) {
      let width = self.width;
      self[index].north_connected = true;
      self[index - width].south_connected = true;
   }

   pub fn connect_cell_south(&mut self, index: usize) {
      let width = self.width;
      self[index].south_connected = true;
      self[index + width].north_connected = true;
   }

   pub fn connect_cell_west(&mut self, index: usize) {
      self[index].west_connected = true;
      self[index - 1].east_connected = true;
   }

   pub fn connect_cell_east(&mut self, index: usize) {
      self[index].east_connected = true;
      self[index + 1].west_connected = true;
   }

   pub fn disconnect_cell_north(&mut self, index: usize) {
      let width = self.width;
      self[index].north_connected = false;
      self[index - width].south_connected = false;
   }

   pub fn disconnect_cell_south(&mut self, index: usize) {
      let width = self.width;
      self[index].south_connected = false;
      self[index + width].north_connected = false;
   }

   pub fn disconnect_cell_west(&mut self, index: usize) {
      self[index].west_connected = false;
      self[index - 1].east_connected = false;
   }

   pub fn disconnect_cell_east(&mut self, index: usize) {
      self[index].east_connected = false;
      self[index + 1].west_connected = false;
   }

   pub fn size(&self) -> usize {
      self.inner.len()
   }

   pub fn write_skeleton_as_svg<W: Write>(&self, dest: &mut W) -> io::Result<()> {
      // first, we draw a simple grid
      for i in 0..self.inner.len() {
         let row = i / self.width;
         let col = i % self.width;

         let upper_left_y = row * 3;
         let upper_left_x = col * 3;
         writeln!(
            dest,
            "<rect class=\"cell\" id=\"{}\" x=\"{}\" y=\"{}\" width=\"3\" height=\"3\"/>",
            i, upper_left_x, upper_left_y
         )?;
      }
      Ok(())
   }

   pub fn write_maze_as_svg<W: Write>(&self, dest: &mut W) -> io::Result<()> {
      // top wall
      writeln!(dest, "<line x1=\"0\" y1=\"0\" x2=\"{}\" y2=\"0\"/>", self.width * 3)?;
      // west wall
      writeln!(dest, "<line x1=\"0\" y1=\"0\" x2=\"0\" y2=\"{}\"/>", self.height * 3)?;
      let mut current_horizontal_line_segment: Option<HorizontalLineSegment> = None;
      let mut current_vertical_line_segments: Box<[Option<VerticalLineSegment>]> =
         vec![None; self.width].into_boxed_slice();
      for (i, cell) in self.inner.iter().enumerate() {
         let row = i / self.width;
         let col = i % self.width;

         let upper_left_y = row * 3;
         let upper_left_x = col * 3;

         let current_vertical_line_segment: &mut Option<VerticalLineSegment> = &mut current_vertical_line_segments[col];

         if cell.south_connected {
            if let Some(ref hls) = current_horizontal_line_segment {
               writeln!(
                  dest,
                  "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                  hls.x_1, hls.y, hls.x_2, hls.y
               )?;
               current_horizontal_line_segment = None;
            }
         } else if let Some(ref mut hls) = current_horizontal_line_segment {
            if hls.y != upper_left_y + 3 {
               writeln!(
                  dest,
                  "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                  hls.x_1, hls.y, hls.x_2, hls.y
               )?;
               hls.y = upper_left_y + 3;
               hls.x_1 = upper_left_x;
               hls.x_2 = upper_left_x + 3;
            } else {
               hls.x_2 += 3;
            }
         } else {
            current_horizontal_line_segment = Some(HorizontalLineSegment {
               y: upper_left_y + 3,
               x_1: upper_left_x,
               x_2: upper_left_x + 3,
            })
         }

         if cell.east_connected {
            if let Some(ref vls) = current_vertical_line_segment {
               writeln!(
                  dest,
                  "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                  vls.x, vls.y_1, vls.x, vls.y_2
               )?;
               *current_vertical_line_segment = None;
            }
         } else if let Some(ref mut vls) = current_vertical_line_segment {
            if vls.x != upper_left_x + 3 {
               writeln!(
                  dest,
                  "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                  vls.x, vls.y_1, vls.x, vls.y_2
               )?;
               vls.x = upper_left_x + 3;
               vls.y_1 = upper_left_y;
               vls.y_2 = upper_left_y + 3;
            } else {
               vls.y_2 += 3;
            }
         } else {
            *current_vertical_line_segment = Some(VerticalLineSegment {
               x: upper_left_x + 3,
               y_1: upper_left_y,
               y_2: upper_left_y + 3,
            })
         }
      }
      if let Some(ref hls) = current_horizontal_line_segment {
         writeln!(
            dest,
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
            hls.x_1, hls.y, hls.x_2, hls.y
         )?;
      }
      for vertical_line_segment in current_vertical_line_segments.iter() {
         if let Some(ref vls) = vertical_line_segment {
            writeln!(
               dest,
               "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
               vls.x, vls.y_1, vls.x, vls.y_2
            )?;
         }
      }
      Ok(())
   }
}

struct HorizontalLineSegment {
   y: usize,
   x_1: usize,
   x_2: usize,
}

#[derive(Clone)]
struct VerticalLineSegment {
   x: usize,
   y_1: usize,
   y_2: usize,
}

#[cfg(test)]
mod test {
   use super::Grid;

   #[test]
   fn has_neighbor() {
      let g = Grid::new(5, 5);
      assert_eq!(g.has_neighbor_north(0), false);
      assert_eq!(g.has_neighbor_south(0), true);
      assert_eq!(g.has_neighbor_east(0), true);
      assert_eq!(g.has_neighbor_west(0), false);
      assert_eq!(g.has_neighbor_north(4), false);
      assert_eq!(g.has_neighbor_south(4), true);
      assert_eq!(g.has_neighbor_east(4), false);
      assert_eq!(g.has_neighbor_west(4), true);
      assert_eq!(g.has_neighbor_north(45), true);
      assert_eq!(g.has_neighbor_south(45), false);
      assert_eq!(g.has_neighbor_east(45), true);
      assert_eq!(g.has_neighbor_west(45), false);
      assert_eq!(g.has_neighbor_north(49), true);
      assert_eq!(g.has_neighbor_south(49), false);
      assert_eq!(g.has_neighbor_east(49), false);
      assert_eq!(g.has_neighbor_west(49), true);
      assert_eq!(g.has_neighbor_north(7), true);
      assert_eq!(g.has_neighbor_south(7), true);
      assert_eq!(g.has_neighbor_east(7), true);
      assert_eq!(g.has_neighbor_west(7), true);
   }
}
