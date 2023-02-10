import time

import matplotlib.pyplot as plt
import numpy as np
from inverse_design_rs import print_profiler_summary

from inverse_design.brushes import notched_square_brush
from inverse_design.conditional_generator import new_latent_design, transform
from inverse_design.design import Design, visualize
from inverse_design.utils import conv2d, randn
from inverse_design.conditional_generator import generate_feasible_design

seed = 42
m = n = 256 + 128

start_time = time.time()
brush = np.asarray(notched_square_brush(15, 3), dtype=np.float32)
latent = np.asarray(new_latent_design((m, n), r=seed), dtype=np.float32)
latent_t = np.asarray(transform(latent, brush, beta=5.0), dtype=np.float32)
#latent_t = latent_t + latent_t[::-1]

# with open(f"latent_t_{seed}_{m}x{n}.bin", "wb") as file:
#     file.write(latent_t.tobytes());

start = time.process_time()
design = generate_feasible_design(
    latent_t,
    brush,
    symmetry="transpose",
    verbose=False,
)

print(f"execution time: {time.time()-start_time:.3f}")

print_profiler_summary()

visualize(design, grid=False)
plt.show()
