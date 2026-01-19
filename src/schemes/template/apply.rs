enum Instructions {
    /// Make whole value uppercase
    Uppercase,

    /// Make whole value lowercase
    Lowercase,
}

impl Instructions {
    fn from(value: impl AsRef<str>) -> Option<Self> {
        match value.as_ref().trim().to_ascii_lowercase().as_str() {
            "uppercase" => Some(Self::Uppercase),
            "lowercase" => Some(Self::Lowercase),
            _ => None,
        }
    }
}

pub struct Apply(Vec<Option<Instructions>>);

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
                .collect::<Vec<Option<Instructions>>>(),
        )
    }

    pub fn execute<T>(&self, input: T) -> String
    where
        T: ToString,
    {
        self.0.iter().fold(
            input.to_string(),
            |current, instruction| match instruction {
                Some(Instructions::Uppercase) => current.to_uppercase(),
                Some(Instructions::Lowercase) => current.to_lowercase(),
                _ => current,
            },
        )
    }
}
