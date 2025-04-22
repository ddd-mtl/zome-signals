use hdk::prelude::*;
//use zome_utils::*;
use crate::*;


/// Don't forget to call create_cap_grant() at init() for this zome function
#[hdk_extern]
fn recv_remote_signal(pulse: ExternIO) -> ExternResult<()> {
  //std::panic::set_hook(Box::new(zome_panic_hook));
  let mut pulse: ZomeSignalProtocol = pulse.decode()
    .map_err(|e| wasm_error!(SerializedBytesError::Deserialize(e.to_string())))?;
  // Received data is not 'safe'
  pulse.clear_validation();
  //
  let signal = ZomeSignal {
    from: call_info()?.provenance,
    pulses: vec![pulse],
  };
  //debug!("Received signal from {}:{:?}", caller,  pulse);
  Ok(emit_signal(&signal)?)
}


///
pub fn create_signal_cap_grant() -> ExternResult<ActionHash> {
  let mut fns = BTreeSet::new();
  fns.insert((zome_info()?.name, FunctionName("recv_remote_signal".into())));
  let cap_grant_entry: CapGrantEntry = CapGrantEntry::new(
    String::from("recv_remote_signal"), // A string by which to later query for saved grants.
    CapAccess::Unrestricted, // Unrestricted access means any external agent can call the extern
    GrantedFunctions::Listed(fns),
  );
  return create_cap_grant(cap_grant_entry);
}