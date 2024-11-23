import pathlib

import hloc
import hloc.extract_features
import hloc.match_features
import hloc.reconstruction
import matplotlib.pyplot as plt

import pairs_from_sequential


def sfm():
    input_dir = pathlib.Path("output/images")
    output_dir = pathlib.Path("output")

    for path in input_dir.iterdir():

        # setup

        input = input_dir / path.stem
        output = output_dir / path.stem

        image_dir = input
        features = output / "features.h5"
        pairs = output / "pairs.txt"
        matches = output / "matches.h5"
        sfm_dir = output / "sfm"

        feature_conf = hloc.extract_features.confs["disk"]
        feature_conf["model"]["max_keypoints"] = 1024
        feature_conf["preprocessing"]["resize_max"] = 1280

        matcher_conf = hloc.match_features.confs["disk+lightglue"]
        matcher_conf["model"]["depth_confidence"] = 0.90
        matcher_conf["model"]["width_confidence"] = 0.95

        # sfm

        print("extract features")
        hloc.extract_features.main(feature_conf, image_dir, feature_path=features)

        print("create sequential pairs")
        pairs_from_sequential.main(pairs, features=features, quadratic=True, overlap=10, quadratic_t=0.6)

        print("match features")
        hloc.match_features.main(matcher_conf, pairs, features=features, matches=matches)

        print("reconstruct")
        hloc.reconstruction.main(sfm_dir, image_dir, pairs, features, matches, camera_mode=hloc.pycolmap.CameraMode.SINGLE)


def plot():
    dir = pathlib.Path("output")

    for path in dir.iterdir():
        try:
            # rec = hloc.pycolmap.Reconstruction(path / "sfm")
            rec = hloc.pycolmap.Reconstruction(path / "sfm/models/0")
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
    sfm()
    plot()
