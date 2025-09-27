# Irelia

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
| **Champion** | Irelia |
| **Title** | the Blade Dancer |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2010-11-16 |
| **Release Patch** | V1.0.0.105 |
| **Latest Changes** | V25.17 |
| **Roles** | Diver |
| **Riot Positions** | Top, Middle |
| **External Positions** | Top, Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 2 |
| **Style** | 30 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $630.0$ | $+115.0$ |
| **Mana** | $350.0$ | $+50.0$ |
| **Health Regen** | $3.5$ | $+0.85$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $36.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+2.05$ |
| **Attack Damage** | $65.0$ | $+3.5$ |
| **Attack Speed** | $0.656$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $200.0$ | $+0.0$ |
| **Base Attack Speed** | $0.656$ | |
| **Attack Speed Ratio** | $0.656$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Attack Windup** | $19.7\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $120$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Ionian Fervor

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Effects** | Proc |
| **Parry** | unknown |

**INNATE:** **Irelia** generates a stack of *Ionian Fervor* for each enemy champion hit by her abilities, or if she hits at least one non-champion, lasting for 6 seconds, refreshing on basic attacks and ability hits against enemy champions, large monsters, and structures, and stacking up to 4 times.

**IONIAN FERVOR:** For each stack, **Irelia** gains 10 to 25 **bonus** attack speed, up to a maximum of 10×4 to 25×4. At maximum stacks, **Irelia**’s basic attacks are empowered to deal 10 to 61 (+ 20% **bonus** AD) **bonus** magic damage on-hit, reduced to「 50% ⟷ 10×0.5 to 61×0.5 (+ 10% **bonus** AD) 」against structures.

**INNATE - UNSTEADY:** Enemy champions and large monsters hit by *Flawless Duet* and *Vanguard's Edge’s* initial barrage are marked as *Unsteady* for 5 seconds. The mark can be consumed by *Bladesurge*.

**Notes:**

- : Parry interactions (block, dodge, and blind).
- *Ionian Fervor* stacks are represented by a counter under **Irelia**’s health bar. It is highlighted while the ability is at its maximum effect.

---

### Q: Bladesurge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 600 units |
| **Speed** | 1400 + 100% movement speed |
| **Cost** | 15 Mana |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 seconds |
| **Queue Time** | $0.5$ (This may not be valid in its entirety when the cooldown refreshes, due to other commands being canceled/queued automatically) seconds |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Parry** | True |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Irelia** dashes 100 units through the target enemy's location, and upon collision or dash completion, she deals physical damage, applies on-hit effects, and heals herself. *Bladesurge* deals 50 to 237 **bonus** physical damage to minions.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 5 / 25 / 45 / 65 / 85 (+ 70% AD) |

| Attribute | Value |
|-----------|------:|
| **Heal** | 9 / 10 / 11 / 12 / 13% AD |

*Bladesurge*’s **current** cooldown is reduced to $0.2$ seconds upon collision of targets marked as *Bladesurge 2.png*, and is reset (The check's cadence is 0.0625 seconds, or a bit less than 2 game ticks) if the target dies to or during *Bladesurge*’s dash.

*Flawless Duet can be cast during the dash.*

**Notes:**

- *Bladesurge* will be buffered and cast when the cooldown ends or is reset if the player attempts to cast it within $0.5$ seconds of the cooldown ending/resetting.
- *Bladesurge* deals basic damage, but also triggers spell effects by also being tagged as spell damage.
- If the target dies during *Bladesurge*’s dash, the cooldown is fully refunded and **Irelia** may also cast her other abilities during the dash.
- If the target dies to *Bladesurge* and was also marked, the cooldown is still fully refunded.
- *Bladesurge* will only allow **Irelia** to dash through walls if there is enough space for her on the other side.
- *Bladesurge* deals its damage and consumes marks when **Irelia** collides with the target, or at the end of the dash if the target moves away.
  - *Bladesurge* will not deal damage if the dash is interrupted beforehand.
    - The damage will be dealt if the target is in collision range when the dash is interrupted however.
    - Flash will interrupt the dash but **Irelia** will still collide with and deal damage to her target if the blink leaves her in contact with it at the new location.
  - **Irelia** will complete her remaining dash distance even if the damage is dealt earlier.
  - If the target is untargetable, *Bladesurge* will instead deals its damage at the end of the dash, rather than on collision.
- **Irelia** will automatically be ordered to acquire and attack *Bladesurge*’s target as soon as the dash completes.
  - If a different target than *Bladesurge*’s is selected during the dash, she will acquire that different target instead.

---

### W: Defiant Dance

| Attribute | Value |
|-----------|------:|
| **Range** | 775 (Recast swipe missile range) / 895 (Recast swipe total range) units |
| **Cast Time** | None (Active) / $0.25$ (Recast) |
| **Effect Radius** | 300 (Recast point blank radius) / 120 (Recast 'lollipop' radius at end of missile range) units |
| **Width** | 240 (Recast swipe missile width) units |
| **Cost** | 70 / 75 / 80 / 85 / 90 Mana |
| **Cooldown** | 20 / 18 / 16 / 14 / 12 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto / Direction |
| **Affects** | Self / Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |
| **Silence** | False |

**ACTIVE:** **Irelia** charges for up to $1.5$ seconds, during which she increases *Defiant Dance*’s damage over the first $0.75$ seconds of the channel and reduces incoming physical damage by 40 to 70 (+ 7% per 100 AP) and incoming magic damage by 20 to 35 (+ $3.5$% per 100 AP).

*Defiant Dance* can be recast within the duration, and does so automatically afterwards. *Defiant Dance*’s charge cannot be interrupted by crowd control.

**RECAST:** **Irelia** swipes her blades in the target direction, dealing physical damage to enemies around her and within a line, increased by 0% / 20% / 40% / 60% / 80% / 100% / 120% / 140% / 160% / 180% / 200%. She also retains the damage reduction for $0.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Minimum Physical Damage** | 10 / 20 / 30 / 40 / 50 (+ 40% AD) (+ 50% AP) |
| **Maximum Physical Damage** | 30 / 60 / 90 / 120 / 150 (+ 120% AD) (+ 150% AP) |

*Defiant Dance's recast can be used while affected by cast-inhibiting crowd control. Ionian Fervor’s duration is paused during the charge; if Ionian Fervor has less than 1 second remaining when the charge starts, it is set to 1 second when the charge ends.*

**Notes:**

- Enemies around **Irelia** will be dealt the damage once the recast's cast time ends, while enemies within the line are dealt the damage via a missile that reaches the furthest targets after a delay after the recast's cast time.
- Quick Casting utilizes hold-and-release for the two casts.
- *Defiant Dance* reaches 100% physical damage reduction at AP.
- *Defiant Dance* reaches 100% magic damage reduction at AP.
- *Defiant Dance* will also recast automatically upon entering resurrection.
- This ability will cast from wherever the caster is at the end of the cast time.

  - This refers to the recast.
- The following table refers for interactions while **Irelia** is channeling:

| Aspect | State / Notes |
|--------|---------------|
| **Type** | Charge channel |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Disabled |
| **Summoner Spells** | Allowed / Disabled / Recasts |
| **Consumables** | Disabled |
| **Interrupted by** | death |
| **Notes** | but can still use trinkets |

---

### E: Flawless Duet

| Attribute | Value |
|-----------|------:|
| **Cast Time** | None (Initial cast/recast) |
| **Target Range** | 775 units |
| **Effect Radius** | Global (Blade convergence range) |
| **Width** | 70 (Convergence missile) units |
| **Speed** | 2000 (Missile speed to location) / Distance units/second |
| **Cost** | 50 Mana |
| **Cooldown** | 16 / 14.5 / 13 / 11.5 / 10 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.3$ (Initial cast/recast) seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Irelia** sends a blade to the target location for $3.5$ seconds. *Flawless Duet* can be recast after $0.15$ seconds while the blade is active, and does so automatically at the end of its duration, though not if she is unable to cast abilities.

**RECAST:** **Irelia** sends a second blade to the target location, or to her current position if *Flawless Duet* was recast automatically.

Once both blades have been placed, they fly toward each other regardless of distance and converge over $0.25$ seconds, afterwards dealing magic damage to all enemies within a line between them and stunning them for $0.75$ seconds, during which they are revealed.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 110 / 150 / 190 / 230 (+ 100% AP) |

*Flawless Duet's recast can be used during Bladesurge and the cast time of Vanguard's Edge. Each of the blades' travel times are .*

**Notes:**

- Both casts count as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
  - If the ability is not manually recasted, the secondary effect will trigger and it will be considered as an ability activation. This is the only ability with this interaction in the game.
- *Flawless Duet* will recast automatically at its end during Defiant Dance despite being disabled.
- *Flawless Duet* will also recast automatically upon entering resurrection.
- The outgoing blades will stop flight upon touching projectile blocking effects. The converging blades will instead be destroyed upon contact with such effects.
  - The blades will not stun nor deal damage if they fail to converge (i.e Yasuo’s Wind Wall).
- Both of *Flawless Duet*’s casts will be buffered and cast when their cooldowns end if the player attempts to cast each within $0.3$ seconds of their cooldown ending.

---

### R: Vanguard's Edge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.4$ seconds |
| **Target Range** | 1000 units |
| **Width** | 320 units |
| **Speed** | 2000 units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 125 / 115 / 105 / 95 / 85 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Irelia** launches a barrage of blades in the target direction, expanding outward upon hitting an enemy champion, dealing magic damage to all enemies hit and revealing them for 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 125 / 162.5 / 200 / 237.5 / 275 (+ 70% AP) |

After expanding, the blades drop on the ground, knocking all enemy units away from them, though not rendering them airborne, and forming a spade-shaped perimeter around the enemy champion hit for $2.5$ seconds that grants sight of its surroundings. Enemies that pass through the blades for the first time are dealt the same magic damage and are slowed by 90% for $1.5$ seconds.

*The perimeter will point at the same angle **Irelia** casted it from, but based on the target's center.*

**Notes:**

- Enemies that cross the perimeter are **not** marked *Unsteady*.
- This ability will cast from wherever the caster is at the end of the cast time.
- In addition to its projectile, *Vanguard's Edge* will check for enemy champions within a cr 90-unit radius area from the projectile's origin point.
  - The projectile interacts with projectile-blocking effects but the area check doesn't.
- The blades will still expand if the champion hit by the blades is protected by a spell shield.
- The sight is granted in a 1350 (estimated) unit radius at the location of the enemy champion at the time of being hit and can be blocked by terrain.

---

## Patch History

### V25.17
- Defiant Dance
  - Minimum AP ratio increased to 50% AP from 40% AP.
    - Maximum AP ratio increased to 150% AP from 120% AP.
- Flawless Duet
  - AP ratio increased to 100% AP from 80% AP.
  - Cooldown reduced to 16 / 14.5 / 13 / 11.5 / 10 seconds from 16 / 15 / 14 / 13 / 12.

### V25.15
- Stats
  - Base health regeneration reduced to $3.5$ from 6.
- Irelia
  - English voice-over added.

### V25.13
- Ionian Fervor
  - Attack speed per stack increased to 10 to 25 from 7.5 to 25.
    - Maximum attack speed increased to 10×4 to 25×4 from 7.5×4 to 25×4.
  - **Bug Fixes:** Granted attack speed now updates on level-up.

### V25.10
- Defiant Dance
  - **New Effect:** Now entirely pauses the duration of Ionian Fervor while charging, instead of refreshing its duration every $0.25$ seconds upon Ionian Fervor reaching the last $0.5$ seconds of its duration.
  - **New Effect:** After finishing the charge, now increases the remaining duration of Ionian Fervor to 1 second if it was below 1 second upon starting the charge.
  - **Bug Fixes:** Now correctly resets the duration of Ionian Fervor if the former hits an enemy very close to the latter's expiration.

### V25.S1.2
- Defiant Dance
  - Base damage reduced to 10 / 20 / 30 / 40 / 50 from 10 / 25 / 40 / 55 / 70.
    - Maximum base damage reduced to 10×3 to to 50×3 from 30 / 75 / 120 / 165 / 210.
- Flawless Duet
  - Base damage reduced to 70 / 110 / 150 / 190 / 230 from 80 / 125 / 170 / 215 / 260.
- Vanguard's Edge
  - Base damage reduced to 125 / 200 / 275 from 125 / 250 / 375.

### V14.22
- Ionian Fervor
  - **New Effect:** Damage now applies against structures at 50% damage.
  - **New Effect:** Duration is now refreshed against structures.

### V14.21
- Bladesurge
  - Cooldown reduced to 10 / 9 / 8 / 7 / 6 seconds from 11 / 10 / 9 / 8 / 7.
  - Mana cost reduced to 15 from 20.
  - AD ratio increased to 70% AD from 60% AD.
  - Bonus minion damage reduced to 50 to 237 from 55 to 259.
- Vanguard's Edge
  - **Removed:*** No longer reduces Bladesurge’s cooldown by 0.5 / 1 / 1.5 seconds.

### V14.15
- Stats
  - Base health increased to 630 from 590.
  - Base health regeneration reduced to 6 from $8.5$.

### V14.14
- Stats
  - Attack damage growth reduced to $3.5$ from 4.
- Ionian Fervor
  - Attack speed per stack changed to 7.5 to 25 from 7.5 to 20. *Now scales with every level.*
    - Maximum attack speed changed to 7.5×4 to 25×4 from 7.5×4 to 20×4.
- Vanguard's Edge
  - Bladesurge cooldown reduction reduced to 0.5 / 1 / 1.5 seconds from 0.5 / 1.5 / 2.5.

### V14.11
- Stats
  - Health growth reduced to 115 from 124.

## Trivia

- According to Kuo-Yen 'Xypherous' Lo, Irelia is based on from Rurouni Kenshin.

---
*This page was automatically generated from League of Legends Wiki data.*