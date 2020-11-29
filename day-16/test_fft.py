import unittest
from fft import FFT

class Day16Test(unittest.TestCase):
    def test_example_1(self):
        example_1 = FFT('12345678')
        example_1.exec()
        self.assertEqual(example_1.input, '48226158')
        example_1.exec()
        self.assertEqual(example_1.input, '34040438')
        example_1.exec()
        self.assertEqual(example_1.input, '03415518')
        example_1.exec()
        self.assertEqual(example_1.input, '01029498')

    def test_example_2(self):
        example_2 = FFT('80871224585914546619083218645595')
        for x in range(100):
            example_2.exec()
        self.assertEqual(example_2.input[0:8], '24176176')

    def test_example_3(self):
        example_3 = FFT('19617804207202209144916044189917')
        for x in range(100):
            example_3.exec()
        self.assertEqual(example_3.input[0:8], '73745418')

    def test_example_4(self):
        example_4 = FFT('69317163492948606335995924319873')
        for x in range(100):
            example_4.exec()
        self.assertEqual(example_4.input[0:8], '52432133')

if __name__ == '__main__':
    unittest.main()
