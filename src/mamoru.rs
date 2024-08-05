use crate::component::guest::types::{Incident, ValueData};
use std::collections::HashMap;

/// Type alias for TransactionId, using String instead of u64 for more flexibility
type TransactionId = String;

// TODO remove this severity and apply wit components
/// Represents the severity level of an incident, where higher numbers indicate greater severity.
pub enum IncidentSeverity {
    Info = 0,
    Warning = 1,
    Error = 2,
    Alert = 3,
}

impl Incident {
    /// Creates a new `Incident` with an `Info` severity.
    ///
    /// Parameters:
    /// - `tx_id`: The transaction identifier.
    /// - `message`: Descriptive message about the incident.
    /// - `data`: Optional data related to the incident.
    /// - `address`: Optional address involved in the incident.
    ///
    /// Returns:
    /// An `Incident` instance with specified details.
    pub fn new_info(
        tx_id: TransactionId,
        message: &str,
        data: Option<ValueData>,
        address: Option<String>,
    ) -> Self {
        Incident {
            severity: IncidentSeverity::Info as u64,
            message: message.to_string(),
            tx_hash: tx_id,
            address: address.unwrap_or_default(),
            data,
        }
    }
}

/// Reports an incident by calling a function from the guest component.
pub fn report(incident: &Incident) {
    crate::component::guest::mamoru::report(incident);
}

/// Enumerates types of logs that can be generated.
pub enum LogType {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

/// Logs an incident with optional data and address.
///
/// Parameters:
/// - `tx_id`: Transaction identifier.
/// - `message`: Log message.
/// - `typ_`: Type of the log.
/// - `data`: Optional data related to the log.
/// - `address`: Optional address involved in the log.
pub fn log(
    tx_id: TransactionId,
    message: &str,
    typ_: Option<LogType>,
    data: Option<ValueData>,
    address: Option<String>,
) {
    let level = typ_.unwrap_or(LogType::INFO);
    let incident = Incident {
        severity: level as u64,
        message: message.to_string(),
        tx_hash: tx_id,
        address: address.unwrap_or_default(),
        data,
    };

    report(&incident);
}

//TODO pending to modify error
/// Generic function to try converting a static string input into a type `T`.
///
/// The function fetches the parameter and attempts to convert it from a `String`.
/// If the conversion fails, it returns an error with a descriptive message.
///
/// Parameters:
/// - `input`: The static string identifier for the parameter.
///
/// Returns:
/// A result containing either the converted value or an error string.
pub fn parameter<T: TryFrom<String, Error = String>>(input: &'static str) -> Result<T, String> {
    let inner_data = crate::component::guest::mamoru::parameter(input);
    T::try_from(inner_data).map_err(|e| format!("Error trying to parse from string {e}"))
}

/// Apply a SQL query via string for extracting mamoru info
///
///
///Parameters:
///- `query`: SQL query
//TODO Add not serialized types
pub fn query(_query: String) {}

/// Http method types
#[allow(dead_code)]
enum HttpMethod {
    Post,
    Get,
    Put,
    Patch,
    Delete,
}

/// Http requests
#[allow(dead_code)]
pub struct HttpRequest {
    method: HttpMethod,
    url: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

/// Http response
#[allow(dead_code)]
pub struct HttpResponse {
    status: u16,
    error: Option<String>,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

/// Basic HTTP requests function
///
/// Parameters:
/// `http_request`: HTTP request
///
/// Returns:
/// A `HttpResponse` response.
#[allow(dead_code)]
pub fn http(_http_request: HttpRequest) -> HttpResponse {
    HttpResponse {
        status: 200,
        error: None,
        headers: HashMap::new(),
        body: None,
    }
}

/// Convert from string to u256
///
/// Parameters:
/// - `payload`: String to convert in u256 via vector
/// Returns:
/// A vector of bytes that represents a u256
pub fn u256_from_str(_payload: String) -> Vec<u8> {
    Vec::new()
}
