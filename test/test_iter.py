from pyon import RowIterator, RowReader
import unittest


class IterTest(unittest.TestCase):

    def test_file(self):
        res = list(RowReader("./test.txt"))
        self.assertEqual(res, ["12345", "67890"])
