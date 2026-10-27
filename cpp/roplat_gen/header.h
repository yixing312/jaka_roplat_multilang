#ifndef ROPLAT_MSG_HEADER_H
#define ROPLAT_MSG_HEADER_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include "stamp.h"
#include "frame_id.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Header {
    Stamp stamp;
    uint64_t seq;
    FrameId frame;
    uint32_t reserved;
} Header;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_HEADER_H
