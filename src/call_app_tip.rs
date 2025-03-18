use hdk::hdi::hdk_extern;
use hdk::info::agent_info;
use hdk::map_extern::ExternResult;
use hdk::prelude::*;
use crate::*;

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CallAppTipInput {
  pub app_tip: SerializedBytes,
  pub recipient: AgentPubKey,
  pub zome_name: String,
}


/// Use call_remote() for sending an AppTip.
/// Used by UI to make sure tip has been received, since remote_signal does not guarantee that.
#[hdk_extern]
pub fn call_app_tip(input: CallAppTipInput) -> ExternResult<()> {
  //std::panic::set_hook(Box::new(zome_panic_hook));
  debug!("call_app_tip() recipient: {:?}", input.recipient);
  /// Pre-conditions: Don't call yourself (otherwise could get concurrency issues)
  let me = agent_info()?.agent_latest_pubkey;
  if me == input.recipient {
    return Ok(());
  }
  /// Prepare payload
  let pulse = ZomeSignalProtocol::Tip(TipProtocol::App(input.app_tip));
  /// call
  let res = call_remote(
    input.recipient,
    ZomeName::from(input.zome_name),
    "recv_remote_signal".into(),
    None,
    ExternIO::encode(pulse).unwrap(),
  );
  if let Err(e) = res {
    error!("recv_remote_signal() failed during call_app_tip(): {:?}", e);
    return Err(wasm_error!("recv_remote_signal() failed during call_app_tip()"));
  }
  trace!("calling remote recv_remote_signal() DONE");
  Ok(())
}
