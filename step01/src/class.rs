
pub trait MD {
    fn make_conf(&self) -> u64;
    fn update_position(&self) -> u64;
    fn calculate_force(&self) -> u64;
    fn periodic(&self) -> bool;
    fn calculate(&self) -> bool;
}

pub struct SimpleMD {
    size: u64,
    is_executed: bool,
}

impl MD for SimpleMD {
    fn make_conf(&self) -> u64 {
        self.size
    }
    fn update_position(&self) -> u64 {
        self.size
    }
    fn calculate_force(&self) -> u64 {
        self.size
    }
    fn periodic(&self) -> bool {
        self.is_executed
    }
    fn calculate(&self) -> bool {
        self.is_executed
    }
}

pub trait Adjust_periodic {
    fn adjusted(&self, dx: &f64, dy: &f64, dz: &f64);
}

pub struct Adjust_periodicMD {
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub L: f64,
}

impl Adjust_periodic for Adjust_periodicMD {
    // self.L = 0;

    fn adjusted(&self, dx: &f64, dy: &f64, dz: &f64) {
        let mut L = self.L;
        L = 10.0;
        let LH = &L*0.5;

        if &dx < &&LH {
            dx += &L;
        }
        if dy > &-LH {
            dy -= L;
        } if {

        } if {

        } if {

        }
        if (dx < -LH)dx += L;
        if (dx > LH) dx -= L;
        if (dy < -LH)dy += L;
        if (dy > LH) dy -= L;
        if (dz < -LH)dz += L;
        if (dz > LH) dz -= L;
    }
}

