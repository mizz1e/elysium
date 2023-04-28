#pragma once
#include "color.hpp"

const int MAX_LOGGING_IDENTIFIER_LENGTH = 32;
const int MAX_LOGGING_CHANNEL_COUNT = 256;
const int MAX_LOGGING_TAG_COUNT = 1024;
const int MAX_LOGGING_TAG_CHARACTER_COUNT = 8192;
const int MAX_LOGGING_LISTENER_COUNT = 16;

typedef int LoggingChannelID_t;

const LoggingChannelID_t INVALID_LOGGING_CHANNEL_ID = -1;

enum LoggingSeverity_t {
    LS_MESSAGE = 0,
    LS_WARNING = 1,
    LS_ASSERT = 2,
    LS_ERROR = 3,
    LS_HIGHEST_SEVERITY = 4,
};

enum LoggingResponse_t {
    LR_CONTINUE,
    LR_DEBUGGER,
    LR_ABORT,
};

enum LoggingChannelFlags_t {
    LCF_CONSOLE_ONLY = 0x00000001,
    LCF_DO_NOT_ECHO = 0x00000002,
};

struct LoggingContext_t {
    LoggingChannelID_t m_ChannelID;
    LoggingChannelFlags_t m_Flags;
    LoggingSeverity_t m_Severity;
    Color m_Color;
};

struct ILoggingListener {
    virtual void Log(const LoggingContext_t *pContext, const char *pMessage) = 0;
};

typedef LoggingResponse_t (*LoggingSystem_Log)(LoggingChannelID_t channelID, LoggingSeverity_t severity, const char *pMessageFormat, ...);
