fn main() {
    let text = vec!["hossein", "kermani", "ali"];

    let more5: Vec<&&str> = text.iter().filter(|name| name.len() > 5).collect();
    println!("{:?}", more5);
}
