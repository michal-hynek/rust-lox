use std::collections::HashMap;

use anyhow::Result;

use crate::scanner::LiteralValue;

struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: HashMap::new() }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Result<LiteralValue> {
        match self.values.get(&name) {
            Some(val) => Ok(val.clone()),
            None => Err(anyhow::anyhow!("Undefined variable {}", name)),
        }
    }
}