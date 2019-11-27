extern crate csv;
extern crate ncurses;

// TODO: Make a test file.
// TODO: Split main function into several functions.
fn main() {
    let csv_file = read_csv_from_arg();
    if let Err(e) = csv_file {
        eprintln!("Failed to read CSV file: {}", e);
        std::process::exit(1);
    }

    init_ncurses();

    for row in csv_file.unwrap().into_records() {
        ncurses::clear();
        let row = row.unwrap();
        for record in row.iter() {
            ncurses::addstr(&format!("{}\n", record));
        }

        for letter in row[1].chars() {
            // TODO: Remove duplication.
            let mut input_letter = ncurses::getch();
            while input_letter != letter as i32 {
                input_letter = ncurses::getch();
            }
            ncurses::addch(letter as u32);
        }
    }
    ncurses::endwin();
}

fn init_ncurses() -> () {
    ncurses::initscr();
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::refresh();
}

fn read_csv_from_arg() -> std::result::Result<csv::Reader<std::fs::File>, csv::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Specify a file to read!");
        std::process::exit(1);
    }
    csv::Reader::from_path(&args[1])
}
