import socket
import time
import math

UDP_IP = "192.168.1.100" # REPLACE WITH YOUR ESP32 IP
UDP_PORT = 7777
LEDS_PER_DIGIT = 7
NUM_DIGITS = 4 # Total digits in the system

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

def get_color(t, offset):
    r = int((math.sin(t + offset) + 1) * 127)
    g = int((math.sin(t + offset + 2) + 1) * 127)
    b = int((math.sin(t + offset + 4) + 1) * 127)
    return [r, g, b]

print(f"Sending UDP to {UDP_IP}:{UDP_PORT}")

try:
    start_time = time.time()
    while True:
        t = time.time() - start_time
        
        packet = []
        # Generate data for ALL digits
        for i in range(NUM_DIGITS * LEDS_PER_DIGIT):
            # Make a rainbow wave
            color = get_color(t * 5, i * 0.5)
            packet.extend(color)
            
        data = bytes(packet)
        sock.sendto(data, (UDP_IP, UDP_PORT))
        
        time.sleep(0.016) # ~60 FPS

except KeyboardInterrupt:
    print("Stopped")
