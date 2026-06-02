use jaka_roplat_multilang::{
    nodes::{JakaMotionCommand, MotionTickNode},
    puppet::{CppSpatialCurve, PyTrajectoryPlanner},
};
use libjaka::JakaMini2;
use robot_behavior::behavior::{Arm, ArmMotionRhythm, Robot};

fn connect_real_robot() -> JakaMini2 {
    if std::env::var("JAKA_REAL_ENABLE").as_deref() != Ok("1") {
        panic!(
            "set JAKA_REAL_ENABLE=1 and JAKA_IP=<robot-ip> before running the real robot example"
        );
    }

    let ip = std::env::var("JAKA_IP").expect("missing JAKA_IP");
    let mut robot = JakaMini2::new(&ip);
    robot.enable().expect("failed to enable JAKA robot");
    robot
        .set_scale(0.05)
        .expect("failed to set JAKA speed scale");
    robot
}

#[roplat::system]
async fn main() {
    let robot = connect_real_robot();
    let mut motion = ArmMotionRhythm::new(robot);
    let mut tick = MotionTickNode::new(750);
    let mut planner = PyTrajectoryPlanner::new();
    let mut curve = CppSpatialCurve::new();
    let mut command = JakaMotionCommand::new();

    motion >> |sample| sample >> tick >> planner >> curve >> command;
}
