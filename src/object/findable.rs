pub trait Findable {
  fn find(&self, ch: u8) -> Option<usize>;
}

impl Findable for [u8] {
  /// Returns the `Some(index)` of the character in the byte slice, or `None`.
  ///
  /// # Example
  /// ```
  /// let my_slice = &['a', 'b', 'c'];
  /// assert_eq!(my_slice.find('a'), Some(0));
  /// assert_eq!(my_slice.find('z'), None);
  /// ```
  fn find(&self, ch: u8) -> Option<usize> {
    for i in 0..self.len() {
      if self.get(i).unwrap().eq(&ch) {
        return Some(i);
      }
    }
    return None;
  }
}
