import math
import os
import pathlib
import multiprocessing
import time

import cv2
import hloc.extract_features
import hloc.match_features
import hloc.reconstruction
import tqdm

import pairs_from_sequential


def video():
    input = pathlib.Path("input")
    output = pathlib.Path("output")
    reference_fps = 8.0
    max_task_size = 32

    with multiprocessing.Pool() as pool:
        root_image_dir = output / "images"

        for path in input.iterdir():
            print("input video: {}".format(path))
            image_dir = root_image_dir / path.stem
            os.makedirs(image_dir, exist_ok=True)

            cap = cv2.VideoCapture(str(path))
            max_frame_count = int(cap.get(cv2.CAP_PROP_FRAME_COUNT))
            digit_count = len(str(max_frame_count))
            print("  frame count: {}".format(max_frame_count))

            if not cap.isOpened():
                return

            fps = int(cap.get(cv2.CAP_PROP_FPS))
            step = math.ceil(fps / reference_fps)
            for i in tqdm.tqdm(range(0, max_frame_count, step)):
                ret, frame = cap.read()

                if not ret:
                    raise Exception("frame read error")

                frame_sign = str(i).zfill(digit_count)
                image_path = image_dir / "{}.png".format(frame_sign)
                pool.apply_async(cv2.imwrite, (str(image_path), frame, [cv2.IMWRITE_PNG_COMPRESSION, 9]))

                while pool._taskqueue.qsize() > max_task_size:
                    time.sleep(1)

        print("waiting for tasks to finish")

    print("done tasks")


def main():
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
    # video()
    main()
