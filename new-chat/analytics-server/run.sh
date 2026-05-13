#!/bin/bash

# Analytics Server Startup Script
# This script starts ClickHouse and initializes the database

set -e

CLICKHOUSE_BIN="/opt/homebrew/bin/clickhouse"
CONTAINER_NAME="clickhouse-analytics"
PORT=8123

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INIT_SQL="${SCRIPT_DIR}/init.sql"

echo "=== Analytics Server Startup ==="

# Check if ClickHouse is installed
if [ ! -f "${CLICKHOUSE_BIN}" ]; then
    echo "Error: ClickHouse not found at ${CLICKHOUSE_BIN}"
    echo "Please install ClickHouse: brew install clickhouse"
    exit 1
fi

# Check if ClickHouse server is already running
if pgrep -f "clickhouse server" > /dev/null 2>&1; then
    echo "ClickHouse server is already running."
else
    echo "Starting ClickHouse server..."
    ${CLICKHOUSE_BIN} server -- --listen_host=127.0.0.1 --http_port=${PORT} > /tmp/clickhouse-server.log 2>&1 &
    sleep 5
fi

# Wait for ClickHouse to be ready
echo "Waiting for ClickHouse to be ready..."
for i in {1..30}; do
    if ${CLICKHOUSE_BIN} client -h 127.0.0.1 --query "SELECT 1" > /dev/null 2>&1; then
        echo "ClickHouse is ready!"
        break
    fi
    if [ "$i" -eq 30 ]; then
        echo "Error: ClickHouse failed to start. Check logs: /tmp/clickhouse-server.log"
        exit 1
    fi
    echo "Waiting... ($i/30)"
    sleep 1
done

# Check if init.sql exists and run it
if [ -f "${INIT_SQL}" ]; then
    echo "Running initialization SQL..."
    ${CLICKHOUSE_BIN} client -h 127.0.0.1 --multiquery < "${INIT_SQL}"
    echo "Database initialized successfully!"
else
    echo "Warning: ${INIT_SQL} not found, skipping database initialization"
fi

echo ""
echo "=== ClickHouse is running ==="
echo "HTTP Interface: http://localhost:${PORT}"
echo ""
echo "To stop ClickHouse:"
echo "  pkill clickhouse-server"
