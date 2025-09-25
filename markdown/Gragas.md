# Gragas

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
| **Champion** | Gragas |
| **Title** | the Rabble Rouser |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2010-02-02 |
| **Release Patch** | V1.0.0.72 |
| **Latest Changes** | V25.09 |
| **Roles** | Vanguard |
| **Riot Positions** | Jungle |
| **External Positions** | Top, Jungle, Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $640.0$ | $+115.0$ |
| **Mana** | $400.0$ | $+47.0$ |
| **Health Regen** | $5.5$ | $+0.5$ |
| **Mana Regen** | $6.0$ | $+0.8$ |
| **Armor** | $38.0$ | $+5.0$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $64.0$ | $+3.5$ |
| **Attack Speed** | $0.675$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.675$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $50$ units | |
| **Selection Radius** | $140$ units | |
| **Selection Height** | $185$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Happy Hour

| Attribute | Value |
|-----------|------:|
| **Static Cooldown** | 12 to 6 |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** Periodically, after casting an ability, **Gragas** heals himself for (health) $5.5$% of his **maximum** health.

**Notes:**

- No additional details.

---

### Q: Barrel Roll

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 850 units |
| **Effect Radius** | 250 units |
| **Speed** | 1000 units/second |
| **Cost** | 80 Mana |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Location / Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |
| **Out of Range Behavior** | Walk in range of the target location to cast (first cast) |

**ACTIVE:** **Gragas** rolls a cask to the target location, remaining there for 4 seconds and granting sight of the area. Upon arrival, the cask starts to ferment over the first 2 seconds of its duration to increase its damage and the effectiveness of its slow, up to a maximum of 150% of their initial values. *Barrel Roll* can be recast at any time within its duration after the cask has fully travelled, and does so automatically after its duration ends.

**RECAST:** **Gragas** detonates the cask, dealing magic damage to nearby enemies, reduced by 30% against minions, and slowing them for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Minimum Magic Damage** | 80 / 120 / 160 / 200 / 240 (+ 80% AP) |
| **Maximum Magic Damage** | 120 / 180 / 240 / 300 / 360 (+ 120% AP) |

| Attribute | Value |
|-----------|------:|
| **Minimum Minion Damage** | 56 / 84 / 112 / 140 / 168 (+ 56% AP) |
| **Maximum Minion Damage** | 84 / 126 / 168 / 210 / 252 (+ 84% AP) |

| Attribute | Value |
|-----------|------:|
| **Minimum Slow** | 40 / 45 / 50 / 55 / 60% |
| **Maximum Slow** | 60 / 67.5 / 75 / 82.5 / 90% |

**Notes:**

*The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive. **Deactivating the ability manually does not.
- The cask ignores unit and terrain collision therefore it will only stop once it reaches the target location.
  - On arrival the cask will begin to glow ever brighter as it ferments until it turns bright red.
- *Barrel Roll* has an internal cooldown while traveling so that **Gragas** may only make it explode on arrival.
- The charge-up countdown will only start once the cask stops rolling. - This ability will cast from wherever the caster is at the end of the cast time.
- The ability will not preserve the caster's facing direction when using Flash and similar effects.

---

### W: Drunken Rage

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 250 units |
| **Cost** | 30 Mana |
| **Cooldown** | 5 (Starts post-effect) seconds |
| **Targeting** | Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Parry** | True |
| **Silence** | True |

**ACTIVE:** **Gragas** channels for $0.75$ seconds, drinking out of his brew, and gains damage reduction for $2.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Damage Reduction** | 10 / 12 / 14 / 16 / 18% (+ 4% per 100 AP) |

Upon completing the channel, **Gragas** empowers his next basic attack within 5 seconds to have an uncancellable windup, gain (range) 50 **bonus** range and deal **bonus** magic damage to the target and nearby enemies, reduced to 50% against structures and capped at 300 against monsters.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 20 / 50 / 80 / 110 / 140 (+ 7% of target's **maximum** health) (+ 70% AP) |

| Attribute | Value |
|-----------|------:|
| **Capped Monster Damage** | 320 / 350 / 380 / 410 / 440 (+ 70% AP) |

**Notes:**

- The enhanced attack will apply other on-hit effects and can critically strike as normal.
- The empowered attack will trigger against structures.
- The empowered attack will trigger against wards but not be consumed nor apply its effects against wards.
- The following table refers for interactions while **Gragas** is channeling:

#### Channel Behavior (channel)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Allowed |
| **Abilities** | Disabled |
| **Items** | Disabled / Allowed |
| **Summoner Spells** | Allowed / Disabled |

---

### E: Body Slam

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 600 units |
| **Effect Radius** | cr 180 (Center-to-center collision radius.) / er 180 (Center-to-edge collision lollipop. This lollipop it located 30 units in front of his dash end location.) |
| **Speed** | 910 units/second |
| **Cost** | 50 Mana |
| **Cooldown** | 14 / 13.5 / 13 / 12.5 / 12 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Gragas** charges in the target direction and stops upon colliding with an enemy, dealing magic damage to all nearby enemies, knocking them back, though not through terrain, and stunning them for 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 125 / 170 / 215 / 260 (+ 60% AP) |

*Body Slam*’s **current** cooldown is reduced by 40% if **Gragas** hits an enemy.

*Barrel Roll and Explosive Cask can be cast during the dash.*

**Notes:**

- Flash can be casted during *Body Slam*, instantly ending it and affecting enemies at the flash location.
- Units hit by *Body Slam* turn away from **Gragas**.
- Displacement immunity will not resist the application of the stun.

---

### R: Explosive Cask

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1000 units |
| **Effect Radius** | 400 units |
| **Cost** | 100 Mana |
| **Cooldown** | 100 / 92.5 / 85 / 77.5 / 70 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Gragas** hurls a cask that travels to the target location over $0.5$ seconds, exploding on impact to deal magic damage to all enemies within the area and knock them back 900 units, though not through terrain, from the epicenter of the explosion, as well as granting sight of the area for 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 200 / 250 / 300 / 350 / 400 (+ 80% AP) |

**Notes:**

- This ability will cast from wherever the caster is at the end of the cast time.

---

## Patch History

### V25.09
- Drunken Rage
  - Tooltip updated to remove mentions of becoming empowered by drinking alcohol in compliance with age rating systems.

### V14.19
- Drunken Rage
  - **Bug Fixes:** SFX is no longer audible through the Fog of War.

### V14.13
- Happy Hour
  - Heal reduced to $5.5$% **maximum** health from $6.5$%.

### V14.9
- Stats
  - Selection radius reduced to 140 units from 155.

### V14.6
- Stats
  - Base health reduced to 640 from 670.
- Barrel Roll
  - Mana cost increased to 80 at all ranks from 80 / 75 / 70 / 65 / 60.
- Gragas
  - Drunken Rage
    - **Bug Fixes:** No longer overrides and cancels other abilities' animations.
- Gragas
  - Drunken Rage
    - **Bug Fixes:** Restored the overlay.

### V14.2
- Stats
  - Health growth increased to 115 from 109.
  - Armor growth increased to 5 from $4.8$.
- Drunken Rage
  - **New Effect:** Empowered attack is now consumed against structures for 50% damage.
  - **New Effect:** Empowered attack's area of effect can now target structures.
- Explosive Cask
  - Cooldown reduced to 100 / 85 / 70 from 120 / 100 / 80.

### V13.24
- Happy Hour
  - Cooldown reduced to 12 to 6 seconds from 12 at all levels.

### V13.22
- Body Slam
  - Cooldown refund changed to 40% of total cooldown from 3 seconds.
- Explosive Cask
  - Travel time reduced to $0.5$ seconds from $0.55$.

### V13.12
- Happy Hour
  - Cooldown increased to 12 seconds from 8.
- Barrel Roll
  - Mana cost reduced to 80 / 75 / 70 / 65 / 60 from 80 at all ranks.

### V13.3
- Explosive Cask
  - **Bug Fixes:** Effect is now fully nullified if the missile collides with projectile-interception effects, rather than triggering at the location of impact.

## Trivia

- Gragas is voiced.md) by J.S. Gilbert.
  - Cho'Gath, pre-rework Dr. Mundo, pre-rework Sion and pre-rework Udyr are also voiced by the same voice actor.
- Gragas used to be the only Mage/Fighter champion as well as the only melee mage.
  - He is now a Fighter/Mage so until recently there have been no melee mages, with Sylas now taking up that role. (even though most champions with the Mage secondary role are melee).
- His first title was 'the Cask Master' but was changed to avoid confusion with Twisted Fate.
- What appears to be Gragas' cask can be seen in the game's Mac Version trailer.
- Gragas is the third champion to have 9 skins, behind Ryze and Annie.

---
*This page was automatically generated from League of Legends Wiki data.*