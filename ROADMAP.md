# Hollow Orbit — Development Roadmap

## 1. Core Concept Recap

### Premise

* You are a lone traveler maintaining fragile orbital habitats after the collapse of a larger system.
* Gravity and decay are constant threats.
* Each orbit decays — stations drift, resources fade, communication weakens.
* You are holding a dying orbit together, knowing it won’t last forever.

### Tone

* Melancholic sci-fi, quiet tension, heavy atmosphere.
* Minimal dialogue.
* Every action — repairing, launching, surviving — feels deliberate.

### Gameplay Loop

* Move between derelict satellites and small habitats.
* Gather power, repair systems, and stabilize orbits.
* Survive as long as possible while managing decay.
* Transition to new orbits when current orbit collapses.

---

## 2. Engine and Tech Stack

| Area      | Choice                                          | Notes                                                |
| --------- | ----------------------------------------------- | ---------------------------------------------------- |
| Language  | Rust                                            | High performance, memory safe, ideal for simulations |
| Engine    | [Bevy](https://bevyengine.org)                  | ECS-based, modular, open-source                      |
| Physics   | [Rapier](https://rapier.rs)                     | 2D collision and rigid-body simulation               |
| Audio     | [Kira](https://github.com/tesselode/kira)       | Dynamic layering, ambient sound                      |
| Assets    | 2D pixel-based                                  | Low-res, minimalist style, parallax for scale        |
| Scripting | None initially                                  | Option to add Lua or Rhai later                      |
| UI        | Bevy UI / [egui](https://github.com/emilk/egui) | Bevy for gameplay HUD, egui for debug tools          |

---

## 3. Architecture Overview

### ECS Philosophy

* Modular and data-driven.
* Components hold state; systems define behavior.

### Core Systems

* Orbit System: manages gravity wells, orbital decay, and station drift.
* Power System: simulates energy generation, distribution, and decay.
* Repair System: handles player interactions for restoring systems.
* Communication System: delivers fragmented logs and fading signals.
* Entity Lifecycle: all entities decay or fail over time.

---

## 4. Development Phases

### Phase 1 — Prototype (v0.1.0)

**Goal:** Core movement and basic physics

**Features**

* Asset folder embedded in binary using `rust-embed` crate.
* Implement player movement in microgravity (thrust, drift, inertia).
* Create a simple orbital station with collision.
* Display an energy meter and basic HUD.
* Add debug visuals (thruster vectors, gravity wells).

**Tech Goals**

* Learn Bevy ECS patterns.
* Integrate Rapier 2D physics.
* Build a deterministic physics step.

**Output**

* A playable prototype demonstrating drift, motion, and decay.

### Phase 2 — Environment (v0.3.0)

**Goal:** Build the orbital world loop

**Features**

* Procedurally generate orbits and stations.
* Implement multiple station states (damaged, derelict, intact).
* Add resource gathering and repair actions.
* Implement light/shadow simulation.
* Create parallax backgrounds and ambient lighting.

**Tech Goals**

* Develop modular orbit management.
* Implement entity lifecycle logic.
* Build reusable environment systems.

### Phase 3 — Systems Core (v0.6.0)

**Goal:** Make the world feel alive

**Features**

* Power network simulation (nodes, panels, batteries).
* Progressive orbital decay requiring maintenance.
* Communication from derelict habitats.
* Environmental hazards (debris, radiation, heat loss).

**Tech Goals**

* Create a simulation clock and event scheduler.
* Implement save/load system.
* Build ECS-driven UI and player feedback.

### Phase 4 — Atmosphere & Narrative (v0.8.0)

**Goal:** Add emotional depth and polish

**Features**

* Dynamic ambient soundtrack system.
* Fragmented logs from prior explorers.
* Particle effects, flickering lights, motion parallax.
* Occasional unknown entity communications.

**Tech Goals**

* Implement audio transitions and spatial zones.
* Develop procedural narrative events.
* Serialize narrative memory for replayability.

### Phase 5 — Content & Polish (v1.0.0)

**Goal:** Deliver a full, stable release

**Features**

* Include 4–5 unique orbital environments.
* Implement complete repair/survival loop.
* Create an ending sequence (symbolic or melancholic).
* Allow configurable speed and difficulty.
* Ensure balanced and polished systems.

**Tech Goals**

* Optimize ECS systems.
* Build cross-platform releases (Linux, Windows, macOS).
* Automate asset loading pipeline.

---

## 5. Long-Term Maintenance

**Post-1.0 Possibilities**

* Modding support via JSON/TOML files.
* Expanded procedural narrative events.
* Shader-based visual effects.
* Experimental co-op mode.

**CI/CD & Versioning**

* Use GitHub Actions for automated builds and tests.
* Follow semantic versioning (`0.x` pre-release, `1.x` stable).
* Deploy to Itch.io or Steam.

---

## 6. Project Infrastructure

**Suggested Repository Structure**

```text
hollow_orbit/
├─ src/
│  ├─ main.rs
│  ├─ components/
│  ├─ systems/
│  ├─ resources/
│  └─ states/
├─ assets/
│  ├─ sprites/
│  ├─ audio/
│  ├─ shaders/
│  └─ data/
├─ scripts/
│  ├─ build_release.sh
│  └─ deploy_itch.sh
├─ Cargo.toml
└─ README.md
```

**Tooling**

* `cargo watch` for live rebuilds.
* `bevy_inspector_egui` for debugging entities.
* Git LFS for asset management.
* `just` for automation scripts.

---

## 7. Visual & Design Language

**Art Style**

* Minimalist pixel art (64-color palette, see PALETTE.md for hex values).
* Depth defined by light and shadow.

**Sound Design**

* Silence is integral to atmosphere.
* Mechanical hums, fading echoes, static.
* Reflects isolation and entropy.

**Inspirations**

* Outer Wilds — existential tone.
* Hyper Light Drifter — precise movement.
* Risk of Rain — scale and isolation.
* Gris — emotional atmosphere.

---

## 8. Version Milestones Summary

| Version | Goal                        | Major Systems                 |
| ------- | --------------------------- | ----------------------------- |
| 0.1.0   | Movement prototype          | Bevy + Rapier integration     |
| 0.3.0   | Procedural orbit generation | Orbit and decay systems       |
| 0.6.0   | Core gameplay loop          | Power, communication, hazards |
| 0.8.0   | Narrative + visuals         | Audio, story fragments        |
| 1.0.0   | Full release                | Complete, polished experience |

---

## 9. Technical Foundations

**Coding Style**

* ECS-first architecture.
* Components hold minimal state.
* Systems are pure functions acting on ECS queries.
* Use Rust modules (`mod`) to organize code.

**Example System**

```rust
// src/systems/orbit.rs
pub fn update_orbit_system(
    mut query: Query<(&mut Transform, &Orbit)>,
    time: Res<Time>
) {
    for (mut transform, orbit) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(orbit.speed * time.delta_seconds());
    }
}
```

**Patterns**

* Resources: global game state (time, player stats, system status).
* Events: decoupled messages (damage, logs, triggers).

---

## 10. Development Practices

**Version Control**

* Keep main branch stable.
* Use feature branches for systems or gameplay modules.
* Merge only after passing builds and tests.

**Testing**

* Unit tests for system logic.
* Integration tests for resource interactions.
* Use Bevy headless mode for simulation testing.

**Documentation**

* Semantic versioning: `v0.x`, `v1.x`.

---

## 11. Project Philosophy

* Hollow Orbit’s antagonist is entropy itself.
* No combat; focus on decay and survival.
* Every mechanic reinforces fragility, isolation, and impermanence.
