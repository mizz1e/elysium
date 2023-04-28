#pragma once
#include "dt_common.hpp"

struct CRecvProxyData;
struct RecvProp;
struct RecvTable;

typedef void (*RecvVarProxyFn)(const CRecvProxyData *pData, void *pStruct, void *pOut);
typedef void (*ArrayLengthRecvProxyFn)(void *pStruct, int objectID, int currentArrayLength);
typedef void (*DataTableRecvVarProxyFn)(const RecvProp *pProp, void **pOut, void *pData, int objectID);

struct CRecvProxyData {
    const RecvProp *m_pRecvProp;
    DVariant m_Value;
    int m_iElement;
    int m_ObjectID;
};

struct RecvProp {
    char *m_pVarName;
    SendPropType m_RecvType;
    int m_Flags;
    int m_StringBufferSize;
    bool m_bInsideArray;
    const void *m_pExtraData;
    RecvProp *m_pArrayProp;
    ArrayLengthRecvProxyFn m_ArrayLengthProxy;
    RecvVarProxyFn m_ProxyFn;
    DataTableRecvVarProxyFn m_DataTableProxyFn;
    RecvTable *m_pDataTable;
    int m_Offset;
    int m_ElementStride;
    int m_nElements;
    const char *m_pParentArrayPropName;
};

struct RecvTable {
    RecvProp *m_pProps;
    int m_nProps;
    CRecvDecoder *m_pDecoder;
    char *m_pNetTableName;
    bool m_bInitialized;
    bool m_bInMainList;
};
