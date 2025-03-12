# YouTube FactCheck Analyzer

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange)](https://www.rust-lang.org/)
[![Web Framework: Axum](https://img.shields.io/badge/Web%20Framework-Axum-blue)](https://github.com/tokio-rs/axum)

A Rust-powered backend service that analyzes YouTube videos using DeepSeek's AI to help verify claims and detect misinformation. Built with **Axum** for high-performance asynchronous routing.

## Features
- 🎥 YouTube video metadata analysis
- 🧠 DeepSeek AI-powered fact-checking
- 📝 Automated claim extraction and verification
- 🚀 Async API endpoints for processing requests
- 🔒 Secure API key configuration

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (v1.70+)
- [Cargo](https://doc.rust-lang.org/cargo/)
- API credentials:
    - DeepSeek API access

## Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/youtube-factcheck-analyzer.git
   cd youtube-factcheck-analyzer
   
2. **Create a .env file**
   ```bash
   DEEPSEEK_API_KEY=your_deepseek_key_here
   DEEPSEEK_API_ENDPOINT=your_deepseek_endpoint_url
   
3. **Build project**
   ```bash
   cargo build --release
   
4. **Run the server**
   ```bash
   cargo run --release
   
5. **Synthesize a YouTube video**
   ```bash
   curl -X GET http://localhost:3000/api/synthesize/VIDEO_ID