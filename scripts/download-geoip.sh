#!/bin/bash
set -e

GEOIP_URL="https://download.maxmind.com/app/geoip_download?edition_id=GeoLite2-City&license_key=YOUR_LICENSE_KEY&suffix=tar.gz"
GEOIP_FILE="GeoLite2-City.tar.gz"

echo "Downloading GeoIP database..."
curl -o $GEOIP_FILE "$GEOIP_URL"

echo "Extracting GeoIP database..."
tar -xzf $GEOIP_FILE
mv GeoLite2-City_*/GeoLite2-City.mmdb ./GeoLite2-City.mmdb
rm -rf GeoLite2-City_* $GEOIP_FILE

echo "GeoIP database downloaded and extracted successfully!" 