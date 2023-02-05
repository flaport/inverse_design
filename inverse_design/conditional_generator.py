# AUTOGENERATED! DO NOT EDIT! File to edit: notebooks/04_conditional_generator.ipynb (unless otherwise specified).

__all__ = ['new_latent_design', 'transform', 'conditional_algirithm_step', 'conditional_generator',
           'generate_feasible_design', 'generate_feasible_design_mask_', 'generate_feasible_design_mask',
           'generate_feasible_design_mask_jvp']

# Cell
import warnings

import jax
import jax.numpy as jnp
import numpy as np
from .design import (
    TOUCH_FREE,
    TOUCH_RESOLVING,
    TOUCH_VALID,
    UNASSIGNED,
    Design,
    add_solid_touch,
    add_void_touch,
    design_mask,
    new_design,
    take_free_solid_touches,
    take_free_void_touches,
)
from .utils import argmax2d, argmin2d, conv2d, randn

# Cell
def new_latent_design(shape, bias=0, r=None):
    arr = randn(shape, r=r)
    arr += bias
    return jnp.asarray(arr, dtype=float)

# Cell
@jax.jit
def transform(latent, brush, beta=5.0):
    convolved = conv2d(latent, jnp.asarray(brush, dtype=float)) / brush.sum()
    return jnp.tanh(beta * convolved)

# Cell
def conditional_algirithm_step(latent_t, design, brush, verbose=False):
    maybe_print = print if verbose else (lambda *args, **kwargs: None)
    void_touch_mask = design.void_touches == TOUCH_VALID
    solid_touch_mask = design.solid_touches == TOUCH_VALID
    touch_mask = void_touch_mask | solid_touch_mask

    void_free_mask = design.void_touches == TOUCH_FREE
    solid_free_mask = design.solid_touches == TOUCH_FREE
    free_mask = void_free_mask | solid_free_mask

    void_resolving_mask = design.void_touches == TOUCH_RESOLVING
    solid_resolving_mask = design.solid_touches == TOUCH_RESOLVING
    resolving_mask = void_resolving_mask | solid_resolving_mask

    if free_mask.any():
        void_selector = jnp.where(void_free_mask, latent_t, 0)
        solid_selector = jnp.where(solid_free_mask, latent_t, 0)
        if abs(void_selector.sum()) > abs(solid_selector.sum()):
            design = take_free_void_touches(design, brush)
            maybe_print(f"take free void.")
        else:
            design = take_free_solid_touches(design, brush)
            maybe_print(f"take free solid.")

    elif resolving_mask.any():
        void_needs_resolving = void_resolving_mask.any()
        solid_needs_resolving = solid_resolving_mask.any()
        void_selector = jnp.where(void_resolving_mask, latent_t, np.inf)
        solid_selector = jnp.where(solid_resolving_mask, latent_t, -np.inf)

        if void_needs_resolving and (not solid_needs_resolving):
            i_v, j_v = argmin2d(void_selector)
            design = add_void_touch(design, brush, (i_v, j_v))
            maybe_print(f"resolve void {int(i_v), int(j_v)}.")
        elif (not void_needs_resolving) and solid_needs_resolving:
            i_s, j_s = argmax2d(solid_selector)
            design = add_solid_touch(design, brush, (i_s, j_s))
            maybe_print(f"resolve solid {int(i_s), int(j_s)}.")
        else:  # both need resolving. TODO: figure out if we actually need this case...
            i_v, j_v = argmin2d(void_selector)
            v = latent_t[i_v, j_v]
            i_s, j_s = argmax2d(solid_selector)
            s = latent_t[i_s, j_s]
            if abs(v) > abs(s):
                design = add_void_touch(design, brush, (i_v, j_v))
                maybe_print(f"resolve void {int(i_v), int(j_v)}.")
            else:
                design = add_solid_touch(design, brush, (i_s, j_s))
                maybe_print(f"resolve solid {int(i_s), int(j_s)}.")

    elif touch_mask.any():
        void_selector = jnp.where(void_touch_mask, latent_t, np.inf)
        solid_selector = jnp.where(solid_touch_mask, latent_t, -np.inf)
        i_v, j_v = argmin2d(void_selector)
        v = latent_t[i_v, j_v]
        i_s, j_s = argmax2d(solid_selector)
        s = latent_t[i_s, j_s]
        if abs(v) > abs(s):
            design = add_void_touch(design, brush, (i_v, j_v))
            maybe_print(f"touch void {int(i_v), int(j_v)}.")
        else:
            design = add_solid_touch(design, brush, (i_s, j_s))
            maybe_print(f"touch solid {int(i_s), int(j_s)}.")
    else:
        raise ValueError("This should never happen.")

    return design

# Cell
def conditional_generator(latent_t, brush, verbose=False):
    I = 0
    design = new_design(latent_t.shape)
    maybe_print = print if verbose else (lambda *args, **kwargs: None)
    maybe_print(f"create empty design.")
    yield design
    while (design.design == UNASSIGNED).any():
        maybe_print(f"iteration {I}")
        I += 1
        design = conditional_algirithm_step(latent_t, design, brush, verbose=verbose)
        yield design

# Cell

def generate_feasible_design(latent_t, brush, verbose=False, backend='auto'): # backend: 'auto', 'rust', 'python'
    try:
        from inverse_design_rs import generate_feasible_design as generate_feasible_design_rs
        if backend == 'auto':
            backend = 'rust'
    except ImportError:
        warnings.warn("falling back on slower python-based feasible design generation!")
        if backend == 'auto':
            backend = 'python'

    if backend == 'rust':
        return _generate_feasible_design_rust(latent_t, brush, verbose=verbose)
    else:
        return _generate_feasible_design_python(latent_t, brush, verbose=verbose)


def _generate_feasible_design_python(latent_t, brush, verbose=False):
    design = None
    for design in conditional_generator(latent_t, brush, verbose=verbose):
        continue
    return design

def _generate_feasible_design_rust(latent_t, brush, verbose=False):
    from inverse_design_rs import generate_feasible_design as generate_feasible_design_rs

    m, n = latent_t.shape
    latent_t = np.asarray(latent_t, dtype=np.float32)
    brush = np.asarray(brush, dtype=np.float32)
    void_pixels, solid_pixels, void_touches, solid_touches = generate_feasible_design_rs(
        latent_t.shape,
        latent_t.tobytes(),
        brush.shape,
        brush.tobytes(),
        verbose
    )
    design = Design(
        np.asarray(void_pixels).reshape(m, n),
        np.asarray(solid_pixels).reshape(m, n),
        np.asarray(void_touches).reshape(m, n),
        np.asarray(solid_touches).reshape(m, n),
    )
    return design


# Cell
def generate_feasible_design_mask_(latent_t, brush, backend='auto'):
    design = generate_feasible_design(latent_t, brush, verbose=False, backend=backend)
    return design_mask(design)

# Cell
@jax.custom_jvp
def generate_feasible_design_mask(latent_t, brush):
    design = generate_feasible_design(latent_t, brush, verbose=False)
    return design_mask(design)

# Cell
@generate_feasible_design_mask.defjvp
def generate_feasible_design_mask_jvp(primals, tangents):
    return primals[0], tangents[0]  # identity function for first argument: latent_t