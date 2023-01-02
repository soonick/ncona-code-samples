# Introduction to Kafka

## Dependencies

- [Docker](https://www.docker.com/)
- [Docker compose](https://docs.docker.com/compose/)

## Single partition single consumer

### Starting Kafka

To start a local instance of Kafka use this command:

```bash
docker rm kafka
docker rm zookeeper
docker-compose -f single-partition-single-consumer/kafka-docker-compose.yaml up
```

### Producing events

Now that we have Kafka running we can start producing events. We have a web UI to do this. To start the we UI:

```bash
docker rm api
docker rm web
docker-compose -f single-partition-single-consumer/producer/docker-compose.yaml up
```

Go to `http://localhost:8080/` in your browser and input a few items.

### Consuming events

Let's now spin up our consumer:

```bash
docker rm consumer
docker-compose -f single-partition-single-consumer/consumer/docker-compose.yaml up
```

The consumer will output something like this:

```bash
consumer    | Getting new events
consumer    | Processing b'{"name": "TV", "stock": "10"}'
consumer    | Getting new events
consumer    | Processing b'{"name": "Computer", "stock": "5"}'
consumer    | Getting new events
consumer    | Processing b'{"name": "Radio", "stock": "7"}'
```

Those are all the items I had input.

Stop all the containers before moving to the next exercise.

## Single partition multiple consumers in same consumer group

### Starting Kafka

To start a local instance of Kafka use this command:

```bash
docker rm kafka
docker rm zookeeper
docker-compose -f single-partition-multiple-consumers/kafka-docker-compose.yaml up
```

### Start Producer

Now that we have Kafka running let's start our producer. To start the we UI:

```bash
docker rm api
docker rm web
docker-compose -f single-partition-multiple-consumers/producer/docker-compose.yaml up
```

Go to `http://localhost:8080/` in your browser. Don't input any events yet.

### Start Consumers

In this scenario we will have two consumers. Start the first one with this command:

```bash
docker rm consumer
docker-compose -f single-partition-multiple-consumers/consumers/docker-compose.yaml up
```

Start the second consumer:

```bash
docker rm consumer_two
docker-compose -f single-partition-multiple-consumers/consumers/docker-compose-two.yaml up
```

### Producing events

Now that we have the consumers running, let's produce a few events using the UI.

Even though we have two consumers, all messages will be delivered to only one of the consumers, so each message gets processed only once.

Kill the consumer that is getting the messages and produce another batch of messages. The new messages will be processed by the consumer that is still up.

Stop all the containers before moving to the next exercise.

## Manual Acknowledment

### Starting Kafka

To start a local instance of Kafka use this command:

```bash
docker rm kafka
docker rm zookeeper
docker-compose -f manual-acknowledgement/kafka-docker-compose.yaml up
```

### Producing events

Now that we have Kafka running we can start producing events. We have a web UI to do this. To start the we UI:

```bash
docker rm api
docker rm web
docker-compose -f manual-acknowledgement/producer/docker-compose.yaml up
```

Go to `http://localhost:8080/` in your browser and input a few items.

### Consuming events

Let's now spin up our consumer:

```bash
docker rm consumer
docker-compose -f manual-acknowledgement/consumer/docker-compose.yaml up
```

The consumer will fail 50% of the time and will resubscribe. We'll see that when it resubscribes it will get the last message again. The output will be something like this:

```bash
consumer    | Getting new events
consumer    | Processing b'{"name": "TV", "stock": "10"}'
consumer    | There was an error consuming message: Random failure. Retrying
consumer    | Building kafka consumer on group manual_ack
consumer    | Built kafka consumer on group manual_ack
consumer    | Consumer subscribing
consumer    | Getting new events
consumer    | Processing b'{"name": "TV", "stock": "10"}'
consumer    | There was an error consuming message: Random failure. Retrying
consumer    | Building kafka consumer on group manual_ack
consumer    | Built kafka consumer on group manual_ack
consumer    | Consumer subscribing
consumer    | Getting new events
consumer    | Processing b'{"name": "TV", "stock": "10"}'
consumer    | There was an error consuming message: Random failure. Retrying
consumer    | Building kafka consumer on group manual_ack
consumer    | Built kafka consumer on group manual_ack
consumer    | Consumer subscribing
consumer    | Getting new events
consumer    | Processing b'{"name": "TV", "stock": "10"}'
consumer    | Processed b'{"name": "TV", "stock": "10"}' successfully
consumer    | Getting new events
consumer    | Processing b'{"name": "Computer", "stock": "5"}'
consumer    | Processed b'{"name": "Computer", "stock": "5"}' successfully
consumer    | Getting new events
consumer    | Processing b'{"name": "Radio", "stock": "7"}'
consumer    | There was an error consuming message: Random failure. Retrying
consumer    | Building kafka consumer on group manual_ack
consumer    | Built kafka consumer on group manual_ack
consumer    | Consumer subscribing
consumer    | Getting new events
consumer    | Processing b'{"name": "Radio", "stock": "7"}'
consumer    | There was an error consuming message: Random failure. Retrying
consumer    | Building kafka consumer on group manual_ack
consumer    | Built kafka consumer on group manual_ack
consumer    | Consumer subscribing
consumer    | Getting new events
consumer    | Processing b'{"name": "Radio", "stock": "7"}'
consumer    | Processed b'{"name": "Radio", "stock": "7"}' successfully
consumer    | Getting new events
```

Stop all the containers before moving to the next exercise.

## Multiple partitions single consumers

### Starting Kafka

To start a local instance of Kafka use this command:

```bash
docker rm kafka
docker rm zookeeper
docker-compose -f multiple-partitions-single-consumer/kafka-docker-compose.yaml up
```

### Producing events

Now that we have Kafka running we can start producing events. We have a web UI to do this. To start the we UI:

```bash
docker rm api
docker rm web
docker-compose -f multiple-partitions-single-consumer/producer/docker-compose.yaml up
```

Go to `http://localhost:8080/` in your browser and input a few items.

### Consuming events

Let's now spin up our consumer:

```bash
docker rm consumer
docker-compose -f multiple-partitions-single-consumer/consumer/docker-compose.yaml up
```

Since we have a single consumer but messages are added to different partitions, messages are not always going to be processed in order (they will be processed in order in their respective partition). A test run from me looks like this (Items were created in alphabetical order):

```bash
consumer    | Processing b'{"name": "aaa", "stock": "aaa"}'
consumer    | Getting new events
consumer    | Processing b'{"name": "ccc", "stock": "ccc"}'
consumer    | Getting new events
consumer    | Processing b'{"name": "eee", "stock": "eee"}'
consumer    | Getting new events
consumer    | Processing b'{"name": "bbb", "stock": "bbb"}'
consumer    | Getting new events
consumer    | Processing b'{"name": "ddd", "stock": "ddd"}'
```
