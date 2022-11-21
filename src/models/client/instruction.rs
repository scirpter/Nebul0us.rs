pub struct Instruction<'a> {
    pub command: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> Instruction<'a> {
    pub fn new(command: &'a str, args: Option<Vec<&'a str>>) -> Self {
        Instruction {
            command,
            args: args.unwrap_or_default(),
        }
    }
}
