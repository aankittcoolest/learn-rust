#!/bin/bash

echo "Waiting for zookeeper to get rid of it's old session..."
sleep 20s
/opt/bitnami/scripts/kafka/run.sh

# Wait for kafka to come up
kafka-topics.sh --bootstrap-server kafka:9092 --list

# Create kafka topic
kafka-topics.sh --bootstrap-server kafka:9092 --topic greetings --partitions 1 --replication-factor 1 --create --if-not-exists

# List kafka topics
kafka-topics.sh --bootstrap-server kafka:9092 --list