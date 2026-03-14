# REAL-TIME MARKET DATA PIPELINE
A high-performance, asynchronous data pipeline built with **Rust** to ingest real-time Aggregate Trade data from the Binance WebSocket API. 

## 🚀 Key Features
- **Asynchronous Networking**: Built on `tokio` and `tokio-tungstenite` for non-blocking I/O.
- **Fast Deserialization**: Leverages `serde` and `serde_json` for high-speed mapping of JSON to internal Rust types.
- **Multithreaded Architecture**: Utilizes **MPSC (Multi-Producer, Single-Consumer) Channels** to decouple the data ingestion (Producer) from the data processing (Consumer).
- **Zero-Copy Philosophy**: Optimized for low-latency market data handling.

## 🛠️ Prerequisites
- **Rust** (Stable version)
- **Cargo** (Rust package manager)

## 📦 Installation & Execution
1. **Clone the repository:**
   ```bash
   git clone https://github.com/hnam06-rmxt/Market-Data-Pipeline
   cd Market-Data-Pipeline