extern crate csv;

fn main() {
    let mut rdr = read_csv_from_arg().unwrap();

    for result in rdr.records() {
        let record = result.expect("a CSV record");

        println!("{:?}", record);
    }
}

fn read_csv_from_arg() -> std::result::Result<csv::Reader<std::fs::File>, csv::Error> {
    let args: Vec<String> = std::env::args().collect();
    csv::Reader::from_path(&args[1])
}
