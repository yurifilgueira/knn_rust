pub mod knn {
    use core::f64;

    use ndarray::Array1;

    pub fn calculate_distance(a: &Array1<f64>, b: &Array1<f64>) -> f64 {
        
        let sum: f64 = a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powf(2.0))
            .sum();

        f64::sqrt(sum as f64)
    }

    pub fn find_knn(k: i32, item: &Array1<f64>) -> Vec<Option<ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::prelude::Dim<[usize; 1]>>>> {
        
        let a = Array1::from_vec([5.1, 3.5, 1.4, 0.2].to_vec());
        let b = Array1::from_vec([5.0, 3.3, 1.4, 0.2].to_vec());
        let c = Array1::from_vec([7.0, 3.2, 4.7, 1.4].to_vec());
        let d = Array1::from_vec([5.7, 2.8, 4.1, 1.3].to_vec());
        let e = Array1::from_vec([6.3, 3.3, 6.0, 2.5].to_vec());
        let f = Array1::from_vec([5.9, 3.0, 5.1, 1.8].to_vec());

        let dataset = [a, b, c, d, e, f];

        let mut distances: Vec<(f64, Array1<f64>)> = dataset
            .iter()
            .map(|data| (calculate_distance(item, data), data.clone()))
            .collect();
        
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        
        let knn: Vec<Option<Array1<f64>>> = (0..k)
            .map(|key| distances.get(key as usize).map(|(_, data)| data.clone()))
            .collect();
        
        knn
    }
}
