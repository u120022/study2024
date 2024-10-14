import ast
import os
import sqlite3

import fire
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import pytransform3d.transformations as pt


# read the structured data from csv and write to sqlite3
def read_from_csv(input="track.csv", output="track.db"):
    if not os.path.isfile(input):
        raise FileNotFoundError(f"{input} not found")

    track = pd.read_csv(input, index_col=0)
    print(f"finished reading to {input}")

    print("start to flatten the structured data field")
    box = track["box"].apply(lambda x: ast.literal_eval(x))
    track["x1"] = box.apply(lambda x: x.get("x1"))
    track["y1"] = box.apply(lambda x: x.get("y1"))
    track["x2"] = box.apply(lambda x: x.get("x2"))
    track["y2"] = box.apply(lambda x: x.get("y2"))
    track.drop(columns=["box"], inplace=True)

    with sqlite3.connect(output) as conn:
        track.to_sql("track_flat", conn, if_exists="replace")
        print(f"finished writing to {output}")


# plot the track data
def plot(input="track.db", output="output.png"):
    if not os.path.isfile(input):
        raise FileNotFoundError(f"{input} not found")

    print(f"start to read from {input}")
    with sqlite3.connect(input) as conn:
        df = pd.read_sql(
            "SELECT *, (x1 + x2) / 2 as x, (y1 + y2) / 2 as y FROM track_flat WHERE name = \"person\" OR name = \"car\" OR name = \"bus\" OR name = \"truck\"",
            conn
        )
        print(df)

        fig, ax = plt.subplots()

        print("start to draw the plot")
        ax.scatter(df["x"], df["y"], s=0.1, c=df["track_id"], cmap="hsv")
        ax.set_xlim(0, 1920)
        ax.set_ylim(0, 1080)
        ax.invert_yaxis()

        fig.savefig(output)
        print(f"finished writing to {output}")


# transform to world-space from screen-space
def transform():
    cam2world = pt.transform_from_pq([0, 0, 0, 0.723343, 0.677349, 0.089033, 0.100232])
    sensor_width = 36 * 0.001
    f = 67.0176 * 0.001

    sensor_size = np.array([sensor_width, sensor_width * (1080 / 1920)])
    cam2img = np.array([
        [f, 0, sensor_size[0] / 2.0, 0],
        [0, f, sensor_size[1] / 2.0, 0],
        [0, 0, 1, 0]
    ])
    img2cam = np.invert(cam2img)

    p = np.array([0.0, 0.0, 0.0])
    p = np.dot(cam2world, np.dot(img2cam, p))


# run the task
if __name__ == "__main__":
    fire.Fire({ "import": read_from_csv, "plot": plot, "transform": transform })
