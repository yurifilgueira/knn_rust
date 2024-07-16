use std::{error::Error, io::{self, Write}};

use crate::knn::knn::find_knn;
mod knn;

use csv::ReaderBuilder;
use ndarray::Array1;

fn main() -> Result<(), Box<dyn Error>> {

    let mut reader = ReaderBuilder::new().from_path("iris.csv")?;

    let (sepal_length, sepal_width, petal_length, petal_width) = get_input_values()?;
    
    let sepal_length = sepal_length.trim().parse::<f64>()?;
    let sepal_width = sepal_width.trim().parse::<f64>()?;
    let petal_length = petal_length.trim().parse::<f64>()?;
    let petal_width = petal_width.trim().parse::<f64>()?;

    find_knn(6, &Array1::from_vec(vec![sepal_length, sepal_width, petal_length, petal_width]), &mut reader);

    Ok(())
}


fn get_input_values() -> Result<(String, String, String, String), Box<dyn Error>> {
    let mut sepal_length = String::new();
    let mut sepal_width = String::new();
    let mut petal_length = String::new();
    let mut petal_width = String::new();

    print!("Sepal length: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut sepal_length)?;

    print!("Sepal width: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut sepal_width)?;

    print!("Petal length: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut petal_length)?;

    print!("Petal width: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut petal_width)?;

    Ok((sepal_length.trim().to_string(), sepal_width.trim().to_string(), petal_length.trim().to_string(), petal_width.trim().to_string()))
}

