use hdk::entry::{get, must_get_action, must_get_entry};
use hdk::map_extern::ExternResult;
use hdk::prelude::*;
use crate::*;
use std::fmt::Debug;

/// Emit Entry or Link signal on post_commit()
pub fn emit_post_commit<E: UnitEnum, L: LinkTypesHelper + Debug>(signedActionList: Vec<SignedActionHashed>) {
    /// Process each Action
    for sah in signedActionList {
        // debug!(" - {}", sah.action());
        let ah = sah.as_hash().to_owned();
        match sah.action() {
            ///
            Action::DeleteLink(delete_link) => {
                let Ok(Some(record)) = get(delete_link.link_add_address.clone(), GetOptions::local())
                    else { error!("Failed to get CreateLink action"); continue };
                let Action::CreateLink(create_link) = record.action()
                    else { error!("Record should be a CreateLink"); continue };
                let res = emit_link_delete_signal(delete_link, create_link, true);
                if let Err(e) = res {
                    error!("Emitting DeleteLink signal failed: {:?}", e);
                }
            },
            ///
            Action::CreateLink(create_link) => {
                let Ok(Some(link_type)) = L::from_type(create_link.zome_index, create_link.link_type)
                    else { error!("CreateLink should have a LinkType. Could be a Link from a different zome: {} ({})", create_link.link_type.0, create_link.zome_index); continue };
                //let _ = emit_system_signal(SystemSignalProtocol::PostCommitStart { entry_type: link_type.clone() });
                debug!("CreateLink: {:?} ({}, {:?})", link_type, create_link.zome_index, create_link.link_type);
                let res = emit_link_create_signal(ah, create_link, true);
                if let Err(e) = res {
                    error!("Emitting CreateLink signal failed: {:?}", e);
                }
                //let _ = emit_system_signal(SystemSignalProtocol::PostCommitEnd { entry_type: link_type, succeeded: result.is_ok() });

            },
            /// NewEntryAction
            Action::Update(_) |
            Action::Create(_) => {
                let EntryType::App(app_entry_def) = sah.action().entry_type().unwrap()
                    else { continue };
                let type_variant = get_variant_from_index::<E>(app_entry_def.entry_index).unwrap();
                /// Emit System Signal
                let variant_name = format!("{:?}", type_variant);
                let _ = emit_system_signal(SystemSignalProtocol::PostCommitNewStart { app_entry_type: variant_name.clone() });
                /// Emit Entry Signal
                let result = emit_new_entry(sah.clone());
                /// Emit System Signal
                let _ = emit_system_signal(SystemSignalProtocol::PostCommitNewEnd { app_entry_type: variant_name, succeeded: result.is_ok() });
                ///
                if let Err(e) = result {
                    error!("<< post_commit() failed: {:?}", e);
                } else {
                    debug!("<< post_commit() SUCCEEDED");
                }
            },
            /// DeleteAction
            Action::Delete(delete) => {
                let Ok(new_sah) = must_get_action(delete.deletes_address.clone())
                    else { error!("Deleted action not found."); continue; };
                let Ok(he) = must_get_entry(delete.deletes_entry_address.clone())
                    else { error!("Deleted entry not found."); continue; };
                let Some(EntryType::App(app_entry_def)) = new_sah.action().entry_type()
                    else { error!("Deleted action should have entry_type."); continue; };
                let type_variant = get_variant_from_index::<E>(app_entry_def.entry_index).unwrap();
                /// Emit System Signal
                let variant_name = format!("{:?}", type_variant);
                let _ = emit_system_signal(SystemSignalProtocol::PostCommitDeleteStart { app_entry_type: variant_name.clone() });
                /// Emit Entry Signal
                let result = emit_delete_entry_signal(new_sah.hashed, he.content, true);
                /// Emit System Signal
                let _ = emit_system_signal(SystemSignalProtocol::PostCommitDeleteEnd { app_entry_type: variant_name, succeeded: result.is_ok() });
                ///
                if let Err(e) = result {
                    error!("<< post_commit() failed: {:?}", e);
                } else {
                    debug!("<< post_commit() SUCCEEDED");
                }
            },
            ///
            _ => (),
        }
    }
}


///
fn emit_new_entry(sah: SignedActionHashed) -> ExternResult<()> {
    let Some(eh) = sah.action().entry_hash() else {
        return Err(wasm_error!("Action has no Entry"));
    };
    let entry = must_get_entry(eh.to_owned())?.content;
    let record = Record::new(sah, Some(entry));
    /// Emit Signal
    emit_new_entry_signal(record, true)?;
    Ok(())
}


///
fn get_variant_from_index<T: UnitEnum>(entry_index: EntryDefIndex) -> ExternResult<T::Unit> {
    let mut i = 0;
    for variant in T::unit_iter() {
        if i == entry_index.0 {
            return Ok(variant);
        }
        i += 1;
    }
    return Err(wasm_error!(format!("Unknown EntryDefIndex: {}", entry_index.0)));
}