use std::collections::HashMap;

use anyhow::Result;

use crate::scanner::LiteralValue;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Option<LiteralValue>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: HashMap::new() }
    }

    pub fn define(&mut self, name: String, value: Option<LiteralValue>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Result<Option<LiteralValue>> {
        match self.values.get(&name) {
            Some(val) => Ok(val.clone()),
            None => Err(anyhow::anyhow!("Undefined variable {}", name)),
        }
    }
}