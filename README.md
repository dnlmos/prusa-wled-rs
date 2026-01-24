```bash
curl -X POST http://192.168.0.38/json/state \
    -H "Content-Type: application/json" \
    -d '{
        "on": true,
        "seg": [{
            "fx": 98,
            "ix": 180
        }]
    }'
```


env
```bash
PRINTER_IP="..."
PRINTER_API_KEY="..."
WLED_IP="..."
```
