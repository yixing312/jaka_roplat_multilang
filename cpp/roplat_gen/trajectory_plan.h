#ifndef ROPLAT_MSG_TRAJECTORYPLAN_H
#define ROPLAT_MSG_TRAJECTORYPLAN_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct TrajectoryPlan {
    uint64_t seq;
    double time_s;
    double phase;
    double amplitude_rad;
    double base_joint[6];
    uint8_t done;
} TrajectoryPlan;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_TRAJECTORYPLAN_H
