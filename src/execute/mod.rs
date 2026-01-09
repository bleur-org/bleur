pub mod task;

use crate::{execute::task::Task, schemes::template::Template, Result};

pub struct Executor {
    tasks: Vec<Task>,
}

impl Executor {
    pub fn consume(template: Template) -> Self {
        Self {
            tasks: template.to_tasks(),
        }
    }

    pub fn compute(self) -> Result<()> {
        self.tasks.iter().try_for_each(|t| t.to_owned().execute())
    }
}
