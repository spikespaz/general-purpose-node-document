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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use once_cell::sync::Lazy;

    use super::*;

    struct Parent {
        arg_one: String,
        arg_two: f64,
        arg_three: Option<i32>,
        param_one: String,
        param_two: f64,
        param_three: Option<i32>,
    }

    impl Node for Parent {
        fn name(&self) -> &str {
            "parent"
        }

        fn args(&self) -> Vec<Value<'_>> {
            vec![
                Value::from(&self.arg_one),
                Value::from(self.arg_two),
                Value::from(self.arg_three),
            ]
        }

        fn params(&self) -> HashMap<&str, Value> {
            HashMap::from([
                ("one", Value::from(&self.param_one)),
                ("two", Value::from(self.param_two)),
                ("three", Value::from(self.param_three)),
            ])
        }
    }

    static PARENT_NODE: Lazy<Parent> = Lazy::new(|| Parent {
        arg_one: "foo".to_owned(),
        arg_two: 2.3,
        arg_three: Some(95),
        param_one: "bar".to_owned(),
        param_two: 3.2,
        param_three: None,
    });

    #[test]
    fn test_node_args() {
        for (index, value) in PARENT_NODE.args().into_iter().enumerate() {
            println!(
                "arg index: {}, kind: {}, value: {:?}",
                index,
                value.kind(),
                value
            );
        }
    }

    #[test]
    fn test_node_params() {
        for (key, value) in PARENT_NODE.params().into_iter() {
            println!(
                "param name: {}, kind: {}, value: {:?}",
                key,
                value.kind(),
                value
            );
        }
    }
}
