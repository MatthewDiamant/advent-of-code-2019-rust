BASE_OPERATOR_PATTERN=[0, 1, 0, -1]

class FFT():
    def __init__(self, input):
        self.input = input

    def exec(self):
        input = self.create_input()
        output = []
        for i, digit in enumerate(input, start=1):
            operator_pattern = self.create_operator_pattern(i)
            output.append(self.create_new_digit(input, operator_pattern))
        self.input = ''.join(output)

    def create_operator_pattern(self, repeat):
        pattern = []
        for element in BASE_OPERATOR_PATTERN:
            for n in range(repeat):
                pattern.append(element)
        return pattern

    def create_input(self):
        return [int(x) for x in list(self.input)]

    def create_new_digit(self, input, operator_pattern):
        new_digit_summands = [
            operand * operator_pattern[i % len(operator_pattern)]
                for i, operand in enumerate(input, start=1)
        ]
        return str(sum(new_digit_summands))[-1]
