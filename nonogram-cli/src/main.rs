use std::{env, error::Error, fs::File, io::BufReader, path::Path, time::Instant};

use log::info;
use nonogram::{schema::NonogramSchema, solver::solve};
use serde_derive::Deserialize;
use serde_json::from_reader;
use simple_logger::init_with_env;

#[derive(Deserialize)]
struct NonogramJson {
    row_labels: Vec<Vec<usize>>,
    col_labels: Vec<Vec<usize>>,
    rows: usize,
    cols: usize,
}

fn read_schema<P: AsRef<Path>>(path: P) -> Result<NonogramSchema, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json_schema: NonogramJson = from_reader(reader)?;
    let schema = NonogramSchema::new(
        json_schema.rows,
        json_schema.cols,
        json_schema.row_labels,
        json_schema.col_labels,
    )?;
    Ok(schema)
}

fn main() {
    init_with_env().unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Expected file name");
    }

    let mut schema = read_schema(&args[1]).unwrap();

    let now = Instant::now();
    solve(&mut schema);
    let duration = now.elapsed().as_millis();

    info!("Solved in {} ms", duration);
    schema.print_solved();
}
