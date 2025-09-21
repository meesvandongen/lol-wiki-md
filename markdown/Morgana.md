# Morgana

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
| **Champion** | Morgana |
| **Title** | the Fallen |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Roles** | Catcher |
| **Riot Positions** | Support |
| **External Positions** | Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $630.0$ | $+104.0$ | $2398.0$ |
| **Mana** | $340.0$ | $+60.0$ | $1360.0$ |
| **Health Regen** | $5.5$ | $+0.4$ | $12.3$ |
| **Mana Regen** | $11.0$ | $+0.4$ | $17.8$ |
| **Armor** | $25.0$ | $+5.0$ | $110.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $56.0$ | $+3.5$ | $115.5$ |
| **Attack Speed** | $0.625$ | $+1.5\%$ | $0.788$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $450.0$ | $+0.0$ | $450.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.5\%$ |
| **Missile Speed** | $1600 units/second$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $145 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Soul Siphon

**Innate:** **Morgana** heal herself for a portion of the damage dealt by her abilities against champions, large minions, and medium and large monsters.

**Innate:** **Morgana** heal herself for 18% of the post-mitigation damage dealt by her abilities against champions, large minions, and medium and large monsters.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- No additional details.

---

### Q: Dark Binding

**Active:** **Morgana** throws a sphere of dark magic in the target direction that deals magic damage to the first enemy hit and root them for a "short time".

**Active:** **Morgana** throws a sphere of dark magic in the target direction that deals magic damage to the first enemy hit and root them for a duration.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 10 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $50/55/60/65/70$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1200 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:* $80*Root Duration:** $2-3$ seconds

**Notes:**

No additional notes.

---

### W: Tormented Shadow

**Passive:** 'Tormented Shadow's* **current** cooldown is cdr whenever **Soul Siphon*' triggers.

**Active:** **Morgana** desecrates the target area for a few seconds, which continually deals magic damage to enemies based on their **missing** health.

**Passive:** 'Tormented Shadow's* **current cooldown** is reduced by 5% of its **total** cooldown whenever **Soul Siphon*' triggers. Simultaneous triggers from multiple targets will stack the cooldown reduction. **Active:** **Morgana** torments the soil at the target location, causing the area to become desecrated for 5 seconds. Enemies within take magic damage on-cast and every $0.5$ seconds thereafter, increased by type=target's **missing** health. *Tormented Shadow* deals 170% damage against monsters.

| Attribute | Value |
|-----------|-------|
| **Range** | 900 units |
| **Cooldown** | 12 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-110$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 280 units |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |

**Scaling:**
- **Minimum Damage Per Tick:* $9-35$ (+ 10% AP)2-35×2$ (+ $10×2$% AP) **Minimum Total Damage:** $9×10-35×10$ (+ $1010$% AP)10×2-35×10×2$ (+ $10×10×2$% AP)

**Notes:**

- Damage from multiple *Tormented Shadows* does not stack.

---

### E: Black Shield

**Active:** **Morgana** shields the target allied champion or herself for a few seconds, which absorbs magic damage and grants cc-immune while the shield holds.

**Active:** **Morgana** grants a shield to the target allied champion or herself for 5 seconds, which absorbs incoming magic damage and grants cc-immune while it holds.

| Attribute | Value |
|-----------|-------|
| **Range** | 800 units |
| **Cooldown** | $26-16$ seconds |
| **Cast Time** | none |
| **Cost** | 80 mana |
| **Targeting** | Unit |
| **Affects** | Self, Allies |

**Scaling:**
- **Magic Shield Strength:** $100-320$ (+ 70% AP)

**Notes:**

- *Black Shield* will not resist self nor allied crowd control (including e.g. an allied *Tempered Fate*).
- *Black Shield* will not resist nearsight.
- *Black Shield* negates crowd control effects before any magic damage is absorbed; even if the shield is broken by an enemy dealing enough damage, its associated disables will not apply. Shield-destroying effects bypass this however, since they destroy the shield before applying their effects.
- *Black Shield* will not prevent effects other than crowd control from triggering (e.g. Nether Grasp will not suppression a target protected by *Black Shield* but the tether still applies).
  - Although not considered a *crowd control* effect, *Black Shield* is special-cased to block spirit pull.
- *Black Shield* takes priority over other sources of cc-immune and those that grant immunity to specific types of crowd control (displacement immunity and slow-immune).
- Spell shield will take priority over *Black Shield*.
- *Black Shield* will always take priority over regular shield. If used in conjunction with Lifeline, the most recently-applied one will have priority however.
- *Black Shield* has a forgiveness radius of 175 units.

---

### R: Soul Shackles

**Active:** **Morgana** latches chains of energy onto nearby enemy champions, dealing magic damage and forming a tether between herself and each target for a short time, during which the targets are true sight and slow, and **Morgana** gains **bonus movement speed**.

*If a target maintains the tether after its duration, they are dealt the same magic damage and become briefly true sight and stun.*

**Active:** **Morgana** latches chains of energy onto nearby enemy champions over the cast time, dealing magic damage and forming a tether between herself and each target for 3 seconds, during which she gains **bonus movement speed** and the targets are true sight and slow by 20%. If a target does not break their tether by the end of its duration, they are dealt the same magic damage again and become stun for a duration, during which they are true sight. *An enemy champion within cr 575 units is required to cast this ability. The target does not have to be sight to be tethered by this ability.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $120-100$ seconds |
| **Cast Time** | $0.35$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Proximity |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 625 units |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:* $200-350$ (+ 80% AP)2-350×2$ (+ $80×2$% AP) **Bonus Movement Speed:** $20-60$%
- **Stun Duration:** $1.5-2$ seconds

**Notes:**

- Spell shield will block the tether's application and initial damage but not the aftereffects of one already applied.
- **Morgana** will turn to face southeast upon casting *Soul Shackles*.
- A circle indicator for Soul Shackles' maximum tether range is visible to **Morgana** and her tethered targets only.

---

## Patch History

### V25.17
- *Tormented Shadow*
  - Minimum base damage per tick increased to $18/2-70/2$ from $14/2-70/2$.
    - Maximum base damage per tick increased to $18-70$ from $14-70$.
  - Mana cost reduced to $70-110$ from $70-130$.

### V25.16
- *Tormented Shadow*
  - Minimum base damage per tick increased to $14/2-70/2$ from $12/2-56/2$.
  - Minimum AP ratio per tick increased to $20/2$% AP from $17/2$% AP.
  - Bonus damage reduced to type=target's **missing** health from type=target's **missing** health.
    - Maximum base damage per tick reduced to $(14/2)*2-(70/2)*2$ from $(12/2)*2.7-(56/2)*2.7$.
    - Maximum AP ratio per tick reduced to $(20/2)*2$% AP from $(17/2)*2.7$% AP.

### V25.09
- *Black Shield*
  - Base shield increased to $100-320$ from $80-300$.
- *Soul Shackles*
  - Base damage per hit increased to $200-350 3$ from $175-325 3$.
  - Bonus movement speed increased to $20-60 3$% from $10-60 3$%.

### V14.24
- *Tormented Shadow*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.20
- *Black Shield*
  - **New Effect:** Cast now has a forgiveness radius of 175 units.

### V14.18
- General
  - **Bug Fixes:** First encounter voice lines are no longer audible to enemies.

### V14.15
- General
  - **Bug Fixes:** Restored first encounter with **Jarvan IV** VO.

### V14.12
- General
  - **Bug Fixes:** "Loves me" and "Loves me not" voice-overs are no longer incorrectly swapped in her Joke.

### V14.4
- *Soul Shackles*
  - Tether now innately checks every $0.05$ seconds for valid range conditions, and no longer waits for every 8th server tick.
- Morgana
  - *Black Shield*
    - **Bug Fixes:** VFX now disappears smoothly when destroyed.

### V13.24
- *Tormented Shadow*
  - **Bug Fixes:** Pool VFX is now visible over terrain.

## Trivia

- *Dark Binding's* debuff reads: *“This unit is unable to move. Lasts for roughly 3 years.”*
- Morgana and Morgana are voiced.md) by Erica Lindbeck, who also voices **Taliyah** and **Zoe**.
- Her dance is a reference to Exid - Up & Down.
  - A side-by-side comparison can be seen here.
- Morgana was voiced.md) by Rebecca Schweitzer, who also voices Pre-rework **Sivir**.
- Her dance is a reference to the whirling practices of the Mevlevi Order.
  - A side-by-side comparison can be seen here.
- Morgana - **Kayle** is one of seven pairs of sibling champions (the others being **Cassiopeia** - **Katarina**, **Garen** - **Lux**, **Nasus** - **Renekton**, **Yasuo** - **Yone**, **Darius** - **Draven**, and **Vi** - **Jinx**).
  - Though not a pair, **Anivia**, **Ornn**, and **Volibear** are also siblings.

---
*This page was automatically generated from League of Legends Wiki data.*