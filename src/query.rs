use hdk::prelude::*;
use crate::*;

///
pub fn query_all_typed<R: TryFrom<Entry>>(entry_type: EntryType) -> ExternResult<()> {
    let tuples = query_all_entry(entry_type)?;
    /// Form & Emit Signal
    let pulses = tuples.into_iter()
        .map(|(record, _entry)| {
            let entry_pulse = EntryPulse::try_from_new_record(record, false).unwrap();
            return ZomeSignalProtocol::Entry(entry_pulse);
        })
        .collect();
    emit_zome_signal(pulses)?;
    /// Done
    Ok(())
}


/// Return vec of typed entries of given entry type found in local source chain
pub fn query_all_entry(entry_type: EntryType) -> ExternResult<Vec<(Record, Entry)>> {
    /// Query type
    let query_args = ChainQueryFilter::default()
        .include_entries(true)
        .action_type(ActionType::Create)
        .action_type(ActionType::Update)
        .entry_type(entry_type);
    let records = query(query_args)?;
    /// Get entries for all results
    let mut entries = Vec::new();
    for record in records {
        let RecordEntry::Present(entry) = record.entry() else {
            return Err(wasm_error!("Record should hold entry data"));
        };
        entries.push((record.clone(), entry.clone()))
    }
    /// Done
    Ok(entries)
}
