import pathlib

import hloc
import matplotlib.pyplot as plt


def main():
    dir = pathlib.Path("output")

    for path in dir.iterdir():
        try:
            rec = hloc.pycolmap.Reconstruction(path / "sfm")
            print(rec.summary())

            xs, ys = [], []
            for _, image in rec.images.items():
                translation = image.cam_from_world.translation
                xs.append(translation[0])
                ys.append(translation[2])

            fig, ax = plt.subplots()
            ax.scatter(xs, ys)
            fig.savefig(path / "plot.png")
        except Exception as e:
            print(e)


if __name__ == "__main__":
    main()
