import time
import threading
from opentelemetry import trace
from opentelemetry import context
from opentelemetry.exporter.jaeger.thrift import JaegerExporter
from opentelemetry.sdk.resources import SERVICE_NAME, Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from opentelemetry.trace import set_span_in_context


trace.set_tracer_provider(
   TracerProvider(
       resource=Resource.create({SERVICE_NAME: "my-service"})
   )
)

jaeger_exporter = JaegerExporter(
   agent_host_name="localhost",
    agent_port=6831,
)

trace.get_tracer_provider().add_span_processor(
    BatchSpanProcessor(jaeger_exporter)
)

def start_span(tracer, context, name):
    with tracer.start_as_current_span(name, context):
        print(f"{name} span")
        time.sleep(1)

tracer = trace.get_tracer(__name__)
with tracer.start_as_current_span("first-span"):
    print("first span")

    # Start execution in new thread
    ctx = context.get_current()
    t1 = threading.Thread(target=start_span, args=(tracer, ctx, "second"))
    t1.start()

    # Start execution in another thread after 1 second
    time.sleep(1)
    t2 = threading.Thread(target=start_span, args=(tracer, ctx, "third"))
    t2.start()

    with tracer.start_as_current_span("fourth"):
        print("fourth span")
        time.sleep(1)

    time.sleep(1)
    t1.join()
    t2.join()
