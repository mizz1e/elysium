#pragma once
#include "math.hpp"

struct CameraThirdData_t {
    float m_flPitch;
    float m_flYaw;
    float m_flDist;
    float m_flLag;
    Vec3 m_vecHullMin;
    Vec3 m_vecHullMax;
};
