#pragma once
#include "math.hpp"

// opaque types
struct IKeyValuesSystem;

// defined types
struct KeyValues;

struct KeyValues {
    enum types_t {
        TYPE_NONE = 0,
        TYPE_STRING,
        TYPE_INT,
        TYPE_FLOAT,
        TYPE_PTR,
        TYPE_WSTRING,
        TYPE_COLOR,
        TYPE_UINT64,
        TYPE_COMPILED_INT_BYTE,
        TYPE_COMPILED_INT_0,
        TYPE_COMPILED_INT_1,
        TYPE_NUMTYPES, 
    };

    enum MergeKeyValuesOp_t {
        MERGE_KV_ALL,
        MERGE_KV_UPDATE,
        MERGE_KV_DELETE,
        MERGE_KV_BORROW,
    };

    uint32_t m_iKeyName : 24;
    uint32_t m_iKeyNameCaseSensitive1 : 8;
    char *m_sValue;
    wchar_t *m_wsValue;

    union {
        int m_iValue;
        float m_flValue;
        void *m_pValue;
        unsigned char m_Color[4];
    };
    
    char m_iDataType;
    char m_bHasEscapeSequences;
    uint16_t m_iKeyNameCaseSensitive2;
    IKeyValuesSystem *m_pKeyValuesSystem;
    bool m_bOwnsCustomKeyValuesSystem;
    KeyValues *m_pPeer;
    KeyValues *m_pSub;
    KeyValues *m_pChain;
};
