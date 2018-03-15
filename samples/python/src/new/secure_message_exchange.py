"""
Example demonstrating how to use Indy SDK to exchange messages in a secure way
over insecure network (UDP protocol), providing security and message authentication

Intended to be executed from two terminals and used interactively.

First, user generates its DID and Verkey in his wallet that will be used for this
communication. After that, correspondent's DID and Verkey are entered manually (in
real world usage, they would be exchanged through other channels)

After that, indy crypto's auth_crypt and auth_decrypt functionality is used for
encrypting and decrypting messages, and verifying the correspondent's Verkey
"""


import asyncio
import time
import re
import socket
from queue import Queue

import sys
sys.path.insert(0, '/home/vagrant/code/evernym/indy-sdk/wrappers/python')

from indy import crypto, did, wallet


def print_colored(colored="", default=""):
    """set the colors for text."""
    HEADER = '\033[1;33m'
    ENDC = '\033[0m'
    print(HEADER + colored + ENDC + default)


class SimpleProtocol(asyncio.DatagramProtocol):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._received_datagrams = Queue()

    def connection_made(self, transport):
        self.transport = transport

    def datagram_received(self, datagram, address):
        self._received_datagrams.put((datagram, address))

    async def get_message(self):
        await asyncio.sleep(0.1)

        if not self._received_datagrams.empty():
            msg = self._received_datagrams.get()
            self._received_datagrams.task_done()
            return msg


async def open_socket_async(port):
    loop = asyncio.get_event_loop()

    try:
        comm = await loop.create_datagram_endpoint(lambda: SimpleProtocol(),
                                                   local_addr=('0.0.0.0', port))
    except OSError as e:
        print('Failed to open socket on port: %s. Exception: %s' % (port, e))
    else:
        print_colored('Successfully opened socket on port: ', str(port))
        return comm


async def init():
    me = input('Who are you? ').strip()
    wallet_name = '%s-wallet' % me

    try:
        await wallet.create_wallet('pool1', wallet_name, None, None, None)
    except:
        pass

    wallet_handle = await wallet.open_wallet(wallet_name, None, None)
    print_colored('Successfully opened wallet')
    print_colored('Creating DID and Key Pair...')
    my_did, my_vk = await did.create_and_store_my_did(wallet_handle, '{}')
    print_colored('DID and Verification Key: ', '%s %s' % (my_did, my_vk))

    their = input("Other party's DID and verkey? ").strip().split(' ')
    return wallet_handle, my_did, my_vk, their[0], their[1]


async def send_secure_message_async(wallet_handle, my_vk, their_vk, transport, remote_host, remote_port, message):
    message = message.encode()
    encrypted = bytes(await crypto.auth_crypt(wallet_handle, my_vk, their_vk, message))
    remote_address = (remote_host, remote_port)
    print_colored('Message: ', message.decode())
    print_colored('Encrypted message: ', str(encrypted))
    print_colored('Sending encrypted message to: ', '%s %s' % (remote_host, remote_port))
    transport.sendto(encrypted, remote_address)


async def receive_message_async(protocol):
    print_colored('Checking incoming messages.')
    msg = await protocol.get_message()
    if not msg:
        print('No messages yet. Try later')
        return

    data, sender = msg
    print_colored('New incoming message: ', str(data))
    print_colored('Sender: ', '%s %s' % (sender[0], sender[1]))
    return data, sender


async def decrypt(wallet_handle, my_vk, raw_message):
    print_colored('Decrypting message...')
    did, decrypted_message = await crypto.auth_decrypt(wallet_handle, my_vk, raw_message)
    print_colored('Verkey of the message sender: ', did)
    print_colored('Decrypted content: ', decrypted_message.decode())


async def demo():
    wallet_handle, my_did, my_vk, their_did, their_vk = await init()
    encoded_message = None
    transport = None
    protocol = None


    while True:
        argv = input('> ').strip().split(' ')
        cmd = argv[0].lower()
        if re.match(cmd, 'prep'):
            message = ' '.join(argv[1:])
            await prep(wallet_handle, my_vk, their_vk, message)
        elif re.match(cmd, 'read'):
            await read(wallet_handle, my_vk)
        elif re.match(cmd, 'quit'):
            break
        elif re.match(cmd, 'open_port'):
            port = int(argv[1])
            transport, protocol = await open_socket_async(port)
        elif re.match(cmd, 'send'):
            remote_host = argv[1]
            remote_port = int(argv[2])
            message = ' '.join(argv[3:])
            await send_secure_message_async(wallet_handle, my_vk, their_vk, transport, remote_host, remote_port, message)
        elif re.match(cmd, 'receive'):
            encoded_message, _ = await receive_message_async(protocol)
        elif re.match(cmd, 'decrypt'):
            await decrypt(wallet_handle, my_vk, encoded_message)
        else:
            print('Huh?')


if __name__ == '__main__':
    try:
        loop = asyncio.get_event_loop()
        loop.run_until_complete(demo())
        time.sleep(1)
    except KeyboardInterrupt:
        print('')
