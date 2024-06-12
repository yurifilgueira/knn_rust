use std::error::Error;

use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {

    // Defining headers
    // let headers = vec!["SepalLength", "SepalWidth", "PetalLength", "PetalWidth", "Class"];
   
    // Defining the reader
    let mut reader = ReaderBuilder::new()
        .from_path("iris.csv")?;
     
    for rec in reader.records() {
        println!("{:?}", rec?);
    }

    Ok(())

}
