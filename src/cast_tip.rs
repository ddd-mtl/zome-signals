use hdk::prelude::*;
use zome_utils::*;
use crate::*;


#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CastTipInput {
  pub tip: TipProtocol,
  pub peers: Vec<AgentPubKey>,
}

///
#[hdk_extern]
fn cast_tip(input: CastTipInput) -> ExternResult<()> {
  std::panic::set_hook(Box::new(zome_panic_hook));
  debug!("Casting tip {:?} to {:?}", input.tip, input.peers);
  /// Pre-conditions: Don't call yourself (otherwise could get concurrency issues)
  let me = agent_info()?.agent_latest_pubkey;
  let filtered = input.peers.into_iter().filter(|agent| agent != &me).collect();
  /// Prepare payload
  let pulse = ZomeSignalProtocol::Tip(input.tip.clone());
  /// Signal peers
  trace!("calling remote recv_remote_signal() to {:?}", filtered);
  trace!("tip = '{:?}'", input.tip);
  let res = send_remote_signal(
    ExternIO::encode(pulse).unwrap(),
    filtered,
  );
  if let Err(e) = res {
    error!("send_remote_signal() failed during cast_tip(): {:?}", e);
    return zome_error!("send_remote_signal() failed during cast_tip()");
  }
  trace!("calling remote recv_remote_signal() DONE");
  Ok(())
}
