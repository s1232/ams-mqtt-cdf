

# Power readings to Cognite Data Fusion

## Context
Minimal implementation of an MQTT client that
- assumes a MQTT broker available locally
- messages under *topic* that hold HDLC frames with a single power consumption reading
- subscribes to the messages, decodes the payloads and writes each value to Cognite Data Fusion


## Configuration
Configuration is `config.toml` in the working directory.

```toml
[mqtt]
broker_host = "localhost"
broker_port = 1883
topic = "tibber"

[cognite]
client_id = ""
client_secret = ""
token_url = ""
base_url = ""
project = ""
timeseries_space = ""
timeseries_external_id = ""
```

## Implementation details
- Writes a single data point to CDF in each request
- Panics on mqtt connection dropping, needs Systemd to restart the process


## Service setup
