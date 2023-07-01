# ThermalSolver

A monte carlo ray tracer for calculating thermal radiation view factors between different pieces of geometry.

## Current capabilities

- Uniformly sample and find ray intersections with surfaces of spheres, rectangles, disks, cylinders, and cones
- Debug renderer that shows which rays hit an object and which rays miss
- Primitive shape abstraction for generating cuboids, cylinders with tops/bottoms, and cones with bases
- Compute view factors and gray body distribution factors (for diffuse and specular surfaces) from a surface in the scene to others
