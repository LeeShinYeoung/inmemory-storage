use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Key(Vec<u8>);
impl Key {
  pub fn new(bytes: Vec<u8>) -> Self {
    Self(bytes)
  }
}

impl AsRef<[u8]> for Key {
  fn as_ref(&self) -> &[u8] {
    &self.0
  }
}

impl AsMut<[u8]> for Key {
  fn as_mut(&mut self) -> &mut [u8] {
    &mut self.0
  }
}

impl From<Vec<u8>> for Key {
  fn from(value: Vec<u8>) -> Self {
    Self::new(value)
  }
}

impl ToString for Key {
  fn to_string(&self) -> String {
    String::from_utf8_lossy(&self.0).to_string()
  }
}

impl Hash for Key {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.hash(state)
  }
}

impl PartialEq for Key {
  fn eq(&self, other: &Self) -> bool {
    self.0.eq(&other.0)
  }
}
impl Eq for Key {}

impl PartialOrd for Key {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.0.partial_cmp(&other.0)
  }
}
