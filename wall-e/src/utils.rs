pub fn opposite_sign(n: f32) -> f32 {
    if n.is_sign_negative() {
        1.0
    } else {
        -1.0
    }
}
