import os
import pandas as pd
import geopandas as gpd
import sqlalchemy

gpd.options.io_engine = "pyogrio"

dfs = []
for f in os.listdir("./data"):
    if f.endswith("shp"):
        df = gpd.read_file(f"./data/{f}")
        df = df.to_crs(6668)
        dfs.append(df)
df = pd.concat(dfs)

print(df)

engine = sqlalchemy.create_engine("postgresql://postgis:0@localhost:5432/postgis")
df.to_postgis("mesh", engine)
