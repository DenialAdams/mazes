use std::ops::{BitOrAssign, Index};

pub const DIAG_UNEXPLORED: DiagStatus = DiagStatus(0x00);
pub const DIAG_GENERATED: DiagStatus = DiagStatus(0x01);
pub const DIAG_EXPANDED: DiagStatus = DiagStatus(0x03);

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct DiagStatus(u8);

impl BitOrAssign for DiagStatus {
   fn bitor_assign(&mut self, rhs: Self) {
      self.0 |= rhs.0
   }
}

pub struct DiagMap {
   pub inner: Box<[DiagStatus]>,
   generated_history: Vec<usize>,
   expanded_history: Vec<usize>,
   num_generated_history: Vec<usize>,
   last_generated_len: usize,
}

impl DiagMap {
   pub fn new(size: usize) -> DiagMap {
      DiagMap {
         inner: vec![DIAG_UNEXPLORED; size].into_boxed_slice(),
         generated_history: Vec::with_capacity(size),
         expanded_history: Vec::with_capacity(size),
         num_generated_history: Vec::with_capacity(size),
         last_generated_len: 0,
      }
   }
}

impl Index<usize> for DiagMap {
   type Output = DiagStatus;

   fn index(&self, index: usize) -> &DiagStatus {
      &self.inner[index]
   }
}

impl DiagMap {
   pub fn mark_expanded(&mut self, index: usize) {
      self.inner[index] |= DIAG_EXPANDED;
      self.expanded_history.push(index);
      self.num_generated_history.push(self.generated_history.len() - self.last_generated_len);
      self.last_generated_len = self.generated_history.len();
   }

   pub fn mark_generated(&mut self, index: usize) {
      self.inner[index] |= DIAG_GENERATED;
      self.generated_history.push(index);
   }
}

pub struct FinalizedDiagMap {
   pub inner: Box<[DiagStatus]>,
   pub generated_history: Box<[usize]>,
   pub expanded_history: Box<[usize]>,
   pub num_generated_history: Box<[usize]>,
}

impl From<DiagMap> for FinalizedDiagMap {
   fn from(d: DiagMap) -> FinalizedDiagMap {
      FinalizedDiagMap {
         inner: d.inner,
         generated_history: d.generated_history.into_boxed_slice(),
         expanded_history: d.expanded_history.into_boxed_slice(),
         num_generated_history: d.num_generated_history.into_boxed_slice(),
      }
   }
}
