extern crate md5;
extern crate clap;
use clap::{Arg, App};
use std::thread;
use std::time::{SystemTime};

fn main() {
    let matches = App::new("UnikeyBruteForcer")
        .version("0.1.0")
        .author("Jeremy Mill <jeremymill@gmail.com>")
        .about("https://github.com/LivingInSyn/UnikeyBruteForcer")
        .arg(Arg::with_name("upass1")
            .help("User password 1")
            .required(true))
        .arg(Arg::with_name("upass2")
            .help("User password 2")
            .required(true))
        .arg(Arg::with_name("threads")
            .help("Number of threads to run with"))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true))
        .get_matches();
    // num threads
    let num_threads = matches.value_of("threads").unwrap_or("8").parse::<u32>().unwrap();
    let upass1: u16 = matches.value_of("upass1").unwrap().parse::<u16>().unwrap();
    let upass2: u16 = matches.value_of("upass2").unwrap().parse::<u16>().unwrap();
    let verbose: bool = match matches.occurrences_of("v") {
        0 => false,
        _ => true
    };
    let num_per_thread: u32 = 0xFFFFFFFF/num_threads;

    let now = SystemTime::now();

    let mut children = vec![];
    for threadnum in 0..num_threads {
        let start = threadnum * num_per_thread;
        let mut end = start + num_per_thread;
        if threadnum == num_threads-1 {
            end = 0xFFFFFFFF;
        }
        children.push(thread::spawn(move || {
            // println!("this is thread number {}:{}:{}", threadnum, start, end);
            brute_pins(start, end, upass1, upass2, threadnum, verbose);
        }));
    }
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    match now.elapsed() {
       Ok(elapsed) => {
           // it prints '2'
           println!("Finished in: {}", elapsed.as_secs());
       }
       Err(e) => {
           // an error occurred!
           println!("Error: {:?}", e);
       }
   }
}

fn brute_pins(start: u32, end: u32, upass1: u16, upass2: u16,
              thread: u32, status: bool) {
    //let mut qs: [bool;4] = [false;4];
    for seed in start..end as u32 {
        if status && thread == 0 && (end - seed) % 1000000 == 0 {
            let perc = ((seed - start) as f32 / (end-start) as f32)*100f32;
            println!("Percent: {:.20}", perc);
        }
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
