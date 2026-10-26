#ifndef ROPLAT_MSG_GRIPPERSTATE_H
#define ROPLAT_MSG_GRIPPERSTATE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct GripperState {
    double width;
    double max_width;
    uint32_t grasped;
    uint32_t reserved;
} GripperState;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_GRIPPERSTATE_H
