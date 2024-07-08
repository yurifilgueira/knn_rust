pub mod knn {
    use core::{f64, panic};
    use std::{collections::HashMap, fs::File};

    use csv::Reader;
    use ndarray::Array1;

    pub fn calculate_distance(a: &Array1<f64>, b: &Array1<f64>) -> f64 {

        let sum: f64 = a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powf(2.0))
            .sum();

        f64::sqrt(sum as f64)
    }

    pub fn find_knn(_k: i32, item: &Array1<f64>, csv_reader: &mut Reader<File>) -> Vec<Option<Array1<f64>>> {
    
        let mut idx = 0;
        let mut all_info: HashMap<String, Vec<f64>> = HashMap::new();
        let mut distances: Vec<f64> = Vec::new();
        for res in csv_reader.records() {

            let mut info: Vec<f64> = Vec::new();

            idx += 1;
            println!("Line: {idx}");

                let r = res.unwrap();

            let sepal_length_str = String::from(r.get(0).unwrap());
            let sepal_length: f64 = match sepal_length_str.trim().parse() {
                Ok(f) => f,
                Err(_) =>
                    panic!("Error converting '{}' to f64", sepal_length_str),
            };

            let sepal_width_str = String::from(r.get(1).unwrap());
            let sepal_width: f64 = match sepal_width_str.trim().parse() {
                Ok(f) => f,
                Err(_) =>
                    panic!("Error converting '{}' to f64", sepal_width_str),
            };

            let petal_length_str = String::from(r.get(2).unwrap());
            let petal_length: f64 = match petal_length_str.trim().parse() {
                Ok(f) => f,
                Err(_) =>
                    panic!("Error converting '{}' to f64", petal_length_str),
            };

            let petal_width_str = String::from(r.get(3).unwrap());
            let petal_width: f64 = match petal_width_str.trim().parse() {
                Ok(f) => f,
                Err(_) =>
                    panic!("Error converting '{}' to f64", petal_width_str),
            };  

            let class = String::from(r.get(4).unwrap());

            println!("sepal_length: {}", sepal_length);
            println!("sepal_width: {}", sepal_width);
            println!("petal_length: {}", petal_length);
            println!("petal_width: {}", petal_width);
            println!("class: {}", class);

            info.push(sepal_length);
            info.push(sepal_width);
            info.push(petal_length);
            info.push(petal_width);

            let info = Array1::from_vec(info);
            let distance = calculate_distance(item, &info);
            
            distances.push(distance);
            
            all_info.insert(class, distances.clone());
        }

        for (k, v) in all_info {
            for value in v {
                println!("{} -> {}", k, value);
            }
        }

        /*
           let mut distances: Vec<(f64, Array1<f64>)> = dataset
           .iter()
           .map(|data| (calculate_distance(item, data), data.clone()))
           .collect();

           distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

           let knn: Vec<Option<Array1<f64>>> = (0..k)
           .map(|key| distances.get(key as usize).map(|(_, data)| data.clone()))
           .collect();

           */

        todo!();
    }
}
