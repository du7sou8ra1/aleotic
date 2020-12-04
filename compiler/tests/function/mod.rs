// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    assert_satisfied,
    expect_compiler_error,
    expect_type_inference_error,
    get_output,
    parse_program,
    parse_program_with_input,
};
use leo_compiler::errors::{CompilerError, ExpressionError, FunctionError, StatementError};

#[test]
fn test_empty() {
    let bytes = include_bytes!("empty.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_iteration() {
    let bytes = include_bytes!("iteration.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_iteration_repeated() {
    let bytes = include_bytes!("iteration_repeated.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_newlines() {
    let input_string = include_str!("input/newlines.in");
    let program_string = include_str!("newlines.leo");
    let program = parse_program_with_input(program_string, input_string).unwrap();

    let expected_string = include_str!("output/newlines.out");
    let actual_bytes = get_output(program);
    let actual_string = std::str::from_utf8(actual_bytes.bytes().as_slice()).unwrap();

    assert_eq!(expected_string, actual_string);
}

#[test]
fn test_multiple_returns() {
    let bytes = include_bytes!("multiple.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_multiple_returns_main() {
    let program_string = include_str!("multiple_main.leo");
    let input_string = include_str!("input/registers.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    let expected_string = include_str!("output/registers.out");
    let actual_bytes = get_output(program);
    let actual_string = std::str::from_utf8(actual_bytes.bytes().as_slice()).unwrap();

    assert_eq!(expected_string, actual_string);
}

#[test]
fn test_repeated_function_call() {
    let bytes = include_bytes!("repeated.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_return() {
    let bytes = include_bytes!("return.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_scope_fail() {
    let bytes = include_bytes!("scope_fail.leo");
    let program = parse_program(bytes).unwrap();

    match expect_compiler_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::FunctionError(value),
        ))) => match *value {
            FunctionError::StatementError(StatementError::ExpressionError(ExpressionError::Error(_))) => {}
            error => panic!("Expected function undefined, got {}", error),
        },
        error => panic!("Expected function undefined, got {}", error),
    }
}

#[test]
fn test_undefined() {
    let bytes = include_bytes!("undefined.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error);
}

#[test]
fn test_value_unchanged() {
    let bytes = include_bytes!("value_unchanged.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_array_input() {
    let bytes = include_bytes!("array_input.leo");
    let error = parse_program(bytes).err().unwrap();

    expect_type_inference_error(error)
}

// Test return multidimensional arrays

#[test]
fn test_return_array_nested_fail() {
    let bytes = include_bytes!("return_array_nested_fail.leo");
    let program = parse_program(bytes).unwrap();

    let _err = expect_compiler_error(program);
}

#[test]
fn test_return_array_nested_pass() {
    let bytes = include_bytes!("return_array_nested_pass.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_return_array_tuple_fail() {
    let bytes = include_bytes!("return_array_tuple_fail.leo");
    let program = parse_program(bytes).unwrap();

    let _err = expect_compiler_error(program);
}

#[test]
fn test_return_array_tuple_pass() {
    let bytes = include_bytes!("return_array_tuple_pass.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

// Test return tuples

#[test]
fn test_return_tuple() {
    let bytes = include_bytes!("return_tuple.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_return_tuple_conditional() {
    let bytes = include_bytes!("return_tuple_conditional.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}
