use std::collections::BTreeSet;
use hdk::capability::create_cap_grant;
use hdk::info::zome_info;
use hdk::map_extern::ExternResult;
use hdk::prelude::{ActionHash, CapAccess, CapGrantEntry, FunctionName, GrantedFunctions};

///
pub fn create_signal_cap_grant() -> ExternResult<ActionHash> {
    let mut fns = BTreeSet::new();
    fns.insert((zome_info()?.name, FunctionName("recv_remote_signal".into())));
    let cap_grant_entry: CapGrantEntry = CapGrantEntry::new(
        String::from("recv_remote_signal"),
        CapAccess::Unrestricted, // Any external agent can call this zome function
        GrantedFunctions::Listed(fns),
    );
    return create_cap_grant(cap_grant_entry);
}