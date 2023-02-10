use reqwest;

struct Parser {
    url: String,
}

impl Parser {
    fn new(url: &str) -> Self {
        Parser {
            url: url.to_owned(),
        }
    }

    fn get_disassembly(&self) -> Result<String, reqwest::Error>{

    }
}