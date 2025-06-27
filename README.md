# Observ

A Rust-based log processing tool that converts log files to structured JSON format for observability and tracing analysis.

## Features

### Current Features
- 📄 **Log File Reader** - Read and parse log files from local filesystem
- 🔄 **JSON Converter** - Transform log entries into structured JSON format
- 🎯 **Pattern Matching** - Extract structured data from unstructured log lines
- ⚡ **High Performance** - Rust-powered fast log processing
- 🔧 **Configurable Parsing** - Support multiple log formats and patterns

### Roadmap Features
- 🚢 **Kubernetes Integration** - Direct kubectl service log access
- 🌊 **Real-time Streaming** - Live log processing and conversion
- 📊 **Jaeger Integration** - Direct export to Jaeger for distributed tracing
- 🖥️ **CLI Interface** - Interactive service selection and log streaming
- 🔌 **Plugin Architecture** - Extensible log format support
- 📡 **Remote Log Sources** - Support for various log endpoints
- 📈 **Metrics Collection** - Performance and processing metrics

## Installation

```bash
# Clone the repository
git clone https://github.com/your-username/observ.git
cd observ

# Build the project
cargo build --release

# Run
./target/release/observ
```

## Usage

### Basic Log Conversion
```bash
# Convert log file to JSON
observ convert --input app.log --output app.json

# Process with specific format
observ convert --input app.log --format nginx --output structured.json
```

### Future CLI Usage (Roadmap)
```bash
# List available services
observ k8s list

# Stream logs from specific service
observ k8s stream --service my-app --namespace default

# Stream directly to Jaeger
observ k8s stream --service my-app --jaeger-endpoint http://jaeger:14268
```

## Configuration

```toml
# observ.toml
[parser]
format = "custom"
timestamp_format = "%Y-%m-%d %H:%M:%S"
fields = ["timestamp", "level", "message", "trace_id"]

[output]
format = "json"
pretty = true

[jaeger]
endpoint = "http://localhost:14268"
service_name = "observ"
```

## Supported Log Formats

- [x] Generic text logs
- [x] JSON logs
- [ ] Nginx access logs
- [ ] Apache logs
- [ ] Kubernetes container logs
- [ ] Application-specific formats

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Architecture

```
┌─────────────┐    ┌──────────────┐    ┌─────────────┐    ┌─────────────┐
│ Log Sources │ -> │ Log Parser   │ -> │ JSON        │ -> │ Jaeger      │
│             │    │              │    │ Converter   │    │ Export      │
│ • Files     │    │ • Patterns   │    │             │    │             │
│ • K8s Pods  │    │ • Regex      │    │ • Structure │    │ • Tracing   │
│ • Streams   │    │ • Formats    │    │ • Metadata  │    │ • Analysis  │
└─────────────┘    └──────────────┘    └─────────────┘    └─────────────┘
```

## Performance

- **Processing Speed**: ~100MB/s log processing
- **Memory Usage**: Low memory footprint with streaming
- **Concurrency**: Multi-threaded processing support
- **Scalability**: Handles large log files efficiently