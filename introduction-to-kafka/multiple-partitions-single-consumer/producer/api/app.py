from flask import Flask, request
from flask_cors import CORS
from confluent_kafka import Producer
from confluent_kafka.admin import AdminClient, NewTopic
import socket
import time
import json

app = Flask(__name__)
CORS(app)

PRODUCTS_TOPIC = "products"
KAFKA = "kafka:9092"

producer = None

@app.route('/product', methods=['POST'])
def add_product():
    produce(json.dumps(request.json))

    return ('', 204)

def produce_callback(err, msg):
    if err is not None:
        print(f'Failed to deliver message: {msg.value()}. Error: {err}')
    else:
        print(f'Message produced: {msg.value()}')

def produce(message):
    app.logger.info(f'Writing message: "{message}" to topic: {PRODUCTS_TOPIC}')
    producer.produce(PRODUCTS_TOPIC, value=message, callback=produce_callback)
    producer.flush()

def build_kafka_producer():
    global producer
    client_id = socket.gethostname()
    conf = {
        'bootstrap.servers': KAFKA,
        'client.id': client_id
    }
    producer = Producer(conf)
    app.logger.info(f'Kafka producer with id {client_id} created')

def create_topic():
    print(f'Creating topic {PRODUCTS_TOPIC}')
    admin_client = AdminClient({
        "bootstrap.servers": KAFKA
    })

    topic_list = []
    # Topic with two partitions and replication factor of 1
    topic_list.append(NewTopic(PRODUCTS_TOPIC, 2, 1))
    admin_client.create_topics(topic_list)

    # Wait for the topic to be created
    while True:
        cluster_metadata = admin_client.list_topics()
        if PRODUCTS_TOPIC in cluster_metadata.topics.keys():
            print(f'Topic {PRODUCTS_TOPIC} created')
            break

        print(f'Topic {PRODUCTS_TOPIC} is not ready yet')
        time.sleep(2)

build_kafka_producer()
create_topic()
