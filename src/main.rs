extern crate csv;

fn main() {
    let rdr = read_csv_from_arg();
    if let Err(e) = rdr {
        eprintln!("Failed to read CSV file: {}", e);
        std::process::exit(1);
    }

    for result in rdr.unwrap().records() {
        let record = result.expect("a CSV record");

        println!("{:?}", record);
    }
}

fn read_csv_from_arg() -> std::result::Result<csv::Reader<std::fs::File>, csv::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Specify a file to read!");
        std::process::exit(1);
    }
    csv::Reader::from_path(&args[1])
}
