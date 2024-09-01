use chrono::DateTime;
use chrono_tz::America::New_York;

pub fn format_time(date_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let date = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z")?;
    let ny_time = date.with_timezone(&New_York);
    Ok(ny_time.format("%I:%M%p").to_string().to_lowercase() + " est")
}
