mod chunk {
    use crate::Error;



    pub struct ChunkType {
        length: u32 ,
        chunk_type: u32,
        chunk_data: Vec<u8>,
        crc: u32,
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