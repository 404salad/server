server
--- 

Must-Haves (for a thin but compliant server)

1. Request Line & Method
Support at least GET (RFC 7231 §4.3.1).
Reject unknown or malformed request lines with 400 Bad Request (RFC 7231 §6.5.1).
Host header is mandatory in HTTP/1.1 (RFC 7230 §5.4). If absent, respond 400.

2. Header Handling
Read until CRLFCRLF (\r\n\r\n) not just \n\n (RFC 7230 §3.5).
Ignore unknown headers (don’t choke).
Recognize Connection: close (RFC 7230 §6.1) and honor it (don’t keep socket open).

3. Response Formatting
Status line: HTTP/1.1 \<code\> \<reason\> (RFC 7230 §3.1.2).
Required headers:
Date (MUST, RFC 7231 §7.1.1.2). i checked its not a must!!! my valid reason in speedddd (also timestamps)
Content-Length (MUST if not chunked, RFC 7230 §3.3.2). ok did it but its a should
Content-Type (RFC 7231 §3.1.1.5).
End headers with \r\n\r\n.

4. File Serving
Canonicalize paths and enforce root restriction (you already do).
If directory and trailing /, serve index.html. If missing, send 404. (done)
Respond with 404 Not Found and 403 Forbidden when applicable (RFC 7231 §6.5.3, §6.5.4).

5. Connection Behavior
Default to persistent connections (RFC 7230 §6.3), but you can simplify by always closing with Connection: close.

Not yet implemented 

Concurrency → Right now,  processes each connection in a blocking loop. If one client is slow, others wait. A “concurrent server” would need thread::spawn, async, or a threadpool.

Efficient filesystem I/O → It reads the whole file into memory (fs::read). That works, but isn’t scalable for large files. More efficient would be buffered streaming (e.g., File with copy or sendfile).

MIME types → It always sets Content-Type: text/html, even for .css, .png, and .js. A real server would infer the MIME type from the file extension.
