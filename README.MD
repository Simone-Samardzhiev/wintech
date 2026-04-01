# Wintech

Work in progress restfull API for ordering and
managing smart windows.

## Table of content

- [Installation](#installation)
- [Docker deployment](#docker-deployment)
---

# Installation

Follow this steps to run the API.

### Prerequisites

- Rust (1.94.0+)
- PostgreSQL
- Git
- sqlx-cli
- trunk

1. **Clone the repository**
    
   ```bash
   git clone https://github.com/Simone-Samardzhiev/wintech
   ```
   
2. **Add environment variables(via .env or export them directly)**

    You can find .env example at [example](.env.example)

3. Run the migrations
   ```bash
   cd backend
   sqlx migrate run    
   ```

4. Build the leptos app
   ```bash
   cd frontend
   trunk build --release
   ```

5. Run the axum API
   ```bash
   cd backend
   cargo run
   ```
---

## Docker deployment

Follow this steps to run the API using docker.

1. **Clone the repository**

   ```bash
   git clone https://github.com/Simone-Samardzhiev/wintech
   ```

2. **Build the image**
   ```bash
   docker build -t wintech .
   ```

3. **Run the container**

   The environment path FRONTEND_PATH equals **dist** as
   docker will place the static files there

   ```bash
   docker run --init -p 8080:8080 --env-file .env wintech     
   ``` 
---

## Docs

- Apidog documentation:
  👉 [View it here](https://d18hyzvhn8.apidog.io)
---

## Development Tools
- [Trunk](https://trunkrs.dev) - for building the leptos app
- [sqlx-cli](https://crates.io/crates/sqlx-cli) - for creating and managing migrations
