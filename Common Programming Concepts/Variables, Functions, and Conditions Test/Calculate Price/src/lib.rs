pub fn calculate_price(amount: u32) -> u32 {
    let mut price: u32 = 2;
    if amount >= 40 {
        price = 1
    }
    amount * price
}
