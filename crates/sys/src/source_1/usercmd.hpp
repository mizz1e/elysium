#pragma once
#include "math.hpp"

struct CUserCmd {
    virtual ~CUserCmd() {};

    int command_number;
    int tick_count;
    Vec3 viewangles;
    Vec3 aimdirection;
    float forwardmove;   
    float sidemove; 
    float upmove; 
    int buttons; 
    uint8_t impulse; 
    int weaponselect;    
    int weaponsubtype;
    int random_seed;
    int server_random_seed;
    short mousedx;
    short mousedy;
    bool hasbeenpredicted;
    Vec3 headangles;
    Vec3 headoffset;
};
