#ifndef ROPLAT_NODE_CPPSPATIALCURVE_BASE_H
#define ROPLAT_NODE_CPPSPATIALCURVE_BASE_H

#include <cstdint>
#include <cstddef>
#include "curve_batch.h"
#include "trajectory_plan.h"

extern "C" {
    void* roplat_node_cpp_spatial_curve_create();
    void  roplat_node_cpp_spatial_curve_destroy(void* ptr);
    int   roplat_node_cpp_spatial_curve_init(void* ptr);
    int   roplat_node_cpp_spatial_curve_shutdown(void* ptr);
    CurveBatch roplat_node_cpp_spatial_curve_process(void* ptr, const TrajectoryPlan* input);
    void  roplat_node_cpp_spatial_curve_set_fields(void* ptr, double radius_m, double height_m, double turns);
}

class CppSpatialCurveBase {
protected:
    double radius_m = {};
    double height_m = {};
    double turns = {};


public:
    virtual ~CppSpatialCurveBase() = default;

    double getRadius_m() const { return radius_m; }
    void setRadius_m(double value) { radius_m = value; }
    double getHeight_m() const { return height_m; }
    void setHeight_m(double value) { height_m = value; }
    double getTurns() const { return turns; }
    void setTurns(double value) { turns = value; }
};

#endif // ROPLAT_NODE_CPPSPATIALCURVE_BASE_H
