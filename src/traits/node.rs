use std::collections::HashMap;

use crate::Value;

pub trait Document {
    fn nodes(&self) -> Vec<&dyn Node>;

    fn get_node(&self, index: usize) -> Option<&dyn Node> {
        self.nodes().get(index).copied()
    }

    fn has_nodes(&self) -> bool {
        !self.nodes().is_empty()
    }
}

pub trait Node {
    fn name(&self) -> &str;

    fn args(&self) -> Vec<Value<'_>>;

    fn params(&self) -> HashMap<&str, Value<'_>>;

    fn get_arg(&self, index: usize) -> Option<Value<'_>> {
        self.args().get(index).cloned()
    }

    fn get_param(&self, key: &str) -> Option<Value<'_>> {
        self.params().get(key).cloned()
    }

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
        child_one: ChildOne,
        child_two: ChildTwo,
    }

    struct ChildOne {
        arg: usize,
    }

    struct ChildTwo {
        param_foo: String,
    }

    impl Document for Parent {
        fn nodes(&self) -> Vec<&dyn Node> {
            vec![&self.child_one, &self.child_two]
        }

        fn get_node(&self, index: usize) -> Option<&dyn Node> {
            match index {
                0 => Some(&self.child_one),
                1 => Some(&self.child_two),
                _ => None,
            }
        }
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

        fn get_arg(&self, index: usize) -> Option<Value<'_>> {
            match index {
                0 => Some(Value::from(&self.arg_one)),
                1 => Some(Value::from(self.arg_two)),
                2 => Some(Value::from(self.arg_three)),
                _ => None,
            }
        }

        fn params(&self) -> HashMap<&str, Value<'_>> {
            HashMap::from([
                ("one", Value::from(&self.param_one)),
                ("two", Value::from(self.param_two)),
                ("three", Value::from(self.param_three)),
            ])
        }

        fn get_param(&self, key: &str) -> Option<Value<'_>> {
            match key {
                "one" => Some(Value::from(&self.param_one)),
                "two" => Some(Value::from(self.param_two)),
                "three" => Some(Value::from(self.param_three)),
                _ => None,
            }
        }
    }

    impl Node for ChildOne {
        fn name(&self) -> &str {
            "one"
        }

        fn args(&self) -> Vec<Value<'_>> {
            vec![Value::from(self.arg)]
        }

        fn params(&self) -> HashMap<&str, Value<'_>> {
            HashMap::new()
        }
    }

    impl Node for ChildTwo {
        fn name(&self) -> &str {
            "two"
        }

        fn args(&self) -> Vec<Value<'_>> {
            vec![]
        }

        fn params(&self) -> HashMap<&str, Value<'_>> {
            HashMap::from([("foo", Value::from(&self.param_foo))])
        }
    }

    static PARENT_NODE: Lazy<Parent> = Lazy::new(|| Parent {
        arg_one: "foo".to_owned(),
        arg_two: 2.3,
        arg_three: Some(95),
        param_one: "bar".to_owned(),
        param_two: 3.2,
        param_three: None,
        child_one: ChildOne { arg: usize::MAX },
        child_two: ChildTwo {
            param_foo: "bar".to_owned(),
        },
    });

    #[test]
    fn test_document_args() {
        for node in PARENT_NODE.nodes() {
            println!(
                "node name: {}, args: {:?}, params: {:?}",
                node.name(),
                node.args(),
                node.params()
            );
        }
    }

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
