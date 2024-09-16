use nom::bytes::complete::take_until;
use nom::IResult;
use nom::combinator::{opt, recognize};
use nom::sequence::{tuple, preceded};
use nom::character::complete::{char, digit0, digit1};

#[derive(Debug, PartialEq)]
pub struct Line {
    line: String,
}

impl Line {
    pub fn new(line: String) -> Self {
        Self { line }
    }

    pub fn parsed_for_state(&self) -> IResult<&str, (Option<(char, &str)>, Option<(char, &str)>, Option<(char, &str)>)> {
        let (_, g_value) = opt(positioning_mode)(&self.line)?;
        let (_, x_value) = opt(x)(&self.line)?;
        let (_, y_value) = opt(y)(&self.line)?;

        Ok(("", (g_value, x_value, y_value)))
    }
}

/// Gコードに到達するまでスキップし、Gコードと値、残りの文字列を返す
pub fn positioning_mode(input: &str) -> IResult<&str, (char, &str)> {
    preceded(
        take_until("G"), 
        tuple((
            char('G'), 
            recognize(
                tuple(( 
                    digit1, 
                    opt(char('.')), 
                    digit0
                ))
            )
        ))
    )(input)
}

/// Xコードに到達するまでスキップし、Xコードと値、残りの文字列を返す
pub fn x(input: &str) -> IResult<&str, (char, &str)> {
    preceded(
        take_until("X"),
        tuple((
            char('X'), 
            recognize(
                tuple((
                    opt(char('-')), 
                    digit1, 
                    opt(char('.')), 
                    digit0
                ))
            )
        ))
    )(input)
    
}

/// Yコードに到達するまでスキップし、Yコードと値、残りの文字列を返す
pub fn y(input: &str) -> IResult<&str, (char, &str)> {
    preceded(
        take_until("Y"), 
        tuple((
            char('Y'), 
            recognize(
                tuple((
                    opt(char('-')), 
                    digit1, 
                    opt(char('.')), 
                    digit0
                ))
            )
        ))
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_new() {
        let expected = "test";
        let result = Line::new("test".to_string());

        assert_eq!(result.line, expected);
    }

    #[test]
    fn line_parsed_for_state_mode() {
        let datas = [
            (
                "G90X100.0Y-100.0",
                ("", (Some(('G', "90")), Some(('X', "100.0")), Some(('Y', "-100.0"))))
            ),
            (
                "G90X100.0",
                ("", (Some(('G', "90")), Some(('X', "100.0")), None))
            ),
            (
                "G90Y-100.0",
                ("", (Some(('G', "90")), None, Some(('Y', "-100.0"))))
            ),
            (
                "Y-100.0X100.0",
                ("", (None, Some(('X', "100.0")), Some(('Y', "-100.0"))))
            ),
            (
                "V500.0G90W700.0X100.0Z200.0Y-100.0C50.0",
                ("", (Some(('G', "90")), Some(('X', "100.0")), Some(('Y', "-100.0"))))
            ),
            (
                "V500.0W700.0",
                ("", (None, None, None))
            ),
            (
                "",
                ("", (None, None, None))
            )
        ];
        
        datas.iter().for_each(|&(input, expected)| {
            let line = Line::new(input.to_string());
            let result = line.parsed_for_state();

            assert_eq!(result.unwrap(), expected);

        })
    }

    #[test]
    fn line_positioning_mode_normal() {
        let datas = [
            (
                "G90X100.0Y-100.0",
                ("X100.0Y-100.0", ('G', "90"))
            ),
            (
                "X100.0Y-100.0G91",
                ("", ('G', "91"))
            ),
            (
                "X100.0G91.2Y-100.0",
                ("Y-100.0", ('G', "91.2"))
            ),
            (
                "X100.0G0Y-100.0",
                ("Y-100.0", ('G', "0"))
            ),
        ];
        
        datas.iter().for_each(|&(input, expected)| {
            let result = positioning_mode(input);

            assert_eq!(result.unwrap(), expected);

        })
    }

    #[test]
    fn line_positioning_mode_abnormality() {
        let datas = [
            (
                "X100.0Y-100.0",
                Err(nom::Err::Error(nom::error::Error {
                    input: "X100.0Y-100.0", 
                    code: nom::error::ErrorKind::TakeUntil
                }))
            ),
            (
                "X100.0GY-100.0",
                Err(nom::Err::Error(nom::error::Error {
                    input: "Y-100.0", 
                    code: nom::error::ErrorKind::Digit
                }))
            ),
            (
                "X100.0g200.0Y-100.0",
                Err(nom::Err::Error(nom::error::Error {
                    input: "X100.0g200.0Y-100.0", 
                    code: nom::error::ErrorKind::TakeUntil
                }))
            ),
            
        ];
        
        datas.iter().for_each(|&(input, ref expected)| {
            let result = positioning_mode(input);

            assert_eq!(result, *expected);
        })
    }

    #[test]
    fn line_x_normal() {
        let datas = [
            (
                "G90X100.0Y-100.0",
                ("Y-100.0", ('X', "100.0"))
            ),
            (
                "X-100.0000G91Y-100.0",
                ("G91Y-100.0", ('X', "-100.0000"))
            ),
            (
                "G91.2Y-100.0X100",
                ("", ('X', "100"))
            ),
            (
                "X0G0Y-100.0",
                ("G0Y-100.0", ('X', "0"))
            ),
        ];
        
        datas.iter().for_each(|&(input, expected)| {
            let result = x(input);

            assert_eq!(result.unwrap(), expected);

        })
    }

    #[test]
    fn line_x_abnormality() {
        let datas = [
            (
                "G90Y-100.0",
                Err(nom::Err::Error(nom::error::Error {
                    input: "G90Y-100.0", 
                    code: nom::error::ErrorKind::TakeUntil
                }))
            ),
            (
                "G90XY-100.0",
                Err(nom::Err::Error(nom::error::Error {
                    input: "Y-100.0", 
                    code: nom::error::ErrorKind::Digit
                }))
            ),
            (
                "G90x100.0Y-100.0",
                Err(nom::Err::Error(nom::error::Error {
                    input: "G90x100.0Y-100.0", 
                    code: nom::error::ErrorKind::TakeUntil
                }))
            ),
            
        ];
        
        datas.iter().for_each(|&(input, ref expected)| {
            let result = x(input);

            assert_eq!(result, *expected);
        })
    }

    #[test]
    fn line_y_normal() {
        let datas = [
            (
                "G90X100.0Y-100.0",
                ("", ('Y', "-100.0"))
            ),
            (
                "Y100.0000G91X-100.0",
                ("G91X-100.0", ('Y', "100.0000"))
            ),
            (
                "G91.2Y100X100",
                ("X100", ('Y', "100"))
            ),
            (
                "X0G0Y0",
                ("", ('Y', "0"))
            ),
        ];
        
        datas.iter().for_each(|&(input, expected)| {
            let result = y(input);

            assert_eq!(result.unwrap(), expected);

        })
    }

    #[test]
    fn line_y_abnormality() {
        let datas = [
            (
                "G90X-100.0",
                Err(nom::Err::Error(nom::error::Error {
                    input: "G90X-100.0", 
                    code: nom::error::ErrorKind::TakeUntil
                }))
            ),
            (
                "G90YX-100.0",
                Err(nom::Err::Error(nom::error::Error {
                    input: "X-100.0", 
                    code: nom::error::ErrorKind::Digit
                }))
            ),
            (
                "G90y100.0X-100.0",
                Err(nom::Err::Error(nom::error::Error {
                    input: "G90y100.0X-100.0", 
                    code: nom::error::ErrorKind::TakeUntil
                }))
            ),
            
        ];
        
        datas.iter().for_each(|&(input, ref expected)| {
            let result = y(input);

            assert_eq!(result, *expected);
        })
    }
}
