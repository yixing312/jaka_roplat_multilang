#ifndef ROPLAT_MSG_WRENCH_H
#define ROPLAT_MSG_WRENCH_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Wrench {
    Vector3 force;
    Vector3 torque;
} Wrench;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_WRENCH_H
