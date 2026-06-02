use crate::msg::{CurveBatch, MotionTick, TrajectoryPlan};

#[roplat::node(lang = "py", input(tick, MotionTick), output(plan, TrajectoryPlan))]
pub struct PyTrajectoryPlanner {
    pub duration_s: f64,
    pub amplitude_rad: f64,
}

impl PyTrajectoryPlanner {
    pub fn new() -> Self {
        Self { duration_s: 6.0, amplitude_rad: 0.08, ..Self::default() }
    }
}

#[roplat::node(lang = "cpp", input(plan, TrajectoryPlan), output(curve, CurveBatch))]
pub struct CppSpatialCurve {
    pub radius_m: f64,
    pub height_m: f64,
    pub turns: f64,
}

impl CppSpatialCurve {
    pub fn new() -> Self {
        Self {
            radius_m: 0.06,
            height_m: 0.04,
            turns: 1.0,
            ..Self::default()
        }
    }
}
