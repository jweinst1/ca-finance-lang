
/// The `Trade` trait embodies the most fundamental and basic properties 
/// of a cell that can buy and sell items. 
pub trait Trade {
	/// Indicates the item quantity currently owned as the supply. 
	fn supply(&self) -> f32;
	fn demand(&self) -> f32;
	fn cash(&self) -> f32;
}
/// This function calculates a price based on the demand of one cell
/// applied to the supply of another. This is a local form of price, and
/// does not take into consideration the second cell's demand.
pub fn price_for<T:Trade>(cell_1:&T, cell_2:&T) -> f32 {
	cell_1.demand() / cell_2.supply()
}

#[cfg(test)]
mod unit_tests {
	use crate::single_item::*;

	struct TestTrader {	}

	impl Trade for TestTrader  {
		fn supply(&self) -> f32 { 1.0 }
		fn demand(&self) -> f32 { 2.0 }
		fn cash(&self) -> f32   { 10.0 }
	}
	#[test]
	fn price_for_works() {
		let a  = TestTrader {};
		let b = TestTrader {};
		assert!((price_for(&a, &b) == 2.0));
	}
}
