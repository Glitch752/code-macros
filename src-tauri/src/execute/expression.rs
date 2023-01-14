use serde::{Serialize, Deserialize};

use super::Variables;
use super::variable::{Variable, get_variable, VariableValue, get_variable_number};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type_")]
#[serde(rename_all = "lowercase")]
pub enum Expression {
    Number { value: f64 },
    Variable { variable: String },
    Arithmetic {
        left: Box<Expression>, 
        kind: String, 
        right: Box<Expression> 
    },
    Bitwise {
        left: Box<Expression>, 
        kind: String, 
        right: Box<Expression> 
    }
}

pub fn get_expression_number(value: Expression) -> f64 {
    match value {
        Expression::Number { value } => {
            return value;
        },
        _ => {
            return 0.0;
        }
    }
}

pub fn evaluate_expression(expression: &Expression, variables: &mut Variables) -> Expression {
    match expression {
        Expression::Number { .. } => {
            return expression.clone();
        },
        Expression::Variable { variable } => {
            return Expression::Number{ value: get_variable_number(
                get_variable(variables, variable.to_string()).unwrap_or( &Variable::new(VariableValue::Number(0.0)) ).value.clone()
            ) };
        },
        Expression::Arithmetic { left, kind, right } => {
            let left_result: f64 = get_expression_number(evaluate_expression(left, variables));
            let right_result: f64 = get_expression_number(evaluate_expression(right, variables));
            match kind.as_str() {
                "addition" => {
                    return Expression::Number{ value: left_result + right_result };
                },
                "subtraction" => {
                    return Expression::Number{ value: left_result - right_result };
                },
                "division" => {
                    return Expression::Number{ value: left_result / right_result };
                },
                "multiplication" => {
                    return Expression::Number{ value: left_result * right_result };
                },
                "modulo" => {
                    return Expression::Number{ value: left_result % right_result };
                },
                "exponent" => {
                    return Expression::Number{ value: left_result.powf(right_result) };
                },
                _ => unimplemented!()
            }
        },
        Expression::Bitwise { left, kind, right } => {
            let left_result: i64 = get_expression_number(evaluate_expression(left, variables)).round() as i64;
            let right_result: i64 = if kind == "not" { 0 } else { get_expression_number(evaluate_expression(right, variables)).round() as i64 };
            match kind.as_str() {
                "and" => {
                    return Expression::Number{ value: (left_result & right_result) as f64 };
                },
                "or" => {
                    return Expression::Number{ value: (left_result | right_result) as f64 };
                },
                "xor" => {
                    return Expression::Number{ value: (left_result ^ right_result) as f64 };
                },
                "not" => {
                    return Expression::Number{ value: (!right_result) as f64 };
                },
                "leftshift" => {
                    return Expression::Number{ value: (left_result << right_result) as f64 };
                },
                "signrightshift" => {
                    return Expression::Number{ value: (left_result >> right_result) as f64 };
                },
                _ => unimplemented!()
            }
        }
    }
}