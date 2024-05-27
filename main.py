import pandas
import os
import subprocess
import concurrent.futures

os.makedirs("./layer1", exist_ok=True)
os.makedirs("./layer2", exist_ok=True)
os.makedirs("./layer3", exist_ok=True)

env = os.environ.copy()
env["PGPASSWORD"] = "0"

subprocess.call(["psql", "-U", "postgis", "-h", "localhost", "-c", "drop table if exists traffic;"], env=env)
subprocess.call(["psql", "-U", "postgis", "-h", "localhost", "-c", "create table if not exists traffic (time timestamp, area text, point int, traffic int);"], env=env)

def csv_convert(f1):
    csv = pandas.read_csv(f"./layer2/{f1}", encoding="sjis")
    csv = csv[["時刻", "情報源コード", "計測地点番号", "断面交通量"]]
    csv = csv.rename(columns={ "時刻": "time", "情報源コード": "area", "計測地点番号": "point", "断面交通量": "traffic" })
    csv.to_csv(f"./layer3/{f1}", index=False)
    print(f1)

for f0 in os.listdir("./data"):
    if not "2023" in f0:
        continue

    for f1 in os.listdir("./layer1"):
        os.remove(f"./layer1/{f1}")
    for f1 in os.listdir("./layer2"):
        os.remove(f"./layer2/{f1}")
    for f1 in os.listdir("./layer3"):
        os.remove(f"./layer3/{f1}")

    subprocess.call(["sh", "-c", f"zipinfo -1 ./data/{f0} | grep tokyo | parallel \"unzip -j -d ./layer1 ./data/{f0} {{}}\""])

    subprocess.call(["sh", "-c", "ls ./layer1 | parallel \"unzip -j -d ./layer2 -O sjis ./layer1/{}\""])

    with concurrent.futures.ProcessPoolExecutor(max_workers=10) as executor:
        executor.map(csv_convert, os.listdir("./layer2"))

    for f1 in os.listdir("./layer3"):
        subprocess.call(["psql", "-U", "postgis", "-h", "localhost", "-c", f"\\copy traffic from ./layer3/{f1} with csv header"], env=env)
