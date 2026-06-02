#ifndef ROPLAT_MSG_MOTIONTICK_H
#define ROPLAT_MSG_MOTIONTICK_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct MotionTick {
    uint64_t seq;
    double time_s;
    double dt_s;
    double current_joint[6];
    uint8_t done;
} MotionTick;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_MOTIONTICK_H
