# API

## GET /health

Returns project, network, and service status.

## GET /api/actions

Returns seeded public-sector records.

## POST /api/actions/quote

Accepts:

```json
{
  "recordId": "proof-of-payment-for-government-services-001",
  "amount": "125.50",
  "asset": "XLM",
  "destination": "G..."
}
```

Returns a Stellar-oriented quote with memo, SEP-7 URI, and Soroban method name.
