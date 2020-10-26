use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight); // We could write "let mut mars_weight: f32 = ..." too.
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711 // This is equivalent to "return (weight / 9.81) * 3.711;"
}