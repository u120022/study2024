import argparse
import datetime
import glob
import pathlib
import subprocess

import pandas as pd
import geopandas as gpd
import sqlalchemy


def upload(args):
    dfs = []

    for path in args.input:
        filename = pathlib.Path(path).name

        if args.header:
            header = "infer"
        else:
            header = None

        if args.utc:
            td = datetime.timedelta(hours=9)
        else:
            td = datetime.timedelta(hours=0)

        df = pd.read_csv(path, header=header)
        df = df.rename(columns={ 0: "dt", 1: "x", 2: "y" })

        df["dt"] = pd.to_datetime(df["dt"], format="%Y-%m-%d %H:%M:%S") + td
        df["filename"] = filename

        dfs.append(df)

    df = pd.concat(dfs, ignore_index=True)
    print(df)

    gdf = gpd.GeoDataFrame(df[["dt", "filename"]], geometry=gpd.points_from_xy(df["x"], df["y"]))
    print(gdf)

    engine = sqlalchemy.create_engine("postgresql://postgres:0@localhost:5433/postgres")
    gdf.to_postgis("gps", engine, if_exists="replace")


def integrate(args):
    conf = pd.read_csv(args.conf)
    print(conf)

    root_dir = pathlib.Path(args.conf).parent
    print(root_dir)

    for _, row in conf.iterrows():
        print(row["id"])

        target_df = pd.read_csv(root_dir / row["target"])
        print(target_df.head())

        event_df = pd.read_csv(root_dir / row["event"])
        print(event_df.head())

        ranges = []
        for video in glob.glob(str(root_dir / row["video"])):
            video_name = pathlib.Path(video).stem
            video_dt = datetime.datetime.strptime(video_name, "%Y%m%d%H%M%S")

            proc = subprocess.run(["ffprobe", "-v", "error", "-show_entries", "format=duration", "-of", "default=noprint_wrappers=1:nokey=1", video], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            video_duration = float(proc.stdout.decode())

            dt0 = video_dt
            dt1 = video_dt + datetime.timedelta(seconds=video_duration)

            # print("{}: {} - {}".format(video, dt0, dt1))
            ranges.append((dt0, dt1, video_duration, video))
        ranges.sort()

        for j, event in event_df.iterrows():
            target_dt0 = pd.to_datetime(event.start)
            target_dt1 = pd.to_datetime(event.end)

            min_i = None
            max_i = None
            for i, (dt0, dt1, _, _) in enumerate(ranges):
                if dt0 <= target_dt0 and (max_i is None or ranges[max_i][0] < dt0):
                    max_i = i
                if dt1 >= target_dt1 and (min_i is None or ranges[min_i][1] > dt1):
                    min_i = i
            # print(ranges[min_i], ranges[max_i])

            if min_i is None or max_i is None:
                print("no range")
                continue

            for i in range(min_i, max_i + 1):
                offset0 = min(max((target_dt0 - ranges[i][0]).total_seconds(), 0), ranges[i][2])
                offset1 = min(max((target_dt1 - ranges[i][0]).total_seconds(), 0), ranges[i][2])
                output = pathlib.Path(args.output) / "{}_{}_{}.mp4".format(row["id"], j, i)
                subprocess.run(["ffmpeg", "-ss", str(offset0), "-to", str(offset1), "-i", ranges[i][3], "-vf", "scale=1270:720", "-c:v", "hevc_nvenc", "-an", "-y", output])


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    subparser = parser.add_subparsers(required=True)

    parser_upload = subparser.add_parser("upload")
    parser_upload.add_argument("-i", "--input", type=str, required=True, nargs="+")
    parser_upload.add_argument("--header", action="store_true")
    parser_upload.add_argument("--utc", action="store_true")
    parser_upload.set_defaults(func=upload)

    parser_integrate = subparser.add_parser("integrate")
    parser_integrate.add_argument("-c", "--conf", type=str)
    parser_integrate.add_argument("-o", "--output", type=str)
    parser_integrate.set_defaults(func=integrate)

    args = parser.parse_args()
    if "func" in args:
        args.func(args)
    else:
        parser.print_help()
