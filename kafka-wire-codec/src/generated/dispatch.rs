// Generated dispatch table — do not edit.
use bytes::{Bytes, BytesMut};
use crate::error::DecodeError;

/// Every generated RPC: (api_key, is_request, min_version, max_version).
pub const MESSAGES: &[(i16, bool, i16, i16)] = &[
    (0, true, 3, 13), // ProduceRequest
    (0, false, 3, 13), // ProduceResponse
    (1, true, 4, 18), // FetchRequest
    (1, false, 4, 18), // FetchResponse
    (2, true, 1, 11), // ListOffsetsRequest
    (2, false, 1, 11), // ListOffsetsResponse
    (3, true, 0, 13), // MetadataRequest
    (3, false, 0, 13), // MetadataResponse
    (4, true, 32767, -32768), // LeaderAndIsrRequest
    (4, false, 32767, -32768), // LeaderAndIsrResponse
    (5, true, 32767, -32768), // StopReplicaRequest
    (5, false, 32767, -32768), // StopReplicaResponse
    (6, true, 32767, -32768), // UpdateMetadataRequest
    (6, false, 32767, -32768), // UpdateMetadataResponse
    (7, true, 32767, -32768), // ControlledShutdownRequest
    (7, false, 32767, -32768), // ControlledShutdownResponse
    (8, true, 2, 10), // OffsetCommitRequest
    (8, false, 2, 10), // OffsetCommitResponse
    (9, true, 1, 10), // OffsetFetchRequest
    (9, false, 1, 10), // OffsetFetchResponse
    (10, true, 0, 6), // FindCoordinatorRequest
    (10, false, 0, 6), // FindCoordinatorResponse
    (11, true, 0, 9), // JoinGroupRequest
    (11, false, 0, 9), // JoinGroupResponse
    (12, true, 0, 4), // HeartbeatRequest
    (12, false, 0, 4), // HeartbeatResponse
    (13, true, 0, 5), // LeaveGroupRequest
    (13, false, 0, 5), // LeaveGroupResponse
    (14, true, 0, 5), // SyncGroupRequest
    (14, false, 0, 5), // SyncGroupResponse
    (15, true, 0, 6), // DescribeGroupsRequest
    (15, false, 0, 6), // DescribeGroupsResponse
    (16, true, 0, 5), // ListGroupsRequest
    (16, false, 0, 5), // ListGroupsResponse
    (17, true, 0, 1), // SaslHandshakeRequest
    (17, false, 0, 1), // SaslHandshakeResponse
    (18, true, 0, 4), // ApiVersionsRequest
    (18, false, 0, 4), // ApiVersionsResponse
    (19, true, 2, 7), // CreateTopicsRequest
    (19, false, 2, 7), // CreateTopicsResponse
    (20, true, 1, 6), // DeleteTopicsRequest
    (20, false, 1, 6), // DeleteTopicsResponse
    (21, true, 0, 2), // DeleteRecordsRequest
    (21, false, 0, 2), // DeleteRecordsResponse
    (22, true, 0, 6), // InitProducerIdRequest
    (22, false, 0, 6), // InitProducerIdResponse
    (23, true, 2, 4), // OffsetForLeaderEpochRequest
    (23, false, 2, 4), // OffsetForLeaderEpochResponse
    (24, true, 0, 5), // AddPartitionsToTxnRequest
    (24, false, 0, 5), // AddPartitionsToTxnResponse
    (25, true, 0, 4), // AddOffsetsToTxnRequest
    (25, false, 0, 4), // AddOffsetsToTxnResponse
    (26, true, 0, 5), // EndTxnRequest
    (26, false, 0, 5), // EndTxnResponse
    (27, true, 1, 2), // WriteTxnMarkersRequest
    (27, false, 1, 2), // WriteTxnMarkersResponse
    (28, true, 0, 5), // TxnOffsetCommitRequest
    (28, false, 0, 5), // TxnOffsetCommitResponse
    (29, true, 1, 3), // DescribeAclsRequest
    (29, false, 1, 3), // DescribeAclsResponse
    (30, true, 1, 3), // CreateAclsRequest
    (30, false, 1, 3), // CreateAclsResponse
    (31, true, 1, 3), // DeleteAclsRequest
    (31, false, 1, 3), // DeleteAclsResponse
    (32, true, 1, 4), // DescribeConfigsRequest
    (32, false, 1, 4), // DescribeConfigsResponse
    (33, true, 0, 2), // AlterConfigsRequest
    (33, false, 0, 2), // AlterConfigsResponse
    (34, true, 1, 2), // AlterReplicaLogDirsRequest
    (34, false, 1, 2), // AlterReplicaLogDirsResponse
    (35, true, 1, 5), // DescribeLogDirsRequest
    (35, false, 1, 5), // DescribeLogDirsResponse
    (36, true, 0, 2), // SaslAuthenticateRequest
    (36, false, 0, 2), // SaslAuthenticateResponse
    (37, true, 0, 3), // CreatePartitionsRequest
    (37, false, 0, 3), // CreatePartitionsResponse
    (38, true, 1, 3), // CreateDelegationTokenRequest
    (38, false, 1, 3), // CreateDelegationTokenResponse
    (39, true, 1, 2), // RenewDelegationTokenRequest
    (39, false, 1, 2), // RenewDelegationTokenResponse
    (40, true, 1, 2), // ExpireDelegationTokenRequest
    (40, false, 1, 2), // ExpireDelegationTokenResponse
    (41, true, 1, 3), // DescribeDelegationTokenRequest
    (41, false, 1, 3), // DescribeDelegationTokenResponse
    (42, true, 0, 2), // DeleteGroupsRequest
    (42, false, 0, 2), // DeleteGroupsResponse
    (43, true, 0, 2), // ElectLeadersRequest
    (43, false, 0, 2), // ElectLeadersResponse
    (44, true, 0, 1), // IncrementalAlterConfigsRequest
    (44, false, 0, 1), // IncrementalAlterConfigsResponse
    (45, true, 0, 1), // AlterPartitionReassignmentsRequest
    (45, false, 0, 1), // AlterPartitionReassignmentsResponse
    (46, true, 0, 0), // ListPartitionReassignmentsRequest
    (46, false, 0, 0), // ListPartitionReassignmentsResponse
    (47, true, 0, 0), // OffsetDeleteRequest
    (47, false, 0, 0), // OffsetDeleteResponse
    (48, true, 0, 1), // DescribeClientQuotasRequest
    (48, false, 0, 1), // DescribeClientQuotasResponse
    (49, true, 0, 1), // AlterClientQuotasRequest
    (49, false, 0, 1), // AlterClientQuotasResponse
    (50, true, 0, 0), // DescribeUserScramCredentialsRequest
    (50, false, 0, 0), // DescribeUserScramCredentialsResponse
    (51, true, 0, 0), // AlterUserScramCredentialsRequest
    (51, false, 0, 0), // AlterUserScramCredentialsResponse
    (52, true, 0, 2), // VoteRequest
    (52, false, 0, 2), // VoteResponse
    (53, true, 0, 1), // BeginQuorumEpochRequest
    (53, false, 0, 1), // BeginQuorumEpochResponse
    (54, true, 0, 1), // EndQuorumEpochRequest
    (54, false, 0, 1), // EndQuorumEpochResponse
    (55, true, 0, 2), // DescribeQuorumRequest
    (55, false, 0, 2), // DescribeQuorumResponse
    (56, true, 2, 3), // AlterPartitionRequest
    (56, false, 2, 3), // AlterPartitionResponse
    (57, true, 0, 2), // UpdateFeaturesRequest
    (57, false, 0, 2), // UpdateFeaturesResponse
    (58, true, 0, 0), // EnvelopeRequest
    (58, false, 0, 0), // EnvelopeResponse
    (59, true, 0, 1), // FetchSnapshotRequest
    (59, false, 0, 1), // FetchSnapshotResponse
    (60, true, 0, 2), // DescribeClusterRequest
    (60, false, 0, 2), // DescribeClusterResponse
    (61, true, 0, 0), // DescribeProducersRequest
    (61, false, 0, 0), // DescribeProducersResponse
    (62, true, 0, 4), // BrokerRegistrationRequest
    (62, false, 0, 4), // BrokerRegistrationResponse
    (63, true, 0, 2), // BrokerHeartbeatRequest
    (63, false, 0, 2), // BrokerHeartbeatResponse
    (64, true, 0, 0), // UnregisterBrokerRequest
    (64, false, 0, 0), // UnregisterBrokerResponse
    (65, true, 0, 0), // DescribeTransactionsRequest
    (65, false, 0, 0), // DescribeTransactionsResponse
    (66, true, 0, 2), // ListTransactionsRequest
    (66, false, 0, 2), // ListTransactionsResponse
    (67, true, 0, 0), // AllocateProducerIdsRequest
    (67, false, 0, 0), // AllocateProducerIdsResponse
    (68, true, 0, 1), // ConsumerGroupHeartbeatRequest
    (68, false, 0, 1), // ConsumerGroupHeartbeatResponse
    (69, true, 0, 1), // ConsumerGroupDescribeRequest
    (69, false, 0, 1), // ConsumerGroupDescribeResponse
    (70, true, 0, 0), // ControllerRegistrationRequest
    (70, false, 0, 0), // ControllerRegistrationResponse
    (71, true, 0, 0), // GetTelemetrySubscriptionsRequest
    (71, false, 0, 0), // GetTelemetrySubscriptionsResponse
    (72, true, 0, 0), // PushTelemetryRequest
    (72, false, 0, 0), // PushTelemetryResponse
    (73, true, 0, 0), // AssignReplicasToDirsRequest
    (73, false, 0, 0), // AssignReplicasToDirsResponse
    (74, true, 0, 1), // ListConfigResourcesRequest
    (74, false, 0, 1), // ListConfigResourcesResponse
    (75, true, 0, 0), // DescribeTopicPartitionsRequest
    (75, false, 0, 0), // DescribeTopicPartitionsResponse
    (76, true, 1, 1), // ShareGroupHeartbeatRequest
    (76, false, 1, 1), // ShareGroupHeartbeatResponse
    (77, true, 1, 1), // ShareGroupDescribeRequest
    (77, false, 1, 1), // ShareGroupDescribeResponse
    (78, true, 1, 2), // ShareFetchRequest
    (78, false, 1, 2), // ShareFetchResponse
    (79, true, 1, 2), // ShareAcknowledgeRequest
    (79, false, 1, 2), // ShareAcknowledgeResponse
    (80, true, 0, 1), // AddRaftVoterRequest
    (80, false, 0, 1), // AddRaftVoterResponse
    (81, true, 0, 0), // RemoveRaftVoterRequest
    (81, false, 0, 0), // RemoveRaftVoterResponse
    (82, true, 0, 0), // UpdateRaftVoterRequest
    (82, false, 0, 0), // UpdateRaftVoterResponse
    (83, true, 0, 0), // InitializeShareGroupStateRequest
    (83, false, 0, 0), // InitializeShareGroupStateResponse
    (84, true, 0, 0), // ReadShareGroupStateRequest
    (84, false, 0, 0), // ReadShareGroupStateResponse
    (85, true, 0, 1), // WriteShareGroupStateRequest
    (85, false, 0, 1), // WriteShareGroupStateResponse
    (86, true, 0, 0), // DeleteShareGroupStateRequest
    (86, false, 0, 0), // DeleteShareGroupStateResponse
    (87, true, 0, 1), // ReadShareGroupStateSummaryRequest
    (87, false, 0, 1), // ReadShareGroupStateSummaryResponse
    (88, true, 0, 0), // StreamsGroupHeartbeatRequest
    (88, false, 0, 0), // StreamsGroupHeartbeatResponse
    (89, true, 0, 0), // StreamsGroupDescribeRequest
    (89, false, 0, 0), // StreamsGroupDescribeResponse
    (90, true, 0, 1), // DescribeShareGroupOffsetsRequest
    (90, false, 0, 1), // DescribeShareGroupOffsetsResponse
    (91, true, 0, 0), // AlterShareGroupOffsetsRequest
    (91, false, 0, 0), // AlterShareGroupOffsetsResponse
    (92, true, 0, 0), // DeleteShareGroupOffsetsRequest
    (92, false, 0, 0), // DeleteShareGroupOffsetsResponse
];

/// Supported (min, max) protocol versions for `api_key`, taken from the
/// request schema. None for unknown api keys and retired (version-less) APIs.
pub fn supported_request_versions(api_key: i16) -> Option<(i16, i16)> {
    match api_key {
        0 => Some((3, 13)), // ProduceRequest
        1 => Some((4, 18)), // FetchRequest
        2 => Some((1, 11)), // ListOffsetsRequest
        3 => Some((0, 13)), // MetadataRequest
        8 => Some((2, 10)), // OffsetCommitRequest
        9 => Some((1, 10)), // OffsetFetchRequest
        10 => Some((0, 6)), // FindCoordinatorRequest
        11 => Some((0, 9)), // JoinGroupRequest
        12 => Some((0, 4)), // HeartbeatRequest
        13 => Some((0, 5)), // LeaveGroupRequest
        14 => Some((0, 5)), // SyncGroupRequest
        15 => Some((0, 6)), // DescribeGroupsRequest
        16 => Some((0, 5)), // ListGroupsRequest
        17 => Some((0, 1)), // SaslHandshakeRequest
        18 => Some((0, 4)), // ApiVersionsRequest
        19 => Some((2, 7)), // CreateTopicsRequest
        20 => Some((1, 6)), // DeleteTopicsRequest
        21 => Some((0, 2)), // DeleteRecordsRequest
        22 => Some((0, 6)), // InitProducerIdRequest
        23 => Some((2, 4)), // OffsetForLeaderEpochRequest
        24 => Some((0, 5)), // AddPartitionsToTxnRequest
        25 => Some((0, 4)), // AddOffsetsToTxnRequest
        26 => Some((0, 5)), // EndTxnRequest
        27 => Some((1, 2)), // WriteTxnMarkersRequest
        28 => Some((0, 5)), // TxnOffsetCommitRequest
        29 => Some((1, 3)), // DescribeAclsRequest
        30 => Some((1, 3)), // CreateAclsRequest
        31 => Some((1, 3)), // DeleteAclsRequest
        32 => Some((1, 4)), // DescribeConfigsRequest
        33 => Some((0, 2)), // AlterConfigsRequest
        34 => Some((1, 2)), // AlterReplicaLogDirsRequest
        35 => Some((1, 5)), // DescribeLogDirsRequest
        36 => Some((0, 2)), // SaslAuthenticateRequest
        37 => Some((0, 3)), // CreatePartitionsRequest
        38 => Some((1, 3)), // CreateDelegationTokenRequest
        39 => Some((1, 2)), // RenewDelegationTokenRequest
        40 => Some((1, 2)), // ExpireDelegationTokenRequest
        41 => Some((1, 3)), // DescribeDelegationTokenRequest
        42 => Some((0, 2)), // DeleteGroupsRequest
        43 => Some((0, 2)), // ElectLeadersRequest
        44 => Some((0, 1)), // IncrementalAlterConfigsRequest
        45 => Some((0, 1)), // AlterPartitionReassignmentsRequest
        46 => Some((0, 0)), // ListPartitionReassignmentsRequest
        47 => Some((0, 0)), // OffsetDeleteRequest
        48 => Some((0, 1)), // DescribeClientQuotasRequest
        49 => Some((0, 1)), // AlterClientQuotasRequest
        50 => Some((0, 0)), // DescribeUserScramCredentialsRequest
        51 => Some((0, 0)), // AlterUserScramCredentialsRequest
        52 => Some((0, 2)), // VoteRequest
        53 => Some((0, 1)), // BeginQuorumEpochRequest
        54 => Some((0, 1)), // EndQuorumEpochRequest
        55 => Some((0, 2)), // DescribeQuorumRequest
        56 => Some((2, 3)), // AlterPartitionRequest
        57 => Some((0, 2)), // UpdateFeaturesRequest
        58 => Some((0, 0)), // EnvelopeRequest
        59 => Some((0, 1)), // FetchSnapshotRequest
        60 => Some((0, 2)), // DescribeClusterRequest
        61 => Some((0, 0)), // DescribeProducersRequest
        62 => Some((0, 4)), // BrokerRegistrationRequest
        63 => Some((0, 2)), // BrokerHeartbeatRequest
        64 => Some((0, 0)), // UnregisterBrokerRequest
        65 => Some((0, 0)), // DescribeTransactionsRequest
        66 => Some((0, 2)), // ListTransactionsRequest
        67 => Some((0, 0)), // AllocateProducerIdsRequest
        68 => Some((0, 1)), // ConsumerGroupHeartbeatRequest
        69 => Some((0, 1)), // ConsumerGroupDescribeRequest
        70 => Some((0, 0)), // ControllerRegistrationRequest
        71 => Some((0, 0)), // GetTelemetrySubscriptionsRequest
        72 => Some((0, 0)), // PushTelemetryRequest
        73 => Some((0, 0)), // AssignReplicasToDirsRequest
        74 => Some((0, 1)), // ListConfigResourcesRequest
        75 => Some((0, 0)), // DescribeTopicPartitionsRequest
        76 => Some((1, 1)), // ShareGroupHeartbeatRequest
        77 => Some((1, 1)), // ShareGroupDescribeRequest
        78 => Some((1, 2)), // ShareFetchRequest
        79 => Some((1, 2)), // ShareAcknowledgeRequest
        80 => Some((0, 1)), // AddRaftVoterRequest
        81 => Some((0, 0)), // RemoveRaftVoterRequest
        82 => Some((0, 0)), // UpdateRaftVoterRequest
        83 => Some((0, 0)), // InitializeShareGroupStateRequest
        84 => Some((0, 0)), // ReadShareGroupStateRequest
        85 => Some((0, 1)), // WriteShareGroupStateRequest
        86 => Some((0, 0)), // DeleteShareGroupStateRequest
        87 => Some((0, 1)), // ReadShareGroupStateSummaryRequest
        88 => Some((0, 0)), // StreamsGroupHeartbeatRequest
        89 => Some((0, 0)), // StreamsGroupDescribeRequest
        90 => Some((0, 1)), // DescribeShareGroupOffsetsRequest
        91 => Some((0, 0)), // AlterShareGroupOffsetsRequest
        92 => Some((0, 0)), // DeleteShareGroupOffsetsRequest
        _ => None,
    }
}

/// Supported (min, max) protocol versions for the RESPONSE of `api_key`.
/// None for unknown api keys and retired (version-less) APIs.
pub fn supported_response_versions(api_key: i16) -> Option<(i16, i16)> {
    match api_key {
        0 => Some((3, 13)), // ProduceResponse
        1 => Some((4, 18)), // FetchResponse
        2 => Some((1, 11)), // ListOffsetsResponse
        3 => Some((0, 13)), // MetadataResponse
        8 => Some((2, 10)), // OffsetCommitResponse
        9 => Some((1, 10)), // OffsetFetchResponse
        10 => Some((0, 6)), // FindCoordinatorResponse
        11 => Some((0, 9)), // JoinGroupResponse
        12 => Some((0, 4)), // HeartbeatResponse
        13 => Some((0, 5)), // LeaveGroupResponse
        14 => Some((0, 5)), // SyncGroupResponse
        15 => Some((0, 6)), // DescribeGroupsResponse
        16 => Some((0, 5)), // ListGroupsResponse
        17 => Some((0, 1)), // SaslHandshakeResponse
        18 => Some((0, 4)), // ApiVersionsResponse
        19 => Some((2, 7)), // CreateTopicsResponse
        20 => Some((1, 6)), // DeleteTopicsResponse
        21 => Some((0, 2)), // DeleteRecordsResponse
        22 => Some((0, 6)), // InitProducerIdResponse
        23 => Some((2, 4)), // OffsetForLeaderEpochResponse
        24 => Some((0, 5)), // AddPartitionsToTxnResponse
        25 => Some((0, 4)), // AddOffsetsToTxnResponse
        26 => Some((0, 5)), // EndTxnResponse
        27 => Some((1, 2)), // WriteTxnMarkersResponse
        28 => Some((0, 5)), // TxnOffsetCommitResponse
        29 => Some((1, 3)), // DescribeAclsResponse
        30 => Some((1, 3)), // CreateAclsResponse
        31 => Some((1, 3)), // DeleteAclsResponse
        32 => Some((1, 4)), // DescribeConfigsResponse
        33 => Some((0, 2)), // AlterConfigsResponse
        34 => Some((1, 2)), // AlterReplicaLogDirsResponse
        35 => Some((1, 5)), // DescribeLogDirsResponse
        36 => Some((0, 2)), // SaslAuthenticateResponse
        37 => Some((0, 3)), // CreatePartitionsResponse
        38 => Some((1, 3)), // CreateDelegationTokenResponse
        39 => Some((1, 2)), // RenewDelegationTokenResponse
        40 => Some((1, 2)), // ExpireDelegationTokenResponse
        41 => Some((1, 3)), // DescribeDelegationTokenResponse
        42 => Some((0, 2)), // DeleteGroupsResponse
        43 => Some((0, 2)), // ElectLeadersResponse
        44 => Some((0, 1)), // IncrementalAlterConfigsResponse
        45 => Some((0, 1)), // AlterPartitionReassignmentsResponse
        46 => Some((0, 0)), // ListPartitionReassignmentsResponse
        47 => Some((0, 0)), // OffsetDeleteResponse
        48 => Some((0, 1)), // DescribeClientQuotasResponse
        49 => Some((0, 1)), // AlterClientQuotasResponse
        50 => Some((0, 0)), // DescribeUserScramCredentialsResponse
        51 => Some((0, 0)), // AlterUserScramCredentialsResponse
        52 => Some((0, 2)), // VoteResponse
        53 => Some((0, 1)), // BeginQuorumEpochResponse
        54 => Some((0, 1)), // EndQuorumEpochResponse
        55 => Some((0, 2)), // DescribeQuorumResponse
        56 => Some((2, 3)), // AlterPartitionResponse
        57 => Some((0, 2)), // UpdateFeaturesResponse
        58 => Some((0, 0)), // EnvelopeResponse
        59 => Some((0, 1)), // FetchSnapshotResponse
        60 => Some((0, 2)), // DescribeClusterResponse
        61 => Some((0, 0)), // DescribeProducersResponse
        62 => Some((0, 4)), // BrokerRegistrationResponse
        63 => Some((0, 2)), // BrokerHeartbeatResponse
        64 => Some((0, 0)), // UnregisterBrokerResponse
        65 => Some((0, 0)), // DescribeTransactionsResponse
        66 => Some((0, 2)), // ListTransactionsResponse
        67 => Some((0, 0)), // AllocateProducerIdsResponse
        68 => Some((0, 1)), // ConsumerGroupHeartbeatResponse
        69 => Some((0, 1)), // ConsumerGroupDescribeResponse
        70 => Some((0, 0)), // ControllerRegistrationResponse
        71 => Some((0, 0)), // GetTelemetrySubscriptionsResponse
        72 => Some((0, 0)), // PushTelemetryResponse
        73 => Some((0, 0)), // AssignReplicasToDirsResponse
        74 => Some((0, 1)), // ListConfigResourcesResponse
        75 => Some((0, 0)), // DescribeTopicPartitionsResponse
        76 => Some((1, 1)), // ShareGroupHeartbeatResponse
        77 => Some((1, 1)), // ShareGroupDescribeResponse
        78 => Some((1, 2)), // ShareFetchResponse
        79 => Some((1, 2)), // ShareAcknowledgeResponse
        80 => Some((0, 1)), // AddRaftVoterResponse
        81 => Some((0, 0)), // RemoveRaftVoterResponse
        82 => Some((0, 0)), // UpdateRaftVoterResponse
        83 => Some((0, 0)), // InitializeShareGroupStateResponse
        84 => Some((0, 0)), // ReadShareGroupStateResponse
        85 => Some((0, 1)), // WriteShareGroupStateResponse
        86 => Some((0, 0)), // DeleteShareGroupStateResponse
        87 => Some((0, 1)), // ReadShareGroupStateSummaryResponse
        88 => Some((0, 0)), // StreamsGroupHeartbeatResponse
        89 => Some((0, 0)), // StreamsGroupDescribeResponse
        90 => Some((0, 1)), // DescribeShareGroupOffsetsResponse
        91 => Some((0, 0)), // AlterShareGroupOffsetsResponse
        92 => Some((0, 0)), // DeleteShareGroupOffsetsResponse
        _ => None,
    }
}

/// Header version for a request of `api_key` at `api_version`.
/// Returns None for unknown api keys.
pub fn request_header_version(api_key: i16, api_version: i16) -> Option<i16> {
    match api_key {
        0 => Some(if api_version >= 9 { 2 } else { 1 }), // ProduceRequest
        1 => Some(if api_version >= 12 { 2 } else { 1 }), // FetchRequest
        2 => Some(if api_version >= 6 { 2 } else { 1 }), // ListOffsetsRequest
        3 => Some(if api_version >= 9 { 2 } else { 1 }), // MetadataRequest
        4 => Some(if false { 2 } else { 1 }), // LeaderAndIsrRequest
        5 => Some(if false { 2 } else { 1 }), // StopReplicaRequest
        6 => Some(if false { 2 } else { 1 }), // UpdateMetadataRequest
        7 => Some(if api_version == 0 { 0 } else if false { 2 } else { 1 }), // ControlledShutdownRequest
        8 => Some(if api_version >= 8 { 2 } else { 1 }), // OffsetCommitRequest
        9 => Some(if api_version >= 6 { 2 } else { 1 }), // OffsetFetchRequest
        10 => Some(if api_version >= 3 { 2 } else { 1 }), // FindCoordinatorRequest
        11 => Some(if api_version >= 6 { 2 } else { 1 }), // JoinGroupRequest
        12 => Some(if api_version >= 4 { 2 } else { 1 }), // HeartbeatRequest
        13 => Some(if api_version >= 4 { 2 } else { 1 }), // LeaveGroupRequest
        14 => Some(if api_version >= 4 { 2 } else { 1 }), // SyncGroupRequest
        15 => Some(if api_version >= 5 { 2 } else { 1 }), // DescribeGroupsRequest
        16 => Some(if api_version >= 3 { 2 } else { 1 }), // ListGroupsRequest
        17 => Some(if false { 2 } else { 1 }), // SaslHandshakeRequest
        18 => Some(if api_version >= 3 { 2 } else { 1 }), // ApiVersionsRequest
        19 => Some(if api_version >= 5 { 2 } else { 1 }), // CreateTopicsRequest
        20 => Some(if api_version >= 4 { 2 } else { 1 }), // DeleteTopicsRequest
        21 => Some(if api_version >= 2 { 2 } else { 1 }), // DeleteRecordsRequest
        22 => Some(if api_version >= 2 { 2 } else { 1 }), // InitProducerIdRequest
        23 => Some(if api_version >= 4 { 2 } else { 1 }), // OffsetForLeaderEpochRequest
        24 => Some(if api_version >= 3 { 2 } else { 1 }), // AddPartitionsToTxnRequest
        25 => Some(if api_version >= 3 { 2 } else { 1 }), // AddOffsetsToTxnRequest
        26 => Some(if api_version >= 3 { 2 } else { 1 }), // EndTxnRequest
        27 => Some(if api_version >= 1 { 2 } else { 1 }), // WriteTxnMarkersRequest
        28 => Some(if api_version >= 3 { 2 } else { 1 }), // TxnOffsetCommitRequest
        29 => Some(if api_version >= 2 { 2 } else { 1 }), // DescribeAclsRequest
        30 => Some(if api_version >= 2 { 2 } else { 1 }), // CreateAclsRequest
        31 => Some(if api_version >= 2 { 2 } else { 1 }), // DeleteAclsRequest
        32 => Some(if api_version >= 4 { 2 } else { 1 }), // DescribeConfigsRequest
        33 => Some(if api_version >= 2 { 2 } else { 1 }), // AlterConfigsRequest
        34 => Some(if api_version >= 2 { 2 } else { 1 }), // AlterReplicaLogDirsRequest
        35 => Some(if api_version >= 2 { 2 } else { 1 }), // DescribeLogDirsRequest
        36 => Some(if api_version >= 2 { 2 } else { 1 }), // SaslAuthenticateRequest
        37 => Some(if api_version >= 2 { 2 } else { 1 }), // CreatePartitionsRequest
        38 => Some(if api_version >= 2 { 2 } else { 1 }), // CreateDelegationTokenRequest
        39 => Some(if api_version >= 2 { 2 } else { 1 }), // RenewDelegationTokenRequest
        40 => Some(if api_version >= 2 { 2 } else { 1 }), // ExpireDelegationTokenRequest
        41 => Some(if api_version >= 2 { 2 } else { 1 }), // DescribeDelegationTokenRequest
        42 => Some(if api_version >= 2 { 2 } else { 1 }), // DeleteGroupsRequest
        43 => Some(if api_version >= 2 { 2 } else { 1 }), // ElectLeadersRequest
        44 => Some(if api_version >= 1 { 2 } else { 1 }), // IncrementalAlterConfigsRequest
        45 => Some(if true { 2 } else { 1 }), // AlterPartitionReassignmentsRequest
        46 => Some(if true { 2 } else { 1 }), // ListPartitionReassignmentsRequest
        47 => Some(if false { 2 } else { 1 }), // OffsetDeleteRequest
        48 => Some(if api_version >= 1 { 2 } else { 1 }), // DescribeClientQuotasRequest
        49 => Some(if api_version >= 1 { 2 } else { 1 }), // AlterClientQuotasRequest
        50 => Some(if true { 2 } else { 1 }), // DescribeUserScramCredentialsRequest
        51 => Some(if true { 2 } else { 1 }), // AlterUserScramCredentialsRequest
        52 => Some(if true { 2 } else { 1 }), // VoteRequest
        53 => Some(if api_version >= 1 { 2 } else { 1 }), // BeginQuorumEpochRequest
        54 => Some(if api_version >= 1 { 2 } else { 1 }), // EndQuorumEpochRequest
        55 => Some(if true { 2 } else { 1 }), // DescribeQuorumRequest
        56 => Some(if true { 2 } else { 1 }), // AlterPartitionRequest
        57 => Some(if true { 2 } else { 1 }), // UpdateFeaturesRequest
        58 => Some(if true { 2 } else { 1 }), // EnvelopeRequest
        59 => Some(if true { 2 } else { 1 }), // FetchSnapshotRequest
        60 => Some(if true { 2 } else { 1 }), // DescribeClusterRequest
        61 => Some(if true { 2 } else { 1 }), // DescribeProducersRequest
        62 => Some(if true { 2 } else { 1 }), // BrokerRegistrationRequest
        63 => Some(if true { 2 } else { 1 }), // BrokerHeartbeatRequest
        64 => Some(if true { 2 } else { 1 }), // UnregisterBrokerRequest
        65 => Some(if true { 2 } else { 1 }), // DescribeTransactionsRequest
        66 => Some(if true { 2 } else { 1 }), // ListTransactionsRequest
        67 => Some(if true { 2 } else { 1 }), // AllocateProducerIdsRequest
        68 => Some(if true { 2 } else { 1 }), // ConsumerGroupHeartbeatRequest
        69 => Some(if true { 2 } else { 1 }), // ConsumerGroupDescribeRequest
        70 => Some(if true { 2 } else { 1 }), // ControllerRegistrationRequest
        71 => Some(if true { 2 } else { 1 }), // GetTelemetrySubscriptionsRequest
        72 => Some(if true { 2 } else { 1 }), // PushTelemetryRequest
        73 => Some(if true { 2 } else { 1 }), // AssignReplicasToDirsRequest
        74 => Some(if true { 2 } else { 1 }), // ListConfigResourcesRequest
        75 => Some(if true { 2 } else { 1 }), // DescribeTopicPartitionsRequest
        76 => Some(if true { 2 } else { 1 }), // ShareGroupHeartbeatRequest
        77 => Some(if true { 2 } else { 1 }), // ShareGroupDescribeRequest
        78 => Some(if true { 2 } else { 1 }), // ShareFetchRequest
        79 => Some(if true { 2 } else { 1 }), // ShareAcknowledgeRequest
        80 => Some(if true { 2 } else { 1 }), // AddRaftVoterRequest
        81 => Some(if true { 2 } else { 1 }), // RemoveRaftVoterRequest
        82 => Some(if true { 2 } else { 1 }), // UpdateRaftVoterRequest
        83 => Some(if true { 2 } else { 1 }), // InitializeShareGroupStateRequest
        84 => Some(if true { 2 } else { 1 }), // ReadShareGroupStateRequest
        85 => Some(if true { 2 } else { 1 }), // WriteShareGroupStateRequest
        86 => Some(if true { 2 } else { 1 }), // DeleteShareGroupStateRequest
        87 => Some(if true { 2 } else { 1 }), // ReadShareGroupStateSummaryRequest
        88 => Some(if true { 2 } else { 1 }), // StreamsGroupHeartbeatRequest
        89 => Some(if true { 2 } else { 1 }), // StreamsGroupDescribeRequest
        90 => Some(if true { 2 } else { 1 }), // DescribeShareGroupOffsetsRequest
        91 => Some(if true { 2 } else { 1 }), // AlterShareGroupOffsetsRequest
        92 => Some(if true { 2 } else { 1 }), // DeleteShareGroupOffsetsRequest
        _ => None,
    }
}

/// Header version for a response of `api_key` at `api_version`.
/// Returns None for unknown api keys.
pub fn response_header_version(api_key: i16, api_version: i16) -> Option<i16> {
    match api_key {
        0 => Some(if api_version >= 9 { 1 } else { 0 }), // ProduceResponse
        1 => Some(if api_version >= 12 { 1 } else { 0 }), // FetchResponse
        2 => Some(if api_version >= 6 { 1 } else { 0 }), // ListOffsetsResponse
        3 => Some(if api_version >= 9 { 1 } else { 0 }), // MetadataResponse
        4 => Some(if false { 1 } else { 0 }), // LeaderAndIsrResponse
        5 => Some(if false { 1 } else { 0 }), // StopReplicaResponse
        6 => Some(if false { 1 } else { 0 }), // UpdateMetadataResponse
        7 => Some(if false { 1 } else { 0 }), // ControlledShutdownResponse
        8 => Some(if api_version >= 8 { 1 } else { 0 }), // OffsetCommitResponse
        9 => Some(if api_version >= 6 { 1 } else { 0 }), // OffsetFetchResponse
        10 => Some(if api_version >= 3 { 1 } else { 0 }), // FindCoordinatorResponse
        11 => Some(if api_version >= 6 { 1 } else { 0 }), // JoinGroupResponse
        12 => Some(if api_version >= 4 { 1 } else { 0 }), // HeartbeatResponse
        13 => Some(if api_version >= 4 { 1 } else { 0 }), // LeaveGroupResponse
        14 => Some(if api_version >= 4 { 1 } else { 0 }), // SyncGroupResponse
        15 => Some(if api_version >= 5 { 1 } else { 0 }), // DescribeGroupsResponse
        16 => Some(if api_version >= 3 { 1 } else { 0 }), // ListGroupsResponse
        17 => Some(if false { 1 } else { 0 }), // SaslHandshakeResponse
        18 => Some(0), // ApiVersionsResponse — always header v0 (KIP-511)
        19 => Some(if api_version >= 5 { 1 } else { 0 }), // CreateTopicsResponse
        20 => Some(if api_version >= 4 { 1 } else { 0 }), // DeleteTopicsResponse
        21 => Some(if api_version >= 2 { 1 } else { 0 }), // DeleteRecordsResponse
        22 => Some(if api_version >= 2 { 1 } else { 0 }), // InitProducerIdResponse
        23 => Some(if api_version >= 4 { 1 } else { 0 }), // OffsetForLeaderEpochResponse
        24 => Some(if api_version >= 3 { 1 } else { 0 }), // AddPartitionsToTxnResponse
        25 => Some(if api_version >= 3 { 1 } else { 0 }), // AddOffsetsToTxnResponse
        26 => Some(if api_version >= 3 { 1 } else { 0 }), // EndTxnResponse
        27 => Some(if api_version >= 1 { 1 } else { 0 }), // WriteTxnMarkersResponse
        28 => Some(if api_version >= 3 { 1 } else { 0 }), // TxnOffsetCommitResponse
        29 => Some(if api_version >= 2 { 1 } else { 0 }), // DescribeAclsResponse
        30 => Some(if api_version >= 2 { 1 } else { 0 }), // CreateAclsResponse
        31 => Some(if api_version >= 2 { 1 } else { 0 }), // DeleteAclsResponse
        32 => Some(if api_version >= 4 { 1 } else { 0 }), // DescribeConfigsResponse
        33 => Some(if api_version >= 2 { 1 } else { 0 }), // AlterConfigsResponse
        34 => Some(if api_version >= 2 { 1 } else { 0 }), // AlterReplicaLogDirsResponse
        35 => Some(if api_version >= 2 { 1 } else { 0 }), // DescribeLogDirsResponse
        36 => Some(if api_version >= 2 { 1 } else { 0 }), // SaslAuthenticateResponse
        37 => Some(if api_version >= 2 { 1 } else { 0 }), // CreatePartitionsResponse
        38 => Some(if api_version >= 2 { 1 } else { 0 }), // CreateDelegationTokenResponse
        39 => Some(if api_version >= 2 { 1 } else { 0 }), // RenewDelegationTokenResponse
        40 => Some(if api_version >= 2 { 1 } else { 0 }), // ExpireDelegationTokenResponse
        41 => Some(if api_version >= 2 { 1 } else { 0 }), // DescribeDelegationTokenResponse
        42 => Some(if api_version >= 2 { 1 } else { 0 }), // DeleteGroupsResponse
        43 => Some(if api_version >= 2 { 1 } else { 0 }), // ElectLeadersResponse
        44 => Some(if api_version >= 1 { 1 } else { 0 }), // IncrementalAlterConfigsResponse
        45 => Some(if true { 1 } else { 0 }), // AlterPartitionReassignmentsResponse
        46 => Some(if true { 1 } else { 0 }), // ListPartitionReassignmentsResponse
        47 => Some(if false { 1 } else { 0 }), // OffsetDeleteResponse
        48 => Some(if api_version >= 1 { 1 } else { 0 }), // DescribeClientQuotasResponse
        49 => Some(if api_version >= 1 { 1 } else { 0 }), // AlterClientQuotasResponse
        50 => Some(if true { 1 } else { 0 }), // DescribeUserScramCredentialsResponse
        51 => Some(if true { 1 } else { 0 }), // AlterUserScramCredentialsResponse
        52 => Some(if true { 1 } else { 0 }), // VoteResponse
        53 => Some(if api_version >= 1 { 1 } else { 0 }), // BeginQuorumEpochResponse
        54 => Some(if api_version >= 1 { 1 } else { 0 }), // EndQuorumEpochResponse
        55 => Some(if true { 1 } else { 0 }), // DescribeQuorumResponse
        56 => Some(if true { 1 } else { 0 }), // AlterPartitionResponse
        57 => Some(if true { 1 } else { 0 }), // UpdateFeaturesResponse
        58 => Some(if true { 1 } else { 0 }), // EnvelopeResponse
        59 => Some(if true { 1 } else { 0 }), // FetchSnapshotResponse
        60 => Some(if true { 1 } else { 0 }), // DescribeClusterResponse
        61 => Some(if true { 1 } else { 0 }), // DescribeProducersResponse
        62 => Some(if true { 1 } else { 0 }), // BrokerRegistrationResponse
        63 => Some(if true { 1 } else { 0 }), // BrokerHeartbeatResponse
        64 => Some(if true { 1 } else { 0 }), // UnregisterBrokerResponse
        65 => Some(if true { 1 } else { 0 }), // DescribeTransactionsResponse
        66 => Some(if true { 1 } else { 0 }), // ListTransactionsResponse
        67 => Some(if true { 1 } else { 0 }), // AllocateProducerIdsResponse
        68 => Some(if true { 1 } else { 0 }), // ConsumerGroupHeartbeatResponse
        69 => Some(if true { 1 } else { 0 }), // ConsumerGroupDescribeResponse
        70 => Some(if true { 1 } else { 0 }), // ControllerRegistrationResponse
        71 => Some(if true { 1 } else { 0 }), // GetTelemetrySubscriptionsResponse
        72 => Some(if true { 1 } else { 0 }), // PushTelemetryResponse
        73 => Some(if true { 1 } else { 0 }), // AssignReplicasToDirsResponse
        74 => Some(if true { 1 } else { 0 }), // ListConfigResourcesResponse
        75 => Some(if true { 1 } else { 0 }), // DescribeTopicPartitionsResponse
        76 => Some(if true { 1 } else { 0 }), // ShareGroupHeartbeatResponse
        77 => Some(if true { 1 } else { 0 }), // ShareGroupDescribeResponse
        78 => Some(if true { 1 } else { 0 }), // ShareFetchResponse
        79 => Some(if true { 1 } else { 0 }), // ShareAcknowledgeResponse
        80 => Some(if true { 1 } else { 0 }), // AddRaftVoterResponse
        81 => Some(if true { 1 } else { 0 }), // RemoveRaftVoterResponse
        82 => Some(if true { 1 } else { 0 }), // UpdateRaftVoterResponse
        83 => Some(if true { 1 } else { 0 }), // InitializeShareGroupStateResponse
        84 => Some(if true { 1 } else { 0 }), // ReadShareGroupStateResponse
        85 => Some(if true { 1 } else { 0 }), // WriteShareGroupStateResponse
        86 => Some(if true { 1 } else { 0 }), // DeleteShareGroupStateResponse
        87 => Some(if true { 1 } else { 0 }), // ReadShareGroupStateSummaryResponse
        88 => Some(if true { 1 } else { 0 }), // StreamsGroupHeartbeatResponse
        89 => Some(if true { 1 } else { 0 }), // StreamsGroupDescribeResponse
        90 => Some(if true { 1 } else { 0 }), // DescribeShareGroupOffsetsResponse
        91 => Some(if true { 1 } else { 0 }), // AlterShareGroupOffsetsResponse
        92 => Some(if true { 1 } else { 0 }), // DeleteShareGroupOffsetsResponse
        _ => None,
    }
}

/// Decode `body` as the message identified by (api_key, is_request) at
/// `version`, assert the whole body was consumed, then re-encode and return
/// the produced bytes. The compat test compares these against the input.
pub fn roundtrip(api_key: i16, is_request: bool, version: i16, body: &Bytes)
    -> Result<Bytes, DecodeError>
{
    let mut buf = body.clone();
    let encoded = match (api_key, is_request) {
        (0, true) => {
            let m = super::produce_request::ProduceRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (0, false) => {
            let m = super::produce_response::ProduceResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (1, true) => {
            let m = super::fetch_request::FetchRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (1, false) => {
            let m = super::fetch_response::FetchResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (2, true) => {
            let m = super::list_offsets_request::ListOffsetsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (2, false) => {
            let m = super::list_offsets_response::ListOffsetsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (3, true) => {
            let m = super::metadata_request::MetadataRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (3, false) => {
            let m = super::metadata_response::MetadataResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (4, true) => {
            let m = super::leader_and_isr_request::LeaderAndIsrRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (4, false) => {
            let m = super::leader_and_isr_response::LeaderAndIsrResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (5, true) => {
            let m = super::stop_replica_request::StopReplicaRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (5, false) => {
            let m = super::stop_replica_response::StopReplicaResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (6, true) => {
            let m = super::update_metadata_request::UpdateMetadataRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (6, false) => {
            let m = super::update_metadata_response::UpdateMetadataResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (7, true) => {
            let m = super::controlled_shutdown_request::ControlledShutdownRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (7, false) => {
            let m = super::controlled_shutdown_response::ControlledShutdownResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (8, true) => {
            let m = super::offset_commit_request::OffsetCommitRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (8, false) => {
            let m = super::offset_commit_response::OffsetCommitResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (9, true) => {
            let m = super::offset_fetch_request::OffsetFetchRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (9, false) => {
            let m = super::offset_fetch_response::OffsetFetchResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (10, true) => {
            let m = super::find_coordinator_request::FindCoordinatorRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (10, false) => {
            let m = super::find_coordinator_response::FindCoordinatorResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (11, true) => {
            let m = super::join_group_request::JoinGroupRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (11, false) => {
            let m = super::join_group_response::JoinGroupResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (12, true) => {
            let m = super::heartbeat_request::HeartbeatRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (12, false) => {
            let m = super::heartbeat_response::HeartbeatResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (13, true) => {
            let m = super::leave_group_request::LeaveGroupRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (13, false) => {
            let m = super::leave_group_response::LeaveGroupResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (14, true) => {
            let m = super::sync_group_request::SyncGroupRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (14, false) => {
            let m = super::sync_group_response::SyncGroupResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (15, true) => {
            let m = super::describe_groups_request::DescribeGroupsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (15, false) => {
            let m = super::describe_groups_response::DescribeGroupsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (16, true) => {
            let m = super::list_groups_request::ListGroupsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (16, false) => {
            let m = super::list_groups_response::ListGroupsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (17, true) => {
            let m = super::sasl_handshake_request::SaslHandshakeRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (17, false) => {
            let m = super::sasl_handshake_response::SaslHandshakeResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (18, true) => {
            let m = super::api_versions_request::ApiVersionsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (18, false) => {
            let m = super::api_versions_response::ApiVersionsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (19, true) => {
            let m = super::create_topics_request::CreateTopicsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (19, false) => {
            let m = super::create_topics_response::CreateTopicsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (20, true) => {
            let m = super::delete_topics_request::DeleteTopicsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (20, false) => {
            let m = super::delete_topics_response::DeleteTopicsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (21, true) => {
            let m = super::delete_records_request::DeleteRecordsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (21, false) => {
            let m = super::delete_records_response::DeleteRecordsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (22, true) => {
            let m = super::init_producer_id_request::InitProducerIdRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (22, false) => {
            let m = super::init_producer_id_response::InitProducerIdResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (23, true) => {
            let m = super::offset_for_leader_epoch_request::OffsetForLeaderEpochRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (23, false) => {
            let m = super::offset_for_leader_epoch_response::OffsetForLeaderEpochResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (24, true) => {
            let m = super::add_partitions_to_txn_request::AddPartitionsToTxnRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (24, false) => {
            let m = super::add_partitions_to_txn_response::AddPartitionsToTxnResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (25, true) => {
            let m = super::add_offsets_to_txn_request::AddOffsetsToTxnRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (25, false) => {
            let m = super::add_offsets_to_txn_response::AddOffsetsToTxnResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (26, true) => {
            let m = super::end_txn_request::EndTxnRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (26, false) => {
            let m = super::end_txn_response::EndTxnResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (27, true) => {
            let m = super::write_txn_markers_request::WriteTxnMarkersRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (27, false) => {
            let m = super::write_txn_markers_response::WriteTxnMarkersResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (28, true) => {
            let m = super::txn_offset_commit_request::TxnOffsetCommitRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (28, false) => {
            let m = super::txn_offset_commit_response::TxnOffsetCommitResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (29, true) => {
            let m = super::describe_acls_request::DescribeAclsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (29, false) => {
            let m = super::describe_acls_response::DescribeAclsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (30, true) => {
            let m = super::create_acls_request::CreateAclsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (30, false) => {
            let m = super::create_acls_response::CreateAclsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (31, true) => {
            let m = super::delete_acls_request::DeleteAclsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (31, false) => {
            let m = super::delete_acls_response::DeleteAclsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (32, true) => {
            let m = super::describe_configs_request::DescribeConfigsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (32, false) => {
            let m = super::describe_configs_response::DescribeConfigsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (33, true) => {
            let m = super::alter_configs_request::AlterConfigsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (33, false) => {
            let m = super::alter_configs_response::AlterConfigsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (34, true) => {
            let m = super::alter_replica_log_dirs_request::AlterReplicaLogDirsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (34, false) => {
            let m = super::alter_replica_log_dirs_response::AlterReplicaLogDirsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (35, true) => {
            let m = super::describe_log_dirs_request::DescribeLogDirsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (35, false) => {
            let m = super::describe_log_dirs_response::DescribeLogDirsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (36, true) => {
            let m = super::sasl_authenticate_request::SaslAuthenticateRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (36, false) => {
            let m = super::sasl_authenticate_response::SaslAuthenticateResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (37, true) => {
            let m = super::create_partitions_request::CreatePartitionsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (37, false) => {
            let m = super::create_partitions_response::CreatePartitionsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (38, true) => {
            let m = super::create_delegation_token_request::CreateDelegationTokenRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (38, false) => {
            let m = super::create_delegation_token_response::CreateDelegationTokenResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (39, true) => {
            let m = super::renew_delegation_token_request::RenewDelegationTokenRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (39, false) => {
            let m = super::renew_delegation_token_response::RenewDelegationTokenResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (40, true) => {
            let m = super::expire_delegation_token_request::ExpireDelegationTokenRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (40, false) => {
            let m = super::expire_delegation_token_response::ExpireDelegationTokenResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (41, true) => {
            let m = super::describe_delegation_token_request::DescribeDelegationTokenRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (41, false) => {
            let m = super::describe_delegation_token_response::DescribeDelegationTokenResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (42, true) => {
            let m = super::delete_groups_request::DeleteGroupsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (42, false) => {
            let m = super::delete_groups_response::DeleteGroupsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (43, true) => {
            let m = super::elect_leaders_request::ElectLeadersRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (43, false) => {
            let m = super::elect_leaders_response::ElectLeadersResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (44, true) => {
            let m = super::incremental_alter_configs_request::IncrementalAlterConfigsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (44, false) => {
            let m = super::incremental_alter_configs_response::IncrementalAlterConfigsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (45, true) => {
            let m = super::alter_partition_reassignments_request::AlterPartitionReassignmentsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (45, false) => {
            let m = super::alter_partition_reassignments_response::AlterPartitionReassignmentsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (46, true) => {
            let m = super::list_partition_reassignments_request::ListPartitionReassignmentsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (46, false) => {
            let m = super::list_partition_reassignments_response::ListPartitionReassignmentsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (47, true) => {
            let m = super::offset_delete_request::OffsetDeleteRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (47, false) => {
            let m = super::offset_delete_response::OffsetDeleteResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (48, true) => {
            let m = super::describe_client_quotas_request::DescribeClientQuotasRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (48, false) => {
            let m = super::describe_client_quotas_response::DescribeClientQuotasResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (49, true) => {
            let m = super::alter_client_quotas_request::AlterClientQuotasRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (49, false) => {
            let m = super::alter_client_quotas_response::AlterClientQuotasResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (50, true) => {
            let m = super::describe_user_scram_credentials_request::DescribeUserScramCredentialsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (50, false) => {
            let m = super::describe_user_scram_credentials_response::DescribeUserScramCredentialsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (51, true) => {
            let m = super::alter_user_scram_credentials_request::AlterUserScramCredentialsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (51, false) => {
            let m = super::alter_user_scram_credentials_response::AlterUserScramCredentialsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (52, true) => {
            let m = super::vote_request::VoteRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (52, false) => {
            let m = super::vote_response::VoteResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (53, true) => {
            let m = super::begin_quorum_epoch_request::BeginQuorumEpochRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (53, false) => {
            let m = super::begin_quorum_epoch_response::BeginQuorumEpochResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (54, true) => {
            let m = super::end_quorum_epoch_request::EndQuorumEpochRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (54, false) => {
            let m = super::end_quorum_epoch_response::EndQuorumEpochResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (55, true) => {
            let m = super::describe_quorum_request::DescribeQuorumRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (55, false) => {
            let m = super::describe_quorum_response::DescribeQuorumResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (56, true) => {
            let m = super::alter_partition_request::AlterPartitionRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (56, false) => {
            let m = super::alter_partition_response::AlterPartitionResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (57, true) => {
            let m = super::update_features_request::UpdateFeaturesRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (57, false) => {
            let m = super::update_features_response::UpdateFeaturesResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (58, true) => {
            let m = super::envelope_request::EnvelopeRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (58, false) => {
            let m = super::envelope_response::EnvelopeResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (59, true) => {
            let m = super::fetch_snapshot_request::FetchSnapshotRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (59, false) => {
            let m = super::fetch_snapshot_response::FetchSnapshotResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (60, true) => {
            let m = super::describe_cluster_request::DescribeClusterRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (60, false) => {
            let m = super::describe_cluster_response::DescribeClusterResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (61, true) => {
            let m = super::describe_producers_request::DescribeProducersRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (61, false) => {
            let m = super::describe_producers_response::DescribeProducersResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (62, true) => {
            let m = super::broker_registration_request::BrokerRegistrationRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (62, false) => {
            let m = super::broker_registration_response::BrokerRegistrationResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (63, true) => {
            let m = super::broker_heartbeat_request::BrokerHeartbeatRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (63, false) => {
            let m = super::broker_heartbeat_response::BrokerHeartbeatResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (64, true) => {
            let m = super::unregister_broker_request::UnregisterBrokerRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (64, false) => {
            let m = super::unregister_broker_response::UnregisterBrokerResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (65, true) => {
            let m = super::describe_transactions_request::DescribeTransactionsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (65, false) => {
            let m = super::describe_transactions_response::DescribeTransactionsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (66, true) => {
            let m = super::list_transactions_request::ListTransactionsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (66, false) => {
            let m = super::list_transactions_response::ListTransactionsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (67, true) => {
            let m = super::allocate_producer_ids_request::AllocateProducerIdsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (67, false) => {
            let m = super::allocate_producer_ids_response::AllocateProducerIdsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (68, true) => {
            let m = super::consumer_group_heartbeat_request::ConsumerGroupHeartbeatRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (68, false) => {
            let m = super::consumer_group_heartbeat_response::ConsumerGroupHeartbeatResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (69, true) => {
            let m = super::consumer_group_describe_request::ConsumerGroupDescribeRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (69, false) => {
            let m = super::consumer_group_describe_response::ConsumerGroupDescribeResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (70, true) => {
            let m = super::controller_registration_request::ControllerRegistrationRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (70, false) => {
            let m = super::controller_registration_response::ControllerRegistrationResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (71, true) => {
            let m = super::get_telemetry_subscriptions_request::GetTelemetrySubscriptionsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (71, false) => {
            let m = super::get_telemetry_subscriptions_response::GetTelemetrySubscriptionsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (72, true) => {
            let m = super::push_telemetry_request::PushTelemetryRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (72, false) => {
            let m = super::push_telemetry_response::PushTelemetryResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (73, true) => {
            let m = super::assign_replicas_to_dirs_request::AssignReplicasToDirsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (73, false) => {
            let m = super::assign_replicas_to_dirs_response::AssignReplicasToDirsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (74, true) => {
            let m = super::list_config_resources_request::ListConfigResourcesRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (74, false) => {
            let m = super::list_config_resources_response::ListConfigResourcesResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (75, true) => {
            let m = super::describe_topic_partitions_request::DescribeTopicPartitionsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (75, false) => {
            let m = super::describe_topic_partitions_response::DescribeTopicPartitionsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (76, true) => {
            let m = super::share_group_heartbeat_request::ShareGroupHeartbeatRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (76, false) => {
            let m = super::share_group_heartbeat_response::ShareGroupHeartbeatResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (77, true) => {
            let m = super::share_group_describe_request::ShareGroupDescribeRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (77, false) => {
            let m = super::share_group_describe_response::ShareGroupDescribeResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (78, true) => {
            let m = super::share_fetch_request::ShareFetchRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (78, false) => {
            let m = super::share_fetch_response::ShareFetchResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (79, true) => {
            let m = super::share_acknowledge_request::ShareAcknowledgeRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (79, false) => {
            let m = super::share_acknowledge_response::ShareAcknowledgeResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (80, true) => {
            let m = super::add_raft_voter_request::AddRaftVoterRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (80, false) => {
            let m = super::add_raft_voter_response::AddRaftVoterResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (81, true) => {
            let m = super::remove_raft_voter_request::RemoveRaftVoterRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (81, false) => {
            let m = super::remove_raft_voter_response::RemoveRaftVoterResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (82, true) => {
            let m = super::update_raft_voter_request::UpdateRaftVoterRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (82, false) => {
            let m = super::update_raft_voter_response::UpdateRaftVoterResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (83, true) => {
            let m = super::initialize_share_group_state_request::InitializeShareGroupStateRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (83, false) => {
            let m = super::initialize_share_group_state_response::InitializeShareGroupStateResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (84, true) => {
            let m = super::read_share_group_state_request::ReadShareGroupStateRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (84, false) => {
            let m = super::read_share_group_state_response::ReadShareGroupStateResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (85, true) => {
            let m = super::write_share_group_state_request::WriteShareGroupStateRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (85, false) => {
            let m = super::write_share_group_state_response::WriteShareGroupStateResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (86, true) => {
            let m = super::delete_share_group_state_request::DeleteShareGroupStateRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (86, false) => {
            let m = super::delete_share_group_state_response::DeleteShareGroupStateResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (87, true) => {
            let m = super::read_share_group_state_summary_request::ReadShareGroupStateSummaryRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (87, false) => {
            let m = super::read_share_group_state_summary_response::ReadShareGroupStateSummaryResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (88, true) => {
            let m = super::streams_group_heartbeat_request::StreamsGroupHeartbeatRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (88, false) => {
            let m = super::streams_group_heartbeat_response::StreamsGroupHeartbeatResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (89, true) => {
            let m = super::streams_group_describe_request::StreamsGroupDescribeRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (89, false) => {
            let m = super::streams_group_describe_response::StreamsGroupDescribeResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (90, true) => {
            let m = super::describe_share_group_offsets_request::DescribeShareGroupOffsetsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (90, false) => {
            let m = super::describe_share_group_offsets_response::DescribeShareGroupOffsetsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (91, true) => {
            let m = super::alter_share_group_offsets_request::AlterShareGroupOffsetsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (91, false) => {
            let m = super::alter_share_group_offsets_response::AlterShareGroupOffsetsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (92, true) => {
            let m = super::delete_share_group_offsets_request::DeleteShareGroupOffsetsRequest::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        (92, false) => {
            let m = super::delete_share_group_offsets_response::DeleteShareGroupOffsetsResponse::decode(version, &mut buf)?;
            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect("just decoded at this version"));
            m.encode(version, &mut e).expect("just decoded at this version");
            e
        }
        _ => return Err(DecodeError::UnknownApiKey(api_key)),
    };
    if !buf.is_empty() {
        return Err(DecodeError::TrailingBytes { remaining: buf.len() });
    }
    Ok(encoded.freeze())
}
