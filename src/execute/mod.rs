pub mod task;

use std::collections::HashMap;

use crate::{execute::task::Task, schemes::template::Template, Result};

#[derive(Debug)]
pub struct Executor {
    tasks: Vec<Task>,
}

impl Executor {
    pub fn consume(template: Template) -> Self {
        Self {
            tasks: template.to_tasks(),
        }
    }

    pub fn compute(self, global: &mut HashMap<String, String>) -> Result<()> {
        self.tasks
            .iter()
            .try_for_each(|t| t.to_owned().execute(global))
    }
}
