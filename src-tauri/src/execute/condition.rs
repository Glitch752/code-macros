use serde::{Serialize, Deserialize};

use super::{get_variable, expression::{get_expression_number, evaluate_expression, Expression}, variable::get_variable_number};
use super::variable::{Variables, Variable, VariableValue};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type_")]
#[serde(rename_all = "lowercase")]
pub enum Condition {
    Boolean { value: bool },
    Comparison {
        left: Box<Condition>, 
        comparison: String, 
        right: Box<Condition> 
    },
    Logical {
        left: Box<Condition>, 
        kind: String, 
        right: Box<Condition> 
    },
    Number { value: f64 },
    Variable { variable: String },
    Expression { expression: Expression }
}

pub fn evaluate_condition(condition: &Condition, variables: &mut Variables) -> Condition {
    match condition {
        Condition::Boolean { value: _ } => {
            return condition.clone();
        },
        Condition::Number { value: _ } => {
            return condition.clone();
        },
        Condition::Comparison { left, comparison, right } => {
            let left_result: f64 = get_condition_number(evaluate_condition(left, variables));
            let right_result: f64 = get_condition_number(evaluate_condition(right, variables));
            match comparison.as_str() {
                ">" => {
                    return Condition::Boolean{ value: left_result > right_result };
                },
                "<" => {
                    return Condition::Boolean{ value: left_result < right_result };
                },
                ">=" => {
                    return Condition::Boolean{ value: left_result >= right_result };
                },
                "<=" => {
                    return Condition::Boolean{ value: left_result <= right_result };
                },
                "==" => {
                    return Condition::Boolean{ value: left_result == right_result };
                },
                "!==" => {
                    return Condition::Boolean{ value: left_result != right_result };
                },
                _ => todo!()
            }
        },
        Condition::Logical { left, kind, right } => {
            let left_result: bool = get_condition_bool(evaluate_condition(left, variables));
            let right_result: bool = get_condition_bool(evaluate_condition(right, variables));
            match kind.as_str() {
                "and" => {
                    return Condition::Boolean{ value: left_result && right_result };
                },
                "or" => {
                    return Condition::Boolean{ value: left_result || right_result };
                },
                "not" => {
                    return Condition::Boolean{ value: !right_result };
                },
                _ => todo!()
            }
        },
        Condition::Variable { variable } => {
            return Condition::Number{ value: get_variable_number(
                get_variable(variables, variable.to_string()).unwrap_or( &Variable::new(VariableValue::Number(0.0)) ).value.clone()
            ) };
        },
        Condition::Expression { expression } => {
            return Condition::Number{ value: get_expression_number(
                evaluate_expression(expression, variables)
            ) }
        }
    }
}

pub fn get_condition_bool(value: Condition) -> bool {
    match value {
        Condition::Boolean { value } => {
            return value;
        },
        Condition::Number { value } => {
            if value == 0.0 {
                return false;
            } else {
                return true;
            }
        },
        _ => {
            return false;
        }
    }
}

pub fn get_condition_number(value: Condition) -> f64 {
    match value {
        Condition::Boolean { value } => {
            return if value {1.0} else {0.0};
        },
        Condition::Number { value } => {
            return value;
        },
        _ => {
            return 0.0;
        }
    }
}