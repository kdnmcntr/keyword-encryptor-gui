use std::io;
// use std::env;
use std::fs::File;
use std::thread;
use std::sync::Arc;
use sha2::{Sha256, Digest};
//use std::time::{Instant, Duration};
use rand::{SeedableRng, RngCore};
use rand_chacha::ChaCha20Rng;
use io_at::{ReadAt, WriteAt};
//use indicatif::{ProgressBar, ProgressStyle};

fn hash_string(keyword: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(keyword.as_bytes());
    let result = hasher.finalize();
    u64::from_be_bytes(result[0..8].try_into().unwrap())
}

pub struct Encryptor{
    input_file: String,
    output_file: String,
    keyword: String,
}

impl Encryptor {
    pub fn new(input_file: String, output_file: String, keyword: String) -> Self {
        Self {input_file, output_file, keyword}
    }

    pub fn encrypt_file(&self) -> io::Result<()> {

        let source: &str = &self.input_file;
        let keyword: &str = &self.keyword;
        let output: &str = &self.output_file;

        let source_file = File::options().read(true).open(source)?;
        let output_file = File::create(output)?;

        let file_size = source_file.metadata()?.len();

        output_file.set_len(file_size)?;

        let seed: u64 = hash_string(keyword);
        let mut rng = ChaCha20Rng::seed_from_u64(seed);

        let segment_size: u64 = 1048576; // 1 MB 1048576, 1 KB 1024
        let number_of_segments: u64 = file_size / segment_size + 1;

        let mut handles = vec![];
        let mut new_seeds = vec![];

        let source_file = Arc::new(source_file);
        let output_file = Arc::new(output_file);

        for i in 0..number_of_segments {
            new_seeds.push(rng.next_u64());
            let new_seeds_clone = new_seeds.clone();

            let source_file_clone = Arc::clone(&source_file);
            let output_file_clone = Arc::clone(&output_file);

            let handle = thread::spawn(move || {


                let mut new_rng = ChaCha20Rng::seed_from_u64(new_seeds_clone[i as usize]);

                let mut buffer = [0u8; 8];
                for j in 0..(segment_size / 8) {
                    let num_bytes_read = source_file_clone.read_at(&mut buffer, (segment_size * i) + (j * 8)).unwrap(); // read 8 bytes from the input file, store in buffer

                    if num_bytes_read == 0 {
                        break;
                    }

                    let read_data = u64::from_be_bytes(buffer); // convert buffer of 8 bytes into u64
                    let rand_data = new_rng.next_u64(); // generate a new u64
                    let new_data = read_data ^ rand_data; // xor the two u64s
                    buffer = new_data.to_be_bytes(); // convert resulting u64 back into array of 8 bytes
                    output_file_clone.as_ref().write_at(&buffer, (segment_size * i) + (j * 8)).unwrap(); // write buffer to the output file
                    buffer = [0u8; 8]; // reset buffer
                }

            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        Ok(())

    }

}


// fn format_time(time: Duration) -> String {
//     let mut secs = time.as_secs();
//     if secs < 1 {
//         return String::from(time.as_millis().to_string() + " milliseconds");
//     } else if secs < 60 {
//         return String::from(secs.to_string() + " seconds");
//     } else {
//         let mut mins = secs / 60;
//         secs = secs % 60;
//
//         if mins < 60 {
//             return String::from(mins.to_string() + " minutes, " + &secs.to_string() + " seconds");
//         } else {
//             let hours = mins / 60;
//             mins = mins % 60;
//             return String::from(hours.to_string() + " hours, " + &mins.to_string() + " minutes," + &secs.to_string() + " seconds");
//         }
//     }
// }