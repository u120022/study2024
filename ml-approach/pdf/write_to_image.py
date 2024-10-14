import pymupdf


# input pdf file
INPUT_FILE = "input.pdf"
# output pdf file
OUTPUT_FILE = "output.pdf"
# pdf scale
ZOOM = 2.0
# available pdf source area
X0, Y0, X1, Y1 = 61.0, 70.0, 954.0, 808.0
# arrangement of the pdf pages
MAP_W, MAP_H = 8, 4
MAP: list[list[int | None]] = [
    [  16,   17,   18, 19,   20, 21, 22,   23],
    [None, None,   10, 11,   12, 13, 14,   15],
    [None, None,    4,  5,    6,  7,  8,    9],
    [None, None, None,  0, None,  1,  2, None],
]

clip = pymupdf.IRect(X0, Y0, X1, Y1)
matrix = pymupdf.Matrix(ZOOM, ZOOM)

# read input from pdf file

doc = pymupdf.open(INPUT_FILE)

# collection available pixmap

width, height = 0, 0
pixmaps: list[pymupdf.Pixmap] = []

for page in doc.pages():
    if not page:
        raise Exception("Page is None")

    pixmap: pymupdf.Pixmap = page.get_pixmap(matrix=matrix, clip=clip)
    if not pixmap:
        raise Exception("Pixmap is None")
    
    width, height = pixmap.width, pixmap.height
    pixmaps.append(pixmap)

# concatenate available pixmap

global_pixmap = pymupdf.Pixmap(pymupdf.csRGB, pymupdf.IRect(0, 0, width * 8, height * 4))
for y in range(MAP_H):
    for x in range(MAP_W):
        index = MAP[y][x]

        if index is None:
            continue

        pixmaps[index].set_origin(x * width, y * height)

        global_pixmap.set_origin(0, 0)
        global_pixmap.copy(pixmaps[index], pymupdf.IRect(x * width, y * height, (x + 1) * width, (y + 1) * height))

# write output to pdf file

global_pixmap.save(OUTPUT_FILE)
