# Akali

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
| **Champion** | Akali |
| **Title** | the Rogue Assassin |
| **Resource** | Energy |
| **Range Type** | Melee |
| **Release Date** | 2010-05-11 |
| **Release Patch** | V1.0.0.85 |
| **Latest Changes** | V25.12 |
| **Roles** | Assassin |
| **Riot Positions** | Top, Middle |
| **External Positions** | Top, Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 3 |
| **Hero Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 65 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $600.0$ | $+119.0$ |
| **Energy** | $200.0$ | $+0.0$ |
| **Health Regen** | $9.0$ | $+0.9$ |
| **Energy Regen** | $50.0$ | $+0.0$ |
| **Armor** | $23.0$ | $+4.7$ |
| **Magic Resist** | $37.0$ | $+2.05$ |
| **Attack Damage** | $62.0$ | $+3.3$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $3.2\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $30$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $138.889$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $95.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Taken** | $90.0\%$ |

## Abilities

### Passive: Assassin's Mark

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 500 (Ring radius) units |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | spell |
| **Parry** | True |

**INNATE:** When **Akali** damages an enemy champion with an ability, she creates a around them for 4 seconds, refreshing on subsequent damaging abilities against champions. For 2 seconds, she gains 30%–60%@1–16 **bonus** movement speed while moving away from the center of the ring. Only one may be active at a time.

When **Akali** exits the ring, for 2 seconds, she regains the **bonus** movement speed while facing nearby enemy champions and becomes empowered with *Swinging Kama* for 4 seconds, during which she cannot create another ring.

**SWINGING KAMA:** **Akali**’s next basic attack is empowered to have its range doubled and deal 35 to 53 for 7 / then + 9*x for 6 / then + 15*x (+ 60% **bonus** AD) (+ 55% AP) **bonus** magic damage.

**Notes:**

- Enemies cannot see the ring, but can see the empower effect.
- *Swinging Kama* gives a static **bonus** range which doesn't change if **Akali**’s range changes during it (i.e Rapid Firecannon):
  - If *Rapid Firecannon* fully charges within *Swinging Kama*’s duration, **Akali**’s total range increases to 337.5 ((125 + 125 (which is the bonus range Swinging Kama gives)).
  - If *Rapid Firecannon* fully charges before gaining *Swinging Kama*, **Akali**’s total range is increased to 395 (125.
- The will be created around the last target hit by *Five Point Strike*.
- The ring's center is offset 120 units away from the enemy's center, towards Akali.
- The empowered attack can be dodged and blocked, and will be mitigated by blinds.
- The empowered attack will trigger but not be consumed nor apply its effects against structures and wards.
- This ability's damage is calculated based on the caster's stats at the time of its application.

---

### Q: Five Point Strike

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25@1; 0.225@6; 0.2@11; 0.175@16 seconds |
| **Target Range** | 500 / 120 (Empowered range offset) units |
| **Angle** | 20° |
| **Width** | 350 (Maximum width. Estimated.) units |
| **Cost** | 110 / 100 / 90 / 80 / 70 Energy |
| **Cooldown** | $1.5$ seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Akali** unleashes kunai in a cone in the target direction, dealing magic damage to enemies hit. Targets beyond a certain range are also slowed by 50% for $0.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 45 / 70 / 95 / 120 / 145 (+ 65% AD) (+ 60% AP) |

**Notes:**

- *Five Point Strike* will hit enemies after only 0.25×0.75 to 0.175×0.75 for 4 seconds (75% of cast time) into the cast time.
  - Due to any actions only starting on integer game ticks, the effective cast time rounds up to 0.264@1; 0.231@6; 0.231@11; 0.198@16 and the hit will occur at 0.198@1; 0.198@6; 0.165@11; 0.132@16 seconds into it.
- This ability's damage is calculated based on the caster's stats at the time of its initial application and does not change dynamically. - This ability will cast from wherever the caster is at the end of the cast time.

---

### W: Twilight Shroud

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 250 units |
| **Effect Radius** | 350 (Minimum radius. Estimated.) / 1175 (Maximum radius. Estimated.) units |
| **Cooldown** | 20 / 19 / 18 / 17 / 16 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |

**ACTIVE:** **Akali** restores 100 energy over $0.4$ seconds and gains (ms) **bonus** movement speed that decays over 2 seconds. She also detonates a smoke bomb a fixed distance away in the target direction, creating a circular shroud that expands over the next 5 seconds into a ring. The shroud does not permeate terrain, and will expand toward nearby enemy champions. While the shroud is active, **Akali**’s **maximum** energy is increased by 100.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 30 / 35 / 40 / 45 / 50% |

| Attribute | Value |
|-----------|------:|
| **Shroud Duration** | 5 / 5.5 / 6 / 6.5 / 7 seconds |

Entering the shroud renders **Akali** invisible, unless she is dashing.

Declaring a basic attack or casting an ability will break the invisibility and prevent **Akali** from entering it for 1@1; 0.9@8; 0.825@11; 0.725@20; 0.625@30 (@=minutes) seconds, refreshing on subsequent attacks and casts.

*The marked section of Twilight Shroud will linger for the mark's duration, even after the shroud ends.*

***Akali** can move during Twilight Shroud's cast time.*

**Notes:**

- **Akali** will also not gain the invisibility if she is Recall.
- It is possible for **Akali** to detonate the smoke bomb on the far of terrain from her current location.
- *Twilight Shroud*’s effects begin at the start of the cast time.
- Using a basic attack breaks the stealth at the start of the attack windup.

---

### E: Shuriken Flip

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.4$ (Initial cast) / $0.25$ (Recast) seconds |
| **Target Range** | er 825 (Shuriken range, based off of Akali's original location) |
| **Effect Radius** | Global (Recast range) |
| **Width** | 120 (Shuriken missile width) units |
| **Speed** | 1800 (Shuriken missile speed) / 1500 (First and second dash speed) units/second |
| **Cost** | 30 energy |
| **Cooldown** | 16 / 14.5 / 13 / 11.5 / 10 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.5$ (Initial cast) / $0.3$ (Recast) seconds |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Akali** flips backward and, after the cast time, throws a shuriken in the target direction that deals magic damage to the first enemy hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 21 / 42 / 63 / 84 / 105 (+ 30% AD) (+ 33% AP) |

The shuriken *marks* and reveals the enemy or the last *smoke* section hit for 3 seconds, during which *Shuriken Flip* can be recast to consume the mark.

**RECAST:** **Akali** dashes towards the marked target or smoke section. Against enemies she deals magic damage upon arrival.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 49 / 98 / 147 / 196 / 245 (+ 70% AD) (+ 77% AP) |
| **Total Magic Damage** | 70 / 140 / 210 / 280 / 350 (+ 100% AD) (+ 110% AP) |

***Akali** will not flip backwards if she is immobilized or grounded during the cast time. Twilight Shroud and Perfect Execution can be cast during the recast's dash.*

**Notes:**

- **Akali** will track the target if they change locations.
  - She will not stop tracking the target until she reaches them, as there is no maximum tracking distance.
- Both casts count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- *Shuriken Flip* does not interrupt **Akali**’s previous move or attack orders.
  - The recast does.
- *Shuriken Flip* cannot be recast if the target is untargetable or is in another realm.
- **Akali** will be ordered to basic attack the target after the recast's dash ends.
- **Akali** will dash backwards up-to 400 units in a straight line. This dash can cross terrain if the end point is beyond it. If she would end the dash inside terrain, she will instead look for a location in either direction left or right that is outside of terrain and dash there. If there is no such location, she will dash only up to the wall at her normal speed, ending the dash early. - This ability will cast from wherever the caster is at the end of the cast time.
  - The shuriken missile will fire from wherever **Akali** is at the end of the cast time (usually mid-dash), but always towards and reaching the location that was 825 units in front of her at the start of cast. The only exception is if she moves via Flash, in which case the range of the missile is decreased relative to the distance she blinked.
- Killing an enemy with the shuriken will still mark the last area of the shroud hit.
- Attempting to cast Five Point Strike during either dash will buffer it to cast as soon as the dash ends.
- This ability's damage is calculated based on the caster's stats at the time of its application. is usable. is disabled for 10 seconds and then interrupts. Both dashes of interrupt.|items=true,,,interrupts|spells=true,interrupts,true,true,false

---

### R: Perfect Execution

| Attribute | Value |
|-----------|------:|
| **Cast Time** | None (Initial cast/recast) |
| **Target Range** | 675 (First cast range) units |
| **Collision Radius** | 110 (Both casts dash collision, estimated) units |
| **Speed** | 1500 (Initial dash speed) / 3000 (Second dash speed) units/second |
| **Cooldown** | 120 / 105 / 90 / 75 / 60 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.5$ (Initial cast) / $0.3$ (Recast) seconds |
| **Targeting** | Unit / Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | Walk in range of the target unit to cast (first cast) |
| **Terrain Grace** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Akali** dashes 750 units in the direction of the target enemy champion, dealing magic damage to enemies she passes through. If this hits an enemy, she flips over them to continue the dash up to the normal range but for at least another 150 units.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 110 / 165 / 220 / 275 / 330 (+ 50% **bonus** AD) (+ 30% AP) |

*Perfect Execution* can be recast after a $2.5$-second static cooldown (Unaffected by ability haste) within 10 seconds of the first activation.

**RECAST:** **Akali** dashes 800 (Estimated) units in the target direction, dealing magic damage to enemies she passes through, increased by 0%–200%@0–70 (@=target's **missing** health).

| Attribute | Value |
|-----------|------:|
| **Minimum Magic Damage** | 70 / 105 / 140 / 175 / 210 (+ 30% AP) |
| **Maximum Magic Damage** | 210 / 315 / 420 / 525 / 630 (+ 90% AP) |

**Notes:**

*Both casts count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- With the first cast, **Akali** can dash up to a maximum of 900 units if the first enemy she hit was at the maximum dash range.
- Casting Flash during either dashes will interrupt them, but **Akali** will deal damage to any enemies she comes in contact with at the new location.
  - Enemies already hit by either of *Perfect Execution*’s casts cannot be affected more than once by the same cast.
- Shuriken Flip can be used during the initial dash if it is cast after **Perfect Execution**’s cast time finishes.
- This ability's damage is calculated based on the caster's stats at the start of cast and does not change dynamically.
- **Akali** is unable to cast her other abilities for $0.25$ seconds after starting the initial cast's dash.

---

## Patch History

### V25.12
- Assassin's Mark
  - **Bug Fixes:** Ring indicator no longer sometimes appears around the incorrect characters.

### V25.11
- Five Point Strike
  - **Bug Fixes:** The ability's collision area is now properly offset based on her cast direction instead of incorrectly always being offset toward the north of the map. Previously, this would cause the ability to hit targets further when targeted parallel to or at an approximate angle to the map's north axis.

### V25.08
- Twilight Shroud
  - **Bug Fixes:** Smoke piece particles no longer layer incorrectly and flicker while on elevated terrain.

### V25.07
- Perfect Execution
  - **UNDOCUMENTED / BUG FIX:** First cast is no longer able to expose (via the indicator) and cast on Neeko disguised as a non-champion.

### V14.18
- General
  - **Bug Fixes:** Certain voiceover lines will now properly trigger to play.

### V14.15
- Twilight Shroud
  - Cooldown reduced to 20 / 19 / 18 / 17 / 16 seconds from 20 at all ranks.
- Perfect Execution
  - First cast base damage changed to 110 / 220 / 330 from 80 / 220 / 360.
  - Second cast minimum base damage increased to 70 / 140 / 210 from 60 / 130 / 200.
    - Second cast maximum base damage increased to 210 / 420 / 630 from 180 / 390 / 600.
  - Cooldown increased to 120 / 90 / 60 seconds from 100 / 80 / 60.

### V14.12
- Shuriken Flip
  - Shuriken base damage reduced to 21 / 42 / 63 / 84 / 105 from 30 / 56.25 / 82.5 / 108.75 / 135.
  - Shuriken AD ratio increased to 30% AD from $25.5$% AD.
  - Shuriken AP ratio reduced to 33% AP from 36% AP.
  - Dash base damage reduced to 49 / 98 / 147 / 196 / 245 from 70 / 131.25 / 192.5 / 253.75 / 315.
  - Dash AD ratio increased to 70% AD from $59.5$% AD.
  - Dash AP ratio reduced to 77% AP from 84% AP.

### V14.8#April 18th Hotfix|V14.8
- Twilight Shroud
  - **Bug Fixes:** No longer restores 200 energy instead of 100 upon cast.

### V14.8
- Stats
  - Base health increased to 600 from 570.

### V14.2
- Assassin's Mark
  - **New Effect:** Empowered attack can no longer expire during its windup against a target.
  - **Bug Fixes:** No longer becomes unable to cast abilities for about $1.5$ seconds if she uses the empowered attack at the last second.

## Trivia

- Akali's dance referenced Single Ladies by Beyoncé.
  - A side-by-side comparison can be seen here.
- Akali was the only energy-based champion to have any associated cost on their ultimate (one Essence of Shadow).
- Shadow Dance was going to be a skillshot but became single-targeted instead.
- Akali used to gain the 'Law of Inverse Ninja Strength' cosmetic Easter egg debuff (*"This unit is a flippin' ninja!"* - *"Ninjas are more effective when they work alone. For every Ninja on your team beyond yourself, you lose 1 health."*) when she, Kennen, Shen, and/or Zed found themselves on the same team. It was removed in V3.14 for unknown reasons.
- In the V1.0.0.115 April Fools' Day patch, the following change regarding Akali was jokingly listed：
  - Akali is no longer a ninja.
- Due to Perception of English /r/ and /l/ by Japanese speakers, *Akali* may sound identical to Japanese 明かり*akari* "light, brightness, gleam", from Old Japanese stative verbal root **aka(-r-)* "to be red, to be bright" (> *aka(-i)* 赤(い) "red")
- Akali is the second champion, after Yasuo, to have an emote that will orient itself the same (facing towards the camera), no matter what direction Akali was facing.
  - However, she is the first to always reposition herself to face the camera when using the emotes, and has this function on both her dance and joke emotes.
- Akali's Series 1 Eternals make the following references:
  - *Check Marks* references the check mark.
  - *Like a Ninja* describes Shuriken Flip’s mobility and use of shuriken, two features common in depictions of ninja in popular culture.

---
*This page was automatically generated from League of Legends Wiki data.*