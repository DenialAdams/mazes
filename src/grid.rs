use std::fmt::{self, Display, Formatter};
use std::ops::{Index, IndexMut};
use std::io::{self, Write};

#[derive(Copy, Clone, Default)]
pub struct Cell {
   pub north_connected: bool,
   pub south_connected: bool,
   pub east_connected: bool,
   pub west_connected: bool,
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

pub trait GridIndex: Copy {
   fn as_1d(&self, grid_width: usize) -> usize;
}

impl GridIndex for (usize, usize) {
   fn as_1d(&self, grid_width: usize) -> usize {
      self.0 * grid_width + self.1
   }
}

impl GridIndex for usize {
   fn as_1d(&self, _grid_width: usize) -> usize {
      *self
   }
}

impl<I> Index<I> for Grid
where
   I: GridIndex,
{
   type Output = Cell;

   fn index(&self, index: I) -> &Cell {
      &self.inner[index.as_1d(self.height)]
   }
}

impl<I> IndexMut<I> for Grid
where
   I: GridIndex,
{
   fn index_mut(&mut self, index: I) -> &mut Cell {
      &mut self.inner[index.as_1d(self.height)]
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

   pub fn get<I: GridIndex>(&mut self, index: I) -> Option<&Cell> {
      self.inner.get(index.as_1d(self.width))
   }

   pub fn get_mut<I: GridIndex>(&mut self, index: I) -> Option<&mut Cell> {
      self.inner.get_mut(index.as_1d(self.width))
   }

   pub fn has_neighbor_north<I: GridIndex>(&self, index: I) -> bool {
      index.as_1d(self.width) >= self.width
   }

   pub fn has_neighbor_south<I: GridIndex>(&self, index: I) -> bool {
      index.as_1d(self.width) < (self.width * (self.height - 1))
   }

   pub fn has_neighbor_east<I: GridIndex>(&self, index: I) -> bool {
      index.as_1d(self.width) % self.width != (self.width - 1)
   }

   pub fn has_neighbor_west<I: GridIndex>(&self, index: I) -> bool {
      index.as_1d(self.width) % self.width != 0
   }

   pub fn neighbors<I: GridIndex>(&self, index: I, buf: &mut Vec<usize>) {
      let index = index.as_1d(self.width);
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

   pub fn connect_cell_north<I: GridIndex>(&mut self, index: I) {
      let width = self.width;
      self[index].north_connected = true;
      self[index.as_1d(width) - width].south_connected = true;
   }

   pub fn connect_cell_south<I: GridIndex>(&mut self, index: I) {
      let width = self.width;
      self[index].south_connected = true;
      self[index.as_1d(width) + width].north_connected = true;
   }

   pub fn connect_cell_west<I: GridIndex>(&mut self, index: I) {
      let width = self.width;
      self[index].west_connected = true;
      self[index.as_1d(width) - 1].east_connected = true;
   }

   pub fn connect_cell_east<I: GridIndex>(&mut self, index: I) {
      let width = self.width;
      self[index].east_connected = true;
      self[index.as_1d(width) + 1].west_connected = true;
   }

   pub fn size(&self) -> usize {
      self.inner.len()
   }

   pub fn write_as_svg<W: Write>(&self, dest: &mut W) -> io::Result<()> {
      writeln!(dest, "<svg viewBox=\"-3 -3 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">", (self.width * 3) + 6, (self.height * 3) + 6)?;
      // first, we draw a simple grid
      for i in 0..self.inner.len() {
         let row = i / self.width;
         let col = i % self.width;

         let upper_left_y = row * 3;
         let upper_left_x = col * 3;
         writeln!(dest, "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" style=\"stroke-width:0.1px;stroke:#ededed;fill:#ffffff\" />", upper_left_x, upper_left_y, 3, 3)?;
      }
      // top wall
      writeln!(dest, "<line x1=\"0\" y1=\"0\" x2=\"{}\" y2=\"0\" style=\"stroke:black;stroke-linecap:square;stroke-width:0.5px\" />", self.width * 3)?;
      // west wall
      writeln!(dest, "<line x1=\"0\" y1=\"0\" x2=\"0\" y2=\"{}\" style=\"stroke:black;stroke-linecap:square;stroke-width:0.5px\" />", self.height * 3)?;
      for (i, cell) in self.inner.iter().enumerate() {
         let row = i / self.width;
         let col = i % self.width;

         let upper_left_y = row * 3;
         let upper_left_x = col * 3;

         if !cell.south_connected {
            writeln!(dest, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:black;stroke-linecap:square;stroke-width:0.5px\" />", upper_left_x, upper_left_y + 3, upper_left_x + 3, upper_left_y + 3)?;
         }

         if !cell.east_connected {
            writeln!(dest, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:black;stroke-linecap:square;stroke-width:0.5px\" />", upper_left_x + 3, upper_left_y, upper_left_x + 3, upper_left_y + 3)?;
         }
      }
      writeln!(dest, "</svg>")?;
      Ok(())
   }
}

#[cfg(test)]
mod test {
   #[test]
   fn has_neighbor() {
      use super::Grid;
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
