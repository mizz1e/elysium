/// `common/netmessages_signon.h`
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub enum SignOnState {
    /// No state yet, about to connect.
    #[default]
    None = 0,

    /// Client challenging server, all OOB packets.
    Challenge = 1,

    /// Client is connected to server, netchan is ready.
    Connected = 2,

    /// Recieved string tables, and server info.
    New = 3,

    /// Received sign on buffers.
    Prespawn = 4,

    /// Ready to receive entity packets.
    Spawn = 5,

    /// Fully connected, first non-delta packet received.
    Full = 6,

    /// Server is changing level, please wait.
    ChangeLevel = 7,
}

impl SignOnState {
    pub fn is_active(&self) -> bool {
        matches!(self, SignOnState::Full)
    }

    pub fn is_connected(&self) -> bool {
        matches!(
            self,
            SignOnState::Connected
                | SignOnState::New
                | SignOnState::Prespawn
                | SignOnState::Spawn
                | SignOnState::Full
                | SignOnState::ChangeLevel
        )
    }

    pub fn is_spawned(&self) -> bool {
        matches!(
            self,
            SignOnState::New
                | SignOnState::Prespawn
                | SignOnState::Spawn
                | SignOnState::Full
                | SignOnState::ChangeLevel
        )
    }
}
