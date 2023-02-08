import time

import matplotlib.pyplot as plt
import numpy as np
from inverse_design_rs import generate_feasible_design, print_profiler_summary

from inverse_design.brushes import notched_square_brush
from inverse_design.conditional_generator import new_latent_design, transform
from inverse_design.design import Design, visualize
from inverse_design.utils import conv2d, randn

seed = 42
m = n = 256 + 128

start_time = time.time()
brush = np.asarray(notched_square_brush(15, 3), dtype=np.float32)
latent = np.asarray(new_latent_design((m, n), r=seed), dtype=np.float32)
latent_t = np.asarray(transform(latent, brush, beta=5.0), dtype=np.float32)
latent_t = latent_t + latent_t[::-1]

# with open(f"latent_t_{seed}_{m}x{n}.bin", "wb") as file:
#     file.write(latent_t.tobytes());

start = time.process_time()
void, void_touch_existing, solid_touch_existing = generate_feasible_design(
    latent_t.shape,
    latent_t.tobytes(),
    brush.shape,
    brush.tobytes(),
    False,
)
print(f"took: {time.time()-start}s")
void = np.asarray(void).reshape(m, n)
void_touch_existing = np.asarray(void_touch_existing).reshape(m, n)
solid_touch_existing = np.asarray(solid_touch_existing).reshape(m, n)

void_pixels = np.asarray(np.where(void, 4, 3), dtype=np.uint8)
solid_pixels = np.asarray(np.where(void, 3, 4), dtype=np.uint8)
void_touches = np.asarray(np.where(void_touch_existing, 9, 8), dtype=np.uint8)
solid_touches = np.asarray(np.where(solid_touch_existing, 9, 8), dtype=np.uint8)

design = Design(
    np.asarray(void_pixels).reshape(m, n),
    np.asarray(solid_pixels).reshape(m, n),
    np.asarray(void_touches).reshape(m, n),
    np.asarray(solid_touches).reshape(m, n),
)
print(f"execution time: {time.time()-start_time:.3f}")

print_profiler_summary()

#visualize(design, grid=False)
#plt.show()
