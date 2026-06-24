use jaka_roplat_multilang::{
    nodes::{JakaMotionCommand, MotionTickNode},
    puppet::{CppSpatialCurve, PyTrajectoryPlanner},
};
use libjaka::JakaMini2;
use robot_behavior::behavior::*;
use roplat::Node;
use std::time::Duration;

fn connect_real_robot() -> JakaMini2 {
    if std::env::var("JAKA_REAL_ENABLE").as_deref() != Ok("1") {
        panic!(
            "set JAKA_REAL_ENABLE=1 and JAKA_IP=<robot-ip> before running the real robot example"
        );
    }

    let ip = std::env::var("JAKA_IP").expect("missing JAKA_IP");
    let mut robot = JakaMini2::new(&ip);
    robot.enable().expect("failed to enable JAKA robot");
    robot.set_scale(0.05);
    robot
}

#[tokio::main]
async fn main() {
    let mut robot = connect_real_robot();
    let mut tick = MotionTickNode::new(750);
    let mut planner = PyTrajectoryPlanner::new();
    let mut curve = CppSpatialCurve::new();
    let mut command = JakaMotionCommand::new();
    let period = Duration::from_secs_f64(1.0 / 125.0);
    let mut trajectory = Vec::new();

    loop {
        let state = robot.state().expect("failed to read JAKA state");
        let tick_msg = tick.process((state, period)).await;
        let plan = planner.process(tick_msg).await;
        let curve_batch = curve.process(plan).await;
        let (joint, done) = command.process(curve_batch).await;
        trajectory.push(joint);
        if done {
            break;
        }
    }

    robot
        .move_traj::<JointSpace<6>>(trajectory)
        .expect("failed to execute planned JAKA trajectory");
}
