import argparse
import collections.abc as collections
from pathlib import Path
from typing import List, Optional, Union

from hloc import logger
from hloc.utils.io import list_h5_names
from hloc.utils.parsers import parse_image_lists


def main(
    output: Path,
    image_list: Optional[Union[Path, List[str]]] = None,
    features: Optional[Path] = None,
    quadratic: bool = False,
    quadratic_t: Optional[float] = None,
    overlap: Optional[int] = None,
):
    if image_list is not None:
        if isinstance(image_list, (str, Path)):
            names_q = parse_image_lists(image_list)
        elif isinstance(image_list, collections.Iterable):
            names_q = list(image_list)
        else:
            raise ValueError(f"Unknown type for image list: {image_list}")
    elif features is not None:
        names_q = list_h5_names(features)
    else:
        raise ValueError("Provide either a list of images or a feature file.")

    names_q.sort()

    if overlap is None:
        overlap = 1

    if quadratic_t is None:
        quadratic_t = 1.0

    shifts = []
    for d in range(overlap):
        if quadratic:
            shifts.append(int(2.0 ** (d * quadratic_t)))
        else:
            shifts.append(d)
    shifts = list(set(shifts))

    pairs = []
    for i, n1 in enumerate(names_q):
        for shift in shifts:
            j = i + shift
            if j < len(names_q):
                n2 = names_q[j]
                pairs.append((n1, n2))

    logger.info(f"Found {len(pairs)} pairs.")
    with open(output, "w") as f:
        f.write("\n".join(" ".join([i, j]) for i, j in pairs))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--image_list", type=Path)
    parser.add_argument("--features", type=Path)
    parser.add_argument("--quadratic", type=bool)
    parser.add_argument("--overlap", type=int)
    args = parser.parse_args()
    main(**args.__dict__)
