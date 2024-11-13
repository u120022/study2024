import pathlib

import hloc.extract_features
import hloc.match_features
import hloc.reconstruction

import pairs_from_sequential


def main():

    # setup

    input = pathlib.Path("input")
    output = pathlib.Path("output")

    image_dir = input
    features = output / "features.h5"
    pairs = output / "pairs.txt"
    matches = output / "matches.h5"
    sfm_dir = output / "sfm"

    feature_conf = hloc.extract_features.confs["disk"]
    matcher_conf = hloc.match_features.confs["disk+lightglue"]

    # sfm

    print("extract features")
    hloc.extract_features.main(feature_conf, image_dir, feature_path=features)

    print("create sequential pairs")
    pairs_from_sequential.main(pairs, features=features, quadratic=True, overlap=8)

    print("match features")
    hloc.match_features.main(matcher_conf, pairs, features=features, matches=matches)

    print("reconstruct")
    hloc.reconstruction.main(sfm_dir, image_dir, pairs, features, matches, camera_mode=hloc.pycolmap.CameraMode.SINGLE)


if __name__ == "__main__":
    main()
