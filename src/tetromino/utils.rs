pub(crate) mod body_generators {
    use crate::tetromino::GRID_SIZE;
    pub(crate) fn generate_i() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -2),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2, -0),
            (GRID_SIZE.0 / 2, 1),
        ]
    }
    pub(crate) fn generate_l() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2, -0),
            (GRID_SIZE.0 / 2, 1),
            (GRID_SIZE.0 / 2 - 1, 1),
        ]
    }
    pub(crate) fn generate_j() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2, -0),
            (GRID_SIZE.0 / 2, 1),
            (GRID_SIZE.0 / 2 + 1, 1),
        ]
    }
    pub(crate) fn generate_o() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2 - 1, -1),
            (GRID_SIZE.0 / 2, 0),
            (GRID_SIZE.0 / 2 - 1, 0),
        ]
    }
    pub(crate) fn generate_s() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2 + 1, -1),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2, 0),
            (GRID_SIZE.0 / 2 - 1, 0),
        ]
    }
    pub(crate) fn generate_t() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2 + 1, -1),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2, 0),
            (GRID_SIZE.0 / 2 - 1, -1),
        ]
    }
    pub(crate) fn generate_z() -> Vec<(i16, i16)> {
        vec![
            (GRID_SIZE.0 / 2 - 1, -1),
            (GRID_SIZE.0 / 2, -1),
            (GRID_SIZE.0 / 2, 0),
            (GRID_SIZE.0 / 2 + 1, 0),
        ]
    }
}
