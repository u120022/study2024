import geopandas as gpd
import pymupdf
import sqlalchemy


# input pdf file
INPUT_FILE = "input.pdf"
# pdf scale
X0, Y0, X1, Y1 = 61.0, 70.0, 954.0, 808.0
# arrangement of the pdf pages
MAP_X, MAP_Y = 8, 4
MAP: list[list[int | None]] = [
    [  16,   17,   18, 19,   20, 21, 22,   23],
    [None, None,   10, 11,   12, 13, 14,   15],
    [None, None,    4,  5,    6,  7,  8,    9],
    [None, None, None,  0, None,  1,  2, None],
]
# available geo destination area
TX0, TY0, TX1, TY1 = 138.9969071712898199, 35.8364566828444779, 139.9967851483917798, 35.5033003677086469

gpd.options.io_engine = "pyogrio"

# read input from pdf file

doc = pymupdf.open(INPUT_FILE)

# collection available textbox

box_page = []
for page in doc.pages():
    if not page:
        raise Exception("Page is None")

    page.remove_rotation()

    box_list = []
    for x0, y0, x1, y1, text, *_ in page.get_text("words"):
        if not text.isdecimal():
            continue

        id = int(text)
        x = ((x0 + x1) * 0.5 - X0) / (X1 - X0)
        y = ((y0 + y1) * 0.5 - Y0) / (Y1 - Y0)
        box_list.append((id, x, y))

    box_page.append(box_list)

# concatenate available textbox

id_list = []
x_list = []
y_list = []
for y in range(MAP_Y):
    for x in range(MAP_X):
        index = MAP[y][x]

        if index is None:
            continue

        for box in box_page[index]:
            id_list.append(box[0])
            x_list.append((box[1] + x) / MAP_X * (TX1 - TX0) + TX0)
            y_list.append((box[2] + y) / MAP_Y * (TY1 - TY0) + TY0)

gdf = gpd.GeoDataFrame(
    { "id": id_list },
    geometry=gpd.points_from_xy(x_list, y_list),
    crs="EPSG:6668"
)

# write output to postgis

engine = sqlalchemy.create_engine("postgresql://postgis:0@localhost:5432/postgis")

gdf.to_postgis("detector_words", engine, if_exists="replace")

# post-processing

with engine.connect() as conn:
    with open("mapping.sql") as query:
        conn.execute(sqlalchemy.text(query.read()))

    conn.commit()
