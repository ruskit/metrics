use crate::errors::MetricsError;
use opentelemetry_sdk::metrics::SdkMeterProvider;

pub fn install() -> Result<SdkMeterProvider, MetricsError> {
    Ok(SdkMeterProvider::default())
}
