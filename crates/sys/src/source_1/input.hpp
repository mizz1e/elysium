#pragma once
#include "buttoncode.hpp"
#include "ehandle.hpp"
#include "iinput.hpp"
#include "inputstacksystem.hpp"
#include "kbutton.hpp"
#include "math.hpp"
#include "shareddefs.hpp"
#include "threadtools.hpp"
#include "usercmd.hpp"

typedef uint32_t CRC32_t;

struct C_BaseCombatWeapon;
struct bf_read;
struct bf_write;

struct CVerifiedUserCmd {
    CUserCmd m_cmd;
    CRC32_t m_crc;
};

struct CKeyboardKey {
    char name[32];
    kbutton_t *pkey;
    CKeyboardKey *next;
};

struct CInput {
    virtual void Init_All() = 0;
    virtual void Shutdown_All() = 0;
    virtual int GetButtonBits(bool bResetState) = 0;
    virtual void CreateMove (int sequence_number, float input_sample_frametime, bool active) = 0;
    virtual void ExtraMouseSample(float frametime, bool active) = 0;
    virtual bool WriteUsercmdDeltaToBuffer(int nSlot, bf_write *buf, int from, int to, bool isnewcommand) = 0;
    virtual void EncodeUserCmdToBuffer(int nSlot, bf_write& buf, int slot) = 0;
    virtual void DecodeUserCmdFromBuffer(int nSlot, bf_read& buf, int slot) = 0;
    virtual CUserCmd *GetUserCmd(int nSlot, int sequence_number) = 0;
    virtual void MakeWeaponSelection(C_BaseCombatWeapon *weapon) = 0;
    virtual float KeyState(kbutton_t *key) = 0;
    virtual int KeyEvent(int down, ButtonCode_t keynum, const char *pszCurrentBinding) = 0;
    virtual kbutton_t *FindKey(const char *name) = 0;
    virtual void ControllerCommands() = 0;
    virtual void Joystick_Advanced(bool bSilent) = 0;
    virtual void Joystick_SetSampleTime(float frametime) = 0;
    virtual float Joystick_GetPitch() = 0;
    virtual float Joystick_GetYaw() = 0;
    virtual void Joystick_Querry(float &forward, float &side, float &pitch, float &yaw) = 0;
    virtual void Joystick_ForceRecentering(int nStick, bool bSet = true) = 0;
    virtual void IN_SetSampleTime(float frametime) = 0;
    virtual void AccumulateMouse(int nSlot) = 0;
    virtual void ActivateMouse() = 0;
    virtual void DeactivateMouse() = 0;
    virtual void ClearStates() = 0;
    virtual float GetLookSpring() = 0;
    virtual void GetFullscreenMousePos(int *mx, int *my, int *unclampedx = nullptr, int *unclampedy = nullptr) = 0;
    virtual void SetFullscreenMousePos(int mx, int my) = 0;
    virtual void ResetMouse() = 0;
    virtual float GetLastForwardMove() = 0;
    virtual void ClearInputButton(int bits) = 0;
    virtual void CAM_Think() = 0;
    virtual int CAM_IsThirdPerson(int nSlot = -1) = 0;
    virtual bool CAM_IsThirdPersonOverview(int nSlot = -1) = 0;
    virtual void CAM_GetCameraOffset(Vec3& ofs) = 0;
    virtual void CAM_ToThirdPerson() = 0;
    virtual void CAM_ToFirstPerson() = 0;
    virtual void CAM_ToThirdPersonShoulder() = 0;
    virtual void CAM_ToThirdPersonOverview() = 0;
    virtual void CAM_StartMouseMove() = 0;
    virtual void CAM_EndMouseMove() = 0;
    virtual void CAM_StartDistance() = 0;
    virtual void CAM_EndDistance() = 0;
    virtual int CAM_InterceptingMouse() = 0;
    virtual void CAM_Command(int command) = 0;
    virtual void CAM_ToOrthographic() = 0;
    virtual bool CAM_IsOrthographic() const = 0;
    virtual void CAM_OrthographicSize(float& w, float& h) const = 0;
    virtual void LevelInit() = 0;
    virtual void CAM_SetCameraThirdData(CameraThirdData_t *pCameraData, const Vec3 &vecCameraOffset) = 0;
    virtual void CAM_CameraThirdThink() = 0;    
    virtual void CheckPaused(CUserCmd *cmd) = 0;
    virtual void CheckSplitScreenMimic(int nSlot, CUserCmd *cmd, CUserCmd *pPlayer0Command) = 0;
    virtual void Init_Camera() = 0;
    virtual void ApplyMouse(int nSlot, Vec3& viewangles, CUserCmd *cmd, float mouse_x, float mouse_y) = 0;
    virtual void JoyStickMove(float frametime, CUserCmd *cmd) = 0;
    virtual void SteamControllerMove(float frametime, CUserCmd *cmd) = 0;
    virtual bool ControllerModeActive() = 0;
    virtual bool JoyStickActive() = 0;
    virtual void JoyStickSampleAxes(float &forward, float &side, float &pitch, float &yaw, bool &bAbsoluteYaw, bool &bAbsolutePitch) = 0;
    virtual void JoyStickThirdPersonPlatformer(CUserCmd *cmd, float &forward, float &side, float &pitch, float &yaw) = 0;
    virtual void JoyStickTurn(CUserCmd *cmd, float &yaw, float &pitch, float frametime, bool bAbsoluteYaw, bool bAbsolutePitch) = 0;
    virtual void JoyStickForwardSideControl(float forward, float side, float &joyForwardMove, float &joySideMove) = 0;
    virtual void JoyStickApplyMovement(CUserCmd *cmd, float joyForwardMove, float joySideMove) = 0;
    virtual void GetWindowCenter(int&x, int& y) = 0;

    typedef struct {
        unsigned int AxisFlags;
        unsigned int AxisMap;
        unsigned int ControlMap;
    } joy_axis_t;

    enum {
        GAME_AXIS_NONE = 0,
        GAME_AXIS_FORWARD,
        GAME_AXIS_PITCH,
        GAME_AXIS_SIDE,
        GAME_AXIS_YAW,
        MAX_GAME_AXES
    };

    enum {
        CAM_COMMAND_NONE = 0,
        CAM_COMMAND_TOTHIRDPERSON = 1,
        CAM_COMMAND_TOFIRSTPERSON = 2
    };

    enum {
        MOUSE_ACCEL_THRESHHOLD1 = 0,
        MOUSE_ACCEL_THRESHHOLD2,
        MOUSE_SPEED_FACTOR,
        NUM_MOUSE_PARAMS,
    };

    bool m_fMouseInitialized;
    bool m_fMouseActive;
    bool m_fJoystickAdvancedInit;
    bool m_bControllerMode;
    float m_fAccumulatedMouseMove;
    float m_lastAutoAimValue;

    struct PerUserInput_t {
        float m_flAccumulatedMouseXMovement;
        float m_flAccumulatedMouseYMovement;
        float m_flPreviousMouseXPosition;
        float m_flPreviousMouseYPosition;
        float m_flRemainingJoystickSampleTime;
        float m_flKeyboardSampleTime;
        float m_flSpinFrameTime;
        float m_flSpinRate;
        float m_flLastYawAngle;
        joy_axis_t m_rgAxes[MAX_JOYSTICK_AXES];
        bool m_fCameraInterceptingMouse;
        bool m_fCameraInThirdPerson;
        bool m_fCameraMovingWithMouse;
        Vec3 m_vecCameraOffset;
        bool m_fCameraDistanceMove;
        int m_nCameraOldX;
        int m_nCameraOldY;
        int m_nCameraX;
        int m_nCameraY;
        bool m_CameraIsOrthographic;
        bool m_CameraIsThirdPersonOverview;
        Vec3 m_angPreviousViewAngles;
        Vec3 m_angPreviousViewAnglesTilt;
        float m_flLastForwardMove;
        int m_nClearInputState;
        CUserCmd *m_pCommands;
        CVerifiedUserCmd *m_pVerifiedCommands;
        CHandle< C_BaseCombatWeapon > m_hSelectedWeapon;
        CameraThirdData_t *m_pCameraThirdData;
        int m_nCamCommand;
        float m_flPreviousJoystickForwardMove;
        float m_flPreviousJoystickSideMove;
        float m_flPreviousJoystickYaw;
        float m_flPreviousJoystickPitch;
        bool m_bPreviousJoystickUseAbsoluteYaw;
        bool m_bPreviousJoystickUseAbsolutePitch;
        bool m_bForceJoystickRecentering[2];
    };

    bool m_fRestoreSPI;
    int m_rgOrigMouseParms[NUM_MOUSE_PARAMS];
    int m_rgNewMouseParms[NUM_MOUSE_PARAMS];
    bool m_rgCheckMouseParam[NUM_MOUSE_PARAMS];
    bool m_fMouseParmsValid;
    CKeyboardKey *m_pKeys;
    PerUserInput_t m_PerUser[MAX_SPLITSCREEN_PLAYERS];
    InputContextHandle_t m_hInputContext;
    CThreadFastMutex m_IKContactPointMutex;
};
