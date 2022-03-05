# copied from http://matplotlib.org/mpl_examples/mplot3d/surface3d_demo.py

from mpl_toolkits.mplot3d import Axes3D  # NOQA
from matplotlib import cm
from matplotlib.ticker import LinearLocator, FormatStrFormatter
import matplotlib.pyplot as plt
import numpy as np
import pickle

with open('error_cache', 'rb') as f:
    X, Y, Z = pickle.load(f)

fig = plt.figure()
ax = fig.gca(projection='3d')

X, Y = np.meshgrid(X, Y)
surf = ax.plot_surface(X, Y, Z, rstride=1, cstride=1, cmap=cm.coolwarm,
                       linewidth=0, antialiased=False)

ax.zaxis.set_major_locator(LinearLocator(10))
ax.zaxis.set_major_formatter(FormatStrFormatter('%.02f'))

fig.colorbar(surf, shrink=0.5, aspect=5)

plt.show()
