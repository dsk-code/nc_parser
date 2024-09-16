/// 位置決め方式の構造体
/// アブソリュートまたはインクレメンタル
#[derive(Debug, PartialEq)]
pub struct PositioningMode {
    code: String,
    incremental: bool,
}

impl PositioningMode {
    /// G90またはG91のコードを検証し、インスタンス化
    pub fn new(code: &str) -> Option<Self> {
        match code {
            "G90" => Some(Self { code: code.to_string(), incremental: false }),
            "G91" => Some(Self { code: code.to_string(), incremental: true}),
            _ => None,
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
        let data = [("G90", false), ("G91", true)];
        data.iter().for_each(|&(code, incremental)| {
            let expected = PositioningMode { code: code.to_string(), incremental};
            let result = PositioningMode::new(code);
            assert_eq!(result.unwrap(), expected);
        });
    }

    #[test]
    fn positioning_mode_new_abnormality() {
        let data = [
            "G92",
            "G80", 
            "X90",
            "X91",
            "g90",
            "g91",
        ];
        data.iter().for_each(|&code| {
            let result = PositioningMode::new(code);
            assert_eq!(result, None);
        });
    }

    #[test]
    fn positioning_mode_get_incremental() {
        let data = [("G90", false), ("G91", true)];
        data.iter().for_each(|&(code, incremental)| {
            let positioning_mode = PositioningMode::new(code);
            let result = positioning_mode.unwrap().get_incremental();
            assert_eq!(result, incremental);
        });
    }
}
