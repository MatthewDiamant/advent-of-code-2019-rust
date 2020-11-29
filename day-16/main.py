import sys

from fft import FFT
from fft2 import FFT2

with open('input.txt') as f:
    read_data = f.read().strip()

    # Part 1
    fft = FFT(read_data)
    for x in range(100):
        sys.stdout.write("%d%%   \r" % x)
        sys.stdout.flush()
        fft.exec()
    print(fft.input[0:8])

    # Part 2
    fft2 = FFT2(read_data * 10000)
    for x in range(100):
        sys.stdout.write("%d%%   \r" % x)
        sys.stdout.flush()
        fft2.exec()
    print(fft2.input[0:8])
