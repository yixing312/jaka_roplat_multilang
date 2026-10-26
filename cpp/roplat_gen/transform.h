#ifndef ROPLAT_MSG_TRANSFORM_H
#define ROPLAT_MSG_TRANSFORM_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Transform {
    FrameId parent;
    FrameId child;
    Pose pose;
} Transform;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_TRANSFORM_H
