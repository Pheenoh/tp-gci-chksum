use std::fs::File;
use std::io::prelude::*;
//use std:io::Write;
use std::mem;
use byteorder::{BigEndian, WriteBytesExt};

fn main() {
let mut file = File::open("q3_cor.gci").expect("Can't open file!");
    let mut buffer = Vec::new();

    // Read the file into vector
    file.read_to_end(&mut buffer)
        .expect("Couldn't read file.");

    // fix regular quest logs
    fix_quest_log(&mut buffer[16456..19164]);
    fix_quest_log(&mut buffer[19164..21872]);
    fix_quest_log(&mut buffer[21872..24580]);

    // fix shadow copies
    fix_quest_log(&mut buffer[24648..27356]);
    fix_quest_log(&mut buffer[27356..30064]);
    fix_quest_log(&mut buffer[30064..32772]);

    let mut f = File::create("output.gci").expect("Unable to create file");                                                                                                                                                             
    f.write_all(&buffer).expect("Unable to write data");                                                                                                                            
}

fn build_checksum(buf: &[u8]) -> [u8; 8] {
    let mut sum: u32 = 0;
    let mut x: u32 = 0;
    let mut done = false;

    // additive sum for first four bytes
    for x in 0..2700 {
        sum += buf[x as usize] as u32;
    }   

    // negative of additive sum + quest log length (2700 bytes)
    let neg_sum: u32 = (sum + 2700).wrapping_neg();
    let mut be_sum = [0u8; mem::size_of::<u32>()];
    let mut be_neg_sum = [0u8; mem::size_of::<u32>()];

    // convert to big endian
    be_sum.as_mut()
        .write_u32::<BigEndian>(sum)
        .expect("Unable to write");

    be_neg_sum.as_mut()
        .write_u32::<BigEndian>(neg_sum)
        .expect("Unable to write");

    // combine (can be optimized l8r)
    let mut checksum: [u8; 8] = [0; 8];
    checksum[0] = be_sum[0];
    checksum[1] = be_sum[1];
    checksum[2] = be_sum[2];
    checksum[3] = be_sum[3];
    checksum[4] = be_neg_sum[0];
    checksum[5] = be_neg_sum[1];
    checksum[6] = be_neg_sum[2];
    checksum[7] = be_neg_sum[3];

    return checksum;
}

fn fix_quest_log(quest_log_buffer: &mut [u8]) {

    let mut current_checksum = &quest_log_buffer[2700..2708];
    let mut new_checksum = build_checksum(quest_log_buffer);

    // Debug
    println!("Old checksum: {:?}", &current_checksum);
    println!("New checksum: {:?}", &new_checksum);

    // Compare checksums
    {
        if &current_checksum == &new_checksum {
            // do nothing
        } else { 
            // set the new checksum
            quest_log_buffer[2700] = new_checksum[0];
            quest_log_buffer[2701] = new_checksum[1];
            quest_log_buffer[2702] = new_checksum[2];
            quest_log_buffer[2703] = new_checksum[3];
            quest_log_buffer[2704] = new_checksum[4];
            quest_log_buffer[2705] = new_checksum[5];
            quest_log_buffer[2706] = new_checksum[6];
            quest_log_buffer[2707] = new_checksum[7];
        }
    }
}