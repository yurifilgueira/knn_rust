mod knn;
use crate::knn::knn::calculate_distance;

fn main() {

    let a = [3, 4, 4, 1, 4];
    let b = [4, 3, 5, 1, 5];

    let result = calculate_distance(&a, &b);

    println!("{}", result);
}
