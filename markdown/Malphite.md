# Malphite

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
| **Champion** | Malphite |
| **Title** | Shard of the Monolith |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-09-02 |
| **Release Patch** | V0.9.22.16 |
| **Latest Changes** | V14.21 |
| **Roles** | Vanguard |
| **Riot Positions** | Top |
| **External Positions** | Top, Middle, Support |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 1 |
| **Hero Type** | Tank |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 1 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 75 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $665.0$ | $+104.0$ |
| **Mana** | $280.0$ | $+60.0$ |
| **Health Regen** | $7.0$ | $+0.55$ |
| **Mana Regen** | $7.3$ | $+0.55$ |
| **Armor** | $37.0$ | $+4.95$ |
| **Magic Resist** | $28.0$ | $+2.05$ |
| **Attack Damage** | $62.0$ | $+4.0$ |
| **Attack Speed** | $0.736$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.736$ | |
| **Attack Speed Ratio** | $0.638$ | |
| **Bonus AS per Level** | $3.4\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $50$ units | |
| **Selection Radius** | $125$ units | |
| **Selection Height** | $180$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |

## Abilities

### Passive: Granite Shield

| Attribute | Value |
|-----------|------:|
| **Static Cooldown** | 8–6@1–13 |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE - SHARD OF THE MONOLITH:** **Malphite** gains increased percentage size equal to 8% of his **total** armor, capped at 35% increased at $437.5$ armor.

**INNATE:** **Malphite** grants himself a shield equal to 10% of his **maximum** health. The shield lasts until it is broken, and refreshes after a few seconds of not taking damage.

**Notes:**

- **Malphite**’s increased is affected by his base armor and growth, but also reduced by Black Cleaver. His increased does not consider Thunderclap’s **bonus** armor.
  - The bonus **will** become a reduction if **Malphite** is dropped to *negative* armor values.
- *Granite Shield* does not refresh its cooldown from damage that is mitigated by shields.

---

### Q: Seismic Shard

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 625 units |
| **Speed** | 1200 units/second |
| **Cost** | 70 / 75 / 80 / 85 / 90 Mana |
| **Cooldown** | 8 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | single |
| **Projectile** | True |
| **Call For Help** | True |

**ACTIVE:** **Malphite** sends a shard to the target enemy that deals magic damage and slows them for 3 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 120 / 170 / 220 / 270 (+ 60% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 20 / 25 / 30 / 35 / 40% |

**Malphite** also gains ms equal to the raw amount the target lost from the slow for the duration.

**Notes:**

- The rock spawns 100 units in front of **Malphite**.
- **Malphite** will gain the bonus movement speed for the full duration as long as *Seismic Shard* impacts the target.
  - Both a buff shown in the hotbar and duration bar above the ability icon will indicate the remaining duration.
- *Seismic Shard*’s movement speed gain is entirely based off the target's current, and loss of, movement speed.
  - Targets that are already slowed by another effect will grant **Malphite** diminished movement speed.
  - Slow-resistant and slow-immune targets will reduce or nullify **Malphite**’s gain.

---

### W: Thunderclap

| Attribute | Value |
|-----------|------:|
| **Cast Time** | None |
| **Cost** | 30 / 35 / 40 / 45 / 50 mana |
| **Cooldown** | 10 / 9.5 / 9 / 8.5 / 8 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | physical |
| **Spell Shield** | False |
| **Spell Effects** | Proc |
| **Parry** | unknown |

**PASSIVE:** **Malphite** gains armor, tripled while Granite Shield is active.

| Attribute | Value |
|-----------|------:|
| **Bonus Armor** | 10 / 15 / 20 / 25 / 30% armor |
| **Increased Bonus Armor** | 30 / 45 / 60 / 75 / 90% armor |

**ACTIVE:** **Malphite** empowers his next basic attack within 6 seconds to have an uncancelable windup, gain 50 **bonus** range, and deal **bonus** physical damage on-hit.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 30 / 40 / 50 / 60 / 70 (+ 20% AP) (+ 15% armor) |

Additionally, **Malphite**’s basic attacks on-hit for the next 5 seconds are empowered to trigger a cone in the direction of the target that deals physical damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 15 / 25 / 35 / 45 / 55 (+ 30% AP) (+ 15% armor) |

*Thunderclap resets **Malphite**’s basic attack timer.*

**Notes:**

- The passive **will** amplify *negative* armor, too, if **Malphite**’s armor is dropped that low.
- The passive's bonus armor stacks with an instance of recursion.
- The empowered attack:
  - Benefits from *Thunderclap*’s cone effect.
  - Prevents **Malphite** from casting Seismic Shard and Ground Slam during its windup.
  - *Cannot* have its windup canceled by new movement or attack commands, or casting Unstoppable Force.
  - *Can* have its windup canceled by casting Hextech Rocketbelt.
- The cone starts from **Malphite**’s location.
  - Unlike similar spells, it does not deal its damage separately to the primary target of the attack(s); if the target is outside the area of effect when struck, it will not take the bonus damage.
- : Interaction with parrying effects (dodge, block, blind).

---

### E: Ground Slam

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.2419$ seconds |
| **Effect Radius** | 400 units |
| **Cost** | 50 Mana |
| **Cooldown** | 7 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |

**ACTIVE:** **Malphite** slams the ground beneath him, dealing magic damage to nearby enemies and crippling them for 3 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 110 / 150 / 190 / 230 (+ 60% AP) (+ 40% armor) || Attribute | Value |
|-----------|------:|
| **Cripple Strength** | 30 / 35 / 40 / 45 / 50% |

**Notes:**

- The cripple stacks multiplicatively with other sources of attack speed reductions. Effect at cast time end

---

### R: Unstoppable Force

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 1000 units |
| **Effect Radius** | 325 units |
| **Speed** | 1500 + 100% movement speed |
| **Cost** | 100 Mana |
| **Cooldown** | 130 / 105 / 80 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Knockdown** | False |

**ACTIVE:** **Malphite** dashes with displacement immunity to the target location. Upon arrival, he deals magic damage to nearby enemies and knocks them up for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 200 / 300 / 400 (+ 90% AP) |

**Notes:**

- The dash speed can be up-to doubled if *Unstoppable Force* is targeted at the middle of a wall so that his destination ends up on the other side.
  - It will be decreased if his destination ends up closer to him than his target location.
- If **Malphite** dies during his dash, the ability cancels.
  - If the dash is interrupted without **Malphite** dying, for instance when he enters resurrection during *Unstoppable Force*, he halts and the ability instead affects enemies at the stopped location.

---

## Patch History

### V14.21
- Stats
  - Base health increased to 665 from 644.

### V14.18
- Granite Shield
  - Shield health ratio increased to 10% **maximum** health from 9%.

### V14.9
- General
  - Adjusted splash artwork for Malphite.

### V13.18
- General
  - Updated ability icons.

### V13.8
- Thunderclap
  - Attack base damage reduced to 30 / 40 / 50 / 60 / 70 from 30 / 45 / 60 / 75 / 90.
  - Cleave armor ratio reduced to 15% armor from 20%.

### V13.4
- Thunderclap
  - Attack armor ratio increased to 15% armor from 10%.
  - Cleave armor ratio increased to 20% armor from 15%.
  - Cooldown reduced to 10 / 9.5 / 9 / 8.5 / 8 seconds from 12 / 11.5 / 11 / 10.5 / 10.

### V12.23
- Ground Slam
  - Mana cost reduced to 50 at all ranks from 50 / 55 / 60 / 65 / 70.
  - Base damage increased to 70 / 110 / 150 / 190 / 230 from 60 / 95 / 130 / 165 / 200.
  - Armor ratio increased to 40% **total** armor from 30%.

### V12.22
- Stats
  - Base mana regeneration reduced to $7.3$ from $7.32$.

### V12.19
- Thunderclap
  - Cleave AP ratio increased to 30% AP from 20% AP.
- Unstoppable Force
  - AP ratio increased to 90% AP from 80% AP.

### V12.16
- Granite Shield
  - Cooldown reduced to 8–6@1–13 seconds from 10–6@1–13.
- Thunderclap
  - Cooldown reduced to 12 / 11.5 / 11 / 10.5 / 10 seconds from 12 at all ranks.

## Trivia

- While the Ground Slam’s icon shows Malphite punching the ground, the actual animation has him slam the ground with his open palm instead.
  - Ground Slam leaves an imprint shaped like the Riot Games Inc. logo upon impact.
- Malphite is one of a few champions to have multiple textures in one skin. When he uses Granite Shield or Thunderclap.
  - Four other champions with this feature are LeBlanc (via Mirror Image), Shaco (via Hallucinate), Nasus (via Fury of the Sands) and Wukong (via Warrior Trickster).
- Malphite's dance references Capoeira ('Ginga Steps' in particular).
  - A side-by-side comparison can be seen here.

---
*This page was automatically generated from League of Legends Wiki data.*