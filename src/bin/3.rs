fn main() {
    let prices = vec![20, 70, 55, 44, 69, 20];

    println!("{}", most(&prices));
}

fn most(prices: &[i32]) -> &i32 {
    let mut most = &prices[0];
    for price in prices {
        if price > most {
            most = price;
        }
    }
    most
}
