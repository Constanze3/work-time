pub struct Command {
    pub name: String,
    pub parameters: Vec<ParamTypes>,
    pub description: String,
}

impl Command {
    pub fn from_str(command: &str) -> Result<Command, String> {
        let command = command.trim();
        let mut parts = command.split_whitespace();

        let name = parts.next().ok_or("Invalid command.".to_string())?;
        let name = name.to_string();
        // construct the command here

        let parameters = Vec::<ParamTypes>::new();

        Ok(Command {
            name,
            parameters,
            description: "description".to_string(),
        })
    }
}

pub struct CommandParam {
    name: String,
    description: String,
}

pub enum ParamTypes {
    IntParam(i32, CommandParam),
    StringParam(String, CommandParam),
}
