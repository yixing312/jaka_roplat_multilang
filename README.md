# JAKA multilingual planning example

The Rust application uses `#[roplat::system]` to connect a Rust motion tick,
Python trajectory planner, C++ spatial curve generator, and Rust command node.
`src/planning.rs` defines a finite planning rhythm: each cycle reads robot state,
computes a valid point, and appends it to a trajectory. Only after planning has
completed and node lifecycle hooks have run does the existing `move_traj` call
send the entire trajectory to the real or simulated robot.

The planning System creates and owns its nodes. The borrowed robot remains owned
by the caller; the System does not enable, reset, or shut down the robot.
Framework failures recover the node tuple before propagation and prevent motion
execution. This is a preplanning example, not streaming realtime control.

The layout expects sibling `drives` and `roplat` checkouts. The build script uses
the matching local `roplat_build` to generate the C++ and Python bridges.
Python nodes import generated messages through their canonical module names
(for example `from trajectory_plan import TrajectoryPlan`), matching the bridge's
cached ctypes classes. Importing the same file as `roplat_gen.trajectory_plan`
would create a second Python class and fail the exact output-type check.

Compile without starting hardware or a GUI:

```sh
BULLET_SKIP_ASSET_EXPORT=1 cargo check -p jaka_roplat_multilang --examples
```

`examples/real.rs` requires explicit `JAKA_REAL_ENABLE=1` and `JAKA_IP`. Do not run
it as an unattended test. `examples/sim.rs` uses a separate Bullet stepping thread;
its simulator timing policy has not been redesigned in this alignment.

AI application authors should use the `roplat-development` skill from
[roplat-skills](https://github.com/Robot-Exp-Platform/roplat-skills) and retain the
System DSL rather than replacing the graph with manual `process()` calls.
