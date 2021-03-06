use std::io;
use std::fs::File;
use std::path::Path;
use glob::glob;

type MyResult = Result<(), Box<dyn std::error::Error>>;

pub fn run() -> MyResult {
    let stdout = io::stdout();
    let out = io::BufWriter::new(stdout.lock());
    let mut wtr = csv::Writer::from_writer(out);
    let mut is_first_csv = true;
    for csvpath in  get_csvpaths() {
        let csvpath = Path::new(&csvpath);
        let input = File::open(&csvpath)?;
        let input = io::BufReader::new(input);
        let mut rdr = csv::Reader::from_reader(input);
        if is_first_csv {
            let header = rdr.headers()?;
            wtr.write_record(header)?;
            is_first_csv = false;
        }
        for result in rdr.records() {
            let record = result?;
            wtr.write_record(&record)?;
        }
    }
    wtr.flush()?;
    Ok(())
}

fn get_csvpaths() -> Vec<String> {
    let mut csvpaths: Vec<String> = Vec::new();
    let args = std::env::args();
    if args.len() < 2 {
        return csvpaths;
    }
    let mut is_cmd_name = true;
    for arg in args {
        if is_cmd_name {
            is_cmd_name = false;
            continue;
        }
        for entry in glob(&arg).unwrap() {
            match entry {
                Ok(path) => csvpaths.push(path.to_string_lossy().to_owned().to_string()),
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                },
            }
        }
    }
    csvpaths
}
