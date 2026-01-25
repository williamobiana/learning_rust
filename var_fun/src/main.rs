
fn calculate_stats() -> (f64, f64, f64) {
    let min = 10.5;
    let max = 99.9;
    let avg = 55.2;
    (min, max, avg)  // Return tuple
}

fn main() {
    let (_minimum, _maximum, _average) = calculate_stats();
    println!("min: {}, max: {}, avg: {}", _minimum, _maximum, _average);
}
