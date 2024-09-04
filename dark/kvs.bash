#!/bin/bash

# Function to run a 'get' command
get_key() {
    key=$1
    echo "Getting value for key: $key"
    cargo run --bin kvs_client get "$key"
}

# Function to run a 'set' command
set_key() {
    key=$1
    value=$2
    echo "Setting key: $key with value: $value"
    cargo run --bin kvs_client set "$key" "$value"
}

# Function to run an 'rm' command
rm_key() {
    key=$1
    echo "Removing key: $key"
    cargo run --bin kvs_client rm "$key"
}

# Example usage of the functions
set_key "1" "apple"
set_key "2" "banana"
set_key "3" "cherry"

get_key "1"
get_key "2"
get_key "3"

rm_key "2"

get_key "2" # This should fail as the key is removed

