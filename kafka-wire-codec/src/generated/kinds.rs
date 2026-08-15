// Generated typed dispatch enums — do not edit.
use bytes::{Bytes, BytesMut};
use crate::codec::WireBuf;
use crate::error::DecodeError;

/// Every Kafka request, as one typed enum: decode by api key, match by variant.
#[derive(Debug, Clone, PartialEq)]
pub enum RequestKind {
    Produce(super::produce_request::ProduceRequest),
    Fetch(super::fetch_request::FetchRequest),
    ListOffsets(super::list_offsets_request::ListOffsetsRequest),
    Metadata(super::metadata_request::MetadataRequest),
    LeaderAndIsr(super::leader_and_isr_request::LeaderAndIsrRequest),
    StopReplica(super::stop_replica_request::StopReplicaRequest),
    UpdateMetadata(super::update_metadata_request::UpdateMetadataRequest),
    ControlledShutdown(super::controlled_shutdown_request::ControlledShutdownRequest),
    OffsetCommit(super::offset_commit_request::OffsetCommitRequest),
    OffsetFetch(super::offset_fetch_request::OffsetFetchRequest),
    FindCoordinator(super::find_coordinator_request::FindCoordinatorRequest),
    JoinGroup(super::join_group_request::JoinGroupRequest),
    Heartbeat(super::heartbeat_request::HeartbeatRequest),
    LeaveGroup(super::leave_group_request::LeaveGroupRequest),
    SyncGroup(super::sync_group_request::SyncGroupRequest),
    DescribeGroups(super::describe_groups_request::DescribeGroupsRequest),
    ListGroups(super::list_groups_request::ListGroupsRequest),
    SaslHandshake(super::sasl_handshake_request::SaslHandshakeRequest),
    ApiVersions(super::api_versions_request::ApiVersionsRequest),
    CreateTopics(super::create_topics_request::CreateTopicsRequest),
    DeleteTopics(super::delete_topics_request::DeleteTopicsRequest),
    DeleteRecords(super::delete_records_request::DeleteRecordsRequest),
    InitProducerId(super::init_producer_id_request::InitProducerIdRequest),
    OffsetForLeaderEpoch(super::offset_for_leader_epoch_request::OffsetForLeaderEpochRequest),
    AddPartitionsToTxn(super::add_partitions_to_txn_request::AddPartitionsToTxnRequest),
    AddOffsetsToTxn(super::add_offsets_to_txn_request::AddOffsetsToTxnRequest),
    EndTxn(super::end_txn_request::EndTxnRequest),
    WriteTxnMarkers(super::write_txn_markers_request::WriteTxnMarkersRequest),
    TxnOffsetCommit(super::txn_offset_commit_request::TxnOffsetCommitRequest),
    DescribeAcls(super::describe_acls_request::DescribeAclsRequest),
    CreateAcls(super::create_acls_request::CreateAclsRequest),
    DeleteAcls(super::delete_acls_request::DeleteAclsRequest),
    DescribeConfigs(super::describe_configs_request::DescribeConfigsRequest),
    AlterConfigs(super::alter_configs_request::AlterConfigsRequest),
    AlterReplicaLogDirs(super::alter_replica_log_dirs_request::AlterReplicaLogDirsRequest),
    DescribeLogDirs(super::describe_log_dirs_request::DescribeLogDirsRequest),
    SaslAuthenticate(super::sasl_authenticate_request::SaslAuthenticateRequest),
    CreatePartitions(super::create_partitions_request::CreatePartitionsRequest),
    CreateDelegationToken(super::create_delegation_token_request::CreateDelegationTokenRequest),
    RenewDelegationToken(super::renew_delegation_token_request::RenewDelegationTokenRequest),
    ExpireDelegationToken(super::expire_delegation_token_request::ExpireDelegationTokenRequest),
    DescribeDelegationToken(super::describe_delegation_token_request::DescribeDelegationTokenRequest),
    DeleteGroups(super::delete_groups_request::DeleteGroupsRequest),
    ElectLeaders(super::elect_leaders_request::ElectLeadersRequest),
    IncrementalAlterConfigs(super::incremental_alter_configs_request::IncrementalAlterConfigsRequest),
    AlterPartitionReassignments(super::alter_partition_reassignments_request::AlterPartitionReassignmentsRequest),
    ListPartitionReassignments(super::list_partition_reassignments_request::ListPartitionReassignmentsRequest),
    OffsetDelete(super::offset_delete_request::OffsetDeleteRequest),
    DescribeClientQuotas(super::describe_client_quotas_request::DescribeClientQuotasRequest),
    AlterClientQuotas(super::alter_client_quotas_request::AlterClientQuotasRequest),
    DescribeUserScramCredentials(super::describe_user_scram_credentials_request::DescribeUserScramCredentialsRequest),
    AlterUserScramCredentials(super::alter_user_scram_credentials_request::AlterUserScramCredentialsRequest),
    Vote(super::vote_request::VoteRequest),
    BeginQuorumEpoch(super::begin_quorum_epoch_request::BeginQuorumEpochRequest),
    EndQuorumEpoch(super::end_quorum_epoch_request::EndQuorumEpochRequest),
    DescribeQuorum(super::describe_quorum_request::DescribeQuorumRequest),
    AlterPartition(super::alter_partition_request::AlterPartitionRequest),
    UpdateFeatures(super::update_features_request::UpdateFeaturesRequest),
    Envelope(super::envelope_request::EnvelopeRequest),
    FetchSnapshot(super::fetch_snapshot_request::FetchSnapshotRequest),
    DescribeCluster(super::describe_cluster_request::DescribeClusterRequest),
    DescribeProducers(super::describe_producers_request::DescribeProducersRequest),
    BrokerRegistration(super::broker_registration_request::BrokerRegistrationRequest),
    BrokerHeartbeat(super::broker_heartbeat_request::BrokerHeartbeatRequest),
    UnregisterBroker(super::unregister_broker_request::UnregisterBrokerRequest),
    DescribeTransactions(super::describe_transactions_request::DescribeTransactionsRequest),
    ListTransactions(super::list_transactions_request::ListTransactionsRequest),
    AllocateProducerIds(super::allocate_producer_ids_request::AllocateProducerIdsRequest),
    ConsumerGroupHeartbeat(super::consumer_group_heartbeat_request::ConsumerGroupHeartbeatRequest),
    ConsumerGroupDescribe(super::consumer_group_describe_request::ConsumerGroupDescribeRequest),
    ControllerRegistration(super::controller_registration_request::ControllerRegistrationRequest),
    GetTelemetrySubscriptions(super::get_telemetry_subscriptions_request::GetTelemetrySubscriptionsRequest),
    PushTelemetry(super::push_telemetry_request::PushTelemetryRequest),
    AssignReplicasToDirs(super::assign_replicas_to_dirs_request::AssignReplicasToDirsRequest),
    ListConfigResources(super::list_config_resources_request::ListConfigResourcesRequest),
    DescribeTopicPartitions(super::describe_topic_partitions_request::DescribeTopicPartitionsRequest),
    ShareGroupHeartbeat(super::share_group_heartbeat_request::ShareGroupHeartbeatRequest),
    ShareGroupDescribe(super::share_group_describe_request::ShareGroupDescribeRequest),
    ShareFetch(super::share_fetch_request::ShareFetchRequest),
    ShareAcknowledge(super::share_acknowledge_request::ShareAcknowledgeRequest),
    AddRaftVoter(super::add_raft_voter_request::AddRaftVoterRequest),
    RemoveRaftVoter(super::remove_raft_voter_request::RemoveRaftVoterRequest),
    UpdateRaftVoter(super::update_raft_voter_request::UpdateRaftVoterRequest),
    InitializeShareGroupState(super::initialize_share_group_state_request::InitializeShareGroupStateRequest),
    ReadShareGroupState(super::read_share_group_state_request::ReadShareGroupStateRequest),
    WriteShareGroupState(super::write_share_group_state_request::WriteShareGroupStateRequest),
    DeleteShareGroupState(super::delete_share_group_state_request::DeleteShareGroupStateRequest),
    ReadShareGroupStateSummary(super::read_share_group_state_summary_request::ReadShareGroupStateSummaryRequest),
    StreamsGroupHeartbeat(super::streams_group_heartbeat_request::StreamsGroupHeartbeatRequest),
    StreamsGroupDescribe(super::streams_group_describe_request::StreamsGroupDescribeRequest),
    DescribeShareGroupOffsets(super::describe_share_group_offsets_request::DescribeShareGroupOffsetsRequest),
    AlterShareGroupOffsets(super::alter_share_group_offsets_request::AlterShareGroupOffsetsRequest),
    DeleteShareGroupOffsets(super::delete_share_group_offsets_request::DeleteShareGroupOffsetsRequest),
}

impl RequestKind {
    /// Decode the request body for `api_key` at `version`.
    /// Returns `DecodeError::UnknownApiKey` for api keys this build doesn't know.
    pub fn decode(api_key: i16, version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        match api_key {
            0 => Ok(Self::Produce(super::produce_request::ProduceRequest::decode(version, buf)?)),
            1 => Ok(Self::Fetch(super::fetch_request::FetchRequest::decode(version, buf)?)),
            2 => Ok(Self::ListOffsets(super::list_offsets_request::ListOffsetsRequest::decode(version, buf)?)),
            3 => Ok(Self::Metadata(super::metadata_request::MetadataRequest::decode(version, buf)?)),
            4 => Ok(Self::LeaderAndIsr(super::leader_and_isr_request::LeaderAndIsrRequest::decode(version, buf)?)),
            5 => Ok(Self::StopReplica(super::stop_replica_request::StopReplicaRequest::decode(version, buf)?)),
            6 => Ok(Self::UpdateMetadata(super::update_metadata_request::UpdateMetadataRequest::decode(version, buf)?)),
            7 => Ok(Self::ControlledShutdown(super::controlled_shutdown_request::ControlledShutdownRequest::decode(version, buf)?)),
            8 => Ok(Self::OffsetCommit(super::offset_commit_request::OffsetCommitRequest::decode(version, buf)?)),
            9 => Ok(Self::OffsetFetch(super::offset_fetch_request::OffsetFetchRequest::decode(version, buf)?)),
            10 => Ok(Self::FindCoordinator(super::find_coordinator_request::FindCoordinatorRequest::decode(version, buf)?)),
            11 => Ok(Self::JoinGroup(super::join_group_request::JoinGroupRequest::decode(version, buf)?)),
            12 => Ok(Self::Heartbeat(super::heartbeat_request::HeartbeatRequest::decode(version, buf)?)),
            13 => Ok(Self::LeaveGroup(super::leave_group_request::LeaveGroupRequest::decode(version, buf)?)),
            14 => Ok(Self::SyncGroup(super::sync_group_request::SyncGroupRequest::decode(version, buf)?)),
            15 => Ok(Self::DescribeGroups(super::describe_groups_request::DescribeGroupsRequest::decode(version, buf)?)),
            16 => Ok(Self::ListGroups(super::list_groups_request::ListGroupsRequest::decode(version, buf)?)),
            17 => Ok(Self::SaslHandshake(super::sasl_handshake_request::SaslHandshakeRequest::decode(version, buf)?)),
            18 => Ok(Self::ApiVersions(super::api_versions_request::ApiVersionsRequest::decode(version, buf)?)),
            19 => Ok(Self::CreateTopics(super::create_topics_request::CreateTopicsRequest::decode(version, buf)?)),
            20 => Ok(Self::DeleteTopics(super::delete_topics_request::DeleteTopicsRequest::decode(version, buf)?)),
            21 => Ok(Self::DeleteRecords(super::delete_records_request::DeleteRecordsRequest::decode(version, buf)?)),
            22 => Ok(Self::InitProducerId(super::init_producer_id_request::InitProducerIdRequest::decode(version, buf)?)),
            23 => Ok(Self::OffsetForLeaderEpoch(super::offset_for_leader_epoch_request::OffsetForLeaderEpochRequest::decode(version, buf)?)),
            24 => Ok(Self::AddPartitionsToTxn(super::add_partitions_to_txn_request::AddPartitionsToTxnRequest::decode(version, buf)?)),
            25 => Ok(Self::AddOffsetsToTxn(super::add_offsets_to_txn_request::AddOffsetsToTxnRequest::decode(version, buf)?)),
            26 => Ok(Self::EndTxn(super::end_txn_request::EndTxnRequest::decode(version, buf)?)),
            27 => Ok(Self::WriteTxnMarkers(super::write_txn_markers_request::WriteTxnMarkersRequest::decode(version, buf)?)),
            28 => Ok(Self::TxnOffsetCommit(super::txn_offset_commit_request::TxnOffsetCommitRequest::decode(version, buf)?)),
            29 => Ok(Self::DescribeAcls(super::describe_acls_request::DescribeAclsRequest::decode(version, buf)?)),
            30 => Ok(Self::CreateAcls(super::create_acls_request::CreateAclsRequest::decode(version, buf)?)),
            31 => Ok(Self::DeleteAcls(super::delete_acls_request::DeleteAclsRequest::decode(version, buf)?)),
            32 => Ok(Self::DescribeConfigs(super::describe_configs_request::DescribeConfigsRequest::decode(version, buf)?)),
            33 => Ok(Self::AlterConfigs(super::alter_configs_request::AlterConfigsRequest::decode(version, buf)?)),
            34 => Ok(Self::AlterReplicaLogDirs(super::alter_replica_log_dirs_request::AlterReplicaLogDirsRequest::decode(version, buf)?)),
            35 => Ok(Self::DescribeLogDirs(super::describe_log_dirs_request::DescribeLogDirsRequest::decode(version, buf)?)),
            36 => Ok(Self::SaslAuthenticate(super::sasl_authenticate_request::SaslAuthenticateRequest::decode(version, buf)?)),
            37 => Ok(Self::CreatePartitions(super::create_partitions_request::CreatePartitionsRequest::decode(version, buf)?)),
            38 => Ok(Self::CreateDelegationToken(super::create_delegation_token_request::CreateDelegationTokenRequest::decode(version, buf)?)),
            39 => Ok(Self::RenewDelegationToken(super::renew_delegation_token_request::RenewDelegationTokenRequest::decode(version, buf)?)),
            40 => Ok(Self::ExpireDelegationToken(super::expire_delegation_token_request::ExpireDelegationTokenRequest::decode(version, buf)?)),
            41 => Ok(Self::DescribeDelegationToken(super::describe_delegation_token_request::DescribeDelegationTokenRequest::decode(version, buf)?)),
            42 => Ok(Self::DeleteGroups(super::delete_groups_request::DeleteGroupsRequest::decode(version, buf)?)),
            43 => Ok(Self::ElectLeaders(super::elect_leaders_request::ElectLeadersRequest::decode(version, buf)?)),
            44 => Ok(Self::IncrementalAlterConfigs(super::incremental_alter_configs_request::IncrementalAlterConfigsRequest::decode(version, buf)?)),
            45 => Ok(Self::AlterPartitionReassignments(super::alter_partition_reassignments_request::AlterPartitionReassignmentsRequest::decode(version, buf)?)),
            46 => Ok(Self::ListPartitionReassignments(super::list_partition_reassignments_request::ListPartitionReassignmentsRequest::decode(version, buf)?)),
            47 => Ok(Self::OffsetDelete(super::offset_delete_request::OffsetDeleteRequest::decode(version, buf)?)),
            48 => Ok(Self::DescribeClientQuotas(super::describe_client_quotas_request::DescribeClientQuotasRequest::decode(version, buf)?)),
            49 => Ok(Self::AlterClientQuotas(super::alter_client_quotas_request::AlterClientQuotasRequest::decode(version, buf)?)),
            50 => Ok(Self::DescribeUserScramCredentials(super::describe_user_scram_credentials_request::DescribeUserScramCredentialsRequest::decode(version, buf)?)),
            51 => Ok(Self::AlterUserScramCredentials(super::alter_user_scram_credentials_request::AlterUserScramCredentialsRequest::decode(version, buf)?)),
            52 => Ok(Self::Vote(super::vote_request::VoteRequest::decode(version, buf)?)),
            53 => Ok(Self::BeginQuorumEpoch(super::begin_quorum_epoch_request::BeginQuorumEpochRequest::decode(version, buf)?)),
            54 => Ok(Self::EndQuorumEpoch(super::end_quorum_epoch_request::EndQuorumEpochRequest::decode(version, buf)?)),
            55 => Ok(Self::DescribeQuorum(super::describe_quorum_request::DescribeQuorumRequest::decode(version, buf)?)),
            56 => Ok(Self::AlterPartition(super::alter_partition_request::AlterPartitionRequest::decode(version, buf)?)),
            57 => Ok(Self::UpdateFeatures(super::update_features_request::UpdateFeaturesRequest::decode(version, buf)?)),
            58 => Ok(Self::Envelope(super::envelope_request::EnvelopeRequest::decode(version, buf)?)),
            59 => Ok(Self::FetchSnapshot(super::fetch_snapshot_request::FetchSnapshotRequest::decode(version, buf)?)),
            60 => Ok(Self::DescribeCluster(super::describe_cluster_request::DescribeClusterRequest::decode(version, buf)?)),
            61 => Ok(Self::DescribeProducers(super::describe_producers_request::DescribeProducersRequest::decode(version, buf)?)),
            62 => Ok(Self::BrokerRegistration(super::broker_registration_request::BrokerRegistrationRequest::decode(version, buf)?)),
            63 => Ok(Self::BrokerHeartbeat(super::broker_heartbeat_request::BrokerHeartbeatRequest::decode(version, buf)?)),
            64 => Ok(Self::UnregisterBroker(super::unregister_broker_request::UnregisterBrokerRequest::decode(version, buf)?)),
            65 => Ok(Self::DescribeTransactions(super::describe_transactions_request::DescribeTransactionsRequest::decode(version, buf)?)),
            66 => Ok(Self::ListTransactions(super::list_transactions_request::ListTransactionsRequest::decode(version, buf)?)),
            67 => Ok(Self::AllocateProducerIds(super::allocate_producer_ids_request::AllocateProducerIdsRequest::decode(version, buf)?)),
            68 => Ok(Self::ConsumerGroupHeartbeat(super::consumer_group_heartbeat_request::ConsumerGroupHeartbeatRequest::decode(version, buf)?)),
            69 => Ok(Self::ConsumerGroupDescribe(super::consumer_group_describe_request::ConsumerGroupDescribeRequest::decode(version, buf)?)),
            70 => Ok(Self::ControllerRegistration(super::controller_registration_request::ControllerRegistrationRequest::decode(version, buf)?)),
            71 => Ok(Self::GetTelemetrySubscriptions(super::get_telemetry_subscriptions_request::GetTelemetrySubscriptionsRequest::decode(version, buf)?)),
            72 => Ok(Self::PushTelemetry(super::push_telemetry_request::PushTelemetryRequest::decode(version, buf)?)),
            73 => Ok(Self::AssignReplicasToDirs(super::assign_replicas_to_dirs_request::AssignReplicasToDirsRequest::decode(version, buf)?)),
            74 => Ok(Self::ListConfigResources(super::list_config_resources_request::ListConfigResourcesRequest::decode(version, buf)?)),
            75 => Ok(Self::DescribeTopicPartitions(super::describe_topic_partitions_request::DescribeTopicPartitionsRequest::decode(version, buf)?)),
            76 => Ok(Self::ShareGroupHeartbeat(super::share_group_heartbeat_request::ShareGroupHeartbeatRequest::decode(version, buf)?)),
            77 => Ok(Self::ShareGroupDescribe(super::share_group_describe_request::ShareGroupDescribeRequest::decode(version, buf)?)),
            78 => Ok(Self::ShareFetch(super::share_fetch_request::ShareFetchRequest::decode(version, buf)?)),
            79 => Ok(Self::ShareAcknowledge(super::share_acknowledge_request::ShareAcknowledgeRequest::decode(version, buf)?)),
            80 => Ok(Self::AddRaftVoter(super::add_raft_voter_request::AddRaftVoterRequest::decode(version, buf)?)),
            81 => Ok(Self::RemoveRaftVoter(super::remove_raft_voter_request::RemoveRaftVoterRequest::decode(version, buf)?)),
            82 => Ok(Self::UpdateRaftVoter(super::update_raft_voter_request::UpdateRaftVoterRequest::decode(version, buf)?)),
            83 => Ok(Self::InitializeShareGroupState(super::initialize_share_group_state_request::InitializeShareGroupStateRequest::decode(version, buf)?)),
            84 => Ok(Self::ReadShareGroupState(super::read_share_group_state_request::ReadShareGroupStateRequest::decode(version, buf)?)),
            85 => Ok(Self::WriteShareGroupState(super::write_share_group_state_request::WriteShareGroupStateRequest::decode(version, buf)?)),
            86 => Ok(Self::DeleteShareGroupState(super::delete_share_group_state_request::DeleteShareGroupStateRequest::decode(version, buf)?)),
            87 => Ok(Self::ReadShareGroupStateSummary(super::read_share_group_state_summary_request::ReadShareGroupStateSummaryRequest::decode(version, buf)?)),
            88 => Ok(Self::StreamsGroupHeartbeat(super::streams_group_heartbeat_request::StreamsGroupHeartbeatRequest::decode(version, buf)?)),
            89 => Ok(Self::StreamsGroupDescribe(super::streams_group_describe_request::StreamsGroupDescribeRequest::decode(version, buf)?)),
            90 => Ok(Self::DescribeShareGroupOffsets(super::describe_share_group_offsets_request::DescribeShareGroupOffsetsRequest::decode(version, buf)?)),
            91 => Ok(Self::AlterShareGroupOffsets(super::alter_share_group_offsets_request::AlterShareGroupOffsetsRequest::decode(version, buf)?)),
            92 => Ok(Self::DeleteShareGroupOffsets(super::delete_share_group_offsets_request::DeleteShareGroupOffsetsRequest::decode(version, buf)?)),
            _ => Err(DecodeError::UnknownApiKey(api_key)),
        }
    }

    /// The Kafka API key of the contained message.
    pub fn api_key(&self) -> i16 {
        match self {
            Self::Produce(_) => 0,
            Self::Fetch(_) => 1,
            Self::ListOffsets(_) => 2,
            Self::Metadata(_) => 3,
            Self::LeaderAndIsr(_) => 4,
            Self::StopReplica(_) => 5,
            Self::UpdateMetadata(_) => 6,
            Self::ControlledShutdown(_) => 7,
            Self::OffsetCommit(_) => 8,
            Self::OffsetFetch(_) => 9,
            Self::FindCoordinator(_) => 10,
            Self::JoinGroup(_) => 11,
            Self::Heartbeat(_) => 12,
            Self::LeaveGroup(_) => 13,
            Self::SyncGroup(_) => 14,
            Self::DescribeGroups(_) => 15,
            Self::ListGroups(_) => 16,
            Self::SaslHandshake(_) => 17,
            Self::ApiVersions(_) => 18,
            Self::CreateTopics(_) => 19,
            Self::DeleteTopics(_) => 20,
            Self::DeleteRecords(_) => 21,
            Self::InitProducerId(_) => 22,
            Self::OffsetForLeaderEpoch(_) => 23,
            Self::AddPartitionsToTxn(_) => 24,
            Self::AddOffsetsToTxn(_) => 25,
            Self::EndTxn(_) => 26,
            Self::WriteTxnMarkers(_) => 27,
            Self::TxnOffsetCommit(_) => 28,
            Self::DescribeAcls(_) => 29,
            Self::CreateAcls(_) => 30,
            Self::DeleteAcls(_) => 31,
            Self::DescribeConfigs(_) => 32,
            Self::AlterConfigs(_) => 33,
            Self::AlterReplicaLogDirs(_) => 34,
            Self::DescribeLogDirs(_) => 35,
            Self::SaslAuthenticate(_) => 36,
            Self::CreatePartitions(_) => 37,
            Self::CreateDelegationToken(_) => 38,
            Self::RenewDelegationToken(_) => 39,
            Self::ExpireDelegationToken(_) => 40,
            Self::DescribeDelegationToken(_) => 41,
            Self::DeleteGroups(_) => 42,
            Self::ElectLeaders(_) => 43,
            Self::IncrementalAlterConfigs(_) => 44,
            Self::AlterPartitionReassignments(_) => 45,
            Self::ListPartitionReassignments(_) => 46,
            Self::OffsetDelete(_) => 47,
            Self::DescribeClientQuotas(_) => 48,
            Self::AlterClientQuotas(_) => 49,
            Self::DescribeUserScramCredentials(_) => 50,
            Self::AlterUserScramCredentials(_) => 51,
            Self::Vote(_) => 52,
            Self::BeginQuorumEpoch(_) => 53,
            Self::EndQuorumEpoch(_) => 54,
            Self::DescribeQuorum(_) => 55,
            Self::AlterPartition(_) => 56,
            Self::UpdateFeatures(_) => 57,
            Self::Envelope(_) => 58,
            Self::FetchSnapshot(_) => 59,
            Self::DescribeCluster(_) => 60,
            Self::DescribeProducers(_) => 61,
            Self::BrokerRegistration(_) => 62,
            Self::BrokerHeartbeat(_) => 63,
            Self::UnregisterBroker(_) => 64,
            Self::DescribeTransactions(_) => 65,
            Self::ListTransactions(_) => 66,
            Self::AllocateProducerIds(_) => 67,
            Self::ConsumerGroupHeartbeat(_) => 68,
            Self::ConsumerGroupDescribe(_) => 69,
            Self::ControllerRegistration(_) => 70,
            Self::GetTelemetrySubscriptions(_) => 71,
            Self::PushTelemetry(_) => 72,
            Self::AssignReplicasToDirs(_) => 73,
            Self::ListConfigResources(_) => 74,
            Self::DescribeTopicPartitions(_) => 75,
            Self::ShareGroupHeartbeat(_) => 76,
            Self::ShareGroupDescribe(_) => 77,
            Self::ShareFetch(_) => 78,
            Self::ShareAcknowledge(_) => 79,
            Self::AddRaftVoter(_) => 80,
            Self::RemoveRaftVoter(_) => 81,
            Self::UpdateRaftVoter(_) => 82,
            Self::InitializeShareGroupState(_) => 83,
            Self::ReadShareGroupState(_) => 84,
            Self::WriteShareGroupState(_) => 85,
            Self::DeleteShareGroupState(_) => 86,
            Self::ReadShareGroupStateSummary(_) => 87,
            Self::StreamsGroupHeartbeat(_) => 88,
            Self::StreamsGroupDescribe(_) => 89,
            Self::DescribeShareGroupOffsets(_) => 90,
            Self::AlterShareGroupOffsets(_) => 91,
            Self::DeleteShareGroupOffsets(_) => 92,
        }
    }

    /// The API name of the contained message (e.g. "Produce").
    pub fn name(&self) -> &'static str {
        match self {
            Self::Produce(_) => "Produce",
            Self::Fetch(_) => "Fetch",
            Self::ListOffsets(_) => "ListOffsets",
            Self::Metadata(_) => "Metadata",
            Self::LeaderAndIsr(_) => "LeaderAndIsr",
            Self::StopReplica(_) => "StopReplica",
            Self::UpdateMetadata(_) => "UpdateMetadata",
            Self::ControlledShutdown(_) => "ControlledShutdown",
            Self::OffsetCommit(_) => "OffsetCommit",
            Self::OffsetFetch(_) => "OffsetFetch",
            Self::FindCoordinator(_) => "FindCoordinator",
            Self::JoinGroup(_) => "JoinGroup",
            Self::Heartbeat(_) => "Heartbeat",
            Self::LeaveGroup(_) => "LeaveGroup",
            Self::SyncGroup(_) => "SyncGroup",
            Self::DescribeGroups(_) => "DescribeGroups",
            Self::ListGroups(_) => "ListGroups",
            Self::SaslHandshake(_) => "SaslHandshake",
            Self::ApiVersions(_) => "ApiVersions",
            Self::CreateTopics(_) => "CreateTopics",
            Self::DeleteTopics(_) => "DeleteTopics",
            Self::DeleteRecords(_) => "DeleteRecords",
            Self::InitProducerId(_) => "InitProducerId",
            Self::OffsetForLeaderEpoch(_) => "OffsetForLeaderEpoch",
            Self::AddPartitionsToTxn(_) => "AddPartitionsToTxn",
            Self::AddOffsetsToTxn(_) => "AddOffsetsToTxn",
            Self::EndTxn(_) => "EndTxn",
            Self::WriteTxnMarkers(_) => "WriteTxnMarkers",
            Self::TxnOffsetCommit(_) => "TxnOffsetCommit",
            Self::DescribeAcls(_) => "DescribeAcls",
            Self::CreateAcls(_) => "CreateAcls",
            Self::DeleteAcls(_) => "DeleteAcls",
            Self::DescribeConfigs(_) => "DescribeConfigs",
            Self::AlterConfigs(_) => "AlterConfigs",
            Self::AlterReplicaLogDirs(_) => "AlterReplicaLogDirs",
            Self::DescribeLogDirs(_) => "DescribeLogDirs",
            Self::SaslAuthenticate(_) => "SaslAuthenticate",
            Self::CreatePartitions(_) => "CreatePartitions",
            Self::CreateDelegationToken(_) => "CreateDelegationToken",
            Self::RenewDelegationToken(_) => "RenewDelegationToken",
            Self::ExpireDelegationToken(_) => "ExpireDelegationToken",
            Self::DescribeDelegationToken(_) => "DescribeDelegationToken",
            Self::DeleteGroups(_) => "DeleteGroups",
            Self::ElectLeaders(_) => "ElectLeaders",
            Self::IncrementalAlterConfigs(_) => "IncrementalAlterConfigs",
            Self::AlterPartitionReassignments(_) => "AlterPartitionReassignments",
            Self::ListPartitionReassignments(_) => "ListPartitionReassignments",
            Self::OffsetDelete(_) => "OffsetDelete",
            Self::DescribeClientQuotas(_) => "DescribeClientQuotas",
            Self::AlterClientQuotas(_) => "AlterClientQuotas",
            Self::DescribeUserScramCredentials(_) => "DescribeUserScramCredentials",
            Self::AlterUserScramCredentials(_) => "AlterUserScramCredentials",
            Self::Vote(_) => "Vote",
            Self::BeginQuorumEpoch(_) => "BeginQuorumEpoch",
            Self::EndQuorumEpoch(_) => "EndQuorumEpoch",
            Self::DescribeQuorum(_) => "DescribeQuorum",
            Self::AlterPartition(_) => "AlterPartition",
            Self::UpdateFeatures(_) => "UpdateFeatures",
            Self::Envelope(_) => "Envelope",
            Self::FetchSnapshot(_) => "FetchSnapshot",
            Self::DescribeCluster(_) => "DescribeCluster",
            Self::DescribeProducers(_) => "DescribeProducers",
            Self::BrokerRegistration(_) => "BrokerRegistration",
            Self::BrokerHeartbeat(_) => "BrokerHeartbeat",
            Self::UnregisterBroker(_) => "UnregisterBroker",
            Self::DescribeTransactions(_) => "DescribeTransactions",
            Self::ListTransactions(_) => "ListTransactions",
            Self::AllocateProducerIds(_) => "AllocateProducerIds",
            Self::ConsumerGroupHeartbeat(_) => "ConsumerGroupHeartbeat",
            Self::ConsumerGroupDescribe(_) => "ConsumerGroupDescribe",
            Self::ControllerRegistration(_) => "ControllerRegistration",
            Self::GetTelemetrySubscriptions(_) => "GetTelemetrySubscriptions",
            Self::PushTelemetry(_) => "PushTelemetry",
            Self::AssignReplicasToDirs(_) => "AssignReplicasToDirs",
            Self::ListConfigResources(_) => "ListConfigResources",
            Self::DescribeTopicPartitions(_) => "DescribeTopicPartitions",
            Self::ShareGroupHeartbeat(_) => "ShareGroupHeartbeat",
            Self::ShareGroupDescribe(_) => "ShareGroupDescribe",
            Self::ShareFetch(_) => "ShareFetch",
            Self::ShareAcknowledge(_) => "ShareAcknowledge",
            Self::AddRaftVoter(_) => "AddRaftVoter",
            Self::RemoveRaftVoter(_) => "RemoveRaftVoter",
            Self::UpdateRaftVoter(_) => "UpdateRaftVoter",
            Self::InitializeShareGroupState(_) => "InitializeShareGroupState",
            Self::ReadShareGroupState(_) => "ReadShareGroupState",
            Self::WriteShareGroupState(_) => "WriteShareGroupState",
            Self::DeleteShareGroupState(_) => "DeleteShareGroupState",
            Self::ReadShareGroupStateSummary(_) => "ReadShareGroupStateSummary",
            Self::StreamsGroupHeartbeat(_) => "StreamsGroupHeartbeat",
            Self::StreamsGroupDescribe(_) => "StreamsGroupDescribe",
            Self::DescribeShareGroupOffsets(_) => "DescribeShareGroupOffsets",
            Self::AlterShareGroupOffsets(_) => "AlterShareGroupOffsets",
            Self::DeleteShareGroupOffsets(_) => "DeleteShareGroupOffsets",
        }
    }

    /// Exact encoded size at `version` (size-first encoding).
    pub fn encoded_size(&self, version: i16) -> usize {
        match self {
            Self::Produce(m) => m.encoded_size(version),
            Self::Fetch(m) => m.encoded_size(version),
            Self::ListOffsets(m) => m.encoded_size(version),
            Self::Metadata(m) => m.encoded_size(version),
            Self::LeaderAndIsr(m) => m.encoded_size(version),
            Self::StopReplica(m) => m.encoded_size(version),
            Self::UpdateMetadata(m) => m.encoded_size(version),
            Self::ControlledShutdown(m) => m.encoded_size(version),
            Self::OffsetCommit(m) => m.encoded_size(version),
            Self::OffsetFetch(m) => m.encoded_size(version),
            Self::FindCoordinator(m) => m.encoded_size(version),
            Self::JoinGroup(m) => m.encoded_size(version),
            Self::Heartbeat(m) => m.encoded_size(version),
            Self::LeaveGroup(m) => m.encoded_size(version),
            Self::SyncGroup(m) => m.encoded_size(version),
            Self::DescribeGroups(m) => m.encoded_size(version),
            Self::ListGroups(m) => m.encoded_size(version),
            Self::SaslHandshake(m) => m.encoded_size(version),
            Self::ApiVersions(m) => m.encoded_size(version),
            Self::CreateTopics(m) => m.encoded_size(version),
            Self::DeleteTopics(m) => m.encoded_size(version),
            Self::DeleteRecords(m) => m.encoded_size(version),
            Self::InitProducerId(m) => m.encoded_size(version),
            Self::OffsetForLeaderEpoch(m) => m.encoded_size(version),
            Self::AddPartitionsToTxn(m) => m.encoded_size(version),
            Self::AddOffsetsToTxn(m) => m.encoded_size(version),
            Self::EndTxn(m) => m.encoded_size(version),
            Self::WriteTxnMarkers(m) => m.encoded_size(version),
            Self::TxnOffsetCommit(m) => m.encoded_size(version),
            Self::DescribeAcls(m) => m.encoded_size(version),
            Self::CreateAcls(m) => m.encoded_size(version),
            Self::DeleteAcls(m) => m.encoded_size(version),
            Self::DescribeConfigs(m) => m.encoded_size(version),
            Self::AlterConfigs(m) => m.encoded_size(version),
            Self::AlterReplicaLogDirs(m) => m.encoded_size(version),
            Self::DescribeLogDirs(m) => m.encoded_size(version),
            Self::SaslAuthenticate(m) => m.encoded_size(version),
            Self::CreatePartitions(m) => m.encoded_size(version),
            Self::CreateDelegationToken(m) => m.encoded_size(version),
            Self::RenewDelegationToken(m) => m.encoded_size(version),
            Self::ExpireDelegationToken(m) => m.encoded_size(version),
            Self::DescribeDelegationToken(m) => m.encoded_size(version),
            Self::DeleteGroups(m) => m.encoded_size(version),
            Self::ElectLeaders(m) => m.encoded_size(version),
            Self::IncrementalAlterConfigs(m) => m.encoded_size(version),
            Self::AlterPartitionReassignments(m) => m.encoded_size(version),
            Self::ListPartitionReassignments(m) => m.encoded_size(version),
            Self::OffsetDelete(m) => m.encoded_size(version),
            Self::DescribeClientQuotas(m) => m.encoded_size(version),
            Self::AlterClientQuotas(m) => m.encoded_size(version),
            Self::DescribeUserScramCredentials(m) => m.encoded_size(version),
            Self::AlterUserScramCredentials(m) => m.encoded_size(version),
            Self::Vote(m) => m.encoded_size(version),
            Self::BeginQuorumEpoch(m) => m.encoded_size(version),
            Self::EndQuorumEpoch(m) => m.encoded_size(version),
            Self::DescribeQuorum(m) => m.encoded_size(version),
            Self::AlterPartition(m) => m.encoded_size(version),
            Self::UpdateFeatures(m) => m.encoded_size(version),
            Self::Envelope(m) => m.encoded_size(version),
            Self::FetchSnapshot(m) => m.encoded_size(version),
            Self::DescribeCluster(m) => m.encoded_size(version),
            Self::DescribeProducers(m) => m.encoded_size(version),
            Self::BrokerRegistration(m) => m.encoded_size(version),
            Self::BrokerHeartbeat(m) => m.encoded_size(version),
            Self::UnregisterBroker(m) => m.encoded_size(version),
            Self::DescribeTransactions(m) => m.encoded_size(version),
            Self::ListTransactions(m) => m.encoded_size(version),
            Self::AllocateProducerIds(m) => m.encoded_size(version),
            Self::ConsumerGroupHeartbeat(m) => m.encoded_size(version),
            Self::ConsumerGroupDescribe(m) => m.encoded_size(version),
            Self::ControllerRegistration(m) => m.encoded_size(version),
            Self::GetTelemetrySubscriptions(m) => m.encoded_size(version),
            Self::PushTelemetry(m) => m.encoded_size(version),
            Self::AssignReplicasToDirs(m) => m.encoded_size(version),
            Self::ListConfigResources(m) => m.encoded_size(version),
            Self::DescribeTopicPartitions(m) => m.encoded_size(version),
            Self::ShareGroupHeartbeat(m) => m.encoded_size(version),
            Self::ShareGroupDescribe(m) => m.encoded_size(version),
            Self::ShareFetch(m) => m.encoded_size(version),
            Self::ShareAcknowledge(m) => m.encoded_size(version),
            Self::AddRaftVoter(m) => m.encoded_size(version),
            Self::RemoveRaftVoter(m) => m.encoded_size(version),
            Self::UpdateRaftVoter(m) => m.encoded_size(version),
            Self::InitializeShareGroupState(m) => m.encoded_size(version),
            Self::ReadShareGroupState(m) => m.encoded_size(version),
            Self::WriteShareGroupState(m) => m.encoded_size(version),
            Self::DeleteShareGroupState(m) => m.encoded_size(version),
            Self::ReadShareGroupStateSummary(m) => m.encoded_size(version),
            Self::StreamsGroupHeartbeat(m) => m.encoded_size(version),
            Self::StreamsGroupDescribe(m) => m.encoded_size(version),
            Self::DescribeShareGroupOffsets(m) => m.encoded_size(version),
            Self::AlterShareGroupOffsets(m) => m.encoded_size(version),
            Self::DeleteShareGroupOffsets(m) => m.encoded_size(version),
        }
    }

    /// Encode the contained message at `version`.
    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        match self {
            Self::Produce(m) => m.encode(version, buf),
            Self::Fetch(m) => m.encode(version, buf),
            Self::ListOffsets(m) => m.encode(version, buf),
            Self::Metadata(m) => m.encode(version, buf),
            Self::LeaderAndIsr(m) => m.encode(version, buf),
            Self::StopReplica(m) => m.encode(version, buf),
            Self::UpdateMetadata(m) => m.encode(version, buf),
            Self::ControlledShutdown(m) => m.encode(version, buf),
            Self::OffsetCommit(m) => m.encode(version, buf),
            Self::OffsetFetch(m) => m.encode(version, buf),
            Self::FindCoordinator(m) => m.encode(version, buf),
            Self::JoinGroup(m) => m.encode(version, buf),
            Self::Heartbeat(m) => m.encode(version, buf),
            Self::LeaveGroup(m) => m.encode(version, buf),
            Self::SyncGroup(m) => m.encode(version, buf),
            Self::DescribeGroups(m) => m.encode(version, buf),
            Self::ListGroups(m) => m.encode(version, buf),
            Self::SaslHandshake(m) => m.encode(version, buf),
            Self::ApiVersions(m) => m.encode(version, buf),
            Self::CreateTopics(m) => m.encode(version, buf),
            Self::DeleteTopics(m) => m.encode(version, buf),
            Self::DeleteRecords(m) => m.encode(version, buf),
            Self::InitProducerId(m) => m.encode(version, buf),
            Self::OffsetForLeaderEpoch(m) => m.encode(version, buf),
            Self::AddPartitionsToTxn(m) => m.encode(version, buf),
            Self::AddOffsetsToTxn(m) => m.encode(version, buf),
            Self::EndTxn(m) => m.encode(version, buf),
            Self::WriteTxnMarkers(m) => m.encode(version, buf),
            Self::TxnOffsetCommit(m) => m.encode(version, buf),
            Self::DescribeAcls(m) => m.encode(version, buf),
            Self::CreateAcls(m) => m.encode(version, buf),
            Self::DeleteAcls(m) => m.encode(version, buf),
            Self::DescribeConfigs(m) => m.encode(version, buf),
            Self::AlterConfigs(m) => m.encode(version, buf),
            Self::AlterReplicaLogDirs(m) => m.encode(version, buf),
            Self::DescribeLogDirs(m) => m.encode(version, buf),
            Self::SaslAuthenticate(m) => m.encode(version, buf),
            Self::CreatePartitions(m) => m.encode(version, buf),
            Self::CreateDelegationToken(m) => m.encode(version, buf),
            Self::RenewDelegationToken(m) => m.encode(version, buf),
            Self::ExpireDelegationToken(m) => m.encode(version, buf),
            Self::DescribeDelegationToken(m) => m.encode(version, buf),
            Self::DeleteGroups(m) => m.encode(version, buf),
            Self::ElectLeaders(m) => m.encode(version, buf),
            Self::IncrementalAlterConfigs(m) => m.encode(version, buf),
            Self::AlterPartitionReassignments(m) => m.encode(version, buf),
            Self::ListPartitionReassignments(m) => m.encode(version, buf),
            Self::OffsetDelete(m) => m.encode(version, buf),
            Self::DescribeClientQuotas(m) => m.encode(version, buf),
            Self::AlterClientQuotas(m) => m.encode(version, buf),
            Self::DescribeUserScramCredentials(m) => m.encode(version, buf),
            Self::AlterUserScramCredentials(m) => m.encode(version, buf),
            Self::Vote(m) => m.encode(version, buf),
            Self::BeginQuorumEpoch(m) => m.encode(version, buf),
            Self::EndQuorumEpoch(m) => m.encode(version, buf),
            Self::DescribeQuorum(m) => m.encode(version, buf),
            Self::AlterPartition(m) => m.encode(version, buf),
            Self::UpdateFeatures(m) => m.encode(version, buf),
            Self::Envelope(m) => m.encode(version, buf),
            Self::FetchSnapshot(m) => m.encode(version, buf),
            Self::DescribeCluster(m) => m.encode(version, buf),
            Self::DescribeProducers(m) => m.encode(version, buf),
            Self::BrokerRegistration(m) => m.encode(version, buf),
            Self::BrokerHeartbeat(m) => m.encode(version, buf),
            Self::UnregisterBroker(m) => m.encode(version, buf),
            Self::DescribeTransactions(m) => m.encode(version, buf),
            Self::ListTransactions(m) => m.encode(version, buf),
            Self::AllocateProducerIds(m) => m.encode(version, buf),
            Self::ConsumerGroupHeartbeat(m) => m.encode(version, buf),
            Self::ConsumerGroupDescribe(m) => m.encode(version, buf),
            Self::ControllerRegistration(m) => m.encode(version, buf),
            Self::GetTelemetrySubscriptions(m) => m.encode(version, buf),
            Self::PushTelemetry(m) => m.encode(version, buf),
            Self::AssignReplicasToDirs(m) => m.encode(version, buf),
            Self::ListConfigResources(m) => m.encode(version, buf),
            Self::DescribeTopicPartitions(m) => m.encode(version, buf),
            Self::ShareGroupHeartbeat(m) => m.encode(version, buf),
            Self::ShareGroupDescribe(m) => m.encode(version, buf),
            Self::ShareFetch(m) => m.encode(version, buf),
            Self::ShareAcknowledge(m) => m.encode(version, buf),
            Self::AddRaftVoter(m) => m.encode(version, buf),
            Self::RemoveRaftVoter(m) => m.encode(version, buf),
            Self::UpdateRaftVoter(m) => m.encode(version, buf),
            Self::InitializeShareGroupState(m) => m.encode(version, buf),
            Self::ReadShareGroupState(m) => m.encode(version, buf),
            Self::WriteShareGroupState(m) => m.encode(version, buf),
            Self::DeleteShareGroupState(m) => m.encode(version, buf),
            Self::ReadShareGroupStateSummary(m) => m.encode(version, buf),
            Self::StreamsGroupHeartbeat(m) => m.encode(version, buf),
            Self::StreamsGroupDescribe(m) => m.encode(version, buf),
            Self::DescribeShareGroupOffsets(m) => m.encode(version, buf),
            Self::AlterShareGroupOffsets(m) => m.encode(version, buf),
            Self::DeleteShareGroupOffsets(m) => m.encode(version, buf),
        }
    }

    /// Size-first encode into a freshly allocated, exact-capacity buffer.
    pub fn to_bytes(&self, version: i16) -> BytesMut {
        let mut buf = BytesMut::with_capacity(self.encoded_size(version));
        self.encode(version, &mut buf);
        buf
    }
}

impl From<super::produce_request::ProduceRequest> for RequestKind {
    fn from(m: super::produce_request::ProduceRequest) -> Self {
        Self::Produce(m)
    }
}

impl From<super::fetch_request::FetchRequest> for RequestKind {
    fn from(m: super::fetch_request::FetchRequest) -> Self {
        Self::Fetch(m)
    }
}

impl From<super::list_offsets_request::ListOffsetsRequest> for RequestKind {
    fn from(m: super::list_offsets_request::ListOffsetsRequest) -> Self {
        Self::ListOffsets(m)
    }
}

impl From<super::metadata_request::MetadataRequest> for RequestKind {
    fn from(m: super::metadata_request::MetadataRequest) -> Self {
        Self::Metadata(m)
    }
}

impl From<super::leader_and_isr_request::LeaderAndIsrRequest> for RequestKind {
    fn from(m: super::leader_and_isr_request::LeaderAndIsrRequest) -> Self {
        Self::LeaderAndIsr(m)
    }
}

impl From<super::stop_replica_request::StopReplicaRequest> for RequestKind {
    fn from(m: super::stop_replica_request::StopReplicaRequest) -> Self {
        Self::StopReplica(m)
    }
}

impl From<super::update_metadata_request::UpdateMetadataRequest> for RequestKind {
    fn from(m: super::update_metadata_request::UpdateMetadataRequest) -> Self {
        Self::UpdateMetadata(m)
    }
}

impl From<super::controlled_shutdown_request::ControlledShutdownRequest> for RequestKind {
    fn from(m: super::controlled_shutdown_request::ControlledShutdownRequest) -> Self {
        Self::ControlledShutdown(m)
    }
}

impl From<super::offset_commit_request::OffsetCommitRequest> for RequestKind {
    fn from(m: super::offset_commit_request::OffsetCommitRequest) -> Self {
        Self::OffsetCommit(m)
    }
}

impl From<super::offset_fetch_request::OffsetFetchRequest> for RequestKind {
    fn from(m: super::offset_fetch_request::OffsetFetchRequest) -> Self {
        Self::OffsetFetch(m)
    }
}

impl From<super::find_coordinator_request::FindCoordinatorRequest> for RequestKind {
    fn from(m: super::find_coordinator_request::FindCoordinatorRequest) -> Self {
        Self::FindCoordinator(m)
    }
}

impl From<super::join_group_request::JoinGroupRequest> for RequestKind {
    fn from(m: super::join_group_request::JoinGroupRequest) -> Self {
        Self::JoinGroup(m)
    }
}

impl From<super::heartbeat_request::HeartbeatRequest> for RequestKind {
    fn from(m: super::heartbeat_request::HeartbeatRequest) -> Self {
        Self::Heartbeat(m)
    }
}

impl From<super::leave_group_request::LeaveGroupRequest> for RequestKind {
    fn from(m: super::leave_group_request::LeaveGroupRequest) -> Self {
        Self::LeaveGroup(m)
    }
}

impl From<super::sync_group_request::SyncGroupRequest> for RequestKind {
    fn from(m: super::sync_group_request::SyncGroupRequest) -> Self {
        Self::SyncGroup(m)
    }
}

impl From<super::describe_groups_request::DescribeGroupsRequest> for RequestKind {
    fn from(m: super::describe_groups_request::DescribeGroupsRequest) -> Self {
        Self::DescribeGroups(m)
    }
}

impl From<super::list_groups_request::ListGroupsRequest> for RequestKind {
    fn from(m: super::list_groups_request::ListGroupsRequest) -> Self {
        Self::ListGroups(m)
    }
}

impl From<super::sasl_handshake_request::SaslHandshakeRequest> for RequestKind {
    fn from(m: super::sasl_handshake_request::SaslHandshakeRequest) -> Self {
        Self::SaslHandshake(m)
    }
}

impl From<super::api_versions_request::ApiVersionsRequest> for RequestKind {
    fn from(m: super::api_versions_request::ApiVersionsRequest) -> Self {
        Self::ApiVersions(m)
    }
}

impl From<super::create_topics_request::CreateTopicsRequest> for RequestKind {
    fn from(m: super::create_topics_request::CreateTopicsRequest) -> Self {
        Self::CreateTopics(m)
    }
}

impl From<super::delete_topics_request::DeleteTopicsRequest> for RequestKind {
    fn from(m: super::delete_topics_request::DeleteTopicsRequest) -> Self {
        Self::DeleteTopics(m)
    }
}

impl From<super::delete_records_request::DeleteRecordsRequest> for RequestKind {
    fn from(m: super::delete_records_request::DeleteRecordsRequest) -> Self {
        Self::DeleteRecords(m)
    }
}

impl From<super::init_producer_id_request::InitProducerIdRequest> for RequestKind {
    fn from(m: super::init_producer_id_request::InitProducerIdRequest) -> Self {
        Self::InitProducerId(m)
    }
}

impl From<super::offset_for_leader_epoch_request::OffsetForLeaderEpochRequest> for RequestKind {
    fn from(m: super::offset_for_leader_epoch_request::OffsetForLeaderEpochRequest) -> Self {
        Self::OffsetForLeaderEpoch(m)
    }
}

impl From<super::add_partitions_to_txn_request::AddPartitionsToTxnRequest> for RequestKind {
    fn from(m: super::add_partitions_to_txn_request::AddPartitionsToTxnRequest) -> Self {
        Self::AddPartitionsToTxn(m)
    }
}

impl From<super::add_offsets_to_txn_request::AddOffsetsToTxnRequest> for RequestKind {
    fn from(m: super::add_offsets_to_txn_request::AddOffsetsToTxnRequest) -> Self {
        Self::AddOffsetsToTxn(m)
    }
}

impl From<super::end_txn_request::EndTxnRequest> for RequestKind {
    fn from(m: super::end_txn_request::EndTxnRequest) -> Self {
        Self::EndTxn(m)
    }
}

impl From<super::write_txn_markers_request::WriteTxnMarkersRequest> for RequestKind {
    fn from(m: super::write_txn_markers_request::WriteTxnMarkersRequest) -> Self {
        Self::WriteTxnMarkers(m)
    }
}

impl From<super::txn_offset_commit_request::TxnOffsetCommitRequest> for RequestKind {
    fn from(m: super::txn_offset_commit_request::TxnOffsetCommitRequest) -> Self {
        Self::TxnOffsetCommit(m)
    }
}

impl From<super::describe_acls_request::DescribeAclsRequest> for RequestKind {
    fn from(m: super::describe_acls_request::DescribeAclsRequest) -> Self {
        Self::DescribeAcls(m)
    }
}

impl From<super::create_acls_request::CreateAclsRequest> for RequestKind {
    fn from(m: super::create_acls_request::CreateAclsRequest) -> Self {
        Self::CreateAcls(m)
    }
}

impl From<super::delete_acls_request::DeleteAclsRequest> for RequestKind {
    fn from(m: super::delete_acls_request::DeleteAclsRequest) -> Self {
        Self::DeleteAcls(m)
    }
}

impl From<super::describe_configs_request::DescribeConfigsRequest> for RequestKind {
    fn from(m: super::describe_configs_request::DescribeConfigsRequest) -> Self {
        Self::DescribeConfigs(m)
    }
}

impl From<super::alter_configs_request::AlterConfigsRequest> for RequestKind {
    fn from(m: super::alter_configs_request::AlterConfigsRequest) -> Self {
        Self::AlterConfigs(m)
    }
}

impl From<super::alter_replica_log_dirs_request::AlterReplicaLogDirsRequest> for RequestKind {
    fn from(m: super::alter_replica_log_dirs_request::AlterReplicaLogDirsRequest) -> Self {
        Self::AlterReplicaLogDirs(m)
    }
}

impl From<super::describe_log_dirs_request::DescribeLogDirsRequest> for RequestKind {
    fn from(m: super::describe_log_dirs_request::DescribeLogDirsRequest) -> Self {
        Self::DescribeLogDirs(m)
    }
}

impl From<super::sasl_authenticate_request::SaslAuthenticateRequest> for RequestKind {
    fn from(m: super::sasl_authenticate_request::SaslAuthenticateRequest) -> Self {
        Self::SaslAuthenticate(m)
    }
}

impl From<super::create_partitions_request::CreatePartitionsRequest> for RequestKind {
    fn from(m: super::create_partitions_request::CreatePartitionsRequest) -> Self {
        Self::CreatePartitions(m)
    }
}

impl From<super::create_delegation_token_request::CreateDelegationTokenRequest> for RequestKind {
    fn from(m: super::create_delegation_token_request::CreateDelegationTokenRequest) -> Self {
        Self::CreateDelegationToken(m)
    }
}

impl From<super::renew_delegation_token_request::RenewDelegationTokenRequest> for RequestKind {
    fn from(m: super::renew_delegation_token_request::RenewDelegationTokenRequest) -> Self {
        Self::RenewDelegationToken(m)
    }
}

impl From<super::expire_delegation_token_request::ExpireDelegationTokenRequest> for RequestKind {
    fn from(m: super::expire_delegation_token_request::ExpireDelegationTokenRequest) -> Self {
        Self::ExpireDelegationToken(m)
    }
}

impl From<super::describe_delegation_token_request::DescribeDelegationTokenRequest> for RequestKind {
    fn from(m: super::describe_delegation_token_request::DescribeDelegationTokenRequest) -> Self {
        Self::DescribeDelegationToken(m)
    }
}

impl From<super::delete_groups_request::DeleteGroupsRequest> for RequestKind {
    fn from(m: super::delete_groups_request::DeleteGroupsRequest) -> Self {
        Self::DeleteGroups(m)
    }
}

impl From<super::elect_leaders_request::ElectLeadersRequest> for RequestKind {
    fn from(m: super::elect_leaders_request::ElectLeadersRequest) -> Self {
        Self::ElectLeaders(m)
    }
}

impl From<super::incremental_alter_configs_request::IncrementalAlterConfigsRequest> for RequestKind {
    fn from(m: super::incremental_alter_configs_request::IncrementalAlterConfigsRequest) -> Self {
        Self::IncrementalAlterConfigs(m)
    }
}

impl From<super::alter_partition_reassignments_request::AlterPartitionReassignmentsRequest> for RequestKind {
    fn from(m: super::alter_partition_reassignments_request::AlterPartitionReassignmentsRequest) -> Self {
        Self::AlterPartitionReassignments(m)
    }
}

impl From<super::list_partition_reassignments_request::ListPartitionReassignmentsRequest> for RequestKind {
    fn from(m: super::list_partition_reassignments_request::ListPartitionReassignmentsRequest) -> Self {
        Self::ListPartitionReassignments(m)
    }
}

impl From<super::offset_delete_request::OffsetDeleteRequest> for RequestKind {
    fn from(m: super::offset_delete_request::OffsetDeleteRequest) -> Self {
        Self::OffsetDelete(m)
    }
}

impl From<super::describe_client_quotas_request::DescribeClientQuotasRequest> for RequestKind {
    fn from(m: super::describe_client_quotas_request::DescribeClientQuotasRequest) -> Self {
        Self::DescribeClientQuotas(m)
    }
}

impl From<super::alter_client_quotas_request::AlterClientQuotasRequest> for RequestKind {
    fn from(m: super::alter_client_quotas_request::AlterClientQuotasRequest) -> Self {
        Self::AlterClientQuotas(m)
    }
}

impl From<super::describe_user_scram_credentials_request::DescribeUserScramCredentialsRequest> for RequestKind {
    fn from(m: super::describe_user_scram_credentials_request::DescribeUserScramCredentialsRequest) -> Self {
        Self::DescribeUserScramCredentials(m)
    }
}

impl From<super::alter_user_scram_credentials_request::AlterUserScramCredentialsRequest> for RequestKind {
    fn from(m: super::alter_user_scram_credentials_request::AlterUserScramCredentialsRequest) -> Self {
        Self::AlterUserScramCredentials(m)
    }
}

impl From<super::vote_request::VoteRequest> for RequestKind {
    fn from(m: super::vote_request::VoteRequest) -> Self {
        Self::Vote(m)
    }
}

impl From<super::begin_quorum_epoch_request::BeginQuorumEpochRequest> for RequestKind {
    fn from(m: super::begin_quorum_epoch_request::BeginQuorumEpochRequest) -> Self {
        Self::BeginQuorumEpoch(m)
    }
}

impl From<super::end_quorum_epoch_request::EndQuorumEpochRequest> for RequestKind {
    fn from(m: super::end_quorum_epoch_request::EndQuorumEpochRequest) -> Self {
        Self::EndQuorumEpoch(m)
    }
}

impl From<super::describe_quorum_request::DescribeQuorumRequest> for RequestKind {
    fn from(m: super::describe_quorum_request::DescribeQuorumRequest) -> Self {
        Self::DescribeQuorum(m)
    }
}

impl From<super::alter_partition_request::AlterPartitionRequest> for RequestKind {
    fn from(m: super::alter_partition_request::AlterPartitionRequest) -> Self {
        Self::AlterPartition(m)
    }
}

impl From<super::update_features_request::UpdateFeaturesRequest> for RequestKind {
    fn from(m: super::update_features_request::UpdateFeaturesRequest) -> Self {
        Self::UpdateFeatures(m)
    }
}

impl From<super::envelope_request::EnvelopeRequest> for RequestKind {
    fn from(m: super::envelope_request::EnvelopeRequest) -> Self {
        Self::Envelope(m)
    }
}

impl From<super::fetch_snapshot_request::FetchSnapshotRequest> for RequestKind {
    fn from(m: super::fetch_snapshot_request::FetchSnapshotRequest) -> Self {
        Self::FetchSnapshot(m)
    }
}

impl From<super::describe_cluster_request::DescribeClusterRequest> for RequestKind {
    fn from(m: super::describe_cluster_request::DescribeClusterRequest) -> Self {
        Self::DescribeCluster(m)
    }
}

impl From<super::describe_producers_request::DescribeProducersRequest> for RequestKind {
    fn from(m: super::describe_producers_request::DescribeProducersRequest) -> Self {
        Self::DescribeProducers(m)
    }
}

impl From<super::broker_registration_request::BrokerRegistrationRequest> for RequestKind {
    fn from(m: super::broker_registration_request::BrokerRegistrationRequest) -> Self {
        Self::BrokerRegistration(m)
    }
}

impl From<super::broker_heartbeat_request::BrokerHeartbeatRequest> for RequestKind {
    fn from(m: super::broker_heartbeat_request::BrokerHeartbeatRequest) -> Self {
        Self::BrokerHeartbeat(m)
    }
}

impl From<super::unregister_broker_request::UnregisterBrokerRequest> for RequestKind {
    fn from(m: super::unregister_broker_request::UnregisterBrokerRequest) -> Self {
        Self::UnregisterBroker(m)
    }
}

impl From<super::describe_transactions_request::DescribeTransactionsRequest> for RequestKind {
    fn from(m: super::describe_transactions_request::DescribeTransactionsRequest) -> Self {
        Self::DescribeTransactions(m)
    }
}

impl From<super::list_transactions_request::ListTransactionsRequest> for RequestKind {
    fn from(m: super::list_transactions_request::ListTransactionsRequest) -> Self {
        Self::ListTransactions(m)
    }
}

impl From<super::allocate_producer_ids_request::AllocateProducerIdsRequest> for RequestKind {
    fn from(m: super::allocate_producer_ids_request::AllocateProducerIdsRequest) -> Self {
        Self::AllocateProducerIds(m)
    }
}

impl From<super::consumer_group_heartbeat_request::ConsumerGroupHeartbeatRequest> for RequestKind {
    fn from(m: super::consumer_group_heartbeat_request::ConsumerGroupHeartbeatRequest) -> Self {
        Self::ConsumerGroupHeartbeat(m)
    }
}

impl From<super::consumer_group_describe_request::ConsumerGroupDescribeRequest> for RequestKind {
    fn from(m: super::consumer_group_describe_request::ConsumerGroupDescribeRequest) -> Self {
        Self::ConsumerGroupDescribe(m)
    }
}

impl From<super::controller_registration_request::ControllerRegistrationRequest> for RequestKind {
    fn from(m: super::controller_registration_request::ControllerRegistrationRequest) -> Self {
        Self::ControllerRegistration(m)
    }
}

impl From<super::get_telemetry_subscriptions_request::GetTelemetrySubscriptionsRequest> for RequestKind {
    fn from(m: super::get_telemetry_subscriptions_request::GetTelemetrySubscriptionsRequest) -> Self {
        Self::GetTelemetrySubscriptions(m)
    }
}

impl From<super::push_telemetry_request::PushTelemetryRequest> for RequestKind {
    fn from(m: super::push_telemetry_request::PushTelemetryRequest) -> Self {
        Self::PushTelemetry(m)
    }
}

impl From<super::assign_replicas_to_dirs_request::AssignReplicasToDirsRequest> for RequestKind {
    fn from(m: super::assign_replicas_to_dirs_request::AssignReplicasToDirsRequest) -> Self {
        Self::AssignReplicasToDirs(m)
    }
}

impl From<super::list_config_resources_request::ListConfigResourcesRequest> for RequestKind {
    fn from(m: super::list_config_resources_request::ListConfigResourcesRequest) -> Self {
        Self::ListConfigResources(m)
    }
}

impl From<super::describe_topic_partitions_request::DescribeTopicPartitionsRequest> for RequestKind {
    fn from(m: super::describe_topic_partitions_request::DescribeTopicPartitionsRequest) -> Self {
        Self::DescribeTopicPartitions(m)
    }
}

impl From<super::share_group_heartbeat_request::ShareGroupHeartbeatRequest> for RequestKind {
    fn from(m: super::share_group_heartbeat_request::ShareGroupHeartbeatRequest) -> Self {
        Self::ShareGroupHeartbeat(m)
    }
}

impl From<super::share_group_describe_request::ShareGroupDescribeRequest> for RequestKind {
    fn from(m: super::share_group_describe_request::ShareGroupDescribeRequest) -> Self {
        Self::ShareGroupDescribe(m)
    }
}

impl From<super::share_fetch_request::ShareFetchRequest> for RequestKind {
    fn from(m: super::share_fetch_request::ShareFetchRequest) -> Self {
        Self::ShareFetch(m)
    }
}

impl From<super::share_acknowledge_request::ShareAcknowledgeRequest> for RequestKind {
    fn from(m: super::share_acknowledge_request::ShareAcknowledgeRequest) -> Self {
        Self::ShareAcknowledge(m)
    }
}

impl From<super::add_raft_voter_request::AddRaftVoterRequest> for RequestKind {
    fn from(m: super::add_raft_voter_request::AddRaftVoterRequest) -> Self {
        Self::AddRaftVoter(m)
    }
}

impl From<super::remove_raft_voter_request::RemoveRaftVoterRequest> for RequestKind {
    fn from(m: super::remove_raft_voter_request::RemoveRaftVoterRequest) -> Self {
        Self::RemoveRaftVoter(m)
    }
}

impl From<super::update_raft_voter_request::UpdateRaftVoterRequest> for RequestKind {
    fn from(m: super::update_raft_voter_request::UpdateRaftVoterRequest) -> Self {
        Self::UpdateRaftVoter(m)
    }
}

impl From<super::initialize_share_group_state_request::InitializeShareGroupStateRequest> for RequestKind {
    fn from(m: super::initialize_share_group_state_request::InitializeShareGroupStateRequest) -> Self {
        Self::InitializeShareGroupState(m)
    }
}

impl From<super::read_share_group_state_request::ReadShareGroupStateRequest> for RequestKind {
    fn from(m: super::read_share_group_state_request::ReadShareGroupStateRequest) -> Self {
        Self::ReadShareGroupState(m)
    }
}

impl From<super::write_share_group_state_request::WriteShareGroupStateRequest> for RequestKind {
    fn from(m: super::write_share_group_state_request::WriteShareGroupStateRequest) -> Self {
        Self::WriteShareGroupState(m)
    }
}

impl From<super::delete_share_group_state_request::DeleteShareGroupStateRequest> for RequestKind {
    fn from(m: super::delete_share_group_state_request::DeleteShareGroupStateRequest) -> Self {
        Self::DeleteShareGroupState(m)
    }
}

impl From<super::read_share_group_state_summary_request::ReadShareGroupStateSummaryRequest> for RequestKind {
    fn from(m: super::read_share_group_state_summary_request::ReadShareGroupStateSummaryRequest) -> Self {
        Self::ReadShareGroupStateSummary(m)
    }
}

impl From<super::streams_group_heartbeat_request::StreamsGroupHeartbeatRequest> for RequestKind {
    fn from(m: super::streams_group_heartbeat_request::StreamsGroupHeartbeatRequest) -> Self {
        Self::StreamsGroupHeartbeat(m)
    }
}

impl From<super::streams_group_describe_request::StreamsGroupDescribeRequest> for RequestKind {
    fn from(m: super::streams_group_describe_request::StreamsGroupDescribeRequest) -> Self {
        Self::StreamsGroupDescribe(m)
    }
}

impl From<super::describe_share_group_offsets_request::DescribeShareGroupOffsetsRequest> for RequestKind {
    fn from(m: super::describe_share_group_offsets_request::DescribeShareGroupOffsetsRequest) -> Self {
        Self::DescribeShareGroupOffsets(m)
    }
}

impl From<super::alter_share_group_offsets_request::AlterShareGroupOffsetsRequest> for RequestKind {
    fn from(m: super::alter_share_group_offsets_request::AlterShareGroupOffsetsRequest) -> Self {
        Self::AlterShareGroupOffsets(m)
    }
}

impl From<super::delete_share_group_offsets_request::DeleteShareGroupOffsetsRequest> for RequestKind {
    fn from(m: super::delete_share_group_offsets_request::DeleteShareGroupOffsetsRequest) -> Self {
        Self::DeleteShareGroupOffsets(m)
    }
}

/// Every Kafka response, as one typed enum: decode by api key, match by variant.
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseKind {
    Produce(super::produce_response::ProduceResponse),
    Fetch(super::fetch_response::FetchResponse),
    ListOffsets(super::list_offsets_response::ListOffsetsResponse),
    Metadata(super::metadata_response::MetadataResponse),
    LeaderAndIsr(super::leader_and_isr_response::LeaderAndIsrResponse),
    StopReplica(super::stop_replica_response::StopReplicaResponse),
    UpdateMetadata(super::update_metadata_response::UpdateMetadataResponse),
    ControlledShutdown(super::controlled_shutdown_response::ControlledShutdownResponse),
    OffsetCommit(super::offset_commit_response::OffsetCommitResponse),
    OffsetFetch(super::offset_fetch_response::OffsetFetchResponse),
    FindCoordinator(super::find_coordinator_response::FindCoordinatorResponse),
    JoinGroup(super::join_group_response::JoinGroupResponse),
    Heartbeat(super::heartbeat_response::HeartbeatResponse),
    LeaveGroup(super::leave_group_response::LeaveGroupResponse),
    SyncGroup(super::sync_group_response::SyncGroupResponse),
    DescribeGroups(super::describe_groups_response::DescribeGroupsResponse),
    ListGroups(super::list_groups_response::ListGroupsResponse),
    SaslHandshake(super::sasl_handshake_response::SaslHandshakeResponse),
    ApiVersions(super::api_versions_response::ApiVersionsResponse),
    CreateTopics(super::create_topics_response::CreateTopicsResponse),
    DeleteTopics(super::delete_topics_response::DeleteTopicsResponse),
    DeleteRecords(super::delete_records_response::DeleteRecordsResponse),
    InitProducerId(super::init_producer_id_response::InitProducerIdResponse),
    OffsetForLeaderEpoch(super::offset_for_leader_epoch_response::OffsetForLeaderEpochResponse),
    AddPartitionsToTxn(super::add_partitions_to_txn_response::AddPartitionsToTxnResponse),
    AddOffsetsToTxn(super::add_offsets_to_txn_response::AddOffsetsToTxnResponse),
    EndTxn(super::end_txn_response::EndTxnResponse),
    WriteTxnMarkers(super::write_txn_markers_response::WriteTxnMarkersResponse),
    TxnOffsetCommit(super::txn_offset_commit_response::TxnOffsetCommitResponse),
    DescribeAcls(super::describe_acls_response::DescribeAclsResponse),
    CreateAcls(super::create_acls_response::CreateAclsResponse),
    DeleteAcls(super::delete_acls_response::DeleteAclsResponse),
    DescribeConfigs(super::describe_configs_response::DescribeConfigsResponse),
    AlterConfigs(super::alter_configs_response::AlterConfigsResponse),
    AlterReplicaLogDirs(super::alter_replica_log_dirs_response::AlterReplicaLogDirsResponse),
    DescribeLogDirs(super::describe_log_dirs_response::DescribeLogDirsResponse),
    SaslAuthenticate(super::sasl_authenticate_response::SaslAuthenticateResponse),
    CreatePartitions(super::create_partitions_response::CreatePartitionsResponse),
    CreateDelegationToken(super::create_delegation_token_response::CreateDelegationTokenResponse),
    RenewDelegationToken(super::renew_delegation_token_response::RenewDelegationTokenResponse),
    ExpireDelegationToken(super::expire_delegation_token_response::ExpireDelegationTokenResponse),
    DescribeDelegationToken(super::describe_delegation_token_response::DescribeDelegationTokenResponse),
    DeleteGroups(super::delete_groups_response::DeleteGroupsResponse),
    ElectLeaders(super::elect_leaders_response::ElectLeadersResponse),
    IncrementalAlterConfigs(super::incremental_alter_configs_response::IncrementalAlterConfigsResponse),
    AlterPartitionReassignments(super::alter_partition_reassignments_response::AlterPartitionReassignmentsResponse),
    ListPartitionReassignments(super::list_partition_reassignments_response::ListPartitionReassignmentsResponse),
    OffsetDelete(super::offset_delete_response::OffsetDeleteResponse),
    DescribeClientQuotas(super::describe_client_quotas_response::DescribeClientQuotasResponse),
    AlterClientQuotas(super::alter_client_quotas_response::AlterClientQuotasResponse),
    DescribeUserScramCredentials(super::describe_user_scram_credentials_response::DescribeUserScramCredentialsResponse),
    AlterUserScramCredentials(super::alter_user_scram_credentials_response::AlterUserScramCredentialsResponse),
    Vote(super::vote_response::VoteResponse),
    BeginQuorumEpoch(super::begin_quorum_epoch_response::BeginQuorumEpochResponse),
    EndQuorumEpoch(super::end_quorum_epoch_response::EndQuorumEpochResponse),
    DescribeQuorum(super::describe_quorum_response::DescribeQuorumResponse),
    AlterPartition(super::alter_partition_response::AlterPartitionResponse),
    UpdateFeatures(super::update_features_response::UpdateFeaturesResponse),
    Envelope(super::envelope_response::EnvelopeResponse),
    FetchSnapshot(super::fetch_snapshot_response::FetchSnapshotResponse),
    DescribeCluster(super::describe_cluster_response::DescribeClusterResponse),
    DescribeProducers(super::describe_producers_response::DescribeProducersResponse),
    BrokerRegistration(super::broker_registration_response::BrokerRegistrationResponse),
    BrokerHeartbeat(super::broker_heartbeat_response::BrokerHeartbeatResponse),
    UnregisterBroker(super::unregister_broker_response::UnregisterBrokerResponse),
    DescribeTransactions(super::describe_transactions_response::DescribeTransactionsResponse),
    ListTransactions(super::list_transactions_response::ListTransactionsResponse),
    AllocateProducerIds(super::allocate_producer_ids_response::AllocateProducerIdsResponse),
    ConsumerGroupHeartbeat(super::consumer_group_heartbeat_response::ConsumerGroupHeartbeatResponse),
    ConsumerGroupDescribe(super::consumer_group_describe_response::ConsumerGroupDescribeResponse),
    ControllerRegistration(super::controller_registration_response::ControllerRegistrationResponse),
    GetTelemetrySubscriptions(super::get_telemetry_subscriptions_response::GetTelemetrySubscriptionsResponse),
    PushTelemetry(super::push_telemetry_response::PushTelemetryResponse),
    AssignReplicasToDirs(super::assign_replicas_to_dirs_response::AssignReplicasToDirsResponse),
    ListConfigResources(super::list_config_resources_response::ListConfigResourcesResponse),
    DescribeTopicPartitions(super::describe_topic_partitions_response::DescribeTopicPartitionsResponse),
    ShareGroupHeartbeat(super::share_group_heartbeat_response::ShareGroupHeartbeatResponse),
    ShareGroupDescribe(super::share_group_describe_response::ShareGroupDescribeResponse),
    ShareFetch(super::share_fetch_response::ShareFetchResponse),
    ShareAcknowledge(super::share_acknowledge_response::ShareAcknowledgeResponse),
    AddRaftVoter(super::add_raft_voter_response::AddRaftVoterResponse),
    RemoveRaftVoter(super::remove_raft_voter_response::RemoveRaftVoterResponse),
    UpdateRaftVoter(super::update_raft_voter_response::UpdateRaftVoterResponse),
    InitializeShareGroupState(super::initialize_share_group_state_response::InitializeShareGroupStateResponse),
    ReadShareGroupState(super::read_share_group_state_response::ReadShareGroupStateResponse),
    WriteShareGroupState(super::write_share_group_state_response::WriteShareGroupStateResponse),
    DeleteShareGroupState(super::delete_share_group_state_response::DeleteShareGroupStateResponse),
    ReadShareGroupStateSummary(super::read_share_group_state_summary_response::ReadShareGroupStateSummaryResponse),
    StreamsGroupHeartbeat(super::streams_group_heartbeat_response::StreamsGroupHeartbeatResponse),
    StreamsGroupDescribe(super::streams_group_describe_response::StreamsGroupDescribeResponse),
    DescribeShareGroupOffsets(super::describe_share_group_offsets_response::DescribeShareGroupOffsetsResponse),
    AlterShareGroupOffsets(super::alter_share_group_offsets_response::AlterShareGroupOffsetsResponse),
    DeleteShareGroupOffsets(super::delete_share_group_offsets_response::DeleteShareGroupOffsetsResponse),
}

impl ResponseKind {
    /// Decode the response body for `api_key` at `version`.
    /// Returns `DecodeError::UnknownApiKey` for api keys this build doesn't know.
    pub fn decode(api_key: i16, version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        match api_key {
            0 => Ok(Self::Produce(super::produce_response::ProduceResponse::decode(version, buf)?)),
            1 => Ok(Self::Fetch(super::fetch_response::FetchResponse::decode(version, buf)?)),
            2 => Ok(Self::ListOffsets(super::list_offsets_response::ListOffsetsResponse::decode(version, buf)?)),
            3 => Ok(Self::Metadata(super::metadata_response::MetadataResponse::decode(version, buf)?)),
            4 => Ok(Self::LeaderAndIsr(super::leader_and_isr_response::LeaderAndIsrResponse::decode(version, buf)?)),
            5 => Ok(Self::StopReplica(super::stop_replica_response::StopReplicaResponse::decode(version, buf)?)),
            6 => Ok(Self::UpdateMetadata(super::update_metadata_response::UpdateMetadataResponse::decode(version, buf)?)),
            7 => Ok(Self::ControlledShutdown(super::controlled_shutdown_response::ControlledShutdownResponse::decode(version, buf)?)),
            8 => Ok(Self::OffsetCommit(super::offset_commit_response::OffsetCommitResponse::decode(version, buf)?)),
            9 => Ok(Self::OffsetFetch(super::offset_fetch_response::OffsetFetchResponse::decode(version, buf)?)),
            10 => Ok(Self::FindCoordinator(super::find_coordinator_response::FindCoordinatorResponse::decode(version, buf)?)),
            11 => Ok(Self::JoinGroup(super::join_group_response::JoinGroupResponse::decode(version, buf)?)),
            12 => Ok(Self::Heartbeat(super::heartbeat_response::HeartbeatResponse::decode(version, buf)?)),
            13 => Ok(Self::LeaveGroup(super::leave_group_response::LeaveGroupResponse::decode(version, buf)?)),
            14 => Ok(Self::SyncGroup(super::sync_group_response::SyncGroupResponse::decode(version, buf)?)),
            15 => Ok(Self::DescribeGroups(super::describe_groups_response::DescribeGroupsResponse::decode(version, buf)?)),
            16 => Ok(Self::ListGroups(super::list_groups_response::ListGroupsResponse::decode(version, buf)?)),
            17 => Ok(Self::SaslHandshake(super::sasl_handshake_response::SaslHandshakeResponse::decode(version, buf)?)),
            18 => Ok(Self::ApiVersions(super::api_versions_response::ApiVersionsResponse::decode(version, buf)?)),
            19 => Ok(Self::CreateTopics(super::create_topics_response::CreateTopicsResponse::decode(version, buf)?)),
            20 => Ok(Self::DeleteTopics(super::delete_topics_response::DeleteTopicsResponse::decode(version, buf)?)),
            21 => Ok(Self::DeleteRecords(super::delete_records_response::DeleteRecordsResponse::decode(version, buf)?)),
            22 => Ok(Self::InitProducerId(super::init_producer_id_response::InitProducerIdResponse::decode(version, buf)?)),
            23 => Ok(Self::OffsetForLeaderEpoch(super::offset_for_leader_epoch_response::OffsetForLeaderEpochResponse::decode(version, buf)?)),
            24 => Ok(Self::AddPartitionsToTxn(super::add_partitions_to_txn_response::AddPartitionsToTxnResponse::decode(version, buf)?)),
            25 => Ok(Self::AddOffsetsToTxn(super::add_offsets_to_txn_response::AddOffsetsToTxnResponse::decode(version, buf)?)),
            26 => Ok(Self::EndTxn(super::end_txn_response::EndTxnResponse::decode(version, buf)?)),
            27 => Ok(Self::WriteTxnMarkers(super::write_txn_markers_response::WriteTxnMarkersResponse::decode(version, buf)?)),
            28 => Ok(Self::TxnOffsetCommit(super::txn_offset_commit_response::TxnOffsetCommitResponse::decode(version, buf)?)),
            29 => Ok(Self::DescribeAcls(super::describe_acls_response::DescribeAclsResponse::decode(version, buf)?)),
            30 => Ok(Self::CreateAcls(super::create_acls_response::CreateAclsResponse::decode(version, buf)?)),
            31 => Ok(Self::DeleteAcls(super::delete_acls_response::DeleteAclsResponse::decode(version, buf)?)),
            32 => Ok(Self::DescribeConfigs(super::describe_configs_response::DescribeConfigsResponse::decode(version, buf)?)),
            33 => Ok(Self::AlterConfigs(super::alter_configs_response::AlterConfigsResponse::decode(version, buf)?)),
            34 => Ok(Self::AlterReplicaLogDirs(super::alter_replica_log_dirs_response::AlterReplicaLogDirsResponse::decode(version, buf)?)),
            35 => Ok(Self::DescribeLogDirs(super::describe_log_dirs_response::DescribeLogDirsResponse::decode(version, buf)?)),
            36 => Ok(Self::SaslAuthenticate(super::sasl_authenticate_response::SaslAuthenticateResponse::decode(version, buf)?)),
            37 => Ok(Self::CreatePartitions(super::create_partitions_response::CreatePartitionsResponse::decode(version, buf)?)),
            38 => Ok(Self::CreateDelegationToken(super::create_delegation_token_response::CreateDelegationTokenResponse::decode(version, buf)?)),
            39 => Ok(Self::RenewDelegationToken(super::renew_delegation_token_response::RenewDelegationTokenResponse::decode(version, buf)?)),
            40 => Ok(Self::ExpireDelegationToken(super::expire_delegation_token_response::ExpireDelegationTokenResponse::decode(version, buf)?)),
            41 => Ok(Self::DescribeDelegationToken(super::describe_delegation_token_response::DescribeDelegationTokenResponse::decode(version, buf)?)),
            42 => Ok(Self::DeleteGroups(super::delete_groups_response::DeleteGroupsResponse::decode(version, buf)?)),
            43 => Ok(Self::ElectLeaders(super::elect_leaders_response::ElectLeadersResponse::decode(version, buf)?)),
            44 => Ok(Self::IncrementalAlterConfigs(super::incremental_alter_configs_response::IncrementalAlterConfigsResponse::decode(version, buf)?)),
            45 => Ok(Self::AlterPartitionReassignments(super::alter_partition_reassignments_response::AlterPartitionReassignmentsResponse::decode(version, buf)?)),
            46 => Ok(Self::ListPartitionReassignments(super::list_partition_reassignments_response::ListPartitionReassignmentsResponse::decode(version, buf)?)),
            47 => Ok(Self::OffsetDelete(super::offset_delete_response::OffsetDeleteResponse::decode(version, buf)?)),
            48 => Ok(Self::DescribeClientQuotas(super::describe_client_quotas_response::DescribeClientQuotasResponse::decode(version, buf)?)),
            49 => Ok(Self::AlterClientQuotas(super::alter_client_quotas_response::AlterClientQuotasResponse::decode(version, buf)?)),
            50 => Ok(Self::DescribeUserScramCredentials(super::describe_user_scram_credentials_response::DescribeUserScramCredentialsResponse::decode(version, buf)?)),
            51 => Ok(Self::AlterUserScramCredentials(super::alter_user_scram_credentials_response::AlterUserScramCredentialsResponse::decode(version, buf)?)),
            52 => Ok(Self::Vote(super::vote_response::VoteResponse::decode(version, buf)?)),
            53 => Ok(Self::BeginQuorumEpoch(super::begin_quorum_epoch_response::BeginQuorumEpochResponse::decode(version, buf)?)),
            54 => Ok(Self::EndQuorumEpoch(super::end_quorum_epoch_response::EndQuorumEpochResponse::decode(version, buf)?)),
            55 => Ok(Self::DescribeQuorum(super::describe_quorum_response::DescribeQuorumResponse::decode(version, buf)?)),
            56 => Ok(Self::AlterPartition(super::alter_partition_response::AlterPartitionResponse::decode(version, buf)?)),
            57 => Ok(Self::UpdateFeatures(super::update_features_response::UpdateFeaturesResponse::decode(version, buf)?)),
            58 => Ok(Self::Envelope(super::envelope_response::EnvelopeResponse::decode(version, buf)?)),
            59 => Ok(Self::FetchSnapshot(super::fetch_snapshot_response::FetchSnapshotResponse::decode(version, buf)?)),
            60 => Ok(Self::DescribeCluster(super::describe_cluster_response::DescribeClusterResponse::decode(version, buf)?)),
            61 => Ok(Self::DescribeProducers(super::describe_producers_response::DescribeProducersResponse::decode(version, buf)?)),
            62 => Ok(Self::BrokerRegistration(super::broker_registration_response::BrokerRegistrationResponse::decode(version, buf)?)),
            63 => Ok(Self::BrokerHeartbeat(super::broker_heartbeat_response::BrokerHeartbeatResponse::decode(version, buf)?)),
            64 => Ok(Self::UnregisterBroker(super::unregister_broker_response::UnregisterBrokerResponse::decode(version, buf)?)),
            65 => Ok(Self::DescribeTransactions(super::describe_transactions_response::DescribeTransactionsResponse::decode(version, buf)?)),
            66 => Ok(Self::ListTransactions(super::list_transactions_response::ListTransactionsResponse::decode(version, buf)?)),
            67 => Ok(Self::AllocateProducerIds(super::allocate_producer_ids_response::AllocateProducerIdsResponse::decode(version, buf)?)),
            68 => Ok(Self::ConsumerGroupHeartbeat(super::consumer_group_heartbeat_response::ConsumerGroupHeartbeatResponse::decode(version, buf)?)),
            69 => Ok(Self::ConsumerGroupDescribe(super::consumer_group_describe_response::ConsumerGroupDescribeResponse::decode(version, buf)?)),
            70 => Ok(Self::ControllerRegistration(super::controller_registration_response::ControllerRegistrationResponse::decode(version, buf)?)),
            71 => Ok(Self::GetTelemetrySubscriptions(super::get_telemetry_subscriptions_response::GetTelemetrySubscriptionsResponse::decode(version, buf)?)),
            72 => Ok(Self::PushTelemetry(super::push_telemetry_response::PushTelemetryResponse::decode(version, buf)?)),
            73 => Ok(Self::AssignReplicasToDirs(super::assign_replicas_to_dirs_response::AssignReplicasToDirsResponse::decode(version, buf)?)),
            74 => Ok(Self::ListConfigResources(super::list_config_resources_response::ListConfigResourcesResponse::decode(version, buf)?)),
            75 => Ok(Self::DescribeTopicPartitions(super::describe_topic_partitions_response::DescribeTopicPartitionsResponse::decode(version, buf)?)),
            76 => Ok(Self::ShareGroupHeartbeat(super::share_group_heartbeat_response::ShareGroupHeartbeatResponse::decode(version, buf)?)),
            77 => Ok(Self::ShareGroupDescribe(super::share_group_describe_response::ShareGroupDescribeResponse::decode(version, buf)?)),
            78 => Ok(Self::ShareFetch(super::share_fetch_response::ShareFetchResponse::decode(version, buf)?)),
            79 => Ok(Self::ShareAcknowledge(super::share_acknowledge_response::ShareAcknowledgeResponse::decode(version, buf)?)),
            80 => Ok(Self::AddRaftVoter(super::add_raft_voter_response::AddRaftVoterResponse::decode(version, buf)?)),
            81 => Ok(Self::RemoveRaftVoter(super::remove_raft_voter_response::RemoveRaftVoterResponse::decode(version, buf)?)),
            82 => Ok(Self::UpdateRaftVoter(super::update_raft_voter_response::UpdateRaftVoterResponse::decode(version, buf)?)),
            83 => Ok(Self::InitializeShareGroupState(super::initialize_share_group_state_response::InitializeShareGroupStateResponse::decode(version, buf)?)),
            84 => Ok(Self::ReadShareGroupState(super::read_share_group_state_response::ReadShareGroupStateResponse::decode(version, buf)?)),
            85 => Ok(Self::WriteShareGroupState(super::write_share_group_state_response::WriteShareGroupStateResponse::decode(version, buf)?)),
            86 => Ok(Self::DeleteShareGroupState(super::delete_share_group_state_response::DeleteShareGroupStateResponse::decode(version, buf)?)),
            87 => Ok(Self::ReadShareGroupStateSummary(super::read_share_group_state_summary_response::ReadShareGroupStateSummaryResponse::decode(version, buf)?)),
            88 => Ok(Self::StreamsGroupHeartbeat(super::streams_group_heartbeat_response::StreamsGroupHeartbeatResponse::decode(version, buf)?)),
            89 => Ok(Self::StreamsGroupDescribe(super::streams_group_describe_response::StreamsGroupDescribeResponse::decode(version, buf)?)),
            90 => Ok(Self::DescribeShareGroupOffsets(super::describe_share_group_offsets_response::DescribeShareGroupOffsetsResponse::decode(version, buf)?)),
            91 => Ok(Self::AlterShareGroupOffsets(super::alter_share_group_offsets_response::AlterShareGroupOffsetsResponse::decode(version, buf)?)),
            92 => Ok(Self::DeleteShareGroupOffsets(super::delete_share_group_offsets_response::DeleteShareGroupOffsetsResponse::decode(version, buf)?)),
            _ => Err(DecodeError::UnknownApiKey(api_key)),
        }
    }

    /// The Kafka API key of the contained message.
    pub fn api_key(&self) -> i16 {
        match self {
            Self::Produce(_) => 0,
            Self::Fetch(_) => 1,
            Self::ListOffsets(_) => 2,
            Self::Metadata(_) => 3,
            Self::LeaderAndIsr(_) => 4,
            Self::StopReplica(_) => 5,
            Self::UpdateMetadata(_) => 6,
            Self::ControlledShutdown(_) => 7,
            Self::OffsetCommit(_) => 8,
            Self::OffsetFetch(_) => 9,
            Self::FindCoordinator(_) => 10,
            Self::JoinGroup(_) => 11,
            Self::Heartbeat(_) => 12,
            Self::LeaveGroup(_) => 13,
            Self::SyncGroup(_) => 14,
            Self::DescribeGroups(_) => 15,
            Self::ListGroups(_) => 16,
            Self::SaslHandshake(_) => 17,
            Self::ApiVersions(_) => 18,
            Self::CreateTopics(_) => 19,
            Self::DeleteTopics(_) => 20,
            Self::DeleteRecords(_) => 21,
            Self::InitProducerId(_) => 22,
            Self::OffsetForLeaderEpoch(_) => 23,
            Self::AddPartitionsToTxn(_) => 24,
            Self::AddOffsetsToTxn(_) => 25,
            Self::EndTxn(_) => 26,
            Self::WriteTxnMarkers(_) => 27,
            Self::TxnOffsetCommit(_) => 28,
            Self::DescribeAcls(_) => 29,
            Self::CreateAcls(_) => 30,
            Self::DeleteAcls(_) => 31,
            Self::DescribeConfigs(_) => 32,
            Self::AlterConfigs(_) => 33,
            Self::AlterReplicaLogDirs(_) => 34,
            Self::DescribeLogDirs(_) => 35,
            Self::SaslAuthenticate(_) => 36,
            Self::CreatePartitions(_) => 37,
            Self::CreateDelegationToken(_) => 38,
            Self::RenewDelegationToken(_) => 39,
            Self::ExpireDelegationToken(_) => 40,
            Self::DescribeDelegationToken(_) => 41,
            Self::DeleteGroups(_) => 42,
            Self::ElectLeaders(_) => 43,
            Self::IncrementalAlterConfigs(_) => 44,
            Self::AlterPartitionReassignments(_) => 45,
            Self::ListPartitionReassignments(_) => 46,
            Self::OffsetDelete(_) => 47,
            Self::DescribeClientQuotas(_) => 48,
            Self::AlterClientQuotas(_) => 49,
            Self::DescribeUserScramCredentials(_) => 50,
            Self::AlterUserScramCredentials(_) => 51,
            Self::Vote(_) => 52,
            Self::BeginQuorumEpoch(_) => 53,
            Self::EndQuorumEpoch(_) => 54,
            Self::DescribeQuorum(_) => 55,
            Self::AlterPartition(_) => 56,
            Self::UpdateFeatures(_) => 57,
            Self::Envelope(_) => 58,
            Self::FetchSnapshot(_) => 59,
            Self::DescribeCluster(_) => 60,
            Self::DescribeProducers(_) => 61,
            Self::BrokerRegistration(_) => 62,
            Self::BrokerHeartbeat(_) => 63,
            Self::UnregisterBroker(_) => 64,
            Self::DescribeTransactions(_) => 65,
            Self::ListTransactions(_) => 66,
            Self::AllocateProducerIds(_) => 67,
            Self::ConsumerGroupHeartbeat(_) => 68,
            Self::ConsumerGroupDescribe(_) => 69,
            Self::ControllerRegistration(_) => 70,
            Self::GetTelemetrySubscriptions(_) => 71,
            Self::PushTelemetry(_) => 72,
            Self::AssignReplicasToDirs(_) => 73,
            Self::ListConfigResources(_) => 74,
            Self::DescribeTopicPartitions(_) => 75,
            Self::ShareGroupHeartbeat(_) => 76,
            Self::ShareGroupDescribe(_) => 77,
            Self::ShareFetch(_) => 78,
            Self::ShareAcknowledge(_) => 79,
            Self::AddRaftVoter(_) => 80,
            Self::RemoveRaftVoter(_) => 81,
            Self::UpdateRaftVoter(_) => 82,
            Self::InitializeShareGroupState(_) => 83,
            Self::ReadShareGroupState(_) => 84,
            Self::WriteShareGroupState(_) => 85,
            Self::DeleteShareGroupState(_) => 86,
            Self::ReadShareGroupStateSummary(_) => 87,
            Self::StreamsGroupHeartbeat(_) => 88,
            Self::StreamsGroupDescribe(_) => 89,
            Self::DescribeShareGroupOffsets(_) => 90,
            Self::AlterShareGroupOffsets(_) => 91,
            Self::DeleteShareGroupOffsets(_) => 92,
        }
    }

    /// The API name of the contained message (e.g. "Produce").
    pub fn name(&self) -> &'static str {
        match self {
            Self::Produce(_) => "Produce",
            Self::Fetch(_) => "Fetch",
            Self::ListOffsets(_) => "ListOffsets",
            Self::Metadata(_) => "Metadata",
            Self::LeaderAndIsr(_) => "LeaderAndIsr",
            Self::StopReplica(_) => "StopReplica",
            Self::UpdateMetadata(_) => "UpdateMetadata",
            Self::ControlledShutdown(_) => "ControlledShutdown",
            Self::OffsetCommit(_) => "OffsetCommit",
            Self::OffsetFetch(_) => "OffsetFetch",
            Self::FindCoordinator(_) => "FindCoordinator",
            Self::JoinGroup(_) => "JoinGroup",
            Self::Heartbeat(_) => "Heartbeat",
            Self::LeaveGroup(_) => "LeaveGroup",
            Self::SyncGroup(_) => "SyncGroup",
            Self::DescribeGroups(_) => "DescribeGroups",
            Self::ListGroups(_) => "ListGroups",
            Self::SaslHandshake(_) => "SaslHandshake",
            Self::ApiVersions(_) => "ApiVersions",
            Self::CreateTopics(_) => "CreateTopics",
            Self::DeleteTopics(_) => "DeleteTopics",
            Self::DeleteRecords(_) => "DeleteRecords",
            Self::InitProducerId(_) => "InitProducerId",
            Self::OffsetForLeaderEpoch(_) => "OffsetForLeaderEpoch",
            Self::AddPartitionsToTxn(_) => "AddPartitionsToTxn",
            Self::AddOffsetsToTxn(_) => "AddOffsetsToTxn",
            Self::EndTxn(_) => "EndTxn",
            Self::WriteTxnMarkers(_) => "WriteTxnMarkers",
            Self::TxnOffsetCommit(_) => "TxnOffsetCommit",
            Self::DescribeAcls(_) => "DescribeAcls",
            Self::CreateAcls(_) => "CreateAcls",
            Self::DeleteAcls(_) => "DeleteAcls",
            Self::DescribeConfigs(_) => "DescribeConfigs",
            Self::AlterConfigs(_) => "AlterConfigs",
            Self::AlterReplicaLogDirs(_) => "AlterReplicaLogDirs",
            Self::DescribeLogDirs(_) => "DescribeLogDirs",
            Self::SaslAuthenticate(_) => "SaslAuthenticate",
            Self::CreatePartitions(_) => "CreatePartitions",
            Self::CreateDelegationToken(_) => "CreateDelegationToken",
            Self::RenewDelegationToken(_) => "RenewDelegationToken",
            Self::ExpireDelegationToken(_) => "ExpireDelegationToken",
            Self::DescribeDelegationToken(_) => "DescribeDelegationToken",
            Self::DeleteGroups(_) => "DeleteGroups",
            Self::ElectLeaders(_) => "ElectLeaders",
            Self::IncrementalAlterConfigs(_) => "IncrementalAlterConfigs",
            Self::AlterPartitionReassignments(_) => "AlterPartitionReassignments",
            Self::ListPartitionReassignments(_) => "ListPartitionReassignments",
            Self::OffsetDelete(_) => "OffsetDelete",
            Self::DescribeClientQuotas(_) => "DescribeClientQuotas",
            Self::AlterClientQuotas(_) => "AlterClientQuotas",
            Self::DescribeUserScramCredentials(_) => "DescribeUserScramCredentials",
            Self::AlterUserScramCredentials(_) => "AlterUserScramCredentials",
            Self::Vote(_) => "Vote",
            Self::BeginQuorumEpoch(_) => "BeginQuorumEpoch",
            Self::EndQuorumEpoch(_) => "EndQuorumEpoch",
            Self::DescribeQuorum(_) => "DescribeQuorum",
            Self::AlterPartition(_) => "AlterPartition",
            Self::UpdateFeatures(_) => "UpdateFeatures",
            Self::Envelope(_) => "Envelope",
            Self::FetchSnapshot(_) => "FetchSnapshot",
            Self::DescribeCluster(_) => "DescribeCluster",
            Self::DescribeProducers(_) => "DescribeProducers",
            Self::BrokerRegistration(_) => "BrokerRegistration",
            Self::BrokerHeartbeat(_) => "BrokerHeartbeat",
            Self::UnregisterBroker(_) => "UnregisterBroker",
            Self::DescribeTransactions(_) => "DescribeTransactions",
            Self::ListTransactions(_) => "ListTransactions",
            Self::AllocateProducerIds(_) => "AllocateProducerIds",
            Self::ConsumerGroupHeartbeat(_) => "ConsumerGroupHeartbeat",
            Self::ConsumerGroupDescribe(_) => "ConsumerGroupDescribe",
            Self::ControllerRegistration(_) => "ControllerRegistration",
            Self::GetTelemetrySubscriptions(_) => "GetTelemetrySubscriptions",
            Self::PushTelemetry(_) => "PushTelemetry",
            Self::AssignReplicasToDirs(_) => "AssignReplicasToDirs",
            Self::ListConfigResources(_) => "ListConfigResources",
            Self::DescribeTopicPartitions(_) => "DescribeTopicPartitions",
            Self::ShareGroupHeartbeat(_) => "ShareGroupHeartbeat",
            Self::ShareGroupDescribe(_) => "ShareGroupDescribe",
            Self::ShareFetch(_) => "ShareFetch",
            Self::ShareAcknowledge(_) => "ShareAcknowledge",
            Self::AddRaftVoter(_) => "AddRaftVoter",
            Self::RemoveRaftVoter(_) => "RemoveRaftVoter",
            Self::UpdateRaftVoter(_) => "UpdateRaftVoter",
            Self::InitializeShareGroupState(_) => "InitializeShareGroupState",
            Self::ReadShareGroupState(_) => "ReadShareGroupState",
            Self::WriteShareGroupState(_) => "WriteShareGroupState",
            Self::DeleteShareGroupState(_) => "DeleteShareGroupState",
            Self::ReadShareGroupStateSummary(_) => "ReadShareGroupStateSummary",
            Self::StreamsGroupHeartbeat(_) => "StreamsGroupHeartbeat",
            Self::StreamsGroupDescribe(_) => "StreamsGroupDescribe",
            Self::DescribeShareGroupOffsets(_) => "DescribeShareGroupOffsets",
            Self::AlterShareGroupOffsets(_) => "AlterShareGroupOffsets",
            Self::DeleteShareGroupOffsets(_) => "DeleteShareGroupOffsets",
        }
    }

    /// Exact encoded size at `version` (size-first encoding).
    pub fn encoded_size(&self, version: i16) -> usize {
        match self {
            Self::Produce(m) => m.encoded_size(version),
            Self::Fetch(m) => m.encoded_size(version),
            Self::ListOffsets(m) => m.encoded_size(version),
            Self::Metadata(m) => m.encoded_size(version),
            Self::LeaderAndIsr(m) => m.encoded_size(version),
            Self::StopReplica(m) => m.encoded_size(version),
            Self::UpdateMetadata(m) => m.encoded_size(version),
            Self::ControlledShutdown(m) => m.encoded_size(version),
            Self::OffsetCommit(m) => m.encoded_size(version),
            Self::OffsetFetch(m) => m.encoded_size(version),
            Self::FindCoordinator(m) => m.encoded_size(version),
            Self::JoinGroup(m) => m.encoded_size(version),
            Self::Heartbeat(m) => m.encoded_size(version),
            Self::LeaveGroup(m) => m.encoded_size(version),
            Self::SyncGroup(m) => m.encoded_size(version),
            Self::DescribeGroups(m) => m.encoded_size(version),
            Self::ListGroups(m) => m.encoded_size(version),
            Self::SaslHandshake(m) => m.encoded_size(version),
            Self::ApiVersions(m) => m.encoded_size(version),
            Self::CreateTopics(m) => m.encoded_size(version),
            Self::DeleteTopics(m) => m.encoded_size(version),
            Self::DeleteRecords(m) => m.encoded_size(version),
            Self::InitProducerId(m) => m.encoded_size(version),
            Self::OffsetForLeaderEpoch(m) => m.encoded_size(version),
            Self::AddPartitionsToTxn(m) => m.encoded_size(version),
            Self::AddOffsetsToTxn(m) => m.encoded_size(version),
            Self::EndTxn(m) => m.encoded_size(version),
            Self::WriteTxnMarkers(m) => m.encoded_size(version),
            Self::TxnOffsetCommit(m) => m.encoded_size(version),
            Self::DescribeAcls(m) => m.encoded_size(version),
            Self::CreateAcls(m) => m.encoded_size(version),
            Self::DeleteAcls(m) => m.encoded_size(version),
            Self::DescribeConfigs(m) => m.encoded_size(version),
            Self::AlterConfigs(m) => m.encoded_size(version),
            Self::AlterReplicaLogDirs(m) => m.encoded_size(version),
            Self::DescribeLogDirs(m) => m.encoded_size(version),
            Self::SaslAuthenticate(m) => m.encoded_size(version),
            Self::CreatePartitions(m) => m.encoded_size(version),
            Self::CreateDelegationToken(m) => m.encoded_size(version),
            Self::RenewDelegationToken(m) => m.encoded_size(version),
            Self::ExpireDelegationToken(m) => m.encoded_size(version),
            Self::DescribeDelegationToken(m) => m.encoded_size(version),
            Self::DeleteGroups(m) => m.encoded_size(version),
            Self::ElectLeaders(m) => m.encoded_size(version),
            Self::IncrementalAlterConfigs(m) => m.encoded_size(version),
            Self::AlterPartitionReassignments(m) => m.encoded_size(version),
            Self::ListPartitionReassignments(m) => m.encoded_size(version),
            Self::OffsetDelete(m) => m.encoded_size(version),
            Self::DescribeClientQuotas(m) => m.encoded_size(version),
            Self::AlterClientQuotas(m) => m.encoded_size(version),
            Self::DescribeUserScramCredentials(m) => m.encoded_size(version),
            Self::AlterUserScramCredentials(m) => m.encoded_size(version),
            Self::Vote(m) => m.encoded_size(version),
            Self::BeginQuorumEpoch(m) => m.encoded_size(version),
            Self::EndQuorumEpoch(m) => m.encoded_size(version),
            Self::DescribeQuorum(m) => m.encoded_size(version),
            Self::AlterPartition(m) => m.encoded_size(version),
            Self::UpdateFeatures(m) => m.encoded_size(version),
            Self::Envelope(m) => m.encoded_size(version),
            Self::FetchSnapshot(m) => m.encoded_size(version),
            Self::DescribeCluster(m) => m.encoded_size(version),
            Self::DescribeProducers(m) => m.encoded_size(version),
            Self::BrokerRegistration(m) => m.encoded_size(version),
            Self::BrokerHeartbeat(m) => m.encoded_size(version),
            Self::UnregisterBroker(m) => m.encoded_size(version),
            Self::DescribeTransactions(m) => m.encoded_size(version),
            Self::ListTransactions(m) => m.encoded_size(version),
            Self::AllocateProducerIds(m) => m.encoded_size(version),
            Self::ConsumerGroupHeartbeat(m) => m.encoded_size(version),
            Self::ConsumerGroupDescribe(m) => m.encoded_size(version),
            Self::ControllerRegistration(m) => m.encoded_size(version),
            Self::GetTelemetrySubscriptions(m) => m.encoded_size(version),
            Self::PushTelemetry(m) => m.encoded_size(version),
            Self::AssignReplicasToDirs(m) => m.encoded_size(version),
            Self::ListConfigResources(m) => m.encoded_size(version),
            Self::DescribeTopicPartitions(m) => m.encoded_size(version),
            Self::ShareGroupHeartbeat(m) => m.encoded_size(version),
            Self::ShareGroupDescribe(m) => m.encoded_size(version),
            Self::ShareFetch(m) => m.encoded_size(version),
            Self::ShareAcknowledge(m) => m.encoded_size(version),
            Self::AddRaftVoter(m) => m.encoded_size(version),
            Self::RemoveRaftVoter(m) => m.encoded_size(version),
            Self::UpdateRaftVoter(m) => m.encoded_size(version),
            Self::InitializeShareGroupState(m) => m.encoded_size(version),
            Self::ReadShareGroupState(m) => m.encoded_size(version),
            Self::WriteShareGroupState(m) => m.encoded_size(version),
            Self::DeleteShareGroupState(m) => m.encoded_size(version),
            Self::ReadShareGroupStateSummary(m) => m.encoded_size(version),
            Self::StreamsGroupHeartbeat(m) => m.encoded_size(version),
            Self::StreamsGroupDescribe(m) => m.encoded_size(version),
            Self::DescribeShareGroupOffsets(m) => m.encoded_size(version),
            Self::AlterShareGroupOffsets(m) => m.encoded_size(version),
            Self::DeleteShareGroupOffsets(m) => m.encoded_size(version),
        }
    }

    /// Encode the contained message at `version`.
    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        match self {
            Self::Produce(m) => m.encode(version, buf),
            Self::Fetch(m) => m.encode(version, buf),
            Self::ListOffsets(m) => m.encode(version, buf),
            Self::Metadata(m) => m.encode(version, buf),
            Self::LeaderAndIsr(m) => m.encode(version, buf),
            Self::StopReplica(m) => m.encode(version, buf),
            Self::UpdateMetadata(m) => m.encode(version, buf),
            Self::ControlledShutdown(m) => m.encode(version, buf),
            Self::OffsetCommit(m) => m.encode(version, buf),
            Self::OffsetFetch(m) => m.encode(version, buf),
            Self::FindCoordinator(m) => m.encode(version, buf),
            Self::JoinGroup(m) => m.encode(version, buf),
            Self::Heartbeat(m) => m.encode(version, buf),
            Self::LeaveGroup(m) => m.encode(version, buf),
            Self::SyncGroup(m) => m.encode(version, buf),
            Self::DescribeGroups(m) => m.encode(version, buf),
            Self::ListGroups(m) => m.encode(version, buf),
            Self::SaslHandshake(m) => m.encode(version, buf),
            Self::ApiVersions(m) => m.encode(version, buf),
            Self::CreateTopics(m) => m.encode(version, buf),
            Self::DeleteTopics(m) => m.encode(version, buf),
            Self::DeleteRecords(m) => m.encode(version, buf),
            Self::InitProducerId(m) => m.encode(version, buf),
            Self::OffsetForLeaderEpoch(m) => m.encode(version, buf),
            Self::AddPartitionsToTxn(m) => m.encode(version, buf),
            Self::AddOffsetsToTxn(m) => m.encode(version, buf),
            Self::EndTxn(m) => m.encode(version, buf),
            Self::WriteTxnMarkers(m) => m.encode(version, buf),
            Self::TxnOffsetCommit(m) => m.encode(version, buf),
            Self::DescribeAcls(m) => m.encode(version, buf),
            Self::CreateAcls(m) => m.encode(version, buf),
            Self::DeleteAcls(m) => m.encode(version, buf),
            Self::DescribeConfigs(m) => m.encode(version, buf),
            Self::AlterConfigs(m) => m.encode(version, buf),
            Self::AlterReplicaLogDirs(m) => m.encode(version, buf),
            Self::DescribeLogDirs(m) => m.encode(version, buf),
            Self::SaslAuthenticate(m) => m.encode(version, buf),
            Self::CreatePartitions(m) => m.encode(version, buf),
            Self::CreateDelegationToken(m) => m.encode(version, buf),
            Self::RenewDelegationToken(m) => m.encode(version, buf),
            Self::ExpireDelegationToken(m) => m.encode(version, buf),
            Self::DescribeDelegationToken(m) => m.encode(version, buf),
            Self::DeleteGroups(m) => m.encode(version, buf),
            Self::ElectLeaders(m) => m.encode(version, buf),
            Self::IncrementalAlterConfigs(m) => m.encode(version, buf),
            Self::AlterPartitionReassignments(m) => m.encode(version, buf),
            Self::ListPartitionReassignments(m) => m.encode(version, buf),
            Self::OffsetDelete(m) => m.encode(version, buf),
            Self::DescribeClientQuotas(m) => m.encode(version, buf),
            Self::AlterClientQuotas(m) => m.encode(version, buf),
            Self::DescribeUserScramCredentials(m) => m.encode(version, buf),
            Self::AlterUserScramCredentials(m) => m.encode(version, buf),
            Self::Vote(m) => m.encode(version, buf),
            Self::BeginQuorumEpoch(m) => m.encode(version, buf),
            Self::EndQuorumEpoch(m) => m.encode(version, buf),
            Self::DescribeQuorum(m) => m.encode(version, buf),
            Self::AlterPartition(m) => m.encode(version, buf),
            Self::UpdateFeatures(m) => m.encode(version, buf),
            Self::Envelope(m) => m.encode(version, buf),
            Self::FetchSnapshot(m) => m.encode(version, buf),
            Self::DescribeCluster(m) => m.encode(version, buf),
            Self::DescribeProducers(m) => m.encode(version, buf),
            Self::BrokerRegistration(m) => m.encode(version, buf),
            Self::BrokerHeartbeat(m) => m.encode(version, buf),
            Self::UnregisterBroker(m) => m.encode(version, buf),
            Self::DescribeTransactions(m) => m.encode(version, buf),
            Self::ListTransactions(m) => m.encode(version, buf),
            Self::AllocateProducerIds(m) => m.encode(version, buf),
            Self::ConsumerGroupHeartbeat(m) => m.encode(version, buf),
            Self::ConsumerGroupDescribe(m) => m.encode(version, buf),
            Self::ControllerRegistration(m) => m.encode(version, buf),
            Self::GetTelemetrySubscriptions(m) => m.encode(version, buf),
            Self::PushTelemetry(m) => m.encode(version, buf),
            Self::AssignReplicasToDirs(m) => m.encode(version, buf),
            Self::ListConfigResources(m) => m.encode(version, buf),
            Self::DescribeTopicPartitions(m) => m.encode(version, buf),
            Self::ShareGroupHeartbeat(m) => m.encode(version, buf),
            Self::ShareGroupDescribe(m) => m.encode(version, buf),
            Self::ShareFetch(m) => m.encode(version, buf),
            Self::ShareAcknowledge(m) => m.encode(version, buf),
            Self::AddRaftVoter(m) => m.encode(version, buf),
            Self::RemoveRaftVoter(m) => m.encode(version, buf),
            Self::UpdateRaftVoter(m) => m.encode(version, buf),
            Self::InitializeShareGroupState(m) => m.encode(version, buf),
            Self::ReadShareGroupState(m) => m.encode(version, buf),
            Self::WriteShareGroupState(m) => m.encode(version, buf),
            Self::DeleteShareGroupState(m) => m.encode(version, buf),
            Self::ReadShareGroupStateSummary(m) => m.encode(version, buf),
            Self::StreamsGroupHeartbeat(m) => m.encode(version, buf),
            Self::StreamsGroupDescribe(m) => m.encode(version, buf),
            Self::DescribeShareGroupOffsets(m) => m.encode(version, buf),
            Self::AlterShareGroupOffsets(m) => m.encode(version, buf),
            Self::DeleteShareGroupOffsets(m) => m.encode(version, buf),
        }
    }

    /// Size-first encode into a freshly allocated, exact-capacity buffer.
    pub fn to_bytes(&self, version: i16) -> BytesMut {
        let mut buf = BytesMut::with_capacity(self.encoded_size(version));
        self.encode(version, &mut buf);
        buf
    }
}

impl From<super::produce_response::ProduceResponse> for ResponseKind {
    fn from(m: super::produce_response::ProduceResponse) -> Self {
        Self::Produce(m)
    }
}

impl From<super::fetch_response::FetchResponse> for ResponseKind {
    fn from(m: super::fetch_response::FetchResponse) -> Self {
        Self::Fetch(m)
    }
}

impl From<super::list_offsets_response::ListOffsetsResponse> for ResponseKind {
    fn from(m: super::list_offsets_response::ListOffsetsResponse) -> Self {
        Self::ListOffsets(m)
    }
}

impl From<super::metadata_response::MetadataResponse> for ResponseKind {
    fn from(m: super::metadata_response::MetadataResponse) -> Self {
        Self::Metadata(m)
    }
}

impl From<super::leader_and_isr_response::LeaderAndIsrResponse> for ResponseKind {
    fn from(m: super::leader_and_isr_response::LeaderAndIsrResponse) -> Self {
        Self::LeaderAndIsr(m)
    }
}

impl From<super::stop_replica_response::StopReplicaResponse> for ResponseKind {
    fn from(m: super::stop_replica_response::StopReplicaResponse) -> Self {
        Self::StopReplica(m)
    }
}

impl From<super::update_metadata_response::UpdateMetadataResponse> for ResponseKind {
    fn from(m: super::update_metadata_response::UpdateMetadataResponse) -> Self {
        Self::UpdateMetadata(m)
    }
}

impl From<super::controlled_shutdown_response::ControlledShutdownResponse> for ResponseKind {
    fn from(m: super::controlled_shutdown_response::ControlledShutdownResponse) -> Self {
        Self::ControlledShutdown(m)
    }
}

impl From<super::offset_commit_response::OffsetCommitResponse> for ResponseKind {
    fn from(m: super::offset_commit_response::OffsetCommitResponse) -> Self {
        Self::OffsetCommit(m)
    }
}

impl From<super::offset_fetch_response::OffsetFetchResponse> for ResponseKind {
    fn from(m: super::offset_fetch_response::OffsetFetchResponse) -> Self {
        Self::OffsetFetch(m)
    }
}

impl From<super::find_coordinator_response::FindCoordinatorResponse> for ResponseKind {
    fn from(m: super::find_coordinator_response::FindCoordinatorResponse) -> Self {
        Self::FindCoordinator(m)
    }
}

impl From<super::join_group_response::JoinGroupResponse> for ResponseKind {
    fn from(m: super::join_group_response::JoinGroupResponse) -> Self {
        Self::JoinGroup(m)
    }
}

impl From<super::heartbeat_response::HeartbeatResponse> for ResponseKind {
    fn from(m: super::heartbeat_response::HeartbeatResponse) -> Self {
        Self::Heartbeat(m)
    }
}

impl From<super::leave_group_response::LeaveGroupResponse> for ResponseKind {
    fn from(m: super::leave_group_response::LeaveGroupResponse) -> Self {
        Self::LeaveGroup(m)
    }
}

impl From<super::sync_group_response::SyncGroupResponse> for ResponseKind {
    fn from(m: super::sync_group_response::SyncGroupResponse) -> Self {
        Self::SyncGroup(m)
    }
}

impl From<super::describe_groups_response::DescribeGroupsResponse> for ResponseKind {
    fn from(m: super::describe_groups_response::DescribeGroupsResponse) -> Self {
        Self::DescribeGroups(m)
    }
}

impl From<super::list_groups_response::ListGroupsResponse> for ResponseKind {
    fn from(m: super::list_groups_response::ListGroupsResponse) -> Self {
        Self::ListGroups(m)
    }
}

impl From<super::sasl_handshake_response::SaslHandshakeResponse> for ResponseKind {
    fn from(m: super::sasl_handshake_response::SaslHandshakeResponse) -> Self {
        Self::SaslHandshake(m)
    }
}

impl From<super::api_versions_response::ApiVersionsResponse> for ResponseKind {
    fn from(m: super::api_versions_response::ApiVersionsResponse) -> Self {
        Self::ApiVersions(m)
    }
}

impl From<super::create_topics_response::CreateTopicsResponse> for ResponseKind {
    fn from(m: super::create_topics_response::CreateTopicsResponse) -> Self {
        Self::CreateTopics(m)
    }
}

impl From<super::delete_topics_response::DeleteTopicsResponse> for ResponseKind {
    fn from(m: super::delete_topics_response::DeleteTopicsResponse) -> Self {
        Self::DeleteTopics(m)
    }
}

impl From<super::delete_records_response::DeleteRecordsResponse> for ResponseKind {
    fn from(m: super::delete_records_response::DeleteRecordsResponse) -> Self {
        Self::DeleteRecords(m)
    }
}

impl From<super::init_producer_id_response::InitProducerIdResponse> for ResponseKind {
    fn from(m: super::init_producer_id_response::InitProducerIdResponse) -> Self {
        Self::InitProducerId(m)
    }
}

impl From<super::offset_for_leader_epoch_response::OffsetForLeaderEpochResponse> for ResponseKind {
    fn from(m: super::offset_for_leader_epoch_response::OffsetForLeaderEpochResponse) -> Self {
        Self::OffsetForLeaderEpoch(m)
    }
}

impl From<super::add_partitions_to_txn_response::AddPartitionsToTxnResponse> for ResponseKind {
    fn from(m: super::add_partitions_to_txn_response::AddPartitionsToTxnResponse) -> Self {
        Self::AddPartitionsToTxn(m)
    }
}

impl From<super::add_offsets_to_txn_response::AddOffsetsToTxnResponse> for ResponseKind {
    fn from(m: super::add_offsets_to_txn_response::AddOffsetsToTxnResponse) -> Self {
        Self::AddOffsetsToTxn(m)
    }
}

impl From<super::end_txn_response::EndTxnResponse> for ResponseKind {
    fn from(m: super::end_txn_response::EndTxnResponse) -> Self {
        Self::EndTxn(m)
    }
}

impl From<super::write_txn_markers_response::WriteTxnMarkersResponse> for ResponseKind {
    fn from(m: super::write_txn_markers_response::WriteTxnMarkersResponse) -> Self {
        Self::WriteTxnMarkers(m)
    }
}

impl From<super::txn_offset_commit_response::TxnOffsetCommitResponse> for ResponseKind {
    fn from(m: super::txn_offset_commit_response::TxnOffsetCommitResponse) -> Self {
        Self::TxnOffsetCommit(m)
    }
}

impl From<super::describe_acls_response::DescribeAclsResponse> for ResponseKind {
    fn from(m: super::describe_acls_response::DescribeAclsResponse) -> Self {
        Self::DescribeAcls(m)
    }
}

impl From<super::create_acls_response::CreateAclsResponse> for ResponseKind {
    fn from(m: super::create_acls_response::CreateAclsResponse) -> Self {
        Self::CreateAcls(m)
    }
}

impl From<super::delete_acls_response::DeleteAclsResponse> for ResponseKind {
    fn from(m: super::delete_acls_response::DeleteAclsResponse) -> Self {
        Self::DeleteAcls(m)
    }
}

impl From<super::describe_configs_response::DescribeConfigsResponse> for ResponseKind {
    fn from(m: super::describe_configs_response::DescribeConfigsResponse) -> Self {
        Self::DescribeConfigs(m)
    }
}

impl From<super::alter_configs_response::AlterConfigsResponse> for ResponseKind {
    fn from(m: super::alter_configs_response::AlterConfigsResponse) -> Self {
        Self::AlterConfigs(m)
    }
}

impl From<super::alter_replica_log_dirs_response::AlterReplicaLogDirsResponse> for ResponseKind {
    fn from(m: super::alter_replica_log_dirs_response::AlterReplicaLogDirsResponse) -> Self {
        Self::AlterReplicaLogDirs(m)
    }
}

impl From<super::describe_log_dirs_response::DescribeLogDirsResponse> for ResponseKind {
    fn from(m: super::describe_log_dirs_response::DescribeLogDirsResponse) -> Self {
        Self::DescribeLogDirs(m)
    }
}

impl From<super::sasl_authenticate_response::SaslAuthenticateResponse> for ResponseKind {
    fn from(m: super::sasl_authenticate_response::SaslAuthenticateResponse) -> Self {
        Self::SaslAuthenticate(m)
    }
}

impl From<super::create_partitions_response::CreatePartitionsResponse> for ResponseKind {
    fn from(m: super::create_partitions_response::CreatePartitionsResponse) -> Self {
        Self::CreatePartitions(m)
    }
}

impl From<super::create_delegation_token_response::CreateDelegationTokenResponse> for ResponseKind {
    fn from(m: super::create_delegation_token_response::CreateDelegationTokenResponse) -> Self {
        Self::CreateDelegationToken(m)
    }
}

impl From<super::renew_delegation_token_response::RenewDelegationTokenResponse> for ResponseKind {
    fn from(m: super::renew_delegation_token_response::RenewDelegationTokenResponse) -> Self {
        Self::RenewDelegationToken(m)
    }
}

impl From<super::expire_delegation_token_response::ExpireDelegationTokenResponse> for ResponseKind {
    fn from(m: super::expire_delegation_token_response::ExpireDelegationTokenResponse) -> Self {
        Self::ExpireDelegationToken(m)
    }
}

impl From<super::describe_delegation_token_response::DescribeDelegationTokenResponse> for ResponseKind {
    fn from(m: super::describe_delegation_token_response::DescribeDelegationTokenResponse) -> Self {
        Self::DescribeDelegationToken(m)
    }
}

impl From<super::delete_groups_response::DeleteGroupsResponse> for ResponseKind {
    fn from(m: super::delete_groups_response::DeleteGroupsResponse) -> Self {
        Self::DeleteGroups(m)
    }
}

impl From<super::elect_leaders_response::ElectLeadersResponse> for ResponseKind {
    fn from(m: super::elect_leaders_response::ElectLeadersResponse) -> Self {
        Self::ElectLeaders(m)
    }
}

impl From<super::incremental_alter_configs_response::IncrementalAlterConfigsResponse> for ResponseKind {
    fn from(m: super::incremental_alter_configs_response::IncrementalAlterConfigsResponse) -> Self {
        Self::IncrementalAlterConfigs(m)
    }
}

impl From<super::alter_partition_reassignments_response::AlterPartitionReassignmentsResponse> for ResponseKind {
    fn from(m: super::alter_partition_reassignments_response::AlterPartitionReassignmentsResponse) -> Self {
        Self::AlterPartitionReassignments(m)
    }
}

impl From<super::list_partition_reassignments_response::ListPartitionReassignmentsResponse> for ResponseKind {
    fn from(m: super::list_partition_reassignments_response::ListPartitionReassignmentsResponse) -> Self {
        Self::ListPartitionReassignments(m)
    }
}

impl From<super::offset_delete_response::OffsetDeleteResponse> for ResponseKind {
    fn from(m: super::offset_delete_response::OffsetDeleteResponse) -> Self {
        Self::OffsetDelete(m)
    }
}

impl From<super::describe_client_quotas_response::DescribeClientQuotasResponse> for ResponseKind {
    fn from(m: super::describe_client_quotas_response::DescribeClientQuotasResponse) -> Self {
        Self::DescribeClientQuotas(m)
    }
}

impl From<super::alter_client_quotas_response::AlterClientQuotasResponse> for ResponseKind {
    fn from(m: super::alter_client_quotas_response::AlterClientQuotasResponse) -> Self {
        Self::AlterClientQuotas(m)
    }
}

impl From<super::describe_user_scram_credentials_response::DescribeUserScramCredentialsResponse> for ResponseKind {
    fn from(m: super::describe_user_scram_credentials_response::DescribeUserScramCredentialsResponse) -> Self {
        Self::DescribeUserScramCredentials(m)
    }
}

impl From<super::alter_user_scram_credentials_response::AlterUserScramCredentialsResponse> for ResponseKind {
    fn from(m: super::alter_user_scram_credentials_response::AlterUserScramCredentialsResponse) -> Self {
        Self::AlterUserScramCredentials(m)
    }
}

impl From<super::vote_response::VoteResponse> for ResponseKind {
    fn from(m: super::vote_response::VoteResponse) -> Self {
        Self::Vote(m)
    }
}

impl From<super::begin_quorum_epoch_response::BeginQuorumEpochResponse> for ResponseKind {
    fn from(m: super::begin_quorum_epoch_response::BeginQuorumEpochResponse) -> Self {
        Self::BeginQuorumEpoch(m)
    }
}

impl From<super::end_quorum_epoch_response::EndQuorumEpochResponse> for ResponseKind {
    fn from(m: super::end_quorum_epoch_response::EndQuorumEpochResponse) -> Self {
        Self::EndQuorumEpoch(m)
    }
}

impl From<super::describe_quorum_response::DescribeQuorumResponse> for ResponseKind {
    fn from(m: super::describe_quorum_response::DescribeQuorumResponse) -> Self {
        Self::DescribeQuorum(m)
    }
}

impl From<super::alter_partition_response::AlterPartitionResponse> for ResponseKind {
    fn from(m: super::alter_partition_response::AlterPartitionResponse) -> Self {
        Self::AlterPartition(m)
    }
}

impl From<super::update_features_response::UpdateFeaturesResponse> for ResponseKind {
    fn from(m: super::update_features_response::UpdateFeaturesResponse) -> Self {
        Self::UpdateFeatures(m)
    }
}

impl From<super::envelope_response::EnvelopeResponse> for ResponseKind {
    fn from(m: super::envelope_response::EnvelopeResponse) -> Self {
        Self::Envelope(m)
    }
}

impl From<super::fetch_snapshot_response::FetchSnapshotResponse> for ResponseKind {
    fn from(m: super::fetch_snapshot_response::FetchSnapshotResponse) -> Self {
        Self::FetchSnapshot(m)
    }
}

impl From<super::describe_cluster_response::DescribeClusterResponse> for ResponseKind {
    fn from(m: super::describe_cluster_response::DescribeClusterResponse) -> Self {
        Self::DescribeCluster(m)
    }
}

impl From<super::describe_producers_response::DescribeProducersResponse> for ResponseKind {
    fn from(m: super::describe_producers_response::DescribeProducersResponse) -> Self {
        Self::DescribeProducers(m)
    }
}

impl From<super::broker_registration_response::BrokerRegistrationResponse> for ResponseKind {
    fn from(m: super::broker_registration_response::BrokerRegistrationResponse) -> Self {
        Self::BrokerRegistration(m)
    }
}

impl From<super::broker_heartbeat_response::BrokerHeartbeatResponse> for ResponseKind {
    fn from(m: super::broker_heartbeat_response::BrokerHeartbeatResponse) -> Self {
        Self::BrokerHeartbeat(m)
    }
}

impl From<super::unregister_broker_response::UnregisterBrokerResponse> for ResponseKind {
    fn from(m: super::unregister_broker_response::UnregisterBrokerResponse) -> Self {
        Self::UnregisterBroker(m)
    }
}

impl From<super::describe_transactions_response::DescribeTransactionsResponse> for ResponseKind {
    fn from(m: super::describe_transactions_response::DescribeTransactionsResponse) -> Self {
        Self::DescribeTransactions(m)
    }
}

impl From<super::list_transactions_response::ListTransactionsResponse> for ResponseKind {
    fn from(m: super::list_transactions_response::ListTransactionsResponse) -> Self {
        Self::ListTransactions(m)
    }
}

impl From<super::allocate_producer_ids_response::AllocateProducerIdsResponse> for ResponseKind {
    fn from(m: super::allocate_producer_ids_response::AllocateProducerIdsResponse) -> Self {
        Self::AllocateProducerIds(m)
    }
}

impl From<super::consumer_group_heartbeat_response::ConsumerGroupHeartbeatResponse> for ResponseKind {
    fn from(m: super::consumer_group_heartbeat_response::ConsumerGroupHeartbeatResponse) -> Self {
        Self::ConsumerGroupHeartbeat(m)
    }
}

impl From<super::consumer_group_describe_response::ConsumerGroupDescribeResponse> for ResponseKind {
    fn from(m: super::consumer_group_describe_response::ConsumerGroupDescribeResponse) -> Self {
        Self::ConsumerGroupDescribe(m)
    }
}

impl From<super::controller_registration_response::ControllerRegistrationResponse> for ResponseKind {
    fn from(m: super::controller_registration_response::ControllerRegistrationResponse) -> Self {
        Self::ControllerRegistration(m)
    }
}

impl From<super::get_telemetry_subscriptions_response::GetTelemetrySubscriptionsResponse> for ResponseKind {
    fn from(m: super::get_telemetry_subscriptions_response::GetTelemetrySubscriptionsResponse) -> Self {
        Self::GetTelemetrySubscriptions(m)
    }
}

impl From<super::push_telemetry_response::PushTelemetryResponse> for ResponseKind {
    fn from(m: super::push_telemetry_response::PushTelemetryResponse) -> Self {
        Self::PushTelemetry(m)
    }
}

impl From<super::assign_replicas_to_dirs_response::AssignReplicasToDirsResponse> for ResponseKind {
    fn from(m: super::assign_replicas_to_dirs_response::AssignReplicasToDirsResponse) -> Self {
        Self::AssignReplicasToDirs(m)
    }
}

impl From<super::list_config_resources_response::ListConfigResourcesResponse> for ResponseKind {
    fn from(m: super::list_config_resources_response::ListConfigResourcesResponse) -> Self {
        Self::ListConfigResources(m)
    }
}

impl From<super::describe_topic_partitions_response::DescribeTopicPartitionsResponse> for ResponseKind {
    fn from(m: super::describe_topic_partitions_response::DescribeTopicPartitionsResponse) -> Self {
        Self::DescribeTopicPartitions(m)
    }
}

impl From<super::share_group_heartbeat_response::ShareGroupHeartbeatResponse> for ResponseKind {
    fn from(m: super::share_group_heartbeat_response::ShareGroupHeartbeatResponse) -> Self {
        Self::ShareGroupHeartbeat(m)
    }
}

impl From<super::share_group_describe_response::ShareGroupDescribeResponse> for ResponseKind {
    fn from(m: super::share_group_describe_response::ShareGroupDescribeResponse) -> Self {
        Self::ShareGroupDescribe(m)
    }
}

impl From<super::share_fetch_response::ShareFetchResponse> for ResponseKind {
    fn from(m: super::share_fetch_response::ShareFetchResponse) -> Self {
        Self::ShareFetch(m)
    }
}

impl From<super::share_acknowledge_response::ShareAcknowledgeResponse> for ResponseKind {
    fn from(m: super::share_acknowledge_response::ShareAcknowledgeResponse) -> Self {
        Self::ShareAcknowledge(m)
    }
}

impl From<super::add_raft_voter_response::AddRaftVoterResponse> for ResponseKind {
    fn from(m: super::add_raft_voter_response::AddRaftVoterResponse) -> Self {
        Self::AddRaftVoter(m)
    }
}

impl From<super::remove_raft_voter_response::RemoveRaftVoterResponse> for ResponseKind {
    fn from(m: super::remove_raft_voter_response::RemoveRaftVoterResponse) -> Self {
        Self::RemoveRaftVoter(m)
    }
}

impl From<super::update_raft_voter_response::UpdateRaftVoterResponse> for ResponseKind {
    fn from(m: super::update_raft_voter_response::UpdateRaftVoterResponse) -> Self {
        Self::UpdateRaftVoter(m)
    }
}

impl From<super::initialize_share_group_state_response::InitializeShareGroupStateResponse> for ResponseKind {
    fn from(m: super::initialize_share_group_state_response::InitializeShareGroupStateResponse) -> Self {
        Self::InitializeShareGroupState(m)
    }
}

impl From<super::read_share_group_state_response::ReadShareGroupStateResponse> for ResponseKind {
    fn from(m: super::read_share_group_state_response::ReadShareGroupStateResponse) -> Self {
        Self::ReadShareGroupState(m)
    }
}

impl From<super::write_share_group_state_response::WriteShareGroupStateResponse> for ResponseKind {
    fn from(m: super::write_share_group_state_response::WriteShareGroupStateResponse) -> Self {
        Self::WriteShareGroupState(m)
    }
}

impl From<super::delete_share_group_state_response::DeleteShareGroupStateResponse> for ResponseKind {
    fn from(m: super::delete_share_group_state_response::DeleteShareGroupStateResponse) -> Self {
        Self::DeleteShareGroupState(m)
    }
}

impl From<super::read_share_group_state_summary_response::ReadShareGroupStateSummaryResponse> for ResponseKind {
    fn from(m: super::read_share_group_state_summary_response::ReadShareGroupStateSummaryResponse) -> Self {
        Self::ReadShareGroupStateSummary(m)
    }
}

impl From<super::streams_group_heartbeat_response::StreamsGroupHeartbeatResponse> for ResponseKind {
    fn from(m: super::streams_group_heartbeat_response::StreamsGroupHeartbeatResponse) -> Self {
        Self::StreamsGroupHeartbeat(m)
    }
}

impl From<super::streams_group_describe_response::StreamsGroupDescribeResponse> for ResponseKind {
    fn from(m: super::streams_group_describe_response::StreamsGroupDescribeResponse) -> Self {
        Self::StreamsGroupDescribe(m)
    }
}

impl From<super::describe_share_group_offsets_response::DescribeShareGroupOffsetsResponse> for ResponseKind {
    fn from(m: super::describe_share_group_offsets_response::DescribeShareGroupOffsetsResponse) -> Self {
        Self::DescribeShareGroupOffsets(m)
    }
}

impl From<super::alter_share_group_offsets_response::AlterShareGroupOffsetsResponse> for ResponseKind {
    fn from(m: super::alter_share_group_offsets_response::AlterShareGroupOffsetsResponse) -> Self {
        Self::AlterShareGroupOffsets(m)
    }
}

impl From<super::delete_share_group_offsets_response::DeleteShareGroupOffsetsResponse> for ResponseKind {
    fn from(m: super::delete_share_group_offsets_response::DeleteShareGroupOffsetsResponse) -> Self {
        Self::DeleteShareGroupOffsets(m)
    }
}

