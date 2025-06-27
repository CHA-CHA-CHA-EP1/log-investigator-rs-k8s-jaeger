use observ::*;

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

