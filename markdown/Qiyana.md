# Qiyana

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
| **Champion** | Qiyana |
| **Title** | Empress of the Elements |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2019-06-28 |
| **Release Patch** | V9.13 |
| **Latest Changes** | V25.18 |
| **Roles** | Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Assassin |
| **Alt Type** | Fighter |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 70 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $590.0$ | $+124.0$ |
| **Mana** | $375.0$ | $+60.0$ |
| **Health Regen** | $8.0$ | $+0.9$ |
| **Mana Regen** | $8.0$ | $+0.7$ |
| **Armor** | $31.0$ | $+4.5$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $64.0$ | $+3.1$ |
| **Attack Speed** | $0.688$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $150.0$ | $+0.0$ |
| **Base Attack Speed** | $0.688$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.1\%$ | |
| **Attack Windup** | $15.3\%$ | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $155$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $115.0\%$ |
| **Damage Taken** | $90.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $90.0\%$ |

## Abilities

### Passive: Royal Privilege

| Attribute | Value |
|-----------|------:|
| **On-target CD Static** | 25 |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Effects** | proc |
| **Parry** | Unknown |

**INNATE:** **Qiyana**’s basic attacks and basic abilities deal 15 to 83 (+ 25% **bonus** AD) (+ 30% AP) **bonus** physical damage. 

This effect cannot occur on the same target more than once every few seconds.

Gathering an Element with *Terrashape* resets *Royal Privilege*’s per-target cooldown for enemies affected by *Royal Privilege* using a different Element (or no Element).

**Notes:**

- *Royal Privilege*’s trigger from a basic attack can be blocked (bonus damage is negated and the on-target cooldown does not apply).
- *Royal Privilege* will not apply a cooldown to targets that are hit with an Elemental Wrath that has a different Element than the Element **Qiyana** currently holds.
  - This does not occur if a new Elemental Wrath with a different Element is cast before the first one lands.
- The effect will not trigger against structures nor wards.
- : *Royal Privilege*’s interaction with *parrying* effects (dodge, blind).
- *Royal Privilege* is applied in a separate damage instance from **Qiyana**’s basic attacks and abilities.
  - This causes effects like Bone Plating and Black Cleaver Carve to be applied twice.

---

### Q: Edge of Ixtal

| Attribute | Value |
|-----------|------:|
| **Range** | 525 (Rectangle forward range) / -40 (Rectangle backward range) units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 140 (Rectangle width) units |
| **Cost** | 35 mana |
| **Cooldown** | 7 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**ACTIVE:** **Qiyana** slashes forward in the target direction, dealing physical damage to enemies in a line, reduced to 75% damage against targets beyond the first. *Edge of Ixtal* deals 175% damage against monsters.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 70 / 100 / 130 / 160 / 190 (+ 90% **bonus** AD) |
| **Reduced Damage** | 52.5 / 75 / 97.5 / 120 / 142.5 (+ 67.5% **bonus** AD) |

Gathering an Element with *Terrashape* resets *Edge of Ixtal*’s cooldown and upgrades it into *Elemental Wrath*.

If cast during *Audacity’s* dash towards an enemy champion and the target is within 150 units of **Qiyana** at the end of the dash, *Edge of Ixtal* will autonomously at the target.

**Notes:**

- If cast during Audacity and the target uses a dash or blink or is no longer in vision, *Edge of Ixtal* will at the target's last location prior to them starting the dash or blink or being in vision. Effect at cast time end

---

### Q: Elemental Wrath

| Attribute | Value |
|-----------|------:|
| **Range** | 865 (Maximum total range (Estimated)) / -50 (Backwards check) units |
| **Width** | 250 (Initial sidewards check) / 200 (Hurl missile and explosion missile width) units |
| **Speed** | 1600 (Hurl missile speed) / 2000 (Explosion missile speed) units/second |
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**ACTIVE:** **Qiyana** hurls her blade in the target direction that blasts in a line upon hitting an enemy or maximum range, consuming the current Element to empower the blade with an additional effect.

Enemies hit are dealt *Edge of Ixtal*’s damage, reduced to 75% against subsequent enemies beyond the closest. *Elemental Wrath* deals 175% damage against monsters.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 70 / 100 / 130 / 160 / 190 (+ 90% **bonus** AD) |
| **Reduced Damage** | 67.5 / 86.25 / 105 / 123.75 / 142.5 (+ 67.5% **bonus** AD) |

**BRUSH:** The blade creates a grass field around **Qiyana** that lasts for up to 3 seconds, granting her invisibility and ms until she attacks, casts an ability other than *Terrashape*, or exits the field.

**RIVER:** The blast roots enemies hit for $0.5$ seconds, then slows them by 20% for 1 second.

**TERRAIN:** The blast deals 60% increased damage against enemies below 50% of their **maximum** health. Subsequent targets beyond the closest that are below the threshold take 35% increased damage instead.

| Attribute | Value |
|-----------|------:|
| **Increased Damage** | 112 / 160 / 208 / 256 / 304 (+ 144% **bonus** AD) |
| **Subsequent Increased Damage** | 94.5 / 135 / 175.5 / 216 / 256.5 (+ 121.5% **bonus** AD) |

**Notes:**

- Using a basic attack breaks the stealth at the start of the attack windup.

---

### W: Terrashape

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 1100 units |
| **Effect Radius** | 366 (Element targeting range) units |
| **Speed** | 440 + 100% movement speed |
| **Cost** | 25 / 30 / 35 / 40 / 45 Mana |
| **Cooldown** | 7 seconds |
| **Targeting** | Location |
| **Affects** | Self |
| **Damage Type** | magic |
| **Spell Effects** | proc |
| **Out of Range Behavior** | cast at max |
| **Parry** | Unknown |
| **Grounded** | True |
| **Knockdown** | True |

**PASSIVE:** While holding an Element, **Qiyana** gains as, range and **bonus** magic damage on her basic attacks on-hit and basic abilities. While out-of-combat and moving near the Element currently being held, she gains ms.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 15 / 20 / 25 / 30 / 35% |

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 8 / 16 / 24 / 32 / 40 (+ 20% **bonus** AD) (+ 45% AP) |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 3 / 5 / 7 / 9 / 11% |

**ACTIVE:** **Qiyana** dashes up to 300 units towards the target location to gather the Element of the nearest Terrashape Brush.png **BRUSH**, Terrashape River.png **RIVER** or Terrashape Rock.png **TERRAIN** she targeted.

*A target Element is required to cast this ability*.

Once *Terrashape* has been learned, **Qiyana** is automatically given the Terrashape Rock.png **TERRAIN** element upon respawning.

***Qiyana** can cast any of her abilities during the dash.*

**Notes:**

- *Terrashape* does not interact with player-generated terrain.
- The water puddles formed by the Ocean Drake on Summoner's Rift count as valid terrain for obtaining the Terrashape River.png **RIVER** element.
- The brushes grown by Brushmaker count for gathering the Terrashape Brush.png **BRUSH** element.
- Passive bonus damage from basic attacks can be blocked.
- The passive damage works with Guinsoo's Rageblade *Phantom Hit*.
- : Passive bonus damage interaction with *parrying* effects (dodge, blind).

---

### E: Audacity

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 650 units |
| **Speed** | 600 + units/second |
| **Cost** | 40 / 45 / 50 / 55 / 60 mana |
| **Cooldown** | 11 / 10 / 9 / 8 / 7 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | Single |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Qiyana** dashes a fixed distance in the direction of the target enemy. If they are in range upon arrival, she deals physical damage.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 50 / 90 / 130 / 170 / 210 (+ 50% **bonus** AD) |

***Qiyana** can cast any of her abilities during the dash.*

**Notes:**

- *Audacity* only damages targets if within 250 range upon completion of the dash.

---

### R: Supreme Display of Talent

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 875 units |
| **Effect Radius** | 11000 (Border limit) units |
| **Width** | 280 (Windblast) / 120 (Terrain Shockwave) units |
| **Speed** | 2000 (Windblast) / 2840 (Shockwave) units/second |
| **Cost** | 100 mana |
| **Cooldown** | 120 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies, Terrain |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**ACTIVE:** **Qiyana** sweeps a windblast in the target direction that knocks back enemies hit by 375 units (estimated), though not through terrain, and stops upon hitting **TERRAIN**.

The windblast creates a cascading shockwave across any **RIVER** or **BRUSH** it passes through, as well as around the borders of **TERRAIN** it reaches, dealing physical damage to enemies hit, stunning them for changedisplay=true seconds, and briefly granting sight of the area along its path. The damage based on the target's health ratio is capped against monsters.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 100 / 200 / 300 (+ 125% **bonus** AD) (+ 10% of target's **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Capped Monster Health Damage** | 500 / 750 / 1000 (+ 125% **bonus** AD) |

**Notes:**

- The windblast needs only to reach near terrain to trigger the terrain shockwave.
- The shockwave grants sight along each part of its path for $0.75$ seconds.
- Artificial terrain (e.g, Crystallize, Weaver's Wall, Pillar of Ice) touching map terrain will be treated as a single piece of terrain, even when bridging two pieces of map terrain together; the shockwave will wrap around all connecting terrain. The shockwave's path is determined once it initially spreads; any expired terrain will be treated as if it were still there.
- Projectile-intercepting effects will block the windblast but not the shockwave. Effect at cast time end

---

## Patch History

### V25.18
- Stats
  - Base attack damage reduced to 64 from 66.
  - Armor growth reduced to $4.5$ from $4.7$.
- Royal Privilege
  - Bonus AD ratio reduced to 25% **bonus** AD from 30%.

### V25.16
- Edge of Ixtal
  - Base damage increased to 70 / 100 / 130 / 160 / 190 from 60 / 90 / 120 / 150 / 180.
- Terrashape
  - Bonus attack speed increased to 15 / 20 / 25 / 30 / 35% from 5 / 10 / 15 / 20 / 25%.

### V14.21
- Stats
  - Base health regeneration increased to 8 from 6.
- Terrashape
  - Bonus AD ratio increased to 20% **bonus** AD from 10%.

### V14.16
- Stats
  - Base armor increased to 31 from 28.
- Edge of Ixtal
  - Base damage changed to 60 / 90 / 120 / 150 / 180 from 50 / 85 / 120 / 155 / 190.
  - Bonus AD ratio increased to 90% **bonus** AD from 75%.
  - Monster damage increased to 175% from 150%.
- Terrashape
  - Base damage reduced to 8 / 16 / 24 / 32 / 40 from 8 / 22 / 36 / 50 / 64.
- Supreme Display of Talent
  - Bonus AD ratio reduced to 125% **bonus** AD from 170%.
  - Monster damage cap bonus AD ratio reduced to 125% **bonus** AD from 150%.
    - *Base monster damage cap unchanged.*

### V14.15
- General
  - **Bug Fixes:** Corrected multiple triggers for various voicelines. This also resolves enemies being able to hear VO that is normally only meant for the player.

### V14.1
- Terrashape
  - **Bug Fixes:** *Terrashape* into Edge of Ixtal fast buffer no longer sends *Edge of Ixtal* towards the 0,0,0 map coordinate.
- Supreme Display of Talent
  - Knockback duration increased slightly to reduce chances of it not overlapping with wall stun.

### V13.24
- Stats
  - Base mana increased to 375 from 320.
  - Mana growth increased to 60 from 50.

### V13.5
- Edge of Ixtal
  - Base damage increased to 50 / 85 / 120 / 155 / 190 from 50 / 80 / 110 / 140 / 170.
- Audacity
  - Cooldown reduced to 11 / 10 / 9 / 8 / 7 seconds from 12 / 11 / 10 / 9 / 8.

### V13.4
- Royal Privilege
  - **Bug Fixes:** On-target cooldown reset now behaves more consistently when casting Elemental Wrath, Terrashape, and *Elemental Wrath* again in quick succession.

### V12.15
- Royal Privilege
  - Bonus AD ratio reduced to 30% **bonus** AD from 45%.
  - **Bug Fixes:** No longer applies an extra proc of *Royal Privilege* with First Strike equipped.

## Trivia

- While Quinn was the first champion whose name starts with Q, the then only remaining unused letter in the alphabet regarding champion names prior to her release, Qiyana's release made it so there were at least 2 champions starting with each letter of the alphabet.
- Qiyana Dab (dance) if she uses any emote within 10 seconds of a takedown.
- Qiyana is the strongest among the 10 sisters and this might be referring to '1v9,’ a common player term used when both having bad teammates and enemies.

---
*This page was automatically generated from League of Legends Wiki data.*