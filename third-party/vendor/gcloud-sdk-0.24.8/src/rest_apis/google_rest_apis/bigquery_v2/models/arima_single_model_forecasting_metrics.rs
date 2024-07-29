use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// ArimaSingleModelForecastingMetrics : Model evaluation metrics for a single ARIMA forecasting model.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArimaSingleModelForecastingMetrics {
    #[serde(
        rename = "arimaFittingMetrics",
        skip_serializing_if = "Option::is_none"
    )]
    pub arima_fitting_metrics:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::ArimaFittingMetrics>>,
    /// Is arima model fitted with drift or not. It is always false when d is not 1.
    #[serde(rename = "hasDrift", skip_serializing_if = "Option::is_none")]
    pub has_drift: Option<bool>,
    /// If true, holiday_effect is a part of time series decomposition result.
    #[serde(rename = "hasHolidayEffect", skip_serializing_if = "Option::is_none")]
    pub has_holiday_effect: Option<bool>,
    /// If true, spikes_and_dips is a part of time series decomposition result.
    #[serde(rename = "hasSpikesAndDips", skip_serializing_if = "Option::is_none")]
    pub has_spikes_and_dips: Option<bool>,
    /// If true, step_changes is a part of time series decomposition result.
    #[serde(rename = "hasStepChanges", skip_serializing_if = "Option::is_none")]
    pub has_step_changes: Option<bool>,
    #[serde(rename = "nonSeasonalOrder", skip_serializing_if = "Option::is_none")]
    pub non_seasonal_order: Option<Box<crate::google_rest_apis::bigquery_v2::models::ArimaOrder>>,
    /// Seasonal periods. Repeated because multiple periods are supported for one time series.
    #[serde(rename = "seasonalPeriods", skip_serializing_if = "Option::is_none")]
    pub seasonal_periods: Option<Vec<SeasonalPeriods>>,
    /// The time_series_id value for this time series. It will be one of the unique values from the time_series_id_column specified during ARIMA model training. Only present when time_series_id_column training option was used.
    #[serde(rename = "timeSeriesId", skip_serializing_if = "Option::is_none")]
    pub time_series_id: Option<String>,
    /// The tuple of time_series_ids identifying this time series. It will be one of the unique tuples of values present in the time_series_id_columns specified during ARIMA model training. Only present when time_series_id_columns training option was used and the order of values here are same as the order of time_series_id_columns.
    #[serde(rename = "timeSeriesIds", skip_serializing_if = "Option::is_none")]
    pub time_series_ids: Option<Vec<String>>,
}

impl ArimaSingleModelForecastingMetrics {
    /// Model evaluation metrics for a single ARIMA forecasting model.
    pub fn new() -> ArimaSingleModelForecastingMetrics {
        ArimaSingleModelForecastingMetrics {
            arima_fitting_metrics: None,
            has_drift: None,
            has_holiday_effect: None,
            has_spikes_and_dips: None,
            has_step_changes: None,
            non_seasonal_order: None,
            seasonal_periods: None,
            time_series_id: None,
            time_series_ids: None,
        }
    }
}

/// Seasonal periods. Repeated because multiple periods are supported for one time series.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SeasonalPeriods {
    #[serde(rename = "SEASONAL_PERIOD_TYPE_UNSPECIFIED")]
    SeasonalPeriodTypeUnspecified,
    #[serde(rename = "NO_SEASONALITY")]
    NoSeasonality,
    #[serde(rename = "DAILY")]
    Daily,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "QUARTERLY")]
    Quarterly,
    #[serde(rename = "YEARLY")]
    Yearly,
}

impl Default for SeasonalPeriods {
    fn default() -> SeasonalPeriods {
        Self::SeasonalPeriodTypeUnspecified
    }
}