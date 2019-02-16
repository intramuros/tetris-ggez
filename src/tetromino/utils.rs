pub mod body_generators {
    use crate::tetromino::GRID_SIZE;
    pub fn generate_I() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -4),
            (GRID_SIZE.0 / 2, -3),
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2, -1),
        ]
    }
    pub fn generate_L() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -3),
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 - 1, -1),
        ]
    }
    pub fn generate_J() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -3),
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 + 1, -1),
        ]
    }
    pub fn generate_O() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2 - 1, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 - 1, -1),
        ]
    }
    pub fn generate_S() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2 + 1, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 - 1, -1),
        ]
    }
    pub fn generate_T() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2 + 1, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 - 1, -2),
        ]
    }
    pub fn generate_Z() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2 - 1, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 + 1, -1),
        ]
    }
}
