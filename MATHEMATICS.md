📐 MATHEMATICS.md: Orbital Decay and System Mechanics

The survival gameplay of Hollow Orbit is driven by a set of simplified, deterministic equations that model the gradual, inevitable decay of the orbital habitats. This model prioritizes atmosphere and player agency over Newtonian physics.

1. The Core Decay Metric: Orbital Integrity (I)

The central value governing the player's immediate survival is the Orbital Integrity (I). This single metric represents the habitat's stability.

1.1. The Integrity Equation

Integrity begins at 100% and decreases over time based on accumulated decay rates.
dtdI​=−(RBase​+RDrag​+RInstability​)

    I (Orbital Integrity): The habitat's health (0-100). When I≤0, the orbit fails, requiring an immediate, high-risk transfer to a new habitat.

    dtdI​: The instantaneous rate of decay (Integrity loss per second/tick).

    RBase​ (Inherent Decay Rate): The unavoidable, constant loss due to the orbit's intrinsic fragility. This value is permanently set when the player enters a new orbit and is the primary difficulty scalar.

    RDrag​ (Atmospheric Drag Rate): The variable loss caused by poor altitude maintenance. This rate is influenced by the player's maintenance of the Hull System.

    RInstability​ (System Instability Rate): The variable loss caused by internal critical component failures (e.g., Gyroscopes).

2. Variable Decay Rates (Player Agency)

The player's actions directly influence RDrag​ and RInstability​, offering temporary control over the total decay rate.

2.1. Atmospheric Drag (RDrag​)

Atmospheric drag increases non-linearly as the habitat's Hull System Integrity (HSystem​) drops, pulling the station into a lower, denser altitude band.
If HSystem​≤50:RDrag​=α⋅(1−100HSystem​​)2
Variable	Description	Notes
HSystem​	Hull System Integrity (0-100): Integrity of the propulsion and structural components.	Repaired by the player to reduce RDrag​.
α	Drag Constant (Difficulty Scaling)	A positive constant used to tune the severity of the drag effect.
Conditional Decay	Drag decay is only active when the system integrity is low (e.g., HSystem​≤50).	This gives the player a buffer before the penalty starts compounding rapidly.
Non-Linearity	The exponent of 2 ensures that drag decays exponentially as HSystem​ approaches 0.	This models the rapid, catastrophic failure of a low orbit.

2.2. System Instability (RInstability​)

Instability is a direct penalty based on the failure of critical internal systems, representing the habitat's tendency to tumble or suffer control loss.
RInstability​=n=1∑N​Wn​⋅Bn​
Variable	Description	Notes
N	The total number of critical components (e.g., Gyroscope, Primary CPU).	Each habitat will have a unique set of critical components.
Wn​	Component Weight/Criticality: A fixed value assigned to each component n.	A broken Gyroscope (Wn​=5) adds more decay than a broken Comm Array (Wn​=1).
Bn​	Broken State Multiplier: Binary value (0 or 1).	Bn​=1 if Component Integrity ≤0; Bn​=0 otherwise.
Effect	Instability decay is the sum of penalties from every currently failed critical component.	This encourages the player to prioritize essential repairs.

3. Player Actions and Resource Consumption

Player actions are governed by Power (P) and Resources (Res​).

3.1. Repair Action (Increasing Component Integrity)

The repair action increases a specific component's integrity (ΔHSystem​) by consuming both Power and general Repair Resources.
ΔHSystem​=Max HSystem​β⋅PConsumed​⋅ResConsumed​​​

    PConsumed​ and ResConsumed​​: Fixed amounts of power and resources spent per repair "tick."

    β (Repair Efficiency): A constant defining how effective the player's tools/effort are.

    Normalization: Dividing by Max HSystem​ ensures that repairing a small, low-integrity component uses the same effort model as a large, high-integrity system.

3.2. Orbital Correction (Emergency Stabilization)

This is a one-time, high-cost action to instantly boost the Orbital Integrity (I), providing a momentary life extension.
ΔICorrection​=Thrust Efficiency⋅Fuel Consumed

    This action consumes a dedicated resource (Fuel) and provides an instant bump to I.

    The effectiveness (Thrust Efficiency) may be negatively modified by the current HSystem​ score, reflecting damaged thrusters.

4. Orbit Transfer Mechanics

When the current orbit fails, the player executes an escape maneuver.

4.1. Difficulty Progression

The core difficulty of the game increases linearly with each subsequent orbit. The new orbit's inherent decay rate (R^Base​) is guaranteed to be worse than the last.
R^Base​=RBasePrevious​​+γ

    γ (Decay Escalation Constant): A fixed positive value that defines how much harder the game becomes after each successful transfer.

    Narrative Effect: This mathematical escalation reinforces the melancholy theme that the player is merely delaying the inevitable and that each sanctuary is less stable than the last.
