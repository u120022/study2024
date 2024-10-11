import pymupdf

ZOOM = 2.0
X0, Y0, X1, Y1 = 61.0, 70.0, 954.0, 808.0
MAP = [
    [  16,   17,   18, 19,   20, 21, 22,   23],
    [None, None,   10, 11,   12, 13, 14,   15],
    [None, None,    4,  5,    6,  7,  8,    9],
    [None, None, None,  0, None,  1,  2, None],
]

clip = pymupdf.IRect(X0, Y0, X1, Y1)
matrix = pymupdf.Matrix(ZOOM, ZOOM)

width, height = 0, 0
pixs: list[pymupdf.Pixmap] = []
doc: pymupdf.Document = pymupdf.open("data.pdf")
for page in doc.pages():
    page: pymupdf.Page
    pix: pymupdf.Pixmap = page.get_pixmap(matrix=matrix, clip=clip)
    width, height = pix.width, pix.height
    pixs.append(pix)

pix = pymupdf.Pixmap(pymupdf.csRGB, pymupdf.IRect(0, 0, width * 8, height * 4))
for iy in range(4):
    for ix in range(8):
        i = MAP[iy][ix]

        if i is None:
            continue

        pix.set_origin(0, 0)
        pixs[i].set_origin(ix * width, iy * height)
        pix.copy(pixs[i], pymupdf.IRect(ix * width, iy * height, (ix + 1) * width, (iy + 1) * height))

pix.save("output.png")
