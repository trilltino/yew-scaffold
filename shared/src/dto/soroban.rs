use serde::{Deserialize, Serialize};

// Re-export types from backend for frontend use
// These match the backend service types exactly

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMetrics {
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub retried_operations: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub xdr_generated: u64,
    pub transactions_submitted: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub healthy: bool,
    pub total_contracts: usize,
    pub enabled_contracts: usize,
    pub total_operations: u64,
    pub failed_operations: u64,
    pub cache_hit_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    pub total_connections: usize,
    pub max_connections: usize,
    pub available: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerStats {
    pub state: CircuitState,
    pub failure_count: u32,
    pub success_count: u32,
    pub is_open: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub active_entries: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkType {
    Testnet,
    Mainnet,
    Futurenet,
    Standalone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMetadata {
    pub contract_id: String,
    pub name: String,
    pub network: NetworkType,
    pub network_passphrase: String,
    pub rpc_url: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    pub metadata: ContractMetadata,
    pub pool_stats: PoolStats,
    pub circuit_breaker_stats: CircuitBreakerStats,
    pub cache_stats: CacheStats,
}

// API Response types - these are what the frontend receives

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsResponse {
    pub success: bool,
    pub metrics: ContractMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfoResponse {
    pub success: bool,
    pub info: ContractInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SorobanHealthResponse {
    pub success: bool,
    pub health: HealthStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListContractsResponse {
    pub success: bool,
    pub contracts: Vec<ContractMetadata>,
    pub count: usize,
}

// ==================== EVENT QUERYING TYPES ====================

/// Event type filter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Contract,
    System,
    Diagnostic,
    All,
}

/// Pagination for event queries
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventPagination {
    /// Start from this ledger sequence (inclusive)
    From { ledger: u32 },
    /// Start and end ledger sequences (start inclusive, end exclusive)
    FromTo { start: u32, end: u32 },
    /// Continue from this cursor (from previous response)
    Cursor { cursor: String },
}

/// Simplified event filter for API requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilterDto {
    /// Event type to filter
    pub event_type: EventType,
    /// Optional contract IDs to filter (max 5)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub contract_ids: Vec<String>,
    /// Optional topic filters as base64 XDR strings
    /// Use "*" for any topic, "**" for greedy match
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub topics: Vec<Vec<String>>,
}

/// Individual event in response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDto {
    /// Event type
    #[serde(rename = "type")]
    pub event_type: String,
    /// Ledger sequence number
    pub ledger: u32,
    /// Unix timestamp of ledger close
    pub ledger_closed_at: String,
    /// Contract ID that emitted the event
    pub contract_id: String,
    /// Unique event ID (for pagination)
    pub id: String,
    /// Paging token for cursor-based pagination
    pub paging_token: String,
    /// Topic values (base64 encoded XDR)
    pub topic: Vec<String>,
    /// Event value (base64 encoded XDR)
    pub value: String,
    /// Whether event is in successful contract call
    pub in_successful_contract_call: bool,
    /// Transaction hash that emitted this event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
}

/// Response from get_events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventsDto {
    /// Events found for the filter
    pub events: Vec<EventDto>,
    /// Cursor for next page (if more events available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Latest ledger sequence at time of request
    pub latest_ledger: u64,
    /// Oldest ledger sequence in RPC storage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_ledger: Option<u64>,
    /// Unix timestamp of latest ledger close
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_ledger_close_time: Option<String>,
    /// Unix timestamp of oldest ledger close
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_ledger_close_time: Option<String>,
}

/// Request to query contract events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryEventsRequest {
    /// Contract ID to query events from
    pub contract_id: String,
    /// Pagination parameters
    pub pagination: EventPagination,
    /// Event filters (max 5)
    pub filters: Vec<EventFilterDto>,
    /// Optional result limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response from query events endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryEventsResponse {
    pub success: bool,
    pub events: GetEventsDto,
}

// ==================== TRANSACTION SIMULATION TYPES ====================

/// Simulation options for transaction testing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimulationOptionsDto {
    /// Allow this many extra CPU instructions when budgeting resources
    #[serde(default)]
    pub cpu_instructions: u64,

    /// Auth mode to apply to the simulation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_mode: Option<AuthModeDto>,
}

/// Auth mode for simulation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthModeDto {
    /// Always enforcement mode
    Enforce,
    /// Always recording mode
    Record,
}

/// Request to simulate a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulateTransactionRequest {
    /// Contract ID to simulate against
    pub contract_id: String,

    /// Base64-encoded transaction envelope XDR
    pub transaction_xdr: String,

    /// Optional simulation options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<SimulationOptionsDto>,
}

/// Response from transaction simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateTransactionResponseDto {
    /// Success status
    pub success: bool,

    /// Latest ledger at simulation time
    pub latest_ledger: u32,

    /// Recommended minimum resource fee (stringified number)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_resource_fee: Option<String>,

    /// Error message if simulation failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Results from host function invocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SimulationResultDto>>,

    /// Recommended Soroban transaction data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_data: Option<String>,

    /// Restoration preamble if archived entries need restoration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_preamble: Option<RestorePreambleDto>,

    /// Events emitted during simulation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,

    /// State changes that would occur
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_changes: Option<Vec<StateChangeDto>>,
}

/// Result from host function simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResultDto {
    /// Authorization entries (base64 XDR)
    pub auth: Vec<String>,

    /// Return value (base64 XDR)
    pub xdr: String,
}

/// Restoration preamble
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestorePreambleDto {
    /// Minimum resource fee for restoration
    pub min_resource_fee: String,

    /// Transaction data for restoration
    pub transaction_data: String,
}

/// State change from simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChangeDto {
    /// Type of change
    #[serde(rename = "type")]
    pub kind: StateChangeKindDto,

    /// Ledger key (base64 XDR)
    pub key: String,

    /// State before (base64 XDR)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// State after (base64 XDR)
    pub after: String,
}

/// Type of state change
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StateChangeKindDto {
    Created,
    Updated,
    Deleted,
}

// ==================== CONTRACT STATE QUERYING TYPES ====================

/// Storage durability for contract data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DurabilityDto {
    /// Temporary storage (short-lived, cannot be restored)
    Temporary,
    /// Persistent storage (long-lived, can be restored)
    Persistent,
}

/// Request to get contract storage data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContractDataRequest {
    /// Contract ID to query
    pub contract_id: String,

    /// Storage key (base64 XDR encoded ScVal)
    pub key: String,

    /// Storage durability type
    pub durability: DurabilityDto,
}

/// Ledger entry result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LedgerEntryResultDto {
    /// Ledger sequence when entry was last modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_ledger_seq: Option<u32>,

    /// Ledger sequence when entry will expire (TTL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_until_ledger_seq: Option<u32>,

    /// Ledger key (base64 XDR)
    pub key: String,

    /// Ledger entry data (base64 XDR)
    pub xdr: String,

    /// Extension data (base64 XDR)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_xdr: Option<String>,
}

/// Response from get contract data endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContractDataResponse {
    /// Success status
    pub success: bool,

    /// Ledger entry data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<LedgerEntryResultDto>,

    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ==================== GENERIC CONTRACT FUNCTION CALL TYPES ====================

/// Function parameter types for contract calls
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum FunctionParameter {
    /// String/Symbol parameter (e.g., "BTC", "EUR")
    Symbol(String),
    /// Unsigned 32-bit integer
    U32(u32),
    /// Unsigned 64-bit integer
    U64(u64),
    /// Signed 32-bit integer
    I32(i32),
    /// Signed 64-bit integer
    I64(i64),
    /// Boolean value
    Bool(bool),
    /// String value
    String(String),
    /// Address (Stellar account or contract)
    Address(String),
    /// Bytes (hex-encoded)
    Bytes(String),
    /// Vector of parameters
    Vec(Vec<FunctionParameter>),
    /// Enum variant (variant_name, optional value)
    /// E.g., for Asset::Other("BTC"), use Enum("Other", Some(Box::new(FunctionParameter::Symbol("BTC"))))
    /// E.g., for Option::None, use Enum("None", None)
    Enum(String, Option<Box<FunctionParameter>>),
}

/// Request to call a contract function (read-only via simulation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallContractFunctionRequest {
    /// Contract ID to call
    pub contract_id: String,

    /// Function name to invoke
    pub function_name: String,

    /// Function parameters (ordered)
    #[serde(default)]
    pub parameters: Vec<FunctionParameter>,

    /// Optional source account for the simulation
    /// If not provided, uses a default testnet account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<String>,
}

/// Response from contract function call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallContractFunctionResponse {
    /// Success status
    pub success: bool,

    /// Function return value (parsed from XDR)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,

    /// Raw XDR result (base64 encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_xdr: Option<String>,

    /// Simulation details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation: Option<SimulationDetailsDto>,

    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Simulation execution details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationDetailsDto {
    /// Latest ledger at simulation time
    pub latest_ledger: u32,

    /// Resource fee required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_resource_fee: Option<String>,

    /// CPU instructions used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_instructions: Option<u64>,

    /// Events emitted during simulation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
}
