#ifndef ROPLAT_MSG_CARTESIANLIMITS_H
#define ROPLAT_MSG_CARTESIANLIMITS_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct CartesianLimits {
    double linear_velocity;
    double linear_acceleration;
    double linear_jerk;
    double angular_velocity;
    double angular_acceleration;
    double angular_jerk;
} CartesianLimits;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_CARTESIANLIMITS_H
