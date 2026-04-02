#!/bin/bash

# Stop ClickHouse Analytics

echo "Stopping ClickHouse server..."
pkill clickhouse-server 2>/dev/null || echo "ClickHouse server was not running."
echo "Done."
