use suppaftp::FtpStream;

pub fn download_files(_dates: Vec<String>) -> Vec<Vec<u8>> {
    let path = String::from("/dissemin/publicos/SIASUS/200801_/Dados/");
    let mut ftp_stream =
        FtpStream::connect("ftp.datasus.gov.br:21").unwrap_or_else(|err| panic!("{}", err));
    // ftp_stream.login("test", "test").expect("login error");
    let dates = vec!["2307".to_string()];
    ftp_stream
        .login("anonymous", " anonymous@datasus.gov.br")
        .expect("erro no login");
    ftp_stream.cwd(path).expect("error changing folder");
    let mut files: Vec<Vec<u8>> = vec![];
    dates.iter().for_each(|_x| {
        files.push(
            ftp_stream
                .retr_as_buffer("PARS2307.dbc")
                .unwrap()
                .into_inner(),
        )
    });
    files
}
