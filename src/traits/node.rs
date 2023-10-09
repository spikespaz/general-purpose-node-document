use std::collections::HashMap;

use crate::Value;

pub trait Document {
    fn nodes(&self) -> Vec<&dyn Node>;

    fn has_nodes(&self) -> bool {
        !self.nodes().is_empty()
    }
}

pub trait Node {
    fn name(&self) -> &str;

    fn args(&self) -> Vec<Value<'_>>;

    fn params(&self) -> HashMap<&str, Value<'_>>;

    fn has_args(&self) -> bool {
        !self.args().is_empty()
    }

    fn has_params(&self) -> bool {
        !self.params().is_empty()
    }
}
