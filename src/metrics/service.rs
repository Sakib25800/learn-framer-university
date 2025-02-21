use chrono::Utc;
use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use prometheus::{proto::MetricFamily, IntGauge};

use super::macros::metrics;
use crate::util::errors::AppResult;
use lfu_database::schema::users;

metrics! {
    pub struct ServiceMetrics {
        /// Number of monthly active users (last 30 days)
        monthly_active_users: IntGauge,
        /// Number of weekly active users (last 7 days)
        weekly_active_users: IntGauge,
        /// Number of daily active users (last 24 hours)
        daily_active_users: IntGauge,
        /// Number of concurrent users (last 15 minutes)
        concurrent_users: IntGauge,
        /// Total registered users
        total_users: IntGauge,
    }

    namespace: "framer_university_service",
}

impl ServiceMetrics {
    pub(crate) async fn gather(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> AppResult<Vec<MetricFamily>> {
        let now = Utc::now().naive_utc();
        let thirty_days_ago = now - chrono::Duration::days(30);
        let seven_days_ago = now - chrono::Duration::days(7);
        let twenty_four_hours_ago = now - chrono::Duration::hours(24);
        let fifteen_minutes_ago = now - chrono::Duration::minutes(15);

        let monthly_active = users::table
            .filter(users::last_active_at.gt(thirty_days_ago))
            .count()
            .get_result::<i64>(conn)
            .await
            .unwrap_or(0);

        let weekly_active = users::table
            .filter(users::last_active_at.gt(seven_days_ago))
            .count()
            .get_result::<i64>(conn)
            .await
            .unwrap_or(0);

        let daily_active = users::table
            .filter(users::last_active_at.gt(twenty_four_hours_ago))
            .count()
            .get_result::<i64>(conn)
            .await
            .unwrap_or(0);

        let concurrent = users::table
            .filter(users::last_active_at.gt(fifteen_minutes_ago))
            .count()
            .get_result::<i64>(conn)
            .await
            .unwrap_or(0);

        let total = users::table
            .count()
            .get_result::<i64>(conn)
            .await
            .unwrap_or(0);

        self.monthly_active_users.set(monthly_active);
        self.weekly_active_users.set(weekly_active);
        self.daily_active_users.set(daily_active);
        self.concurrent_users.set(concurrent);
        self.total_users.set(total);

        Ok(self.registry.gather())
    }
}
