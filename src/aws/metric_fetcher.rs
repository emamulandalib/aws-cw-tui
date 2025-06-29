use super::metric_types::MetricFetchParams;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use std::time::SystemTime;

pub async fn fetch_comprehensive_metric(
    client: &CloudWatchClient,
    params: MetricFetchParams,
    start_time: SystemTime,
    end_time: SystemTime,
    period_seconds: i32,
) -> (f64, Vec<f64>, Vec<SystemTime>) {
    let mut request = client
        .get_metric_statistics()
        .namespace(&params.namespace)
        .metric_name(&params.metric_name)
        .dimensions(
            aws_sdk_cloudwatch::types::Dimension::builder()
                .name("DBInstanceIdentifier")
                .value(&params.instance_id)
                .build(),
        )
        .start_time(aws_sdk_cloudwatch::primitives::DateTime::from(start_time))
        .end_time(aws_sdk_cloudwatch::primitives::DateTime::from(end_time))
        .period(period_seconds)
        .statistics(aws_sdk_cloudwatch::types::Statistic::Average);

    if let Some(ref u) = params.unit {
        match u.as_str() {
            "Percent" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::Percent),
            "Count" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::Count),
            "Count/Second" => {
                request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::CountSecond)
            }
            "Bytes" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::Bytes),
            "Bytes/Second" => {
                request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::BytesSecond)
            }
            "Seconds" => request = request.unit(aws_sdk_cloudwatch::types::StandardUnit::Seconds),
            _ => {}
        }
    }

    let resp = request.send().await;

    match resp {
        Ok(data) => {
            if let Some(mut datapoints) = data.datapoints {
                datapoints.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

                let latest_value = datapoints.last().and_then(|dp| dp.average).unwrap_or(0.0);

                let recent_datapoints: Vec<_> = datapoints.iter().rev().take(36).rev().collect();

                let history: Vec<f64> = recent_datapoints
                    .iter()
                    .filter_map(|dp| dp.average)
                    .collect();

                let timestamps: Vec<SystemTime> = recent_datapoints
                    .iter()
                    .map(|dp| {
                        dp.timestamp
                            .map(|ts| {
                                SystemTime::UNIX_EPOCH
                                    + std::time::Duration::from_secs(ts.secs() as u64)
                            })
                            .unwrap_or_else(SystemTime::now)
                    })
                    .collect();

                (latest_value, history, timestamps)
            } else {
                (0.0, Vec::new(), Vec::new())
            }
        }
        Err(_) => (0.0, Vec::new(), Vec::new()),
    }
}
