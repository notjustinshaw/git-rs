pub trait Findable {
  fn find(&self, ch: u8, offset: usize) -> Option<usize>;
}

impl Findable for [u8] {
  /// Returns the `Some(index)` of the character in the byte slice, or `None`.
  ///
  /// # Example
  /// ```
  /// let my_slice = &['a', 'b', 'c'];
  /// assert_eq!(my_slice.find('a', 0), Some(0));
  /// assert_eq!(my_slice.find('a', 1), None);
  /// assert_eq!(my_slice.find('z'), None);
  /// ```
  fn find(&self, ch: u8, offset: usize) -> Option<usize> {
    for i in offset..self.len() {
      if self.get(i).unwrap().eq(&ch) {
        return Some(i);
      }
    }
    None
  }
}
