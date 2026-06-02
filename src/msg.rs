#[roplat::roplat_msg]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionTick {
    pub seq: u64,
    pub time_s: f64,
    pub dt_s: f64,
    pub current_joint: [f64; 6],
    pub done: u8,
}

#[roplat::roplat_msg]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrajectoryPlan {
    pub seq: u64,
    pub time_s: f64,
    pub phase: f64,
    pub amplitude_rad: f64,
    pub base_joint: [f64; 6],
    pub done: u8,
}

#[roplat::roplat_msg]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurveBatch {
    pub seq: u64,
    pub point_x: [f64; 8],
    pub point_y: [f64; 8],
    pub point_z: [f64; 8],
    pub target_joint: [f64; 6],
    pub done: u8,
}
