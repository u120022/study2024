import pandas as pd
import geopandas as gpd
import sqlalchemy

df = pd.concat([
    pd.read_csv("data_2019.csv"),
    pd.read_csv("data_2020.csv"),
    pd.read_csv("data_2021.csv"),
    pd.read_csv("data_2022.csv"),
])

# convert deg-min-sec to digit for x
x = df["地点　経度（東経）"]
dx = x // 10000000
mx = x // 100000 % 100
sx = x / 1000.0 % 100
x = dx + mx / 60.0 + sx / 3600.0

# convert deg-min-sec to digit for y
y = df["地点　緯度（北緯）"] 
dy = y // 10000000
my = y // 100000 % 100
sy = y / 1000.0 % 100
y = dy + my / 60.0 + sy / 3600.0

date = pd.to_datetime({
    "year": df["発生日時　　年"],
    "month": df["発生日時　　月"],
    "day": df["発生日時　　日"],
    "hour": df["発生日時　　時"],
    "minute": df["発生日時　　分"]
})
df["発生日時"] = date

gdf = gpd.GeoDataFrame(
    df,
    geometry=gpd.points_from_xy(x, y),
    crs="EPSG:6668"
)

engine = sqlalchemy.create_engine("postgresql://postgis:0@localhost:5432/postgis")
gdf.to_postgis("accidents", engine)
