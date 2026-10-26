#ifndef ROPLAT_MSG_SPATIALSAMPLE_H
#define ROPLAT_MSG_SPATIALSAMPLE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct SpatialSample {
    uint32_t valid;
    uint32_t reserved;
    Pose pose;
    Twist twist;
    Accel accel;
    Wrench wrench;
} SpatialSample;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_SPATIALSAMPLE_H
