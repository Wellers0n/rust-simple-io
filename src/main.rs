use std::{
    collections::HashMap,
    env::Args,
    fs::{read, read_to_string, write},
    io::Error,
    iter::Skip,
};

fn main() {
    let mut arguments: Skip<Args> = std::env::args().skip(1);
    let key: String = arguments.next().expect("Key was not there");
    let value: String = arguments.next().unwrap();

    print!("The key is '{}' and the value is '{}'", key, value);

    let contents: String = format!("{}\t{}\n", key, value);

    write("database.db", contents).unwrap();

    let mut database: Database = Database::new().expect("Creating db failed");

    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value)
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, Error> {
        let mut map = HashMap::new();
        let contents = match read_to_string("database.db") {
            Ok(contents) => contents,
            Err(error) => {
                return Result::Err(error);
            }
        };

        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("ERROR: No key or value!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { map: map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
}
