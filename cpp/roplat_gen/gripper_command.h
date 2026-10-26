#ifndef ROPLAT_MSG_GRIPPERCOMMAND_H
#define ROPLAT_MSG_GRIPPERCOMMAND_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct GripperCommand {
    double width;
    double speed;
    double force;
} GripperCommand;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_GRIPPERCOMMAND_H
