use std::{collections::HashMap, io::Error};
use crate::{as_bytes::AsBytes, db::db::DB};

const MAX_INTERNAL_WIDTH: u8 = 10;

pub struct Matrix<T: AsBytes + Copy>{
    // to user
    x_size: usize,
    y_size: usize,

    // underlying structure
    matrix: HashMap<(usize, usize), T>,
    // max size before the in-memory map begins to be clear
    //max_size: u16,
    db: DB,
}

impl<T: AsBytes + Copy> Matrix<T> {
    pub fn new(x_size: usize, y_size:usize) -> Result<Self, Error> {
        let db = DB::new();
        let db = match db {
            Ok(db) => db,
            Err(_) => panic!("Failed to initialize database"),
        };

        Ok(Matrix{
            matrix: HashMap::new(),
            x_size,
            y_size,
            db
        })
    }

    fn fetch_value(x: usize, y: usize) -> T {
        let returned_bytes: [u8; 20] = [0; 20];
        
        return T::from_bytes(&returned_bytes);
    }

    pub fn get(&mut self, index: (usize, usize)) -> T {

        if index.0 >= self.x_size || index.1 >= self.y_size {
            panic!("Out of range");
        }

        self.matrix.entry((index.0, index.1)).or_insert_with(|| Matrix::<T>::fetch_value(index.0, index.1)).clone()

        // match &self.matrix.get(&(index[0], index[1])).unwrap() {
        //     Some(x) => x,
        //     None => {
        //         let value: T = Matrix::fetch_value(index[0], index[1]);
        //         self.matrix.insert((index[0], index[1]), Some(value)).unwrap();
        //         match &self.matrix.get(&(index[0], index[1])).unwrap() {
        //             None => panic!("Error getting object from hashmap that was just put in there!"),
        //             Some(x) => return x,
        //         }
        //     },
        // }
    }
}
