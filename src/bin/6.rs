fn main(){

    let numbers = vec![10,20,1,5,15];

    let numbers2 = add_number(numbers, |number| number>=&&10);
    println!("{:?}", numbers2);

}

fn add_number<F: Fn(&&u32)-> bool>(numbers: Vec<u32>, closuer: F) -> Vec<u32>{
    numbers.iter()
    .filter(closuer)
    .map(|number| number*2)
    .collect()
}