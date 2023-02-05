import sys

sys.path.insert(0, "..")
import numpy as np
import matplotlib.pyplot as plt
from inverse_design_rs import generate_feasible_design
from inverse_design.design import Design, visualize
from inverse_design.brushes import notched_square_brush
from inverse_design.conditional_generator import transform, new_latent_design

seed = 42
m, n = 30, 30
latent = np.asarray(new_latent_design((m, n), r=seed), dtype=np.float32)
brush = np.asarray(notched_square_brush(5, 1), dtype=np.float32)
void_pixels, solid_pixels, void_touches, solid_touches = generate_feasible_design(
    latent.shape,
    latent.tobytes(),
    brush.shape,
    brush.tobytes(),
)
design = Design(
    np.asarray(void_pixels).reshape(m, n),
    np.asarray(solid_pixels).reshape(m, n),
    np.asarray(void_touches).reshape(m, n),
    np.asarray(solid_touches).reshape(m, n),
)
visualize(design)
plt.show()
