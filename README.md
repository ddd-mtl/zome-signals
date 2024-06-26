# Zome Signals

Zome module for handling signals in a specific way.


# Design

This design is to be used in a way that information is transmitted from zome wasm to UI mainly via *Signals* and not return values of zome functions.
The main reason for this is to simplify how UI processes information from zome wasm.
Since zome wasm can send information during zome callbacks only via signals, then the only way to have consistency is for zome functions to also do this.

## Terminology

In Holochain, a **Signal** is some data (serialized `Vec<u8>`) sent from the zome wasm to the UI.

A *Signal* is composed of multiple **Pulses**.

A ***Tip*** is a *Signal* received from another peer via `recv_remote_signal()` (sent with `remote_signal()`).

A ***System Signal*** is a *Signal* informing about "internal" events.

An ***Entry Signal*** is a *Signal* informing about an Entry.

A ***Link Signal*** is a *Signal* informing about a Link.

A *Signal* is **emitted**, while a *Tip* is **cast**.



## Building

1. [Install rustup](https://rustup.rs/) and the `wasm32` target with: ``rustup target add wasm32-unknown-unknown``
1. Run ``cargo build --release --target wasm32-unknown-unknown``
