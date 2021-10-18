import hashlib
import base64
import hmac
import struct
import sys
import time

"""
Implementation of the HTOP Algorithm 

"""


def hotp(key, counter, digits=6, digest='sha1'):
    key = base64.b32decode(key.upper() + '=' * ((8 - len(key)) % 8))

    counter = struct.pack('>Q', counter)
    mac = hmac.new(key, counter, digest).digest()
    offset = mac[-1] & 0x0f
    binary = struct.unpack('>L', mac[offset:offset+4])[0] & 0x7fffffff
    return str(binary)[-digits:].zfill(digits)


"""
Implementation of the Time-Based One-Time Password Algorithm
"""


def totp(key, time_step=30, digits=6, digest='sha1'):
    return hotp(key, int(time.time() / time_step), digits, digest)


"""
The Challenge Answer implementation
"""


def answer_challenge(client_private_hash, challenge):
    totp_code = totp(challenge)
    concated = "{0}{1}".format(totp_code, client_private_hash)
    return hashlib.sha1(concated.encode()).hexdigest()


if len(sys.argv) < 3:
    print("Usage: "+sys.argv[0]+" private_hash session_challenge")
    exit(-1)
private_hash = sys.argv[1]
session_challenge = sys.argv[2]
answer = answer_challenge(private_hash, session_challenge)

print(answer, end='')
