#ifndef ROPLAT_MSG_IMU_H
#define ROPLAT_MSG_IMU_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include "quaternion.h"
#include "vector3.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Imu {
    uint32_t valid;
    uint32_t reserved;
    Quaternion orientation;
    Vector3 angular_velocity;
    Vector3 linear_acceleration;
} Imu;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_IMU_H
