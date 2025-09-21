# Karthus

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
| **Champion** | Karthus |
| **Title** | the Deathsinger |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2009-06-12 |
| **Release Patch** | June 12, 2009 Patch |
| **Roles** | Battlemage |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $620.0$ | $+110.0$ | $2490.0$ |
| **Mana** | $467.0$ | $+31.0$ | $994.0$ |
| **Health Regen** | $6.5$ | $+0.55$ | $15.9$ |
| **Mana Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Armor** | $21.0$ | $+4.7$ | $100.9$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $46.0$ | $+3.25$ | $101.2$ |
| **Attack Speed** | $0.625$ | $+2.1\%$ | $0.849$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $450.0$ | $+0.0$ | $450.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.1\%$ |
| **Missile Speed** | $1500 units/second$ |
| **Acquisition Radius** | $450 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $150 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Death Defied

**Innate:** Upon death, **Karthus** enters an untargetable Zombie state for a few seconds that allows him to continue casting spells at no cost but without being able to move.

*If **Defile** has been learned, it will remain toggled on for 'Death Defied's entire duration.*

**Innate:** Upon taking death, **Karthus** enters a **zombie state** for 7 seconds, during which he can cast his abilities at no cost. If **Defile** has been learned, it will remain toggled on for 'Death Defied's* entire duration. **Requiem*' becomes disabled after 4 seconds of the duration have elapsed. While under this state, **Karthus** becomes untargetable and cc-immune as well as prevents all incoming damage, but is also rendered unable to move, declare basic attacks, use summoner spells, and activate items.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- 'Death Defied's untargetability does not homing projectile destruction in-flight projectiles.
- **Karthus** cannot be execute by the Aspect of the Dragon during *Death Defied*.
- At the start of *Death Defied*, **Karthus** is set to health.
  - **Karthus** can still regenerate his health over the duration, but he will always die at the end of it.
- After *Death Defied* ends, the corpse of **Karthus** will retain unit collision despite being dead on the ground.
- ''Karthus'* mana bar drains over the duration of *Death Defied' as an indicator of his time remaining in this state.

---

### Q: Lay Waste

**Active:** **Karthus** conjures a delayed blast at the target location that deals magic damage to all enemies within, doubled when only one target is struck.

**Active:** **Karthus** conjures a blast at the target location that detonates after See notes, granting sight of the area and dealing magic damage to all enemies within, doubled when only one target is struck.

| Attribute | Value |
|-----------|-------|
| **Range** | 875 units |
| **Cooldown** | 1 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $20-40$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 160 units |
| **Spell Shield** | True |
| **Spell Effects** | Special |

**Scaling:**
- **Magic Damage:* $40-116$ (+ 35% AP)2-116×2$ (+ $35×2$% AP)

**Notes:**

- The delay between the cast and the detonation is inconsistent, but always matches up with the VFX.
  - *Defile*’s ticks beyond the first has a similar issue.
- Applies spell damage if it hits a single target and area damage if it hits multiple targets.
  - *Lay Waste* also plays a different sound effect when it only hits one target.

---

### W: Wall of Pain

**Active:** **Karthus** erects a wall of pain at the target location that lasts a few seconds, enemies that touch it suffer reduced magic penetration and become slow for a few seconds.

**Active:** **Karthus** erects a wall of pain at the target location perpendicular to his facing that lasts 5 seconds, granting sight around its pillars and center. Enemies that touch the wall are inflicted with magic penetration and become slow for 5 seconds, decaying over the duration. This can affect enemies only once per cast.

| Attribute | Value |
|-----------|-------|
| **Range** | 1000 units |
| **Cooldown** | 15 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 70 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Spell Shield** | False |

**Scaling:**
- **Wall Length:** $800-1200$
- **Slow:** $40-80$% Reduced Slow $40/2-80/2$%

**Notes:**

No additional notes.

---

### E: Defile

**Passive:** **Karthus** restores *mana* whenever he kills an enemy.

**Toggle:** **Karthus** surrounds himself in a necrotic aura that continually deals magic damage to all nearby enemies, quickly draining his own *mana*.

**Passive:** **Karthus** restores *mana* whenever he kills an enemy. **Toggle:** **Karthus** surrounds himself in a necrotic aura that deals magic damage every $0.25$ seconds to all nearby enemies. Toggling *Defile* off triggers a final tick of damage. *Defile cannot be toggled off during *Death Defied*.*

| Attribute | Value |
|-----------|-------|
| **Cast Time** | none |
| **Cost** | $30-78$ Mana per second |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 550 units |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |

**Scaling:**
- **Mana Restored:** $10-50$
- **Magic Damage Per Tick:** $30/4-110/4$ (+ 5% AP) Damage Per Second $30-110$ (+ 20% AP)

**Notes:**

- *Defile* will restore mana if **Karthus** destroys an enemy structure.
- *Defile* re-calculates its damage with each tick based on changes in ''Karthus's* ability power or an increase in the rank of *Defile' without the need to toggle it off and on again.
- *Defile* will toggle off automatically if **Karthus** enters resurrection.

---

### R: Requiem

**Active:** **Karthus** channel for a short time, then deals magic damage to all targetable enemy champions upon completion.

**Active:** **Karthus** channel for 3 seconds, then deals magic damage to all targetable enemy champions upon completion.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $200-160$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | Global |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |

**Scaling:**
- **Magic Damage:** $200-500$ (+ 70% AP)

**Notes:**

- 'Requiem's channel is telegraphed to all enemy champions by a beam of light that descends on all targets, including if they are untargetable or even death.
- *Requiem* will affect clone.
- The damage will apply spell effects to targets in order of being spawned. Single target spell effects (such as *Luden's Companion*) are thus triggered on the leftmost champion on the loading screen.
- *Requiem* will not affect **Neeko** if she's *disguised* as a non-champion if **Karthus** is affected by *Death Defied*.
- The following table refers for interactions while **Karthus** is channel:

---

## Patch History

### V25.18
- *Wall of Pain*
  - **Bug Fixes:** Wall VFX now properly layers over terrain that is not at default elevation. Previously, the VFX could be completely hidden if the VFX coincided entirely with those sections of the map.

### V14.24
- *Lay Waste*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.21
- *Requiem*
  - AP ratio reduced to 70% AP from 75% AP.

### V14.15
- *Death Defied*
  - **Bug Fixes:** Restored death voice lines after *Death Defied* expires.

### V14.14
- *Death Defied*
  - **Bug Fixes:** Is no longer sometimes unable to issue movement commands after the zombie state has expired.

### V14.13
- *Death Defied*
  - **Bug Fixes:** Corpse is no longer sometimes a valid movement collision target.
- *Lay Waste*
  - Base damage reduced to $40-116$ from $43-119$.
    - Single-target damage reduced to $40×2-116×2$ from $43×2-119×2$.

### V14.11
- *Lay Waste*
  - Base damage reduced to $43-119$ from $45-125$.
    - Single-target damage reduced to $43×2-119×2$ from $45×2-125×2$.
- *Wall of Pain*
  - **Bug Fixes:** Magic resistance reduction while in *zombie form* no longer multiplies the magic resistance of enemies hit by 25 instead of $0.75$.
- *Requiem*
  - **Bug Fixes:** Now properly benefits from ability haste as soon as he respawns.

### V14.2
- *Lay Waste*
  - Base damage increased to $45-125$ from $43-115$.
    - Single-target damage increased to $90-250$ from $86-230$.
- *Wall of Pain*
  - Magic resistance reduction increased to 25% from 15%.

### V13.22
- Stats
  - Basic attack missile speed increased to 1500 from 1200.

### V13.19
- *Death Defied*
  - **Bug Fixes:** No longer deals damage to his allies with *Defile* when the effect was triggered by taking lethal damage from an enemy **Briar** in her *Blood Frenzy* state.

## Trivia

- The name *Karthus* resembles Medieval Latin Carthusian Order.
- Karthus is voiced.md) by Adam_Harrington_(voice_actor), who also voices **Kassadin**, **Mordekaiser**, **Ryze**, and **Shaco**.
- During development he was called *Lich*.
- Requiem old.png was likely named after Requiem Mass referencing Karthus' singing for/about the dead.
- His Karthus made a cameo in Fright Night (2011 film) along with Fiddlesticks.
- A glass case reading **'Press *R** to Win'* can be seen in the game's Mac Version launch trailer.
- Before 2016, Karthus had been disabled for every single Ultra Rapid Fire edition (most likely to prevent 'Spam *R* to Win' abuse cases).
- For a brief time in Season 2015, Karthus had a bug where he could remain in *Death Defied* indefinitely while alive and held an 87% win rate.

---
*This page was automatically generated from League of Legends Wiki data.*