#ifndef ROPLAT_MSG_QUATERNION_H
#define ROPLAT_MSG_QUATERNION_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Quaternion {
    double x;
    double y;
    double z;
    double w;
} Quaternion;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_QUATERNION_H
