use jaka_roplat_multilang::planning::plan_trajectory;
use libjaka::JakaMini2;
use robot_behavior::behavior::*;

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
    let trajectory = plan_trajectory(|| robot.state(), 750)
        .await
        .expect("failed to plan JAKA trajectory");

    robot
        .move_traj::<JointSpace<6>>(trajectory)
        .expect("failed to execute planned JAKA trajectory");
}
