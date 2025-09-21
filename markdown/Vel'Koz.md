# Vel'Koz

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
| **Champion** | Vel'Koz |
| **Title** | the Eye of the Void |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2014-02-27 |
| **Release Patch** | V4.3 |
| **Roles** | Artillery |
| **Riot Positions** | Support |
| **External Positions** | Middle, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $590.0$ | $+102.0$ | $2324.0$ |
| **Mana** | $469.0$ | $+21.0$ | $826.0$ |
| **Health Regen** | $5.5$ | $+0.55$ | $14.9$ |
| **Mana Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Armor** | $22.0$ | $+4.7$ | $101.9$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $55.0$ | $+3.1416$ | $108.4$ |
| **Attack Speed** | $0.643$ | $+1.6\%$ | $0.817$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $525.0$ | $+0.0$ | $525.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.643$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.6\%$ |
| **Missile Speed** | $0 units/second$ |
| **Acquisition Radius** | $575 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $302.778 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Organic Deconstruction

**Innate:** **Vel'Koz**’s abilities apply a stack of *Deconstruction* to enemies hit for a few seconds.

*The third stack consumes them all to deal **bonus** true damage.*

**Innate:** ''Vel'Koz's* abilities apply a stack of *Deconstruction' to enemies hit for 7 seconds, refreshing on basic attacks and subsequent applications and stacking up to 3 times. The third stack consumes them all to deal true damage (+ 60% AP) **bonus** true damage.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies / Self |
| **Damage Type** | True |
| **Spell Shield** | True |
| **Spell Effects** | Proc |

**Notes:**

- No additional notes.

---

### Q: Plasma Fission

**Active:** **Vel'Koz** fires a plasma bolt that deals magic damage to the first enemy hit, slow them for a short time. *Plasma Fission* can be recast while the bolt is in flight, and does so automatically upon hitting an enemy or reaching max range.

**Recast:** The bolt splits in two, each firing in perpendicular opposite directions.

**Active:** ''Vel'Koz'* fires a plasma bolt in the target direction that deals magic damage to the first enemy hit and slow them by 70% decaying over a duration. *Plasma Fission' can be recast after $0.25$ seconds while the bolt is in flight, and does so automatically upon hitting an enemy or reaching maximum range. **Recast:** ''Vel'Koz'' splits the bolt in two over $0.25$ seconds, each part firing perpendicularly in opposite directions and applying the same effects to enemies hit. Whenever *Plasma Fission* kills an enemy, ''Vel'Koz'' restores mana.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 7 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $40-60$ Mana |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1300 / 2100 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Area of Effect |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $80-240$ (+ 90% AP)
- **Slow Duration:** $1-2.6$ seconds
- **Mana Restored per Kill:* $20-30$3-30×3$

**Notes:**

- The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
  - Detonating the ability manually does not.
- 'Plasma Fission's* damage applies before **Organic Deconstruction*’s'.
  - This also ensures ''Vel'Koz'* does not lose *Plasma Fission's restoration effect.
- An enemy cannot be hit by multiple bolts even when having blocked one with a spell shield.
- The split occurs at a location 55 units in front of the primary bolt's final location.
- *Plasma Fission* can reach a maximum 1595 units diagonally (ignoring the split missile's width and enemy hitbox radius), reaching this distance after a total of about ~$1.65$ seconds.Effect at cast time end

---

### W: Void Rift

**Active:** **Vel'Koz** cracks a in the target direction that opens after a brief delay, cascading through the area to deal magic damage to enemies within.

*After a brief delay, the collapses, dealing magic damage to enemies within.*

**Active:** ''Vel'Koz'' cracks a in the target direction that opens after a $0.25$-second delay, cascading through the area over $0.65$ seconds to deal magic damage to enemies within. After $0.75$ seconds, the violently collapses, dealing magic damage to enemies within and granting sight of the surrounding area for $0.25$ seconds. ''Vel'Koz'* periodically stocks a *Void Rift' charge, up to a maximum of 2.

| Attribute | Value |
|-----------|-------|
| **Recharge** | $19-15$ seconds |
| **Cast Time** | none |
| **Cost** | $50-70$ Mana + 1 Charge |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1700 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**Scaling:**
- **Magic Damage:** $30-110$ (+ 20% AP)
- **Magic Damage:** $45-165$ (+ 25% AP)

**Notes:**

- *Unbreakable* and *Wind Wall* will block the ripple and prevent the from forming further, but they will not destroy the section that was created already.
- Spell shield only blocks one instance of damage.

---

### E: Tectonic Disruption

**Active:** **Vel'Koz** hurls a disruptive anomaly at the target location that deals magic damage to enemies hit, briefly airborne and stuns them.

**Active:** ''Vel'Koz'' hurls a disruptive anomaly that lands to the target location after 0.25–0.25 to 0.55@0–0 (@=cast distance) seconds, dealing magic damage to enemies hit and airborne and stun them for $0.75$ seconds, as well as granting sight of the area briefly. If *Tectonic Disruption* is cast in close proximity of ''Vel'Koz'', the anomaly will also slightly airborne enemies hit in the direction of the cast.

| Attribute | Value |
|-----------|-------|
| **Range** | 800<!-- units |
| **Cooldown** | $12-10$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $50-70$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 225 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**Scaling:**
- **Magic Damage:** $70-190$ (+ 30% AP)

**Notes:**

- The required proximity for the airborne is relative to ''Vel'Koz's' position when casting and not his current one.
- While the effect is fully prevented by blocking the missile (e.g. with *Unbreakable* and *Wind Wall*), the missile may not be destroyed *visually*.
- The area of effect indicator appears at the end of the cast time.
  - It will persist even if the cast cancels (e.g. ''Vel'Koz'' dies before cast time completes) despite the projectile not firing.
- Displacement immunity will also resist the application of the stun. Effect at cast time start

---

### R: Life Form Disintegration Ray

**Passive - Researched:** Applying 3 **Deconstruction** stacks will mark the target *Researched* for a few seconds, which refreshes on basic attacks and ability hits.

**Active:** **Vel'Koz** channel for a short time to project an energy beam that continually deals magic damage, applies **Deconstruction**, and briefly slow enemies hit. During this time, ''Vel'Koz'* can steer the beam in the target direction. *Life Form Disintegration Ray' can be recast within the duration, and does so automatically afterwards.

**Passive - Researched:** Applying 3 **Deconstruction** stacks to an enemy champion marks them as *Researched* for 7 seconds, refreshing on basic attacks and ability hits against them. **Active:** After a $0.2$-second delay, ''Vel'Koz'* channel for up to $2.6$ seconds to project an energy beam, during which he can steer the beam in the target direction. *Life Form Disintegration Ray' can be recast after 1 second during the channel, and does so automatically after the duration. The beam deals magic damage to enemies hit every $0.2$ seconds, and slow them by 20%, lingering for 1 second. **Deconstruction** is applied every $0.7$ seconds to enemies hit. **Researched** enemies take true damage instead. **Recast:** ''Vel'Koz'* ends *Life Form Disintegration Ray'.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $100-80$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic True |
| **Spell Effects** | aoedot |
| **Projectile** | False |

**Scaling:**
- **Damage Per Tick:** $(4500.2)/2.60.2)/2.6 (9250.2)/2.60.2/2.6 Maximum Damage $450/700/925$ (+ 125% AP)

**Notes:**

- 'Life Form Disintegration Ray's* direction updates gradually (moving the cursor from one of *'Vel'Koz'' to the other will not make him rotate instantly).
- ''Vel'Koz'' will sight himself if the ray is near an enemy champion.
- The beam hits in a rectangular shape, intersecting with the edge of an enemy gameplay radius.
  - Because of this, the effect will hit enemies whose center location is to the sides and/or slightly behind Vel'Koz, as long as their radius intersects with the area.
- The following table refers for interactions while ''Vel'Koz'' is channel:

---

## Patch History

### V25.18
- *Tectonic Disruption*
  - Cooldown reduced to $12-10$ seconds from $14-12$.
- *Life Form Disintegration Ray*
  - Total base damage increased to $450/700/925$ from $450-800 3$.

### V25.14
- General
  - *Organic Deconstruction*’s damage now applies after *Plasma Fission*’s.

### V14.24
- *Tectonic Disruption*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.
- *Tectonic Disruption*
  - **Bug Fixes:** Travel time is once again reduced with decreasing cast distance, instead of always requiring the maximum amount of time.

### V14.22
- *Tectonic Disruption*
  - Cooldown reduced to $14-12$ seconds from $16-12$.
- *Life Form Disintegration Ray*
  - Cooldown reduced to $100-80 3$ seconds from $120-80 3$.

### V14.9
- Stats
  - Selection radius increased to 120 units from $88.8889007568359$.

### V14.7
- Vel'Koz, Vel'Koz, Vel'Koz
  - *Void Rift*
    - **Bug Fixes:** No longer uses VFX and particles from Rell *Magnet Storm*.

### V14.1
- *Tectonic Disruption*
  - Travel time reduced to 0.25–0.25 to 0.55@0–0 (@=cast distance) seconds from $0.55$ seconds at all distances.
  - **New Effect:** Ground indicator now appears when the cast time has finished and not on-cast.

### V13.24
- *Void Rift*
  - **Bug Fixes:** Rift missile and the entire visual effects now spawn even if he dies in the first $0.25$ seconds after casting. Consequently, the collapse damage is now accompanied by its proper visuals in this case.
- *Tectonic Disruption*
  - **New Effect:** Missile now spawns near the target location instead of at his location.
  - **Bug Fixes:** Spell no longer fizzles out it if he moved further than 1100 units away from the target location before the end of the cast time (e.g. using Flash).

### V13.23
- *Organic Deconstruction*
  - Base true damage increased to 35 to 180 from 33 to 169.
  - AP ratio increased to 60% AP from 50% AP.

### V13.22
- Stats
  - Base attack speed increased to $0.643$ from $0.625$.
  - Attack speed growth increased to $1.59$% from $1.36$%.

## Trivia

- Vel'Koz has Pi ($3.141592$) attack damage growth and uses Trigonometry to land *Plasma Fission*.
- Vel'Koz's taunt is Schrödinger's equation written in from V (franchise) by Kenneth Johnson (producer).
- Vel'Koz is based on the Oculothorax archetype, with one of these in particular (Shuma-Gorath from Marvel Comics) used as the basis for *Life Form Disintegration Ray*.
- Vel'Koz's personality might have been based on the amoral Decepticon scientist Shockwave_(Transformers), from the Transformers franchise. Shockwave's head consists mostly of his single glaring eye, which might also have inspired Vel'Koz.
- Vel'Koz had his game assets reused for several featured game modes.
  - He was given a retexture and features as a monster called the "Shooty Eyeball Monster" (along with its huge variant) in Invasion.
  - His animations were reused for the Vel'Koz and Vel'Koz monsters in Odyssey: Extraction.
- *Life Form Disintegration Ray* is the longest ability name, at 28 characters long.
  - For contrast, it is longer than *Hop*, the shortest ability name at 3 characters long, by 25 characters.
- Vel'Koz's cost is 1618, a reference to the Golden ratio.

---
*This page was automatically generated from League of Legends Wiki data.*