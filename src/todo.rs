use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub _id: String,
    pub name: String,
    pub completed: bool 
}

pub struct Todo {
    pub map: HashMap<String, bool>,
}

impl Todo {
    pub fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    pub fn save(self) -> Result<(), Box<dyn std::error::Error>> {
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

    pub fn new() -> Result<Todo, std::io::Error> {
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

    pub fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
