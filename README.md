🦀 Rust OrderBook
A high-performance OrderBook implementation in Rust for cryptocurrency exchanges.
It efficiently manages and matches market and limit orders across multiple trading pairs (e.g., BTC-USD), maintaining separate queues for bids (buy) and asks (sell).

The system prioritizes both speed and fairness, ensuring deterministic matching based on price-time priority — all while being ready for real-time integration via WebSocket notifications.

🚀 Features
-Order Management – Add and manage both market and limit orders with ease.

-Order Matching Engine – Automatically matches market orders against the best available limit orders, and executes limit orders as soon as compatible counterparts appear.

-Order Querying – Retrieve current order book state, including all bids, asks, or filtered views by specific criteria.

-WebSocket Notifier – Push updates and match events to connected clients in real time.

-Fair Matching Priority – Orders are processed according to price and timestamp to ensure fairness and consistency.
