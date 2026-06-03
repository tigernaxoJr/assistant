# Personal Assistant Tools (Rust)

這是一個用 Rust 建立的個人助手工具專案，位於工作區的 `src/assistant/` 目錄下。目前包含取得中央氣象署（CWA）天氣圖卡的功能。

## 專案結構

- [Cargo.toml](file:///home/naxo/assitant/src/assistant/Cargo.toml): 專案設定檔與依賴套件管理。
- [src/lib.rs](file:///home/naxo/assitant/src/assistant/src/lib.rs): 程式庫進入點，導出各個助手模組。
- [src/weather.rs](file:///home/naxo/assitant/src/assistant/src/weather.rs): 取得中央氣象署天氣圖卡的模組。
- [src/main.rs](file:///home/naxo/assitant/src/assistant/src/main.rs): 命令列介面進入點。

## 核心函式與 API

### 天氣圖卡模組 (`weather`)

1. **[get_weather_chart](file:///home/naxo/assitant/src/assistant/src/weather.rs#L18)**
   - 簽名: `pub fn get_weather_chart() -> Result<(String, Vec<u8>), WeatherError>`
   - 功能: 取得最新報天氣圖卡的檔案名稱與圖片 bytes。
   
2. **[download_weather_chart_to](file:///home/naxo/assitant/src/assistant/src/weather.rs#L55)**
   - 簽名: `pub fn download_weather_chart_to<P: AsRef<Path>>(output_dir: P) -> Result<PathBuf, WeatherError>`
   - 功能: 下載並儲存最新報天氣圖卡至指定目錄。

## 如何使用

請先將工作目錄切換至 `src/assistant/` 資料夾：

### 執行命令列工具

下載天氣圖卡至目前目錄：
```bash
cargo run -- weather
```

下載天氣圖卡至指定目錄：
```bash
cargo run -- weather /path/to/directory
```

顯示說明選單：
```bash
cargo run -- help
```
