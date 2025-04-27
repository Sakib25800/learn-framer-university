//! This module defines all the instance-level metrics.
//!
//! Instance-level metrics are collected separately for each instance of the application,
//! and are then aggregated at the Prometheus level. They're not suited for service-level metrics
//! (like "how many users are there").
//!
//! There are two ways to update instance-level metrics:
//!
//! * Continuously as things happen in the instance: every time something worth recording happens
//!   the application updates the value of the metrics, accessing the metric through
//!   `req.app().instance_metrics.$metric_name`.
//!
//! * When metrics are scraped by Prometheus: every `N` seconds Prometheus sends a request to the
//!   instance asking what the value of the metrics are, and you can update metrics when that
//!   happens by calculating them in the `gather` method.
//!
//! As a rule of thumb, if the metric requires a database query to be updated it's probably a
//! service-level metric, and you should add it to `src/metrics/service.rs` instead.

use crate::app::App;
use crate::metrics::macros::metrics;
use prometheus::{proto::MetricFamily, HistogramVec, IntCounter, IntCounterVec, IntGauge};

metrics! {
    pub struct InstanceMetrics {
        /// Number of requests processed by this instance
        pub requests_total: IntCounter,
        /// Number of requests currently being processed
        pub requests_in_flight: IntGauge,

        /// Response times of our endpoints
        pub response_times: HistogramVec["endpoint"],
        /// Number of responses per status code
        pub responses_by_status_code_total: IntCounterVec["status"],
    }

    // All instance metrics will be prefixed with this namespace.
    namespace: "framer_university_instance",
}

impl InstanceMetrics {
    pub fn gather(&self, _app: &App) -> prometheus::Result<Vec<MetricFamily>> {
        Ok(self.registry.gather())
    }
}
