using Sockets

ip = ip"169.254.11.17"
port = 50000

socket = Sockets.connect(ip, port)

println(socket, "X.")
x = readavailable(socket)
val = Char.(x)

println(val)

Sockets.close(socket)