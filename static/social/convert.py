# Batch SVG to PNG renderer - CTCL 2024
# Purpose: Renders all SVG files in the current directory
# Created: May 9, 2024
# Modified: May 9, 2024

from cairosvg import svg2png
from os import listdir
from os.path import isfile, join

paths = [f for f in listdir(".") if isfile(join(".", f))]
# Only select files with the extension .svg
paths = [f for f in paths if f.endswith(".svg")]

# Use a function so "return" would stop the script when needed
def main():
    if len(paths) < 1:
        print("No SVG files found in the current directory")
        return

    for fpath in paths
        with open(fpath) as f:
            svg = f.read()
    
        # Just replace the last item. This is in case there is a file name like "image.txt.svg".
        newfilename = "".join(fpath.split(".")[:-1].append(".png"))

        svg2png(bytesring = svg, write_to = newfilename)

main()
