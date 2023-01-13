use std::{collections::HashMap, cmp::Ordering};

pub type Variables = HashMap<String, Variable>;

#[derive(Debug, Clone)]
pub struct Variable {
    pub value: VariableValue,
}

impl Variable {
    pub fn new(value: VariableValue) -> Variable<> {
        Variable { value: value }
    }
}

#[derive(Clone, Debug)]
pub enum VariableValue {
    String(String),
    Number(f64),
    Array(Vec<VariableValue>)
}

impl Ord for VariableValue {
    fn cmp(&self, other: &VariableValue) -> Ordering {
        match (self, other) {
            (VariableValue::String(a), VariableValue::String(b)) => a.cmp(b),
            (VariableValue::Number(a), VariableValue::Number(b)) => a.partial_cmp(b).unwrap(),
            (VariableValue::Array(a), VariableValue::Array(b)) => a.cmp(b),
            // Pretty much arbitrary comparisons. This just allows for sorting into groups.
            (VariableValue::String(_), _) => Ordering::Less,
            (VariableValue::Number(_), VariableValue::String(_)) => Ordering::Greater,
            (VariableValue::Number(_), VariableValue::Array(_)) => Ordering::Less,
            (VariableValue::Array(_), _) => Ordering::Greater,
            // _ => todo!("Implement comparison between {:?} and {:?}.", self, other)
        }
    }
}

impl PartialOrd for VariableValue {
    fn partial_cmp(&self, other: &VariableValue) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Eq for VariableValue {}

impl PartialEq for VariableValue {
    fn eq(&self, other: &VariableValue) -> bool {
        match (self, other) {
            (VariableValue::String(a), VariableValue::String(b)) => a == b,
            (VariableValue::Number(a), VariableValue::Number(b)) => a == b,
            (VariableValue::Array(a), VariableValue::Array(b)) => a == b,
            _ => false
        }
    }
}

pub fn get_variable_string(variable_value: VariableValue) -> String {
    match variable_value {
        VariableValue::String(value) => {
            return value;
        },
        VariableValue::Number(value) => {
            return value.to_string();
        },
        VariableValue::Array(value) => {
            let mut converted_value = "[".to_string();
            for i in 0..value.len() {
                converted_value.push_str(&(get_variable_string(value[i].clone())));
                if i != (value.len()-1) {
                    converted_value.push_str(", ");
                }
            }
            converted_value.push_str("]");
            return converted_value;
        }
    }
}

pub fn get_variable_number(variable_value: VariableValue) -> f64 {
    match variable_value {
        VariableValue::String(_value) => {
            // TODO: Make this return the number in the string if possible.
            return 0.0;
        },
        VariableValue::Number(value) => {
            return value;
        },
        VariableValue::Array(_value) => {
            return 0.0;
        }
    }
}

pub fn get_variable_vector(variable_value: VariableValue) -> Vec<VariableValue> {
    match variable_value {
        VariableValue::String(value) => {
            return value.chars().map(|c| VariableValue::String(c.to_string())).collect();
        },
        VariableValue::Number(_value) => {
            return Vec::new();
        },
        VariableValue::Array(value) => {
            return value;
        }
    }
}

pub fn set_variable(variables: &mut Variables, variable: String, value: VariableValue) {
    *variables.entry(variable).or_insert(Variable::new(value.clone())) = Variable::new(value.clone());
}

pub fn get_variable(variables: &mut Variables, variable: String) -> Option<&Variable> {
    return variables.get(&variable);
}