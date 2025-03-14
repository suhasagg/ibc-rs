# Changelog

## Unreleased

> Nothing yet.

## v0.3.0
*May 7h, 2021*

Special thanks to Jongwhan Lee (@leejw51crypto) for his contributions ([#878]).

This release mostly focuses on improving the UX and the experimental multi-paths relayer (`start-multi` command),
which has been made more resilient against nodes going down, and is now able to clear pending packets
and periodically refresh IBC clients. The relayer now also supports [ICS 027 (Interchain Accounts)][ics27].

[ics27]: https://github.com/cosmos/ibc/blob/master/spec/app/ics-027-interchain-accounts/README.md

### FEATURES

- [ibc-relayer]
  - Support for ICS27 ([#794])

- [ibc-relayer-cli]
  - Added packet clearing and client refresh capabilities for the `start-multi` command ([#784], [#786])

### IMPROVEMENTS

- [ibc]
  - Reinstated `ics23` dependency ([#854])
  - Use proper Timestamp type to track time ([#758])

- [ibc-relayer]
  - Change the default for client creation to allow governance recovery in case of expiration or misbehaviour ([#785])
  - Use a single supervisor in `start-multi` to subscribe to all configured chains ([#862])
  - The `start-multi` command is now more resilient to a node not being up or going down, and will attempt to reconnect ([#871])

### BUG FIXES

- [ibc]
  - Fix parsing in `chain_version` when chain identifier has multiple dashes ([#878])

- [ibc-relayer]
  - Fix pagination in gRPC query for clients ([#811])
  - Fix relayer crash when hermes starts in the same time as packets are being sent ([#851])
  - Fix missing port information in `hermes query channels` ([#840])
  - Fix crash during initialization of event monitor when node is down ([#863])
  - Spawn a single Tokio runtime for the whole supervisor instead of one per chain ([#909])

- [ibc-relayer-cli]
  - Fix for `ft-transfer` mismatching arguments ([#869])
  - Fix channel destination chain mismatch on unreceived-packets or unreceived-acks ([#873])

### BREAKING CHANGES

- [ibc-relayer]
  - `hermes -j query channels` command now returns `result` array with the format
    `[{"channel_id":"channel-0","port_id":"transfer"}, ...]` instead of `["channel-0", ...]` ([#840])


[#758]: https://github.com/informalsystems/ibc-rs/issues/758
[#784]: https://github.com/informalsystems/ibc-rs/issues/784
[#785]: https://github.com/informalsystems/ibc-rs/issues/785
[#786]: https://github.com/informalsystems/ibc-rs/issues/786
[#794]: https://github.com/informalsystems/ibc-rs/issues/794
[#811]: https://github.com/informalsystems/ibc-rs/issues/811
[#840]: https://github.com/informalsystems/ibc-rs/issues/840
[#851]: https://github.com/informalsystems/ibc-rs/issues/851
[#854]: https://github.com/informalsystems/ibc-rs/issues/854
[#862]: https://github.com/informalsystems/ibc-rs/issues/862
[#863]: https://github.com/informalsystems/ibc-rs/issues/863
[#869]: https://github.com/informalsystems/ibc-rs/issues/869
[#871]: https://github.com/informalsystems/ibc-rs/issues/871
[#873]: https://github.com/informalsystems/ibc-rs/issues/873
[#878]: https://github.com/informalsystems/ibc-rs/issues/878
[#909]: https://github.com/informalsystems/ibc-rs/issues/909

## v0.2.0
*April 14th, 2021*

This release includes initial support for relaying over multiple paths from a single `hermes` instance.
Adds support for relayer restart, where pending packets are cleared.
Includes support for ordered channels, packet delay, misbehaviour detection and evidence submission, client upgrade after counterparty chain upgrades.

This release brings improvements to the relayer UX by providing new and updated commands for keys, client, connection and channel management.
In addition, it simplifies the configuration of and integration with the light client.

This release also finalizes the initial implementation of all the ICS 004 handlers.

### FEATURES

- Update to `tendermint-rs` v0.19.0 ([#798])

- [ibc]
  - Added handler(s) for sending packets ([#695]), recv. and ack. packets ([#736]), and timeouts ([#362])

- [ibc-relayer]
  - Support for relayer restart ([#561])
  - Add support for ordered channels ([#599])
  - Misbehaviour detection and evidence submission ([#632])
  - Use a stateless light client without a runtime ([#673])

- [ibc-relayer-cli]
  - Added `create connection` and `create channel` CLIs ([#630], [#715])
  - Proposed ADR 006 to describe Hermes v0.2.0 use-cases ([#637])
  - Added `client-upgrade` CLI ([#357])
  - Added delay feature for packet relaying ([#640])
  - Update gaia to version 4.2.0 for e2e tests on CI ([#809])
  - Add `start-multi` command to relay on all paths defined in the configuration ([#748])
  - Add option to specify which events to listen for in `listen` command ([#550])
  - Add option to customise receiver address for `ft-transfer` command ([#806])
  - Add `keys restore` command to import a signing key from its mnemonic ([#813])

### IMPROVEMENTS

- [ibc]
  - Follow Rust guidelines naming conventions ([#689])
  - Per client structure modules ([#740])
  - MBT: use modelator crate ([#761])

- [ibc-relayer]
  - Consistent identifier handling across ICS 02, 03 and 04 ([#622])

- [ibc-relayer-cli]
  - Clarified success path for updating a client that is already up-to-date ([#734])
  - Added `create` and `update` wrappers for client raw commands ([#772])
  - Output by default is human-readable, and JSON is optional ([#805])

### BUG FIXES

- [ibc]
  - Fix overflow bug in ICS03 client consensus height verification method ([#685])
  - Allow a conn open ack to succeed in the happy case ([#699])

- [ibc-relayer]
  - Replaced `rust-crypto` & `bitcoin-wallet` deprecated dependencies ([#352])
  - Fix for hard-coded account number ([#752])
  - Fix for chains that don't have `cosmos` account prefix ([#416])
  - Fix for building the `trusted_validator_set` for the header used in client updates ([#770])
  - Don't send `MsgAcknowledgment` if channel is closed ([#675])
  - Fix a bug where the keys addresses had their account prefix overriden by the prefix in the configuration ([#751])

- [ibc-relayer-cli]
  - Hermes guide: improved installation guideline ([#672])
  - Make fee denom and amount configurable ([#754])

- [ibc-proto]
  - Fix for proto files re-compilation bug ([#801])

### BREAKING CHANGES

- [ibc]
  - `MsgConnectionOpenAck.counterparty_connection_id` is now a `ConnectionId` instead of an `Option<ConnectionId>`([#700])

- [ibc-relayer]
  - Remove the light client configuration from the global configuration ([#793])

- [ibc-relayer-cli]
    - Remove the light add and light rm commands ([#793])


[#352]: https://github.com/informalsystems/ibc-rs/issues/352
[#362]: https://github.com/informalsystems/ibc-rs/issues/362
[#357]: https://github.com/informalsystems/ibc-rs/issues/357
[#416]: https://github.com/informalsystems/ibc-rs/issues/416
[#561]: https://github.com/informalsystems/ibc-rs/issues/561
[#550]: https://github.com/informalsystems/ibc-rs/issues/550
[#599]: https://github.com/informalsystems/ibc-rs/issues/599
[#630]: https://github.com/informalsystems/ibc-rs/issues/630
[#632]: https://github.com/informalsystems/ibc-rs/issues/632
[#640]: https://github.com/informalsystems/ibc-rs/issues/640
[#672]: https://github.com/informalsystems/ibc-rs/issues/672
[#673]: https://github.com/informalsystems/ibc-rs/issues/673
[#675]: https://github.com/informalsystems/ibc-rs/issues/675
[#685]: https://github.com/informalsystems/ibc-rs/issues/685
[#689]: https://github.com/informalsystems/ibc-rs/issues/689
[#695]: https://github.com/informalsystems/ibc-rs/issues/695
[#699]: https://github.com/informalsystems/ibc-rs/issues/699
[#700]: https://github.com/informalsystems/ibc-rs/pull/700
[#715]: https://github.com/informalsystems/ibc-rs/issues/715
[#734]: https://github.com/informalsystems/ibc-rs/issues/734
[#736]: https://github.com/informalsystems/ibc-rs/issues/736
[#740]: https://github.com/informalsystems/ibc-rs/issues/740
[#748]: https://github.com/informalsystems/ibc-rs/issues/748
[#751]: https://github.com/informalsystems/ibc-rs/issues/751
[#752]: https://github.com/informalsystems/ibc-rs/issues/752
[#754]: https://github.com/informalsystems/ibc-rs/issues/754
[#761]: https://github.com/informalsystems/ibc-rs/issues/761
[#772]: https://github.com/informalsystems/ibc-rs/issues/772
[#770]: https://github.com/informalsystems/ibc-rs/issues/770
[#793]: https://github.com/informalsystems/ibc-rs/pull/793
[#798]: https://github.com/informalsystems/ibc-rs/issues/798
[#801]: https://github.com/informalsystems/ibc-rs/issues/801
[#805]: https://github.com/informalsystems/ibc-rs/issues/805
[#806]: https://github.com/informalsystems/ibc-rs/issues/806
[#809]: https://github.com/informalsystems/ibc-rs/issues/809


## v0.1.1
*February 17, 2021*

This release brings a quick fix for a problem with a dependency of crate
`ibc-relayer`, which causes build & installation issues. Many thanks to
@Fraccaman for bringing this problem to our attention! ([#672])


Additionally, this release also introduces initial implementation for most of
ICS 004 handlers, and several bug fixes and improvements, e.g., refactored
some CLI code, refactored the Height type in the IBC Events, and a bug fix
involving packet acks in a 3-chain setup. More details below.

### FEATURES
- [ibc-relayer]
  - Listen to channel close initialization event and perform the close handshake ([#560])
  - Updated to tendermint-rs `v0.18.1` ([#682], [#671])

### IMPROVEMENTS

- [ibc]
  - Change event height to ICS height ([#549])

- [ibc-relayer-cli]
  - Cleanup CLI code ([#572])

### BUG FIXES

- [ibc]
  - Fix panic in conn open try when no connection id is provided ([#626])
  - Disable MBT tests if the "mocks" feature is not enabled ([#643])

- [ibc-relayer]
  - Quick fix for `funty` breaking change bug ([#665])

- [ibc-relayer-cli]
  - Fix wrong acks sent with `tx raw packet-ack` in a 3-chain setup ([#614])

### BREAKING CHANGES

- [ibc]
  - Implementation of the `ChanOpenAck`, `ChanOpenConfirm`, `ChanCloseInit`, and `ChanCloseConfirm` handlers ([#316])
  - Remove dependency on `tendermint-rpc` ([#624])

- [ibc-relayer-cli]
  - Remove the `proof` option from CLI ([#572])

[#316]: https://github.com/informalsystems/ibc-rs/issues/316
[#549]: https://github.com/informalsystems/ibc-rs/issues/549
[#560]: https://github.com/informalsystems/ibc-rs/issues/560
[#572]: https://github.com/informalsystems/ibc-rs/issues/572
[#614]: https://github.com/informalsystems/ibc-rs/issues/614
[#622]: https://github.com/informalsystems/ibc-rs/issues/622
[#624]: https://github.com/informalsystems/ibc-rs/issues/624
[#626]: https://github.com/informalsystems/ibc-rs/issues/626
[#637]: https://github.com/informalsystems/ibc-rs/issues/637
[#643]: https://github.com/informalsystems/ibc-rs/issues/643
[#665]: https://github.com/informalsystems/ibc-rs/issues/665
[#671]: https://github.com/informalsystems/ibc-rs/pull/671
[#682]: https://github.com/informalsystems/ibc-rs/issues/682

[ibc]: https://github.com/informalsystems/ibc-rs/tree/master/modules
[ibc-relayer-cli]: https://github.com/informalsystems/ibc-rs/tree/master/relayer-cli

## v0.1.0
*February 4, 2021*

🎉 This release brings the first publication of `ibc-relayer` and
`ibc-relayer-cli` to [crates.io](https://crates.io).

Noteworthy changes in this release include:

- The binary in the `ibc-relayer-cli` crate was given the name Hermes.
- We published a comprehensive guide for Hermes at [hermes.informal.systems](https://hermes.informal.systems).
- Major improvements to user experience, in particular at CLI level: JSON output,
  configurable log output level, dedicated channel handshake command, as well as
  overall improvements to error display and output.

### FEATURES

- Continous Integration (CI) end-to-end (e2e) testing with gaia v4 ([#32], [#582], [#602])
- Add support for streamlining releases ([#507])

- [ibc-relayer-cli]
  - Implement command to query the channels associated with a connection ([#505])
  - JSON output for queries and txs ([#500])
  - Added 'required' annotation for CLIs queries & txs; better error display ([#555])
  - Implement commands for channel close init and confirm ([#538])
  - Implement command to perform the handshake for a new channel ([#557])
  - Query all clients command ([#552])
  - Query all connections command ([#553])
  - Query all channels command ([#568])
  - Added a relayer binary guide ([#542])
  - Split the dev-env script in `setup_chains` and `init_clients` ([#577])

- [ibc-relayer]
  - Added retry mechanism, restructured relayer ([#519])
  - Relay `MsgTimeoutOnClose` if counterparty channel state is `State::Closed`

- [ibc]
  - Add `MsgTimeoutOnClose` message type ([#563])
  - Implement `MsgChannelOpenTry` message handler ([#543])

### IMPROVEMENTS

- Update to `tendermint-rs` v0.18.0 ([#517], [#583])
- Update to `tokio` 1.0, `prost` 0.7 and `tonic` 0.4 ([#527])

- [ibc-relayer-cli]
  - Replace `ChannelConfig` in `Channel::new` ([#511])
  - Add `packet-send` CLI ([#470])
  - UX improvements for relayer txs ([#536], [#540], [#554])
  - Allow running standalone commands concurrently to the main relayer loop ([#501])
  - Remove the simd-based integration tests ([#593])

- [ibc-relayer]
  - Performance improvements ([#514], [#537])
  - Fix for mismatching `bitcoin` dep ([#525])

- [ibc]
  - Clean the `validate_basic` method ([#94])
  - `MsgConnectionOpenAck` testing improvements ([#306])

### BUG FIXES:
- [ibc-relayer-cli]
  - Help and usage commands show 'hermes' for executable name ([#590])

- [ibc]
  - Fix for storing `ClientType` upon 'create-client' ([#513])

### BREAKING CHANGES:

- [ibc]
  - The `ibc::handler::Event` is removed and handlers now produce `ibc::events::IBCEvent`s ([#535])

[#32]: https://github.com/informalsystems/ibc-rs/issues/32
[#94]: https://github.com/informalsystems/ibc-rs/issues/94
[#306]: https://github.com/informalsystems/ibc-rs/issues/306
[#470]: https://github.com/informalsystems/ibc-rs/issues/470
[#500]: https://github.com/informalsystems/ibc-rs/issues/500
[#501]: https://github.com/informalsystems/ibc-rs/issues/501
[#505]: https://github.com/informalsystems/ibc-rs/issues/505
[#507]: https://github.com/informalsystems/ibc-rs/issues/507
[#511]: https://github.com/informalsystems/ibc-rs/pull/511
[#513]: https://github.com/informalsystems/ibc-rs/issues/513
[#514]: https://github.com/informalsystems/ibc-rs/issues/514
[#517]: https://github.com/informalsystems/ibc-rs/issues/517
[#519]: https://github.com/informalsystems/ibc-rs/issues/519
[#525]: https://github.com/informalsystems/ibc-rs/issues/525
[#527]: https://github.com/informalsystems/ibc-rs/issues/527
[#535]: https://github.com/informalsystems/ibc-rs/issues/535
[#536]: https://github.com/informalsystems/ibc-rs/issues/536
[#537]: https://github.com/informalsystems/ibc-rs/issues/537
[#538]: https://github.com/informalsystems/ibc-rs/issues/538
[#540]: https://github.com/informalsystems/ibc-rs/issues/540
[#542]: https://github.com/informalsystems/ibc-rs/issues/542
[#543]: https://github.com/informalsystems/ibc-rs/issues/543
[#552]: https://github.com/informalsystems/ibc-rs/issues/553
[#553]: https://github.com/informalsystems/ibc-rs/issues/553
[#554]: https://github.com/informalsystems/ibc-rs/issues/554
[#555]: https://github.com/informalsystems/ibc-rs/issues/555
[#557]: https://github.com/informalsystems/ibc-rs/issues/557
[#563]: https://github.com/informalsystems/ibc-rs/issues/563
[#568]: https://github.com/informalsystems/ibc-rs/issues/568
[#577]: https://github.com/informalsystems/ibc-rs/issues/577
[#582]: https://github.com/informalsystems/ibc-rs/issues/582
[#583]: https://github.com/informalsystems/ibc-rs/issues/583
[#590]: https://github.com/informalsystems/ibc-rs/issues/590
[#593]: https://github.com/informalsystems/ibc-rs/issues/593
[#602]: https://github.com/informalsystems/ibc-rs/issues/602

## v0.0.6
*December 23, 2020*

This release focuses on upgrading the relayer and ibc modules to the latest interfaces from the ecosystem:
tendermint-rs `v0.17`, which brings the protobuf changes from tendermint `v0.34.0`, plus alignment with
the latest cosmos proto versions from `v0.40.0-rc5` (sometimes called 'stargate-5').

### FEATURES
- Update to tendermint-rs version `0.17` ([#451])
- Update to cosmos-sdk IBC proto version `v0.40.0-rc5` ([#451])

- [ibc-relayer]

- [ibc-relayer-cli]
  - Packet CLIs for recv_packet ([#443])
  - Packet CLIs for acknowledging packets ([#468])

### IMPROVEMENTS
- [ibc-relayer]
  - Mock chain (implementing IBC handlers) and integration against CLI ([#158])
  - Relayer tests for client update (ping pong) against MockChain ([#381])
  - Relayer refactor to improve testing and add semantic dependencies ([#447])

[#158]: https://github.com/informalsystems/ibc-rs/issues/158
[#379]: https://github.com/informalsystems/ibc-rs/issues/379
[#381]: https://github.com/informalsystems/ibc-rs/issues/381
[#443]: https://github.com/informalsystems/ibc-rs/issues/443
[#447]: https://github.com/informalsystems/ibc-rs/issues/447
[#451]: https://github.com/informalsystems/ibc-rs/issues/451
[#468]: https://github.com/informalsystems/ibc-rs/issues/468


## v0.0.5
*December 2, 2020*

This release focuses on implementing relayer and relayer-cli functionality towards a full v0 implementation.
We now have the full-stack implementation for supporting client creation & updates, as well as connection- and channel handshakes.
We also consolidated our TLA+ specs into an "IBC Core TLA+ specification," and added ICS 020 spec.

Special thanks to external contributors for this release: @CharlyCst ([#347], [#419]).

- [ibc-relayer-cli]
  - Add `--all` option to `light rm` command to remove all peers for a given chain ([#431])

[#431]: https://github.com/informalsystems/ibc-rs/issues/431

### FEATURES

- Update to tendermint-rs version `0.17-RC3` ([#403])
- [changelog] Added "unreleased" section in `CHANGELOG.MD` to help streamline releases ([#274])
- [ibc]
    - Implement flexible connection id selection ([#332])
    - ICS 4 Domain Types for channel handshakes and packets ([#315], [#95])
    - Introduce LightBlock support for MockContext ([#389])
- [ibc-relayer]
    - Retrieve account sequence information from a chain using a GRPC client (#337)
    - Implementation of chain runtime for v0 ([#330])
    - Integrate relayer spike into ibc-relayer crate ([#335])
    - Implement `query_header_at_height` via plain RPC queries (no light client verification) ([#336])
    - Implement the relayer logic for connection handshake messages ([#358], [#359], [#360])
    - Implement the relayer logic for channel handshake messages ([#371], [#372], [#373], [#374])
- [ibc-relayer-cli]
    - Merge light clients config in relayer config and add commands to add/remove light clients ([#348])
    - CLI for client update message ([#277])
    - Implement the relayer CLI for connection handshake messages ([#358], [#359], [#360])
    - Implement the relayer CLI for channel handshake messages ([#371], [#372], [#373], [#374])
    - Added basic client, connection, and channel lifecyle in relayer v0 ([#376], [#377], [#378])
    - Implement commands to add and list keys for a chain ([#363])
    - Allow overriding of peer_id, height and hash in light add command ([#428])
- [proto-compiler]
    - Refactor and allow specifying a commit at which the Cosmos SDK should be checked out ([#366])
    - Add a `--tag` option to the `clone-sdk` command to check out a tag instead of a commit ([#369])
    - Fix `--out` command line parameter (instead of `--path`) ([#419])
- [ibc/relayer-spec]
    - ICS 020 spec in TLA+ ([#386])
    - Prepare IBC Core TLA+ specs ([#404])

### IMPROVEMENTS

- [ibc-relayer]
    - Pin chain runtime against Tokio 0.2 by downgrading for 0.3 to avoid dependency hell ([#415], follow up to [#402])
- [ibc-relayer-cli]
    - Split tasks spawned by CLI commands into their own modules ([#331])
    - V0 command implementation ([#346])
- [ibc]
    - Split `msgs.rs` of ICS002 in separate modules ([#367])
    - Fixed inconsistent versioning for ICS003 and ICS004 ([#97])
    - Fixed `get_sign_bytes` method for messages ([#98])
    - Homogenize ConnectionReader trait so that all functions return owned objects ([#347])
    - Align with tendermint-rs in the domain type definition of `block::Id` ([#338])


[#95]: https://github.com/informalsystems/ibc-rs/issues/95
[#97]: https://github.com/informalsystems/ibc-rs/issues/97
[#98]: https://github.com/informalsystems/ibc-rs/issues/98
[#274]: https://github.com/informalsystems/ibc-rs/issues/274
[#277]: https://github.com/informalsystems/ibc-rs/issues/277
[#315]: https://github.com/informalsystems/ibc-rs/issues/315
[#330]: https://github.com/informalsystems/ibc-rs/issues/330
[#332]: https://github.com/informalsystems/ibc-rs/issues/332
[#335]: https://github.com/informalsystems/ibc-rs/pull/335
[#336]: https://github.com/informalsystems/ibc-rs/issues/336
[#337]: https://github.com/informalsystems/ibc-rs/issues/337
[#338]: https://github.com/informalsystems/ibc-rs/issues/338
[#346]: https://github.com/informalsystems/ibc-rs/issues/346
[#347]: https://github.com/informalsystems/ibc-rs/issues/347
[#348]: https://github.com/informalsystems/ibc-rs/pull/348
[#358]: https://github.com/informalsystems/ibc-rs/issues/358
[#359]: https://github.com/informalsystems/ibc-rs/issues/359
[#360]: https://github.com/informalsystems/ibc-rs/issues/360
[#363]: https://github.com/informalsystems/ibc-rs/issues/363
[#366]: https://github.com/informalsystems/ibc-rs/issues/366
[#367]: https://github.com/informalsystems/ibc-rs/issues/367
[#368]: https://github.com/informalsystems/ibc-rs/issues/368
[#369]: https://github.com/informalsystems/ibc-rs/pull/369
[#371]: https://github.com/informalsystems/ibc-rs/issues/371
[#372]: https://github.com/informalsystems/ibc-rs/issues/372
[#373]: https://github.com/informalsystems/ibc-rs/issues/373
[#374]: https://github.com/informalsystems/ibc-rs/issues/374
[#376]: https://github.com/informalsystems/ibc-rs/issues/376
[#377]: https://github.com/informalsystems/ibc-rs/issues/377
[#378]: https://github.com/informalsystems/ibc-rs/issues/378
[#386]: https://github.com/informalsystems/ibc-rs/issues/386
[#389]: https://github.com/informalsystems/ibc-rs/issues/389
[#402]: https://github.com/informalsystems/ibc-rs/issues/402
[#403]: https://github.com/informalsystems/ibc-rs/issues/403
[#404]: https://github.com/informalsystems/ibc-rs/issues/404
[#419]: https://github.com/informalsystems/ibc-rs/issues/419
[#415]: https://github.com/informalsystems/ibc-rs/issues/415
[#428]: https://github.com/informalsystems/ibc-rs/issues/428
[changelog]: https://github.com/informalsystems/ibc-rs/tree/master/CHANGELOG.md
[proto-compiler]: https://github.com/informalsystems/ibc-rs/tree/master/proto-compiler

## v0.0.4
*October 19, 2020*

This release focuses on alignment with the Cosmos ecosystem: adaptations to Tendermint-rs 0.16 and subsequently to 0.17 (`0.17.0-rc1`), and numerous protobuf updates following latest stargate releases.

Additional highlights:
- Adding DomainTypes and (de)serialization capability to ICS02 and ICS03 messages and structures.
- Improvements of the IBC message processor framework (handlers, contexts and mocks).
- Added initial implementations for the ICS26 (routing module) and ICS18 (basic relayer algorithms module) for use in testing.
- Also added support for packet handling in the relayer algorithm specifications.

### BREAKING CHANGES:
- [ibc-relayer] & [ibc] Alignment with ecosystem updates:
    - Compatibility with the latest protobuf (Gaia stargate-3 and stargate-4) ([#191], [#272], [#273], [#278])
    - Adaptations to tendermint 0.17 ([#286], [#293], [#300], [#302], [#308])
- [ibc-relayer] UX improvement: Remove proof option from client connections command ([#205])

### FEATURES:
- [ibc/ics03] ICS03 Ack and Confirm message processors ([#223])
- [ibc-relayer-cli]
    - Relayer CLIs for client messages ([#207])
    - Relayer CLIs for connection-open-init ([#206])
    - Queries for consensus state and client state ([#149], [#150])
- [ibc] Routing module minimal implementation for MVP ([#159], [#232])
- [ibc/relayer-spec] Relayer specification for packet handling ([#229], [#234], [#237])
- [ibc/relayer-spec] Basic packet handling in TLA+([#124])
- [ibc] Basic relayer functionality: a test with ClientUpdate ping-pong between two mocked chains ([#276])

### IMPROVEMENTS:
- [ibc] Implemented the `DomainType` trait for IBC proto structures ([#245], [#249]).
- [ibc] & [ibc-proto] Several improvements to message processors, among which ([#218]):
    - ICS03 connection handshake protocol initial implementation and tests ([#160])
    - Add capability to decode from protobuf Any* type into Tendermint and Mock client states
    - Cleanup Any* client wrappers related code
    - Migrate handlers to newer protobuf definitions ([#226])
    - Extend client context mock ([#221])
    - Context mock simplifications and cleanup ([#269], [#295], [#296], [#297])
- [ibc/ics03] Split `msgs.rs` in multiple files, implement `From` for all messages ([#253])
- [ibc-proto]
    - Move ibc-proto source code into ibc-rs ([#142]) and fixed code deduplication ([#282], [#284])
    - Consolidate proto-compiler logic [#241]
- [ibc/relayer-spec] Add support for APALACHE to the Relayer TLA+ spec ([#165])
- [ibc-relayer] Update to tendermint v.0.16 and integrate with the new light client implementation ([#90], [#243])

### BUG FIXES:
- [ibc] Removed "Uninitialized" state from connection ([#217])
- [ibc-relayer-cli] Fix for client query subcommands ([#231])
- [disclosure-log] & [spec/connection-handshake] Disclosed bugs in ICS3 version negotiation and proposed a fix ([#209], [#213])

[#90]: https://github.com/informalsystems/ibc-rs/issues/90
[#124]: https://github.com/informalsystems/ibc-rs/issues/124
[#142]: https://github.com/informalsystems/ibc-rs/issues/142
[#149]: https://github.com/informalsystems/ibc-rs/issues/149
[#150]: https://github.com/informalsystems/ibc-rs/issues/150
[#159]: https://github.com/informalsystems/ibc-rs/issues/159
[#160]: https://github.com/informalsystems/ibc-rs/issues/160
[#165]: https://github.com/informalsystems/ibc-rs/issues/165
[#191]: https://github.com/informalsystems/ibc-rs/issues/191
[#205]: https://github.com/informalsystems/ibc-rs/issues/205
[#206]: https://github.com/informalsystems/ibc-rs/issues/206
[#207]: https://github.com/informalsystems/ibc-rs/issues/207
[#209]: https://github.com/informalsystems/ibc-rs/issues/209
[#213]: https://github.com/informalsystems/ibc-rs/issues/213
[#217]: https://github.com/informalsystems/ibc-rs/issues/217
[#218]: https://github.com/informalsystems/ibc-rs/issues/218
[#221]: https://github.com/informalsystems/ibc-rs/issues/221
[#223]: https://github.com/informalsystems/ibc-rs/issues/223
[#226]: https://github.com/informalsystems/ibc-rs/issues/226
[#229]: https://github.com/informalsystems/ibc-rs/issues/229
[#231]: https://github.com/informalsystems/ibc-rs/issues/231
[#232]: https://github.com/informalsystems/ibc-rs/issues/232
[#234]: https://github.com/informalsystems/ibc-rs/issues/234
[#237]: https://github.com/informalsystems/ibc-rs/issues/237
[#241]: https://github.com/informalsystems/ibc-rs/issues/241
[#243]: https://github.com/informalsystems/ibc-rs/issues/243
[#245]: https://github.com/informalsystems/ibc-rs/issues/245
[#249]: https://github.com/informalsystems/ibc-rs/issues/249
[#253]: https://github.com/informalsystems/ibc-rs/issues/253
[#269]: https://github.com/informalsystems/ibc-rs/issues/269
[#272]: https://github.com/informalsystems/ibc-rs/issues/272
[#273]: https://github.com/informalsystems/ibc-rs/issues/273
[#276]: https://github.com/informalsystems/ibc-rs/issues/276
[#278]: https://github.com/informalsystems/ibc-rs/issues/278
[#282]: https://github.com/informalsystems/ibc-rs/issues/282
[#284]: https://github.com/informalsystems/ibc-rs/issues/284
[#286]: https://github.com/informalsystems/ibc-rs/issues/286
[#293]: https://github.com/informalsystems/ibc-rs/issues/293
[#295]: https://github.com/informalsystems/ibc-rs/issues/295
[#296]: https://github.com/informalsystems/ibc-rs/issues/296
[#297]: https://github.com/informalsystems/ibc-rs/issues/297
[#300]: https://github.com/informalsystems/ibc-rs/issues/300
[#302]: https://github.com/informalsystems/ibc-rs/issues/302
[#308]: https://github.com/informalsystems/ibc-rs/issues/308
[ibc-proto]: https://github.com/informalsystems/ibc-rs/tree/master/proto
[disclosure-log]: https://github.com/informalsystems/ibc-rs/blob/master/docs/disclosure-log.md
[spec/connection-handshake]: https://github.com/informalsystems/ibc-rs/tree/master/docs/spec/connection-handshake
[ibc-relayer]: https://github.com/informalsystems/ibc-rs/tree/master/relayer

## v0.0.3
*September 1, 2020*

This release focuses on the IBC message processor framework and initial
implementations in ICS02 and ICS07. It also introduces an initial specification for the relayer algorithm.

Other highlights:
- The ibc crate is published as [ibc](https://crates.io/crates/ibc) in crates.io
- ADR-001 and ADR-003 are complete. 🎉

### BREAKING CHANGES:
- [ibc] Renamed `modules` crate to `ibc` crate. Version number for the new crate is not reset. ([#198])
- [ibc/ics02] `ConnectionId`s are now decoded to `Vec<ConnectionId>` and validated instead of `Vec<String>` ([#185])
- [ibc/ics03] Removed `Connection` and `ConnectionCounterparty` traits ([#193])
- [ibc/ics04] Removed `Channel` and `ChannelCounterparty` traits ([#192])

### FEATURES:
- [ibc/ics02] partial implementation of message handler ([#119], [#194])
- [ibc/ics07] partial implementation of message handler ([#119], [#194])
- [architecture/ADR-003] Proposal for IBC handler (message processor) architecture ([#119], [#194])
- [ibc/relayer-spec] Detailed technical specification of the relayer algorithm with focus on client update ([#84])
- [architecture/ADR-001] Documentation for the repository structure ([#1])
- [architecture/FSM-1] Connection Handshake FSM English description ([#122])

### IMPROVEMENTS:
- [contributing] Updated CONTRIBUTING.md. Please read before opening PRs ([#195])
- [ibc-relayer-cli] Refactor ConnectionId decoding in `query client` ([#185])

### BUG FIXES:
- [ibc/ics24] Identifiers limit update according to ICS specs ([#168])

[ibc/relayer-spec]: https://github.com/informalsystems/ibc-rs/blob/master/docs/spec/relayer/Relayer.md
[#84]: https://github.com/informalsystems/ibc-rs/issues/84
[architecture/ADR-001]: https://github.com/informalsystems/ibc-rs/blob/master/docs/architecture/adr-001-repo.md
[#1]: https://github.com/informalsystems/ibc-rs/issues/1
[contributing]: https://github.com/informalsystems/ibc-rs/blob/master/CONTRIBUTING.md
[#195]: https://github.com/informalsystems/ibc-rs/pull/195
[ibc]: https://github.com/informalsystems/ibc-rs/tree/master/modules
[#198]: https://github.com/informalsystems/ibc-rs/issues/198
[ibc/ics02]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/ics02_client
[#185]: https://github.com/informalsystems/ibc-rs/issues/185
[ibc/ics03]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/ics03_connection
[#193]: https://github.com/informalsystems/ibc-rs/issues/193
[ibc/ics04]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/ics04_channel
[#192]: https://github.com/informalsystems/ibc-rs/issues/192
[ibc-relayer-cli]: https://github.com/informalsystems/ibc-rs/tree/master/relayer-cli
[architecture/FSM-1]: https://github.com/informalsystems/ibc-rs/blob/master/docs/architecture/fsm-async-connection.md
[#122]: https://github.com/informalsystems/ibc-rs/issues/122
[architecture/ADR-003]: https://github.com/informalsystems/ibc-rs/blob/master/docs/architecture/adr-003-handler-implementation.md
[#119]: https://github.com/informalsystems/ibc-rs/issues/119
[#194]: https://github.com/informalsystems/ibc-rs/issues/194
[ibc/ics24]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/ics24_host
[#168]: https://github.com/informalsystems/ibc-rs/issues/168
[ibc/ics07]: https://github.com/informalsystems/ibc-rs/tree/master/modules/src/ics07_tendermint

## v0.0.2

*August 1, 2020*

This release is focused on updating the query system from amino to protobuf,
implementing a few queries from the CLI, and establishing an initial testing framework
that will support multiple chain types.

It does not target a stable release of Cosmos-SDK chains, but is tracking
the latest state of development towards the Cosmos-SDK Stargate release.

### BREAKING CHANGES:

- [ibc|ibc-relayer] Refactor queries, paths, and Chain trait to reduce code and use
  protobuf instead of Amino.
        [\#152](https://github.com/informalsystems/ibc-rs/pull/152),
        [\#174](https://github.com/informalsystems/ibc-rs/pull/174),
        [\#155](https://github.com/informalsystems/ibc-rs/pull/155)
- [repo] Moved relayer/cli to relayer-cli, relayer/relay to relayer. [\#183](https://github.com/informalsystems/ibc-rs/pull/183)

### FEATURES:

- [ibc-relayer] Query connections given client id. [\#169](https://github.com/informalsystems/ibc-rs/pull/169)
- [ibc-relayer] Query connection given connection id. [\#136](https://github.com/informalsystems/ibc-rs/pull/136)
- [ibc-relayer] Query channel given channel id and port [\#163](https://github.com/informalsystems/ibc-rs/pull/163)
- [spec] Channel closing datagrams in TLA+ [\#141](https://github.com/informalsystems/ibc-rs/pull/141)

### IMPROVEMENTS:

- [ci] Framework (scripts and Github Actions) for integration testing the relayer queries against
    the Cosmos-SDK's `simd` binary with prepopulated IBC state in the genesis
        [\#140](https://github.com/informalsystems/ibc-rs/pull/140),
        [\#184](https://github.com/informalsystems/ibc-rs/pull/184)
- [ibc-relayer|ibc] Implemented better Raw type handling. [\#156](https://github.com/informalsystems/ibc-rs/pull/156)
- [repo] Add rust-toolchain file. [\#154](https://github.com/informalsystems/ibc-rs/pull/154)

### BUG FIXES:

- [ibc] Fixed the identifiers limits according to updated ics spec. [\#189](https://github.com/informalsystems/ibc-rs/pull/189)
- [ibc/relayer] Remove some warnings triggered during compilation due to dependency specification. [\#132](https://github.com/informalsystems/ibc-rs/pull/132)
- [ibc] Fix nightly runs. [\#161](https://github.com/informalsystems/ibc-rs/pull/161)
- [repo] Fix for incomplete licence terms. [\#153](https://github.com/informalsystems/ibc-rs/pull/153)

## 0.0.1

*July 1st, 2020*

This is the initial prototype release of an IBC relayer and TLA+ specifications.
There are no compatibility guarantees until v0.1.0.

Includes:

- Configuration file definition and validation
- Client state, consensus state, connection, channel queries.
    - Note: deserialization is unimplemented as it has dependency on migration to protobuf for ABCI queries
- Per chain light clients threads are created and headers are periodically retrieved and verified.
- Per chain IBC event monitor threads are spawned and main event handler that receives them.
    - Note: the event handler just displays the events.
- IBC Modules partial implementation for datastructures, messages and queries.
- Some English and TLA+ specifications for Connection & Channel Handshake as well as naive relayer algorithm.
