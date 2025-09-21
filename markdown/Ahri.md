# Ahri

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
| **Title** | the Nine-Tailed Fox |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-12-14 |
| **Release Patch** | V1.0.0.131 |
| **Latest Changes** | V25.13 |
| **Roles** | Burst |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Alt Type** | Assassin |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $590.0$ | $+104.0$ |
| **Mana** | $418.0$ | $+25.0$ |
| **Health Regen** | $2.5$ | $+0.6$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $21.0$ | $+4.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $53.0$ | $+3.0$ |
| **Attack Speed** | $0.668$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.668$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.2\%$ | |
| **Missile Speed** | $1750$ units/second | |
| **Acquisition Radius** | $525$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $135$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $100.0\%$ |
| **Healing** | $90.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $105.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: Essence Theft

**Innate:** **Ahri** generates a stack of *Essence Fragment* from killing minions and monsters. At max stacks, she consumes them to herself.

*Whenever **Ahri** scores a champion takedown within a short time of damaging them, she consumes their essence to herself.*

**Innate:** **Ahri** generates a stack of *Essence Fragment* whenever she kills a minion or monster. At 9 stacks, she consumes them to herself for 35 to 95 (+ 20% AP). Additionally, whenever **Ahri** scores a champion takedown within 3 seconds of damaging them, she brings their essence to her to consume it to heal herself for 75 to 165 (+ 30% AP).

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- No additional details.

---

### Q: Orb of Deception

**Active:** **Ahri** sends out her orb, dealing magic damage to enemies it passes through. After a point, the orb travels back to her, dealing the same amount in true damage to enemies it passes through.

**Active:** **Ahri** sends her orb in the target direction that deals magic damage to enemies it passes through. Upon reaching maximum range, it returns to her to deal the same amount in *true damage* to enemies it passes through. *Enemies can be hit only once per pass.*

| Attribute | Value |
|-----------|------:|
| **Range** | 900 (Without return point aoe) units |
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 100 (Circle check at return point) units |
| **Width** | 200 units |
| **Speed** | 1550 (Outgoing missile) / 60 - 2600 (Returning missile, acceleration 1900) units/second |
| **Cost** | $55-95$ mana |
| **Cooldown** | 7 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic True |
| **Spell Shield** | Special |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**Scaling:**
| Attribute | Value |
|-----------|------:|
| **Damage Per Pass** | $40-140$ (+ 50% AP) |
| **Total Mixed Damage** | $40×2-140×2$ (+ $50×2$% AP) |

**Notes:**

- *Orb of Deception* will hit additional units around the return point in a small circle when the orb turns around.
  - This applies both to the initial and return missile's damage, but the initial missile only deals damage if the unit was not hit by it already.
- Each pass of the projectile can only damage an enemy once.
- If **Ahri** dies while the orb is out, the orb will visually disappear but continue to deal damage and return to **Ahri**. Effect at cast time end
- This ability's damage is calculated based on the caster's current stats and changes dynamically.

---

### W: Fox-Fire

**Active:** **Ahri** gains a burst of *ms|move speed* and conjures three fox-fires that orbit her for a short time. Fox-fires home onto nearby enemies, dealing magic damage.

**Active:** **Ahri** gains *40% **bonus** movement speed* that decays over 2 seconds and conjures three flames which orbit her clockwise for up to $2.5$ seconds. After $0.25$ seconds of the cast, each flame targets a sight enemy based on priority, or after $0.4$ seconds, simply targets the closest visible enemy in range. Each flame deals magic damage, with subsequent flames against a target dealing 30% damage. Flames prioritize enemy champions hit by **Charm**, then enemy champion, then minions that would die to 'Fox-Fire's* damage, and then the target of *'Ahri's' last basic attack within 3 seconds. 'Fox-Fire's damage is doubled against minions below *20% **maximum** health*.

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 150 (Orbit radius) / 550 (Non-prioritized targets, from the missile location) / 725 (Prioritized targets, from Ahri's location) units |
| **Speed** | $75.92$ / 1400 (Fired missile speed) units/second |
| **Cost** | 30 mana |
| **Cooldown** | $10-6$ (Starts post-effect) seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |

**Scaling:**
| Attribute | Value |
|-----------|------:|
| **Initial Flame Magic Damage** | $40-120$ (+ 40% AP) |
| **Subsequent Flame Magic Damage** | $40×0.3-120×0.3$ (+ $40×0.3$% AP) |
| **Total Single-Target Damage** | $40×1.6-120×1.6$ (+ $40×1.6$% AP) |

| Attribute | Value |
|-----------|------:|
| **Increased Initial Flame Minion Damage** | $40×2-120×2$ (+ $40×2$% AP) |
| **Increased Subsequent Flame Minion Damage** | $40×2×0.3-120×2×0.3$ (+ $40×2×0.3$% AP) |

**Notes:**

- Each missile of *Fox-Fire* has its own shorter non-priority range.
- Any unused *Fox-Fires* will fizzle upon death.
- This ability's damage is calculated based on the caster's current stats and changes dynamically.

---

### E: Charm

**Active:** **Ahri** blows forth a kiss that charms the first enemy hit, dealing magic damage and knockdown.

**Active:** **Ahri** blows forth a kiss in the target direction that deals magic damage to the first enemy hit, knockdown and charm and slow them by 65% for a duration.

| Attribute | Value |
|-----------|------:|
| **Range** | 1000 units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 120 units |
| **Speed** | 1550 units/second |
| **Cost** | 60 mana |
| **Cooldown** | 12 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
| Attribute | Value |
|-----------|------:|
| **Magic Damage** | $80-240$ (+ 85% AP) |

| Attribute | Value |
|-----------|------:|
| **Disable Duration** | $1.2-1.8$ seconds |

**Notes:**

- *Charm* is able to slow targets otherwise slow immunity upon charm them.
- This ability's damage is calculated based on the caster's stats at the time of its application. Effect at cast time end

---

### R: Spirit Rush

**Active:** **Ahri** dash forward and fires essence bolts to nearby enemies, dealing magic damage.

**Ahri** can cast this ability up to three times in quick succession. During this time, additional casts are gained whenever a champion essence is consumed via **Essence Theft**.

**Active:** **Ahri** dashes to the target location and then fires essence bolts to up to 3 nearby sight enemies, each dealing magic damage. 

*Spirit Rush* can be recast twice more within 15 seconds of the activation at no additional cost, with a 1-second static cooldown (Unaffected by ability haste) between casts. Consuming a champion's essence with **Essence Theft** while *Spirit Rush* is active extends the recast duration by and up to 10 seconds, and grants an additional recast, storing up to 3 recasts at a time. **Recast:** **Ahri** mimics the first cast's effects. 'Spirit Rush's recast duration will persist even after using all recasts. *Fox-Fire* can be cast during the dash.'

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 500 (Maximum dash range) units |
| **Effect Radius** | 600 (Energy bolts target search radius, from Ahri's location on dash end/interrupt) units |
| **Speed** | 1200 + / 1400 (Energy bolt missile speed) units/second |
| **Cost** | 100 mana |
| **Cooldown** | $140-100$ (Starts after first cast, and refreshes upon gaining recasts) seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Knockdown** | True |

**Scaling:**
| Attribute | Value |
|-----------|------:|
| **Magic Damage** | $60-120$ (+ 35% AP) |

**Notes:**

- Each cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- *Spirit Rush* uses quick cast by default.
- There's a slight delay before a champion gains vision of the fog of war once inside it. Because of this, if **Ahri** dashes into it, it is possible that *Spirit Rush* will not target any enemy in range.
- The bolts do not fire if **Ahri** dies while dash, unless she was saved by resurrection.
  - The bolts will fire if the dash is interrupt by other means.
- This ability's damage is calculated based on the caster's current stats and changes dynamically.

---

## Patch History

### V25.13
- *Essence Theft*
  - **Bug Fixes:** Healing is now properly granted while she is untargetable.

### V25.12
- *Charm*
  - Charm duration reduced to $1.2-1.8$ seconds from $1.2-2$.
- *Spirit Rush*
  - Cooldown increased to $140-100 3$ seconds from $130-100 3$.

### V25.08
- Stats
  - Armor growth reduced to $4.2$ from $4.7$.
- *Fox-Fire*
  - Initial flame base damage reduced to $40-120$ from $45-145$.
  - Initial flame AP ratio increased to 40% AP from 30% AP.
  - Cooldown increased to $10-6$ seconds from $9-5$.
- *Charm*
  - AP ratio increased to 85% AP from 75% AP.

### V25.04
- *Essence Theft*
  - **Bug Fixes:** Heal no longer interrupts Recall.
- Ahri (Signature)
  - **Bug Fixes:** Non-damaging displacement effects can no longer be prematurely canceled by using the Toggle expression during the movement.
- Ahri / Ahri
  - Updated skin border, Recall VFX and decal VFX to reflect Faker's fifth World Championship title.

### V14.18
- *Charm*
  - Base damage increased to $80-240$ from $80-200$.
  - AP ratio increased to 75% AP from 60% AP.

### V14.14
- General
  - **Bug Fixes:** No longer creates a fake clone of an enemy champion upon **Ahri** discovering their corpse, if that enemy died within the Fog of War.

### V14.12
- Ahri
  - **Bug Fixes:** All tails now properly overlap during walk cycle animations instead of being slightly spread.

### V14.10
- Ahri
  - **Bug Fixes:** Champion model no longer disappears after dying and reviving.

### V14.9
- *Fox-Fire*
  - Initial flame base damage reduced to $45-145$ from $50-150$.
    - Subsequent flame base damage reduced to $45×0.3-145×0.3$ from $50×0.3-150×0.3$.
      - Total base damage reduced to $45×1.6-145×1.6$ from $50×1.6-150×1.6$.
    - Minion increased base damage reduced to $45×2-145×2$ from $50×2-150×2$.
- *Spirit Rush*
  - Cooldown increased to $130-100 3$ seconds from $130-80 3$.

### V14.4
- Stats
  - Health growth increased to 104 from 96.
- *Orb of Deception*
  - AP ratio per hit increased to 50% AP from 45% AP.
    - Total AP ratio increased to 100% AP from 90% AP.

## Trivia

- Ahri's dance references “RunDevilRun” by Girls' Generation.
- The first icon for *variant=old*’s displays Ahri without fox ears, which harkens to the time she didn't have them.
- Some of her older models, such as Ahri and Ahri, lack whiskers ingame.
- *Ahri* 아리 could've been shortened from '아리땁다' *Arittabda*, from stative verbal stem *aritta(b)-* "be beautiful" & verbal suffix *-da.*
- For "The most visually appealing champion in League", Ahri's visual appeal is statistically ranked 1st among men, and 2nd among women (second to **Jinx**).
- Ahri's Series 1 Eternals make the following references:
  - *LDR* refernces the eponymous acronym that stands for "long-distance relationship" and the large distance covered from *Spirit Rush*’s multiple dashes.
  - *On the Rebound* references the eponymous phrase associated with unhappy relationships and the literal rebound of *Orb of Deception*’s projectile.

---
*This page was automatically generated from League of Legends Wiki data.*