use serde::{ Deserialize, Serialize };

use std::collections::HashMap;

use tauri::api::notification::Notification;

static MAX_LOOP_ITERATIONS: u64 = 100000;

use std::thread;

use super::Macro;

use super::initiators::Initiator;

use inputbot::{KeySequence, MouseCursor, KeybdKey, MouseButton};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Execution {
    pub type_: String,
    pub data: ExecutionData,
    pub variables: Vec<VariableType>,
    pub code_inside: ExecutionCodeInside
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutionData {
    pub time: Option<f64>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub start: Option<f64>,
    pub end: Option<f64>,
    pub step: Option<f64>,
    pub condition: Option<Condition>,
    pub variable: Option<String>,
    pub value: Option<f64>,
    pub content: Option<Expression>,
    pub function: Option<String>,
    pub string: Option<String>,
    pub key: Option<String>,
    pub button: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VariableType {
    pub type_: String,
    pub name: String
}

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutionCodeInside {
    pub loop_: Option<ExecutionWrapper>,
    pub then: Option<ExecutionWrapper>,
    pub else_: Option<ExecutionWrapper>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecutionWrapper {
    pub executes: Vec<Execution>
}

impl<'a> Default for &'a ExecutionWrapper {
    fn default() -> &'a ExecutionWrapper {
        static DEFAULT: ExecutionWrapper = ExecutionWrapper {
            executes: Vec::new()
        };
        &DEFAULT
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub executes: Vec<Execution>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub type_: String,
    pub default_value: String,
}

pub fn run_macro_initiator(initiator: Initiator, macro_: Macro) {
    println!("Running macro initiator from macro \"{}\"", macro_.name);
    thread::spawn(move || {
        let mut new_variables: Variables = Variables::new();
        execute_macro_code(&initiator.executes, &mut new_variables, &mut false, macro_);
    });
}

pub fn run_macro_function(function: Function, macro_: Macro, variables: &mut Variables) {
    println!("Running macro function from macro \"{}\"", macro_.name);
    execute_macro_code(&function.executes, variables, &mut false, macro_);
}


type Variables = HashMap<String, Variable>;

#[derive(Debug, Clone)]
pub struct Variable {
    pub value: VariableValue,
}

impl Variable {
    fn new(value: VariableValue) -> Variable<> {
        Variable { value: value }
    }
}

#[derive(Clone, Debug)]
pub enum VariableValue {
    String(String),
    Number(f64),
}

fn get_variable_string(variable_value: VariableValue) -> String {
    match variable_value {
        VariableValue::String(value) => {
            return value;
        },
        VariableValue::Number(value) => {
            return value.to_string();
        }
    }
}

fn get_variable_number(variable_value: VariableValue) -> f64 {
    match variable_value {
        VariableValue::String(_value) => {
            return 0.0;
        },
        VariableValue::Number(value) => {
            return value;
        }
    }
}

fn execute_macro_code(code: &Vec<Execution>, variables: &mut Variables, stop_execution: &mut bool, macro_: Macro) {
    for execution in code {
        if *stop_execution {
            return;
        }

        match execution.type_.as_str() {
            "wait" => {
                let time = execution.data.time.as_ref().unwrap();
                thread::sleep(std::time::Duration::from_millis((time * 1000.0) as u64));
            }
            "notification" => {
                let title = execution.data.title.as_ref().unwrap();
                let message = execution.data.message.as_ref().unwrap();
                let _ = Notification::new("code-macros")
                    .title(parse_string(title, variables))
                    .body(parse_string(message, variables))
                    .show();
            }
            "fromtoloop" => {
                let from: f64 = *execution.data.start.as_ref().unwrap_or(&f64::from(0));
                let to: f64 = *execution.data.end.as_ref().unwrap_or(&f64::from(4));
                let step: f64 = *execution.data.step.as_ref().unwrap_or(&f64::from(1));
                let mut i: f64 = from;
                let mut iterations: u64 = 0;
                let mut value_variable: Option<String> = None;
                for variable in &execution.variables {
                    if variable.type_ == "value".to_string() {
                        value_variable = Some(variable.name.clone());
                    }
                }
                let variable_name: String = value_variable.unwrap_or("".to_string());

                if to > from {
                    while i <= to {
                        set_variable(variables, variable_name.to_string().clone(), VariableValue::Number(i));
                        execute_macro_code(&execution.code_inside.loop_.as_ref().unwrap_or_default().executes, variables, stop_execution, macro_.clone());
                        i += step;
                        iterations += 1;
                        if iterations > MAX_LOOP_ITERATIONS {
                            break;
                        }
                    }
                } else {
                    while i >= to {
                        set_variable(variables, variable_name.to_string().clone(), VariableValue::Number(i));
                        execute_macro_code(&execution.code_inside.loop_.as_ref().unwrap_or_default().executes, variables, stop_execution, macro_.clone());
                        i += step;
                        iterations += 1;
                        if iterations > MAX_LOOP_ITERATIONS {
                            break;
                        }
                    }
                }
            }
            "whileloop" => {
                // TODO: Properly implement variables so this loop can be used.
                let condition: &Condition = &execution.data.condition.as_ref().unwrap();

                let mut value_variable: Option<String> = None;
                for variable in &execution.variables {
                    if variable.type_ == "iteration".to_string() {
                        value_variable = Some(variable.name.clone());
                    }
                }
                let variable_name: String = value_variable.unwrap_or("".to_string());

                let mut i: u64 = 0;

                while get_condition_bool(evaluate_condition(condition, variables)) {
                    set_variable(variables, variable_name.to_string().clone(), VariableValue::Number(i as f64));
                    i += 1;
                    execute_macro_code(&execution.code_inside.loop_.as_ref().unwrap_or_default().executes, variables, stop_execution, macro_.clone());
                    
                    if i > MAX_LOOP_ITERATIONS {
                        break;
                    }
                }
            }
            "if" => {
                // TODO: Properly implement variables so 'if' can be used
                let condition: &Condition = &execution.data.condition.as_ref().unwrap();
                if get_condition_bool(evaluate_condition(condition, variables)) {
                    execute_macro_code(&execution.code_inside.then.as_ref().unwrap_or_default().executes, variables, stop_execution, macro_.clone());
                } else {
                    execute_macro_code(&execution.code_inside.else_.as_ref().unwrap_or_default().executes, variables, stop_execution, macro_.clone());
                }
            }
            "stop" => {
                *stop_execution = true;
            }
            "setvariable" => {
                let variable: &String = &execution.data.variable.as_ref().unwrap();
                let content: &Expression = execution.data.content.as_ref().unwrap();

                set_variable(variables, variable.to_string().clone(), VariableValue::Number(
                    get_expression_number(evaluate_expression(content, &mut variables.clone()))
                ));
            }
            "function" => {
                let function_name: &String = &execution.data.function.as_ref().unwrap();
                for function in &macro_.clone().macro_.functions.unwrap() {
                    if function.name == *function_name {
                        run_macro_function(function.clone(), macro_.clone(), variables);
                        break;
                    }
                }
            }
            "typestring" => {
                let string: &String = &execution.data.string.as_ref().unwrap();

                // FIXME: Yes, this is really, really bad code. It is intentionally creating a memory leak.
                let string_static = Box::leak(parse_string(&string.clone(), &mut variables.clone()).into_boxed_str());

                KeySequence(string_static).send();
            }
            "movemouserelative" => {
                let x: f64 = *execution.data.x.as_ref().unwrap();
                let y: f64 = *execution.data.y.as_ref().unwrap();

                MouseCursor::move_rel(x.round() as i32, y.round() as i32);
            }
            "movemouseabsolute" => {
                let x: f64 = *execution.data.x.as_ref().unwrap();
                let y: f64 = *execution.data.y.as_ref().unwrap();

                MouseCursor::move_abs(x.round() as i32, y.round() as i32);
            }
            "presskey" => {
                let key: &String = &execution.data.key.as_ref().unwrap();
                
                todo!();
            }
            "releasekey" => {
                let key: &String = &execution.data.key.as_ref().unwrap();

                todo!();
            }
            "pressmouse" => {
                let button: &String = &execution.data.key.as_ref().unwrap();

                match button.as_str() {
                    "LMB" => {
                        MouseButton::LeftButton.press();
                    }
                    "RMB" => {
                        MouseButton::RightButton.press();
                    }
                    _ => todo!()
                }
            }
            "releasemouse" => {
                let button: &String = &execution.data.key.as_ref().unwrap();

                match button.as_str() {
                    "LMB" => {
                        MouseButton::LeftButton.release();
                    }
                    "RMB" => {
                        MouseButton::RightButton.release();
                    }
                    _ => todo!()
                }
            }
            _ => todo!()
        }
    }
}

fn evaluate_condition(condition: &Condition, variables: &mut Variables) -> Condition {
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

fn evaluate_expression(expression: &Expression, variables: &mut Variables) -> Expression {
    match expression {
        Expression::Number { value: _ } => {
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
                _ => todo!()
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
                _ => todo!()
            }
        }
    }
}

fn get_expression_number(value: Expression) -> f64 {
    match value {
        Expression::Number { value } => {
            return value;
        },
        _ => {
            return 0.0;
        }
    }
}

fn get_condition_bool(value: Condition) -> bool {
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

fn get_condition_number(value: Condition) -> f64 {
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

fn parse_string<'a>(string: &'a String, variables: &'a mut Variables) -> String {
    let variable_split: Vec<&str> = string.split("{{").collect();
    let mut result = String::from(variable_split[0]);
    let mut index: u64 = 0;
    for split in variable_split {
        index += 1;
        if index == 1 {
            continue;
        }
        let halves: Vec<&str> = split.split("}}").collect();
        let variable_name: String = halves[0].to_string();
        let variable_value = get_variable(variables, variable_name);
        result.push_str(&get_variable_string(
            variable_value.unwrap_or(
                &Variable::new(VariableValue::String("undefined".to_string()))
            ).value.clone()
        ));
        if halves.len() > 1 {
            result.push_str(&halves[1].to_string());
        }
    }

    let characters: std::str::Chars = result.chars();

    let mut skip: bool = false;

    let mut index: usize= 0;
    let mut characters_copy: Vec<char> = characters.clone().collect();
    for character in characters {
        if skip {
            skip = false;
            continue;
        }
        if character == '\\' {
            characters_copy.remove(index);
            skip = true;
        }
        index += 1;
    }

    result = characters_copy.into_iter().collect();

    return result;
}

fn set_variable(variables: &mut Variables, variable: String, value: VariableValue) {
    *variables.entry(variable).or_insert(Variable::new(value.clone())) = Variable::new(value.clone());
}

fn get_variable(variables: &mut Variables, variable: String) -> Option<&Variable> {
    return variables.get(&variable);
}