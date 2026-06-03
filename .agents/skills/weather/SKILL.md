---
name: weather-chart-downloader
description: Downloads the latest weather chart ("報天氣圖卡") from the Central Weather Administration (CWA) website.
---

# Weather Chart Downloader

This skill downloads the latest weather chart ("報天氣圖卡") from the Taiwan Central Weather Administration (CWA) website.

## How it works

1. It fetches the HTML content of the CWA home news component at `https://www.cwa.gov.tw/V8/C/ajax/_home_news.html`.
2. It parses the HTML to locate the `<img>` element with `alt="報天氣圖卡"`.
3. It resolves the absolute URL of the image from its `src` attribute.
4. It downloads the image file to the destination directory.

## How to use

You can execute the provided Python script to download the weather chart:

```bash
python3 .agents/skills/weather/scripts/download_weather_chart.py [output_directory]
```

- If `output_directory` is not specified, it defaults to the current working directory.
- The script automatically handles TLS certificate validation issues by using an unverified SSL context, as the CWA server might have non-standard certificate configurations.
