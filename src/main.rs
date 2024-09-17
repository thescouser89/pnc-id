use base32::Alphabet;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    id: String,
}

fn encode_base32(pnc_id: u64) -> String {
    base32::encode(Alphabet::Rfc4648 { padding: false }, &pnc_id.to_be_bytes())
}

fn decode_base32(pnc_id_encoded: &str) -> u64 {

    let value = base32::decode(Alphabet::Rfc4648 { padding: false }, pnc_id_encoded).unwrap();

    let mut array = [0; 8];
    let mut count = 0;

    // transform Vec to array
    for item in value {
        array[count] = item;
        count = count + 1;
    }

    // Big endian:: consider array as a u64
    u64::from_be_bytes(array)
}

fn main() {

    let args = Args::parse();

    // if id can be parsed to a u64, then it's probably a long that needs to be converted to a base32 string
    // else: parse the string as a base32 encoded string that needs to be decoded into a u64
    let output = match args.id.parse() {
        Ok(number) => encode_base32(number),
        Err(_) => decode_base32(&args.id).to_string(),
    };

    println!("{}", output);
}
