use std::collections::HashMap;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert("Ancient Roman History".to_string(), "Very accurate.".to_string());
    reviews.insert("Cooking with Rhubarb".to_string(), "Sweet recipes.".to_string());
    reviews.insert("Programming in Rust".to_string(), "Great examples.".to_string());

    let ireviews: HashMap<String, String> =
    [("Programming in Rust".to_string(), "Great examples.".to_string()),
     ("Ancient Roman History".to_string(), "Very accurate.".to_string()),
     ("Cooking with Rhubarb".to_string(), "Sweet recipes.".to_string())]
    .iter().cloned().collect();

    let book: &str = "Programming in Rust";
    let result = reviews.get(book);
    if result.is_none() {
        println!("Could not find book {}.", book);
    } else {
        println!("\nReview for \'{}\': {:?}", book, result);
    }
    
    let result = ireviews.get(book);
    if result.is_none() {
        println!("Could not find book {}.", book);
    } else {
        println!("\nReview for \'{}\': {:?}", book, result);
    }

    let non_book: &str = "Programming in Python";
    let result = reviews.get(non_book);
    if result.is_none() {
        println!("Could not find book {}.", non_book);
    } else {
        println!("\nReview for \'{}\': {:?}", non_book, result);
    }

    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }
}