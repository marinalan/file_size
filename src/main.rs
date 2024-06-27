use std::env;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn new (size:u64) -> Sizes {
        let bytes= size;
        let kilobytes = size / 1000;
        let megabytes = size / 1_000_000;
        let gigabytes = size / 1_000_000_000;
        Sizes {
            bytes: format!("{} bytes", bytes),
            kilobytes: format!("{} kilobytes", kilobytes),
            megabytes: format!("{} megabytes", megabytes),
            gigabytes: format!("{} gigabytes", gigabytes),
        }
    }
}
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}


fn main() {
    let result = format_size(6888837399);
    println!("{}", result);

    let args: Vec<String> = env::args().collect();
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    let file_size = if args.len() > 1 { &args[1] } else { "300kb" };

    println!("Size is: {}.", file_size);

    let file_size = file_size.to_lowercase();
    let parts:Vec<&str> = file_size.split_whitespace().collect();
    println!("{:?}", parts);

    let size: u64 = parts[0].parse().expect("Not a valid integer number");
    println!("got size {}", size);
    let size_in_bytes: u64 = match parts[1] {
        "bytes" => size,
        "kb" => size * 1000, //1024,
        "mb" => size * 1_000_000, //1_048_576,
        "gb" => size * 1_000_000_000, //1_073_741_824,
        _ => panic!("Invalid size specfier")
    };
    println!("entered size for {}", format_size(size_in_bytes));
    let sizes:Sizes = Sizes::new(size_in_bytes);
    println!("{:?}", sizes);
}
