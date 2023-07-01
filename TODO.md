# TODO

## Short-Term

- Better rendering back end to be able to add theta max to debug meshes
  - Switch to Bevy from kiss3d
  - Add code to surfaces for generating mesh
- Triangles
- Meshes
  - Importing
  - Sampling/colliding
  - Testing.  Can compare to view factors of discretized primitive surfaces
- Handling sided-ness
  - Active/inactive side for surface (how to map hits?)
  - Primitives need to track sides for each surface?
  - Fix leaking boxes / rectangles if x-axis points wrong way?
- More surface parameterization
  - Add theta max to disk, cylinder, and cone
  - Add z_max to cone
- Some way of mapping primitives to their surfaces
- Binning strategies for surfaces
- Rayon for view factors?
- Rework trace record -> probably pretty slow due to allocations

## Long-Term

- Run on the GPU
  - Directly using vulkan via ash?
  - Try and use rust-gpu? Just glsl might be easier honestly
  - Experiment with normal ray-tracing?
  - Get working with just spheres -> might be worth making a new repo for testing
- Celestial bodies
  - Sampling a skybox texture?
  - Modeling them as spheres?
  - Atmospheric effects?
  - Eclipses
- Materials
  - Bidirectional Reflective
  - Transmissive
- Actuators?
- Convert from a TRASYS file
