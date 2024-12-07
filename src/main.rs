use opentelemetry::trace::get_active_span;
use opentelemetry::trace::Tracer;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{global, Key, KeyValue, Value};
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::{trace::TracerProvider, Resource};
use opentelemetry_semantic_conventions::{
    attribute::{SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};

#[tokio::main]
async fn main() {
    let resource = Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        ],
        SCHEMA_URL,
    );

    let exporter = SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    let provider = TracerProvider::builder()
        .with_simple_exporter(exporter)
        .with_resource(resource)
        .build();

    let tracer = provider.tracer("trace_demo");


    global::set_tracer_provider(provider);

    tracer.in_span("experiment", |_context| {
        get_active_span(|span| {
            span.set_attribute(KeyValue::new(
                Key::from_static_str("year"),
                Value::I64(3434),
            ));
        })
    });

    // Ensure all spans are exported before the program exits
    global::shutdown_tracer_provider();
}
