//scraps of some functions usable when executing md calculation

// adjust_periodic
pub fn adjust_periodic(dx: f64, dy: f64, dz: f64) -> (f64, f64, f64) {
    // adjust in respect to the periodic boundrary condition
    const L: f64 = 10.0;
    const LH: f64 = L * 0.5;

    if dx < -LH {
        let  dx = dx + L;
    } if dx > LH {
        let dx = dx - L;
    } if dy < -LH {
        let dy = dy + L;
    } if dy > LH {
        let dy = dy - L;
    } if dz < -LH {
        let dz = dz + L;
    } if dz > LH {
        let dz = dz - L;
    }
    (dx, dy, dz)
}