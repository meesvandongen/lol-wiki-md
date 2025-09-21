# Xerath

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
| **Champion** | Xerath |
| **Title** | the Magus Ascendant |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-10-05 |
| **Release Patch** | V1.0.0.126 |
| **Roles** | Artillery |
| **Riot Positions** | Middle, Support |
| **External Positions** | Middle, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $596.0$ | $+106.0$ | $2398.0$ |
| **Mana** | $400.0$ | $+22.0$ | $774.0$ |
| **Health Regen** | $5.5$ | $+0.55$ | $14.9$ |
| **Mana Regen** | $6.85$ | $+0.8$ | $20.5$ |
| **Armor** | $22.0$ | $+4.7$ | $101.9$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $55.0$ | $+3.0$ | $106.0$ |
| **Attack Speed** | $0.658$ | $+1.4\%$ | $0.810$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $525.0$ | $+0.0$ | $525.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.658$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.4\%$ |
| **Missile Speed** | $2000 units/second$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $302.778 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Mana Surge

**Innate:** Periodically, **Xerath**’s next basic attack restores *mana*, doubled against an enemy champion.

*Killing enemies ah this ability's *cooldown*.*

**Innate:** Periodically, **Xerath** empowers his next basic attack to restore mana, doubled to 60–390 against enemy champions. 'Mana Surge's *cooldown* is reduced by $3.5$ seconds whenever **Xerath** kills an enemy. *Mana Surge does not trigger if **Xerath** would restore above his **maximum** mana.*

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- *Mana Surge* will trigger even if the attack is blocked.
- The empowered attack will trigger against structures.
- : 'Mana Surge's ' interactions with dodge, and blind effects.

---

### Q: Arcanopulse

**Active:** **Xerath** channel up to a short time, increasing the *range* of his beam.

*Arcanopulse* can be recast within the duration and will cancel automatically afterwards, refunding half the mana.

**Active:** **Xerath** channel while being slow by key=%5% per 0.25 seconds channeled, capped at 40% for up to 3 seconds to increase 'Arcanopulse's *range* over the first $1.5$ to $1.75$ seconds of the channel. *Arcanopulse* can be recast within the duration. If the charge is interrupt or completes without reactivation, *Arcanopulse* is cancelled and refunds mana. **Recast:** **Xerath** becomes lockout for and afterwards fires a beam of energy in a line in the target direction that deals magic damage to enemies hit.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $9-5$ seconds |
| **Cast Time** | none |
| **Cost** | $80-120$ mana |
| **Targeting** | Auto / Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Scaling:**
- **Mana Refunded:** $40-60$
- **Magic Damage:** $75-235$ (+ 90% AP)

**Notes:**

- The onset of 'Arcanopulse's self-slow and first range increase is at a random interval of 0 to $0.25$ seconds. The real time to maximum self-slow and range is thus $1.5$ to $1.75$ seconds.
- The rectangle hitbox's minimum range is er 0, right through ''Xerath's' center.
- *Arcanopulse* will not fire and deal damage if **Xerath** dies during the recast delay.
- The following table refers for interactions while **Xerath** is channel:
- The following table refers for interactions while **Xerath** is locked out during the recast delay:

---

### W: Eye of Destruction

**Active:** **Xerath** casts down a blast of energy that strikes the target location after a brief delay, dealing magic damage and slow enemies hit. The effects are increased against enemies in the epicenter.

**Active:** **Xerath** casts down a blast of arcane energy that strikes the target location after , briefly granting sight of the area and dealing magic damage to enemies hit and slow them by 25% for $2.5$ seconds. Enemies in the epicenter take $66.7$% increased damage and are slow by a greater amount, decaying to 25% over the duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 1000 units |
| **Cooldown** | $14-10$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $80-120$ mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 275 / 125 / 250 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |

**Scaling:**
- **Magic Damage:** $50-190$ (+ 65% AP)
- **Increased Damage:** $50×1.667-190×1.667$ (+ $65×1.667$% AP)
- **Increased Slow:** $60-80$%

**Notes:**

- ''Xerath's* location gets sight to the enemy (400 radius for 4.5 seconds) if *Eye of Destruction's cast location is within 100 units of a valid enemy at the start of the cast time due to something related to targeting types.
- *Eye of Destruction*'s targeting indicator appears on the player's minimap, despite the ability not having the longer range of others with this quality.
- *Eye of Destruction* will not fire and deal damage if **Xerath** dies during the 0.25s cast time.

---

### E: Shocking Orb

**Active:** **Xerath** fires an orb of energy in the target direction that deals magic damage to the first enemy hit, stun them based on travel distance.

**Active:** **Xerath** fires an orb of energy in the target direction that deals magic damage to the first enemy hit and stuns them for type=orb travel distance seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $13-11$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-80$ mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1400 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $70-190$ (+ 45% AP)

**Notes:**

No additional notes.

---

### R: Rite of the Arcane

**Active:** **Xerath** channel up to a period, being able to fire several arcane missiles across the surrounding area.

*If *Rite of the Arcane* ends without using any recasts, half of the cooldown is cdr.*

**Active:** **Xerath** channel for up to 10 seconds, gaining the ability to recast *Rite of the Arcane* multiple times after $0.5$ seconds within the duration. If *Rite of the Arcane* ends without any of the recasts being used, half of its *cooldown* is refunded. **Recast - Arcane Barrage:** **Xerath** catapults an arcane missile that strikes the target location after a delay, briefly granting sight of the area and dealing magic damage to enemies hit. Each cast has a static cooldown of $0.5$ seconds. Hitting at least one enemy champion grants a stack of *Arcane Perfection* during the channel, stacking up to a maximum amount. Each stack increases 'Arcane Barrage's damage. **Xerath** will sight himself if a missile strikes near an enemy.

| Attribute | Value |
|-----------|-------|
| **Range** | 5000 units |
| **Cooldown** | $130-100$ seconds |
| **Cast Time** | none |
| **Cost** | 100 mana |
| **Targeting** | Auto / Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 200 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Scaling:**
- **Number of Recasts:** $4-6$
- **Magic Damage:* $170-270$ (+ 45% AP)4/22056$ (+ $45×4-45×6$% AP)
- **Maximum Stacks:** $3-5$
- **Increased Damage per Stack:** $20-30 3$ (+ 5% AP)

**Notes:**

- *Rite of the Arcane* cannot be canceled via movement commands during the first second of its channel.
- Each cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- *Arcane Barrage* will not apply its increased damage per stack to the cast that granted the stack.
  - Only successive casts of the ability will benefit from the damage increase.
- A circular indicator will be displayed at the target location to signify that the area will be blasted.
- The circle indicating the ability's range expands rapidly from near **Xerath** up to the maximum range over 1 second during the channel.
  - The maximum-range indicator is visible to both allies and enemies and is color coded for friendly (blue) or hostile (red).
- *Rite of the Arcane* uses a barrage system so that it can be recast multiple times within a given period at no additional cost.
- **Xerath** will turn to face toward the direction of the target location after using a recast.
- **Xerath** gains a wider field of view during *Rite of the Arcane*.
- **Xerath** can use *Dark Passage* while channeling.
- The following table refers for interactions while **Xerath** is channel:
  - Teleport and Recall are disabled for the first $1.25$ seconds and otherwise interrupt the channel if they are used.

---

## Patch History

### V25.08
- *Mana Surge*
  - Cooldown reduction per kill increased to $3.5$ seconds from $2.5$.
- *Arcanopulse*
  - Base damage increased to $75-235$ from $70-230$.

### V25.07
- *Arcanopulse*
  - AP ratio increased to 90% AP from 85% AP.
- *Eye of Destruction*
  - Base damage reduced to $50-190$ from $60-200$.
    - Center base damage reduced to $50×1.667-190×1.667$ from $60×1.667-200×1.667$.
  - AP ratio increased to 65% AP from 60% AP.
    - Center AP ratio increased to $65×1.667$% AP from $60×1.667$% AP.
- *Shocking Orb*
  - Base damage reduced to $70-190$ from $80-200$.
- *Rite of the Arcane*
  - Base damage reduced to $170-270 3$ from $180-280 3$.
  - AP ratio increased to 45% AP from 40% AP.

### V14.19
- Xerath
  - **Bug Fixes:** Resolved VO audio being too low in certain languages due to the audio filter.

### V14.9
- Stats
  - Selection radius increased to 120 units from $88.8889007568359$.
- *Arcanopulse*, *Eye of Destruction*, *Shocking Orb*
  - **Bug Fixes:** No longer reference to his long-deprecated ammo system.
- *Eye of Destruction*
  - **Bug Fixes:** Vision bubble is no longer inconsistent inside terrain, causing some targets to remain undetected in its radius.

### V14.7
- General
  - Adjusted splash artwork for Xerath.

### V13.22
- General
  - The two basic attack animations now have a 50% chance to play instead of 75% for the first and 25% for the second.
- Stats
  - Base attack speed increased to $0.658$ from $0.625$.
  - Attack windup reduced to 20% from $25.074$%.

### V13.17
- Stats
  - Base mana regeneration reduced to $6.85$ from 8.
- *Mana Surge*
  - Cooldown refund increased to $2.5$ seconds from 2.
- *Eye of Destruction*
  - Mana cost increased to $80-120$ from $70-110$.

### V13.15
- *Rite of the Arcane*
  - **Bug Fixes:** Stacking damage buff is now properly applied to the last recast.

### V13.14
- Stats
  - Base mana reduced to 400 from 459.
- *Mana Surge*
  - Cooldown increased to 16 seconds from 12.
  - **New Effect:** Cooldown is now reduced by 2 seconds per unit killed.
  - **New Effect:** Now triggers against structures.
- *Rite of the Arcane*
  - Number of recasts increased to $4-6 3$ from $3-5 3$.
  - Base damage reduced to $180-280 3$ from $200-300 3$.
  - AP ratio reduced to 40% AP from 45% AP.
  - **New Effect:** Hitting at least one enemy champion grants a stack of Arcane Perfection during the channel, stacking up to $4-6 3$ times. Each stack increases Arcane Barrage's damage by $20-30 3$ (+ 5% AP).

### V13.4
- General
  - **Bug Fixes:** Emote SFX no longer continue to play even after the animation was interrupted halfway.

## Trivia

- When Xerath dies, his body quickly becomes unstable and overly shiny, just to blow up a moment later, much like a nuclear-powered machine.
- The icon for Xerath's ability, *Arcanopulse* is slightly similar to that of the Season 3 mastery .
- Xerath's dance is a reference "industrial dancing", which is evident in the song Pong by Eisenfunk.
  - A side-by-side comparison can be seen here.
- Xerath himself can be seen when he performs his taunt. As he disarms the parts of his sarcophagus, a person made of pure energy or electricity can be briefly seen.
  - When he dies this "person" disappears, what remains on the ground are the shattered remains of his sarcophagus.
  - It's likely that that form was his actual look before he got locked away.
- Some speculate that Xerath is ambidextrous as he uses both left and right hands for different attacks and abilities, unlike most champions who almost always use their dominant hand (usually their right one).
- Xerath, **Blitzcrank**, **Caitlyn**, **Lissandra**, **Rumble**, **Sion**, **Varus**, **Vi**, **Viego**, and **Ziggs** are the only champions who can apply crowd control on themselves.
  - Xerath's ethereal form imprisoned in a human-shapen sarcophagus also resembles Battle Realms' , consisting of an ethereal warlock inside another warlock's skeletal remains.
- Xerath features as the Ancient Ascendant in the Ascension featured game mode.

---
*This page was automatically generated from League of Legends Wiki data.*