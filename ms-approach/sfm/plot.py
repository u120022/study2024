import hloc
import matplotlib.pyplot as plt


def main():
    rec = hloc.pycolmap.Reconstruction("output/sfm")
    print(rec.summary())

    xs, ys = [], []
    for _, image in rec.images.items():
        translation = image.cam_from_world.translation
        xs.append(translation[0])
        ys.append(translation[2])

    fig, ax = plt.subplots()
    ax.scatter(xs, ys)
    fig.savefig("output/plot.png")


if __name__ == "__main__":
    main()
