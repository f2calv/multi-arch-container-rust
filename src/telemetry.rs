//! Structured logging setup.
//!
//! `tracing` + `tracing-subscriber` is the Rust analogue of Serilog in the sibling .NET repository
//! and `log/slog` in the sibling Go repository.

use std::error::Error;
use std::time::Duration;

use opentelemetry::global;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{Protocol, WithExportConfig as _};
use opentelemetry_sdk::error::OTelSdkResult;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::resource::{
    EnvResourceDetector, ResourceDetector, TelemetryResourceDetector,
};
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_sdk::Resource;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer as _};

use crate::config::{LogFormat, Settings, UNKNOWN};

/// Instrumentation scope shared by traces and metrics.
pub const INSTRUMENTATION_NAME: &str = "io.github.f2calv.multi-arch-container-rust";

/// Owns the OpenTelemetry providers that must be flushed during shutdown.
pub struct Telemetry {
    logger_provider: SdkLoggerProvider,
    meter_provider: SdkMeterProvider,
    tracer_provider: SdkTracerProvider,
}

/// Install the global `tracing` subscriber.
///
/// Verbosity is controlled by the conventional `RUST_LOG` environment variable
/// (e.g. `RUST_LOG=debug`, `RUST_LOG=multi_arch_container_rust=trace`) and defaults to `info`.
///
/// # Errors
///
/// Returns an error when an OTLP exporter or the global subscriber cannot be initialized.
pub fn init(settings: &Settings) -> Result<Option<Telemetry>, Box<dyn Error + Send + Sync>> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let console_layer = match settings.app.log_format {
        LogFormat::Json => fmt::layer().json().flatten_event(true).boxed(),
        LogFormat::Text => fmt::layer().compact().boxed(),
    };
    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(console_layer);

    if std::env::var_os("OTEL_EXPORTER_OTLP_ENDPOINT").is_none() {
        registry.try_init()?;
        return Ok(None);
    }

    let service_name = std::env::var("OTEL_SERVICE_NAME")
        .unwrap_or_else(|_| "multi-arch-container-rust".to_owned());
    let service_version = if settings.git_tag == UNKNOWN {
        "unknown".to_owned()
    } else {
        settings.git_tag.clone()
    };
    let detectors: [Box<dyn ResourceDetector>; 2] = [
        Box::new(EnvResourceDetector::new()),
        Box::new(TelemetryResourceDetector),
    ];
    let resource = Resource::builder_empty()
        .with_detectors(&detectors)
        .with_attributes([KeyValue::new("service.version", service_version)])
        .with_service_name(service_name)
        .build();

    let span_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .build()?;
    let metric_exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .build()?;
    let log_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .build()?;

    let tracer_provider = SdkTracerProvider::builder()
        .with_resource(resource.clone())
        .with_batch_exporter(span_exporter)
        .build();
    let meter_provider = SdkMeterProvider::builder()
        .with_resource(resource.clone())
        .with_periodic_exporter(metric_exporter)
        .build();
    let logger_provider = SdkLoggerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(log_exporter)
        .build();

    let span_layer = tracing_opentelemetry::layer()
        .with_tracer(tracer_provider.tracer(INSTRUMENTATION_NAME))
        .with_filter(filter_fn(|metadata| metadata.is_span()));
    let log_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    registry.with(span_layer).with(log_layer).try_init()?;
    global::set_tracer_provider(tracer_provider.clone());
    global::set_meter_provider(meter_provider.clone());

    Ok(Some(Telemetry {
        logger_provider,
        meter_provider,
        tracer_provider,
    }))
}

impl Telemetry {
    /// Flush all telemetry and stop the exporters.
    ///
    /// # Errors
    ///
    /// Returns the first provider shutdown error after attempting to stop every provider.
    pub fn shutdown(self) -> OTelSdkResult {
        let timeout = Duration::from_secs(5);
        let logger_result = self.logger_provider.shutdown_with_timeout(timeout);
        let meter_result = self.meter_provider.shutdown_with_timeout(timeout);
        let tracer_result = self.tracer_provider.shutdown_with_timeout(timeout);

        logger_result?;
        meter_result?;
        tracer_result
    }
}
