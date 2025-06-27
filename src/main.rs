use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct LogEntry {
    timestamp: String,
    severity: String,
    message: String,
    request_id: Option<String>,
    service_name: Option<String>,
    // เพิ่มฟิลด์อื่นๆ ได้ตามต้องการ
    err: Option<String>,
    tag: Option<String>,
}

// Jaeger format structures
#[derive(Debug, Serialize, Clone)]
struct JaegerTrace {
    #[serde(rename = "traceID")]
    trace_id: String,
    spans: Vec<JaegerSpan>,
    processes: HashMap<String, JaegerProcess>,
}

#[derive(Debug, Serialize, Clone)]
struct JaegerSpan {
    #[serde(rename = "traceID")]
    trace_id: String,
    #[serde(rename = "spanID")]
    span_id: String,
    #[serde(rename = "parentSpanID")]
    parent_span_id: Option<String>,
    #[serde(rename = "operationName")]
    operation_name: String,
    #[serde(rename = "startTime")]
    start_time: i64, // microseconds
    duration: i64, // microseconds
    tags: Vec<JaegerTag>,
    #[serde(rename = "processID")]
    process_id: String,
}

#[derive(Debug, Serialize, Clone)]
struct JaegerTag {
    key: String,
    #[serde(rename = "type")]
    tag_type: String,
    value: String,
}

#[derive(Debug, Serialize, Clone)]
struct JaegerProcess {
    #[serde(rename = "serviceName")]
    service_name: String,
    tags: Vec<JaegerTag>,
}

#[derive(Debug, Serialize)]
struct JaegerSubmission {
    data: Vec<JaegerTrace>,
}

fn main() {
    println!("🔍 Loading logs from file...");

    let file_path = "logs/logs.txt";

    match load_logs(file_path) {
        Ok(logs) => {
            println!("✅ Loaded {} log entries", logs.len());

            // แสดง log 3 รายการแรก
            println!("\n📝 First 3 entries:");
            for (i, log) in logs.iter().take(3).enumerate() {
                println!(
                    "{}. {} [{}] {}",
                    i + 1,
                    log.timestamp,
                    log.severity,
                    log.message
                );
            }

            // แปลงเป็น Jaeger format
            println!("\n🔄 Converting to Jaeger format...");
            match convert_to_jaeger(&logs) {
                Ok(traces) => {
                    println!("✅ Created {} traces", traces.len());

                    // แสดง trace แรก
                    if let Some(first_trace) = traces.first() {
                        println!("\n📊 First trace: {}", first_trace.trace_id);
                        println!("   Spans: {}", first_trace.spans.len());
                        for span in &first_trace.spans {
                            println!("   - {} [{}]", span.operation_name, span.process_id);
                        }
                    }

                    // บันทึกเป็น JSON file ก่อน (ก่อนจะส่งไป Jaeger)
                    save_jaeger_json(&traces);
                }
                Err(e) => {
                    eprintln!("❌ Error converting to Jaeger: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error loading logs: {}", e);
        }
    }
}

fn load_logs(file_path: &str) -> Result<Vec<LogEntry>, Box<dyn std::error::Error>> {
    // อ่านไฟล์ทั้งหมด
    let content = fs::read_to_string(file_path)?;

    let mut logs = Vec::new();

    // แปลงทีละบรรทัด
    for (line_num, line) in content.lines().enumerate() {
        // ข้าม empty lines
        if line.trim().is_empty() {
            continue;
        }

        // พยายาม parse JSON
        match serde_json::from_str::<LogEntry>(line) {
            Ok(log_entry) => {
                logs.push(log_entry);
            }
            Err(e) => {
                eprintln!("⚠️  Line {}: Failed to parse JSON - {}", line_num + 1, e);
                eprintln!("    Content: {}", line);
            }
        }
    }

    println!("📊 Parsed {} lines successfully", logs.len());

    Ok(logs)
}

fn convert_to_jaeger(logs: &[LogEntry]) -> Result<Vec<JaegerTrace>, Box<dyn std::error::Error>> {
    let mut traces_map: HashMap<String, Vec<&LogEntry>> = HashMap::new();

    // Group logs by request_id
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

            // เพิ่ม process ถ้ายังไม่มี
            processes
                .entry(process_id.clone())
                .or_insert_with(|| JaegerProcess {
                    service_name: service_name.to_string(),
                    tags: vec![],
                });

            // แปลง timestamp เป็น microseconds
            let start_time = parse_timestamp(&log.timestamp)?;

            // สร้าง span
            let span = JaegerSpan {
                trace_id: trace_id.clone(),
                span_id: format!("{:016x}", i + 1), // hex format
                parent_span_id: if i > 0 {
                    Some(format!("{:016x}", i))
                } else {
                    None
                },
                operation_name: log.tag.as_deref().unwrap_or(&log.message).to_string(),
                start_time,
                duration: 1000, // 1ms default
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
    // Parse "2025-06-28 14:32:15.123456 +0700" format
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

fn save_jaeger_json(traces: &[JaegerTrace]) {
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
