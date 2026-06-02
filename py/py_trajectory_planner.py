from roplat_gen.node import Node
from roplat_gen.motion_tick import MotionTick
from roplat_gen.py_trajectory_planner_base import PyTrajectoryPlannerBase
from roplat_gen.trajectory_plan import TrajectoryPlan


class PyTrajectoryPlanner(PyTrajectoryPlannerBase, Node):
    def process(self, *args, **kwargs) -> TrajectoryPlan:
        input_data: MotionTick = args[0]
        out = TrajectoryPlan()
        duration = max(float(self.duration_s), 1e-6)
        phase = min(max(float(input_data.time_s) / duration, 0.0), 1.0)

        out.seq = input_data.seq
        out.time_s = input_data.time_s
        out.phase = phase
        out.amplitude_rad = float(self.amplitude_rad)
        out.done = input_data.done
        for index in range(6):
            out.base_joint[index] = input_data.current_joint[index]
        return out
