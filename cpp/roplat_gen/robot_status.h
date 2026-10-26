#ifndef ROPLAT_MSG_ROBOTSTATUS_H
#define ROPLAT_MSG_ROBOTSTATUS_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct RobotStatus {
    uint32_t mode;
    uint32_t error_code;
} RobotStatus;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_ROBOTSTATUS_H
