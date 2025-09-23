# Rumble

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Map-specific Stats](#map-specific-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Rumble |
| **Title** | the Mechanized Menace |
| **Resource** | Heat |
| **Range Type** | Melee |
| **Release Date** | 2011-04-26 |
| **Release Patch** | V1.0.0.116 |
| **Latest Changes** | V25.17 |
| **Roles** | Battlemage |
| **Riot Positions** | Top |
| **External Positions** | Top, Middle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $655.0$ | $+105.0$ |
| **Mana** | $150.0$ | $+0.0$ |
| **Health Regen** | $7.0$ | $+0.6$ |
| **Armor** | $36.0$ | $+4.7$ |
| **Magic Resist** | $28.0$ | $+1.55$ |
| **Attack Damage** | $64.0$ | $+3.2$ |
| **Attack Speed** | $0.644$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.644$ | |
| **Attack Speed Ratio** | $0.644$ | |
| **Bonus AS per Level** | $1.9\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $140$ units | |
| **Selection Height** | $170$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Junkyard Titan

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Effects** | Proc |
| **Parry** | True |

**INNATE:** **Rumble**’s basic ability casts generate Heat, which decays by 10 Heat per second after not using any basic ability within 4 seconds or *The Equalizer* within 2 seconds. **Rumble**’s mech enters the *Danger Zone* when at or above 50 Heat, and becomes *Overheated* while at 150 Heat.

**DANGER ZONE:** **Rumble**’s mech enters the *Danger Zone*, empowering his basic abilities.

**OVERHEATED:** **Rumble**’s mech becomes *Overheated*, disabling his abilities as his Heat decays back down to 0 over 4 seconds. During this time, he gains and empowers his basic attacks to deal 5 to 40 (+ 25% AP) (+ 4% of the target's **maximum** health) **bonus** magic damage on-hit. The damage based on the target's health ratio is capped at 65+((150-65)/17)*(x-1)*(0.7025+0.0175*(x-1)) against monsters.

**Notes:**

- While **Rumble** is *overheating*, a *Silenced* icon will appear above him to signify he cannot cast abilities. This is visible to all units.
- The attacks do not deal the **bonus** damage against structures.

---

### Q: Flamespitter

| Attribute | Value |
|-----------|------:|
| **Range** | 600 (Cone length) / -50 (Cone origin point behind Rumble's center) units |
| **Cast Time** | none |
| **Angle** | er 64° |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | AoeDoT |
| **Projectile** | False |

**ACTIVE:** **Rumble** generates 20 Heat to activate his flamethrower for 3 seconds, spewing forth flames in a frontal (In his facing direction) cone every $0.25$ seconds. Enemies hit by the flame are scorched for 0.5 + 0.1 seconds, taking magic damage every $0.25$ seconds as well as upon being hit if not currently scorched, refreshing on subsequent inflictions.

*Flamespitter*’s damage is reduced to 70% against minions.

| Attribute | Value |
|-----------|------:|
| **Minimum Magic Damage** | 15 / 21.25 / 27.5 / 33.75 / 40 (+ 25% AP) (+ 1.5 / 1.625 / 1.75 / 1.875 / 2% of target's **maximum** health) |
| **Magic Damage per Second** | 20 / 28.33 / 36.67 / 45 / 53.33 (+ 33.33% AP) (+ 2 / 2.167 / 2.333 / 2.5 / 2.667% of target's **maximum** health) |
| **Magic Damage per Tick** | 5 / 7.083 / 9.167 / 11.25 / 13.333 (+ 8.333% AP) (+ 0.5 / 0.542 / 0.583 / 0.625 / 0.667% of target's **maximum** health) |
| **Maximum Magic Damage** | 75 / 106.25 / 137.5 / 168.75 / 200 (+ 125% AP) (+ 7.5 / 8.13 / 8.75 / 9.38 / 10% of target's **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Minimum Minion Damage** | 10.5 / 14.88 / 19.25 / 23.63 / 28 (+ 17.5% AP) (+ 1.05 / 1.14 / 1.22 / 1.31 / 1.4% of target's **maximum** health) |
| **Minion Damage per Second** | 14 / 19.83 / 25.67 / 31.5 / 37.33 (+ 23.33% AP) (+ 1.4 / 1.52 / 1.63 / 1.75 / 1.87% of target's **maximum** health) |
| **Minion Damage per Tick** | 3.5 / 4.958 / 6.417 / 7.875 / 9.333 (+ 5.833% AP) (+ 0.35 / 0.379 / 0.408 / 0.437 / 0.467% of target's **maximum** health) |
| **Maximum Minion Damage** | 52.5 / 74.38 / 96.25 / 118.13 / 140 (+ 87.5% AP) (+ 5.25 / 5.69 / 6.13 / 6.56 / 7% of target's **maximum** health) |

** *Flamespitter*’s damage is increased by 50%.

*Flamespitter*’s total damage based on the target's health is capped at 65+((300-65)/17)*(x-1)*(0.7025+0.0175*(x-1)) against monsters.

| Attribute | Value |
|-----------|------:|
| **Minimum Enhanced Damage** | 22.5 / 31.875 / 41.25 / 50.625 / 60 (+ 37.5% AP) (+ 2.25 / 2.438 / 2.625 / 2.813 / 3% of target's **maximum** health) |
| **Enhanced Damage per Second** | 30 / 42.5 / 55 / 67.5 / 80 (+ 50% AP) (+ 3 / 3.25 / 3.5 / 3.75 / 4% of target's **maximum** health) |
| **Enhanced Damage per Tick** | 7.5 / 10.625 / 13.75 / 16.875 / 20 (+ 12.5% AP) (+ 0.75 / 0.813 / 0.875 / 0.938 / 1% of target's **maximum** health) |
| **Maximum Enhanced Damage** | 112.5 / 159.38 / 206.25 / 253.13 / 300 (+ 187.5% AP) (+ 11.25 / 12.19 / 13.13 / 14.06 / 15% of target's **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Minimum Enhanced Minion Damage** | 15.75 / 22.313 / 28.875 / 35.438 / 42 (+ 26.25% AP) (+ 1.575 / 1.706 / 1.837 / 1.969 / 2.1% of target's **maximum** health) |
| **Enhanced Minion Damage per Second** | 21 / 29.75 / 38.5 / 47.25 / 56 (+ 35% AP) (+ 2.1 / 2.275 / 2.45 / 2.625 / 2.8% of target's **maximum** health) |
| **Enhanced Minion Damage per Tick** | 5.25 / 7.438 / 9.625 / 11.813 / 14 (+ 8.75% AP) (+ 0.525 / 0.569 / 0.612 / 0.656 / 0.7% of target's **maximum** health) |
| **Maximum Enhanced Minion Damage** | 78.75 / 124.69 / 170.63 / 216.56 / 262.5 (+ 131.25% AP) (+ 7.88 / 8.53 / 9.19 / 9.84 / 10.5% of target's **maximum** health) |

**Notes:**

- *Flamespitter* will deal an additional tick of damage to a minion if it would die to its per-tick damage. This is intended.
- **Rumble** will instantly turn to face the cursor if he is not moving when *Flamespitter* is cast. He will also instantly turn when issuing an Attack order on an enemy while *Flamespitter* is active.
  - Issuing an Attack Move order at a location will not cause **Rumble** to instantly turn.
  - If **Rumble** does not path to a location after an Attack or Attack Move order, and the player issues a Stop or Hold order immediately afterwards, **Rumble** will not visually turn completely to face the new direction, but the area will turn to have the correct direction as intended.
- When *Flamespitter*’s debuff is applied instead of refreshed, the target is dealt one additional instance of the debuff at the same time as the application. This is done by delaying the actual debuff by $0.1$ seconds.
  - If the target is already affected by the debuff, its duration is refreshed to the maximum ($0.6$ seconds).

---

### W: Scrap Shield

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cooldown** | 6 seconds |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Rumble** generates 20 Heat to grant himself a shield for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 25 / 55 / 85 / 115 / 145 (+ 30% AP) (+ 4% of **maximum** health) |

**Rumble** also gains ms for 1 to $1.32$ seconds. (See notes)

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 10 / 15 / 20 / 25 / 30% |

** *Scrap Shield*’s shield strength and **bonus** movement speed are increased in effectiveness by 50%.

| Attribute | Value |
|-----------|------:|
| **Enhanced Shield Strength** | 37.5 / 82.5 / 127.5 / 172.5 / 217.5 (+ 45% AP) (+ 6% of **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Enhanced Bonus Movement Speed** | 15 / 22.5 / 30 / 37.5 / 45% |

**Notes:**

- The movement speed buff is supposed to last for 1 second, but due to a bug may last longer.
  - Like most buffs, the stat bonus is already only gained at the next stat update (every ). Despite this, the duration still inconsistent.

---

### E: Electro Harpoon

| Attribute | Value |
|-----------|------:|
| **Range** | 950 / er 890 units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 120 units |
| **Speed** | 2000 units/second |
| **Cooldown** | $0.5$ seconds |
| **Recharge** | 6 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**ACTIVE:** **Rumble** generates 20 Heat to shoot a harpoon in the target direction that deals magic damage to the first enemy hit, inflicting them with magic penetration for 4 seconds and slowing them for 2 seconds. These effects stack additively with multiple harpoons, refreshing their duration and stacking up to 2 times.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 55 / 80 / 105 / 130 / 155 (+ 50% AP) |
| **Total Magic Damage** | 110 / 160 / 210 / 260 / 310 (+ 100% AP) |

| Attribute | Value |
|-----------|------:|
| **Magic Resistance Reduction** | 10 / 12 / 14 / 16 / 18% |
| **Total MR Reduction** | 20 / 24 / 28 / 32 / 36% |

| Attribute | Value |
|-----------|------:|
| **Slow** | 15 / 20 / 25 / 30 / 35% |
| **Total Slow** | 30 / 40 / 50 / 60 / 70% |

**Rumble** periodically stocks an *Electro Harpoon* charge, up to a maximum of 2.

** *Electro Harpoon*’s effects are increased by 50%.

| Attribute | Value |
|-----------|------:|
| **Enhanced Damage** | 82.5 / 120 / 157.5 / 195 / 232.5 (+ 75% AP) |
| **Total Enhanced Damage** | 165 / 240 / 315 / 390 / 465 (+ 150% AP) |

| Attribute | Value |
|-----------|------:|
| **Enhanced Magic Resistance Reduction** | 15 / 18 / 21 / 24 / 27% |
| **Total Enhanced MR Reduction** | 30 / 36 / 42 / 48 / 54% |

| Attribute | Value |
|-----------|------:|
| **Enhanced Slow** | 22.5 / 30 / 37.5 / 45 / 52.5% |
| **Total Enhanced Slow** | 45 / 60 / 75 / 90 / 105% |

*If **Rumble** casts Electro Harpoon before his mech becomes Overheated, he may still use another charge within 3 seconds of the initial cast.*

**Notes:**

- If *Electro Harpoon* is empowered with Danger Zone's bonus and hits a target that is afflicted with the non-enhanced slow and shred applied by previous, non-empowered *Electro Harpoon* casts, the effectiveness of both debuffs on the target are increased to their enhanced values for all stacks of each debuff.
  - This also applies vice versa; a non-empowered *Electro Harpoon* cast will reduce the enhanced slow and shred applied by previous, empowered *Electro Harpoon* casts back to the non-enhanced values, for all stacks.
- A buff with the magic resistance reduction tooltip is applied to minions and pets, but their magic resistance is unaffected by *Electro Harpoon*.
- Each *Electro Harpoon* cast triggers Spellblade and reduces Force Pulse’s cooldown.
- *Electro Harpoon*’s missile will fail to fire if **Rumble** is suppressed during the cast time. Effect at cast time end
- **Rumble** stocks an *Electro Harpoon* charge before the ability is even learned.

---

### R: The Equalizer

| Attribute | Value |
|-----------|------:|
| **Range** | -175 (Damage range behind vector origin) / 1135 (Maximum damage range forward, estimated) units |
| **Cast Time** | None / $0.5833$ (Starts after the spell is cast) |
| **Target Range** | 1700 units |
| **Width** | 410 - $368.4$ (Widest and thinnest point of the chain of circles, respectively) units |
| **Speed** | 1600 (Vector deployment speed) units/second |
| **Cooldown** | 130 / 117.5 / 105 / 92.5 / 80 seconds |
| **Targeting** | Vector |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |
| **Projectile** | False |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Rumble** deploys a barrage of rockets from the sky along the target path in a line over $0.75$ seconds. Each rocket impacts the ground to create a field of fire that lasts $4.5$ seconds.

Enemies struck by the impact or within the field are marked *Burning* for 1 second, taking magic damage every $0.25$ seconds and being slowed by 35%, refreshing while in the area. Enemies may be *Burning* for up to 5 seconds, for a total of 20 instances of its effect.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Tick** | 35 / 43.75 / 52.5 / 61.25 / 70 (+ 8.75% AP) |
| **Magic Damage per Second** | 140 / 175 / 210 / 245 / 280 (+ 35% AP) |
| **Maximum Magic Damage** | 700 / 875 / 1050 / 1225 / 1400 (+ 175% AP) |

**Notes:**

- *The Equalizer*’s cumulative hitbox consists of a row of rockets, each having a circular area of effect with a radius of 205 units. Each rocket drops at an offset of 180 units from the previous in the vector of cast.
  - The missile that manages the rockets being deployed starts 150 units behind the target vector origin, and thus drops the first rocket at 30 units in front of the cast vector origin, which is equivalent to a period of from the cast point.
- A given field checks for targets to apply or refresh the debuff on once every $0.25$ seconds.
- Due to each field expiring at different points in time, champions moving in the duration of the rockets' launch as they are expiring may sometimes receive 1 more instance of the debuff at most beyond the maximum.

---

## Patch History

### V25.17
- Junkyard Titan
  - Overheat on-hit damage target health ratio reduced to 4% of target's **maximum** health from 5%.
- Flamespitter
  - Base damage per tick reduced to 5 / 7.08 / 9.17 / 11.25 / 13.33 from 5 / 7.5 / 10 / 12.5 / 15.

### V25.16
- General
  - Each ability's Heat generation amount is now displayed in the HUD when the setting "Show Spell Costs" is enabled. They exist in place of resource costs for most other champions.
- Flamespitter
  - AP ratio per tick reduced to 8.333% AP from 9.167% AP.
  - Target health ratio per tick reduced to 0.5 / 0.542 / 0.583 / 0.625 / 0.667% of target's **maximum** health from 0.5 / 0.567 / 0.633 / 0.7 / 0.767%.
  - **Bug Fixes:** The timing of damage by casters on the red will now match the timing of damage by casters on the blue side.
    - Debuff now affects enemies once at the same time as they are hit when the debuff is applied instead of refreshed.
  - **New Effect:** Upon being affected by *Flamespitter*’s debuff, targets now receive damage through a burn that lasts $0.6$ seconds, refreshing on subsequent hits and not stacking with the flame if already debuffed.
    - Now has an internal counter to apply 2 additional instances of damage at most after the final refresh, including the final application instance, as a safety mechanism.
- The Equalizer
  - **Bug Fixes:** The timing of damage by casters on the red will now match the timing of damage by casters on the blue side.
  - **Bug Fixes:** Rocket zones no longer start affecting enemies at an up-to $0.25$ second delay (their debuff application is no longer fuzzy).
  - Debuff hit cadence increased to once every $0.25$ seconds from once every $0.5$ seconds.
    - Now has an internal counter to apply 4 additional instances of damage at most after the final refresh, including the final application instance, as a safety mechanism.
      - This may sometimes be inconsistent and apply one more instance at most beyond this maximum after the whole duration, when standing inside overlapping zones.
  - **Bug Fixes:** Each rocket zone now always lasts exactly $4.5$ seconds each instead of all rockets' zones disappearing at once 5 seconds after the first missile lands due to their duration collectively refreshing to the same amount with each subsequent rocket launch.

### V25.12#June 12th Hotfix|V25.12
- Flamespitter
  - Target health ratio per tick increased to 0.5 / 0.567 / 0.633 / 0.7 / 0.767% of target's **maximum** health from 0.5 / 0.542 / 0.583 / 0.625 / 0.667%.
    - Total health ratio increased to 6 / 6.8 / 7.6 / 8.4 / 9.2% of target's **maximum** health from 6 / 6.5 / 7 / 7.5 / 8%.
- Electro Harpoon
  - Base slow increased to 15 / 20 / 25 / 30 / 35% from 10 / 15 / 20 / 25 / 30%.

### V25.12
- Flamespitter
  - Target health ratio per tick reduced to 0.5 / 0.542 / 0.583 / 0.625 / 0.667% of target's **maximum** health from 0.5 / 0.583 / 0.667 / 0.75 / 0.833%.
    - Total health ratio reduced to 6 / 6.5 / 7 / 7.5 / 8% of target's **maximum** health from 6 / 7 / 8 / 9 / 10%.
- Electro Harpoon
  - Base slow reduced to 10 / 15 / 20 / 25 / 30% from 15 / 20 / 25 / 30 / 35%.

### V25.S1.2
- Electro Harpoon
  - **Bug Fixes:** Magic resistance reduction is once again enhanced while in Danger Zone.

### V25.S1.1#January 22nd Hotfix|V25.S1.1
- Junkyard Titan
  - **Bug Fixes:** Now once again properly gains all bonus effects while in Danger Zone.

### V25.S1.1
- Rumble
  - The Equalizer
    - **Bug Fixes:** Restored all SFX.

### V14.24
- Flamespitter
  - AP ratio per tick increased to 9.167% AP from 8.333% AP.
    - Total AP ratio increased to 110% AP from 100% AP.
- Scrap Shield
  - AP ratio increased to 30% AP from 25% AP.

### V14.23
- Electro Harpoon
  - **Bug Fixes:** No longer incorrectly gains its Danger Zone bonus based on the resulting heat amount instead of the heat amount prior to casting the ability.

### V14.22
- Stats
  - Base health increased to 655 from 625.

## Trivia

- Rumble, Corki, and Kled are the only yordles with a mount (Tristy, ROFL Copter, and Skaarl, respectively)
- Rumble, Bard, Blitzcrank, Caitlyn, Lissandra, Pyke, Sion, Varus, Vi, Viego, Xerath, and Ziggs are the only champions who can apply crowd control on themselves.
- Until the release of Rumble, both Rumble and all of his skins have been released on April.

---
*This page was automatically generated from League of Legends Wiki data.*