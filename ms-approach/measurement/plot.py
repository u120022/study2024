import sqlite3
import pandas as pd
import matplotlib.pyplot as plt

def main():
    with sqlite3.connect("track.db") as conn:
        df = pd.read_sql("SELECT *, (x1 + x2) / 2 as x, (y1 + y2) / 2 as y FROM track_flat WHERE name = \"person\" OR name = \"car\" OR name = \"bus\" OR name = \"truck\"", conn)
        print(df)

        fig, ax = plt.subplots()
        ax.scatter(df["x"], df["y"], s=0.1, c=df["track_id"], cmap="hsv")
        ax.set_xlim(0, 1920)
        ax.set_ylim(0, 1080)
        ax.invert_yaxis()
        fig.savefig("plot.png")


if __name__ == "__main__":
    main()
