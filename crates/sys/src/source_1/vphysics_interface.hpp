#pragma once
#include "isoundemittersystembase.hpp"

// public/vphysics_interface.h

struct surfacephysicsparams_t {
    float friction;
    float elasticity;
    float density;
    float thickness;
    float dampening;
};

struct surfaceaudioparams_t {
    float reflectivity;
    float hardnessFactor;
    float roughnessFactor;
    float roughThreshold;
    float hardThreshold;
    float hardVelocityThreshold;
};

struct surfacesoundnames_t {
    unsigned short walkStepLeft;
    unsigned short walkStepRight;
    unsigned short runStepLeft;
    unsigned short runStepRight;
    unsigned short impactSoft;
    unsigned short impactHard;
    unsigned short scrapeSmooth;
    unsigned short scrapeRough;
    unsigned short bulletImpact;
    unsigned short rolling;
    unsigned short breakSound;
    unsigned short strainSound;
};

struct surfacesoundhandles_t {
    HSOUNDSCRIPTHASH walkStepLeft;
    HSOUNDSCRIPTHASH walkStepRight;
    HSOUNDSCRIPTHASH runStepLeft;
    HSOUNDSCRIPTHASH runStepRight;
    HSOUNDSCRIPTHASH impactSoft;
    HSOUNDSCRIPTHASH impactHard;
    HSOUNDSCRIPTHASH scrapeSmooth;
    HSOUNDSCRIPTHASH scrapeRough;
    HSOUNDSCRIPTHASH bulletImpact;
    HSOUNDSCRIPTHASH rolling;
    HSOUNDSCRIPTHASH breakSound;
    HSOUNDSCRIPTHASH strainSound;
};

struct surfacegameprops_t {
    float maxSpeedFactor;
    float jumpFactor;
    float penetrationModifier;
    float damageModifier;
    unsigned short material;
    unsigned char climbable;
    unsigned char pad;
};

struct surfacedata_t {
    surfacephysicsparams_t physics;
    surfaceaudioparams_t audio;
    surfacesoundnames_t sounds;
    surfacegameprops_t game;
    surfacesoundhandles_t soundhandles;
};

struct ISaveRestoreOps;

struct IPhysicsSurfaceProps {
    virtual ~IPhysicsSurfaceProps(void) {}
    virtual int ParseSurfaceData(const char *pFilename, const char *pTextfile) = 0;
    virtual int SurfacePropCount(void) const = 0;
    virtual int GetSurfaceIndex(const char *pSurfacePropName) const = 0;
    virtual void GetPhysicsProperties(int surfaceDataIndex, float *density, float *thickness, float *friction, float *elasticity) const = 0;
    virtual surfacedata_t *GetSurfaceData(int surfaceDataIndex) = 0;
    virtual const char *GetString(unsigned short stringTableIndex) const = 0;
    virtual const char *GetPropName(int surfaceDataIndex) const = 0;
    virtual void SetWorldMaterialIndexTable(int *pMapArray, int mapSize) = 0;
    virtual void GetPhysicsParameters(int surfaceDataIndex, surfacephysicsparams_t *pParamsOut) const = 0;
    virtual ISaveRestoreOps *GetMaterialIndexDataOps() const = 0;
};
