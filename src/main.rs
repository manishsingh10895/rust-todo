use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};
use std::collections::HashMap;
use std::error::Error;
use tokio;
mod database;
mod settings;
mod todo;

async fn setup_db() -> Result<(), Box<dyn Error>>{
    let db = database::Database::init().await?;



    Ok(())
}

fn main() {

    setup_db();

    let action = std::env::args().nth(1).expect("Please Specify an action");
    let item = std::env::args().nth(2).expect("Please Specify an item");

    let mut todo = todo::Todo::new().expect("Initialization Failed");

    let conf = settings::Settings::new().unwrap();

    println!("{:?}", conf);

    // if action == "add" {
    //     todo.insert(item);

    //     match todo.save() {
    //         Ok(_) => println!("Saved"),
    //         Err(why) => println!("Error occured {}", why),
    //     }
    // } else if action == "complete" {
    //     match todo.complete(&item) {
    //         None => println!("'{}' is not present in the database", item),
    //         Some(_) => match todo.save() {
    //             Ok(_) => println!("Todo saved"),
    //             Err(why) => println!("An error occured: {}", why),
    //         },
    //     }
    // }
}

#[cfg(test)]
mod tests {
    #[test]

    fn new() {
        use crate::todo::Todo;

        let todo = Todo::new().unwrap();

        assert_eq!(todo.map.len(), 0, "Map is empty");
    }

    #[test]
    fn add() {
        use crate::todo::Todo;

        let mut todo = Todo::new().unwrap();

        todo.insert(String::from("Something"));

        let value = todo.map.get("Something").unwrap();

        assert_eq!(*value, true, "Value is not saved");
    }

    #[test]
    fn complete() {
        use crate::todo::Todo;
        let mut todo = Todo::new().unwrap();

        todo.insert(String::from("Something"));

        todo.complete(&String::from("Something"));

        let value = todo.map.get("Something").unwrap();

        assert_eq!(*value, true, "Item is not completed");
    }
}
