//scraps of some functions usable when executing md calculation

// adjust_periodic
pub fn adjust_periodic(mut dx: f64, mut dy: f64, mut dz: f64) -> (f64, f64, f64) {
    // adjust in respect to the periodic boundrary condition
    const L: f64 = 10.0;
    const LH: f64 = L * 0.5;

    if dx < -LH {
        dx += L;
    } if dx > LH {
        dx -= L;
    } if dy < -LH {
        dy += L;
    } if dy > LH {
        dy -= L;
    } if dz < -LH {
        dz += L;
    } if dz > LH {
        dz -= L;
    }
    (dx, dy, dz)
}