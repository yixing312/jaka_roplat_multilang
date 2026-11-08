#ifndef ROPLAT_MSG_CARTESIANLIMITS_H
#define ROPLAT_MSG_CARTESIANLIMITS_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct CartesianLimits {
    double linear_velocity_max;
    double linear_acceleration_max;
    double linear_jerk_max;
    double angular_velocity_max;
    double angular_acceleration_max;
    double angular_jerk_max;
} CartesianLimits;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_CARTESIANLIMITS_H
