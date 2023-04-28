#pragma once
#include "math.hpp"

struct CRecvDecoder;

typedef enum {
    DPT_Int = 0,
    DPT_Float,
    DPT_Vector,
    DPT_VectorXY,
    DPT_String,
    DPT_Array,
    DPT_DataTable,
    DPT_Int64,
    DPT_NUMSendPropTypes
} SendPropType;

struct DVariant {
    union {
        float m_Float;
        long m_Int;
        char *m_pString;
        void *m_pData;
        float m_Vector[3];
        int64_t m_Int64;
    };

    SendPropType m_Type;
};
