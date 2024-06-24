import sqlalchemy
import osmnx.graph
import osmnx.convert
import osmnx.geocoder
import osmnx.utils_geo
import geopandas as gpd

gpd.options.io_engine = "pyogrio"

nodes, edges = [], []
df = osmnx.geocoder.geocode_to_gdf("Tokyo,Japan")
geometry = df["geometry"].unary_union
for geometry in list(geometry.geoms):
    try:
        g = osmnx.graph.graph_from_polygon(geometry)
        print(g)

        (node, edge) = osmnx.convert.graph_to_gdfs(g)
        nodes.append(node[["street_count", "geometry"]])
        edges.append(edge[["highway", "length", "geometry"]])
    except Exception as e:
        print(e)

node = gpd.pd.concat(nodes)
edge = gpd.pd.concat(edges)
engine = sqlalchemy.create_engine("postgresql://postgis:0@localhost:5432/postgis")
node.to_postgis("nodes", engine, if_exists="replace")
edge.to_postgis("edges", engine, if_exists="replace")
