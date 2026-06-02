pub mod msg;
pub mod nodes;
pub mod puppet;

use std::time::Duration;

use robot_behavior::behavior::{ArmMotionRhythm, ArmRealtimeControl};

pub async fn run_jaka_multilang_motion<A>(arm: A, max_ticks: u64, sim_period: Option<Duration>)
where
    A: ArmRealtimeControl<6> + Send,
{
    let rhythm = match sim_period {
        Some(period) => ArmMotionRhythm::new(arm).with_period(period),
        None => ArmMotionRhythm::new(arm),
    };

    roplat::system_item! {
        let mut motion = rhythm;
        let mut tick = nodes::MotionTickNode::new(max_ticks);
        let mut planner = puppet::PyTrajectoryPlanner::new();
        let mut curve = puppet::CppSpatialCurve::new();
        let mut command = nodes::JakaMotionCommand::new();

        motion >> |sample| {
            sample >> tick >> planner >> curve >> command
        };
    }
}
