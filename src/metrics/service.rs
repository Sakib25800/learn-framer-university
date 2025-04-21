use lfu_database::PgDbClient;
use prometheus::{proto::MetricFamily, IntGauge};

use super::macros::metrics;
use crate::util::errors::AppResult;

metrics! {
    pub struct ServiceMetrics {
        /// Number of monthly active users (last 30 days)
        // monthly_active_users: IntGauge,
        /// Number of weekly active users (last 7 days)
        // weekly_active_users: IntGauge,
        /// Number of daily active users (last 24 hours)
        // daily_active_users: IntGauge,
        /// Number of concurrent users (last 15 minutes)
        // concurrent_users: IntGauge,
        /// Total registered users
        total_users: IntGauge,
    }

    namespace: "learn_framer_university_service",
}

impl ServiceMetrics {
    pub(crate) async fn gather(&self, db: &PgDbClient) -> AppResult<Vec<MetricFamily>> {
        // TODO: Implement this
        // let monthly_active = User::count_monthly_active(pool).await.unwrap_or(0);
        // let weekly_active = User::count_weekly_active(pool).await.unwrap_or(0);
        // let daily_active = User::count_daily_active(pool).await.unwrap_or(0);
        // let concurrent = User::count_concurrent(pool).await.unwrap_or(0);
        let total = db.users.count().await?.unwrap_or(-1);

        // self.monthly_active_users.set(monthly_active);
        // self.weekly_active_users.set(weekly_active);
        // self.daily_active_users.set(daily_active);
        // self.concurrent_users.set(concurrent);
        self.total_users.set(total);

        Ok(self.registry.gather())
    }
}
