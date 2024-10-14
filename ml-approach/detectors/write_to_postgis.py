import geopandas as gpd
import pandas as pd
import sqlalchemy


# input csv file
INPUT_FILE = "data.csv"

gpd.options.io_engine = "pyogrio"

# read input from csv file

df = pd.read_csv(INPUT_FILE)

# transform data

df = df.rename(columns={
    "情報源コード": "src_code",
    "計測地点番号": "loc_code",
    "計測地点名": "loc_name",
    "２次メッシュ番号": "mesh_code",
    "交通管理リンク番号": "link_code",
    "経度": "x",
    "緯度": "y",
})

gdf = gpd.GeoDataFrame(
    df.drop(columns=["x", "y"]),
    geometry=gpd.points_from_xy(df["x"], df["y"]),
    crs="EPSG:6668"
)

# write output to postgis

engine = sqlalchemy.create_engine("postgresql://postgis:0@localhost:5432/postgis")

gdf.to_postgis("detectors", engine, if_exists="replace")
