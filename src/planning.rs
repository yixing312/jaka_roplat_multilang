//! Build a complete trajectory with a finite System before sending any motion.

use std::{future::Future, time::Duration};

use robot_behavior::{ArmState, RobotResult};
use roplat::rhythm::Rhythm;
use roplat::{Completion, Execution, ExecutionContext, Lifecycle, RoplatError};

use crate::{
    nodes::{JakaMotionCommand, MotionTickNode},
    puppet::{CppSpatialCurve, PyTrajectoryPlanner},
};

/// This rhythm reads the device during planning, but never publishes commands.
/// The caller retains robot ownership and executes `move_traj` after it returns.
struct PlanningRhythm<F> {
    read_state: F,
    max_ticks: u64,
}

impl<F> Lifecycle for PlanningRhythm<F> {
    type Error = RoplatError;
}

impl<Read> Rhythm for PlanningRhythm<Read>
where
    Read: FnMut() -> RobotResult<ArmState<6>> + Send,
{
    type Input = ();
    type Yield = (ArmState<6>, Duration);
    type Feed = ([f64; 6], bool);
    type Output = Vec<[f64; 6]>;

    async fn drive<N, F, Fut>(
        &mut self,
        mut nodes: N,
        mut domain: F,
        (): (),
        context: ExecutionContext,
    ) -> (Execution<Self::Output>, N)
    where
        N: Send,
        F: FnMut(N, Self::Yield, ExecutionContext) -> Fut + Send,
        Fut: Future<Output = (Execution<Self::Feed>, N)> + Send,
    {
        let period = Duration::from_secs_f64(1.0 / 125.0);
        let mut trajectory = Vec::new();
        for _ in 0..self.max_ticks {
            if context.is_stopping() {
                return (Ok(Completion::Stopped), nodes);
            }
            let state = match (self.read_state)() {
                Ok(state) => state,
                Err(error) => {
                    context.request_stop();
                    return (Err(RoplatError::Io(std::io::Error::other(error))), nodes);
                }
            };
            let (outcome, returned) = domain(nodes, (state, period), context.clone()).await;
            nodes = returned;
            let (command, done) = match outcome {
                Ok(Completion::Completed(feed)) => feed,
                Ok(Completion::Stopped) => {
                    context.request_stop();
                    return (Ok(Completion::Stopped), nodes);
                }
                Err(error) => {
                    context.request_stop();
                    return (Err(error), nodes);
                }
            };
            trajectory.push(command);
            if done {
                break;
            }
        }
        if context.is_stopping() {
            (Ok(Completion::Stopped), nodes)
        } else {
            (Ok(Completion::Completed(trajectory)), nodes)
        }
    }
}

/// Plan at most `max_ticks` points. Nodes are enabled and closed by this System;
/// the externally borrowed robot is neither enabled nor closed here.
#[roplat::system]
pub async fn plan_trajectory<Read>(
    read_state: Read,
    max_ticks: u64,
) -> roplat::RoplatResult<Vec<[f64; 6]>>
where
    Read: FnMut() -> RobotResult<ArmState<6>> + Send,
{
    let mut planning = PlanningRhythm { read_state, max_ticks };
    let mut tick = MotionTickNode::new(max_ticks);
    let mut planner = PyTrajectoryPlanner::new();
    let mut curve = CppSpatialCurve::new();
    let mut command = JakaMotionCommand::new();

    planning
        >> |observation| {
            observation >> tick >> planner >> curve >> command;
            command.output
        };

    Ok(planning.output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use robot_behavior::RobotException;

    #[derive(Default)]
    struct MockRobot {
        reads: usize,
        fail_read: bool,
    }

    impl MockRobot {
        fn read_state(&mut self) -> RobotResult<ArmState<6>> {
            self.reads += 1;
            if self.fail_read {
                Err(RobotException::NetworkError("read failed".into()))
            } else {
                Ok(ArmState::default())
            }
        }
    }

    #[tokio::test]
    async fn system_runs_rust_python_cpp_pipeline_without_hardware() {
        let mut robot = MockRobot::default();
        let points = plan_trajectory(|| robot.read_state(), 3).await.unwrap();
        assert_eq!(points.len(), 3);
        assert_eq!(robot.reads, 3);
        assert!(points.iter().flatten().all(|value| value.is_finite()));
        assert_ne!(points[0], points[2]);
    }

    #[tokio::test]
    async fn planning_collects_only_until_final_valid_point() {
        let mut robot = MockRobot::default();
        let mut planning = PlanningRhythm { read_state: || robot.read_state(), max_ticks: 10 };
        let (outcome, nodes) = planning
            .drive(
                0,
                |nodes, (_, period), _| async move {
                    assert_eq!(period, Duration::from_secs_f64(1.0 / 125.0));
                    (
                        Ok(Completion::Completed(([nodes as f64; 6], nodes == 2))),
                        nodes + 1,
                    )
                },
                (),
                ExecutionContext::new(),
            )
            .await;
        assert!(
            matches!(outcome, Ok(Completion::Completed(points)) if points == vec![[0.0;6], [1.0;6], [2.0;6]])
        );
        assert_eq!(nodes, 3);
        assert_eq!(robot.reads, 3);
    }

    #[tokio::test]
    async fn domain_failure_returns_nodes_without_a_partial_trajectory() {
        let mut robot = MockRobot::default();
        let mut planning = PlanningRhythm { read_state: || robot.read_state(), max_ticks: 10 };
        let context = ExecutionContext::new();
        let (outcome, nodes) = planning
            .drive(
                0,
                |nodes, _, _| async move {
                    (
                        Err(RoplatError::Arithmetic("planning failed".into())),
                        nodes + 1,
                    )
                },
                (),
                context.clone(),
            )
            .await;
        assert!(
            matches!(outcome, Err(RoplatError::Arithmetic(message)) if message == "planning failed")
        );
        assert_eq!(nodes, 1);
        assert!(context.is_stopping());
    }

    #[tokio::test]
    async fn read_failure_preserves_the_typed_device_error() {
        let mut robot = MockRobot { fail_read: true, ..MockRobot::default() };
        let mut planning = PlanningRhythm { read_state: || robot.read_state(), max_ticks: 10 };
        let (outcome, nodes) = planning
            .drive(
                7,
                |_, _, _| async { panic!("domain must not run") },
                (),
                ExecutionContext::new(),
            )
            .await;
        let Err(RoplatError::Io(error)) = outcome else {
            panic!("device error required")
        };
        assert!(
            matches!(error.get_ref().and_then(|source| source.downcast_ref::<RobotException>()), Some(RobotException::NetworkError(message)) if message == "read failed")
        );
        assert_eq!(nodes, 7);
    }
}

#[cfg(test)]
mod stop_test {
    use super::*;
    #[tokio::test]
    async fn domain_stop_notifies_context_and_returns_nodes() {
        let mut planning = PlanningRhythm { read_state: || Ok(ArmState::default()), max_ticks: 3 };
        let context = ExecutionContext::new();
        let (outcome, nodes) = planning
            .drive(
                7,
                |nodes, _, _| async move { (Ok(Completion::Stopped), nodes + 1) },
                (),
                context.clone(),
            )
            .await;
        assert!(matches!(outcome, Ok(Completion::Stopped)));
        assert_eq!(nodes, 8);
        assert!(context.is_stopping());
    }
}
