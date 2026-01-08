mod tap;

use crate::{schemes::template::Template, Result};
use tap::*;

pub struct Executor {
    template: Template,
}

impl Executor {
    pub fn consume(template: Template) -> Self {
        Self { template }
    }

    pub fn compute(self) -> Result<()> {
        self.variables().unwrap();

        Ok(())
    }

    /// Go through every rename instructions
    pub fn variables(self) -> Result<Self> {
        self.try_tap(|s| {
            s.template
                .clone()
                .variables()
                .iter()
                .try_for_each(|v| v.execute())
        })
    }
}
