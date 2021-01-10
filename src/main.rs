use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(())
    }

    /**
     * Non JSON save
     **/
    // fn save(self) -> Result<(), std::io::Error> {
    //     let mut content = String::new();

    //     for (k, v) in self.map {
    //         let record = format!("{}\t{}\n", k, v);

    //         content.push_str(&record);
    //     }

    //     std::fs::write("db.txt", content)
    // }

    /**
     * Non JSON new
     **/
    // fn new() -> Result<Todo, std::io::Error> {
    //     let mut f = std::fs::OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .read(true)
    //         .open("db.txt")?;

    //     let mut content = String::new();

    //     f.read_to_string(&mut content)?;

    //     let map: HashMap<String, bool> = content
    //         .lines()
    //         .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
    //         .map(|v| (v[0], v[1]))
    //         .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
    //         .collect();

    //     Ok(Todo { map })
    // }

    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occured: {}", e),
        }
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please Specify an action");
    let item = std::env::args().nth(2).expect("Please Specify an item");

    let mut todo = Todo::new().expect("Initialization Failed");

    if action == "add" {
        todo.insert(item);

        match todo.save() {
            Ok(_) => println!("Saved"),
            Err(why) => println!("Error occured {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the database", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved"),
                Err(why) => println!("An error occured: {}", why),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]

    fn new() {
        use crate::Todo;
        let todo = Todo::new().unwrap();

        assert_eq!(todo.map.len(), 0, "Map is empty");
    }

    #[test]
    fn add() {
        use crate::Todo;

        let mut todo = Todo::new().unwrap();

        todo.insert(String::from("Something"));

        let value = todo.map.get("Something").unwrap();

        assert_eq!(*value, true, "Value is not saved");
    }

    #[test]
    fn complete() {
        use crate::Todo;
        let mut todo = Todo::new().unwrap();
        todo.insert(String::from("Something"));

        todo.complete(&String::from("Something"));

        let value = todo.map.get("Something").unwrap();

        assert_eq!(*value, true, "Item is not completed");
    }
}
