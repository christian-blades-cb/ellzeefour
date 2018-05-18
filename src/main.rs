extern crate byteorder;
extern crate failure;

use byteorder::{NativeEndian, ReadBytesExt};
use failure::Error;
use std::io::{Cursor, Read};

fn main() {
    println!("Hello, world!");
    let mut buf = Cursor::new(vec![0x11, 0xfc, 0xca, 0xfe]);
    let blk = block_decode(buf.by_ref()).unwrap();
    println!(
        "literal: {}, offset: {}, len: {}",
        &blk.literal[0], blk.dedup_offset, blk.dedup_length
    );
}

struct Block {
    literal: Vec<u8>,
    dedup_offset: u16,
    dedup_length: usize,
}

fn block_decode<T: Read>(buf: &mut T) -> Result<Block, Error> {
    let mut token = [0; 1];

    buf.read_exact(&mut token)?;

    let t1 = token[0] >> 4;
    let t2 = token[0] & 0x0f;

    let mut literal_length = t1 as usize;
    if t1 == 15 {
        let bufref = buf.by_ref();
        let mut e_1 = [0; 1];
        'e1_read: loop {
            bufref.read_exact(&mut e_1)?;
            literal_length += e_1[0] as usize;
            if e_1[0] != 0xff {
                break 'e1_read;
            }
        }
    }

    let mut literal: Vec<u8> = Vec::with_capacity(literal_length);

    buf.take(literal_length as u64).read_to_end(&mut literal)?;

    let o = buf.read_u16::<NativeEndian>()?;

    let mut dedup_length = t2 as usize;
    if t2 == 0xff {
        let bufref = buf.by_ref();
        let mut e_2 = [0; 1];
        'e2_read: loop {
            bufref.read_exact(&mut e_2)?;
            dedup_length += e_2[0] as usize;
            if e_2[0] != 0xff {
                break 'e2_read;
            }
        }
    }

    Ok(Block {
        literal: literal,
        dedup_offset: o as u16,
        dedup_length: dedup_length,
    })
}
