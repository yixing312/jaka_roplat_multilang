#ifndef ROPLAT_MSG_STAMP_H
#define ROPLAT_MSG_STAMP_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Stamp {
    int64_t nanos;
    uint32_t clock;
    uint32_t reserved;
} Stamp;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_STAMP_H
