from confluent_kafka import Consumer, KafkaException
from confluent_kafka.admin import AdminClient, NewTopic
import json
import os

PRODUCTS_TOPIC = "products"
GROUP = "two_consumers_group"
KAFKA = "kafka:9092"

consumer = None
con = None

def build_kafka_consumer():
    print(f'Building kafka consumer on group {GROUP}')

    global consumer
    conf = {
        'bootstrap.servers': KAFKA,
        'group.id': GROUP,
        'client.id': os.environ['KAFKA_CONSUMER_ID'],
        'auto.offset.reset': 'smallest'
    }
    consumer = Consumer(conf)

    print(f'Built kafka consumer on group {GROUP}')

def process_message(message):
    print(f'Processing {message.value()}')

def consume():
    try:
        consumer.subscribe([PRODUCTS_TOPIC])

        while True:
            print(f'Getting new events')
            message = consumer.poll(timeout=10.0)
            if message is None:
                continue

            if message.error():
                raise KafkaException(message.error())
            else:
                process_message(message)
    finally:
        consumer.close()

print('Starting')
build_kafka_consumer()
consume()
