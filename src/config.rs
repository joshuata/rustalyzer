use clap::{App, Arg};

pub fn set_args<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.author(crate_authors!())
        .version(crate_version!())
        .about("A rust program analysis framework")
        .arg(Arg::with_name("output-file").short("o").help("The output filename.").required(true))
        .arg(Arg::with_name("output-type")
                 .short("t")
                 .help("Sets the output file type. Default (and currently only) type is datalog.")
                 .default_value("datalog"))
}
