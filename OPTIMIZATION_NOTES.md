# Optimization Notes

The optimization branch preserves the completed Pong behavior while improving engine-side resource and update paths.

## Completed

- Shared font initialization with `OnceLock`.
- Shared text sampler.
- Shared camera uniform buffer.
- Deferred Windows surface reconfiguration during live resize/fullscreen transitions.
- Protected gameplay delta time with a 100 ms simulation cap.
- Preallocated Pong entity storage.
- O(1) entity lookup for the current contiguous ID-based entity storage.
- Cached glyph metrics for repeated text layout width calculations.
- Transform revision tracking to avoid redundant per-frame entity uniform uploads.
- Indexed renderer storage for the current stable entity IDs.
- Shared GPU texture ownership so sprites can reuse the same loaded texture resource.

## Next profiling targets

- Measure per-frame CPU time and GPU time.
- Add allocation/resource counters around text regeneration.
- Replace the current path-based texture loading with a centralized asset manager.
- Add resource lifetime/handle management once entity IDs evolve into stable engine handles.
- Reduce text GPU resource replacement during dynamic text changes.
- Investigate batching only after profiling establishes draw-call overhead.

## Validation

Optimization changes should preserve the existing Pong gameplay and should be validated with a release build and runtime test before merging into the stable branch.
