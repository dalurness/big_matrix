use std::io::{Write, Read};
use std::str;
use unqlite::UnQLite;

pub struct DB {
    connection: UnQLite,
}

impl DB {
    pub fn new() -> Self  {
        DB {
            connection: UnQLite::create_temp()
        }
    }
    pub fn save_cell(&self, x: usize, y:usize, value: Vec<u8>) -> Result<(), &'static str> {
        self.connection.kv_store(
            x.to_string() + "," + y.to_string(),
            str::from_utf8(&value).unwrap()
        );
        Ok(())
    }
}