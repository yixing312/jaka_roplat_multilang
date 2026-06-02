use std::time::Duration;

use libjaka::JakaMini2;
use robot_behavior::{ArmState, MotionType, behavior::ArmParam};
use roplat::Node;

use crate::msg::{CurveBatch, MotionTick};

pub struct MotionTickNode {
    seq: u64,
    elapsed_s: f64,
    max_ticks: u64,
}

impl MotionTickNode {
    pub fn new(max_ticks: u64) -> Self {
        Self { seq: 0, elapsed_s: 0.0, max_ticks }
    }
}

impl Node for MotionTickNode {
    type Input = (ArmState<6>, Duration);
    type Output = MotionTick;
    type Error = roplat::RoplatError;

    async fn process(&mut self, (state, dt): Self::Input) -> Self::Output {
        self.seq += 1;
        self.elapsed_s += dt.as_secs_f64();
        let current_joint = state.measured.joint.unwrap_or(JakaMini2::JOINT_DEFAULT);
        MotionTick {
            seq: self.seq,
            time_s: self.elapsed_s,
            dt_s: dt.as_secs_f64(),
            current_joint,
            done: u8::from(self.seq >= self.max_ticks),
        }
    }
}

pub struct JakaMotionCommand {
    log_every: u64,
}

impl JakaMotionCommand {
    pub fn new() -> Self {
        Self { log_every: 40 }
    }
}

impl Node for JakaMotionCommand {
    type Input = CurveBatch;
    type Output = (MotionType<6>, bool);
    type Error = roplat::RoplatError;

    async fn process(&mut self, input: Self::Input) -> Self::Output {
        let mut target = input.target_joint;
        for (index, joint) in target.iter_mut().enumerate() {
            *joint = joint.clamp(JakaMini2::JOINT_MIN[index], JakaMini2::JOINT_MAX[index]);
        }

        if input.seq == 1 || input.seq % self.log_every == 0 || input.done != 0 {
            println!(
                "[jaka-roplat] seq={} first_point=({:.3}, {:.3}, {:.3}) q0={:.3} done={}",
                input.seq,
                input.point_x[0],
                input.point_y[0],
                input.point_z[0],
                target[0],
                input.done != 0
            );
        }

        (MotionType::Joint(target), input.done != 0)
    }
}
