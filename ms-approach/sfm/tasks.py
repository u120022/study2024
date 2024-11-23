import argparse
import glob
import os
import pathlib
import subprocess

import hloc
import hloc.extract_features
import hloc.match_features
import hloc.reconstruction
import matplotlib.pyplot as plt

import pairs_from_sequential


def preprocess(args):
    for path in glob.glob(args.input):
        input_path = pathlib.Path(path)
        output_dir = pathlib.Path("{}.sfm/images".format(path))

        os.makedirs(output_dir, exist_ok=True)

        output_path = pathlib.Path("{}/%04d.png".format(output_dir))
        subprocess.Popen(["ffmpeg", "-i", input_path, "-vf", "scale=1280:720", "-c:v", "png", "-r", "30", output_path]).wait()


def sfm(args):
    for path in glob.glob(args.input):
        image_dir = pathlib.Path("{}.sfm/images".format(path))
        features_path = pathlib.Path("{}.sfm/features.h5".format(path))
        pairs_path = pathlib.Path("{}.sfm/pairs.txt".format(path))
        matches_path = pathlib.Path("{}.sfm/matches.h5".format(path))
        sfm_dir = pathlib.Path("{}.sfm/sfm".format(path))

        feature_conf = hloc.extract_features.confs["disk"]
        feature_conf["model"]["max_keypoints"] = 1024
        feature_conf["preprocessing"]["resize_max"] = 1280

        matcher_conf = hloc.match_features.confs["disk+lightglue"]
        matcher_conf["model"]["depth_confidence"] = 0.90
        matcher_conf["model"]["width_confidence"] = 0.95

        print("extract features")
        hloc.extract_features.main(feature_conf, image_dir, feature_path=features_path)

        print("create sequential pairs")
        pairs_from_sequential.main(pairs_path, features=features_path, quadratic=True, overlap=10, quadratic_t=0.6)

        print("match features")
        hloc.match_features.main(matcher_conf, pairs_path, features=features_path, matches=matches_path)

        print("reconstruct")
        hloc.reconstruction.main(sfm_dir, image_dir, pairs_path, features_path, matches_path, camera_mode=hloc.pycolmap.CameraMode.SINGLE)


def plot(args):
    for path in glob.glob("{}.sfm/sfm/models/*".format(args.input)):
        try:
            model_path = pathlib.Path(path)
            image_path = pathlib.Path("{}.png".format(path))

            model = hloc.pycolmap.Reconstruction(model_path)
            print(model.summary())

            xs, ys = [], []
            for _, image in model.images.items():
                translation = image.cam_from_world.translation
                xs.append(translation[0])
                ys.append(translation[2])

            fig, ax = plt.subplots()
            ax.scatter(xs, ys)
            fig.savefig(image_path)
            print(image_path)

        except Exception as e:
            print(e)


def pipeline(args):
    preprocess(args)
    sfm(args)
    plot(args)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    subparser = parser.add_subparsers(required=True)

    parser_preprocess = subparser.add_parser("preprocess")
    parser_preprocess.add_argument("-i" "--input", type=str, required=True)
    parser_preprocess.set_defaults(func=preprocess)

    parser_sfm = subparser.add_parser("sfm")
    parser_sfm.add_argument("-i", "--input", type=str, required=True)
    parser_sfm.set_defaults(func=sfm)

    parser_plot = subparser.add_parser("plot")
    parser_plot.add_argument("-i", "--input", type=str, required=True)
    parser_plot.set_defaults(func=plot)

    parser_pipeline = subparser.add_parser("pipeline")
    parser_pipeline.add_argument("-i", "--input", type=str, required=True)
    parser_pipeline.set_defaults(func=pipeline)

    args = parser.parse_args()
    if "func" in args:
        args.func(args)
    else:
        parser.print_help()
