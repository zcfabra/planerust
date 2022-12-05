pub mod chunk {
    use std::{fmt, str::FromStr, io::Read};

    use crate::Error;

    pub struct ChunkType {
        length: u32 ,
        chunk_type: u32,
        chunk_data: Vec<u8>,
        crc: u32,
    }

    impl ChunkType{
        pub fn bytes(&self)->[u8; 4]{
            let out= &self.chunk_data;
            let mut arr = [0u8; 4];

            for (ix, each) in out.iter().enumerate(){
                arr[ix] = *each;
            }

            return arr;
    
        }

        pub fn is_valid(&self)->bool{
            let valid_bit = self.chunk_data[1];
            return valid_bit == 1;

        }
    }

    impl FromStr for ChunkType{
        type Err = Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let vec = s.as_bytes().to_vec();
            return Ok(ChunkType { length: vec.len() as u32, chunk_type: 2, chunk_data: vec, crc: 1 });
        }
    }

    impl fmt::Display for ChunkType{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"{:?}", self.chunk_data)
        }
    }

    impl PartialEq for ChunkType{
        fn eq(&self, other: &ChunkType)->bool{
            return self.chunk_data == other.chunk_data;
        }
    }
    impl Eq for ChunkType{

    }

    impl TryFrom<[u8; 4]> for ChunkType{
        type Error = Error;
         fn try_from(bytes_to_try: [u8;4])->Result<ChunkType, Error>{
            let bytes_vec = bytes_to_try.to_vec();
            let out = ChunkType{
                length: 4,
                chunk_data: bytes_vec,
                chunk_type: 1,
                crc:1,
            };
            return Ok(out);
        }
    }
}