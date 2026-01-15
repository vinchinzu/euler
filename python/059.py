#!/usr/bin/env python3
from pathlib import Path

def main():
    script_dir = Path(__file__).parent
    cipher_path = script_dir.parent / 'data' / 'cipher1.txt'
    
    with open(cipher_path) as f:
        cipher = [int(x) for x in f.read().strip().split(',')]

    for a in range(ord('a'), ord('z') + 1):
        for b in range(ord('a'), ord('z') + 1):
            for c in range(ord('a'), ord('z') + 1):
                key = [a, b, c]
                decrypted = [val ^ key[i % 3] for i, val in enumerate(cipher)]
                text = ''.join(chr(d) for d in decrypted)
                if ' ' in text and ' and ' in text:
                    print(sum(decrypted))
                    return

if __name__ == "__main__":
    main()
