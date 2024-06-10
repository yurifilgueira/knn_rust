pub mod knn {
    use core::f64;

    pub fn calculate_distance(a: &[i32; 5], b: &[i32; 5]) -> f64 {
        
        let sum: i32 = a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).pow(2))
            .sum();

        f64::sqrt(sum as f64)
    }
}   
