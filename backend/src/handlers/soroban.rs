use axum::{extract::State, response::Json, extract::Path};
use tracing::{info, debug};

use crate::config::AppState;
use crate::error::{AppError, Result};
use crate::services::soroban::{EventFilter, EventType as BackendEventType, Pagination, Topic};
use shared::dto::soroban::{
    MetricsResponse, ContractInfoResponse, SorobanHealthResponse, ListContractsResponse,
    QueryEventsRequest, QueryEventsResponse, EventType as SharedEventType, EventPagination,
    EventDto, GetEventsDto, CallContractFunctionRequest, CallContractFunctionResponse
};

/// Get Soroban service metrics
pub async fn metrics_handler(
    State(state): State<AppState>,
) -> Result<Json<MetricsResponse>> {
    info!("Soroban metrics request received");

    let manager = state
        .soroban_manager
        .as_ref()
        .ok_or_else(|| AppError::Config("Soroban manager not initialized".to_string()))?;

    let metrics = manager.get_metrics().await;

    Ok(Json(MetricsResponse {
        success: true,
        metrics: metrics.into(),
    }))
}

/// Get contract information
pub async fn contract_info_handler(
    State(state): State<AppState>,
    Path(contract_id): Path<String>,
) -> Result<Json<ContractInfoResponse>> {
    info!("Contract info request for: {}", contract_id);

    let manager = state
        .soroban_manager
        .as_ref()
        .ok_or_else(|| AppError::Config("Soroban manager not initialized".to_string()))?;

    let info = manager.get_contract_info(&contract_id).await?;

    Ok(Json(ContractInfoResponse {
        success: true,
        info: info.into(),
    }))
}

/// Get Soroban service health status
pub async fn soroban_health_handler(
    State(state): State<AppState>,
) -> Result<Json<SorobanHealthResponse>> {
    info!("Soroban health check request received");

    let manager = state
        .soroban_manager
        .as_ref()
        .ok_or_else(|| AppError::Config("Soroban manager not initialized".to_string()))?;

    let health = manager.health_check().await;

    Ok(Json(SorobanHealthResponse {
        success: true,
        health: health.into(),
    }))
}

/// List all registered contracts
pub async fn list_contracts_handler(
    State(state): State<AppState>,
) -> Result<Json<ListContractsResponse>> {
    info!("List contracts request received");

    let manager = state
        .soroban_manager
        .as_ref()
        .ok_or_else(|| AppError::Config("Soroban manager not initialized".to_string()))?;

    let contracts = manager.list_contracts().await;
    let count = contracts.len();

    Ok(Json(ListContractsResponse {
        success: true,
        contracts: contracts.into_iter().map(|c| c.into()).collect(),
        count,
    }))
}

/// Query contract events with filtering and pagination
pub async fn query_events_handler(
    State(state): State<AppState>,
    Json(request): Json<QueryEventsRequest>,
) -> Result<Json<QueryEventsResponse>> {
    info!("[HANDLER] Query events request - contract: {}", request.contract_id);
    debug!("[HANDLER] Filters: {:?}, Pagination: {:?}", request.filters.len(), request.pagination);

    let manager = state
        .soroban_manager
        .as_ref()
        .ok_or_else(|| AppError::Config("Soroban manager not initialized".to_string()))?;

    // Convert shared EventType to backend EventType
    fn convert_event_type(shared: SharedEventType) -> BackendEventType {
        match shared {
            SharedEventType::Contract => BackendEventType::Contract,
            SharedEventType::System => BackendEventType::System,
            SharedEventType::Diagnostic => BackendEventType::Diagnostic,
            SharedEventType::All => BackendEventType::All,
        }
    }

    // Convert shared pagination to backend pagination
    let pagination = match request.pagination {
        EventPagination::From { ledger } => Pagination::From(ledger),
        EventPagination::FromTo { start, end } => Pagination::FromTo(start, end),
        EventPagination::Cursor { cursor } => Pagination::Cursor(cursor),
    };

    // Convert shared filters to backend filters
    let filters: Vec<EventFilter> = request
        .filters
        .into_iter()
        .map(|filter_dto| {
            let mut filter = EventFilter::new(convert_event_type(filter_dto.event_type));

            // Add contract IDs
            for contract_id in filter_dto.contract_ids {
                filter = filter.contract(contract_id);
            }

            // Add topics (convert string topics to Topic enum)
            for topic_vec in filter_dto.topics {
                let topics: Vec<Topic> = topic_vec
                    .into_iter()
                    .map(|topic_str| {
                        if topic_str == "*" {
                            Topic::Any
                        } else if topic_str == "**" {
                            Topic::Greedy
                        } else {
                            // For now, treat as Any - in future, parse XDR ScVal
                            // TODO: Parse base64 XDR string to ScVal
                            Topic::Any
                        }
                    })
                    .collect();
                filter = filter.topic(topics);
            }

            filter
        })
        .collect();

    debug!("[HANDLER] Converted {} filters", filters.len());

    // Query events from manager
    let events_result = manager
        .query_events(&request.contract_id, pagination, filters, request.limit)
        .await?;

    info!("[HANDLER] ✅ Query events successful - {} events returned", events_result.events.len());

    // Convert backend response to shared DTO
    let events_dto = GetEventsDto {
        events: events_result
            .events
            .into_iter()
            .map(|event| EventDto {
                event_type: event.event_type,
                ledger: event.ledger,
                ledger_closed_at: event.ledger_closed_at,
                contract_id: event.contract_id,
                id: event.id,
                paging_token: event.paging_token,
                topic: event.topic,
                value: event.value,
                in_successful_contract_call: event.in_successful_contract_call,
                transaction_hash: event.transaction_hash,
            })
            .collect(),
        cursor: events_result.cursor,
        latest_ledger: events_result.latest_ledger,
        oldest_ledger: events_result.oldest_ledger,
        latest_ledger_close_time: events_result.latest_ledger_close_time,
        oldest_ledger_close_time: events_result.oldest_ledger_close_time,
    };

    Ok(Json(QueryEventsResponse {
        success: true,
        events: events_dto,
    }))
}

/// Simulate a transaction before submitting it to the network
///
/// This handler allows frontend to test transactions before actual submission,
/// enabling:
/// - Transaction validation without fees
/// - Accurate cost estimation
/// - Detection of required restorations
/// - Preview of transaction results and state changes
pub async fn simulate_transaction_handler(
    State(state): State<AppState>,
    Json(request): Json<shared::dto::soroban::SimulateTransactionRequest>,
) -> Result<Json<shared::dto::soroban::SimulateTransactionResponseDto>> {
    info!("[HANDLER] Simulate transaction request - contract: {}", request.contract_id);

    let manager = state.soroban_manager.as_ref()
        .ok_or_else(|| AppError::Config("Soroban manager not initialized".to_string()))?;

    // Convert shared SimulationOptions to backend SimulationOptions
    let backend_options = request.options.map(|opt| {
        crate::services::soroban::simulation::SimulationOptions {
            cpu_instructions: opt.cpu_instructions,
            auth_mode: opt.auth_mode.map(|mode| match mode {
                shared::dto::soroban::AuthModeDto::Enforce =>
                    crate::services::soroban::simulation::AuthMode::Enforce,
                shared::dto::soroban::AuthModeDto::Record =>
                    crate::services::soroban::simulation::AuthMode::Record,
            }),
        }
    });

    // Simulate transaction via manager
    let simulation_result = manager.simulate_transaction(
        &request.contract_id,
        &request.transaction_xdr,
        backend_options,
    ).await?;

    // Convert backend response to shared DTO
    let response_dto = shared::dto::soroban::SimulateTransactionResponseDto {
        success: simulation_result.is_success(),
        latest_ledger: simulation_result.latest_ledger,
        min_resource_fee: simulation_result.min_resource_fee.clone(),
        error: simulation_result.error.clone(),
        results: simulation_result.results.as_ref().map(|results| {
            results.iter().map(|r| shared::dto::soroban::SimulationResultDto {
                auth: r.auth.clone(),
                xdr: r.xdr.clone(),
            }).collect()
        }),
        transaction_data: simulation_result.transaction_data.clone(),
        restore_preamble: simulation_result.restore_preamble.as_ref().map(|rp| {
            shared::dto::soroban::RestorePreambleDto {
                min_resource_fee: rp.min_resource_fee.clone(),
                transaction_data: rp.transaction_data.clone(),
            }
        }),
        events: simulation_result.events.clone(),
        state_changes: simulation_result.state_changes.as_ref().map(|changes| {
            changes.iter().map(|sc| shared::dto::soroban::StateChangeDto {
                kind: match sc.kind {
                    crate::services::soroban::simulation::StateChangeKind::Created =>
                        shared::dto::soroban::StateChangeKindDto::Created,
                    crate::services::soroban::simulation::StateChangeKind::Updated =>
                        shared::dto::soroban::StateChangeKindDto::Updated,
                    crate::services::soroban::simulation::StateChangeKind::Deleted =>
                        shared::dto::soroban::StateChangeKindDto::Deleted,
                },
                key: sc.key.clone(),
                before: sc.before.clone(),
                after: sc.after.clone(),
            }).collect()
        }),
    };

    info!(
        "[HANDLER] ✅ Simulate transaction {} - fee: {:?}",
        if response_dto.success { "successful" } else { "failed" },
        response_dto.min_resource_fee
    );

    Ok(Json(response_dto))
}

/// Get contract storage data
///
/// This handler allows reading contract storage directly without executing transactions,
/// enabling:
/// - Read persistent storage (user data, game state)
/// - Read temporary storage (session data, cache)
/// - Verify contract state
/// - Build dashboards from real-time contract data
/// - No gas fees (read-only operation)
pub async fn get_contract_data_handler(
    State(state): State<AppState>,
    Json(request): Json<shared::dto::soroban::GetContractDataRequest>,
) -> Result<Json<shared::dto::soroban::GetContractDataResponse>> {
    info!(
        "[HANDLER] Get contract data request - contract: {}, durability: {:?}",
        request.contract_id, request.durability
    );

    let manager = state.soroban_manager.as_ref()
        .ok_or_else(|| AppError::Config("Soroban manager not initialized".to_string()))?;

    // Convert shared Durability to backend Durability
    let backend_durability = match request.durability {
        shared::dto::soroban::DurabilityDto::Temporary =>
            crate::services::soroban::state::Durability::Temporary,
        shared::dto::soroban::DurabilityDto::Persistent =>
            crate::services::soroban::state::Durability::Persistent,
    };

    // Get contract data via manager
    match manager.get_contract_data(
        &request.contract_id,
        &request.key,
        backend_durability,
    ).await {
        Ok(entry) => {
            // Convert backend LedgerEntryResult to shared DTO
            let entry_dto = shared::dto::soroban::LedgerEntryResultDto {
                last_modified_ledger_seq: entry.last_modified_ledger_seq,
                live_until_ledger_seq: entry.live_until_ledger_seq,
                key: entry.key.clone(),
                xdr: entry.xdr.clone(),
                ext_xdr: entry.ext_xdr.clone(),
            };

            info!("[HANDLER] ✅ Get contract data successful");

            Ok(Json(shared::dto::soroban::GetContractDataResponse {
                success: true,
                data: Some(entry_dto),
                error: None,
            }))
        }
        Err(e) => {
            info!("[HANDLER] ⚠️ Get contract data failed: {}", e);

            Ok(Json(shared::dto::soroban::GetContractDataResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            }))
        }
    }
}

/// Call a generic contract function (read-only via simulation)
///
/// This handler allows the frontend to call ANY Soroban contract function by:
/// - Accepting function name and typed parameters
/// - Building transaction XDR
/// - Simulating the transaction (no fees, read-only)
/// - Parsing the result from XDR to JSON
///
/// Use cases:
/// - Query Reflector Oracle prices: lastprice("BTC")
/// - Read any contract function without gas fees
/// - Test function calls before actual transactions
/// - Build real-time dashboards from contract data
pub async fn call_contract_function_handler(
    State(state): State<AppState>,
    Json(request): Json<CallContractFunctionRequest>,
) -> Result<Json<CallContractFunctionResponse>> {
    info!(
        "[HANDLER] Call contract function - contract: {}, function: {}",
        request.contract_id, request.function_name
    );
    debug!("[HANDLER] Parameters: {} params", request.parameters.len());

    let manager = state.soroban_manager.as_ref()
        .ok_or_else(|| AppError::Config("Soroban manager not initialized".to_string()))?;

    // Call contract function via manager
    let result = manager.call_contract_function(
        &request.contract_id,
        &request.function_name,
        request.parameters,
        request.source_account.as_deref(),
    ).await?;

    info!(
        "[HANDLER] ✅ Call contract function {} - result: {}",
        if result.success { "successful" } else { "failed" },
        result.result.as_ref().map(|r| format!("{:?}", r)).unwrap_or_else(|| "None".to_string())
    );

    Ok(Json(result))
}
