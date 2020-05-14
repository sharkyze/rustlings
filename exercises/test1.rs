// test1.rs
// This is a test for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// Put your function here!
fn calculate_apple_price(qty: usize) -> usize {
    match qty {
        1..=39 => 2 * qty,
        _ => 1 * qty,
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);
    let price3 = calculate_apple_price(40);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
    assert_eq!(40, price3);
}
