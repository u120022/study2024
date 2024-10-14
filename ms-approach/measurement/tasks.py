import ast
import os
import sqlite3
import argparse

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import pytransform3d.rotations


# read the structured data from csv and write to sqlite3
def read_from_csv(args):
    if not os.path.isfile(args.input):
        raise FileNotFoundError(f"{args.input} not found")

    track = pd.read_csv(args.input, index_col=0)
    print(f"finished reading to {input}")

    print("start to flatten the structured data field")
    box = track["box"].apply(lambda x: ast.literal_eval(x))
    track["x1"] = box.apply(lambda x: x.get("x1"))
    track["y1"] = box.apply(lambda x: x.get("y1"))
    track["x2"] = box.apply(lambda x: x.get("x2"))
    track["y2"] = box.apply(lambda x: x.get("y2"))
    track.drop(columns=["box"], inplace=True)

    with sqlite3.connect(args.output) as conn:
        track.to_sql("track_flat", conn, if_exists="replace")
        print(f"finished writing to {args.output}")


# transform to world-space from screen-space
def transform(args):
    Z_BASIS = 1.0

    f = args.f
    if args.sensor_fit == "horizontal":
        sensor_size = np.transpose(np.array([[args.sensor_width, args.sensor_width * (args.image_height / args.image_width)]]))
    elif args.sensor_fit == "vertical":
        sensor_size = np.transpose(np.array([[args.sensor_height * (args.image_width / args.image_height), args.sensor_height]]))
    else:
        raise ValueError("sensor_fit must be either \"horizontal\" or \"vertical\"")
    resolution = np.transpose(np.array([[args.image_width, args.image_height]]))
    cam2world = pytransform3d.rotations.matrix_from_quaternion([args.qx, args.qy, args.qz, args.qw])

    if not os.path.isfile(args.input):
        raise FileNotFoundError(f"{args.input} not found")

    print(f"start to read from {args.input}")
    with sqlite3.connect(args.input) as conn:
        df = pd.read_sql(
            """
            SELECT 
                *,
                (x1 + x2) / 2 as x,
                (y1 + y2) / 2 as y
            FROM
                track_flat
            WHERE
                name = "person"
                OR name = "car"
                OR name = "bus"
                OR name = "truck"
            """,
            conn
        )
        print(df)

        print("start to transform coordinate")
        batch_size = len(df.index)
        track_data = [df["x"].values, df["y"].values]
        # pixel coordinate point (x, y)
        p_p = np.array(track_data)
        # camera coordinate point (x, y, z)
        p_c = np.append(Z_BASIS / f * (p_p / resolution - 0.5) * sensor_size, np.full([1, batch_size], Z_BASIS), axis=0)
        # world coordinate vector (x, y, z)
        u_w = np.dot(cam2world, p_c)
        # world coordinate point (x, y, z)
        p_w = np.reshape(args.y_base / u_w[1], [1, batch_size]) * u_w
        print(p_w)

        df.drop(columns=["x1", "y1", "x2", "y2", "x", "y"], inplace=True)
        df["x"] = p_w[0]
        df["z"] = p_w[2]
        df.to_sql("track_world", conn, if_exists="replace")
        print(f"finished writing to {args.input}")


# plot the track data
def plot(args):
    if not os.path.isfile(args.input):
        raise FileNotFoundError(f"{args.input} not found")

    print(f"start to read from {args.input}")
    with sqlite3.connect(args.input) as conn:
        # draw screen-space track data

        df = pd.read_sql(
            """
            SELECT 
                *,
                (x1 + x2) / 2 as x,
                (y1 + y2) / 2 as y
            FROM
                track_flat
            WHERE
                name = "person"
                OR name = "car"
                OR name = "bus"
                OR name = "truck"
            """,
            conn
        )
        print(df)

        print("start to draw the plot")
        fig, ax = plt.subplots()
        ax.scatter(df["x"], df["y"], s=0.1, c=df["track_id"], cmap="hsv")
        ax.set_xlim(0, args.image_width)
        ax.set_ylim(0, args.image_height)
        ax.invert_yaxis()

        fig.savefig(args.output_screen_space)
        print(f"finished writing to {args.output_screen_space}")

        # draw world-space track data

        df = pd.read_sql("SELECT * FROM track_world", conn)
        print(df)

        print("start to draw the plot")
        fig, ax = plt.subplots()
        ax.scatter(df["x"], df["z"], s=0.1, c=df["track_id"], cmap="hsv")
        ax.set_xlim(args.min_x, args.max_x)
        ax.set_ylim(args.min_y, args.max_y)
        ax.invert_yaxis()

        fig.savefig(args.output_world_space)
        print(f"finished writing to {args.output_world_space}")


# run the task
if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    subparser = parser.add_subparsers()

    import_parser = subparser.add_parser("import")
    import_parser.add_argument("--input", default="track.csv", type=str)
    import_parser.add_argument("--output", default="track.db", type=str)
    import_parser.set_defaults(func=read_from_csv)

    transform_parser = subparser.add_parser("transform")
    transform_parser.add_argument("--input", default="track.db", type=str)
    transform_parser.add_argument("--f", default=0.067017, type=float)
    transform_parser.add_argument("--sensor-width", default=0.036, type=float)
    transform_parser.add_argument("--sensor-height", default=0.024, type=float)
    transform_parser.add_argument("--sensor-fit", default="horizontal", type=str, choices=["horizontal", "vertical"])
    transform_parser.add_argument("--image-width", default=1920, type=int)
    transform_parser.add_argument("--image-height", default=1080, type=int)
    transform_parser.add_argument("--qx", default=0.0325227864, type=float)
    transform_parser.add_argument("--qy", default=0.990438759, type=float)
    transform_parser.add_argument("--qz", default=-0.00791876111, type=float)
    transform_parser.add_argument("--qw", default=0.133830741, type=float)
    transform_parser.add_argument("--y_base", default=-10.0, type=float)
    transform_parser.set_defaults(func=transform)

    plot_parser = subparser.add_parser("plot")
    plot_parser.add_argument("--input", default="track.db", type=str)
    plot_parser.add_argument("--output-screen-space", default="output-screen-space.png", type=str)
    plot_parser.add_argument("--output-world-space", default="output-world-space.png", type=str)
    plot_parser.add_argument("--image-width", default=1920, type=int)
    plot_parser.add_argument("--image-height", default=1080, type=int)
    plot_parser.add_argument("--min-x", default=-20, type=int)
    plot_parser.add_argument("--min-y", default=0, type=int)
    plot_parser.add_argument("--max-x", default=100, type=int)
    plot_parser.add_argument("--max-y", default=-120, type=int)
    plot_parser.set_defaults(func=plot)

    args = parser.parse_args()
    args.func(args)
