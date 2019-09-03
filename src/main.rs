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

    // Create buffers for questlogs (includes checksums)
    let mut qlog_1_buffer = &buffer[16456..19164]; 
    let mut qlog_2_buffer = &buffer[19164..21872];
    let mut qlog_3_buffer = &buffer[21872..24580];
    
    // Questlog checksums
    let mut qlog_1_checksum = &buffer[19156..19164];
    let mut qlog_2_checksum = &buffer[21864..21872];
    let mut qlog_3_checksum = &buffer[24572..24580];

    // Shadow copy buffers
    // let qlog_1_shadow_buffer = &buffer[24648..27356];
    // let qlog_2_shadow_buffer = &buffer[27356..30064];
    // let qlog_3_shadow_buffer = &buffer[30064..32772];
    // let qlog_1_shadow_checksum = &buffer[27348..27356];
    // let qlog_2_shadow_checksum = &buffer[30056..32764];
    // let qlog_3_shadow_checksum = &buffer[32764..32772];

    let mut qlog_1_status = check_quest_log(qlog_1_buffer, qlog_1_checksum);
    let mut qlog_1_status = check_quest_log(qlog_2_buffer, qlog_2_checksum);
    let mut qlog_1_status = check_quest_log(qlog_3_buffer, qlog_3_checksum);


}

fn build_checksum(buf: &[u8]) -> Vec<u8> {
    let mut sum: u32 = 0;
    let mut x: u32 = 0;
    let mut done = false; // mut done: bool

    // Additive sum for first four bytes
    for x in 0..2700 {
        sum += (buf[x as usize] as u32);
        //println!("index: {0}, value: {1}", x, sum);
    }   

    let neg_sum: u32 = (sum + 2700).wrapping_neg();
    let mut be_sum = [0u8; mem::size_of::<u32>()];
    be_sum.as_mut()
        .write_u32::<BigEndian>(sum)
        .expect("Unable to write");

    let mut be_neg_sum = [0u8; mem::size_of::<u32>()];
    be_neg_sum.as_mut()
        .write_u32::<BigEndian>(neg_sum)
        .expect("Unable to write");

    for i in &be_sum {
        println!("{:X}", i);
    }

    for i in &be_neg_sum {
        println!("{:X}", i)
    }

    let mut vec_be_sum = be_sum.to_vec();
    let mut vec_be_neg_sum = be_neg_sum.to_vec();
    vec_be_sum.append(&mut vec_be_neg_sum);
    return vec_be_sum;
}

fn check_checksum(chksum_old: &[u8], chksum_new: &[u8]) -> bool {
    if (chksum_old == chksum_new) {
        return true;
    } else { 
        return false; 
    }
}

// fn check_quest_log(quest_log_buffer: &[u8], quest_log_checksum: &[u8]) -> (bool, &[u8]) {
//     // Get checksum and convert from vec<u8> to [u8] for comparison
//     let mut new_checksum = &(build_checksum(quest_log_buffer)[..]);

//     // Compare checksums
//     let mut result = check_checksum(quest_log_checksum, &new_checksum);

//     println!("Checksum: {:?}", &new_checksum);
//     println!("Equivalent?: {:?}", &result);

//     return result, &new_checksum;
// }

fn check_quest_log<'a>(quest_log_buffer: &'a [u8], quest_log_checksum: &'a [u8]) -> (bool, &'a [u8]) {
    // Get checksum and convert from vec<u8> to [u8] for comparison
    let mut new_checksum = &(build_checksum(quest_log_buffer)[..]);

    // Compare checksums
    let mut result = check_checksum(quest_log_checksum, &new_checksum);

    println!("Checksum: {:?}", &new_checksum);
    println!("Equivalent?: {:?}", &result);

    return (result, &new_checksum);
}