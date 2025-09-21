# Lux

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
| **Champion** | Lux |
| **Title** | the Lady of Luminosity |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2010-10-19 |
| **Release Patch** | V1.0.0.103 |
| **Roles** | Burst, Artillery |
| **Riot Positions** | Middle, Support |
| **External Positions** | Middle, Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $580.0$ | $+99.0$ | $2263.0$ |
| **Mana** | $480.0$ | $+23.5$ | $879.5$ |
| **Health Regen** | $5.5$ | $+0.55$ | $14.9$ |
| **Mana Regen** | $7.0$ | $+0.8$ | $20.6$ |
| **Armor** | $21.0$ | $+5.2$ | $109.4$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $54.0$ | $+3.3$ | $110.1$ |
| **Attack Speed** | $0.669$ | $+3.0\%$ | $1.010$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $550.0$ | $+0.0$ | $550.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.669$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $3.0\%$ |
| **Missile Speed** | $1600 units/second$ |
| **Acquisition Radius** | $625 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $85 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Illumination

**Innate:** **Lux**’s abilities apply a mark to enemies hit for a few seconds. Her basic attacks and **Final Spark** consume the mark to deal **bonus** magic damage.

**Innate:** ''Lux's** abilities apply a mark to enemies hit for 6 seconds, refreshing on subsequent hits. **Lux's* basic attacks and **Final Spark*' consume the mark to deal 30 to 200 (+ 30% AP) **bonus** magic damage.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Proc |

**Notes:**

- Starting an attack windup against a target with a mark that is about to expire refreshes its duration to $0.25$ seconds. Casting **Final Spark** refreshes the marks of all marked enemies to $1.25$ seconds if they are within 5000 units of the ability's casting position and have a mark with a remaining duration of less than 1 second.
- Spell shield prevents *Final Spark* from consuming the mark.
  - The on-hit damage from *Illumination* via basic attack cannot be blocked by *spell shield*.
- 'Illumination's trigger from a basic attack can be blocked (bonus damage is negated and the mark will still be consumed).
- : Parry interactions (dodge, blind).

---

### Q: Light Binding

**Active:** **Lux** shoots a sphere of light in the target direction that deals magic damage to the first two enemies hit and root them for a short time.

**Active:** **Lux** shoots a sphere of light in the target direction that deals magic damage to the first two enemies hit and root them for 2 seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $11-9$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 50 Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1200 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $80-240$ (+ 65% AP)

**Notes:**

- Targets immune to the root still count towards 'Light Binding's two-target limit. Effect at cast time end
- *Light Binding* spawns an additional missile at the same location and vector as the main missile. This additional "Dummy" missile has the same visual effects as the main missile and gets destroyed when it hits a single enemy, thereby making the surviving VFX of the single missile weaker in intensity.
  - This hacky solution also causes Lux Q to count as two skillshots for things such as "skillshots dodged" Eternals.

---

### W: Prismatic Barrier

**Active:** **Lux** throws her wand in the target direction that homes back to her after reaching maximum range.

*Allied champions hit by the wand gain a shield for a short time, which can stack up to twice. **Lux** receives the shield upon throwing the wand and upon its return.*

**Active:** **Lux** throws her wand in the target direction that homes back to her after reaching maximum range. Allied champions hit by the wand gain a shield for $2.5$ seconds, which can stack up to 2 times, stacking with the previous shield and refreshing its duration. **Lux** gains the shield upon throwing and upon retrieving the wand.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $14-10$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-80$ Mana |
| **Targeting** | Direction |
| **Affects** | Allies |
| **Speed** | 2400 units/second |
| **Effect Radius** | 110 units |
| **Projectile** | Special |

**Scaling:**
- **Shield Strength:* $40-100$ (+ 40% AP)2-100×2$ (+ $40×2$% AP)

**Notes:**

- Each pass of the wand grants a shield to each target once.
- *Prismatic Barrier* may only stack up to 2 times. Subsequent instances of passing through the wand have no effect on the shield's strength nor duration.
- The shield will also be granted to allies near the wand's return location.
- Both passes of *Prismatic Barrier* cannot hit units whose center range is beyond the missile range or behind its origin, even if their edge range overlaps.
  - At the return location, this behaviour is overridden by an additional center check with the same diameter as the missile width.
- If **Lux** death before her wand returns it will fizzle upon reaching maximum range.
- There is an additional circle check at the end of the missile's length that allows it to hit the edge of an ally's radius. This is intended.
- ''Lux's' wand decelerates on the way out and accelerates on the way back. Effect at cast time start
- *Prismatic Barrier* is destroyed by *Wind Wall* and *Blade Whirl* but not *Unbreakable*.

---

### E: Lucent Singularity

**Active:** **Lux** sends a lucent singularity to the target location that lasts for a few seconds and slows enemies within.

*Lucent Singularity* can be recast within this time, and does so automatically after the duration.

**Active:** **Lux** sends a lucent singularity to the target location, remaining there for 5 seconds to grant sight of the area and slow nearby enemies. *Lucent Singularity* can be recast at any time while it is in flight or within the duration, and does so automatically after the duration. **Recast:** **Lux** detonates the singularity, dealing magic damage to enemies within. If *Lucent Singularity* was recast while in flight, it will detonate upon arrival. The slow lingers for 1 second after leaving the area. Enemies hit by 'Lucent Singularity's detonation are also slowed by the same amount for 1 second.

| Attribute | Value |
|-----------|-------|
| **Range** | 1100 units |
| **Cooldown** | $10-8$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-110$ Mana |
| **Targeting** | Location / Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1200 units/second |
| **Effect Radius** | 310 / 650 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**Scaling:**
- **Slow:** $25-45$%
- **Magic Damage:** $65-265$ (+ 80% AP)

**Notes:**

- The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
  - The manual recast does not.
- *Lucent Singularity* grants sight while also in flight.
- The ability will not preserve the caster's facing direction when using Flash and similar effects.

---

### R: Final Spark

**Active:** **Lux** fires a massive light beam in a line in the target direction that deals magic damage and briefly standard sight enemies hit.

**Active:** **Lux** fires a massive light beam in a line in the target direction that deals magic damage to enemies hit and standard sight them for $1.5$ seconds, as well as grants sight of the surrounding area.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $60-40$ seconds |
| **Cast Time** | 1 seconds |
| **Cost** | 100 Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** $300-500$ (+ 120% AP)

**Notes:**

- *Final Spark* grants sight of its surroundings during the cast time and for $0.5$ seconds afterwards. Effect at cast time start
- The ability will not preserve the caster's facing direction when using Flash and similar effects.

---

## Patch History

### V14.10
- General
  - **Bug Fixes:** Restored Recall winddown animation for all skins.
  - **Bug Fixes:** Restored Respawn animation for all skins.

### V14.9
- *Lucent Singularity*
  - **Bug Fixes:** Vision bubble is no longer inconsistent inside terrain, causing some targets to remain undetected in its radius.

### V14.7
- *Illumination*
  - AP ratio increased to 30% AP from 25% AP.
- *Light Binding*
  - AP ratio increased to 65% AP from 60% AP.

### V14.2
- *Light Binding*
  - **Bug Fixes:** No longer has a very rare chance to break a spell shield that was added right after it applied its full effects already.
    - Width of the extra dummy VFX missile is now identical to the functionally relevant one.
- Stats
  - Base health increased to 580 from 560.
  - Base armor increased to 21 from 19.
  - Attack speed growth increased to 3% from 2%.
- *Prismatic Barrier*
  - AP ratio per hit increased to 40% AP from 35% AP.

### V13.17
- Stats
  - Base mana regeneration reduced to 7 from 8.
- *Illumination*
  - Base damage increased to 30 to 200 from 20 to 190.
  - AP ratio increased to 25% AP from 20% AP.
- *Light Binding*
  - Cooldown reduced to $11-9$ seconds from 11 at all ranks.
- *Lucent Singularity*
  - Base damage reduced to $65-265$ from $70-270$.

### V13.14
- Lux and Lux
  - **Bug Fixes:** Staff no longer resets position while using the *Taunt* emote in a loop transition.
- Lux
  - *Light Binding*
    - **Bug Fixes:** Orb no longer renders behind a turret.
  - *Prismatic Barrier*
    - **Bug Fixes:** VFX trail no longer renders behind environment objects.
- *Lucent Singularity*
  - AP ratio increased to 80% AP from 70% AP.
- *Final Spark*
  - AP ratio increased to 120% AP from 100% AP.

### V12.14
- *Light Binding*
  - Root buff renamed to *Light Binding* from *Prismatic Seal*.
- *Prismatic Barrier*
  - Shield buff renamed to *Prismatic Barrier* from *Stardust Barrier*.

### V12.10
- Stats
  - Base health increased to 560 from 490.
  - Health growth increased to 99 from 85.
  - Armor growth increased to $5.2$ from 4.
  - Magic resistance growth increased to $1.3$ from $0.5$.
- *Prismatic Barrier*
  - Base shield per hit reduced to $40-100$ from $45-125$.

### V12.6
- *Lucent Singularity*
  - **Bug Fixes:** Missile is no longer blocked by spell shield.
- *Light Binding*
  - Cooldown increased to 11 seconds at all ranks from $11-9$.
  - Base damage reduced to $80-240$ from $80-260$.

### V11.21
- Stats
  - Attack speed growth increased to 2% from 1%.
- *Illumination*
  - **New Effect:** Starting an attack windup against a target with a mark that is about to expire refreshes its duration to $0.25$ seconds.
  - **New Effect:** Casting *Final Spark* refreshes the marks of all marked enemies to $1.25$ seconds if they are within a certain distance of the ability's casting position and have a mark with a remaining duration of less than 1 second.
- *Lucent Singularity*
  - Base damage increased to $70-270$ from $60-260$.
  - AP ratio increased to 70% AP from 65% AP.
- *Final Spark*
  - Cooldown reduced to $60-40 3$ seconds from $80-40 3$.

## Trivia

- *Luxanna* comes from Latin word for light, lux.
- Lux's ultimate has had three names, all referencing from Touhou Project.
- #*Finales Funkeln* (German: 'final sparkle') (V1.0.0.103 - V1.0.0.144)
- # *Infinite Light* (V1.0.0.144 - V1.0.0.145)
- # *Final Spark* (V1.0.0.145 - present, references Marisa Kirisame's Last Spell in Imperishable Night)
- #* *Final Spark* also references by from Dragon Ball Z.
- *The Lady of Luminosity* might come from Luminance Fräulein ("Lady" in German) (an ability name during production).
- Lux, **Ezreal**, and **Katarina** are the only champions to be visually redesigned between being announced and being released (Lux's was conceived by NA Summoner 'Katertot' aka Katie 'TeaTime' De Sousa).
- Lux - **Garen** is one of seven pairs of sibling champions (the others being **Cassiopeia** - **Katarina**, **Kayle** - **Morgana**, **Nasus** - **Renekton**, **Yasuo** - **Yone**, **Darius** - **Draven**, and **Vi** - **Jinx**).
  - Though not a pair, **Anivia**, **Ornn**, and **Volibear** are also siblings.
- The ward skin Luminosity Ward.png is her classic staff.
- Lux's Series 2 Eternals make the following references:
  - *Double Rainbow* is a reference to Lux's joke interaction based on Yosemitebear62's viral video.
  - *Ultraviolent* is a name puns on "Ultra Violet" and "Violence".

---
*This page was automatically generated from League of Legends Wiki data.*