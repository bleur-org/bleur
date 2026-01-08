pub mod task;

use crate::{
    execute::task::{Task, ToTask},
    schemes::template::Template,
    Result,
};

pub struct Executor {
    tasks: Vec<Task>,
}

impl Executor {
    pub fn consume(template: Template) -> Self {
        let variables = template
            .variables()
            .iter()
            .map(|v| v.to_owned().to_task())
            .collect();

        Self { tasks: variables }
    }

    pub fn compute(self) -> Result<()> {
        self.tasks.iter().try_for_each(|t| t.to_owned().execute())
    }
}
