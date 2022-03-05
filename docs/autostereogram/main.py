import numpy as np
from PIL import Image

def make_sphere_depthmap(xlim, ylim, *, rad=1., size=200):
    return np.array([
        [
            (rad**2 - x**2 - y**2)**(1./2) if x**2 + y**2 < rad**2 else 0
            for x in np.linspace(xlim[0], xlim[1], size)
        ]
        for y in np.linspace(ylim[0], ylim[1], size)
    ])

def make_random_dot_stereogram(depthmap, pattern_size, *, shift_coef=0.3):
    repeated = np.random.randint(0, 256, size=pattern_size).astype(np.uint8)
    rds = np.zeros(depthmap.shape, dtype=np.uint8)
    for x in range(rds.shape[1]):
        for y in range(rds.shape[0]):
            if x < repeated.shape[1]:
                rds[y, x] = repeated[y % repeated.shape[0], x]
            else:
                shift = int(depthmap[y, x] * shift_coef * repeated.shape[1])
                rds[y, x] = rds[y, x - repeated.shape[1] + shift]
    return rds

if __name__ == '__main__':
    depthmap = make_sphere_depthmap((-2, 2), (-1.5, 1.5))
    Image.fromarray((depthmap*256).astype(np.uint8)).save('depthmap.png')
    Image.fromarray(make_random_dot_stereogram(depthmap, (50, 50))).save('rds.png')

