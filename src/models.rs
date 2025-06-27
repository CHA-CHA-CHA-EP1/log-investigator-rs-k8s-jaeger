use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub severity: String,
    pub message: String,
    pub request_id: Option<String>,
    pub service_name: Option<String>,
    pub err: Option<String>,
    pub tag: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct JaegerTrace {
    #[serde(rename = "traceID")]
    pub trace_id: String,
    pub spans: Vec<JaegerSpan>,
    pub processes: HashMap<String, JaegerProcess>,
}

#[derive(Debug, Serialize, Clone)]
pub struct JaegerSpan {
    #[serde(rename = "traceID")]
    pub trace_id: String,
    #[serde(rename = "spanID")]
    pub span_id: String,
    #[serde(rename = "parentSpanID")]
    pub parent_span_id: Option<String>,
    #[serde(rename = "operationName")]
    pub operation_name: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    pub duration: i64,
    pub tags: Vec<JaegerTag>,
    #[serde(rename = "processID")]
    pub process_id: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct JaegerTag {
    pub key: String,
    #[serde(rename = "type")]
    pub tag_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct JaegerProcess {
    #[serde(rename = "serviceName")]
    pub service_name: String,
    pub tags: Vec<JaegerTag>,
}

#[derive(Debug, Serialize)]
pub struct JaegerSubmission {
    pub data: Vec<JaegerTrace>,
}