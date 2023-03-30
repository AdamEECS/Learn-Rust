use std::collections::HashMap;

pub fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    // Two bananas are already given to you :)
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 4);
    basket.insert(String::from("orange"), 6);

    basket
}