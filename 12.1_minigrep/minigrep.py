import argparse

parser = argparse.ArgumentParser()

parser.add_argument("query")
parser.add_argument("filepath")

args = parser.parse_args()

file = open(args.filepath)

for i, line in enumerate(file):
    if args.query in line:
        print(str(i) + " " + line, end="")
