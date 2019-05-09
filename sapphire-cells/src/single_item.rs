
/// The `Trade` trait embodies the most fundamental and basic properties 
/// of a cell that can buy and sell items. 
pub trait Trade {
	/// Indicates the item quantity currently owned as the supply. 
	fn supply(&self) -> f32;
	/// Indicates the demand for the item in the market. 
	fn demand(&self) -> f32;
	/// Indicates the amount of money this `Trade` cell possesses.
	fn cash(&self) -> f32;
}
/// This function calculates a price based on the demand of one cell
/// applied to the supply of another. This is a local form of price, and
/// does not take into consideration the second cell's demand.
pub fn price_for<T:Trade>(cell_1:&T, cell_2:&T) -> f32 {
	cell_1.demand() / cell_2.supply()
}
/// This function is very similar to `price_for` except it
/// accounts for the selling `Trade` object's demand as well.
/// In a single item market, all demand in the market is directed toward 
/// the same item, thus sellers may take that into account.
pub fn price_for_demand<T:Trade>(cell_1:&T, cell_2:&T) -> f32 {
	(cell_1.demand() + cell_2.demand()) / cell_2.supply()
}
/// This function is also similar to `price_for` except it alters 
/// the usual `demand / supply` result to account for the buyer's
/// supply. This means buyers with higher supplies will have their
/// demand adjusted to be lower, vica versa.
pub fn price_self_supply<T:Trade>(cell_1:&T, cell_2:&T) -> f32 {
	(cell_1.demand() / cell_1.supply()) / cell_2.supply()
}
/// Determines if the `Trade` cell has more supply than demand.
pub fn is_supply_major<T:Trade>(cell:&T) -> bool {
	cell.supply() > cell.demand()
}
/// Determines if the `Trade` cell has more demand than supply.
pub fn is_demand_major<T:Trade>(cell:&T) -> bool {
	cell.supply() < cell.demand()
}

pub fn find_max_demand<T:Trade>(cells:&[T]) -> &T {
    let mut max:&T = &cells[0];
    for i in 1..cells.len() {
        if cells[i].demand() > max.demand() {
            max = &cells[i];
        }
    }
    &max
}
/// The `Transact` trait provides an interface of mutable operations 
/// to facilitate buying and selling items. This trait permits the adding 
/// or subtracting of cash and item supply.
/// ## Note
/// The floating point number returned here represents the quantity 
/// that is withdrawn or deposited. It is possible for an implementation 
/// of this trait to not withdraw or deposit the `amount` being passed in 
/// due to special crtieria.
pub trait Transact {
	fn withdraw_cash(&mut self, amount:f32) -> f32;

	fn deposit_cash(&mut self, amount:f32) -> f32;

	fn withdraw_supply(&mut self, amount:f32) -> f32;

	fn deposit_supply(&mut self, amount:f32) -> f32;
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
	#[test]
	fn price_for_demand_works() {
		let a  = TestTrader {};
		let b  = TestTrader {};
		assert!((price_for_demand(&a, &b) == 4.0));
	}
	#[test]
	fn price_self_supply_works() {
		let a  = TestTrader {};
		let b  = TestTrader {};
		assert!((price_self_supply(&a, &b) == 2.0));
	}

	struct TestCustomer {
		_cash:f32,
		_demand:f32,
		_supply:f32
	}

	impl Trade for TestCustomer  {
		fn supply(&self) -> f32 { self._supply }
		fn demand(&self) -> f32 { self._demand }
		fn cash(&self) -> f32   { self._cash }
	}

	#[test]
	fn find_max_demand_works() {
		let lst = [TestCustomer{_cash:5.0, _demand:4.2, _supply:0.7},
		          TestCustomer{_cash:5.0, _demand:2.2, _supply:0.7},
		          TestCustomer{_cash:5.0, _demand:6.3, _supply:0.7}
		          ];
		let maxer = find_max_demand(&lst);
		assert_eq!(maxer.demand(), 6.3);
	}

}
