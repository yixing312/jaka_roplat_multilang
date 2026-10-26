#ifndef ROPLAT_MSG_TWIST_H
#define ROPLAT_MSG_TWIST_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Twist {
    Vector3 linear;
    Vector3 angular;
} Twist;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_TWIST_H
