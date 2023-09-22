mod decompress;
mod download_files;
mod interpret_file;

use dbase::FieldValue;
use decompress::decompress;
use download_files::download_files;
use interpret_file::interpret_file;

pub fn get_files(_years: Vec<u8>, _months: Vec<u8>) {
    let variables = vec![
        "PA_CODUNI",
        "PA_CBOCOD",
        "PA_MUNPCN",
        "PA_MVM",
        "PA_CMP",
        "PA_PROC_ID",
        "PA_NIVCPL",
        "PA_OBITO",
        "PA_ENCERR",
        "PA_PERMAN",
        "PA_ALTA",
        "PA_TRANSF",
        "PA_CIDPRI",
        "PA_IDADE",
        "PA_QTDPRO",
    ];
    let mut database = interpret_file(decompress(&download_files(vec!["01".to_string()])[0]));

    let mut wtr = csv::Writer::from_path("./test_files/test.csv").unwrap();
    csv::Writer::from_writer(std::io::stdout());

    wtr.write_record(variables.clone()).unwrap();

    for record_result in database.iter_records() {
        // println!("Record {}", i);
        let record = record_result.unwrap();
        let mut line: Vec<&FieldValue> = vec![];
        for i in variables.clone() {
            let value = record.get(i).unwrap();
            line.push(value);
        }

        wtr.write_record(line.iter().map(|x| match x {
            dbase::FieldValue::Character(Some(x)) => x.to_string(),
            dbase::FieldValue::Numeric(Some(x)) => x.to_string(),
            _ => "oops".to_string(),
        }))
        .unwrap();
    }

    wtr.flush().unwrap();
}
