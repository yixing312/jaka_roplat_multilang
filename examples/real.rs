use std::time::Duration;

use anyhow::{Result, bail};
use jaka_roplat_multilang::run_jaka_multilang_motion;
use libjaka::JakaMini2;
use robot_behavior::behavior::{Arm, Robot};

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("JAKA_REAL_ENABLE").as_deref() != Ok("1") {
        bail!("set JAKA_REAL_ENABLE=1 and JAKA_IP=<robot-ip> before running the real robot example");
    }

    let ip = std::env::var("JAKA_IP")?;
    let mut robot = JakaMini2::new(&ip);
    robot.enable()?;
    robot.set_scale(0.05)?;

    run_jaka_multilang_motion(robot, 750, None).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(())
}
