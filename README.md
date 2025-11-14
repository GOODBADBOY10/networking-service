
## 🧱 Microservice Echo System — Level 1

# This project demonstrates a simple two-service microservice setup using Rust, Axum, and Reqwest, where one service makes an HTTP call to another.

## 🚀 Overview

# This system contains two small Axum services:

# Service A (Client)

Exposes: POST /ping

Makes an HTTP request to Service B using Reqwest

Returns the response from Service B back to the client

Service B (Server)

Exposes: GET /pong

Returns a JSON response:

{ "message": "PONG from Service B" }

# 🧩 How It Works
Client → POST /ping
          ↓
     Service A
          ↓ calls
     Service B (/pong)
          ↓ returns JSON
     Service A
          ↓ responds
Client receives final response

# 🛠 What You Learn

How to make async HTTP requests inside Axum routes

How to use Reqwest to call another microservice

How to send and receive JSON between services

How to handle network errors and timeouts

(Bonus) How to use tracing logs to observe inter-service communication

# 📦 Running the Services
1. Start Service B
cd service-b
cargo run

2. Start Service A
cd service-a
cargo run

🧪 Testing
Test Service B directly:
GET http://localhost:3001/pong

Test everything through Service A:
POST http://localhost:3000/ping


Expected:

{ "from_service_b": "PONG from Service B" }