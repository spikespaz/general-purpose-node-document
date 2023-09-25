use std::collections::HashMap;

use crate::Value;

pub trait Document {
    fn nodes(&self) -> &[&dyn Node];
}

pub trait Node: Document {
    fn name(&self) -> &str;

    fn args(&self) -> &[&Value];

    fn params(&self) -> HashMap<&str, &Value>;

    fn has_nodes(&self) -> bool {
        !self.nodes().is_empty()
    }

    fn has_args(&self) -> bool {
        !self.args().is_empty()
    }

    fn has_params(&self) -> bool {
        !self.params().is_empty()
    }
}
