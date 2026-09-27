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

## Next profiling targets

- Measure per-frame CPU time and GPU time.
- Add allocation/resource counters around text regeneration.
- Replace per-entity texture ownership with shared asset handles.
- Move render-object lookup from a hash map to an engine-level indexed resource table once entity IDs become stable engine handles.
- Add change tracking for transforms and GPU buffer writes.
- Investigate batching only after profiling establishes draw-call overhead.
