# Kassadin

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
| **Champion** | Kassadin |
| **Title** | the Void Walker |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-08-07 |
| **Release Patch** | V0.9.22.7 |
| **Latest Changes** | V25.17 |
| **Roles** | Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Assassin |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 1 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $646.0$ | $+119.0$ |
| **Mana** | $400.0$ | $+87.0$ |
| **Health Regen** | $6.0$ | $+0.5$ |
| **Mana Regen** | $6.0$ | $+0.8$ |
| **Armor** | $21.0$ | $+4.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $59.0$ | $+3.9$ |
| **Attack Speed** | $0.640$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $150.0$ | $+0.0$ |
| **Base Attack Speed** | $0.64$ | |
| **Attack Speed Ratio** | $0.64$ | |
| **Bonus AS per Level** | $3.7\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $165$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Void Stone

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Kassadin** is permanently ghosted and takes 10% reduced magic damage.

**Notes:**

- No additional details.

---

### Q: Null Sphere

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 650 units |
| **Speed** | 1400 units/second |
| **Cost** | 60 / 65 / 70 / 75 / 80 Mana |
| **Cooldown** | 10 / 9.5 / 9 / 8.5 / 8 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**ACTIVE:** **Kassadin** fires an orb of void energy at the target enemy that deals magic damage and disrupts their ongoing channels.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 65 / 95 / 125 / 155 / 185 (+ 70% AP) |

He also gains a shield that absorbs magic damage for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Shield Strength** | 80 / 110 / 140 / 170 / 200 (+ 30% AP) |

**Notes:**

- The disrupt is 'wrapped' into a status effect that says the target is Silenced for $0.25$ seconds, but it does not actually *silence*. It however makes sure that the *disrupt* is prevented by immunity to silences.
- The shield is granted at the start of the cast time.

---

### W: Nether Blade

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 1 Mana |
| **Cooldown** | 7 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Parry** | Special |

**PASSIVE:** **Kassadin**’s basic attacks deal 20 (+ 10% AP) **bonus** magic damage on-hit.

**ACTIVE:** **Kassadin** empowers his next basic attack within 5 seconds to have an uncancellable windup, gain range, deal increased **bonus** magic damage, and restore mana, with the restoration amount quintupled against champions.

| Attribute | Value |
|-----------|------:|
| **Increased Bonus Magic Damage** | 50 / 75 / 100 / 125 / 150 (+ 80% AP) |

| Attribute | Value |
|-----------|------:|
| **Mana Restored** | 4 / 4.5 / 5 / 5.5 / 6% of **missing** mana |
| **Mana Restored Against Champions** | 20 / 22.5 / 25 / 27.5 / 30% of **missing** mana |

*Nether Blade resets **Kassadin**’s basic attack timer.*

**Notes:**

- The passive damage applies proc damage and the active damage applies spell damage.
- The enhanced attack will apply other on-hit effects and can critically strike as normal.
- *Nether Blade* will not grant mana if the attack is dodged or if it misses, but will do so if the attack is blocked. In all cases the damage is parried.
- Spell shield will block the *active* damage but not the *passive* one.
- The passive **bonus** damage applies to structures.
- The empowered attack will trigger but not be consumed nor apply its effects against structures.

---

### E: Force Pulse

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 600 / 1800 (Detection Radius) units |
| **Angle** | er 78° |
| **Cost** | 60 / 65 / 70 / 75 / 80 Mana |
| **Cooldown** | 21 / 20 / 19 / 18 / 17 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**PASSIVE:** Each time **Kassadin** or a nearby champion casts an ability, *Force Pulse*’s **current** cooldown is reduced by $0.75$ seconds.

**ACTIVE:** **Kassadin** emits a pulse of void energy in a cone in the target direction that deals magic damage to enemies hit and slows them for 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 100 / 130 / 160 / 190 (+ 70% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 50 / 60 / 70 / 80 / 90% |

**Notes:**

- Untargetable champions do not reduce *Force Pulse*’s cooldown upon ability activations. *Toggle abilities and transformation abilities do not count as ability activations and **will not** reduce *Force Pulse*’s cooldown. Effect at cast time end

---

### R: Riftwalk

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 500 (Maximum range, has no minimum range) units |
| **Effect Radius** | 270 units |
| **Cost** | 40×2^(x-1) 2 ^ Stacks Mana |
| **Cooldown** | 5 / 4.25 / 3.5 / 2.75 / 2 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |

**ACTIVE:** **Kassadin** blinks toward the target location, dealing magic damage to all nearby enemies upon arrival. He then gains a stack of *Riftwalk* for 15 seconds, refreshing on subsequent casts and stacking up to 4 times.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 80 / 90 / 100 / 110 (+ 50% AP) (+ 2% **maximum** mana) |

**RIFTWALK:** For each stack, *Riftwalk* deals **bonus** magic damage at an increased mana cost.

| Attribute | Value |
|-----------|------:|
| **Bonus Damage Per Stack** | 35 / 40 / 45 / 50 / 55 (+ 7% AP) (+ 1% **maximum** mana) |
| **Maximum Bonus Damage** | 140 / 160 / 180 / 200 / 220 (+ 28% AP) (+ 4% **maximum** mana) |
| **Maximum Magic Damage** | 210 / 240 / 270 / 300 / 330 (+ 78% AP) (+ 6% **maximum** mana) |

**Notes:**

- Flash can be used during the cast time, allowing **Kassadin** to blink further away.

---

## Patch History

### V25.17
- Null Sphere
  - AP ratio increased to 70% AP from 60% AP.
- Force Pulse
  - AP ratio increased to 70% AP from 65% AP.

### V25.11
- Stats
  - Base armor increased to 21 from 19.
- Force Pulse
  - Base damage increased to 70 / 100 / 130 / 160 / 190 from 60 / 90 / 120 / 150 / 180.
- Riftwalk
  - AP ratio per stack reduced to 7% AP from 10% AP.
    - Maximum AP ratio reduced to 78% AP from 90% AP.

### V14.24
- Riftwalk
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.22
- Null Sphere
  - **Bug Fixes:** Now correctly interrupts the following channels:
    - Glacial Storm
    - Royal Maelstrom
    - The Culling
    - Realm Warp
    - Inferno Trigger
    - Stand United
    - Ixtal's Impact.
    - *This includes their hijacked version.*
  - **Bug Fixes:** Now properly interrupts Hero's Entrance, instead of causing it to skip its channel's initial duration before the untargetability is granted.
  - **Bug Fixes:** No longer interrupts channels while the target is under the effects of crowd control immunity.
  - **Bug Fixes:** No longer interrupts Nether Grasp while his Void Shift is available.
  - **Bug Fixes:** When interrupting a champion's channel, no longer counts progress twice toward the "404's" (Champion channels interrupted) Eternal.

### V14.21
- Null Sphere
  - AP ratio reduced to 60% AP from 70% AP.
- Force Pulse
  - AP ratio reduced to 65% AP from 70% AP.

### V14.19
- Null Sphere
  - **New Effect:** No longer cancels from losing vision on the target.

### V14.10
- Null Sphere
  - **Bug Fixes:** No longer interrupts Unstoppable Onslaught.

### V14.9
- Force Pulse
  - AP ratio reduced to 70% AP from 80% AP.

### V14.2
- Kassadin
  - **Bug Fixes:** Parallax textures have been restored.

### V13.22
- Force Pulse
  - AP ratio reduced to 80% AP from 85% AP.
- Riftwalk
  - AP ratio reduced to 50% AP from 60% AP.

## Trivia

- Kassadin's dance references the Sprinkler dance dance.
  - A side-by-side comparison can be seen here.
- Kassadin's breathing might be referencing from Star Wars.
- Kassadin has been disabled for every Ultra Rapid Fire edition except for Snow Battle ARURF. This is most likely because he would have been (and be) able to Riftwalk the entirety of Summoner's&nbsp;Rift in seconds.
- Kassadin was named after Jeff 'Kassadin' Jew, the Lead Producer of Legends of Runeterra.
  - Ezreal, Ryze and Tryndamere are also named after Rioters' nicknames.
- Kassadin's Series 1 Eternals make the following references:
  - *404's* is a reference to computer error HTTP 404, because the information was most likely deleted when searching the server.
- Kassadin's Series 2 Eternals make the following references:
  - *Kassassin* is a word game of "Assassin" with "Kassadin".

---
*This page was automatically generated from League of Legends Wiki data.*