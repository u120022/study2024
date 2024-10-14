import os

import geopandas as gpd
import pandas as pd
import sqlalchemy

# input data directory
INPUT_DIR = "data"

gpd.options.io_engine = "pyogrio"

# read and concatenate all mesh shapefiles

meshs = []
for f in os.listdir(INPUT_DIR):
    if f.endswith(".shp"):
        mesh = gpd.read_file(f"{INPUT_DIR}/{f}")
        print(mesh)

        meshs.append(mesh)

mesh = pd.concat(meshs)
mesh = mesh.to_crs(6668)

# write output to postgis

engine = sqlalchemy.create_engine("postgresql://postgis:0@localhost:5432/postgis")

mesh.to_postgis("mesh", engine, if_exists="replace")
