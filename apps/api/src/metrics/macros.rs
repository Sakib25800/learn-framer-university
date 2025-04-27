use prometheus::{Histogram, HistogramOpts, HistogramVec, Opts};

// Prometheus's histograms work by dividing datapoints in buckets, with each bucket containing
/// the count of datapoints equal or greater to the bucket value.
///
/// These buckets are optimized for measuring API response times:
/// - 10ms to 100ms: High resolution for fast responses
/// - 100ms to 1s: Medium resolution for normal responses
/// - 1s to 10s: Lower resolution for slow responses
const HISTOGRAM_BUCKETS: &[f64] = &[0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0];

pub(super) trait MetricFromOpts: Sized {
    fn from_opts(opts: Opts) -> Result<Self, prometheus::Error>;
}

macro_rules! metrics {
    (
        $vis:vis struct $name:ident {
            $(
                #[doc = $help:expr]
                $(#[$meta:meta])*
                $metric_vis:vis $metric:ident: $ty:ty $([$($label:expr),* $(,)?])?
            ),* $(,)?
        }
        namespace: $namespace:expr,
    ) => {
        $vis struct $name {
            registry: prometheus::Registry,
            $(
                #[doc = $help]
                $(#[$meta])*
                $metric_vis $metric: $ty,
            )*
        }
        impl $name {
            $vis fn new() -> Result<Self, prometheus::Error> {
                use $crate::metrics::macros::MetricFromOpts;

                let registry = prometheus::Registry::new();
                $(
                    let $metric = <$ty>::from_opts(
                        prometheus::Opts::new(stringify!($metric), $help)
                            .namespace($namespace)
                            $(.variable_labels(vec![$($label.into()),*]))?
                    )?;
                    registry.register(Box::new($metric.clone()))?;
                )*
                Ok(Self {
                    registry,
                    $(
                        $metric,
                    )*
                })
            }
        }
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", stringify!($name))
            }
        }
    };
}

pub(crate) use metrics;

macro_rules! load_metric_type {
    ($name:ident as single) => {
        use prometheus::$name;
        impl MetricFromOpts for $name {
            fn from_opts(opts: Opts) -> Result<Self, prometheus::Error> {
                $name::with_opts(opts.into())
            }
        }
    };
    ($name:ident as vec) => {
        use prometheus::$name;
        impl MetricFromOpts for $name {
            fn from_opts(opts: Opts) -> Result<Self, prometheus::Error> {
                $name::new(
                    opts.clone().into(),
                    opts.variable_labels
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
            }
        }
    };
}

load_metric_type!(Counter as single);
load_metric_type!(CounterVec as vec);
load_metric_type!(IntCounter as single);
load_metric_type!(IntCounterVec as vec);
load_metric_type!(Gauge as single);
load_metric_type!(GaugeVec as vec);
load_metric_type!(IntGauge as single);
load_metric_type!(IntGaugeVec as vec);

impl MetricFromOpts for Histogram {
    fn from_opts(opts: Opts) -> Result<Self, prometheus::Error> {
        Histogram::with_opts(HistogramOpts {
            common_opts: opts,
            buckets: HISTOGRAM_BUCKETS.to_vec(),
        })
    }
}

impl MetricFromOpts for HistogramVec {
    fn from_opts(opts: Opts) -> Result<Self, prometheus::Error> {
        HistogramVec::new(
            HistogramOpts {
                common_opts: opts.clone(),
                buckets: HISTOGRAM_BUCKETS.to_vec(),
            },
            opts.variable_labels
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
        )
    }
}
