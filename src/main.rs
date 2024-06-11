use knn::knn::find_knn;
use ndarray::Array1;

mod knn;

fn main() {

    let item = Array1::from_vec(vec![5.6, 2.8, 6.4, 2.2]);

    let results = find_knn(2, &item);

    for r in results {
        println!("{}", r.unwrap());
    }

}
