
/// The `Trade` trait embodies the most fundamental and basic properties 
/// of a cell that can buy and sell itmes.
pub trait Trade {
	fn supply(&self) -> f32;
	fn demand(&self) -> f32;
	fn cash(&self) -> f32;
}
