import sys

from fft import FFT

def main():
    with open('input.txt') as f:
        read_data = f.read().strip()
        fft = FFT(read_data)
        for x in range(100):
            sys.stdout.write("%d%%   \r" % x)
            sys.stdout.flush()
            fft.exec()
        print(fft.input)

main()
