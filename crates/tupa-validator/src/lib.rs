//! Compatibility facade crate for validation APIs.
//!
//! This crate re-exports `tupa-typecheck` validation entrypoints so
//! integrations can depend on a stable `tupa-validator` crate name.

pub use tupa_typecheck::{
    typecheck_program,
    typecheck_program_with_warnings,
    TypeError,
    Warning,
};

use tupa_parser::Program;

/// Validates a parsed program and returns warnings.
#[allow(clippy::result_large_err)]
pub fn validate_program(program: &Program) -> Result<Vec<Warning>, TypeError> {
    typecheck_program_with_warnings(program)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tupa_parser::parse_program;

    #[test]
    fn validator_facade_works() {
        let program = parse_program("fn main() { let x: i64 = 1; }").expect("parse");
        let result = validate_program(&program);
        assert!(result.is_ok());
    }
}