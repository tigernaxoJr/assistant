#!/usr/bin/env python3
import os
import ssl
import sys
import urllib.parse
import urllib.request
from html.parser import HTMLParser

class CWAHTMLParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.img_src = None

    def handle_starttag(self, tag, attrs):
        if tag == "img":
            attrs_dict = dict(attrs)
            if attrs_dict.get("alt") == "報天氣圖卡":
                self.img_src = attrs_dict.get("src")

def main():
    base_url = "https://www.cwa.gov.tw"
    ajax_url = "https://www.cwa.gov.tw/V8/C/ajax/_home_news.html"
    
    # Use argument for output directory, default to current directory
    output_dir = sys.argv[1] if len(sys.argv) > 1 else "."
    
    os.makedirs(output_dir, exist_ok=True)
    
    print(f"Fetching news ajax from {ajax_url}...")
    
    # Handle SSL verification bypass
    ctx = ssl._create_unverified_context()
    
    req = urllib.request.Request(
        ajax_url,
        headers={"User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"}
    )
    
    try:
        with urllib.request.urlopen(req, context=ctx) as response:
            html_content = response.read().decode("utf-8")
    except Exception as e:
        print(f"Error fetching HTML: {e}", file=sys.stderr)
        sys.exit(1)
        
    parser = CWAHTMLParser()
    parser.feed(html_content)
    
    if not parser.img_src:
        print("Error: Could not find image with alt='報天氣圖卡'", file=sys.stderr)
        sys.exit(1)
        
    img_url = urllib.parse.urljoin(base_url, parser.img_src)
    filename = os.path.basename(parser.img_src)
    output_path = os.path.join(output_dir, filename)
    
    print(f"Found image: {img_url}")
    print(f"Downloading to {output_path}...")
    
    img_req = urllib.request.Request(
        img_url,
        headers={"User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"}
    )
    
    try:
        with urllib.request.urlopen(img_req, context=ctx) as response, open(output_path, "wb") as out_file:
            out_file.write(response.read())
        print(f"Success! Saved weather chart to {output_path}")
        print(f"OUTPUT_PATH:{os.path.abspath(output_path)}")
    except Exception as e:
        print(f"Error downloading image: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
