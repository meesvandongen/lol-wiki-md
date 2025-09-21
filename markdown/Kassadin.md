# Kassadin

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
| **Champion** | Kassadin |
| **Title** | the Void Walker |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-08-07 |
| **Release Patch** | V0.9.22.7 |
| **Roles** | Assassin |
| **Riot Positions** | Middle |
| **External Positions** | Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $646.0$ | $+119.0$ | $2669.0$ |
| **Mana** | $400.0$ | $+87.0$ | $1879.0$ |
| **Health Regen** | $6.0$ | $+0.5$ | $14.5$ |
| **Mana Regen** | $6.0$ | $+0.8$ | $19.6$ |
| **Armor** | $21.0$ | $+4.0$ | $89.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $59.0$ | $+3.9$ | $125.3$ |
| **Attack Speed** | $0.640$ | $+3.7\%$ | $1.043$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $150.0$ | $+0.0$ | $150.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.64$ |
| **Attack Speed Ratio** | $0.64$ |
| **Bonus AS per Level** | $3.7\%$ |
| **Acquisition Radius** | $400 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $165 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Void Stone

**Innate:** **Kassadin** is permanently ghosted and takes reduced magic damage.

**Innate:** **Kassadin** is permanently ghosted and takes 10% reduced magic damage.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- No additional details.

---

### Q: Null Sphere

**Active:** **Kassadin** fires an orb of void energy at the target enemy that deals magic damage and disrupt their ongoing channel.

*He then gains a brief shield that absorbs magic damage.*

**Active:** **Kassadin** fires an orb of void energy at the target enemy that deals magic damage and disrupt their ongoing channel. He also gains a shield that absorbs magic damage for $1.5$ seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 650 units |
| **Cooldown** | $10-8$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-80$ Mana |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Speed** | 1400 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $65-185$ (+ 70% AP)
- **Magic Shield Strength:** $80-200$ (+ 30% AP)

**Notes:**

- The disrupt is 'wrapped' into a status effect that says the target is Silence for $0.25$ seconds, but it does not actually *silence*. It however makes sure that the *disrupt* is prevented by Cc-immune.
- The shield is granted at the start of the cast time.

---

### W: Nether Blade

**Passive:** **Kassadin**’s basic attacks deal **bonus** magic damage.

**Active:** **Kassadin** empowers his blade, causing his next basic attack within a few seconds to gain *range **bonus** range*, deal increased **bonus** magic damage, and restore *mana*, with the restoration amount quintupled against champion.

**Passive:** ''Kassadin's** basic attacks deal 20 (+ 10% AP) **bonus'' magic damage on-hit. **Active:** **Kassadin** empowers his next basic attack within 5 seconds to have an uncancellable windup, gain range*bonus** range*, deal increased **bonus** magic damage, and restore *mana*, with the restoration amount quintupled against champion. *Nether Blade basic attack reset *'Kassadin's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 7 seconds |
| **Cast Time** | none |
| **Cost** | 1 Mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |

**Scaling:**
- **Increased Bonus Magic Damage:** $50-150$ (+ 80% AP)
- **Mana Restored:** $4-6$% of **missing* mana5-6×5$% of
- **missing** mana

**Notes:**

- The passive damage applies proc damage and the active damage applies spell damage.
- The enhanced attack will apply other on-hit effects and can critical strike as normal.
- *Nether Blade* will not grant mana if the attack is dodge or if it blind, but will do so if the attack is block. In all cases the damage is parried.
- Spell shield will block the *active* damage but not the *passive* one.
- The passive **bonus** damage applies to structures.
- The empowered attack will trigger but not be consumed nor apply its effects against structures.

---

### E: Force Pulse

**Passive:** Ability casts reduce the cooldown of this ability.

**Active:** **Kassadin** emits a pulse of void energy in a cone in the target direction that deals magic damage to enemies and briefly slow them.

**Passive:** Each time **Kassadin** or a nearby champion casts an ability, 'Force Pulse's **current cooldown** is reduced by $0.75$ seconds. **Active:** **Kassadin** emits a pulse of void energy in a cone in the target direction that deals magic damage to enemies hit and slow them for 1 second.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $21-17$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-80$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 600 / 1800 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** $70-190$ (+ 70% AP)
- **Slow:** $50-90$%

**Notes:**

- Untargetable champions do not reduce 'Force Pulse's cooldown upon ability activations. *Toggle abilities and transformation abilities do not count as ability activations and **will not** reduce 'Force Pulse's cooldown. Effect at cast time end

---

### R: Riftwalk

**Active:** **Kassadin** blinks toward the target location, dealing magic damage to all nearby enemies upon arrival. He then gains a stack of *Riftwalk*, stacking up to a cap.

**Active:** **Kassadin** blinks toward the target location, dealing magic damage to all nearby enemies upon arrival. He then gains a stack of *Riftwalk* for 15 seconds, refreshing on subsequent casts and stacking up to 4 times. **Riftwalk:** For each stack, *Riftwalk* deals **bonus** magic damage at an increased *mana cost*.

| Attribute | Value |
|-----------|-------|
| **Range** | 500 units |
| **Cooldown** | $5-2$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 40×2^(x-1) 2 ^ Stacks Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 270 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |

**Scaling:**
- **Magic Damage:** $70-110$ (+ 50% AP)
- **maximum** mana)
- **Bonus Damage Per Stack:** $35-55$ (+ 7% AP) (+ 1% **maximum* mana)4-55×4$ (+ $7×4$% AP) (+ 4% **maximum* mana)4-110+55×4$ (+ $50+7×4$% AP) (+ 6%
- **maximum** mana)

**Notes:**

- Flash can be used during the cast time, allowing **Kassadin** to blink further away.

---

## Patch History

### V25.17
- *Null Sphere*
  - AP ratio increased to 70% AP from 60% AP.
- *Force Pulse*
  - AP ratio increased to 70% AP from 65% AP.

### V25.11
- Stats
  - Base armor increased to 21 from 19.
- *Force Pulse*
  - Base damage increased to $70-190$ from $60-180$.
- *Riftwalk*
  - AP ratio per stack reduced to 7% AP from 10% AP.
    - Maximum AP ratio reduced to $50+7×4$% AP from $50+10×4$% AP.

### V14.24
- *Riftwalk*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.22
- *Null Sphere*
  - **Bug Fixes:** Now correctly interrupts the following channels:
    - Glacial Storm
    - Royal Maelstrom
    - The Culling
    - Realm Warp
    - Inferno Trigger
    - Stand United
    - Ixtal's Impact.
    - *This includes their *hijacked* version.*
  - **Bug Fixes:** Now properly interrupts Hero's Entrance, instead of causing it to skip its channel's initial duration before the untargetability is granted.
  - **Bug Fixes:** No longer interrupts channels while the target is under the effects of crowd control immunity.
  - **Bug Fixes:** No longer interrupts Nether Grasp while his *Void Shift* is available.
  - **Bug Fixes:** When interrupting a champion's channel, no longer counts progress twice toward the "404's" (Champion channels interrupted) Eternal.

### V14.21
- *Null Sphere*
  - AP ratio reduced to 60% AP from 70% AP.
- *Force Pulse*
  - AP ratio reduced to 65% AP from 70% AP.

### V14.19
- *Null Sphere*
  - **New Effect:** No longer cancels from losing vision on the target.

### V14.10
- *Null Sphere*
  - **Bug Fixes:** No longer interrupts Unstoppable Onslaught.

### V14.9
- *Force Pulse*
  - AP ratio reduced to 70% AP from 80% AP.

### V14.2
- Kassadin
  - **Bug Fixes:** Parallax textures have been restored.

### V13.22
- *Force Pulse*
  - AP ratio reduced to 80% AP from 85% AP.
- *Riftwalk*
  - AP ratio reduced to 50% AP from 60% AP.

## Trivia

- Kassadin's dance references the Sprinkler dance dance.
  - A side-by-side comparison can be seen here.
- Kassadin's breathing might be referencing from Star Wars.
- Kassadin has been disabled for every Ultra Rapid Fire edition except for Snow Battle ARURF. This is most likely because he would have been (and be) able to *Riftwalk* the entirety of Summoner's&nbsp;Rift in seconds.
- Kassadin was named after Jeff 'Kassadin' Jew, the Lead Producer of Legends of Runeterra.
  - **Ezreal**, **Ryze** and **Tryndamere** are also named after Rioters' nicknames.
- Kassadin's Series 1 Eternals make the following references:
  - '404's is a reference to computer error HTTP 404, because the information was most likely deleted when searching the server.
- Kassadin's Series 2 Eternals make the following references:
  - *Kassassin* is a word game of "Assassin" with "**Kassadin**".

---
*This page was automatically generated from League of Legends Wiki data.*