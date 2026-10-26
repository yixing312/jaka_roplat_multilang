#ifndef ROPLAT_MSG_CAMERAINFO_H
#define ROPLAT_MSG_CAMERAINFO_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct CameraInfo {
    uint32_t width;
    uint32_t height;
    double k[9];
    uint32_t distortion_model;
    uint32_t reserved;
    double d[8];
} CameraInfo;

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ROPLAT_MSG_CAMERAINFO_H
