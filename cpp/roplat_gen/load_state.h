#ifndef ROPLAT_MSG_LOADSTATE_H
#define ROPLAT_MSG_LOADSTATE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct LoadState {
    double mass;
    double center_of_mass[3];
    double inertia[9];
} LoadState;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_LOADSTATE_H
