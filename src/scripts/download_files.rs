use suppaftp::FtpStream;

pub fn download_file(month: String) -> Vec<u8> {
    let path = String::from("/dissemin/publicos/SIASUS/200801_/Dados/");
    let mut ftp_stream =
        FtpStream::connect("ftp.datasus.gov.br:21").unwrap_or_else(|err| panic!("{}", err));
    // ftp_stream.login("test", "test").expect("login error");
    ftp_stream
        .login("anonymous", " anonymous@datasus.gov.br")
        .expect("erro no login");
    ftp_stream.cwd(path).expect("error changing folder");
    let mut file: Vec<u8> = vec![];

    file.append(
        &mut ftp_stream
            .retr_as_buffer(format!("PARS{}.dbc", month).as_str())
            .expect("this file is not on the ftp")
            .into_inner(),
    );
    file
}
