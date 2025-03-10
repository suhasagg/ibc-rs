use crate::downcast;
use crate::ics02_client::client_consensus::{AnyConsensusState, ConsensusState};
use crate::ics02_client::client_state::{AnyClientState, ClientState};
use crate::ics02_client::client_type::ClientType;
use crate::ics02_client::error::Kind;
use crate::ics02_client::header::{AnyHeader, Header};
use crate::ics03_connection::connection::ConnectionEnd;
use crate::ics04_channel::channel::ChannelEnd;
use crate::ics04_channel::packet::Sequence;
use crate::ics07_tendermint::client_def::TendermintClient;
use crate::ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot};
use crate::ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId};
use crate::Height;

#[cfg(any(test, feature = "mocks"))]
use crate::mock::client_def::MockClient;

pub trait ClientDef: Clone {
    type Header: Header;
    type ClientState: ClientState;
    type ConsensusState: ConsensusState;

    /// TODO
    fn check_header_and_update_state(
        &self,
        client_state: Self::ClientState,
        header: Self::Header,
    ) -> Result<(Self::ClientState, Self::ConsensusState), Box<dyn std::error::Error>>;

    /// Verification functions as specified in:
    /// https://github.com/cosmos/ics/tree/master/spec/ics-002-client-semantics
    ///
    /// Verify a `proof` that the consensus state of a given client (at height `consensus_height`)
    /// matches the input `consensus_state`. The parameter `counterparty_height` represent the
    /// height of the counterparty chain that this proof assumes (i.e., the height at which this
    /// proof was computed).
    #[allow(clippy::too_many_arguments)]
    fn verify_client_consensus_state(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        client_id: &ClientId,
        consensus_height: Height,
        expected_consensus_state: &AnyConsensusState,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Verify a `proof` that a connection state matches that of the input `connection_end`.
    fn verify_connection_state(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        connection_id: Option<&ConnectionId>,
        expected_connection_end: &ConnectionEnd,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Verify a `proof` that a channel state matches that of the input `channel_end`.
    #[allow(clippy::too_many_arguments)]
    fn verify_channel_state(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        port_id: &PortId,
        channel_id: &ChannelId,
        expected_channel_end: &ChannelEnd,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Verify the client state for this chain that it is stored on the counterparty chain.
    #[allow(clippy::too_many_arguments)]
    fn verify_client_full_state(
        &self,
        _client_state: &Self::ClientState,
        height: Height,
        root: &CommitmentRoot,
        prefix: &CommitmentPrefix,
        client_id: &ClientId,
        proof: &CommitmentProofBytes,
        client_state: &AnyClientState,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Verify a `proof` that a packet has been commited.
    #[allow(clippy::too_many_arguments)]
    fn verify_packet_data(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        proof: &CommitmentProofBytes,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
        commitment: String,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Verify a `proof` that a packet has been commited.
    #[allow(clippy::too_many_arguments)]
    fn verify_packet_acknowledgement(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        proof: &CommitmentProofBytes,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
        ack: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Verify a `proof` that of the next_seq_received.
    #[allow(clippy::too_many_arguments)]
    fn verify_next_sequence_recv(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        proof: &CommitmentProofBytes,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Verify a `proof` that a packet has not been received.
    #[allow(clippy::too_many_arguments)]
    fn verify_packet_receipt_absence(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        proof: &CommitmentProofBytes,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnyClient {
    Tendermint(TendermintClient),

    #[cfg(any(test, feature = "mocks"))]
    Mock(MockClient),
}

impl AnyClient {
    pub fn from_client_type(client_type: ClientType) -> AnyClient {
        match client_type {
            ClientType::Tendermint => Self::Tendermint(TendermintClient),

            #[cfg(any(test, feature = "mocks"))]
            ClientType::Mock => Self::Mock(MockClient),
        }
    }
}

// ⚠️  Beware of the awful boilerplate below ⚠️
impl ClientDef for AnyClient {
    type Header = AnyHeader;
    type ClientState = AnyClientState;
    type ConsensusState = AnyConsensusState;

    /// Validates an incoming `header` against the latest consensus state of this client.
    fn check_header_and_update_state(
        &self,
        client_state: AnyClientState,
        header: AnyHeader,
    ) -> Result<(AnyClientState, AnyConsensusState), Box<dyn std::error::Error>> {
        match self {
            Self::Tendermint(client) => {
                let (client_state, header) = downcast!(
                    client_state => AnyClientState::Tendermint,
                    header => AnyHeader::Tendermint,
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Tendermint))?;

                let (new_state, new_consensus) =
                    client.check_header_and_update_state(client_state, header)?;

                Ok((
                    AnyClientState::Tendermint(new_state),
                    AnyConsensusState::Tendermint(new_consensus),
                ))
            }

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(client) => {
                let (client_state, header) = downcast!(
                    client_state => AnyClientState::Mock,
                    header => AnyHeader::Mock,
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Mock))?;

                let (new_state, new_consensus) =
                    client.check_header_and_update_state(client_state, header)?;

                Ok((
                    AnyClientState::Mock(new_state),
                    AnyConsensusState::Mock(new_consensus),
                ))
            }
        }
    }

    fn verify_client_consensus_state(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        client_id: &ClientId,
        consensus_height: Height,
        expected_consensus_state: &AnyConsensusState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Tendermint(client) => {
                let client_state = downcast!(
                    client_state => AnyClientState::Tendermint
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Tendermint))?;

                client.verify_client_consensus_state(
                    client_state,
                    height,
                    prefix,
                    proof,
                    client_id,
                    consensus_height,
                    expected_consensus_state,
                )
            }

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(client) => {
                let client_state = downcast!(
                    client_state => AnyClientState::Mock
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Mock))?;

                client.verify_client_consensus_state(
                    client_state,
                    height,
                    prefix,
                    proof,
                    client_id,
                    consensus_height,
                    expected_consensus_state,
                )
            }
        }
    }

    fn verify_connection_state(
        &self,
        client_state: &AnyClientState,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        connection_id: Option<&ConnectionId>,
        expected_connection_end: &ConnectionEnd,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Tendermint(client) => {
                let client_state = downcast!(client_state => AnyClientState::Tendermint)
                    .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Tendermint))?;

                client.verify_connection_state(
                    client_state,
                    height,
                    prefix,
                    proof,
                    connection_id,
                    expected_connection_end,
                )
            }

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(client) => {
                let client_state = downcast!(client_state => AnyClientState::Mock)
                    .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Mock))?;

                client.verify_connection_state(
                    client_state,
                    height,
                    prefix,
                    proof,
                    connection_id,
                    expected_connection_end,
                )
            }
        }
    }

    fn verify_channel_state(
        &self,
        client_state: &AnyClientState,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        port_id: &PortId,
        channel_id: &ChannelId,
        expected_channel_end: &ChannelEnd,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Tendermint(client) => {
                let client_state = downcast!(client_state => AnyClientState::Tendermint)
                    .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Tendermint))?;

                client.verify_channel_state(
                    client_state,
                    height,
                    prefix,
                    proof,
                    port_id,
                    channel_id,
                    expected_channel_end,
                )
            }

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(client) => {
                let client_state = downcast!(client_state => AnyClientState::Mock)
                    .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Mock))?;

                client.verify_channel_state(
                    client_state,
                    height,
                    prefix,
                    proof,
                    port_id,
                    channel_id,
                    expected_channel_end,
                )
            }
        }
    }

    fn verify_client_full_state(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        root: &CommitmentRoot,
        prefix: &CommitmentPrefix,
        client_id: &ClientId,
        proof: &CommitmentProofBytes,
        client_state_on_counterparty: &AnyClientState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Tendermint(client) => {
                let client_state = downcast!(
                    client_state => AnyClientState::Tendermint
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Tendermint))?;

                client.verify_client_full_state(
                    client_state,
                    height,
                    root,
                    prefix,
                    client_id,
                    proof,
                    client_state_on_counterparty,
                )
            }

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(client) => {
                let client_state = downcast!(
                    client_state => AnyClientState::Mock
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Mock))?;

                client.verify_client_full_state(
                    client_state,
                    height,
                    root,
                    prefix,
                    client_id,
                    proof,
                    client_state_on_counterparty,
                )
            }
        }
    }
    fn verify_packet_data(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        proof: &CommitmentProofBytes,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
        commitment: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Tendermint(client) => {
                let client_state = downcast!(
                    client_state => AnyClientState::Tendermint
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Tendermint))?;

                client.verify_packet_data(
                    client_state,
                    height,
                    proof,
                    port_id,
                    channel_id,
                    seq,
                    commitment,
                )
            }

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(client) => {
                let client_state = downcast!(
                    client_state => AnyClientState::Mock
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Mock))?;

                client.verify_packet_data(
                    client_state,
                    height,
                    proof,
                    port_id,
                    channel_id,
                    seq,
                    commitment,
                )
            }
        }
    }

    fn verify_packet_acknowledgement(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        proof: &CommitmentProofBytes,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
        ack: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Tendermint(client) => {
                let client_state = downcast!(
                    client_state => AnyClientState::Tendermint
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Tendermint))?;

                client.verify_packet_acknowledgement(
                    client_state,
                    height,
                    proof,
                    port_id,
                    channel_id,
                    seq,
                    ack,
                )
            }

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(client) => {
                let client_state = downcast!(
                    client_state => AnyClientState::Mock
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Mock))?;

                client.verify_packet_acknowledgement(
                    client_state,
                    height,
                    proof,
                    port_id,
                    channel_id,
                    seq,
                    ack,
                )
            }
        }
    }

    fn verify_next_sequence_recv(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        proof: &CommitmentProofBytes,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Tendermint(client) => {
                let client_state = downcast!(
                    client_state => AnyClientState::Tendermint
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Tendermint))?;

                client.verify_next_sequence_recv(
                    client_state,
                    height,
                    proof,
                    port_id,
                    channel_id,
                    seq,
                )
            }

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(client) => {
                let client_state = downcast!(
                    client_state => AnyClientState::Mock
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Mock))?;

                client.verify_next_sequence_recv(
                    client_state,
                    height,
                    proof,
                    port_id,
                    channel_id,
                    seq,
                )
            }
        }
    }
    fn verify_packet_receipt_absence(
        &self,
        client_state: &Self::ClientState,
        height: Height,
        proof: &CommitmentProofBytes,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Tendermint(client) => {
                let client_state = downcast!(
                    client_state => AnyClientState::Tendermint
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Tendermint))?;

                client.verify_packet_receipt_absence(
                    client_state,
                    height,
                    proof,
                    port_id,
                    channel_id,
                    seq,
                )
            }

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(client) => {
                let client_state = downcast!(
                    client_state => AnyClientState::Mock
                )
                .ok_or_else(|| Kind::ClientArgsTypeMismatch(ClientType::Mock))?;

                client.verify_packet_receipt_absence(
                    client_state,
                    height,
                    proof,
                    port_id,
                    channel_id,
                    seq,
                )
            }
        }
    }
}
