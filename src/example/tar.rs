use std::borrow::Cow;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Archive;

fn basename<'a>(path: &'a str, sep: char) -> Cow<'a, str> {
    let mut pieces = path.rsplit(sep);
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}

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
    let file_name = basename(path.to_str().unwrap(), '/').to_string() + ".tar.gz";
    let tar_path = path.parent().unwrap().join(file_name);
    match File::create(tar_path) {
        Ok(tar_gz) => {
            let enc = GzEncoder::new(tar_gz, Compression::default());
            let mut tar = tar::Builder::new(enc);
            tar.append_dir_all(".", path).unwrap();
            Ok(())
        }
        Err(err) => Err(err),
    }
}
