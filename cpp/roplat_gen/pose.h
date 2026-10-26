#ifndef ROPLAT_MSG_POSE_H
#define ROPLAT_MSG_POSE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Pose {
    Vector3 position;
    Quaternion orientation;
} Pose;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_POSE_H
