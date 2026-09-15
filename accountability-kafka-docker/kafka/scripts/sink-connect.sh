#!/bin/sh

# ----- Sink to MongoDB

curl -s \
     -X "POST" "http://localhost:8083/connectors/" \
     -H "Content-Type: application/json" \
     -d '{
  "name": "mongo-sink",
  "config": {
     "connector.class": "com.mongodb.kafka.connect.MongoSinkConnector",
        "tasks.max": "1",
        "topics": "ROSMessagesTopic",
        "connection.uri":"mongodb+srv://user:password@cluster0.cluster_uri.mongodb.net",
        "database": "langchain_demo",
        "collection": "collection_of_text_blobs3",
        "key.converter": "org.apache.kafka.connect.storage.StringConverter",
        "value.converter": "org.apache.kafka.connect.json.JsonConverter",
        "value.converter.schemas.enable": "false"
  }
}'
