use machine::state::State;

/// multiple or immediate
pub fn mori(state: &mut State, x: u8, y: u8, z: u8) {
    // Load first operand
    let mut op1: u64 = state.gpr[y].into();

    // Make z mutable
    let mut z = z;

    // Execute
    let mut matrix_op1:     [[bool; 8]; 8] = [[false; 8]; 8];
    let mut matrix_z:       [[bool; 8]; 8] = [[false; 8]; 8];
    let mut matrix_result:  [[bool; 8]; 8] = [[false; 8]; 8];

    for i in 0..8 {
        for j in 0..8 {
            matrix_op1[7 - i][7 - j] = get_last_bit(op1);
            matrix_z[7 - i][7 - j] = get_last_bit(z as u64);
            op1 >>= 1;
            z >>= 1;
        }
    }

    for i in 0..8 {
        for j in 0..8 {
            let mut entry = false;
            for k in 0..8 {
                entry = entry | (matrix_op1[i][k] & matrix_z[k][j]);
            }
            matrix_result[i][j] = entry;
        }
    }

    let mut result: u64 = 0;

    for i in 0..8 {
        for j in 0..8 {
            result <<= 1;
            result += matrix_result[i][j] as u64;

        }
    }

    // Store result
    state.gpr[x] = result.into();
}

fn get_last_bit(bits: u64) -> bool {
    bits % 2 != 0
}
