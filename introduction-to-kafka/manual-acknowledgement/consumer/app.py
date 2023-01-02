from confluent_kafka import Consumer, KafkaException
from confluent_kafka.admin import AdminClient, NewTopic
from random import randint
import json

PRODUCTS_TOPIC = "products"
GROUP = "manual_ack"
KAFKA = "kafka:9092"

consumer = None
con = None

def build_kafka_consumer():
    print(f'Building kafka consumer on group {GROUP}')

    global consumer
    conf = {
        'bootstrap.servers': KAFKA,
        'group.id': GROUP,
        'auto.offset.reset': 'smallest',
        'enable.auto.commit': False
    }
    consumer = Consumer(conf)

    print(f'Built kafka consumer on group {GROUP}')

def random_fail():
    if randint(1, 2) > 1:
        raise RuntimeError('Random failure')

def process_message(message):
    print(f'Processing {message.value()}')
    random_fail()
    print(f'Processed {message.value()} successfully')
    consumer.commit(message=message)

def consume():
    while True:
        try:
            build_kafka_consumer()
            print(f'Consumer subscribing')
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
        except RuntimeError as e:
            print(f'There was an error consuming message: {e}. Retrying')
        finally:
            consumer.close()

print('Starting')
consume()
