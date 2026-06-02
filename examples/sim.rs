use std::{path::PathBuf, thread, time::Duration};

use jaka_roplat_multilang::{
    nodes::{JakaMotionCommand, MotionTickNode},
    puppet::{CppSpatialCurve, PyTrajectoryPlanner},
};
use libjaka::JakaMini2;
use robot_behavior::behavior::{
    AddRobot, AddSearchPath, ArmMotionRhythm, EntityBuilder, PhysicsEngine,
};
use rsbullet::{Mode, RsBullet, RsBulletRobot};

fn create_sim_robot() -> RsBulletRobot<JakaMini2> {
    let mode = if std::env::var("JAKA_ROPLAT_SIM_GUI").as_deref() == Ok("1") {
        Mode::Gui
    } else {
        Mode::Direct
    };

    let mut physics = RsBullet::new(mode).expect("failed to connect to Bullet");
    let asserts_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../asserts");
    physics
        .add_search_path(&asserts_dir)
        .expect("failed to add drives/asserts as Bullet search path")
        .set_gravity([0.0, 0.0, -10.0])
        .expect("failed to set Bullet gravity")
        .set_step_time(Duration::from_secs_f64(1.0 / 240.0))
        .expect("failed to set Bullet step time");

    let robot: RsBulletRobot<JakaMini2> = physics
        .robot_builder::<JakaMini2>("jaka_roplat")
        .base_fixed(true)
        .load()
        .expect("failed to load JAKA URDF in Bullet");

    thread::Builder::new()
        .name("jaka-roplat-bullet-step".to_string())
        .spawn(move || {
            loop {
                if let Err(err) = physics.step() {
                    eprintln!("Bullet stepping stopped: {err}");
                    break;
                }
                thread::sleep(Duration::from_secs_f64(1.0 / 240.0));
            }
        })
        .expect("failed to spawn Bullet stepping thread");

    robot
}

#[roplat::system]
async fn main() {
    let robot = create_sim_robot();
    let mut motion = ArmMotionRhythm::new(robot).with_period(Duration::from_secs_f64(1.0 / 125.0));
    let mut tick = MotionTickNode::new(750);
    let mut planner = PyTrajectoryPlanner::new();
    let mut curve = CppSpatialCurve::new();
    let mut command = JakaMotionCommand::new();

    motion >> |sample| sample >> tick >> planner >> curve >> command;
}
