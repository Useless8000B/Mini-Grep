pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, &'static str> {
        Ok(Self {
            query: &args[1],
            file_path: &args[2],
        })
    }
}
