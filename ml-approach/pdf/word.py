import pymupdf
import geopandas as gpd
import sqlalchemy

# PDF coordinates
X0, Y0, X1, Y1 = 61.0, 70.0, 954.0, 808.0

# Map coordinates
MAP_X, MAP_Y = 8, 4
MAP = [
    [  16,   17,   18, 19,   20, 21, 22,   23],
    [None, None,   10, 11,   12, 13, 14,   15],
    [None, None,    4,  5,    6,  7,  8,    9],
    [None, None, None,  0, None,  1,  2, None],
]
TX0, TY0, TX1, TY1 = 138.9969071712898199, 35.8364566828444779, 139.9967851483917798, 35.5033003677086469

box_page = []
doc: pymupdf.Document = pymupdf.open("data.pdf")
for page in doc.pages():
    page: pymupdf.Page
    page.remove_rotation()

    box_list = []
    for x0, y0, x1, y1, text, *_ in page.get_text("words"):
        x0: float
        y0: float
        x1: float
        y1: float
        text: str

        if not text.isdecimal():
            continue

        id = int(text)
        x = ((x0 + x1) * 0.5 - X0) / (X1 - X0)
        y = ((y0 + y1) * 0.5 - Y0) / (Y1 - Y0)
        box_list.append((id, x, y))

    box_page.append(box_list)

id_list = []
x_list = []
y_list = []
for iy in range(MAP_Y):
    for ix in range(MAP_X):
        i = MAP[iy][ix]

        if i is None:
            continue

        for box in box_page[i]:
            id_list.append(box[0])
            x_list.append((box[1] + ix) / MAP_X * (TX1 - TX0) + TX0)
            y_list.append((box[2] + iy) / MAP_Y * (TY1 - TY0) + TY0)

engine = sqlalchemy.create_engine("postgresql://postgis:0@localhost:5432/postgis")

gdf = gpd.GeoDataFrame({ "id": id_list }, geometry=gpd.points_from_xy(x_list, y_list), crs="EPSG:6668")
gdf.to_postgis("detector_words", engine, if_exists="replace")
