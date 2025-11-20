#!/usr/bin/env python3

import asyncio
import sys


async def read_stdin_and_send(writer):
    """Read from stdin and send to TCP connection"""
    loop = asyncio.get_event_loop()
    reader = asyncio.StreamReader()
    protocol = asyncio.StreamReaderProtocol(reader)
    await loop.connect_read_pipe(lambda: protocol, sys.stdin)

    try:
        while True:
            line = await reader.readline()
            if not line:
                break
            writer.write(line)
            await writer.drain()
    except (ConnectionResetError, asyncio.CancelledError):
        pass
    finally:
        writer.close()
        await writer.wait_closed()


async def read_tcp_and_print(reader):
    """Read from TCP connection and print to stdout"""
    try:
        while True:
            data = await reader.read(1024)
            if not data:
                break
            print(data)
    except (ConnectionResetError, asyncio.CancelledError):
        pass


async def tcp_translation_client(host, port):
    """Main client function"""
    try:
        reader, writer = await asyncio.open_connection(host, port)
        print(f"Connected to {host}:{port}", file=sys.stderr)

        stdin_task = asyncio.create_task(read_stdin_and_send(writer))
        tcp_task = asyncio.create_task(read_tcp_and_print(reader))

        await asyncio.wait([stdin_task, tcp_task], return_when=asyncio.FIRST_COMPLETED)

        # Clean up remaining tasks
        stdin_task.cancel()
        tcp_task.cancel()
        await asyncio.gather(stdin_task, tcp_task, return_exceptions=True)

    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
    finally:
        print("\nDisconnected", file=sys.stderr)


def main():
    host = "127.0.0.1"
    port = 8124

    try:
        asyncio.run(tcp_translation_client(host, port))
    except KeyboardInterrupt:
        print("\nExiting...", file=sys.stderr)


if __name__ == "__main__":
    main()
