use crate::structs::coordinate::{XCoordinate, YCoordinate};
use crate::structs::positioning_mode::PositioningMode;
use crate::error::Error;
use crate::structs::line_parser::Line;

/// 読み込んだ座標の状態
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct State {
    incremental: bool,
    x: f32,
    y: f32,
}

impl State {
    /// インスタンス化
    pub fn new(
        positioning_mode: PositioningMode, 
        x_coordinate: XCoordinate, 
        y_coordinate: YCoordinate
    ) -> Self {
        let incremental = positioning_mode.get_incremental();
        let x = x_coordinate.get_value();
        let y = y_coordinate.get_value();

        Self { incremental, x, y }
    }

    /// incrementalの値を設定
    pub fn incremental_set(&mut self, incremental: PositioningMode) {
        self.incremental = incremental.get_incremental();
    }

    /// Xの値を設定
    /// アブソリュートの時はそのまま代入
    /// インクレメンタルの時は加算
    pub fn x_set(&mut self, value: XCoordinate) {
        if !self.incremental {
            self.x = value.get_value();
        } else if self.incremental {
            self.x += value.get_value();
        }
    }

    /// Yの値を設定
    /// アブソリュートの時はそのまま代入
    /// インクレメンタルの時は加算
    pub fn y_set(&mut self, value: YCoordinate) {
        if !self.incremental {
            self.y = value.get_value();
        } else if self.incremental {
            self.y += value.get_value();
        }
    }

    /// incrementalの値を返す
    pub fn get_incremental(self) -> bool {
        self.incremental
    }

    /// xの値を返す
    pub fn get_x(self) -> f32 {
        self.x
    }

    /// yの値を返す
    pub fn get_y(self) -> f32 {
        self.y
    }

    pub fn state_update(&mut self, line: Line) -> Result<Option<State>, Error> {
        let (_, (g, x, y)) = line.parsed_for_state()
            .map_err(|e| Error::InvalidParser(format!("{:?}", e)))?;
        match (g, x, y) {
            (
                Some((g_code, g_value)), 
                Some((x_code, x_value)), 
                Some((y_code, y_value))
            ) => {
                if let Some(mode) = PositioningMode::new(g_code, g_value.parse()?) {
                    self.incremental_set(mode);
                };
                self.x_set(XCoordinate::new(x_code, x_value.parse::<f32>()?)?);
                self.y_set(YCoordinate::new(y_code, y_value.parse::<f32>()?)?);

                Ok(Some(self.clone()))
            },
            (
                Some((g_code, g_value)), 
                Some((x_code, x_value)), 
                None
            ) => {
                if let Some(mode) = PositioningMode::new(g_code, g_value.parse()?) {
                    self.incremental_set(mode);
                };
                self.x_set(XCoordinate::new(x_code, x_value.parse::<f32>()?)?);

                Ok(Some(self.clone()))
            },
            (
                Some((g_code, g_value)), 
                None, 
                Some((y_code, y_value))
            ) => {
                if let Some(mode) = PositioningMode::new(g_code, g_value.parse()?) {
                    self.incremental_set(mode);
                };
                self.y_set(YCoordinate::new(y_code, y_value.parse::<f32>()?)?);

                Ok(Some(self.clone()))
            },
            (
                None, 
                Some((x_code, x_value)), 
                Some((y_code, y_value))
            ) => {
                self.x_set(XCoordinate::new(x_code, x_value.parse::<f32>()?)?);
                self.y_set(YCoordinate::new(y_code, y_value.parse::<f32>()?)?);

                Ok(Some(self.clone()))
            },
            (
                None, 
                Some((x_code, x_value)), 
                None
            ) => {
                self.x_set(XCoordinate::new(x_code, x_value.parse::<f32>()?)?);

                Ok(Some(self.clone()))
            },
            (
                None, 
                None, 
                Some((y_code, y_value))
            ) => {
                self.y_set(YCoordinate::new(y_code, y_value.parse::<f32>()?)?);

                Ok(Some(self.clone()))
            },
            (
                None, 
                None, 
                None
            ) => {
                // 値がないのでStateを返さない
                Ok(None)
            },
            (
                Some((g_code, g_value)), 
                None, 
                None
            ) => {
                // インクレメンタルの値の更新のみなのでStateを返さない
                if let Some(mode) = PositioningMode::new(g_code, g_value.parse()?) {
                    self.incremental_set(mode);
                };

                Ok(None)
            },

        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_default() {
        let result = State::default();
        assert_eq!(result, State {incremental: false, x: 0.0, y: 0.0});
    }

    #[test]
    fn state_new_normal() {
        let datas = [
            (
                State { incremental: false, x: 100.0, y: -100.0 }, 
                (('G', 90), ('X', 100.0), ('Y', -100.0)),
            ),
            (
                State { incremental: true, x: 100.0, y: -100.0 }, 
                (('G', 91), ('X', 100.0), ('Y', -100.0)),
            ),
        ];
        datas.iter().for_each(
            |&(
                expected,
                ((gcode, gvalue),(xcode, xvalue),(ycode, yvalue)),
            )| {
                let result = State::new(
                    PositioningMode::new(gcode, gvalue).unwrap(), 
                    XCoordinate::new(xcode, xvalue).unwrap(), 
                    YCoordinate::new(ycode, yvalue).unwrap()
                );

                assert_eq!(result, expected);
            });
    }

    #[test]
    fn state_incremental_set() {
        let mut expected = State { incremental: true, x: 100.0, y: -100.0 };
        let mut state = State::default();
        state.incremental_set(PositioningMode::new('G', 91).unwrap());
        assert_eq!(state.incremental, expected.incremental);

        expected.incremental = false;
        state.incremental_set(PositioningMode::new('G', 90).unwrap());
        assert_eq!(state.incremental, expected.incremental);
    }

    #[test]
    fn state_absolute_x_set() {
        let mut result = State {incremental: false, x: 0.0, y: 0.0};
        result.x_set(XCoordinate::new('X', 100.0).unwrap());
        result.x_set(XCoordinate::new('X', -100.0).unwrap());
        result.x_set(XCoordinate::new('X', 10.0).unwrap());
        result.x_set(XCoordinate::new('X', -125.0).unwrap());

        assert_eq!(result, State {incremental: false, x: -125.0, y: 0.0});
    }

    #[test]
    fn state_incremental_x_set() {
        let mut result = State {incremental: true, x: 0.0, y: 0.0};
        result.x_set(XCoordinate::new('X', 100.0).unwrap());
        result.x_set(XCoordinate::new('X', -100.0).unwrap());
        result.x_set(XCoordinate::new('X', 10.0).unwrap());
        result.x_set(XCoordinate::new('X', -125.0).unwrap());

        assert_eq!(result, State {incremental: true, x: -115.0, y: 0.0});
    }

    #[test]
    fn state_absolute_y_set() {
        let mut result = State {incremental: false, x: 0.0, y: 0.0};
        result.y_set(YCoordinate::new('Y', 100.0).unwrap());
        result.y_set(YCoordinate::new('Y', -100.0).unwrap());
        result.y_set(YCoordinate::new('Y', 10.0).unwrap());
        result.y_set(YCoordinate::new('Y', -125.0).unwrap());

        assert_eq!(result, State {incremental: false, x: 0.0, y: -125.0});
    }

    #[test]
    fn state_incremental_y_set() {
        let mut result = State {incremental: true, x: 0.0, y: 0.0};
        result.y_set(YCoordinate::new('Y', 100.0).unwrap());
        result.y_set(YCoordinate::new('Y', -100.0).unwrap());
        result.y_set(YCoordinate::new('Y', 10.0).unwrap());
        result.y_set(YCoordinate::new('Y', -125.0).unwrap());

        assert_eq!(result, State {incremental: true, x: 0.0, y: -115.0});
    }

    #[test]
    fn state_get_incremental() {
        let datas = [
            (State 
                {incremental: true, x: 0.0, y: 0.0},
                true
            ),
            (State 
                {incremental: false, x: 0.0, y: 0.0},
                false
            )
        ];

        datas.iter().for_each(|&(
            state,
            expected
        )| {
            assert_eq!(state.get_incremental(), expected);
        })
    }

    #[test]
    fn state_get_x() {
        let datas = [
            (State 
                {incremental: true, x: 100.0, y: 0.0},
                100.0
            ),
            (State 
                {incremental: false, x: -125.0, y: 0.0},
                -125.0
            )
        ];

        datas.iter().for_each(|&(
            state,
            expected
        )| {
            assert_eq!(state.get_x(), expected);
        })
    }

    #[test]
    fn state_get_y() {
        let datas = [
            (State 
                {incremental: true, x: 0.0, y: 100.0},
                100.0
            ),
            (State 
                {incremental: false, x: 0.0, y: -125.0},
                -125.0
            )
        ];

        datas.iter().for_each(|&(
            state,
            expected
        )| {
            assert_eq!(state.get_y(), expected);
        })
    }

    #[test]
    fn state_state_update() {
        let datas = [
            (
                ("Z-500.0G91X100.0Y-100.0W-700.0"),
                Some(State {incremental: true, x: 200.0, y: -200.0})
            ),
            (
                ("Z-500.0G91X100.0W-700.0"),
                Some(State {incremental: true, x: 200.0, y: -100.0})
            ),
            (
                ("Z-500.0G91Y-100.0W-700.0"),
                Some(State {incremental: true, x: 100.0, y: -200.0})
            ),
            (
                ("Z-500.0X100.0Y-100.0W-700.0"),
                Some(State {incremental: false, x: 100.0, y: -100.0})
            ),
            (
                ("Z-500.0X100.0W-700.0"),
                Some(State {incremental: false, x: 100.0, y: -100.0})
            ),
            (
                ("Z-500.0Y-100.0W-700.0"),
                Some(State {incremental: false, x: 100.0, y: -100.0})
            ),
            (
                ("Z-500.0W-700.0"),
                None
            ),
            (
                ("Z-500.0G91W-700.0"),
                None
            )
        ];
        datas.iter().for_each(|&(line_str, expected)| {
            let line = Line::new(line_str.to_string());
            let mut state = State::new(
                PositioningMode::new('G', 90).unwrap(), 
                XCoordinate::new('X', 100.0).unwrap(), 
                YCoordinate::new('Y', -100.0).unwrap()
            );
            let result = state.state_update(line);

            assert_eq!(result.unwrap(), expected);
        })

    }
}
