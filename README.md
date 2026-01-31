# [WIP] prusa‑wled‑rs

A tiny **finite‑state automaton** that talks to a Prusa printer and drives a WLED‑compatible LED strip to visualise heating and printing progress.

---  
## Schema
- No connection → Connected → Heating → Printing → Finished
---  

## Quick start  

```bash
# Clone & build
git clone https://github.com/dnlmos/prusa-wled-rs.git
cd prusa-wled-rs
cargo build --release

# Set environment (you can put these in a .env file)
export PRINTER_IP="http://192.168.1.42"
export PRINTER_API_KEY="your‑printer‑api‑key"
export WLED_IP="http://192.168.1.77"

# Run
./target/release/prusa-wled-rs
```

1. **Connection check** – polls the printer every 5 s.  
2. **Job detection** – when a job starts, the LED strip switches to *operational* colour.  
3. **Heating** – every 5 s reads the bed/heater temperature and renders a progress bar.  
4. **Printing** – every 10 s reads the current print completion and updates the strip.  
5. **Finished / No job** – restores the saved pre‑run WLED state.
