# Vel'Koz

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
| **Champion** | Vel'Koz |
| **Title** | the Eye of the Void |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2014-02-27 |
| **Release Patch** | V4.3 |
| **Latest Changes** | V25.18 |
| **Roles** | Artillery |
| **Riot Positions** | Support |
| **External Positions** | Middle, Support |
| **Blue Essence** | 1618 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $590.0$ | $+102.0$ |
| **Mana** | $469.0$ | $+21.0$ |
| **Health Regen** | $5.5$ | $+0.55$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $22.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $55.0$ | $+3.1416$ |
| **Attack Speed** | $0.643$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $525.0$ | $+0.0$ |
| **Base Attack Speed** | $0.643$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $1.6\%$ | |
| **Missile Speed** | $0$ units/second | |
| **Acquisition Radius** | $575$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $302.778$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $105.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Organic Deconstruction

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies / Self |
| **Damage Type** | True |
| **Spell Shield** | True |
| **Spell Effects** | Proc |

**INNATE:** **Vel'Koz**’s abilities apply a stack of *Deconstruction* to enemies hit for 7 seconds, refreshing on basic attacks and subsequent applications and stacking up to 3 times.

The third stack consumes them all to deal (true damage) 35 to 180 (+ 60% AP) **bonus** true damage.

**Notes:**

- No additional notes.

---

### Q: Plasma Fission

| Attribute | Value |
|-----------|------:|
| **Range** | 1100 (Primary missile range) / 1100 (Split missile range) units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 100 (Primary missile width) / 90 (Split missile width) units |
| **Speed** | 1300 (Primary missile speed) / 2100 (Split missile speed) units/second |
| **Cost** | 40 / 45 / 50 / 55 / 60 Mana |
| **Cooldown** | 7 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of Effect |
| **Projectile** | True |

**ACTIVE:** **Vel'Koz** fires a plasma bolt in the target direction that deals magic damage to the first enemy hit and slows them by 70% decaying over a duration.


*Plasma Fission* can be recast after $0.25$ seconds while the bolt is in flight, and does so automatically upon hitting an enemy or reaching maximum range.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 120 / 160 / 200 / 240 (+ 90% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow Duration** | 1 / 1.4 / 1.8 / 2.2 / 2.6 seconds |

**RECAST:** **Vel'Koz** splits the bolt in two over $0.25$ seconds (Estimated), each part firing perpendicularly in opposite directions and applying the same effects to enemies hit.

Whenever *Plasma Fission* kills an enemy, **Vel'Koz** restores (mana) mana.

| Attribute | Value |
|-----------|------:|
| **Mana Restored per Kill** | 20 / 22.5 / 25 / 27.5 / 30 |
| **Maximum Mana Restored** | 60 / 67.5 / 75 / 82.5 / 90 |

**Notes:**

- The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
  - Detonating the ability manually does not.
- *Plasma Fission*’s damage applies before *Organic Deconstruction’s*.
  - This also ensures **Vel'Koz** does not lose *Plasma Fission*’s restoration effect.
- An enemy cannot be hit by multiple bolts even when having blocked one with a spell shield.
- The split occurs at a location 55 units in front of the primary bolt's final location.
- *Plasma Fission* can reach a maximum 1595 units diagonally (ignoring the split missile's width and enemy hitbox radius), reaching this distance after a total of about ~$1.65$ seconds.
- This ability will cast from wherever the caster is at the end of the cast time.

---

### W: Void Rift

| Attribute | Value |
|-----------|------:|
| **Range** | 1105 (Ripple missile range) / er 1105 (Rift collapse distance) / -95 (Rift collapsing backwards distance) units |
| **Cast Time** | none |
| **Width** | 175 (Ripple missile width) / 175 (Rift collapse width) units |
| **Speed** | 1700 (Ripple missile speed) units/second |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana + 1 Charge |
| **Recharge** | 19 / 18 / 17 / 16 / 15 seconds |
| **Static Cooldown** | $1.5$ |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**ACTIVE:** **Vel'Koz** cracks a in the target direction that opens after a $0.25$-second delay, cascading through the area over $0.65$ seconds to deal magic damage to enemies within.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 30 / 50 / 70 / 90 / 110 (+ 20% AP) |

After $0.75$ seconds (Estimated), the violently collapses, dealing magic damage to enemies within and granting sight of the surrounding area for $0.25$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 45 / 75 / 105 / 135 / 165 (+ 25% AP) |
| **Total Magic Damage** | 75 / 125 / 175 / 225 / 275 (+ 45% AP) |

**Vel'Koz** periodically stocks a *Void Rift* charge, up to a maximum of 2.

**Notes:**

- Unbreakable and Wind Wall will block the ripple and prevent the from forming further, but they will not destroy the section that was created already.
- Spell shield only blocks one instance of damage.

---

### E: Tectonic Disruption

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 800<!-- units |
| **Effect Radius** | 225 units |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 12 / 11.5 / 11 / 10.5 / 10 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Vel'Koz** hurls a disruptive anomaly that lands to the target location after 0.25–0.25 to 0.55@0–0 (@=cast distance) seconds, dealing magic damage to enemies hit and knocking them up and stunning them for $0.75$ seconds, as well as granting sight of the area briefly.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 100 / 130 / 160 / 190 (+ 30% AP) |

If *Tectonic Disruption* is cast in close proximity of **Vel'Koz**, the anomaly will also slightly knock back enemies hit in the direction of the cast.

**Notes:**

- The required proximity for the knock back is relative to **Vel'Koz**’s position when casting and not his current one.
- While the effect is fully prevented by blocking the missile (e.g. with Unbreakable and Wind Wall), the missile may not be destroyed *visually*.
- The area of effect indicator appears at the end of the cast time.
  - It will persist even if the cast cancels (e.g. **Vel'Koz** dies before cast time completes) despite the projectile not firing.
- Displacement immunity will also resist the application of the stun.
- This ability will cast from wherever the caster is at the start of the cast time.

---

### R: Life Form Disintegration Ray

| Attribute | Value |
|-----------|------:|
| **Range** | 1555 (Measured) units |
| **Cast Time** | none |
| **Width** | 175 units |
| **Cost** | 100 Mana |
| **Cooldown** | 100 / 95 / 90 / 85 / 80 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic True |
| **Spell Effects** | aoedot |
| **Projectile** | False |
| **Silence** | True |

**PASSIVE - RESEARCHED:** Applying 3 *Deconstruction* stacks to an enemy champion marks them as *Researched* for 7 seconds, refreshing on basic attacks and ability hits against them.

**ACTIVE:** After a $0.2$-second delay, **Vel'Koz** channels for up to $2.6$ seconds to project an energy beam, during which he can steer the beam in the target direction. *Life Form Disintegration Ray* can be recast after 1 second during the channel, and does so automatically after the duration.

The beam deals magic damage to enemies hit every $0.2$ seconds, and slows them by 20%, lingering for 1 second. *Deconstruction* is applied every $0.7$ seconds to enemies hit. *Researched* enemies take true damage instead.

| Attribute | Value |
|-----------|------:|
| **Damage Per Tick** | 34.62 / 53.85 / 71.15 (+ 9.62% AP) |
| **Maximum Damage** | 450 / 700 / 925 (+ 125% AP) |

**RECAST:** **Vel'Koz** ends *Life Form Disintegration Ray*.

**Notes:**

- *Life Form Disintegration Ray*’s direction updates gradually (moving the cursor from one of **Vel'Koz** to the other will not make him rotate instantly).
- **Vel'Koz** will reveal himself if the ray is near an enemy champion.
- The beam hits in a rectangular shape, intersecting with the edge of an enemy gameplay radius.
  - Because of this, the effect will hit enemies whose center location is to the sides and/or slightly behind Vel'Koz, as long as their radius intersects with the area.
- The following table refers for interactions while **Vel'Koz** is channeling:

| Aspect | State / Notes |
|--------|---------------|
| **Type** | Charge channel |
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Usable: Shurelya's Battlesong, Youmuu's Ghostblade, Randuin's Omen; Disabled: Zhonya's Hourglass; Other items: Interrupt |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite; Disabled: Flash, Hexflash; Interrupted by: Teleport, Recall |

---

## Patch History

### V25.18
- Tectonic Disruption
  - Cooldown reduced to 12 / 11.5 / 11 / 10.5 / 10 seconds from 14 / 13.5 / 13 / 12.5 / 12.
- Life Form Disintegration Ray
  - Total base damage increased to 450 / 700 / 925 from 450 / 625 / 800.

### V25.14
- General
  - Organic Deconstruction’s damage now applies after Plasma Fission’s.

### V14.24
- Tectonic Disruption
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.22#November 6th Hotfix|V14.22
- Tectonic Disruption
  - **Bug Fixes:** Travel time is once again reduced with decreasing cast distance, instead of always requiring the maximum amount of time.

### V14.22
- Tectonic Disruption
  - Cooldown reduced to 14 / 13.5 / 13 / 12.5 / 12 seconds from 16 / 15 / 14 / 13 / 12.
- Life Form Disintegration Ray
  - Cooldown reduced to 100 / 90 / 80 seconds from 120 / 100 / 80.

### V14.9
- Stats
  - Selection radius increased to 120 units from $88.8889007568359$.

### V14.7
- Vel'Koz, Vel'Koz, Vel'Koz
  - Void Rift
    - **Bug Fixes:** No longer uses VFX and particles from Rell Magnet Storm.

### V14.1
- Tectonic Disruption
  - Travel time reduced to 0.25–0.25 to 0.55@0–0 (@=cast distance) seconds from $0.55$ seconds at all distances.
  - **New Effect:** Ground indicator now appears when the cast time has finished and not on-cast.

### V13.24
- Void Rift
  - **Bug Fixes:** Rift missile and the entire visual effects now spawn even if he dies in the first $0.25$ seconds after casting. Consequently, the collapse damage is now accompanied by its proper visuals in this case.
- Tectonic Disruption
  - **New Effect:** Missile now spawns near the target location instead of at his location.
  - **Bug Fixes:** Spell no longer fizzles out it if he moved further than 1100 units away from the target location before the end of the cast time (e.g. using Flash).

### V13.23
- Organic Deconstruction
  - Base true damage increased to 35 to 180 from 33 to 169.
  - AP ratio increased to 60% AP from 50% AP.

## Trivia

- Vel'Koz has Pi ($3.141592$) attack damage growth and uses Trigonometry to land Plasma Fission.
- Vel'Koz's taunt is Schrödinger's equation written in from V (franchise) by Kenneth Johnson (producer).
- Vel'Koz is based on the Oculothorax archetype, with one of these in particular (Shuma-Gorath from Marvel Comics) used as the basis for Life Form Disintegration Ray.
- Vel'Koz's personality might have been based on the amoral Decepticon scientist Shockwave_(Transformers), from the Transformers franchise. Shockwave's head consists mostly of his single glaring eye, which might also have inspired Vel'Koz.
- Vel'Koz had his game assets reused for several featured game modes.
  - He was given a retexture and features as a monster called the "Shooty Eyeball Monster" (along with its huge variant) in Invasion.
  - His animations were reused for the Vel'Koz and Vel'Koz monsters in Odyssey: Extraction.
- Life Form Disintegration Ray is the longest ability name, at 28 characters long.
  - For contrast, it is longer than Hop, the shortest ability name at 3 characters long, by 25 characters.
- Vel'Koz's cost is 1618, a reference to the Golden ratio.

---
*This page was automatically generated from League of Legends Wiki data.*