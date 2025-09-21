# Akali

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
| **Champion** | Akali |
| **Title** | the Rogue Assassin |
| **Resource** | Energy |
| **Range Type** | Melee |
| **Release Date** | 2010-05-11 |
| **Release Patch** | V1.0.0.85 |
| **Roles** | Assassin |
| **Riot Positions** | Top, Middle |
| **External Positions** | Top, Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $600.0$ | $+119.0$ | $2623.0$ |
| **Energy** | $200.0$ | $+0.0$ | $200.0$ |
| **Health Regen** | $9.0$ | $+0.9$ | $24.3$ |
| **Energy Regen** | $50.0$ | $+0.0$ | $50.0$ |
| **Armor** | $23.0$ | $+4.7$ | $102.9$ |
| **Magic Resist** | $37.0$ | $+2.05$ | $71.8$ |
| **Attack Damage** | $62.0$ | $+3.3$ | $118.1$ |
| **Attack Speed** | $0.625$ | $+3.2\%$ | $0.965$ |
| **Movement Speed** | $345.0$ | $+0.0$ | $345.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $3.2\%$ |
| **Acquisition Radius** | $400 units$ |
| **Pathing Radius** | $30 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $138.889 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Assassin's Mark

**Innate:** **Akali**’s damage with abilities against a champion create a around them for a few seconds, and she gains *move speed* while moving away from the ring.

*Once **Akali** exits the ring, she regains the *move speed* towards enemy champions and empowers her next basic attack with *range **bonus** range* and magic damage.*

**Innate:** When **Akali** damages an enemy champion with an ability, she creates a around them for 4 seconds, refreshing on subsequent damaging abilities against champions. For 2 seconds, she gains key=% while moving away from the center of the ring. Only one may be active at a time. When **Akali** exits the ring, for 2 seconds, she regains the **bonus** movement speed while facing nearby enemy champions and becomes empowered with *Swinging Kama* for 4 seconds, during which she cannot create another ring. **Swinging Kama:** ''Akali's** next basic attack is empowered to have its *range* doubled and deal 35 to 53 for 7–then + 15*x (+ 60% *bonus AD) (+ 55% AP) **bonus'' magic damage.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 500 units |
| **Spell Shield** | False |
| **Spell Effects** | spell |

**Notes:**

- Enemies cannot see the ring, but can see the empower effect.
- *Swinging Kama* gives a static **bonus** range which doesn't change if ''Akali's' range changes during it (i.e *Rapid Firecannon*):
  - If *Rapid Firecannon* fully charges within 'Swinging Kama's* duration, *'Akali's' total range increases to 337.51.35.
  - If *Rapid Firecannon* fully charges before gaining *Swinging Kama*, ''Akali's' total range is increased to 3951.35 + 125×1.35 (which is the bonus range Swinging Kama gives)×1.35 (further increased by Rapid Firecannon).
- The will be created around the last target hit by **Five Point Strike**.
- The ring's center is offset 120 units away from the enemy's center, towards Akali.
- The empowered attack can be dodge and block, and will be mitigated by blind.
- The empowered attack will trigger but not be consumed nor apply its effects against structures and wards.
- This ability's damage is calculated based on the caster's stats at the time of its application.

---

### Q: Five Point Strike

**Active:** **Akali** throws out a cone of kunai that deals magic damage to enemies hit. Targets hit at maximum range are briefly slow.

**Active:** **Akali** unleashes kunai in a cone in the target direction, dealing magic damage to enemies hit. Targets beyond a certain range are also slow by 50% for $0.5$ seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 500 / 120 units |
| **Cooldown** | $1.5$ seconds |
| **Cast Time** | 0.25@1; 0.225@6; 0.2@11; 0.175@16 seconds |
| **Cost** | $110-70$ Energy |
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $45-145$ (+ 65% AD)

**Notes:**

- *Five Point Strike* will hit enemies after only 0.25×0.75 to 0.175×0.75 for 4 seconds (75% of cast time) into the cast time.
  - Due to any actions only starting on integer game ticks, the effective cast time rounds up to 0.264@1; 0.231@6; 0.231@11; 0.198@16 and the hit will occur at 0.198@1; 0.198@6; 0.165@11; 0.132@16 seconds into it.
- This ability's damage is calculated based on the caster's stats at the time of its initial application and does not change dynamically. Effect at cast time end

---

### W: Twilight Shroud

**Active:** **Akali** restores and gains a brief burst of ms. She also drops a smoke shroud that makes her invisibility, attacking or using abilities will briefly reveal her.

*The shroud will expand into a over a few seconds, during which **Akali** gains increased energy.*

**Active:** **Akali** restores *100 energy* over $0.4$ seconds and gains *ms **bonus** movement speed* that decays over 2 seconds. She also detonates a smoke bomb a fixed distance away in the target direction, creating a circular shroud that expands over the next 5 seconds into a ring. The shroud does not permeate terrain, and will expand toward nearby enemy champions. While the shroud is active, ''Akali's** **maximum'' energy is increased by 100. Entering the shroud renders **Akali** invisibility, unless she is dash. Declaring a basic attack or casting an ability will break the invisibility and prevent **Akali** from entering it for type=minutes seconds, refreshing on subsequent attacks and casts. 'The *marked* section of Twilight Shroud will linger for the mark's duration, even after the shroud ends.'**Akali** can move during Twilight Shroud's cast time.'

| Attribute | Value |
|-----------|-------|
| **Range** | 250 units |
| **Cooldown** | $20-16$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Effect Radius** | 350 / 1175 units |

**Scaling:**
- **Bonus Movement Speed:** $30-50$%
- **Shroud Duration:** $5-7$ seconds

**Notes:**

- **Akali** will also not gain the invisibility if she is Recall.
- It is possible for **Akali** to detonate the smoke bomb on the far of terrain from her current location.
- 'Twilight Shroud's effects begin at the start of the cast time.
- Using a basic attack breaks the stealth at the start of the attack windup.

---

### E: Shuriken Flip

**Active:** **Akali** dashes backward and throws a shuriken forward that deals magic damage and marks the first enemy or *smoke cloud* hit for a short time.

*While *Shuriken Flip* marks a target, it can be recast.*

**Active:** **Akali** dash backward and, after the cast time, throws a shuriken in the target direction that deals magic damage to the first enemy hit. The shuriken *marks* and true sight the enemy or the last **smoke** section hit for 3 seconds, during which *Shuriken Flip* can be recast to consume the mark. **Recast:** **Akali** dash towards the marked target or smoke section. Against enemies she deals magic damage upon arrival. '**Akali** will not flip backwards if she is immobilized or grounded during the cast time. *Twilight Shroud* and *Perfect Execution* can be cast during the recast's dash.'

| Attribute | Value |
|-----------|-------|
| **Range** | 825 units |
| **Cooldown** | $16-10$ seconds |
| **Cast Time** | $0.4$ / $0.25$ seconds |
| **Cost** | 30 energy |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | magic |
| **Speed** | 1800 / 1500 units/second |
| **Effect Radius** | Global |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $70×0.3-350×0.3$; pass-through: $70×0.3-350×0.3$ × 0.3 (+ $100×0.3$% bonus AD)
- **Magic Damage:** $70×0.7-350×0.7$ (+ 100% AD); pass-through: $70×0.7-350×0.7$ × 0.7 (+ $100×0.7$% bonus AD)

**Notes:**

- **Akali** will track the target if they change locations.
  - She will not stop tracking the target until she reaches them, as there is no maximum tracking distance.
- Both casts count as ability activations for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- *Shuriken Flip* does not interrupt ''Akali's' previous move or attack orders.
  - The recast does.
- *Shuriken Flip* cannot be recast if the target is untargetable or is in another realm.
- **Akali** will be ordered to basic attack the target after the recast's dash ends.
- **Akali** will dash backwards up-to 400 units in a straight line. This dash can cross terrain if the end point is beyond it. If she would end the dash inside terrain, she will instead look for a location in either direction left or right that is outside of terrain and dash there. If there is no such location, she will dash only up to the wall at her normal speed, ending the dash early. Effect at cast time end
  - The shuriken missile will fire from wherever **Akali** is at the end of the cast time (usually mid-dash), but always towards and reaching the location that was 825 units in front of her at the start of cast. The only exception is if she moves via Flash, in which case the range of the missile is decreased relative to the distance she blinked.
- Killing an enemy with the shuriken will still mark the last area of the *shroud* hit.
- Attempting to cast *Five Point Strike* during either dash will buffer it to cast as soon as the dash ends.
- This ability's damage is calculated based on the caster's stats at the time of its application.

---

### R: Perfect Execution

**Active:** **Akali** dashes in the direction of the target enemy champion, dealing physical damage to enemies she passes through.

*Perfect Execution* can be recast within a period.

**Active:** **Akali** dash 750 units in the direction of the target enemy champion, dealing magic damage to enemies she passes through. If this hits an enemy, she flips over them to continue the dash up to the normal range but for at least another 150 units. *Perfect Execution* can be recast after a $2.5$-second static cooldown within 10 seconds of the first activation. **Recast:** **Akali** dash 800 units in the target direction, dealing magic damage to enemies she passes through, increased by key=%.

| Attribute | Value |
|-----------|-------|
| **Range** | 675 units |
| **Cooldown** | $120-60$ seconds |
| **Cast Time** | None |
| **Targeting** | Unit / Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1500 / 3000 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Magic Damage:** $110-330$
- *bonus AD) (+ 30% AP)
- **Minimum Magic Damage:* $70-210$ (+ 30% AP)3-210×3$ (+ $30×3$% AP)

**Notes:**

- With the first cast, **Akali** can dash up to a maximum of 900 units if the first enemy she hit was at the maximum dash range.
- Casting Flash during either dashes will interrupt them, but **Akali** will deal damage to any enemies she comes in contact with at the new location.
  - Enemies already hit by either of 'Perfect Execution's casts cannot be affected more than once by the same cast.
- *Shuriken Flip* can be used during the initial dash if it is cast after ''Perfect Execution's' cast time finishes.
- This ability's damage is calculated based on the caster's stats at the start of cast and does not change dynamically.
- **Akali** is lockout to cast her other abilities for $0.25$ seconds after starting the initial cast's dash.

---

## Patch History

### V25.12
- *Assassin's Mark*
  - **Bug Fixes:** Ring indicator no longer sometimes appears around the incorrect characters.

### V25.11
- *Five Point Strike*
  - **Bug Fixes:** The ability's collision area is now properly offset based on her cast direction instead of incorrectly always being offset toward the north of the map. Previously, this would cause the ability to hit targets further when targeted parallel to or at an approximate angle to the map's north axis.

### V25.08
- *Twilight Shroud*
  - **Bug Fixes:** Smoke piece particles no longer layer incorrectly and flicker while on elevated terrain.

### V25.07
- *Perfect Execution*
  - **Undocumented / Bug Fix:** First cast is no longer able to expose (via the indicator) and cast on **Neeko** *disguised* as a non-champion.

### V14.18
- General
  - **Bug Fixes:** Certain voiceover lines will now properly trigger to play.

### V14.15
- *Twilight Shroud*
  - Cooldown reduced to $20-16$ seconds from 20 at all ranks.
- *Perfect Execution*
  - First cast base damage changed to $110-330 3$ from $80-360 3$.
  - Second cast minimum base damage increased to $70-210 3$ from $60-200 3$.
    - Second cast maximum base damage increased to $70×3-210×3 3$ from $60×3-200×3 3$.
  - Cooldown increased to $120-60 3$ seconds from $100-60 3$.

### V14.12
- *Shuriken Flip*
  - Shuriken base damage reduced to $21-105$ from $30-135$.
  - Shuriken AD ratio increased to 30% AD from $25.5$% AD.
  - Shuriken AP ratio reduced to 33% AP from 36% AP.
  - Dash base damage reduced to $49-245$ from $70-315$.
  - Dash AD ratio increased to 70% AD from $59.5$% AD.
  - Dash AP ratio reduced to 77% AP from 84% AP.
- *Twilight Shroud*
  - **Bug Fixes:** No longer restores 200 energy instead of 100 upon cast.

### V14.8
- Stats
  - Base health increased to 600 from 570.

### V14.2
- *Assassin's Mark*
  - **New Effect:** Empowered attack can no longer expire during its windup against a target.
  - **Bug Fixes:** No longer becomes unable to cast abilities for about $1.5$ seconds if she uses the empowered attack at the last second.

### V13.16
- *Five Point Strike*
  - Base damage increased to $45-145$ from $40-140$.
  - Energy cost reduced to $110-70$ from $130-70$.

## Trivia

- Akali's dance referenced Single Ladies by Beyoncé.
  - A side-by-side comparison can be seen here.
- Akali was the only energy-based champion to have any associated cost on their *ultimate* (one Essence of Shadow).
- *Shadow Dance* was going to be a skillshot but became single-targeted instead.
- Akali used to gain the 'Law of Inverse Ninja Strength' cosmetic Easter egg debuff ('"This unit is a flippin' ninja!"* - *"Ninjas are more effective when they work alone. For every Ninja on your team beyond yourself, you lose 1 health."') when she, **Kennen**, **Shen**, and/or **Zed** found themselves on the same team. It was removed in V3.14 for unknown reasons.
- In the V1.0.0.115 April Fools' Day patch, the following change regarding Akali was jokingly listed：
  - Akali is no longer a ninja.
- Due to Perception of English /r/ and /l/ by Japanese speakers, *Akali* may sound identical to Japanese 明かり*akari* "light, brightness, gleam", from Old Japanese stative verbal root **aka(-r-)* "to be red, to be bright" (> *aka(-i)* 赤(い) "red")
- Akali is the second champion, after **Yasuo**, to have an emote that will orient itself the same (facing towards the camera), no matter what direction Akali was facing.
  - However, she is the first to always reposition herself to face the camera when using the emotes, and has this function on both her dance and joke emotes.
- Akali's Series 1 Eternals make the following references:
  - *Check Marks* references the check mark.
  - *Like a Ninja* describes *Shuriken Flip*’s mobility and use of shuriken, two features common in depictions of ninja in popular culture.

---
*This page was automatically generated from League of Legends Wiki data.*