#pragma once

typedef struct player_info_s {
    uint64            version;
    uint64            xuid;
    char            name[MAX_PLAYER_NAME_LENGTH];
    int                userID;
    char            guid[SIGNED_GUID_LEN + 1];
    uint32            friendsID;
    char            friendsName[MAX_PLAYER_NAME_LENGTH];
    bool            fakeplayer;
    bool            ishltv;
    bool            isreplay;
    CRC32_t            customFiles[MAX_CUSTOM_FILES];
    unsigned char    filesDownloaded;
} player_info_t;

typedef struct player_info_s_version_1 {
    uint64            xuid;
    char            name[32];
    int                userID;
    char            guid[32 + 1];
    uint32            friendsID;
    char            friendsName[32];
    bool            fakeplayer;
    bool            ishltv;
    CRC32_t            customFiles[4];
    unsigned char    filesDownloaded;
} player_info_t_version_1;
