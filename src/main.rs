extern crate csv;
extern crate ncurses;

// TODO: Rename rdr variable.
// TODO: Make a test file.

fn main() {
    let rdr = read_csv_from_arg();
    if let Err(e) = rdr {
        eprintln!("Failed to read CSV file: {}", e);
        std::process::exit(1);
    }

    ncurses::initscr();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::refresh();
    for row in rdr.unwrap().into_records() {
        for record in row.unwrap().iter() {
            ncurses::addstr(&format!("{}\n", record));
        }
    }
    ncurses::getch();
    ncurses::endwin();
}

fn read_csv_from_arg() -> std::result::Result<csv::Reader<std::fs::File>, csv::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Specify a file to read!");
        std::process::exit(1);
    }
    csv::Reader::from_path(&args[1])
}
