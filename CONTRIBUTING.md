# Engineering contributions

Changes must state the engineering effect, affected subsystem and validation.
Use concise technical English in commit messages, comments and documentation.
Format Rust with rustfmt, Go with gofmt and Erlang with erlfmt. Keep dependencies
local or pinned. Run the tests for modified areas before committing.

## Ownership

| Engineer | Primary area |
| --- | --- |
| Mara Stein | Propulsion, pumps, valves and pressure interlocks |
| Elena Petrova | Guidance, navigation and shaping |
| Daniel Brooks | Battery, circuits and power allocation |
| Tariq Saad | Communications transport and supervision |
| Priya Nair | Wire contracts and cross-language compatibility |
| Lucas Meyer | Go command service, sessions and backend APIs |
| Sofia Alvarez | HTMX fragments, CSS and console accessibility |
| William Novak | Telemetry, events and health monitoring |
| Hannah Reed | Builds, release metadata and integration coordination |
| Mateo Costa | Thermal loops, fault management and watchdogs |

Adjacent changes must remain small and support the same engineering feature.
Frontend work may adjust its Go handlers or DTOs but does not change communications.
Substantial cross-area changes are authored or reviewed by the domain owner.
Release metadata updates are mechanical release-engineering work. Integrating a
change preserves its author's identity; the committer records who integrated it.

## Review checklist

1. Validate finite numeric values, units, capacities and time ordering.
2. Preserve bounded queues and explicit state transitions.
3. Cover the nominal path and rejected boundary cases.
4. Update wire vectors and protocol documentation when layouts change.
5. Record operating consequences for changed interlocks or recovery behavior.
6. Keep refactors separate from unrelated feature changes.
