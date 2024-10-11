import sqlite3
import ast
import pandas as pd

def main():
    track = pd.read_csv("track.csv", index_col=0)
    print("finished reading to track.csv")

    print("start to flatten the structured data field")
    box = track["box"].apply(lambda x: ast.literal_eval(x))
    track["x1"] = box.apply(lambda x: x.get("x1"))
    track["y1"] = box.apply(lambda x: x.get("y1"))
    track["x2"] = box.apply(lambda x: x.get("x2"))
    track["y2"] = box.apply(lambda x: x.get("y2"))
    track.drop(columns=["box"], inplace=True)

    with sqlite3.connect("track.db") as conn:
        track.to_sql("track_flat", conn, if_exists="replace")
        print("finished writing to track.db")


if __name__ == "__main__":
    main()
