from pyon import RowIterator, RowReader
import unittest


class IterTest(unittest.TestCase):

    def test_file(self):
        from pathlib import Path
        path = str(Path(__file__).parent / "data/test_crlf.txt")

        res = list(RowReader(path))
        self.assertEqual(res, ["12345", "67890"])

    def test_multiple(self):
        from pathlib import Path
        path = str(Path(__file__).parent / "data/test_crlf.txt")

        res1 = []
        for row in RowReader(path):
            res1.append(row)

        res2 = []
        for row in RowReader(path):
            res2.append(row)

        self.assertEqual(res1, ["12345", "67890"])
        self.assertEqual(res2, ["12345", "67890"])

    def test_multiple2(self):
        from pathlib import Path
        path = str(Path(__file__).parent / "data/test_crlf.txt")

        res1 = []
        res2 = []
        for row1, row2 in zip(RowReader(path), RowReader(path)):
            res1.append(row1)
            res2.append(row2)

        self.assertEqual(res1, ["12345", "67890"])
        self.assertEqual(res2, ["12345", "67890"])
