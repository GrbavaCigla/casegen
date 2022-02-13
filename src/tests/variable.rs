use crate::parser::parse_variable;
use crate::structures::Variable;

#[test]
fn test_variable_with_name() {
    assert_eq!(
        parse_variable("n(0, 100)"),
        Ok((
            "",
            Variable {
                name: Some("n"),
                lower: 0,
                upper: 100,
            }
        ))
    );
}

#[test]
fn test_variable_without_name() {
    assert_eq!(
        parse_variable("(0, 100)"),
        Ok((
            "",
            Variable {
                name: None,
                lower: 0,
                upper: 100,
            }
        ))
    );
}

#[test]
fn test_variable_without_spaces() {
    assert_eq!(
        parse_variable("(0,100)"),
        Ok((
            "",
            Variable {
                name: None,
                lower: 0,
                upper: 100,
            }
        ))
    );
}

#[test]
fn test_variable_with_spaces() {
    assert_eq!(
        parse_variable("n(   0 ,   100   )"),
        Ok((
            "",
            Variable {
                name: Some("n"),
                lower: 0,
                upper: 100,
            }
        ))
    );
}
