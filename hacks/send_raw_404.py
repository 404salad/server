import socket

path = "../notes/newwords.txt"
request = f"GET {path} HTTP/1.1\r\nHost: localhost\r\n\r\n"

with socket.create_connection(("localhost", 9999)) as s:
    s.sendall(request.encode("utf-8"))
    
    chunks = []
    while True:
        data = s.recv(4096)
        if not data:  # server closed connection
            break
        chunks.append(data)

    response = b"".join(chunks)
    print(response.decode("utf-8", errors="replace"))
