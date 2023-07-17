# Ferio

Everyday is a holiday

## Running The API

```bash
cd ferio-api
cargo run
```

## Running The CLI

```bash
cd ferio-cli
cargo run
```

## API Docs

- `GET /` - Get all holidays for the current day
- `GET /?date=month_day` - Get all holidays for a specific day
  - Example: `GET /?date=March_14`
