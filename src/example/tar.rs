use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use flate2::bufread::{GzDecoder, GzEncoder};
use flate2::Compression;
use tar::Archive;

pub fn decode_file(p: &str) -> Result<(), std::io::Error> {
    let path = Path::new(p);
    match File::open(path) {
        Ok(tar_gz) => {
            let tar = GzDecoder::new(BufReader::new(tar_gz));
            let mut archive = Archive::new(tar);
            archive.unpack(".")
        }
        Err(err) => Err(err),
    }
}

pub fn tar_file(p: &str) -> Result<(), std::io::Error> {
    let path = Path::new(p);
    match File::create(path) {
        Ok(tar_gz) => {
            let buf_reader = BufReader::new(tar_gz);
            println!("{:?}", buf_reader);
            let enc = GzEncoder::new(buf_reader, Compression::default());
            let mut tar = Archive::new(enc);

            Ok(())
        }
        Err(err) => Err(err),
    }
}
