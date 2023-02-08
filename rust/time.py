import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(__file__)))

import time
import numpy as np
import matplotlib.pyplot as plt
from tqdm import tqdm
from inverse_design.conditional_generator import generate_feasible_design, new_latent_design, transform
from inverse_design.brushes import notched_square_brush

seed = 42

durations = {}
brushes = [(5, 1), (10, 2), (15, 3)]
ms = np.arange(20, 300+1, 20)

for brush_size in brushes:
    brush = np.asarray(notched_square_brush(*brush_size), dtype=np.float32)
    pixels = ms**2
    durations[brush_size] = []
    for m in (pb:=tqdm(ms)):
        pb.set_postfix(m=m, pixels=m**2, brush_size=brush_size)
        latent = np.asarray(new_latent_design((m, m), r=seed), dtype=np.float32)
        latent_t = np.asarray(transform(latent, brush, beta=5.0), dtype=np.float32)

        start_time = time.time()
        design = generate_feasible_design(latent_t, brush, verbose=False)
        durations[brush_size].append(time.time() - start_time)

    durations[brush_size] = np.array(durations[brush_size], dtype=np.float32)*1000

for brush_size, durations in durations.items():
    plt.plot(pixels, durations, label=f"notched square {brush_size}")

plt.grid(True)
plt.xlabel("pixels")
plt.ylabel("time [ms]")
plt.legend()
plt.show()
