/// 位置決め方式の構造体
/// アブソリュートまたはインクレメンタル
#[derive(Debug, PartialEq)]
pub struct PositioningMode {
    code: char,
    value: i32,
    incremental: bool,
}

impl PositioningMode {
    /// G90またはG91のコードを検証し、インスタンス化
    pub fn new(code: char, value: i32) -> Option<Self> {
        if code == 'G' && value == 90 {
            Some(Self { code, value, incremental: false })
        } else if code == 'G' && value == 91 {
            Some(Self { code, value, incremental: true})
        }else {
            None
        }
    }

    /// モードがインクレメンタルであるかの真偽値を返す
    pub fn get_incremental(&self) -> bool {
        self.incremental
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positioning_mode_new_normal() {
        let data = [('G', 90, false), ('G', 91, true)];
        data.iter().for_each(|&(code, value, incremental)| {
            let expected = PositioningMode { code, value, incremental};
            let result = PositioningMode::new(code, value);
            assert_eq!(result.unwrap(), expected);
        });
    }

    #[test]
    fn positioning_mode_new_abnormality() {
        let data = [
            ('G', 92),
            ('G', 80), 
            ('X', 90),
            ('X', 91),
        ];
        data.iter().for_each(|&(code, value)| {
            let result = PositioningMode::new(code, value);
            assert_eq!(result, None);
        });
    }

    #[test]
    fn positioning_mode_get_incremental() {
        let data = [('G', 90, false), ('G', 91, true)];
        data.iter().for_each(|&(code, value, incremental)| {
            let positioning_mode = PositioningMode::new(code, value);
            let result = positioning_mode.unwrap().get_incremental();
            assert_eq!(result, incremental);
        });
    }
}
