extern crate csv;

fn main() {
    let mut rdr = csv::Reader::from_reader(std::io::stdin());

    for result in rdr.records() {
        let record = result.expect("a CSV record");

        println!("{:?}", record);
    }
}
