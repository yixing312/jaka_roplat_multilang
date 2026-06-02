#ifndef JAKA_ROPLAT_MULTILANG_CPP_SPATIAL_CURVE_H
#define JAKA_ROPLAT_MULTILANG_CPP_SPATIAL_CURVE_H

#include <cmath>
#include "../roplat_gen/cpp_spatial_curve_base.h"
#include "../roplat_gen/curve_batch.h"
#include "../roplat_gen/trajectory_plan.h"
#include "roplat/node.h"

class CppSpatialCurve : public CppSpatialCurveBase,
                        public roplat::Node<TrajectoryPlan, CurveBatch> {
public:
    CppSpatialCurve() = default;
    ~CppSpatialCurve() override = default;

    using Input = TrajectoryPlan;
    using Output = CurveBatch;

    CurveBatch process(const TrajectoryPlan& input) override {
        CurveBatch out{};
        out.seq = input.seq;
        out.done = input.done;

        constexpr double kPi = 3.14159265358979323846;
        constexpr int kSamples = 8;
        const double turns = getTurns() <= 0.0 ? 1.0 : getTurns();
        const double phase = input.phase;
        const double base_angle = 2.0 * kPi * turns * phase;

        for (int i = 0; i < kSamples; ++i) {
            const double lookahead = static_cast<double>(i) / static_cast<double>(kSamples - 1);
            const double angle = base_angle + lookahead * 0.7;
            out.point_x[i] = getRadius_m() * std::cos(angle);
            out.point_y[i] = getRadius_m() * std::sin(angle);
            out.point_z[i] = getHeight_m() * std::sin(0.5 * angle);
        }

        for (int i = 0; i < 6; ++i) {
            out.target_joint[i] = input.base_joint[i];
        }
        out.target_joint[0] += input.amplitude_rad * std::sin(base_angle);
        out.target_joint[1] += 0.5 * input.amplitude_rad * std::cos(base_angle);
        out.target_joint[2] += 0.35 * input.amplitude_rad * std::sin(2.0 * base_angle);
        out.target_joint[3] += 0.2 * input.amplitude_rad * std::cos(0.5 * base_angle);
        return out;
    }
};

#endif
