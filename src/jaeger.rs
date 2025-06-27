use crate::models::*;
use chrono::DateTime;
use std::collections::HashMap;
use std::fs;

pub fn convert_to_jaeger(logs: &[LogEntry]) -> Result<Vec<JaegerTrace>, Box<dyn std::error::Error>> {
    let mut traces_map: HashMap<String, Vec<&LogEntry>> = HashMap::new();

    for log in logs {
        if let Some(request_id) = &log.request_id {
            traces_map.entry(request_id.clone()).or_default().push(log);
        }
    }

    let mut jaeger_traces = Vec::new();

    for (request_id, trace_logs) in traces_map {
        let trace_id = request_id.replace("-", "").replace("_", "");

        let mut spans = Vec::new();
        let mut processes = HashMap::new();

        for (i, log) in trace_logs.iter().enumerate() {
            let service_name = log.service_name.as_deref().unwrap_or("unknown");
            let process_id = format!("p{}", service_name);

            processes
                .entry(process_id.clone())
                .or_insert_with(|| JaegerProcess {
                    service_name: service_name.to_string(),
                    tags: vec![],
                });

            let start_time = parse_timestamp(&log.timestamp)?;

            let span = JaegerSpan {
                trace_id: trace_id.clone(),
                span_id: format!("{:016x}", i + 1),
                parent_span_id: if i > 0 {
                    Some(format!("{:016x}", i))
                } else {
                    None
                },
                operation_name: log.tag.as_deref().unwrap_or(&log.message).to_string(),
                start_time,
                duration: 1000,
                tags: create_tags(log),
                process_id,
            };

            spans.push(span);
        }

        jaeger_traces.push(JaegerTrace {
            trace_id,
            spans,
            processes,
        });
    }

    Ok(jaeger_traces)
}

fn parse_timestamp(timestamp: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let dt = DateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S%.f %z")?;
    Ok(dt.timestamp_micros())
}

fn create_tags(log: &LogEntry) -> Vec<JaegerTag> {
    let mut tags = vec![
        JaegerTag {
            key: "severity".to_string(),
            tag_type: "string".to_string(),
            value: log.severity.clone(),
        },
        JaegerTag {
            key: "message".to_string(),
            tag_type: "string".to_string(),
            value: log.message.clone(),
        },
    ];

    if let Some(err) = &log.err {
        tags.push(JaegerTag {
            key: "error".to_string(),
            tag_type: "bool".to_string(),
            value: "true".to_string(),
        });
        tags.push(JaegerTag {
            key: "error.message".to_string(),
            tag_type: "string".to_string(),
            value: err.clone(),
        });
    }

    tags
}

pub fn save_jaeger_json(traces: &[JaegerTrace]) {
    let submission = JaegerSubmission {
        data: traces.to_vec(),
    };

    match serde_json::to_string_pretty(&submission) {
        Ok(json) => match fs::write("jaeger_traces.json", json) {
            Ok(_) => println!("💾 Saved Jaeger traces to: jaeger_traces.json"),
            Err(e) => eprintln!("❌ Failed to save JSON: {}", e),
        },
        Err(e) => eprintln!("❌ Failed to serialize to JSON: {}", e),
    }
}