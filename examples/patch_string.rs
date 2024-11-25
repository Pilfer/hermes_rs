use hermes_rs::{HermesFile, StringKind};

use std::io::{Cursor, Read, Write};
use std::io::{Seek, SeekFrom};

use std::{fs::File, io};

fn main() {
    /**
     * This file shows how to patch a specific string within a HBC file.
     * One might use this functionality to change the hostname of an API endpoint
     * to proxy it to the original host. There's obviously a bunch of use cases
     * here.
     */
    const FILENAME: &'static str = "./index.android.bundle";

    let f: File = File::open(FILENAME).expect("no file found");
    let mut reader = io::BufReader::new(f);

    let mut hermes_file = HermesFile::deserialize(&mut reader);

    let strings = hermes_file.get_strings_by_kind();

    let mut final_pairs = vec![];

    for mut pair in strings {
        match pair.kind {
            StringKind::String => {
                if pair.string == "https://jsonplaceholder.typicode.com/todos/1" {
                    pair.string = "https://httpbin.org/anything?example".to_string();
                }
                final_pairs.push(pair);
            }
            StringKind::Identifier => {
                final_pairs.push(pair);
            }
            StringKind::Predefined => {
                final_pairs.push(pair);
            }
        }
    }

    hermes_file.set_string_pairs_unordered(final_pairs);

    _ = hermes_file.get_instructions();

    println!("File length is: {:?}", hermes_file.header.file_length);
    println!("Header: {:?}", hermes_file.header);

    let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    println!("Serializing...");
    hermes_file.serialize(&mut writer);
    println!("Done serializing...");

    let mut file: File = File::create("patched_strings.hbc").expect("unable to create file");
    writer
        .seek(SeekFrom::Start(0))
        .expect("unable to seek to start");

    println!("Writing to file...");

    file.write_all(&writer.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>())
        .expect("unable to write to file");

    println!("Done writing to file...");

    println!("File 'patched_strings.hbc' was created.");
}
