import os
import rclpy
from rclpy.node import Node
from ament_index_python.packages import get_package_share_directory
from confluent_kafka import Producer
from uuid import uuid4
import json


from rclpy.qos import QoSProfile, QoSDurabilityPolicy, QoSHistoryPolicy, QoSReliabilityPolicy

from rcl_interfaces.msg import Log


# Mapping dictionary to convert numeric log levels to string representations
LOG_LEVEL_MAP = {
    10: "DEBUG",
    20: "INFO",
    30: "WARNING",
    40: "ERROR",
    50: "FATAL",
}


class KafkaProducerNode(Node):
    def __init__(self):
        super().__init__("kafka_producer_node")
        self.get_logger().info("This node just says 'Hello'")
        self.load_kafka_config()

        # QoS profiles subscription definition
        self.rosout_qos = QoSProfile(
          durability=QoSDurabilityPolicy.TRANSIENT_LOCAL,
          reliability=QoSReliabilityPolicy.RELIABLE,
          history=QoSHistoryPolicy.KEEP_LAST,
          depth=1000)


        # Subscription to /rosout topic
        self.subscription = self.create_subscription(Log, '/rosout', self.extract_rosout_info, 10)
        self.subscription


    # Process messages from /rosout topic
    def extract_rosout_info(self, msg):
        sec = msg.stamp.sec
        nanosec = msg.stamp.nanosec
        log_level = LOG_LEVEL_MAP.get(msg.level, "UNKNOWN")
        name = msg.name
        message = msg.msg
        file_location = msg.file
        function_name = msg.function
        line_number = msg.line

        # Create a dictionary with all the relevant fields
        log_data = {
            "timestamp": {
                "sec": sec,
                "nanosec": nanosec
            },
            "log_level": log_level,
            "name": name,
            "message": message,
            "file_location": file_location,
            "function_name": function_name,
            "line_number": line_number
        }

        # Convert the dictionary to a JSON string
        self.json_message = json.dumps(log_data, indent=4)
        self.get_logger().info(f"Log message as JSON: {self.json_message}")
        self.publish_to_kafka()
        

    def load_kafka_config(self):
        # Get the absolute file path of the .pem files    
        self.CARoot_cert_file = os.path.join(get_package_share_directory('rt_recorder_explainer'), 'certs', 'CARoot.pem')
        self.producer_cert_file = os.path.join(get_package_share_directory('rt_recorder_explainer'), 'certs', 'producer.pem')
        self.producer_key_file = os.path.join(get_package_share_directory('rt_recorder_explainer'), 'certs', 'producer_key.pem')

        self.kafka_topic_name = "ROSMessagesTopic" 
        self.conf = {'bootstrap.servers': 'localhost:19093',
                'security.protocol': 'SSL',
                'ssl.ca.location': self.CARoot_cert_file,
                'ssl.certificate.location': self.producer_cert_file,
                'ssl.key.location': self.producer_key_file,
                #'debug':'all',
                'ssl.endpoint.identification.algorithm':'none'
            }

        jsonString1 = """ {"name":"Gal", "email":"Gadot84@gmail.com", "salary": "8345.55"} """
        jsonString2 = """ {"name":"Dwayne", "email":"Johnson52@gmail.com", "salary": "7345.75"} """
        jsonString3 = """ {"name":"Momoa", "email":"Jason91@gmail.com", "salary": "3345.25"} """

        self.jsonv1 = jsonString1.encode()
        self.jsonv2 = jsonString2.encode()
        self.jsonv3 = jsonString3.encode()

    def publish_to_kafka(self):

        producer = Producer(self.conf)

        producer.poll(0)

        # Producción de mensajes asíncrona con Kafka
        try:
            producer.produce(topic=self.kafka_topic_name, key=str(uuid4()), value=self.json_message, on_delivery=self.delivery_report)
            producer.produce(topic=self.kafka_topic_name, key=str(uuid4()), value=self.jsonv2, on_delivery=self.delivery_report)
            producer.produce(topic=self.kafka_topic_name, key=str(uuid4()), value=self.jsonv3, on_delivery=self.delivery_report)
            producer.flush()
            self.get_logger().info("Mensajes enviados a Kafka")
        except Exception as ex:
            self.get_logger().error(f"Exception: {ex}")

    def delivery_report(self, errmsg, msg):
        if errmsg is not None:
            self.get_logger().error(f"Fallo en la entrega del mensaje: {msg.key()} : {errmsg}")
        else:
            self.get_logger().info(f"Mensaje: {msg.key()} producido exitosamente en el Topic: {msg.topic()} en la Partición: [{msg.partition()}] en el offset {msg.offset()}")

        # Detener el nodo después de enviar los mensajes
        self.get_logger().info("Mensajes enviados exitosamente")


def main(args=None):
    rclpy.init(args=args)
    node = KafkaProducerNode()
    #rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()

if __name__ == "__main__":
    main()