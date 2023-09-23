mod decompress;
mod download_files;
mod interpret_file;

use dbase::FieldValue;
use decompress::decompress;
use download_files::download_file;
use interpret_file::interpret_file;

pub fn get_file(
    month: String,
    variables: Vec<String>,
    temp_path: std::path::PathBuf,
    output: std::path::PathBuf,
) {
    let mut database = interpret_file(temp_path, decompress(&download_file(month.clone())));

    let mut wtr = csv::Writer::from_path(format!("{}{}.csv", output.display(), month))
        .expect("error on output file");
    csv::Writer::from_writer(std::io::stdout());

    wtr.write_record(variables.clone())
        .expect("error creating header");

    for record_result in database.iter_records() {
        // println!("Record {}", i);
        let record = record_result.expect("error reading result");
        let mut line: Vec<&FieldValue> = vec![];
        for i in variables.clone() {
            let value = record
                .get(i.as_str())
                .expect("error getting value from result");
            line.push(value);
        }

        wtr.write_record(line.iter().map(|x| match x {
            dbase::FieldValue::Character(Some(x)) => x.to_string(),
            dbase::FieldValue::Numeric(Some(x)) => x.to_string(),
            _ => "oops".to_string(),
        }))
        .expect("error writing record");
    }

    wtr.flush().expect("error flushing to csv");
}
