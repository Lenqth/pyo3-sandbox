from pyon import RowIterator, RowReader
import unittest


class IterTest(unittest.TestCase):

    def test_file(self):
        from pathlib import Path
        path = str(Path(__file__).parent / "data/test_crlf.txt")

        res = list(RowReader(path))
        self.assertEqual(res, ["12345", "67890"])
