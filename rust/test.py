import os
import sys;
sys.path.insert(0, os.path.dirname(os.path.dirname(__file__)))

import time

import matplotlib.pyplot as plt
import numpy as np
from inverse_design.brushes import notched_square_brush
from inverse_design.conditional_generator import new_latent_design, transform
from inverse_design.utils import conv2d, randn

from inverse_design_rs import generate_feasible_design

seed = 42
m = n = 30

brush = np.asarray(notched_square_brush(5, 1), dtype=np.float32)
latent = np.asarray(new_latent_design((m,n), r=seed), dtype=np.float32)
latent_t = np.asarray(transform(latent, brush, beta=5.0), dtype=np.float32)

# with open(f"latent_t_{seed}_{m}x{n}.bin", "wb") as file:
#     file.write(latent_t.tobytes());

#void_pixels, solid_pixels, void_touches, solid_touches =
generate_feasible_design(
    latent_t.shape,
    latent_t.tobytes(),
    brush.shape,
    brush.tobytes(),
    True,
)

#design = Design(
#    np.asarray(void_pixels).reshape(m, n),
#    np.asarray(solid_pixels).reshape(m, n),
#    np.asarray(void_touches).reshape(m, n),
#    np.asarray(solid_touches).reshape(m, n),
#)

#total_time = time.time() - start_time
#time_per_sim = total_time / n_iter
#print(f"{m=} {n=} {total_time=:.2f} {time_per_sim=:.2f}")
#
#visualize(design)
#plt.show()
