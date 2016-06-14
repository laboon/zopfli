use std::env;
use std::io::prelude::*;
use std::fs::File;

extern crate zopfli;

fn main() {
    let options = zopfli::Options::default();
    let output_type = zopfli::Format::Gzip;

    // TODO: CLI arguments
    // TODO: Allow specifying output to STDOUT

    let extension = match output_type {
        zopfli::Format::Gzip => ".gz",
        zopfli::Format::Zlib => ".zlib",
        zopfli::Format::Deflate => ".deflate",
    };

    for filename in env::args().skip(1) {
        let mut file = File::open(&filename)
            .unwrap_or_else(|why| panic!("couldn't open {}: {}", filename, why));
        let filesize = file.metadata().map(|x| x.len()).unwrap_or(0);

        let mut data = Vec::with_capacity(filesize as usize);
        // Read the contents of the file into in_data; panic if the file could not be read
        file.read_to_end(&mut data)
            .unwrap_or_else(|why| panic!("couldn't read {}: {}", filename, why));

        let mut out_data = vec![];
        let out_filename = format!("{}{}", filename, extension);

        // Attempt to create the output file, panic if the output file could not be opened
        let mut out_file = File::create(&out_filename)
            .unwrap_or_else(|why| panic!("couldn't create output file {}: {}", out_filename, why));

        zopfli::compress(&options, &output_type, &data, &mut out_data);

        // Write the `out_data` into the newly created file.
        out_file.write_all(&out_data)
            .unwrap_or_else(|why| panic!("couldn't write to output file {}: {}", out_filename, why));
    }
}
