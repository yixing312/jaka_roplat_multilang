#ifndef ROPLAT_MSG_ACCEL_H
#define ROPLAT_MSG_ACCEL_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Accel {
    Vector3 linear;
    Vector3 angular;
} Accel;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_ACCEL_H
