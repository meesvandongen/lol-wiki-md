# Qiyana

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
| **Champion** | Qiyana |
| **Title** | Empress of the Elements |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2019-06-28 |
| **Release Patch** | V9.13 |
| **Roles** | Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $590.0$ | $+124.0$ | $2698.0$ |
| **Mana** | $375.0$ | $+60.0$ | $1395.0$ |
| **Health Regen** | $8.0$ | $+0.9$ | $23.3$ |
| **Mana Regen** | $8.0$ | $+0.7$ | $19.9$ |
| **Armor** | $31.0$ | $+4.5$ | $107.5$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $64.0$ | $+3.1$ | $116.7$ |
| **Attack Speed** | $0.688$ | $+2.1\%$ | $0.934$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $150.0$ | $+0.0$ | $150.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.688$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.1\%$ |
| **Attack Windup** | $15.3\%$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $155 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Royal Privilege

**Innate:** **Qiyana**’s basic attacks and ability deal **bonus** physical damage.

*This effect cannot occur on the same target for a period of time. Gathering an Element with **Terrashape** will cdr the cooldown.*

**Innate:** ''Qiyana's** basic attacks and ability deal 15 to 83 (+ 25% *bonus AD) (+ 30% AP) **bonus'' physical damage. This effect cannot occur on the same target more than once every few seconds. Gathering an Element with **Terrashape** resets 'Royal Privilege's* per-target *cooldown* for enemies affected by *Royal Privilege' using a different Element (or no Element).

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Effects** | proc |

**Notes:**

- 'Royal Privilege's trigger from a basic attack can be blocked (bonus damage is negated and the on-target cooldown does not apply).
- *Royal Privilege* will not apply a cooldown to targets that are hit with an *Elemental Wrath* that has a different *Element* than the *Element* **Qiyana** currently holds.
  - This does not occur if a new *Elemental Wrath* with a different *Element* is cast before the first one lands.
- The effect will not trigger against structures nor wards.
- : 'Royal Privilege's* interaction with *parrying' effects (dodge, blind).
- *Royal Privilege* is applied in a separate damage instance from ''Qiyana's' basic attacks and abilities.
  - This causes effects like *Bone Plating* and *Black Cleaver* Carve to be applied twice.

---

### Q: Edge of Ixtal

**Active:** **Qiyana** slashes forward in the target direction, dealing physical damage to enemies hit in a line.

*Gathering an Element with **Terrashape** will cdr the cooldown and upgrades *Edge of Ixtal* into *Elemental Wrath*.*

**Active:** **Qiyana** slashes forward in the target direction, dealing physical damage to enemies in a line, reduced to 75% damage against targets beyond the first. *Edge of Ixtal* deals 175% damage against monsters. Gathering an Element with **Terrashape** resets 'Edge of Ixtal's* *cooldown* and upgrades it into *Elemental Wrath'. If cast during **Audacity*’s* dash towards an enemy champion and the target is within 150 units of **Qiyana** at the end of the dash, *Edge of Ixtal* will autonomously at the target.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 7 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 35 mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**Scaling:**
- **Physical Damage:** $70-190$ (+ 90% bonus AD); pass-through: $70-190$ × 0.75 (+ $90×0.75$% bonus AD)

**Notes:**

- If cast during *Audacity* and the target uses a dash or blink or is no longer in vision, *Edge of Ixtal* will at the target's last location prior to them starting the dash or blink or being in vision. Effect at cast time end

---

### Q: Elemental Wrath

**Active:** **Qiyana** hurls her blade in the target direction that blasts in a line upon hitting an enemy or max range, dealing physical damage, reduced against enemies beyond the closest. This consumes the current Element to empower the blade with an additional effect. * **Brush:** The blade creates a grass field around **Qiyana** that lasts a short time, granting her invisibility and *ms **bonus** movement speed* until she attacks or exits the field. * **River:** The blast briefly root enemies hit, then briefly slow them. * **Terrain:** The blast deals **bonus** damage against enemies below half health.

**Active:** **Qiyana** hurls her blade in the target direction that blasts in a line upon hitting an enemy or maximum range, consuming the current Element to empower the blade with an additional effect. Enemies hit are dealt 'Edge of Ixtal's* damage, reduced to 75% against subsequent enemies beyond the closest. *Elemental Wrath' deals 175% damage against monsters. **Brush:** The blade creates a grass field around **Qiyana** that lasts for up to 3 seconds, granting her invisibility and ms*bonus** movement speed* until she attacks, casts an ability other than **Terrashape**, or exits the field. **River:** The blast root enemies hit for $0.5$ seconds, then slow them by 20% for 1 second. **Terrain:** The blast deals 60% increased damage against enemies below 50% of their **maximum** health. Subsequent targets beyond the closest that are below the threshold take 35% increased damage instead.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Speed** | 1600 / 2000 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:** $70-190$ (+ 90% bonus AD); pass-through: $70-190$ × 0.75 (+ $90×0.75$% bonus AD)

**Notes:**

- Using a basic attack breaks the stealth at the start of the attack windup.

---

### W: Terrashape

**Passive:** While holding an Element, **Qiyana** gains **bonus** as and range, as well as **bonus** magic damage on her basic attacks and basic abilities. **Qiyana** gains *ms **bonus** movement speed* speed while moving near the current Element.

**Active:** **Qiyana** dashes in the target direction to gather the Element of the nearest Terrashape Brush.png **Brush**, Terrashape River.png **River** or Terrashape Rock.png **Terrain** she targeted.

**Passive:** While holding an Element, **Qiyana** gains *as *bonus attack speed*, range*bonus** attack range* and **bonus** magic damage on her basic attacks on-hit and basic abilities. While out-of-combat and moving near the Element currently being held, she gains *ms **bonus** movement speed*. **Active:** **Qiyana** dashes up to 300 units towards the target location to gather the Element of the nearest Terrashape Brush.png **Brush**, Terrashape River.png **River** or Terrashape Rock.png **Terrain** she targeted. *A target Element is required to cast this ability*. Once *Terrashape* has been learned, **Qiyana** is automatically given the Terrashape Rock.png **Terrain** element upon respawning. **Qiyana can cast any of her abilities during the dash.**

| Attribute | Value |
|-----------|-------|
| **Range** | 1100 units |
| **Cooldown** | 7 seconds |
| **Cast Time** | none |
| **Cost** | $25-45$ Mana |
| **Targeting** | Location |
| **Affects** | Self |
| **Damage Type** | magic |
| **Speed** | 440 + 100% movement speed |
| **Effect Radius** | 366 units |
| **Spell Effects** | proc |

**Scaling:**
- **Bonus Attack Speed:** $15-35$%
- **Bonus Magic Damage:** $8-40$ (+ 20%
- *bonus AD) (+ 45% AP)
- **Bonus Movement Speed:** $3-11$%

**Notes:**

- *Terrashape* does not interact with player-generated terrain.
- The water puddles formed by the Ocean Drake on Summoner's Rift count as valid terrain for obtaining the Terrashape River.png **River** element.
- The brush grown by Brushmaker count for gathering the Terrashape Brush.png **Brush** element.
- Passive bonus damage from basic attacks can be blocked.
- The passive damage works with *Guinsoo's Rageblade* *Phantom Hit*.
- : Passive bonus damage interaction with *parrying* effects (dodge, blind).

---

### E: Audacity

**Active:** **Qiyana** dash toward the target enemy and deals physical damage upon arrival.

**Active:** **Qiyana** dash a fixed distance in the direction of the target enemy. If they are in range upon arrival, she deals physical damage. **Qiyana can cast any of her abilities during the dash.**

| Attribute | Value |
|-----------|-------|
| **Range** | 650 units |
| **Cooldown** | $11-7$ seconds |
| **Cast Time** | none |
| **Cost** | $40-60$ mana |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | physical |
| **Speed** | 600 + Capped at 1200 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Single |

**Scaling:**
- **Physical Damage:** $50-210$ bonus AD)

**Notes:**

- *Audacity* only damages targets if within 250 range upon completion of the dash.

---

### R: Supreme Display of Talent

**Active:** **Qiyana** sweeps a windblast in the target direction that airborne enemies hit and stops upon hitting **Terrain**.

*The windblast creates a cascading shockwave across any **River** or **Brush** it passes, as well as around the borders of **Terrain** it hits. The shockwave briefly stun enemies hit and deals physical damage based on their **maximum** health.*

**Active:** **Qiyana** sweeps a windblast in the target direction that airborne enemies hit by 375 units, though not through terrain, and stops upon hitting **Terrain**. The windblast creates a cascading shockwave across any **River** or **Brush** it passes through, as well as around the borders of **Terrain** it reaches, dealing physical damage to enemies hit, stun them for changedisplay=true seconds, and briefly granting sight of the area along its path. The damage based on the target's health ratio is capped against monster.

| Attribute | Value |
|-----------|-------|
| **Range** | 875 units |
| **Cooldown** | 120 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 mana |
| **Targeting** | Direction |
| **Affects** | Enemies, Terrain |
| **Damage Type** | physical |
| **Speed** | 2000 / 2840 units/second |
| **Effect Radius** | 11000 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**Scaling:**
- **Physical Damage:** $100-300 3$
- *bonus AD) (+ 10% of target's
- **maximum** health)
- **Capped Monster Health Damage:** $500-1000 3$ (+ 125% bonus AD)

**Notes:**

- The windblast needs only to reach near terrain to trigger the terrain shockwave.
- The shockwave grants sight along each part of its path for $0.75$ seconds.
- Artificial terrain (e.g, Crystallize, Weaver's Wall, Pillar of Ice) touching map terrain will be treated as a single piece of terrain, even when bridging two pieces of map terrain together; the shockwave will wrap around all connecting terrain. The shockwave's path is determined once it initially spreads; any expired terrain will be treated as if it were still there.
- Projectile effects will block the windblast but not the shockwave. Effect at cast time end

---

## Patch History

### V25.18
- Stats
  - Base attack damage reduced to 64 from 66.
  - Armor growth reduced to $4.5$ from $4.7$.
- *Royal Privilege*
  - Bonus AD ratio reduced to 25% *bonus AD from 30%.

### V25.16
- *Edge of Ixtal*
  - Base damage increased to $70-190$ from $60-180$.
- *Terrashape*
  - Bonus attack speed increased to $15-35$% from $5-25$%.

### V14.21
- Stats
  - Base health regeneration increased to 8 from 6.
- *Terrashape*
  - Bonus AD ratio increased to 20% *bonus AD from 10%.

### V14.16
- Stats
  - Base armor increased to 31 from 28.
- *Edge of Ixtal*
  - Base damage changed to $60-180$ from $50-190$.
  - Bonus AD ratio increased to 90% *bonus AD from 75%.
  - Monster damage increased to 175% from 150%.
- *Terrashape*
  - Base damage reduced to $8-40$ from $8-64$.
- *Supreme Display of Talent*
  - Bonus AD ratio reduced to 125% *bonus AD from 170%.
  - Monster damage cap bonus AD ratio reduced to 125% *bonus AD from 150%.
    - *Base monster damage cap unchanged.*

### V14.15
- General
  - **Bug Fixes:** Corrected multiple triggers for various voicelines. This also resolves enemies being able to hear VO that is normally only meant for the player.

### V14.1
- *Terrashape*
  - **Bug Fixes:** *Terrashape* into *Edge of Ixtal* fast buffer no longer sends *Edge of Ixtal* towards the 0,0,0 map coordinate.
- *Supreme Display of Talent*
  - Knockback duration increased slightly to reduce chances of it not overlapping with wall stun.

### V13.24
- Stats
  - Base mana increased to 375 from 320.
  - Mana growth increased to 60 from 50.

### V13.5
- *Edge of Ixtal*
  - Base damage increased to $50-190$ from $50-170$.
- *Audacity*
  - Cooldown reduced to $11-7$ seconds from $12-8$.

### V13.4
- *Royal Privilege*
  - **Bug Fixes:** On-target cooldown reset now behaves more consistently when casting *Elemental Wrath*, *Terrashape*, and *Elemental Wrath* again in quick succession.

### V12.15
- *Royal Privilege*
  - Bonus AD ratio reduced to 30% *bonus AD from 45%.
  - **Bug Fixes:** No longer applies an extra proc of *Royal Privilege* with *First Strike* equipped.

## Trivia

- While **Quinn** was the first champion whose name starts with Q, the then only remaining unused letter in the alphabet regarding champion names prior to her release, Qiyana's release made it so there were at least 2 champions starting with each letter of the alphabet.
- Qiyana Dab (dance) if she uses any emote within 10 seconds of a takedown.
- Qiyana is the strongest among the 10 sisters and this might be referring to '1v9,’ a common player term used when both having bad teammates and enemies.

---
*This page was automatically generated from League of Legends Wiki data.*