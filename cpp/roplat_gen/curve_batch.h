#ifndef ROPLAT_MSG_CURVEBATCH_H
#define ROPLAT_MSG_CURVEBATCH_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct CurveBatch {
    uint64_t seq;
    double point_x[8];
    double point_y[8];
    double point_z[8];
    double target_joint[6];
    uint8_t done;
} CurveBatch;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_CURVEBATCH_H
