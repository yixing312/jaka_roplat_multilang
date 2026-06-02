use std::{path::PathBuf, thread, time::Duration};

use anyhow::Result;
use jaka_roplat_multilang::run_jaka_multilang_motion;
use libjaka::JakaMini2;
use robot_behavior::behavior::{AddRobot, AddSearchPath, EntityBuilder, PhysicsEngine};
use rsbullet::{Mode, RsBullet, RsBulletRobot};

#[tokio::main]
async fn main() -> Result<()> {
    let mode = if std::env::var("JAKA_ROPLAT_SIM_GUI").as_deref() == Ok("1") {
        Mode::Gui
    } else {
        Mode::Direct
    };

    let mut physics = RsBullet::new(mode)?;
    let asserts_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../asserts");
    physics
        .add_search_path(&asserts_dir)?
        .set_gravity([0.0, 0.0, -10.0])?
        .set_step_time(Duration::from_secs_f64(1.0 / 240.0))?;

    let robot: RsBulletRobot<JakaMini2> = physics
        .robot_builder::<JakaMini2>("jaka_roplat")
        .base_fixed(true)
        .load()?;

    let _physics_thread = thread::Builder::new()
        .name("jaka-roplat-bullet-step".to_string())
        .spawn(move || loop {
            if let Err(err) = physics.step() {
                eprintln!("Bullet stepping stopped: {err}");
                break;
            }
            thread::sleep(Duration::from_secs_f64(1.0 / 240.0));
        })?;

    run_jaka_multilang_motion(robot, 750, Some(Duration::from_secs_f64(1.0 / 125.0))).await;
    Ok(())
}
