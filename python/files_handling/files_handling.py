# -*- coding:utf-8 -*-

import os


def read(file_path):
    lines_upper = []
    with open(file_path) as file:
        for line in file:
            lines_upper.append(line.upper())
    return lines_upper


def create_and_write(lines, file_path):
    """Not raising exceptions only for accurate reconstruction of rust's example."""
    try:
        with open(file_path, 'w') as file:
            for line in lines:
                try:
                    file.write(f"{line} Length {len(line)}\n")
                except Exception:
                    print("Failed to save line")
    except Exception:
        print("Failed to create and save file")


def delete(file_name):
    try:
        os.remove(file_name)
    except Exception:
        print("Can't remove file")


if __name__ == '__main__':
    lines_with_len = read("zen.txt")
    create_and_write(lines_with_len, "zen_with_len.txt")
    delete("zen_with_len.txt")
