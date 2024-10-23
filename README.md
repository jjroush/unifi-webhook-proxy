# Rust Proxy Server for UniFi Webhooks

Enjoy full-bodied payloads with this Rust proxy server for UniFi webhooks. 🚀

## Problem

UniFi Dream Machines (UDM) are great for managing your home or office network, but they have one little quirk: **they don't let you include bodies in webhook requests**. So if you want to trigger something more complex, like a service that needs custom headers or request bodies, you're stuck! It's like trying to write a novel using only tweets—possible, but frustratingly limited.

## Solution

This Rust proxy server solves that by acting as an intermediary between your UniFi Dream Machine and the real webhook you want to call. It allows you to define an endpoint that UniFi can hit, and then it forwards that request to the actual webhook **with custom headers and a body**.

This is especially useful if you need to include information from the request headers (such as `User-Agent`, `X-Custom-Header`, etc.) as part of the webhook payload.

### Why Rust?
Rust is:
- Fast ⚡️
- Safe 🛡️
- Memory-efficient 💾
- And let's be honest, it's just cool 😎.

### Why Do I Need This?

- **UniFi Webhooks without bodies**: UniFi’s built-in webhook feature is great… unless you need to send a request with a body. This proxy server lets you map headers into the body of the request, so your downstream webhook can receive more useful data.

- **Flexibility**: Easily configure which headers are included in the body and the URL of the actual webhook using a simple JSON config file.

## How It Works

1. **Configure** the proxy with your desired endpoint, webhook URL, and which headers you want to forward.
2. **Run** the Rust proxy server.
3. **Point your UniFi Dream Machine** to the proxy server as the destination for your webhook.
4. The proxy will **forward** the request to your actual webhook, appending headers into the body if needed.

## Configuration

Create a `config.json` file with the following format:

```json
{
    "endpoint": "/webhook-proxy",
    "webhook_url": "https://your-webhook-url.com",
    "headers_to_body": ["User-Agent", "X-Custom-Header"]
}
```

- `endpoint`: The path on the proxy server that UniFi will call (e.g., `/webhook-proxy`).
- `webhook_url`: The actual webhook URL that the proxy should forward the request to.
- `headers_to_body`: A list of headers that will be extracted from the incoming request and included in the body of the outgoing webhook request.

## Installation

1. Clone this repo.
2. Install the Rust toolchain if you haven't already: [Rust installation](https://www.rust-lang.org/tools/install).
3. Build the project:

   ```bash
   cargo build --release
   ```

4. Run the proxy server:

   ```bash
   ./target/release/rust-proxy-server
   ```

## Why This Matters?

Because **UniFi doesn’t allow you to add a body to webhooks**—and that’s almost as frustrating as a Wi-Fi signal drop when you're binge-watching your favorite show. This Rust proxy server fixes that issue so you can unleash the full potential of your network automations.

## License

MIT License. Enjoy and contribute!

---

**Pro Tip**: UniFi doesn't let you include a body in your webhook, but with this proxy, you can finally stop missing out on those full-bodied payloads—just like you stopped missing out on full Wi-Fi coverage after upgrading to UniFi!