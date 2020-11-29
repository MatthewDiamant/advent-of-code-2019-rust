class FFT2():
    def __init__(self, input):
        offset = int(input[0:7])
        self.input = input[offset:]

    def exec(self):
        output = ''
        last = 0;
        for n in self.create_input()[::-1]:
            last = (last + n) % 10
            output += str(last)
        self.input = ''.join(output[::-1])

    def create_input(self):
        return [int(x) for x in list(self.input)]
