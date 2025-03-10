use prost_types::Any;
use tendermint_proto::Protobuf;

use crate::application::ics20_fungible_token_transfer::msgs::transfer;
use crate::application::ics20_fungible_token_transfer::relay_application_logic::send_transfer::send_transfer as ics20_msg_dispatcher;
use crate::ics02_client::handler::dispatch as ics2_msg_dispatcher;
use crate::ics02_client::msgs::{create_client, update_client, ClientMsg};
use crate::ics03_connection::handler::dispatch as ics3_msg_dispatcher;
use crate::ics03_connection::msgs::{
    conn_open_ack, conn_open_confirm, conn_open_init, conn_open_try, ConnectionMsg,
};
use crate::ics04_channel::handler::channel_dispatch as ics4_msg_dispatcher;
use crate::ics04_channel::handler::packet_dispatch as ics04_packet_msg_dispatcher;
use crate::{events::IbcEvent, handler::HandlerOutput};

use crate::ics04_channel::msgs::{
    acknowledgement, chan_close_confirm, chan_close_init, chan_open_ack, chan_open_confirm,
    chan_open_init, chan_open_try, recv_packet, timeout, timeout_on_close, ChannelMsg, PacketMsg,
};
use crate::ics26_routing::context::Ics26Context;
use crate::ics26_routing::error::{Error, Kind};
use crate::ics26_routing::msgs::Ics26Envelope::{
    self, Ics20Msg, Ics2Msg, Ics3Msg, Ics4ChannelMsg, Ics4PacketMsg,
};

/// Mimics the DeliverTx ABCI interface, but a slightly lower level. No need for authentication
/// info or signature checks here.
/// https://github.com/cosmos/cosmos-sdk/tree/master/docs/basics
/// Returns a vector of all events that got generated as a byproduct of processing `messages`.
pub fn deliver<Ctx>(ctx: &mut Ctx, messages: Vec<Any>) -> Result<Vec<IbcEvent>, Error>
where
    Ctx: Ics26Context,
{
    // Create a clone, which will store each intermediary stage of applying txs.
    let mut ctx_interim = ctx.clone();

    // A buffer for all the events, to be used as return value.
    let mut res: Vec<IbcEvent> = vec![];

    for any_msg in messages {
        // Decode the proto message into a domain message, creating an ICS26 envelope.
        let envelope = match any_msg.type_url.as_str() {
            // ICS2 messages
            create_client::TYPE_URL => {
                // Pop out the message and then wrap it in the corresponding type.
                let domain_msg = create_client::MsgCreateAnyClient::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics2Msg(ClientMsg::CreateClient(domain_msg)))
            }
            update_client::TYPE_URL => {
                let domain_msg = update_client::MsgUpdateAnyClient::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics2Msg(ClientMsg::UpdateClient(domain_msg)))
            }

            // ICS03
            conn_open_init::TYPE_URL => {
                let domain_msg = conn_open_init::MsgConnectionOpenInit::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics3Msg(ConnectionMsg::ConnectionOpenInit(domain_msg)))
            }
            conn_open_try::TYPE_URL => {
                let domain_msg = conn_open_try::MsgConnectionOpenTry::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics3Msg(ConnectionMsg::ConnectionOpenTry(Box::new(
                    domain_msg,
                ))))
            }
            conn_open_ack::TYPE_URL => {
                let domain_msg = conn_open_ack::MsgConnectionOpenAck::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics3Msg(ConnectionMsg::ConnectionOpenAck(Box::new(
                    domain_msg,
                ))))
            }
            conn_open_confirm::TYPE_URL => {
                let domain_msg =
                    conn_open_confirm::MsgConnectionOpenConfirm::decode_vec(&any_msg.value)
                        .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics3Msg(ConnectionMsg::ConnectionOpenConfirm(domain_msg)))
            }

            // ICS04 channel messages
            chan_open_init::TYPE_URL => {
                let domain_msg = chan_open_init::MsgChannelOpenInit::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics4ChannelMsg(ChannelMsg::ChannelOpenInit(domain_msg)))
            }
            chan_open_try::TYPE_URL => {
                let domain_msg = chan_open_try::MsgChannelOpenTry::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics4ChannelMsg(ChannelMsg::ChannelOpenTry(domain_msg)))
            }
            chan_open_ack::TYPE_URL => {
                let domain_msg = chan_open_ack::MsgChannelOpenAck::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics4ChannelMsg(ChannelMsg::ChannelOpenAck(domain_msg)))
            }
            chan_open_confirm::TYPE_URL => {
                let domain_msg =
                    chan_open_confirm::MsgChannelOpenConfirm::decode_vec(&any_msg.value)
                        .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics4ChannelMsg(ChannelMsg::ChannelOpenConfirm(domain_msg)))
            }
            chan_close_init::TYPE_URL => {
                let domain_msg = chan_close_init::MsgChannelCloseInit::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics4ChannelMsg(ChannelMsg::ChannelCloseInit(domain_msg)))
            }
            chan_close_confirm::TYPE_URL => {
                let domain_msg =
                    chan_close_confirm::MsgChannelCloseConfirm::decode_vec(&any_msg.value)
                        .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics4ChannelMsg(ChannelMsg::ChannelCloseConfirm(domain_msg)))
            }
            // ICS20 - 04 - Send packet
            transfer::TYPE_URL => {
                let domain_msg = transfer::MsgTransfer::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics20Msg(domain_msg))
            }
            // ICS04 packet messages
            recv_packet::TYPE_URL => {
                let domain_msg = recv_packet::MsgRecvPacket::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics4PacketMsg(PacketMsg::RecvPacket(domain_msg)))
            }
            acknowledgement::TYPE_URL => {
                let domain_msg = acknowledgement::MsgAcknowledgement::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics4PacketMsg(PacketMsg::AckPacket(domain_msg)))
            }
            timeout::TYPE_URL => {
                let domain_msg = timeout::MsgTimeout::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics4PacketMsg(PacketMsg::ToPacket(domain_msg)))
            }
            timeout_on_close::TYPE_URL => {
                let domain_msg = timeout_on_close::MsgTimeoutOnClose::decode_vec(&any_msg.value)
                    .map_err(|e| Kind::MalformedMessageBytes.context(e))?;
                Ok(Ics4PacketMsg(PacketMsg::ToClosePacket(domain_msg)))
            }

            _ => Err(Kind::UnknownMessageTypeUrl(any_msg.type_url)),
        }?;

        // Process the envelope, and accumulate any events that were generated.
        let mut output = dispatch(&mut ctx_interim, envelope)?;
        // TODO: output.log and output.result are discarded
        res.append(&mut output.events);
    }

    // No error has surfaced, so we now apply the changes permanently to the original context.
    *ctx = ctx_interim;
    Ok(res)
}

/// Top-level ICS dispatch function. Routes incoming IBC messages to their corresponding module.
/// Returns a handler output with empty result of type `HandlerOutput<()>` which contains the log
/// and events produced after processing the input `msg`.
pub fn dispatch<Ctx>(ctx: &mut Ctx, msg: Ics26Envelope) -> Result<HandlerOutput<()>, Error>
where
    Ctx: Ics26Context,
{
    let output = match msg {
        Ics2Msg(msg) => {
            let handler_output =
                ics2_msg_dispatcher(ctx, msg).map_err(|e| Kind::HandlerRaisedError.context(e))?;

            // Apply the result to the context (host chain store).
            ctx.store_client_result(handler_output.result)
                .map_err(|e| Kind::KeeperRaisedError.context(e))?;

            HandlerOutput::builder()
                .with_log(handler_output.log)
                .with_events(handler_output.events)
                .with_result(())
        }

        Ics3Msg(msg) => {
            let handler_output =
                ics3_msg_dispatcher(ctx, msg).map_err(|e| Kind::HandlerRaisedError.context(e))?;

            // Apply any results to the host chain store.
            ctx.store_connection_result(handler_output.result)
                .map_err(|e| Kind::KeeperRaisedError.context(e))?;

            HandlerOutput::builder()
                .with_log(handler_output.log)
                .with_events(handler_output.events)
                .with_result(())
        }

        Ics4ChannelMsg(msg) => {
            let handler_output =
                ics4_msg_dispatcher(ctx, msg).map_err(|e| Kind::HandlerRaisedError.context(e))?;

            // Apply any results to the host chain store.
            ctx.store_channel_result(handler_output.result)
                .map_err(|e| Kind::KeeperRaisedError.context(e))?;

            HandlerOutput::builder()
                .with_log(handler_output.log)
                .with_events(handler_output.events)
                .with_result(())
        }

        Ics20Msg(msg) => {
            let handler_output =
                ics20_msg_dispatcher(ctx, msg).map_err(|e| Kind::HandlerRaisedError.context(e))?;

            // Apply any results to the host chain store.
            ctx.store_packet_result(handler_output.result)
                .map_err(|e| Kind::KeeperRaisedError.context(e))?;

            HandlerOutput::builder()
                .with_log(handler_output.log)
                .with_events(handler_output.events)
                .with_result(())
        }

        Ics4PacketMsg(msg) => {
            let handler_output = ics04_packet_msg_dispatcher(ctx, msg)
                .map_err(|e| Kind::HandlerRaisedError.context(e))?;

            // Apply any results to the host chain store.
            ctx.store_packet_result(handler_output.result)
                .map_err(|e| Kind::KeeperRaisedError.context(e))?;

            HandlerOutput::builder()
                .with_log(handler_output.log)
                .with_events(handler_output.events)
                .with_result(())
        }
    };

    Ok(output)
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::application::ics20_fungible_token_transfer::msgs::transfer::test_util::get_dummy_msg_transfer;
    use crate::events::IbcEvent;
    use crate::ics02_client::client_consensus::AnyConsensusState;
    use crate::ics02_client::client_state::AnyClientState;
    use crate::ics02_client::msgs::{
        create_client::MsgCreateAnyClient, update_client::MsgUpdateAnyClient, ClientMsg,
    };
    use crate::ics03_connection::msgs::{
        conn_open_ack::{test_util::get_dummy_raw_msg_conn_open_ack, MsgConnectionOpenAck},
        conn_open_init::{test_util::get_dummy_raw_msg_conn_open_init, MsgConnectionOpenInit},
        conn_open_try::{test_util::get_dummy_raw_msg_conn_open_try, MsgConnectionOpenTry},
        ConnectionMsg,
    };
    use crate::ics04_channel::msgs::{
        chan_close_confirm::{
            test_util::get_dummy_raw_msg_chan_close_confirm, MsgChannelCloseConfirm,
        },
        chan_close_init::{test_util::get_dummy_raw_msg_chan_close_init, MsgChannelCloseInit},
        chan_open_ack::{test_util::get_dummy_raw_msg_chan_open_ack, MsgChannelOpenAck},
        chan_open_init::{test_util::get_dummy_raw_msg_chan_open_init, MsgChannelOpenInit},
        chan_open_try::{test_util::get_dummy_raw_msg_chan_open_try, MsgChannelOpenTry},
        recv_packet::{test_util::get_dummy_raw_msg_recv_packet, MsgRecvPacket},
        timeout_on_close::{test_util::get_dummy_raw_msg_timeout_on_close, MsgTimeoutOnClose},
        ChannelMsg, PacketMsg,
    };

    use crate::ics24_host::identifier::ConnectionId;
    use crate::ics26_routing::handler::dispatch;
    use crate::ics26_routing::msgs::Ics26Envelope;
    use crate::mock::client_state::{MockClientState, MockConsensusState};
    use crate::mock::context::MockContext;
    use crate::mock::header::MockHeader;
    use crate::test_utils::get_dummy_account_id;
    use crate::timestamp::Timestamp;
    use crate::Height;

    #[test]
    /// These tests exercise two main paths: (1) the ability of the ICS26 routing module to dispatch
    /// messages to the correct module handler, and more importantly: (2) the ability of ICS handlers
    /// to work with the context and correctly store results (i.e., the `ClientKeeper`,
    /// `ConnectionKeeper`, and `ChannelKeeper` traits).
    fn routing_module_and_keepers() {
        // Test parameters
        struct Test {
            name: String,
            msg: Ics26Envelope,
            want_pass: bool,
        }
        let default_signer = get_dummy_account_id();
        let client_height = 5;
        let start_client_height = Height::new(0, client_height);
        let update_client_height = Height::new(0, 34);
        let update_client_height_after_send = Height::new(0, 35);

        let update_client_height_after_second_send = Height::new(0, 36);

        // We reuse this same context across all tests. Nothing in particular needs parametrizing.
        let mut ctx = MockContext::default();

        let create_client_msg = MsgCreateAnyClient::new(
            AnyClientState::from(MockClientState(MockHeader::new(start_client_height))),
            AnyConsensusState::from(MockConsensusState(MockHeader::new(start_client_height))),
            default_signer.clone(),
        )
        .unwrap();

        //
        // Connection handshake messages.
        //
        let msg_conn_init =
            MsgConnectionOpenInit::try_from(get_dummy_raw_msg_conn_open_init()).unwrap();

        let correct_msg_conn_try = MsgConnectionOpenTry::try_from(get_dummy_raw_msg_conn_open_try(
            client_height,
            client_height,
        ))
        .unwrap();

        // The handler will fail to process this msg because the client height is too advanced.
        let incorrect_msg_conn_try = MsgConnectionOpenTry::try_from(
            get_dummy_raw_msg_conn_open_try(client_height + 1, client_height + 1),
        )
        .unwrap();

        let msg_conn_ack = MsgConnectionOpenAck::try_from(get_dummy_raw_msg_conn_open_ack(
            client_height,
            client_height,
        ))
        .unwrap();

        //
        // Channel handshake messages.
        //
        let msg_chan_init =
            MsgChannelOpenInit::try_from(get_dummy_raw_msg_chan_open_init()).unwrap();

        // The handler will fail to process this b/c the associated connection does not exist
        let mut incorrect_msg_chan_init = msg_chan_init.clone();
        incorrect_msg_chan_init.channel.connection_hops = vec![ConnectionId::new(590)];

        let msg_chan_try =
            MsgChannelOpenTry::try_from(get_dummy_raw_msg_chan_open_try(client_height)).unwrap();

        let msg_chan_ack =
            MsgChannelOpenAck::try_from(get_dummy_raw_msg_chan_open_ack(client_height)).unwrap();

        let msg_chan_close_init =
            MsgChannelCloseInit::try_from(get_dummy_raw_msg_chan_close_init()).unwrap();

        let msg_chan_close_confirm =
            MsgChannelCloseConfirm::try_from(get_dummy_raw_msg_chan_close_confirm(client_height))
                .unwrap();

        let msg_transfer = get_dummy_msg_transfer(35);

        let msg_transfer_two = get_dummy_msg_transfer(36);

        let mut msg_to_on_close =
            MsgTimeoutOnClose::try_from(get_dummy_raw_msg_timeout_on_close(36, 5)).unwrap();
        msg_to_on_close.packet.sequence = 2.into();
        msg_to_on_close.packet.timeout_height = msg_transfer_two.timeout_height;
        msg_to_on_close.packet.timeout_timestamp =
            Timestamp::from_nanoseconds(msg_transfer_two.timeout_timestamp).unwrap();

        let msg_recv_packet = MsgRecvPacket::try_from(get_dummy_raw_msg_recv_packet(35)).unwrap();

        // First, create a client..
        let res = dispatch(
            &mut ctx,
            Ics26Envelope::Ics2Msg(ClientMsg::CreateClient(create_client_msg.clone())),
        );

        assert_eq!(
            true,
            res.is_ok(),
            "ICS26 routing dispatch test 'client creation' failed for message {:?} with result: {:?}",
            create_client_msg,
            res
        );

        ctx.add_port(msg_chan_init.port_id().clone());

        // Figure out the ID of the client that was just created.
        let mut events = res.unwrap().events;
        let client_id_event = events.pop();
        assert!(
            client_id_event.is_some(),
            "There was no event generated for client creation!"
        );
        let client_id = match client_id_event.unwrap() {
            IbcEvent::CreateClient(create_client) => create_client.client_id().clone(),
            event => panic!("unexpected IBC event: {:?}", event),
        };

        let tests: Vec<Test> = vec![
            // Test some ICS2 client functionality.
            Test {
                name: "Client update successful".to_string(),
                msg: Ics26Envelope::Ics2Msg(ClientMsg::UpdateClient(MsgUpdateAnyClient {
                    client_id: client_id.clone(),
                    header: MockHeader::new(update_client_height).into(),
                    signer: default_signer.clone(),
                })),
                want_pass: true,
            },
            Test {
                name: "Client update fails due to stale header".to_string(),
                msg: Ics26Envelope::Ics2Msg(ClientMsg::UpdateClient(MsgUpdateAnyClient {
                    client_id: client_id.clone(),
                    header: MockHeader::new(update_client_height).into(),
                    signer: default_signer.clone(),
                })),
                want_pass: false,
            },
            Test {
                name: "Connection open init succeeds".to_string(),
                msg: Ics26Envelope::Ics3Msg(ConnectionMsg::ConnectionOpenInit(
                    msg_conn_init.with_client_id(client_id.clone()),
                )),
                want_pass: true,
            },
            Test {
                name: "Connection open try fails due to InvalidConsensusHeight (too high)"
                    .to_string(),
                msg: Ics26Envelope::Ics3Msg(ConnectionMsg::ConnectionOpenTry(Box::new(
                    incorrect_msg_conn_try,
                ))),
                want_pass: false,
            },
            Test {
                name: "Connection open try succeeds".to_string(),
                msg: Ics26Envelope::Ics3Msg(ConnectionMsg::ConnectionOpenTry(Box::new(
                    correct_msg_conn_try.with_client_id(client_id.clone()),
                ))),
                want_pass: true,
            },
            Test {
                name: "Connection open ack succeeds".to_string(),
                msg: Ics26Envelope::Ics3Msg(ConnectionMsg::ConnectionOpenAck(Box::new(
                    msg_conn_ack,
                ))),
                want_pass: true,
            },
            // ICS04
            Test {
                name: "Channel open init succeeds".to_string(),
                msg: Ics26Envelope::Ics4ChannelMsg(ChannelMsg::ChannelOpenInit(msg_chan_init)),
                want_pass: true,
            },
            Test {
                name: "Channel open init fail due to missing connection".to_string(),
                msg: Ics26Envelope::Ics4ChannelMsg(ChannelMsg::ChannelOpenInit(
                    incorrect_msg_chan_init,
                )),
                want_pass: false,
            },
            Test {
                name: "Channel open try succeeds".to_string(),
                msg: Ics26Envelope::Ics4ChannelMsg(ChannelMsg::ChannelOpenTry(msg_chan_try)),
                want_pass: true,
            },
            Test {
                name: "Channel open ack succeeds".to_string(),
                msg: Ics26Envelope::Ics4ChannelMsg(ChannelMsg::ChannelOpenAck(msg_chan_ack)),
                want_pass: true,
            },
            //ICS20-04-packet
            Test {
                name: "Packet send".to_string(),
                msg: Ics26Envelope::Ics20Msg(msg_transfer),
                want_pass: true,
            },
            // The client update is required in this test, because the proof associated with
            // msg_recv_packet has the same height as the packet TO height (see get_dummy_raw_msg_recv_packet)
            Test {
                name: "Client update successful".to_string(),
                msg: Ics26Envelope::Ics2Msg(ClientMsg::UpdateClient(MsgUpdateAnyClient {
                    client_id: client_id.clone(),
                    header: MockHeader::new(update_client_height_after_send).into(),
                    signer: default_signer.clone(),
                })),
                want_pass: true,
            },
            Test {
                name: "Receive packet".to_string(),
                msg: Ics26Envelope::Ics4PacketMsg(PacketMsg::RecvPacket(msg_recv_packet.clone())),
                want_pass: true,
            },
            Test {
                name: "Re-Receive packet".to_string(),
                msg: Ics26Envelope::Ics4PacketMsg(PacketMsg::RecvPacket(msg_recv_packet)),
                want_pass: false,
            },
            Test {
                name: "Packet send".to_string(),
                msg: Ics26Envelope::Ics20Msg(msg_transfer_two),
                want_pass: true,
            },
            Test {
                name: "Client update successful".to_string(),
                msg: Ics26Envelope::Ics2Msg(ClientMsg::UpdateClient(MsgUpdateAnyClient {
                    client_id,
                    header: MockHeader::new(update_client_height_after_second_send).into(),
                    signer: default_signer,
                })),
                want_pass: true,
            },
            //ICS04-close channel
            Test {
                name: "Channel close init succeeds".to_string(),
                msg: Ics26Envelope::Ics4ChannelMsg(ChannelMsg::ChannelCloseInit(
                    msg_chan_close_init,
                )),
                want_pass: true,
            },
            Test {
                name: "Channel close confirm fails cause channel is already closed".to_string(),
                msg: Ics26Envelope::Ics4ChannelMsg(ChannelMsg::ChannelCloseConfirm(
                    msg_chan_close_confirm,
                )),
                want_pass: false,
            },
            //ICS04-to_on_close
            Test {
                name: "Timeout on close".to_string(),
                msg: Ics26Envelope::Ics4PacketMsg(PacketMsg::ToClosePacket(msg_to_on_close)),
                want_pass: true,
            },
        ]
        .into_iter()
        .collect();

        for test in tests {
            let res = dispatch(&mut ctx, test.msg.clone());

            assert_eq!(
                test.want_pass,
                res.is_ok(),
                "ICS26 routing dispatch test '{}' failed for message {:?}\nwith result: {:?}",
                test.name,
                test.msg,
                res
            );
        }
    }
}
