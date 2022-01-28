#[cfg(test)]
mod tests {
    use traverse_big_matrix::{matrix::Matrix, as_bytes::AsBytes};

    #[derive(Clone, Copy)]
    struct TestObj {
        d1: u8,
        d2: u8,
    }

    impl AsBytes for TestObj {
        fn to_bytes(&self) -> Vec<u8>{
            vec![self.d1, self.d2]
        }

        fn from_bytes(bytes: Vec<u8>) -> Self {
            TestObj{
                d1: bytes[0],
                d2: bytes[1],
            }
        }
    }

    #[test]
    fn it_works() {
        let matrix = Matrix::<TestObj>::new(12, 14);
        match matrix {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
    }
}
