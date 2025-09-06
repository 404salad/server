## **RFC 7230 – Message Syntax and Routing**

This is the **essential one** for you.
You need to cover:

* **Message format (Section 3):**

  * Request line: `GET /path HTTP/1.1`
  * Response line: `HTTP/1.1 200 OK`
  * CRLF line endings
  * Headers must be followed by an empty line

* **Headers (Section 3.2):**

  * You must handle at least the `Host` header (mandatory in 1.1).
  * You can ignore other headers (just read and discard them).

* **Message body (Section 3.3):**

  * If you send a body, you must include `Content-Length`.

* **Connections (Section 6):**

  * Simplest: always send `Connection: close` and close after one request.
  * You can add `keep-alive` later if you want to show off.

* **Message routing (Section 7):**

  * Map `/` → `index.html`
  * Map `/path` → `www/path`

👉 That’s enough for a fully working **basic HTTP/1.1 file server**.

---

## **RFC 7231 – Semantics and Content**

This defines what methods and status codes *mean*.
For a minimal implementation, you only need:

* **Methods (Section 4):**

  * Implement `GET`
  * Optionally: `HEAD` (same as GET but no body in response)

* **Status codes (Section 6):**

  * `200 OK` – success
  * `404 Not Found` – file not found
  * `400 Bad Request` – malformed request line
  * (Optional: `500 Internal Server Error`)

---

## **What you can safely ignore (for a minimal project)**

* Caching (RFC 7234)
* Authentication (RFC 7235)
* Range requests (RFC 7233)
* Conditional requests (RFC 7232)
* Chunked transfer (7230 §4.1) → advanced, skip at first

