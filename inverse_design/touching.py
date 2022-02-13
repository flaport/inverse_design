# AUTOGENERATED! DO NOT EDIT! File to edit: notebooks/04_touching.ipynb (unless otherwise specified).

__all__ = ['add_void_touch', 'take_free_void_touches', 'add_solid_touch', 'take_free_solid_touches']

# Internal Cell

import jax
import jax.numpy as jnp
from .brushes import notched_square_brush, show_mask
from .design import (
    PIXEL_EXISTING,
    PIXEL_IMPOSSIBLE,
    PIXEL_POSSIBLE,
    PIXEL_REQUIRED,
    SOLID,
    TOUCH_EXISTING,
    TOUCH_FREE,
    TOUCH_INVALID,
    TOUCH_REQUIRED,
    TOUCH_RESOLVING,
    TOUCH_VALID,
    VOID,
    Design,
    new_design,
)
from .utils import batch_conv2d, dilute, or_, and_, not_, float_mask, where_

# Cell

@jax.jit
def _find_free_touches(touches_mask, pixels_mask, brush):
    r = jnp.zeros_like(touches_mask, dtype=float)
    m, n = r.shape
    i, j = jnp.arange(m), jnp.arange(n)
    I, J = [idxs.ravel() for idxs in jnp.meshgrid(i, j)]
    K = jnp.arange(m * n)
    R = jnp.broadcast_to(r[None, :, :], (m * n, m, n)).at[K, I, J].set(1.0)
    Rb = or_(batch_conv2d(R, brush[None]), pixels_mask)
    free_idxs = (Rb - pixels_mask < 1e-10).all((1, 2))
    free_touches_mask = where_(free_idxs[:, None, None], R, 0.0).sum(0)
    return and_(free_touches_mask, not_(touches_mask))

# Cell
@jax.jit
def _find_required_pixels(pixel_map, brush):
    mask = and_(not_(pixel_map), not_(dilute(pixel_map, brush)))
    return not_(or_(dilute(mask, brush), pixel_map))

# Cell
@jax.jit
def add_void_touch(design, brush, pos):
    if isinstance(pos, tuple):
        void_touch_existing = design.void_touch_existing.at[pos[0], pos[1]].set(1.0)
    else:
        void_touch_existing = or_(pos,  design.void_touch_existing)
    void = or_(dilute(void_touch_existing, brush),  design.void)
    solid_touch_invalid = dilute(void, brush)
    void_pixel_required = _find_required_pixels(void, brush)
    void_touch_free = _find_free_touches(void_touch_existing, or_(void, void_pixel_required), brush)
    void_touch_valid = and_(design.void_touch_valid, not_(design.void_touch_invalid))
    void_touch_valid = or_(void_touch_valid, design.void_touch_resolving)
    void_touch_resolving = and_(dilute(void_pixel_required, brush), void_touch_valid)
    void_touch_resolving = and_(void_touch_resolving, not_(void_touch_free))
    void_pixel_existing = or_(void, design.void_pixel_existing)
    solid_pixel_impossible = or_(or_(design.solid_pixel_impossible, void), void_pixel_required)
    void_pixel_possible = and_(design.void_pixel_possible, not_(or_(void_pixel_existing, design.void_pixel_impossible)))
    return design.copy(
        void=void,
        solid_pixel_impossible=solid_pixel_impossible,
        void_pixel_existing=void_pixel_existing,
        void_pixel_possible=void_pixel_possible,
        void_pixel_impossible=and_(design.void_pixel_impossible, not_(void_pixel_possible)),
        void_pixel_required=void_pixel_required,
        solid_touch_invalid=solid_touch_invalid,
        solid_touch_valid=and_(design.solid_touch_valid, not_(solid_touch_invalid)),
        void_touch_existing=void_touch_existing,
        void_touch_valid=and_(void_touch_valid, not_(void_touch_existing)),
        void_touch_free=void_touch_free,
        void_touch_resolving=void_touch_resolving,
    )

# Cell
@jax.jit
def take_free_void_touches(design, brush):
    return add_void_touch(design, brush, design.void_touch_free)

# Cell
@jax.jit
def add_solid_touch(design, brush, pos):
    if isinstance(pos, tuple):
        solid_touch_existing = design.solid_touch_existing.at[pos[0], pos[1]].set(1.0)
    else:
        solid_touch_existing = or_(pos,  design.solid_touch_existing)
    solid = or_(dilute(solid_touch_existing, brush),  design.solid)
    void_touch_invalid = dilute(solid, brush)
    solid_pixel_required = _find_required_pixels(solid, brush)
    solid_touch_free = _find_free_touches(solid_touch_existing, or_(solid, solid_pixel_required), brush)
    solid_touch_valid = and_(design.solid_touch_valid, not_(design.solid_touch_invalid))
    solid_touch_valid = or_(solid_touch_valid, design.solid_touch_resolving)
    solid_touch_resolving = and_(dilute(solid_pixel_required, brush), solid_touch_valid)
    solid_touch_resolving = and_(solid_touch_resolving, not_(solid_touch_free))
    solid_pixel_existing = or_(solid, design.solid_pixel_existing)
    void_pixel_impossible = or_(or_(design.void_pixel_impossible, solid), solid_pixel_required)
    solid_pixel_possible = and_(design.solid_pixel_possible, not_(or_(solid_pixel_existing, design.solid_pixel_impossible)))
    return design.copy(
        solid=solid,
        void_pixel_impossible=void_pixel_impossible,
        solid_pixel_existing=solid_pixel_existing,
        solid_pixel_possible=solid_pixel_possible,
        solid_pixel_impossible=and_(design.solid_pixel_impossible, not_(solid_pixel_possible)),
        solid_pixel_required=solid_pixel_required,
        void_touch_invalid=void_touch_invalid,
        void_touch_valid=and_(design.void_touch_valid, not_(void_touch_invalid)),
        solid_touch_existing=solid_touch_existing,
        solid_touch_valid=and_(solid_touch_valid, not_(solid_touch_existing)),
        solid_touch_free=solid_touch_free,
        solid_touch_resolving=solid_touch_resolving,
    )

# Cell
@jax.jit
def take_free_solid_touches(design, brush):
    return add_solid_touch(design, brush, design.solid_touch_free)