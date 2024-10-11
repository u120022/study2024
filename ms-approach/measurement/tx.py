import numpy as np
import pytransform3d.transformations as pt


cam2world = pt.transform_from_pq([0, 0, 0, 0.723343, 0.677349, 0.089033, 0.100232])
sensor_width = 36 * 0.001
f = 67.0176 * 0.001

sensor_size = np.array([sensor_width, sensor_width * (1080 / 1920)])
cam2img = np.array([
    [f, 0, sensor_size[0] / 2.0, 0],
    [0, f, sensor_size[1] / 2.0, 0],
    [0, 0, 1, 0]
])
img2cam = np.invert(cam2img)

p = np.array([0.0, 0.0, 0.0])
p = np.dot(cam2world, np.dot(img2cam, p))
