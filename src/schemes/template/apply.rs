enum Instructions {
    /// Make whole value uppercase
    Uppercase,

    /// Make whole value lowercase
    Lowercase,

    /// If the function name is not recognized
    Unknown,
}

impl<T> From<T> for Instructions
where
    T: ToString,
{
    fn from(value: T) -> Self {
        match value.to_string().to_lowercase().as_str() {
            "uppercase" => Self::Uppercase,
            "lowercase" => Self::Lowercase,
            _ => Self::Unknown,
        }
    }
}

pub struct Apply(Vec<Instructions>);

impl Apply {
    pub fn parse<T>(input: T) -> Apply
    where
        T: ToString,
    {
        Apply(
            input
                .to_string()
                .split(",")
                .map(Instructions::from)
                .collect::<Vec<Instructions>>(),
        )
    }

    pub fn execute<T>(&self, input: T) -> String
    where
        T: ToString,
    {
        self.0.iter().fold(
            input.to_string(),
            |current, instruction| match instruction {
                Instructions::Uppercase => current.to_uppercase(),
                Instructions::Lowercase => current.to_lowercase(),
                Instructions::Unknown => current,
            },
        )
    }
}
