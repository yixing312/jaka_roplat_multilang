#ifndef ROPLAT_MSG_INPUTAXES_H
#define ROPLAT_MSG_INPUTAXES_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct InputAxes {
    double axes[6];
    uint32_t buttons;
    uint32_t reserved;
} InputAxes;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_INPUTAXES_H
