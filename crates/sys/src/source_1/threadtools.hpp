#pragma once

struct CThreadFastMutex {
    volatile uint32_t m_ownerID;
    int m_depth;
};
