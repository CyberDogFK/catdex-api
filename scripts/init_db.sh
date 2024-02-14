#!/usr/bin/env bash
set -x
set -eo pipefail

docker run \
--name catdex-db \
-e POSTGRES_PASSWORD=mypassword \
-p 5433:5432 \
-d postgres:12.3-alpine
