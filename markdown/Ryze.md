# Ryze

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
| **Champion** | Ryze |
| **Title** | the Rune Mage |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Latest Changes** | V25.13 |
| **Roles** | Battlemage |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 2400 |
| **Riot Points** | 260 |
| **Difficulty** | 3 |
| **Hero Type** | Mage |
| **Alt Type** | Fighter |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 2 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $645.0$ | $+124.0$ |
| **Mana** | $300.0$ | $+70.0$ |
| **Health Regen** | $8.0$ | $+0.8$ |
| **Mana Regen** | $8.0$ | $+1.0$ |
| **Armor** | $22.0$ | $+4.2$ |
| **Magic Resist** | $32.0$ | $+1.3$ |
| **Attack Damage** | $58.0$ | $+3.0$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.1\%$ | |
| **Missile Speed** | $2400$ units/second | |
| **Acquisition Radius** | $575$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $150$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Arcane Mastery

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Ryze** increases his (mana) **maximum** mana by (10% per 100 AP).

**Notes:**

- The increase in maximum mana differenciates between **base** and **bonus** mana, similar to Overgrowth. The product base off of **base** mana does *not* count as bonus mana.
- With Rabadon's Deathcap, a total of (40% AP) ability power and (14% AP)% **maximum** mana is gained.
- *Arcane Mastery* together with Archangel's Staff or Seraph's Embrace creates a recursive stat loop: ability power giving mana and so on. *Another way to create a recursive stat loop, is to have any converter of mana to health (Winter's Approach, Fimbulwinter or the U.R.F. buff) and Riftmaker. The first converter giving mana to health, Riftmaker giving health to ability power and *Arcane Mastery* giving ability power to mana.

---

### Q: Overload

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1000 units |
| **Width** | 110 units |
| **Speed** | 1700 units/second |
| **Cost** | 40 / 38 / 36 / 34 / 32 Mana |
| **Cooldown** | 5 seconds |
| **Queue Time** | $0.35$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |

**PASSIVE:** **Ryze**’s other basic ability casts reset *Overload*’s cooldown and charge a *Rune* stack for 4 seconds, refreshing on subsequent casts and stacking up to 2 times.

**ACTIVE:** **Ryze** unleashes a runic blast in the target direction that deals magic damage to the first enemy hit and consumes all *Rune* stacks.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 75 / 95 / 115 / 135 / 155 (+ 55% AP) (+ 2% **bonus** mana) |

If **Ryze** consumed 2 stacks, he gains (ms) **bonus** movement speed for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 28 / 32 / 36 / 40 / 44% |

**FLUX BONUS:** *Overload* deals 20px increased damage and spreads to surrounding *Fluxed* enemies.

**Notes:**

- *Overload* can be buffered $0.5$ seconds before it comes off cooldown.
- *Overload* can be buffered while casting either Spell Flux or Rune Prison to cast immediately after the previous spell's cast time.
- Applies spell damage to the primary target and area damage to secondary targets affected by Flux. - This ability will cast from wherever the caster is at the start of the cast time.

---

### W: Rune Prison

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 550 units |
| **Cost** | 50 / 60 / 70 / 80 / 90 Mana |
| **Cooldown** | 11 / 10.5 / 10 / 9.5 / 9 seconds |
| **Queue Time** | $0.35$ seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Call For Help** | True |

**ACTIVE:** **Ryze** seizes the target enemy, dealing magic damage and slowing them by 50% for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 90 / 120 / 150 / 180 (+ 70% AP) (+ 4% **bonus** mana) |

**FLUX BONUS:** The target is rooted instead of slowed.

**Notes:**

- *Rune Prison* can be buffered $0.5$ seconds before it comes off cooldown.

---

### E: Spell Flux

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 550 units |
| **Effect Radius** | 350 (Bounce range) units |
| **Speed** | 4000 (Cast missile) / 1500 (Spreading bounce missiles) units/second |
| **Cost** | 35 / 45 / 55 / 65 / 75 Mana |
| **Cooldown** | 3.5 / 3.25 / 3 / 2.75 / 2.5 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | Special |
| **Call For Help** | True |

**ACTIVE:** **Ryze** projects an orb upon the target enemy that deals magic damage.


The target and surrounding enemies are also marked with *Flux* for 4 seconds. **Ryze**’s basic abilities against *Flux* targets consume the mark to become empowered with an additional effect.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 90 / 120 / 150 / 180 (+ 50% AP) (+ 2% **bonus** mana) |

**FLUX BONUS:** *Spell Flux* spreads farther.

**Notes:**

- *Spell Flux*’s second cast on a target will apply a new *Flux* debuff as well as triggering the previous one (effectively refreshing the duration).
- The "main" (initial cast) missile of *Spell Flux* is blocked by projectile-intercepting effects, while the spreading bounce missiles are not.
- bounce range er to-edge or cr to-center?
  - Game data notes additional range checks of 400 (bounce range) and 500 (Q damage bounce range) against 'large' enemies.
- *Spell Flux* can be buffered $0.5$ seconds before it comes off cooldown.

---

### R: Realm Warp

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 3000 (Maximum range) / 1000 (Minimum range) units |
| **Effect Radius** | cr 250 (Definitive portal entry radius for all units) – 350 (Inner portal entry radius for champion player forgiveness) – 465 (Outer portal radius for all units) / sight 515 (Destination vision radius during channel) |
| **Cost** | 100 Mana |
| **Cooldown** | 180 / 170 / 160 / 150 / 140 seconds |
| **Targeting** | Location |
| **Affects** | Allies |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Silence** | True |

**PASSIVE:** *Overload’s* *Flux* empowered damage is increased.

| Attribute | Value |
|-----------|------:|
| **Bonus Overload Damage** | 50 / 62.5 / 75 / 87.5 / 100% |

**ACTIVE:** **Ryze** channels for 2 seconds to open a portal beneath him, marking the target location as its destination and granting sight of its area. **Ryze** is able to act while channeling, but the portal will cancel if he is interrupted.

Upon completion, **Ryze** and allied units within (See notes) will blink to the location and become rooted, disarmed, silenced and untargetable for $0.75$ (Estimated) seconds.

**Notes:**

- *Realm Warp*’s portal may try to predict allied players' intention via their movement. For this purpose, it uses three different radiuses around its center for registering units: a definitive radius of 250 units, an inner radius of 365 units and an outer radius of 465 units.
  - **Ryze** himself will always be teleported so long as he is within the inner radius, regardless of movement.
  - Non-champion units within the outer radius will always be teleported, regardless of movement.
  - Champions within the definitive radius will always be teleported, regardless of movement.
  - Champions within the inner radius but outside of the definitive radius must not be currently moving in order to be teleported.
  - Champions within the inner radius but outside of the definitive radius that are currently moving will not be teleported if their destination is outside the inner radius.
  - Champions within the outer radius but outside of the inner radius that are currently moving will only be teleported if their destination is within the inner radius or their movement pathing otherwise intersects the inner radius' circle at two points.
    - This detection may occasionally fail when pathing around terrain.
- Blinked units retain the same relative positions to one another upon arrival.
- *Realm Warp* will teleport allies even if they are in a zombie state, are untargetable, or are affected by crowd control.
- *Realm Warp* will not teleport allies that are using an ability that preloads UnstoppableForceMarker or are being affected by another ally's *Realm Warp* first.
- *Realm Warp* will still teleport an allied Rift Herald that is winding up its leap.
- *Realm Warp* has various interactions with channeled abilities that are being performed by an ally. Some channels will prevent their caster from being teleported and others will not.
  - For channels that do not prevent the teleport, *Realm Warp* will specifically not apply its silence to the caster.
  - Cast times will not prevent the teleport.
  - The following channels will prevent the teleport:
    - Caitlyn’s R
    - Galio’s R
    - Kayn’s R
    - Jhin’s R
    - Warwick’s Q
    - Xerath’s R
    - Recall / Empowered Recall
    - Teleport / Unleashed Teleport
- The following table refers for interactions while **Ryze** is channeling:

#### Channel Behavior (channel)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Allowed |
| **Movement** | Allowed |
| **Abilities** | Allowed |
| **Items** | Allowed / Interrupts |
| **Summoner Spells** | Allowed |
| **Interrupted by** | silence,root,revival-false, ground |

---

## Patch History

### V25.13
- Rune Prison
  - Base damage reduced to 60 / 90 / 120 / 150 / 180 from 70 / 100 / 130 / 160 / 190.
  - Mana cost changed to 50 / 60 / 70 / 80 / 90 from 40 / 55 / 70 / 85 / 100.

### V25.11
- Overload
  - Spell Flux bonus damage increased to pp|type

### File:Realm Warp.png|20px|border
- Rune Prison
  - Base damage reduced to 70 / 100 / 130 / 160 / 190 from 80 / 110 / 140 / 170 / 200.

### V14.21
- Overload
  - **Bug Fixes:** Now is able to trigger Horizon Focus Hypershot.

### V14.20
- Overload
  - **Bug Fixes:** Spell shields now correctly block its damage.
- Spell Flux
  - **Bug Fixes:** Spell shields now correctly block its damage.

### V14.17
- Overload
  - Base damage increased to 75 / 95 / 115 / 135 / 155 from 70 / 90 / 110 / 130 / 150.
- Realm Warp
  - Cooldown reduced to 180 / 160 / 140 seconds from 210 / 180 / 150.

### V14.8
- Rune Prison
  - Slow increased to 50% from 35%.
  - Cooldown reduced to 11 / 10.5 / 10 / 9.5 / 9 seconds from 13 / 12 / 11 / 10 / 9.

### V14.7
- Realm Warp
  - **Bug Fixes:** Ground VFX now properly layers over terrain that is not at default elevation. Previously, the VFX could be completely hidden if the VFX coincided entirely with those sections of the map.

### V13.22
- Stats
  - Base attack speed increased to $0.658$ from $0.625$.

### V13.20
- Ryze
  - Skin renamed to *Worlds 2019 Ryze* from *Championship Ryze*.

## Trivia

- His name comes from Riot Games's CEO Brandon 'Ryze' Beck's alias.
- Ryze was one of the original playable characters in the the Summoner's Rift Battle Training tutorial, along with Ashe and Garen.
- Ryze was the first champion to have 9 skins.
  - He was also the first to have 2 Harrowing ones (Ryze in 2010, Ryze in 2012).
    - The other champion is Katarina (Katarina and Katarina). Hecarim has 2 Harrowing skins but his Hecarim skin did not come in Harrowing event.
- His giant scroll can be seen behind an overturned chair in the trailer for the game's Mac version.
  - While the scroll is supposedly indestructible Nocturne it apart it in 'A Twist of Fate'.
    - Yet later on the scroll is whole again but whether it is capable of self-repairing is uncertain.
- Ryze was deemed as having next to no counterplay in Ultra Rapid Fire (2014 edition) and was disabled in non-custom games.

---
*This page was automatically generated from League of Legends Wiki data.*