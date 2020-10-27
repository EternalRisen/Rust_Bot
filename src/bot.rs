pub struct Bot {
    pub token: String,
    pub prefix: String,
}

impl Bot {
    pub fn new(tkn: &str, pfx: &str) -> Bot {
        Bot { token: tkn.to_owned(), prefix: pfx.to_owned()}
    }
}
