# Nocturne

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
| **Champion** | Nocturne |
| **Title** | the Eternal Nightmare |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2011-03-15 |
| **Release Patch** | V1.0.0.113 |
| **Roles** | Assassin |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $655.0$ | $+109.0$ | $2508.0$ |
| **Mana** | $275.0$ | $+35.0$ | $870.0$ |
| **Health Regen** | $7.0$ | $+0.75$ | $19.8$ |
| **Mana Regen** | $7.0$ | $+0.45$ | $14.7$ |
| **Armor** | $38.0$ | $+4.2$ | $109.4$ |
| **Magic Resist** | $32.0$ | $+1.55$ | $58.4$ |
| **Attack Damage** | $62.0$ | $+2.6$ | $106.2$ |
| **Attack Speed** | $0.721$ | $+2.7\%$ | $1.052$ |
| **Movement Speed** | $345.0$ | $+0.0$ | $345.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.721$ |
| **Attack Speed Ratio** | $0.721$ |
| **Bonus AS per Level** | $2.7\%$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $155 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Umbra Blades

**Innate:** Periodically, **Nocturne**’s next basic attack causes him to slash in a circle that deals **increased** physical damage to the target and nearby enemies, heal him for each enemy hit.

*Basic attacks cdr 'Umbra Blade's cooldown, increased against monsters and champions.*

**Innate:** Periodically, **Nocturne** empowers his next basic attack to slash in a circle, dealing 120% AD physical damage to the target and nearby enemies and healing himself for 13 to 30 (+ 30% AP) per enemy hit. Against minions, 'Umbra Blade's damage to secondary targets and healing are reduced by 50% and it applies on-hit effects to all targets at 100% effectiveness. *Umbra Blades can critically strike against the primary target, modifying the physical damage dealt to Basic attacks reduce 'Umbra Blades' *cooldown* by 1 second, increased to 3 against enemy champions and monsters.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 360 units |
| **Spell Shield** | False |
| **Spell Effects** | Special |

**Notes:**

- The primary target of the enhanced attack is dealt basic damage while the secondary targets of the slash are dealt default damage.
- The empowered attack will not trigger against structures nor wards.
  - Basic attacks against them will still grant the cooldown reduction.
- Parry effects will only block the damage dealt to the primary target.

---

### Q: Duskbringer

**Active:** **Nocturne** casts out a shadow blade in the target direction that leaves a *Dusk Trail* in its wake, dealing physical damage to enemies hit. Enemy champions hit will leave a *Dusk Trail* in their wake for a few seconds.

*While on the *Dusk Trail*, Nocturne is ghosted and gains *ms **bonus** movement speed* and .*

**Active:** **Nocturne** casts out a shadow blade in the target direction that leaves a *Dusk Trail* in its wake, dealing physical damage to enemies hit. Enemy champions and large monsters hit will leave a *Dusk Trail* behind while moving. *Dusk Trails* last 5 seconds and will slowly disappear afterwards. While on the *Dusk Trail*, **Nocturne** is ghosted and gains and *ms **bonus total** movement speed*.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 8 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-80$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 1600 units/second |
| **Effect Radius** | 150 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**Scaling:**
- **Physical damage:** $65-245$ (+ 85% bonus AD)
- **Bonus Attack Damage:** $15-55$
- **Bonus Movement Speed:** $15-35$%

**Notes:**

- *Duskbringer* checks for and hits targets 25 units outside of the missile's maximum range.
- 'Duskbringer's damage is not increased by the *bonus AD gained from its trail, meaning that casting it again while on the trail from the last cast will not result in higher damage.
  - It is observed from seasons ago that this is intentionally hard-coded into the ability to avoid the projectile increasing its own damage. This feature is somewhat deprecated as it also limits chain-casting *Duskbringer*.
- 'Duskbringer's Dusk Trail' will expose the path of affected units until its duration ends. It is a form of obscured vision; the trail indirectly reveals the location of enemy units even in the fog of war, including those shrouded by stealth.
- Spell shield will block the damage and also prevents the *Dusk Trail* from following the target.
- The formed *Dusk Trail* does not disappear if *Duskbringer* is intercepted by *Wind Wall* or *Unbreakable*. Effect at cast time start
- The **bonus** movement speed stacks multiplicatively with other sources of movement speed boosts.

---

### W: Shroud of Darkness

**Passive:** **Nocturne** gains **bonus attack speed**.

**Active:** **Nocturne** gains a brief spell shield. If it blocks an ability, the *bonus attack speed is doubled for a few seconds.

**Passive:** **Nocturne** gains **bonus attack speed**. **Active:** **Nocturne** gains a spell shield for $1.5$ seconds. Upon successfully blocking a hostile effect, 'Shroud of Darkness'*bonus attack speed is doubled for 5 seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $20-12$ seconds |
| **Cast Time** | none |
| **Cost** | 50 Mana |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- **Bonus Attack Speed:** $30-50$%
- **Enhanced Bonus Attack Speed:** $30×2-50×2$%

**Notes:**

- If *Shroud of Darkness* is active when **Nocturne** re-casts **Paranoia** to dash to a target, its duration is refreshed every $0.25 seconds$ during the dash.

---

### E: Unspeakable Horror

**Passive:** **Nocturne** gains *ms **bonus** movement speed* while moving towards fear targets.

**Active:** **Nocturne** torments the target, forming a tether that continually deals magic damage.

**Passive:** **Nocturne** gains ms*bonus** movement speed* while facing nearby fear targets. **Active:** **Nocturne** torments the target, forming a tether between himself and the target for 2 seconds, during which the target takes magic damage every $0.5$ seconds over the duration. If the tether is not broken by the end of its duration, the target is fear for a duration while being slow by 90%.

| Attribute | Value |
|-----------|-------|
| **Range** | 425 units |
| **Cooldown** | $15-11$ seconds |
| **Cast Time** | none |
| **Cost** | $60-80$ mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 1000 units |
| **Spell Shield** | Special |
| **Spell Effects** | DoT |
| **Projectile** | False |

**Scaling:**
- **Total Magic Damage:** $80-260$ (+ 100% AP) Magic Damage per Tick $80/4-260/4$ (+ $100/4$% AP)
- **Disable Duration:** $1.25-2.25$ seconds

**Notes:**

- Feared enemies in a cr 180° total angle within ''Nocturne's* facing direction will trigger *Unspeakable Horror's passive.
- Casting *Unspeakable Horror* may grant **Nocturne** permanent ghosting until he has died.
- Spell shield will block the tether's application but not the damage and aftereffects of one already applied.

---

### R: Paranoia

**Active:** **Nocturne** terrorizes all enemy champions, nearsight them for a few seconds. He can recast *Paranoia* during this time.

**Recast:** **Nocturne** dash with displacement immunity to the target enemy champion, dealing physical damage upon arrival.

**Active:** **Nocturne** terrorizes all enemy champions, nearsight them for 6 seconds. He can recast *Paranoia* for the same duration after $0.25$ seconds. **Recast:** **Nocturne** dash with displacement immunity to the target enemy champion, dealing physical damage upon arrival. **Nocturne can cast any of his abilities during the dash.**

| Attribute | Value |
|-----------|-------|
| **Range** | $2500/3250/4000$ units |
| **Cooldown** | $140-90$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Auto / Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 1800 units/second |
| **Effect Radius** | Global |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **Physical Damage:* $150bonus AD)

**Notes:**

- **Nocturne** will track the target if they change locations.
  - He will dash to the target's previous location without dealing damage if the target is too far away or moves beyond $3500/4250/5000$ (based on 'Paranoia's rank) units.
- Enemy players affected by 'Paranoia's nearsight cannot interact with the allied portrait icons on the HUD.
- 'Paranoia's nearsight will apply to enemy champions that are untargetable or are dead, and will persist through death.
- **Nocturne** can cast any of his abilities, summoner spells (excluding Hexflash), or item actives during flight.
  - Using a dash or blink ability will interrupt the flight.
  - The flight will also be interrupted if **Nocturne** is affected by a Blast Cone triggered by an allied champion.
- *Paranoia* cannot be recast while ground or root.
- 'Paranoia's nearsight does not apply to clone.
- Players' screens will turn a different color when *Paranoia* is cast, based on their perspective: blue for allies and red for enemies.
- A spell indicator telegraphed to **Nocturne** and his allies will be placed on all enemy champion within range of 'Paranoia's* recast while the ability is active. *The following images display the indicator for the default skin:'

---

## Patch History

### V25.16
- Stats
  - Armor growth reduced to $4.2$ from $4.7$.
- *Duskbringer*
  - Bonus attack damage reduced to $15-55$ from $20-60$.

### V25.10
- *Unspeakable Horror*
  - **Bug Fixes:** Now properly grants the passive effect while facing flee targets.

### V14.9
- *Umbra Blades*
  - **Bug Fixes:** The VFX that indicates it is active no longer persists through use if the ability's cooldown ended while under the effects of polymorph.

### V14.4
- *Unspeakable Horror*
  - Tether now innately checks every $0.05$ seconds for valid range conditions, and no longer waits for every 8th server tick.

### V14.2
- *Duskbringer*
  - **New Effect:** Now checks for and hits targets 25 units outside of the missile's maximum range.
  - **New Effect:** Large monsters hit now leave behind a *Dusk Trail* when they move.

### V13.15
- *Paranoia*
  - **Bug Fixes:** No longer applies the nearsight debuff to **Neeko** while she is disguised as a non-champion with *I*.

### V12.18
- Stats
  - Attack damage growth reduced to $2.6$ from $3.1$.

### V12.17
- Stats
  - Attack speed ratio increased to $0.721$ from $0.668$.
- *Umbra Blades*
  - Cooldown reduced to 13 seconds from 14.

### V12.14
- *Umbra Blades*
  - **Removed:*** No longer deals 50% reduced damage against minions that are the primary target.

### V12.10
- Stats
  - Base health increased to 655 from 585.
  - Health growth increased to 109 from 95.
  - Armor growth increased to $4.7$ from $3.5$.
  - Magic resistance growth increased to $1.55$ from $0.75$.
- *Umbra Blades*
  - Base heal reduced to 13 to 30 from 15 to 40.
  - Heal AP ratio increased to 30% AP from 15% AP.

## Trivia

- *Nocturne* comes from the French *nocturne*, which is derived from the Latin *nocturnus* "of the night".
- *Umbra Blades* is named after Latin *umbra* for shade or shadow.
- Nocturne was the first champion to be previewed at a major event prior to release (Penny Arcade Expo East 2011).
  - He was also previewed in IGN.

---
*This page was automatically generated from League of Legends Wiki data.*