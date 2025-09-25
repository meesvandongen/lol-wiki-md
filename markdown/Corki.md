# Corki

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
| **Champion** | Corki |
| **Title** | the Daring Bombardier |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-09-19 |
| **Release Patch** | V0.9.25.21 |
| **Latest Changes** | V25.15 |
| **Roles** | Marksman |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Marksman |
| **Alt Type** | Mage |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 45 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $610.0$ | $+100.0$ |
| **Mana** | $350.0$ | $+40.0$ |
| **Health Regen** | $5.5$ | $+0.55$ |
| **Mana Regen** | $7.4$ | $+0.7$ |
| **Armor** | $27.0$ | $+4.5$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $52.0$ | $+2.0$ |
| **Attack Speed** | $0.644$ | |
| **Movement Speed** | $325.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.644$ | |
| **Attack Speed Ratio** | $0.644$ | |
| **Bonus AS per Level** | $2.8\%$ | |
| **Missile Speed** | $2000$ units/second | |
| **Acquisition Radius** | $550$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $135$ units | |
| **Selection Height** | $170$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Taken** | $90.0\%$ |

## Abilities

### Passive: Hextech Munitions

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Physical True |
| **Spell Effects** | basic |
| **Parry** | True |

**INNATE:** **Corki**’s basic attacks deal **bonus** true damage equal to 20% AD. This damage is affected by critical strike modifiers.

**Notes:**

- The mixed damage is dealt in two simultaneous instances of damage, but will pretend to be a single instance for most effects (such as Conqueror’s stacks).
  - The true damage is dealt in an instance before the physical damage.
  - Both instances deal basic damage and thus natively apply life steal.
  - The attack applies on-hit effects only once, right at the start.
  - Even if the target dies from the true damage, the physical damage will still be applied to it.
    - If the target dies from on-hit damage, the true damage portion will be skipped and only the physical damage applied.
  - Runaan's Hurricane Wind's Fury and Spellblade effects have both been special cased to benefit from *Hextech Munitions*, causing their damage dealt to deal **bonus** true damage equal to 20% of the pre-mitigation damage (Damage calculated before modifiers) dealt.

---

### Q: Phosphorus Bomb

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 825 units |
| **Effect Radius** | 275 units |
| **Speed** | 1100 units/second |
| **Cost** | 60 / 65 / 70 / 75 / 80 mana |
| **Cooldown** | 9 / 8.5 / 8 / 7.5 / 7 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Corki** launches a bomb at the target location that explodes upon impact, dealing magic damage to enemies hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 115 / 160 / 205 / 250 (+ 130% **bonus** AD) (+ 100% AP) |

The bomb also grants sight of the area for 6 seconds and reveals enemy champions hit for the same duration.

**Notes:**

- The missile has a minimum travel time of $0.227$ seconds.

---

### W: Valkyrie

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 300 (Minimum dash range) / 600 (Maximum dash range) units |
| **Collision Radius** | 100 units |
| **Effect Radius** | 200 units |
| **Speed** | 650 + units/second |
| **Cost** | 80 / 85 / 90 / 95 / 100 mana |
| **Cooldown** | 20 / 18 / 16 / 14 / 12 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Corki** dashes to the target location and drops bombs that leave up to 3 blazing patches along his path, depending on the distance traveled. Each patch lasts $2.5$ seconds.

Enemies within the patches are dealt magic damage every $0.5$ seconds, lingering for 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Tick** | 40 / 55 / 70 / 85 / 100 (+ 40% **bonus** AD) (+ 30% AP) |
| **Total Magic Damage** | 200 / 275 / 350 / 425 / 500 (+ 200% **bonus** AD) (+ 150% AP) |

*Gatling Gun can be cast during the dash.*

**Notes:**

- No additional information.

---

### W: Special Delivery

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 1800 units |
| **Collision Radius** | 100 units |
| **Effect Radius** | 200 units |
| **Speed** | 1500 units/second |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |
| **Grounded** | True |
| **Knockdown** | False |

**ACTIVE:** **Corki** dashes with displacement immunity to the target location, knocking aside all enemies in his path 500 units and leaving behind a trail of fire for 5 seconds, which grants sight of the area for its duration and for 3 seconds afterwards.

Enemies hit by **Corki**’s dash or within the trail are burned and slowed by 90% for 2 seconds, refreshing every $0.25$ seconds while inside the area. The burn deals 7.5 to 7.5 for 7 / then + 1.25*x for 8 / then + 2.5*x (+ 50% **bonus** AD) (+ 6% AP) magic damage every $0.25$ seconds.

Casting *Special Delivery* instantly resets *Valkyrie’s* cooldown.

*Special Delivery will cast at max range if cast beyond that.*

**Notes:**

- Enemies that stay within the trail for its entire duration are dealt (7.5 to 7.5)*7×4 for 7 / then + 1.25*x*28 for 8 / then +2.5*x*28 (+ 1400% **bonus** AD) (+ 168% AP) **total** magic damage.

---

### E: Gatling Gun

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 690 (Cone radius) units |
| **Angle** | er 35° |
| **Cost** | 50 / 55 / 60 / 65 / 70 mana |
| **Cooldown** | 12 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |
| **Projectile** | False |

**ACTIVE:** **Corki** sprays bullets in a cone toward his facing direction for 4 seconds, dealing physical damage every $0.25$ seconds to all enemies hit and applying a stack to them at the same tick rate for 2 seconds, refreshing with subsequent hits and stacking up to 4 times. Each stack reduces the target's (lethality) armor and (magic penetration) magic resistance.

| Attribute | Value |
|-----------|------:|
| **Physical Damage Per Tick** | 5 / 8.125 / 11.25 / 14.375 / 17.5 (+ 15% **bonus** AD) |
| **Total Physical Damage** | 80 / 130 / 180 / 230 / 280 (+ 240% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Resistances Reduction Per Stack** | 3 / 3.5 / 4 / 4.5 / 5 |
| **Total Resistances Reduction** | 12 / 14 / 16 / 18 / 20 |

**Notes:**

- The maximum resistances reduction is applied after 4 ticks (1 second).
- The direction of the gun firing changes depending on where **Corki** is facing.
  - **Corki** will turn to face in the direction of the cursor upon casting *Gatling Gun*. He will also instantly turn when issuing an Attack order on enemies while *Gatling Gun* is active.
  - Issuing an Attack Move order at a location will not cause **Corki** to instantly turn.
- *Gatling Gun*’s area of effect can hit close by enemies next to and behind him, due to registering enemies' edge.

---

### R: Missile Barrage

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.175$ seconds |
| **Target Range** | 1300 (Standard missile range) / 1500 (Big One range) units |
| **Effect Radius** | 150 (Standard missile explosion) / 300 (Big One explosion) units |
| **Width** | 80 (Both standard and Big One width) units |
| **Speed** | 2000 (Both standard and Big One speed) units/second |
| **Cost** | 35 mana + 1 Ammo |
| **Cooldown** | 2 seconds |
| **Recharge** | 20 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**PASSIVE:** Basic attacks on-hit against champions reduce *Missile Barrage*’s remaining recharge time by 2–4@0–100 (@=critical strike chance) seconds.

**ACTIVE:** **Corki** fires a missile in the target direction that explodes upon the first enemy hit, dealing physical damage to enemies within the area.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 90 / 130 / 170 / 210 / 250 (+ 85% **bonus** AD) |

**Corki** periodically stocks a *Missile Barrage* charge, up to a maximum of 4. He gains 2 charges upon learning the ability and maximum charges upon respawning.

Every third missile **Corki** fires is a *Big One*, dealing 100% increased damage as well as gaining increased range and explosion radius.

| Attribute | Value |
|-----------|------:|
| **Big One Physical Damage** | 180 / 260 / 340 / 420 / 500 (+ 170% **bonus** AD) |

**Notes:**

- Once **Corki** has learned the ability:
  - He receives a buff that counts a cycle of the number of times *Missile Barrage* has been used since the last *Big One*.
  - An indicator becomes visible below his health bar that tracks the number of missiles in reserve and whether a *Big One* is available. They are spent from left to right (default, for left-to-right locales).
- **Corki** retains progress towards a *Big One* when he dies.
- This ability will cast from wherever the caster is at the start of the cast time.
- *Missile Barrage*’s effect radius is centered around the location of the missile as it collides.
- Malignance damage from *Missile Barrage* does not count toward Eclipse passive.

---

## Patch History

### V25.15
- Phosphorus Bomb
  - Base damage reduced to 70 / 115 / 160 / 205 / 250 from 70 / 120 / 170 / 220 / 270.
  - Bonus AD ratio increased to 130% **bonus** AD from 120%.
- Missile Barrage
  - Bonus AD ratio increased to 85% **bonus** AD from 80%.
    - Big One bonus AD ratio increased to 170% **bonus** AD from 160%.

### V25.06
- Stats
  - Base armor reduced to 27 from 30.

### V14.24
- Phosphorus Bomb
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.
- Gatling Gun
  - Base damage per tick reduced to 5 / 8.125 / 11.25 / 14.375 / 17.5 from 6.25 / 9.375 / 12.5 / 15.625 / 18.75.
    - Total base damage reduced to 80 / 130 / 180 / 230 / 280 from 100 / 150 / 200 / 250 / 300.
  - Bonus AD ratio per tick reduced to 15% **bonus** AD from 15.625%.
    - Total bonus AD ratio reduced to 240% **bonus** AD from 250%.

### V14.22
- Stats
  - Attack damage growth reduced to 2 from $2.5$.
  - Mana growth reduced to 40 from 54.

### V14.21
- Hextech Munitions
  - AD ratio increased to 20% AD from 15% AD.
- Valkyrie
  - Base damage per tick increased to 40 / 55 / 70 / 85 / 100 from 30 / 45 / 60 / 75 / 90.
    - Total base damage increased to 200 / 275 / 350 / 425 / 500 from 150 / 225 / 300 / 375 / 450.
  - Bonus AD ratio per tick increased to 40% **bonus** AD from 30%.
    - Total bonus AD ratio increased to 200% **bonus** AD from 150%.
- Missile Barrage
  - Base damage increased to 90 / 170 / 250 from 80 / 160 / 240.
    - Big One base damage increased to 180 / 340 / 500 from 160 / 320 / 480.
  - Bonus AD ratio increased to 80% **bonus** AD from 70%.
    - Big One bonus AD ratio increased to 160% **bonus** AD from 140%.

### V14.20
- Phosphorus Bomb
  - Mana cost reduced to 60 / 65 / 70 / 75 / 80 from 80 at all ranks.
- Valkyrie
  - **New Effect:** Now scales with 150% **bonus** AD.
- Gatling Gun
  - Mana cost reduced to 50 / 55 / 60 / 65 / 70 from 50 / 60 / 70 / 80 / 90.

### V14.19#September 26th Hotfix|V14.19
- Phosphorus Bomb
  - Base damage increased to 70 / 120 / 170 / 220 / 270 from 70 / 115 / 160 / 205 / 250.
- Gatling Gun
  - Base damage per tick increased to 6.25 / 9.38 / 12.5 / 15.63 / 18.75 from 6.25 / 9.0625 / 11.875 / 14.6875 / 17.5.
    - Total base damage increased to 100 / 150 / 200 / 250 / 300 from 100 / 145 / 190 / 235 / 280.

### V14.18
- Phosphorus Bomb
  - Base damage reduced to 70 / 115 / 160 / 205 / 250 from 70 / 120 / 170 / 220 / 270.
- Missile Barrage
  - Base damage reduced to 80 / 160 / 240 from 80 / 180 / 280.
  - Bonus AD ratio reduced to 70% **bonus** AD from 80%.

### V14.16
- Phosphorus Bomb
  - Cooldown increased to 9 / 8.5 / 8 / 7.5 / 7 seconds from 8 / 7.5 / 7 / 6.5 / 6.
  - Mana cost increased to 80 at all ranks from 60 / 65 / 70 / 75 / 80.

### V14.14
- Stats
  - Base health reduced to 610 from 640.

## Trivia

- It was speculated that Corki was inspired by from *Dota 2*.
  - This has been proven false given Corki was released on 19-Sep-2009 (patch V0.9.25.21) while Gyrocopter was released on 28-Jul-2010 for the original DotA custom map.
    - Yet both of them are most likely based on the from *Warcraft III: Reign of Chaos* (*Warcraft III: The Frozen Throne*’s name: ).
- Corki was the first champion to have two skins as well as the first to have three.
  - All of them are old skins that are either Legacy or were never for sale to begin with.
- Corki's dance references the "Do a barrel roll!" phrase spoken by in Star Fox 64.
  - A side-by-side comparison can be seen here.
- In the now-removed official League of Legends forums, the original icon of Missile Barrage was used to represent the "In-Game HUD Discussion" section.
- The old sound effect for when Corki picks up The Package was similar to an raid siren, which was pointed out by a player in Syria. It was promptly changed.

---
*This page was automatically generated from League of Legends Wiki data.*