extern crate md5;
use std::thread;

fn main() {
    let num_threads: u32 = 8;
    let upass1: u16 = 52868;
    let upass2: u16 = 198;

    let num_per_thread: u32 = 0xFFFFFFFF/num_threads;

    let mut children = vec![];
    for threadnum in 0..num_threads {
        let start = threadnum * num_per_thread;
        let mut end = start + num_per_thread;
        if threadnum == num_threads-1 {
            end = 0xFFFFFFFF;
        }
        children.push(thread::spawn(move || {
            // println!("this is thread number {}:{}:{}", threadnum, start, end);
            brute_pins(start, end, upass1, upass2);
        }));
    }
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}

fn brute_pins(start: u32, end: u32, upass1: u16, upass2: u16) {
    for seed in start..end as u32 {
        //let bytes: [u8; 4] = unsafe { transmute(seed.to_be()) };
        let bytes = transform_u32_to_array_of_u8(seed);
        let digest = md5::compute(bytes);
        let mut acstack444 = digest.to_vec();
        for _ in 0..4{
            acstack444.push(0u8);
        }
        // do the funky bitwise ops here
        let local_8c = bitwise_ops(digest);
        //check the user passwords
        let tpw1 = local_8c[0] as u16 + ((local_8c[1] as u16) << 8);
        let tpw2 = local_8c[2] as u16 + ((local_8c[3] as u16) << 8);
        if tpw1 == upass1 && tpw2 == upass2 {
            // if the user passwords matched, you can check the admin passwords here
            let adminpass1 = local_8c[4] as u16 + ((local_8c[5] as u16) << 8);
            let adminpass2 = local_8c[6] as u16 + ((local_8c[7] as u16) << 8);
            println!("Possible candidate: {:?}", seed);
            println!("Try: {}, {}", adminpass1, adminpass2);
        } 
    }
}

fn bitwise_ops(digest: md5::Digest) -> [u8;8] {
    let mut local_8c: [u8;8] = [0;8];
    for local_68 in 0..0x10 as usize {
        let uvar1 = (local_68 >> 0x1f) >> 0x1d;
        let ivar5 = (local_68 + uvar1 & 7) - uvar1;
        local_8c[ivar5] = local_8c[ivar5] ^ digest[local_68];
    }
    return local_8c;
}

fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b4, b3, b2, b1]
}
