use crate::error::Error;

/// X座標の構造体
#[derive(Debug, PartialEq)]
pub struct XCoordinate {
    code: char,
    value: f32,
}

impl XCoordinate {
    /// 座標がXの座標であるかを検証し、構造体インスタンス化
    pub fn new(code: char, value: f32) -> Result<Self, Error> {
        if code  == 'X' {
            Ok(Self { code, value })
        } else {
            Err(Error::InvalidCode(code))
        }
    }

    /// valueの値を返す
    pub fn get_value(&self) -> f32 {
        self.value
    }
}

/// Y座標の構造体
#[derive(Debug, PartialEq)]
pub struct YCoordinate {
    code: char,
    value: f32,
}

impl YCoordinate {
    /// 座標がYの座標であるかを検証し、構造体インスタンス化
    pub fn new(code: char, value: f32) -> Result<Self, Error> {
        if code == 'Y' {
            Ok(Self { code, value })
        } else {
            Err(Error::InvalidCode(code))
        }
    }

    /// valueの値を返す
    pub fn get_value(&self) -> f32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_coordinate_new_normal() {
        let expected = XCoordinate {code: 'X', value: 100.0 };
        let result = XCoordinate::new('X', 100.0);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn x_coordinate_new_abnormality() {
        let codes = ['x', 'Y', 'G', 'z'];
        codes.iter().for_each(|&code| {
            let result = XCoordinate::new(code, 100.0);
            assert_eq!(result, Err(Error::InvalidCode(code)));
        });
        
    }

    #[test]
    fn x_coordinate_get_value() {
        let expected = 100.0;
        let xcode = XCoordinate {code: 'X', value: 100.0 };
        let result = xcode.get_value();
        assert_eq!(result, expected);
    }

    #[test]
    fn y_coordinate_new_normal() {
        let expected = YCoordinate {code: 'Y', value: 100.0 };
        let result = YCoordinate::new('Y', 100.0);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn y_coordinate_new_abnormality() {
        let codes = ['y', 'X', 'G', 'z'];
        codes.iter().for_each(|&code| {
            let result = YCoordinate::new(code, 100.0);
            assert_eq!(result, Err(Error::InvalidCode(code)));
        });
        
    }

    #[test]
    fn y_coordinate_get_value() {
        let expected = 100.0;
        let ycode = YCoordinate {code: 'Y', value: 100.0 };
        let result = ycode.get_value();
        assert_eq!(result, expected);
    }
}
