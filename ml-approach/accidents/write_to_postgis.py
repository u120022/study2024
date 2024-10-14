import geopandas as gpd
import pandas as pd
import sqlalchemy


# input csv files
INPUT_FILES = [
    "data_2019.csv"
    "data_2020.csv"
    "data_2021.csv"
    "data_2022.csv"
]

# read input from csv files

dfs = []
for f in INPUT_FILES:
    dfs.append(pd.read_csv(f))

df = pd.concat(dfs)

# transform data

# convert deg-min-sec to digit for x
x = df["地点　経度（東経）"]
dx = x // 10000000
mx = x // 100000 % 100
sx = x / 1000.0 % 100
df["x"] = dx + mx / 60.0 + sx / 3600.0

# convert deg-min-sec to digit for y
y = df["地点　緯度（北緯）"] 
dy = y // 10000000
my = y // 100000 % 100
sy = y / 1000.0 % 100
df["y"] = dy + my / 60.0 + sy / 3600.0

df["発生日時"] = pd.to_datetime({
    "year": df["発生日時　　年"],
    "month": df["発生日時　　月"],
    "day": df["発生日時　　日"],
    "hour": df["発生日時　　時"],
    "minute": df["発生日時　　分"]
})

gdf = gpd.GeoDataFrame(
    df.drop(columns=["x", "y"]),
    geometry=gpd.points_from_xy(x, y),
    crs="EPSG:6668"
)

# write output to postgis

engine = sqlalchemy.create_engine("postgresql://postgis:0@localhost:5432/postgis")

gdf.to_postgis("accidents", engine, if_exists="replace")
