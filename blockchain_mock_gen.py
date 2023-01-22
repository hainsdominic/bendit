from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import serialization, asymmetric, hashes

import hashlib
import random
import time
import json


def get_file_hash(file_path):
    sha256 = hashlib.sha256()
    with open(file_path, 'rb') as file:
        while True:
            data = file.read(65536)
            if not data:
                break
            sha256.update(data)
    return sha256.hexdigest()


def generate_rsa_key_pair():
    private_key = asymmetric.rsa.generate_private_key(
        public_exponent=65537,
        key_size=2048,
        backend=default_backend()
    )
    public_key = private_key.public_key()
    private_pem = private_key.private_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PrivateFormat.PKCS8,
        encryption_algorithm=serialization.NoEncryption()
    )
    public_pem = public_key.public_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PublicFormat.SubjectPublicKeyInfo
    )
    return private_pem, public_pem


def encrypt_with_rsa_private_key(message: bytes, private_key: bytes) -> bytes:
    private_key = serialization.load_pem_private_key(
        private_key,
        password=None,
        backend=default_backend()
    )

    signature = private_key.sign(
        message,
        asymmetric.padding.PSS(
            mgf=asymmetric.padding.MGF1(hashes.SHA256()),
            salt_length=asymmetric.padding.PSS.MAX_LENGTH
        ),
        hashes.SHA256()
    )

    return signature


def generate_block(rsa_keys, file_hashes):
    block = {}

    block["timestamp"] = int(time.time())

    sender = random.choice(rsa_keys)

    block["sender_pub_key"] = sender[1]

    block["receiver_pub_key"] = ""

    while not block["receiver_pub_key"]:
        receiver = random.choice(rsa_keys)[1]

        if receiver != block["sender_pub_key"]:
            block["receiver_pub_key"] = receiver

    block["file_hash"] = random.choice(file_hashes)

    signature = encrypt_with_rsa_private_key(
        bytes(block["file_hash"], "utf8"), sender[0])
    block["sender_signature"] = str(signature)

    return block


def generate_block_hash(block):
    block_data = str(block["index"]) + str(block["timestamp"]) + \
        block["sender_pub_key"] + block["receiver_pub_key"] + block["file_hash"] + \
        block["sender_signature"] + block["prev_block_hash"]

    sha256 = hashlib.sha256()
    sha256.update(bytes(block_data, "utf8"))
    return sha256.hexdigest()


def generate_chain(nb_blocks):
    rsa_keys = []
    files = ["AequitasData.json", "AlphaData.json", "TSXData.json"]
    file_hashes = []

    for n in range(10):
        private, public = generate_rsa_key_pair()
        rsa_keys.append((private, public))

    for file_path in files:
        file_hashes.append(get_file_hash(file_path))

    blocks = []
    genesis_block = {
        "index": 0,
        "timestamp": int(time.time()),
        "sender_pub_key": "0",
        "receiver_pub_key": "0",
        "file_hash": "genesis",
        "sender_signature": "0",
        "prev_block_hash": "0",
    }

    genesis_block["block_hash"] = generate_block_hash(genesis_block)

    blocks.append(genesis_block)

    for n in range(nb_blocks):
        block = generate_block(rsa_keys, file_hashes)
        block["index"] = n + 1
        block["prev_block_hash"] = blocks[-1]["block_hash"]
        block["sender_pub_key"] = str(block["sender_pub_key"])
        block["receiver_pub_key"] = str(block["receiver_pub_key"])
        block["block_hash"] = generate_block_hash(block)
        blocks.append(block)

    return blocks


def main():
    chain = generate_chain(1000)

    with open("blockchain.json", "w") as f:
        f.write(json.dumps(chain))


if __name__ == "__main__":
    main()
