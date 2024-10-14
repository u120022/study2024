import os
import subprocess
import concurrent.futures

import pandas as pd


# input data directory
INPUT_DIR = "data"
# filter for zip files
FILTER = "tokyo"

env = os.environ.copy()
env["PGHOST"] = "localhost"
env["PGUSER"] = "postgis"
env["PGPASSWORD"] = "0"

# create traffics table
subprocess.call([
    "psql",
    "-c",
    "DROP TABLE IF EXISTS traffics;",
], env=env)
subprocess.call([
    "psql",
    "-c",
    "CREATE TABLE IF NOT EXISTS traffics (datetime Timestamp, src_code Text, loc_code Int, link_type Int, link_code Int, traffics Int);",
], env=env)

for f0 in os.listdir(INPUT_DIR):
    # create working directories
    os.makedirs("layer1", exist_ok=True)
    os.makedirs("layer2", exist_ok=True)
    os.makedirs("layer3", exist_ok=True)

    # extract top-level zip files
    subprocess.call([
        "sh",
        "-c",
        f"zipinfo -1 {INPUT_DIR}/{f0} | grep {FILTER} | parallel \"unzip -j -d layer1 {INPUT_DIR}/{f0} {{}}\"",
    ])

    # extract next-level zip files
    subprocess.call([
        "sh",
        "-c",
        "ls layer1 | parallel \"unzip -j -d layer2 -O sjis layer1/{}\"",
    ])

    # parallelize csv transform
    with concurrent.futures.ProcessPoolExecutor(max_workers=10) as executor:
        def data_transform(f1):
            df = pd.read_csv(f"layer2/{f1}", encoding="sjis")
            df = df[["時刻", "情報源コード", "計測地点番号", "リンク区分", "リンク番号", "断面交通量"]]
            df = df.rename(columns={
                "時刻": "datetime",
                "情報源コード": "src_code",
                "計測地点番号": "loc_code",
                "リンク区分": "link_type",
                "リンク番号": "link_code",
                "断面交通量": "traffics",
            })
            df.to_csv(f"layer3/{f1}", index=False)
            print(f1)

        executor.map(data_transform, os.listdir("layer2"))

    # write output to postgis
    for f1 in os.listdir("layer3"):
        subprocess.call([
            "psql",
            "-c",
            f"\\COPY traffics FROM layer3/{f1} WITH CSV HEADER",
        ], env=env)

    # cleanup working directories
    for f1 in os.listdir("layer1"):
        os.remove(f"layer1/{f1}")
    for f1 in os.listdir("layer2"):
        os.remove(f"layer2/{f1}")
    for f1 in os.listdir("layer3"):
        os.remove(f"layer3/{f1}")
    os.rmdir("layer1")
    os.rmdir("layer2")
    os.rmdir("layer3")
