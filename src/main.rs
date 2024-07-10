use std::error::Error;

use crate::knn::knn::find_knn;
mod knn;

use csv::ReaderBuilder;
use ndarray::Array1;

fn main() -> Result<(), Box<dyn Error>> {

    // Defining headers
    // let headers = vec!["SepalLength", "SepalWidth", "PetalLength", "PetalWidth", "Class"];
   
    // Defining the reader
    let mut reader = ReaderBuilder::new().from_path("iris.csv")?;

    find_knn(100, &Array1::from_vec(vec![6.1,2.8,5.0,1.8]), &mut reader);

   Ok(())

}
