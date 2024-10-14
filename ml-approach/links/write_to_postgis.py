import geopandas as gpd
import osmnx.convert
import osmnx.geocoder
import osmnx.graph
import osmnx.utils_geo
import sqlalchemy


# osm search query
QUERY = "Tokyo,Japan"

gpd.options.io_engine = "pyogrio"

# read osm data from internet

df = osmnx.geocoder.geocode_to_gdf(QUERY)

# concatenate all geometries

nodes, edges = [], []
geometry = df["geometry"].unary_union
for geometry in list(geometry.geoms):
    try:
        graph = osmnx.graph.graph_from_polygon(geometry)
        print(graph)

        (node, edge) = osmnx.convert.graph_to_gdfs(graph)
        nodes.append(node[["street_count", "geometry"]])
        edges.append(edge[["highway", "oneway", "reversed", "length", "geometry"]])
    except Exception as e:
        print(e)

node = gpd.pd.concat(nodes)
edge = gpd.pd.concat(edges)

node = node.to_crs(6668)
edge = edge.to_crs(6668)

# write output to postgis

engine = sqlalchemy.create_engine("postgresql://postgis:0@localhost:5432/postgis")

node.to_postgis("nodes", engine, if_exists="replace", index=True)
edge.to_postgis("edges", engine, if_exists="replace", index=True)
