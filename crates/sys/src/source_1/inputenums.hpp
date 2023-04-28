#pragma once

#define MAX_BUTTONSAMPLE 32768

enum {
    MAX_JOYSTICKS = 4,
    MOUSE_BUTTON_COUNT = 5,
    MAX_NOVINT_DEVICES = 2,
};

enum JoystickAxis_t {
    JOY_AXIS_X = 0,
    JOY_AXIS_Y,
    JOY_AXIS_Z,
    JOY_AXIS_R,
    JOY_AXIS_U,
    JOY_AXIS_V,
    MAX_JOYSTICK_AXES,
};

enum JoystickDeadzoneMode_t {
    JOYSTICK_DEADZONE_CROSS = 0,
    JOYSTICK_DEADZONE_SQUARE = 1,
};

enum {
    MS_WM_XBUTTONDOWN    = 0x020B,
    MS_WM_XBUTTONUP = 0x020C,
    MS_WM_XBUTTONDBLCLK    = 0x020D,
    MS_MK_BUTTON4 = 0x0020,
    MS_MK_BUTTON5 = 0x0040,
};

enum InputEventType_t {
    IE_ButtonPressed = 0,
    IE_ButtonReleased,
    IE_ButtonDoubleClicked,
    IE_AnalogValueChanged,

    IE_FirstSystemEvent = 100,
    IE_Quit = IE_FirstSystemEvent,
    IE_ControllerInserted,
    IE_ControllerUnplugged,
    IE_Close,
    IE_WindowSizeChanged,
    IE_PS_CameraUnplugged,
    IE_PS_Move_OutOfView,

    IE_FirstUIEvent = 200,
    IE_LocateMouseClick = IE_FirstUIEvent,
    IE_SetCursor,
    IE_KeyTyped,
    IE_KeyCodeTyped,
    IE_InputLanguageChanged,
    IE_IMESetWindow,
    IE_IMEStartComposition,
    IE_IMEComposition,
    IE_IMEEndComposition,
    IE_IMEShowCandidates,
    IE_IMEChangeCandidates,
    IE_IMECloseCandidates,
    IE_IMERecomputeModes,
    IE_OverlayEvent,

    IE_FirstVguiEvent = 1000,
    IE_FirstAppEvent = 2000,
};

struct InputEvent_t {
    int m_nType;
    int m_nTick;
    int m_nData;
    int m_nData2;
    int m_nData3;
};

#define MAX_STEAM_CONTROLLERS 16

typedef enum {
    SK_NULL,
    SK_BUTTON_A,
    SK_BUTTON_B,
    SK_BUTTON_X,
    SK_BUTTON_Y,
    SK_BUTTON_UP,
    SK_BUTTON_RIGHT,
    SK_BUTTON_DOWN,
    SK_BUTTON_LEFT,
    SK_BUTTON_LEFT_BUMPER,
    SK_BUTTON_RIGHT_BUMPER,
    SK_BUTTON_LEFT_TRIGGER,
    SK_BUTTON_RIGHT_TRIGGER,
    SK_BUTTON_LEFT_GRIP,
    SK_BUTTON_RIGHT_GRIP,
    SK_BUTTON_LPAD_TOUCH,
    SK_BUTTON_RPAD_TOUCH,
    SK_BUTTON_LPAD_CLICK,
    SK_BUTTON_RPAD_CLICK,
    SK_BUTTON_LPAD_UP,
    SK_BUTTON_LPAD_RIGHT,
    SK_BUTTON_LPAD_DOWN,
    SK_BUTTON_LPAD_LEFT,
    SK_BUTTON_RPAD_UP, 
    SK_BUTTON_RPAD_RIGHT, 
    SK_BUTTON_RPAD_DOWN, 
    SK_BUTTON_RPAD_LEFT, 
    SK_BUTTON_SELECT, 
    SK_BUTTON_START, 
    SK_BUTTON_STEAM, 
    SK_BUTTON_INACTIVE_START, 
    SK_MAX_KEYS
} sKey_t;

enum ESteamPadAxis {
    LEFTPAD_AXIS_X,
    LEFTPAD_AXIS_Y,
    RIGHTPAD_AXIS_X,
    RIGHTPAD_AXIS_Y,
    LEFT_TRIGGER_AXIS,
    RIGHT_TRIGGER_AXIS,
    GYRO_AXIS_PITCH,
    GYRO_AXIS_ROLL,
    GYRO_AXIS_YAW,
    MAX_STEAMPADAXIS = GYRO_AXIS_YAW
};

enum {
    LASTINPUT_KBMOUSE = 0,
    LASTINPUT_CONTROLLER = 1,
    LASTINPUT_STEAMCONTROLLER = 2
};
