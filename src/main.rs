use opentelemetry::trace::get_active_span;
use opentelemetry::trace::Tracer;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{global, Key, KeyValue, Value};
use opentelemetry_otlp::SpanExporter;
// use opentelemetry_sdk::export::trace::SpanExporter as _;
use opentelemetry_sdk::{
    runtime,
    trace::{RandomIdGenerator, Sampler, TracerProvider},
    Resource,
};
/*

use opentelemetry_semantic_conventions::{
    attribute::{SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
// Create a Resource that captures information about the entity for which telemetry is recorded.
fn resource() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        ],
        SCHEMA_URL,
    )
}
*/


#[tokio::main]
async fn main() {
    let exporter = SpanExporter::builder().with_tonic().build().unwrap();

    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_simple_exporter(exporter)
        .build();

    let tracer = provider.tracer("trace_demo");

    tracer.in_span("experiment", |_context| {
        get_active_span(|span| {
            span.set_attribute(KeyValue::new(
                Key::from_static_str("year"),
                Value::I64(3434),
            ));
        })
    });

    global::set_tracer_provider(provider.clone());

    // Ensure all spans are exported before the program exits
    global::shutdown_tracer_provider();
}
