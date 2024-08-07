pub mod knn {
    use core::{f64, panic};
    use std::{collections::HashMap, fs::File, usize};

    use csv::Reader;
    use ndarray::Array1;

    pub fn calculate_distance(a: &Array1<f64>, b: &Array1<f64>) -> f64 {

        let sum: f64 = a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powf(2.0))
            .sum();

        f64::sqrt(sum as f64)
    }

    pub fn find_knn(k: usize, item: &Array1<f64>, csv_reader: &mut Reader<File>) {
    
        let mut distances: Vec<_> = Vec::new();
        for res in csv_reader.records() {

            let mut info: Vec<f64> = Vec::new();

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

            //println!("sepal_length: {}", sepal_length);
            //println!("sepal_width: {}", sepal_width);
            //println!("petal_length: {}", petal_length);
            //println!("petal_width: {}", petal_width);
            //println!("class: {}", class);

            info.push(sepal_length);
            info.push(sepal_width);
            info.push(petal_length);
            info.push(petal_width);

            let info = Array1::from_vec(info);
            let distance = calculate_distance(item, &info);
            
            distances.push((distance, class));    
        }

        distances.sort_by(|a, b| a.partial_cmp(&b).unwrap());

        let selected_distances: Vec<(f64, String)> = distances.iter().take(k).cloned().collect();


        let mut counter: HashMap<String, usize> = HashMap::new();
        for (_, class) in selected_distances.iter() {
            

            let class = String::from(class);

            if !counter.contains_key(&class) {
                counter.insert(String::from(class), 1);
            }
            else {
                counter.insert(class.clone(), *counter.get(&class.clone()).unwrap() + 1);
            }
        }

        
        if let Some((max_key, max_value)) = counter.iter().max_by_key(|&(_, v)| v) {
            println!("Chave com o maior valor: {}, Valor: {}", max_key, max_value);
        }
    }
}
