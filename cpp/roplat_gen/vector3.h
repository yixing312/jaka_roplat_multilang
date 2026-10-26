#ifndef ROPLAT_MSG_VECTOR3_H
#define ROPLAT_MSG_VECTOR3_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Vector3 {
    double x;
    double y;
    double z;
} Vector3;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_VECTOR3_H
