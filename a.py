from pyon import RowIterator, RowReader

# print(CSVLoader("test.txt").open().read())

for row in RowReader("./test.txt"):
    print(row)
