#ifndef ROPLAT_MSG_CONTACTSAMPLE_H
#define ROPLAT_MSG_CONTACTSAMPLE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include "vector3.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ContactSample {
    uint32_t valid;
    uint32_t active;
    Vector3 force;
    Vector3 point;
    Vector3 normal;
} ContactSample;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_CONTACTSAMPLE_H
