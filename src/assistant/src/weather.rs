use regex::Regex;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] ureq::Error),

    #[error("Parsing error: {0}")]
    Parse(String),

    #[error("Image with alt='報天氣圖卡' was not found in the HTML content")]
    ImageNotFound,
}

/// Fetches the latest weather chart ("報天氣圖卡") from the Taiwan Central Weather Administration (CWA) website.
/// 
/// Returns a tuple containing the filename (e.g. "WT_L20260531191410_1.png") and the image bytes (`Vec<u8>`).
pub fn get_weather_chart() -> Result<(String, Vec<u8>), WeatherError> {
    let base_url = "https://www.cwa.gov.tw";
    let ajax_url = "https://www.cwa.gov.tw/V8/C/ajax/_home_news.html";
    
    // Fetch the news AJAX page
    let res = ureq::get(ajax_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .call()?;
        
    let html = res.into_body().read_to_string()
        .map_err(|e| WeatherError::Parse(format!("Failed to read HTML body: {}", e)))?;
        
    // Regex to match img tag with alt="報天氣圖卡"
    // Handles both alt before src and src before alt
    let re = Regex::new(
        r#"<img\s+[^>]*alt="報天氣圖卡"[^>]*src="([^"]+)"|<img\s+[^>]*src="([^"]+)"[^>]*alt="報天氣圖卡""#
    ).map_err(|e| WeatherError::Parse(format!("Failed to compile regex: {}", e)))?;
    
    let img_path = re.captures(&html)
        .and_then(|caps| caps.get(1).or_else(|| caps.get(2)))
        .map(|m| m.as_str())
        .ok_or(WeatherError::ImageNotFound)?;
        
    let full_img_url = format!("{}{}", base_url, img_path);
    
    // Fetch the image bytes
    let img_res = ureq::get(&full_img_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .call()?;
        
    let img_bytes = img_res.into_body().read_to_vec()
        .map_err(|e| WeatherError::Parse(format!("Failed to read image bytes: {}", e)))?;
        
    let filename = img_path
        .split('/')
        .last()
        .unwrap_or("weather_chart.png")
        .to_string();
        
    Ok((filename, img_bytes))
}

/// Fetches the latest weather chart and saves it to the specified output directory.
/// 
/// Returns the path to the saved file.
pub fn download_weather_chart_to<P: AsRef<Path>>(output_dir: P) -> Result<PathBuf, WeatherError> {
    let (filename, bytes) = get_weather_chart()?;
    let out_dir = output_dir.as_ref();
    std::fs::create_dir_all(out_dir)
        .map_err(|e| WeatherError::Parse(format!("Failed to create output directory: {}", e)))?;
        
    let out_path = out_dir.join(filename);
    std::fs::write(&out_path, bytes)
        .map_err(|e| WeatherError::Parse(format!("Failed to write image file: {}", e)))?;
        
    Ok(out_path)
}
