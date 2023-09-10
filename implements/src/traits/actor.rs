pub trait Actor {
  fn update(&self) -> Result<(), Box<dyn std::error::Error>>;
}