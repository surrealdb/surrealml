import sys
import struct


def read_from_stdin():
    # Read the tag (2 bytes)
    tag_bytes = sys.stdin.buffer.read(2)
    tag = struct.unpack('<H', tag_bytes)[0]  # Little endian 2-byte unsigned int

    # Read the length (4 bytes)
    len_bytes = sys.stdin.buffer.read(4)
    length = struct.unpack('<I', len_bytes)[0]  # Little endian 4-byte unsigned int

    # Read the data
    data = []
    for _ in range(length):
        data_bytes = sys.stdin.buffer.read(2)
        value = struct.unpack('<H', data_bytes)[0]  # Little endian 2-byte unsigned int
        data.append(value)

    return tag, length, data


if __name__ == "__main__":
    tag, length, data = read_from_stdin()
    print("Tag:", tag)
    print("Length:", length)
    # print("Data:", data)
