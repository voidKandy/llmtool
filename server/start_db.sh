#!/bin/bash
source .env

# Default to non-test environment
PREFIX=""

# Check if -t flag is passed
if [[ "$1" == "-t" ]]; then
  PREFIX="TEST_"
fi

# Dynamically fetch variables with or without the prefix
SURREAL_HOST=$(eval echo "\$${PREFIX}SURREAL_HOST")
SURREAL_PORT=$(eval echo "\$${PREFIX}SURREAL_PORT")
SURREAL_USER=$(eval echo "\$${PREFIX}SURREAL_USER")
SURREAL_PASS=$(eval echo "\$${PREFIX}SURREAL_PASS")

# Display the environment variables being used
echo "${PREFIX}SURREAL_HOST: $SURREAL_HOST"
echo "${PREFIX}SURREAL_PORT: $SURREAL_PORT"
echo "${PREFIX}SURREAL_USER: $SURREAL_USER"
echo "${PREFIX}SURREAL_PASS: $SURREAL_PASS"

# Check if all variables are set
if [[ -z "$SURREAL_HOST" || -z "$SURREAL_PORT" || -z "$SURREAL_USER" || -z "$SURREAL_PASS" ]]; then
  echo "Error: One or more environment variables are missing."
  echo "Please set ${PREFIX}SURREAL_HOST, ${PREFIX}SURREAL_PORT, ${PREFIX}SURREAL_USER, and ${PREFIX}SURREAL_PASS."
  exit 1
fi

# Start SurrealDB
surreal start -b "$SURREAL_HOST:$SURREAL_PORT" -u "$SURREAL_USER" -p "$SURREAL_PASS"
