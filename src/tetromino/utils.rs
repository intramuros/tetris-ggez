pub mod body_generators {
    use crate::tetromino::GRID_SIZE;
    pub fn generate_i() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -4),
            (GRID_SIZE.0 / 2, -3),
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2, -1),
        ]
    }
    pub fn generate_l() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -3),
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 - 1, -1),
        ]
    }
    pub fn generate_j() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -3),
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 + 1, -1),
        ]
    }
    pub fn generate_o() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2 - 1, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 - 1, -1),
        ]
    }
    pub fn generate_s() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2 + 1, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 - 1, -1),
        ]
    }
    pub fn generate_t() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2 + 1, -2),
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 - 1, -2),
        ]
    }
    pub fn generate_z() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2 - 1, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 + 1, -1),
        ]
    }
}
