use hdk::prelude::*;
use crate::*;


///
#[derive(Serialize, Deserialize, Debug)]
pub struct ZomeSignal {
    pub from: AgentPubKey,
    pub pulses: Vec<ZomeSignalProtocol>,
}


///
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub enum ZomeSignalProtocol {
    System(SystemSignalProtocol), // From "System"
    Entry(EntryPulse), // From self
    Link(LinkPulse),   // From self
    Tip(TipProtocol),  // From Other peer
}

impl ZomeSignalProtocol {
    // Reset ValidationBy to None
    pub fn clear_validation(&mut self) {
        match self {
            ZomeSignalProtocol::Entry(e) => {
                e.clear_validation();
            },
            ZomeSignalProtocol::Link(l) => {
                l.clear_validation();
            },
            ZomeSignalProtocol::Tip(t) => {
                match t {
                    TipProtocol::Entry(e) => {
                        e.clear_validation();
                    },
                    TipProtocol::Link(l) => {
                        l.clear_validation();
                    },
                    _ => (),
                }
            }
            _ => (),
        };
    }
}


/// Protocol for notifying the ViewModel (UI) of system level events
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
#[serde(tag = "type")]
pub enum SystemSignalProtocol {
    PostCommitNewStart {app_entry_type: String},
    PostCommitNewEnd {app_entry_type: String, succeeded: bool},
    PostCommitDeleteStart {app_entry_type: String},
    PostCommitDeleteEnd {app_entry_type: String, succeeded: bool},
    SelfCallStart {zome_name: String, fn_name: String},
    SelfCallEnd {zome_name: String, fn_name: String, succeeded: bool},
}


/// Used by UI ONLY. That's why we use B64 here.
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub enum TipProtocol {
    Ping(AgentPubKey),
    Pong(AgentPubKey),
    Entry(EntryPulse),
    Link(LinkPulse),
    App(SerializedBytes),
}


