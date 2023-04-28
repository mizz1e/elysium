#pragma once
#include "shareddefs.hpp"

struct kbutton_t {
    struct Split_t {
        int down[2]; 
        int state; 
    };

    Split_t m_PerUser[MAX_SPLITSCREEN_PLAYERS];
};
