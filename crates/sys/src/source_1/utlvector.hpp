//! public/tier1/utlvector.h

#pragma once
#include "utlmemory.hpp"

template<typename T>
struct CUtlVector {
    CUtlMemory<T> m_Memory;
    int m_Size;
    T *m_pElements;
};
