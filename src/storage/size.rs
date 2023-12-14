pub const fn byte(b: usize) -> usize {
  b
}
pub const fn kb(b: usize) -> usize {
  (1 << 10) * byte(b)
}
pub const fn mb(b: usize) -> usize {
  (1 << 10) * kb(b)
}
pub const fn gb(b: usize) -> usize {
  (1 << 10) * mb(b)
}
pub const fn tb(b: usize) -> usize {
  (1 << 10) * gb(b)
}
pub const fn pb(b: usize) -> usize {
  (1 << 10) * tb(b)
}
