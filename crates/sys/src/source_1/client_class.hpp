#pragma once
#include "dt_recv.hpp"

struct IClientNetworkable;

typedef IClientNetworkable *(*CreateClientClassFn)(int entnum, int serialNum);
typedef IClientNetworkable *(*CreateEventFn)();

struct ClientClass {
    CreateClientClassFn m_pCreateFn;
    CreateEventFn m_pCreateEventFn;
    char *m_pNetworkName;
    RecvTable *m_pRecvTable;
    ClientClass *m_pNext;
    int m_ClassID;
    const char *m_pMapClassname;
};
