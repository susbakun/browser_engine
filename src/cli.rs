use std::fs;

pub fn parse_args() -> (String, String) {
    let mut opts = getopts::Options::new();
    opts.optopt("h", "html", "HTML document", "FILENAME");
    opts.optopt("c", "css", "CSS stylesheet", "FILENAME");

    let matches = opts
        .parse(std::env::args().skip(1))
        .expect("Couldn't parse the args");
    let str_arg = |flag: &str, default: &str| -> String {
        matches.opt_str(flag).unwrap_or(default.to_string())
    };

    let html = read_source(str_arg("h", "./test.html"));
    let css = read_source(str_arg("c", "./test.css"));

    (html, css)
}

fn read_source(file_name: String) -> String {
    fs::read_to_string(file_name).expect("Couldn't read the source")
}
