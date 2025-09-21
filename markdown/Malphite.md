# Malphite

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Advanced Stats](#advanced-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Malphite |
| **Title** | Shard of the Monolith |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-09-02 |
| **Release Patch** | V0.9.22.16 |
| **Roles** | Vanguard |
| **Riot Positions** | Top |
| **External Positions** | Top, Middle, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $665.0$ | $+104.0$ | $2433.0$ |
| **Mana** | $280.0$ | $+60.0$ | $1300.0$ |
| **Health Regen** | $7.0$ | $+0.55$ | $16.4$ |
| **Mana Regen** | $7.3$ | $+0.55$ | $16.7$ |
| **Armor** | $37.0$ | $+4.95$ | $121.2$ |
| **Magic Resist** | $28.0$ | $+2.05$ | $62.8$ |
| **Attack Damage** | $62.0$ | $+4.0$ | $130.0$ |
| **Attack Speed** | $0.736$ | $+3.4\%$ | $1.161$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.736$ |
| **Attack Speed Ratio** | $0.638$ |
| **Bonus AS per Level** | $3.4\%$ |
| **Acquisition Radius** | $400 units$ |
| **Gameplay Radius** | $80 units$ |
| **Pathing Radius** | $50 units$ |
| **Selection Radius** | $125 units$ |
| **Selection Height** | $180 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Granite Shield

**Innate:** **Malphite** shields himself equal to a portion of his **maximum** health, which refreshes after a few seconds without taking damage.

**Innate - Shard of the Monolith:** **Malphite** gains increased percentage size equal to 8% of his **total** armor, capped at 35% increased at $. **Innate:** **Malphite** grants himself a shield equal to 10% of his **maximum** health. The shield lasts until it is broken, and refreshes after a few seconds of not taking damage.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- ''Malphite's** increased is affected by his base armor and growth, but also reduced by *Black Cleaver*. His increased does not consider *Thunderclap*’s **bonus'' armor.
  - The bonus **will** become a reduction if **Malphite** is dropped to *negative* armor values.
- *Granite Shield* does not refresh its cooldown from damage that is mitigated by shield.

---

### Q: Seismic Shard

**Active:** **Malphite** sends a shard to the target enemy that deals magic damage and slow them for a few seconds.

*He also gains *ms **bonus** movement speed* for the same duration.*

**Active:** **Malphite** sends a shard to the target enemy that deals magic damage and slow them for 3 seconds. **Malphite** also gains *ms **bonus** movement speed* equal to the raw amount the target lost from the slow for the duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 625 units |
| **Cooldown** | 8 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-90$ Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Speed** | 1200 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | single |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $70-270$ (+ 60% AP)
- **Slow:** $20-40$%

**Notes:**

- The rock spawns 100 units in front of **Malphite**.
- **Malphite** will gain the bonus movement speed for the full duration as long as *Seismic Shard* impacts the target.
  - Both a buff shown in the hotbar and duration bar above the ability icon will indicate the remaining duration.
- 'Seismic Shard's movement speed gain is entirely based off the target's current, and loss of, movement speed.
  - Targets that are already slow by another effect will grant **Malphite** diminished movement speed.
  - Slow resist and slow-immune targets will reduce or nullify ''Malphite's' gain.

---

### W: Thunderclap

**Passive:** **Malphite** gains *armor *bonus armor*, tripled while *Granite Shield* is active.

**Active:** ''Malphite's** next basic attack within a few seconds is empowered to gain **bonus range** and deal **bonus'' physical damage.

**Passive:** **Malphite** gains *armor *bonus armor*, tripled while *Granite Shield* is active. **Active:** **Malphite** empowers his next basic attack within 6 seconds to have an uncancelable windup, gain *50 **bonus** range*, and deal **bonus** physical damage on-hit. Additionally, ''Malphite's' basic attacks on-hit for the next 5 seconds are empowered to trigger a cone in the direction of the target that deals physical damage to enemies hit. *Thunderclap basic attack reset *'Malphite's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $10-8$ seconds |
| **Cast Time** | None |
| **Cost** | $30-50$ mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | physical |
| **Spell Shield** | False |
| **Spell Effects** | Proc |

**Scaling:**
- **Bonus Armor:* $10-30$% armor3-30×3$% armor
- **Bonus Physical Damage:** $30-70$ (+ 20% AP) (+ 15% armor)
- **Physical Damage:** $15-55$ (+ 30% AP)

**Notes:**

- The passive **will** amplify *negative* armor, too, if ''Malphite's' armor is dropped that low.
- The passive's bonus armor stacks with an instance of recursion.
- The empowered attack:
  - Benefits from 'Thunderclap's cone effect.
  - Prevents **Malphite** from casting *Seismic Shard* and *Ground Slam* during its windup.
  - *Cannot* have its attack windup canceled by new movement or attack commands, or casting *Unstoppable Force*.
  - *Can* have its windup canceled by casting *Hextech Rocketbelt*.
- The cone starts from ''Malphite's' location.
  - Unlike similar spells, it does not deal its damage separately to the primary target of the attack(s); if the target is outside the area of effect when struck, it will not take the bonus damage.
- : Interaction with parrying effects (dodge, block, blind).

---

### E: Ground Slam

**Active:** **Malphite** slams the ground beneath him, dealing magic damage to nearby enemies and cripple them for a short time.

**Active:** **Malphite** slams the ground beneath him, dealing magic damage to nearby enemies and cripple them for 3 seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 7 seconds |
| **Cast Time** | $0.2419$ seconds |
| **Cost** | 50 Mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Effect Radius** | 400 units |
| **Spell Shield** | True |
| **Spell Effects** | aoe |

**Scaling:**
- **Magic Damage:** $70-230$ (+ 60% AP)
- **Cripple Strength:** $30-50$%

**Notes:**

- The cripple stacks multiplicatively with other sources of attack speed reductions. Effect at cast time end

---

### R: Unstoppable Force

**Active:** **Malphite** dashes with displacement immunity to the target location. Upon arrival, he deals magic damage to nearby enemies and briefly airborne.

**Active:** **Malphite** dashes with displacement immunity to the target location. Upon arrival, he deals magic damage to nearby enemies and airborne for $1.5$ seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 1000 units |
| **Cooldown** | $130/105/80$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Speed** | 1500 + 100% movement speed |
| **Effect Radius** | 325 units |
| **Spell Shield** | True |
| **Spell Effects** | aoe |

**Scaling:**
- **Magic Damage:** $200/300/400$ (+ 90% AP)

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
- *Granite Shield*
  - Shield health ratio increased to 10% **maximum** health from 9%.

### V14.9
- General
  - Adjusted splash artwork for Malphite.

### V13.18
- General
  - Updated ability icons.

### V13.8
- *Thunderclap*
  - Attack base damage reduced to $30-70$ from $30-90$.
  - Cleave armor ratio reduced to 15% armor from 20%.

### V13.4
- *Thunderclap*
  - Attack armor ratio increased to 15% armor from 10%.
  - Cleave armor ratio increased to 20% armor from 15%.
  - Cooldown reduced to $10-8$ seconds from $12-10$.

### V12.23
- *Ground Slam*
  - Mana cost reduced to 50 at all ranks from $50-70$.
  - Base damage increased to $70-230$ from $60-200$.
  - Armor ratio increased to 40% **total** armor from 30%.

### V12.22
- Stats
  - Base mana regeneration reduced to $7.3$ from $7.32$.

### V12.19
- *Thunderclap*
  - Cleave AP ratio increased to 30% AP from 20% AP.
- *Unstoppable Force*
  - AP ratio increased to 90% AP from 80% AP.

### V12.16
- *Granite Shield*
  - Cooldown reduced to 8–6@1–13 seconds from 10–6@1–13.
- *Thunderclap*
  - Cooldown reduced to $12-10$ seconds from 12 at all ranks.

## Trivia

- While the *Ground Slam*’s icon shows Malphite punching the ground, the actual animation has him slam the ground with his open palm instead.
  - *Ground Slam* leaves an imprint shaped like the Riot Games Inc. logo upon impact.
- Malphite is one of a few champions to have multiple textures in one skin. When he uses *Granite Shield* or *Thunderclap*.
  - Four other champions with this feature are **LeBlanc** (via *Mirror Image*), **Shaco** (via *Hallucinate*), **Nasus** (via *Fury of the Sands*) and **Wukong** (via *Warrior Trickster*).
- Malphite's dance references Capoeira ('Ginga Steps' in particular).
  - A side-by-side comparison can be seen here.
- In Skarner's Theme Video, starting at 1:49, Malphite can be seen briefly with an updated visual model.

---
*This page was automatically generated from League of Legends Wiki data.*