# Blitzcrank

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
| **Champion** | Blitzcrank |
| **Title** | the Great Steam Golem |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-09-02 |
| **Release Patch** | V0.9.22.16 |
| **Roles** | Catcher |
| **Riot Positions** | Support |
| **External Positions** | Support |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $600.0$ | $+109.0$ | $2453.0$ |
| **Mana** | $267.0$ | $+40.0$ | $947.0$ |
| **Health Regen** | $7.5$ | $+0.75$ | $20.2$ |
| **Mana Regen** | $8.5$ | $+0.8$ | $22.1$ |
| **Armor** | $37.0$ | $+4.7$ | $116.9$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $62.0$ | $+3.5$ | $121.5$ |
| **Attack Speed** | $0.625$ | $+1.1\%$ | $0.745$ |
| **Movement Speed** | $325.0$ | $+0.0$ | $325.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.1\%$ |
| **Acquisition Radius** | $600 units$ |
| **Gameplay Radius** | $80 units$ |
| **Pathing Radius** | $50 units$ |
| **Selection Radius** | $140 units$ |
| **Selection Height** | $170 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Mana Barrier

**Innate:** Periodically, when **Blitzcrank** gets to low-health, *Mana Barrier* generates a shield that scales with mana.

**Innate:** Periodically, when damaged to , **Blitzcrank** generates a shield equal to mana*maximum** mana*, lasting for up to 10 seconds.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- The shield will absorb the portion of incoming damage that would reduce health past 30%.
- 'Mana Barrier's interactions with other shield effects that trigger before taking damage that would reduce below 30% health:
  - *Mana Barrier* will trigger simultaneously with the Lifeline effect from *Hexdrinker*, *Maw of Malmortius*, or *Immortal Shieldbow* if **Blitzcrank** takes damage while above 30% health, and trigger before them when the damage is taken while below 30% health. *** *Mana Barrier* will always trigger simultaneously with the Lifeline effect from *Sterak's Gage* if **Blitzcrank** takes damage while above or below 30% health.

---

### Q: Rocket Grab

**Active:** **Blitzcrank** fires the right hand which deals magic damage and stun the first enemy hit. The target is also airborne towards **Blitzcrank**.

**Active:** **Blitzcrank** fires their right hand in the target direction that catches the first enemy hit, dealing them magic damage, stun them for $0.65 seconds$, and airborne them towards **Blitzcrank**, during which they are standard sight. **Blitzcrank is lockout to move or attack while Rocket Grab is in flight.**

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $20-16$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1800 / 1800 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $110-310$ (+ 120% AP)

**Notes:**

- 'Rocket Grab's range is composed of a missile with 1080 range, and a smaller center-range area check at the end (colloqially known as a 'lollipop').
  - The area check is currently half as long as it is supposed to be, reducing the intended 1150 range of the ability to 1115 range. *** Hitting an enemy with this area check only also fails to play the ability's hit SFX. This makes it possible to easily distinguish whether the target was picked up by the missile collision or 'lollipop'.
  - Targets larger than 95 units in radius can be hit by a different edge-range circle check at the end of the missile.
- The airborne debuff lasts for up-to 1 second but will end prematurely when the target's movement stops, while the stun will persist (if duration permits).
- Displacement immunity will not resist the application of the stun.
- The target's destination lies 75 units in front of **Blitzcrank**.
  - If the target is closer than this, the effect will still move them to this location.
  - If this location would be inside static terrain, the target's destination will end up on ''Blitzcrank's** side, even if that means pulling them right on top of **Blitzcrank''. *** This can occasionally fail.
- If **Blitzcrank** is being moved (e.g. Airborne, Kinematics or *Fate's Call*) during the grab, the target will be pulled towards **Blitzcrank**s old location.
- **Blitzcrank** will be ordered to basic attack the target once the target arrives.
- If the target dies to 'Rocket Grab's damage, they will not be pulled. Effect at cast time end

---

### W: Overdrive

**Active:** **Blitzcrank** shifts into overdrive, gaining **bonus attack speed** and a burst of ms.

*When the effect ends, **Blitzcrank** is briefly slow.*

**Active:** **Blitzcrank** shifts into overdrive, gaining **bonus attack speed** for 5 seconds and *ms **bonus** movement speed* that decays to 10% over the first $2.9$ seconds. When *Overdrive* ends, **Blitzcrank** is slow by 30% for $1.5$ seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 15 seconds |
| **Cast Time** | none |
| **Cost** | 75 mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | magic |
| **Spell Effects** | proc |

**Scaling:**
- **Bonus Attack Speed:** $30-70$%
- **Initial Bonus Movement Speed:** $60-80$%

**Notes:**

- The movement speed boost's decay is linear. On the first stat update after the buff is gained, the boost does not decay.
- The self-slow is affected by slow resist and Tenacity.

---

### E: Power Fist

**Active:** **Blitzcrank** charges up a fist to make the next basic damage deal double damage and airborne the target into the air.

**Active:** **Blitzcrank** empowers their next basic attack within 5 seconds to have an uncancellable windup, deal 100% AD (+ 25% AP) **bonus** physical damage and airborne the target for 1 second. This damage is affected by critical strike modifiers. *Power Fist basic attack reset *'Blitzcrank's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $9-5$ seconds |
| **Cast Time** | none |
| **Cost** | 25 mana |
| **Targeting** | Auto |
| **Affects** | Enemy, Structure |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | Special |

**Notes:**

- *Power Fist* deals basic damage, but also triggers spell effects by also being tagged as spell damage.
  - This includes the basic attack itself.
- The airborne is applied even if the attack is dodge or blind.
  - It is not applied if it is block.
- Spell shield prevents the airborne but not the damage.
- The enhanced attack will still complete and hit the target even if they become untargetable during the attack windup.
- The target is displaced a short distance during the airborne in a random direction.

---

### R: Static Field

**Passive:** While *Static Field* is ready, **Blitzcrank**’s basic attacks apply a stack of *Static Field*.

**Active:** **Blitzcrank** detonates a powerful static charge that destroys shield, deals magic damage, and silence nearby enemies.

**Passive:** While *Static Field* is not on cooldown, ''Blitzcrank's' basic attacks apply a stack to the target on-hit. While the target is afflicted, one stack is consumed every 1 second to strike them with lightning, dealing magic damage each time. **Active:** **Blitzcrank** detonates a static field that destroys the damage-mitigating shield of all nearby enemies, excluding the shields of monsters, and then deals them magic damage and silence them for $0.5$ seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $60-20$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 600 units |
| **Spell Shield** | True |
| **Spell Effects** | Special |

**Scaling:**
- **Magic Damage:** $50-150$
- **maximum** mana)
- **Magic Damage:** $275-525$ (+ 100% AP)

**Notes:**

- ''Blitzcrank's' attacks do not have to deal damage to apply the mark.
- The mark will be consumed even if the target becomes untargetable.
- The lightning bolts deal default damage while the static field applies area damage.
- Even if *Static Field* is blocked by a spell shield, it will still destroy any existing damage shield on the target.
- The visual effects will appear from wherever **Blitzcrank** is at the end of the cast time.
- Effect at cast time start

---

## Patch History

### V25.17
- *Static Field*
  - **Undocumented:** Bolt damage now applies as default damage instead of spell damage.

### V25.08
- *Mana Barrier*
  - Mana ratio increased to 35% **maximum** mana from 30%.
- *Rocket Grab*
  - Base damage increased to $110-310$ from $105-285$.

### V14.22
- *Power Fist*
  - AD ratio increased to 100% AD from 80% AD.

### V14.12
- Stats
  - Base health reduced to 600 from 650.
- *Mana Barrier*
  - Shield duration increased to 10 seconds from 4.

### V14.9
- Stats
  - Selection radius reduced to 140 units from 165.
- *Mana Barrier*
  - Shield duration reduced to 4 seconds from 10.
- *Overdrive*
  - Initial bonus movement speed reduced to $60-80$% from $70-90$%.

### V14.4
- *Static Field*
  - **Removed:*** No longer destroys shields against monsters.

### V14.2
- Stats
  - Base armor reduced to 37 from 40.
- *Power Fist*
  - AD ratio reduced to 80% AD from 100% AD.

### V13.18
- General
  - Updated ability icons.

### V13.17
- Stats
  - Base health increased to 650 from 633.
  - Base magic resistance increased to 32 from 28.
  - Base attack speed reduced to $0.625$ from $0.65$.
  - Attack speed ratio reduced to $0.625$ from $0.7$.
- *Mana Barrier*
  - Shield strength changed to 30% **maximum** mana at all levels from 15 to 45.
- *Overdrive*
  - Mana cost reduced to 75 from 85.
  - Bonus attack speed reduced to $30-70$% from $30-82$%.
  - **Removed:*** No longer deals 1% of target's **maximum** health magic damage on-hit while active, increased by 60 to 160 for 6–165 to 220 against non-champions.
- *Power Fist*
  - Mana cost reduced to 25 from 40.
  - AD ratio increased to 100% AD from 75% AD.
  - **Removed:*** Damage is no longer increased by 175% AD (+ 125% AP) against non-champions.
- *Static Field*
  - **New Effect:** On-hit damage now scales with 2% **maximum** mana.

### V13.9
- *Power Fist*
  - **Bug Fixes:** No longer causes *Glacial Augment* ice zones to last longer than intended.

## Trivia

- While initially referred to using *He/Him* pronouns, Blitzcrank's most recent lore update updated pronouns to *They/Them*.
  - The client and in-game ability descriptions still refer to Blitzcrank using *He/Him* pronouns.
- Concept art for a champion codenamed Iron Engineer was likely incorporated into Blitzcrank.
- Their dance references the Macarena dance.
  - A side-by-side comparison can be seen here.
  - They share this dance with Shaco with his animated variant.
- Blitzcrank has been jokingly referred to as a girl by Tom 'Zileas' Cadwell.
- They resemble Zog from Astro Boy (film)
- Blitzcrank's feet resemble that of a duck's, due to Riot's love of ducks.
- Blitzcrank and **Amumu** were planned to have Amumu Blitzcrank skins, but both were cancelled for failing to meet Riot's quality standards.
  - They may return in the future.
- *variant=old*'s old icon is reused for the Teamfight Tactics item.md) Rocket-Propelled Fist.
- In V12.20, a change was added to where *Power Fist* would occasionally knock up non-champion targets it killed "*to the moon*".
- Blitzcranks's Series 2 Eternals make the following references:
  - *Get Over Here* is a reference to 's eponymous quote from the Mortal Kombat franchise.
- Blitzcrank, **Caitlyn**, **Lissandra**, **Rumble**, **Sion**, **Varus**, **Vi**, **Viego**, **Xerath**, and **Ziggs** are the only champions who can apply crowd control on themselves.

---
*This page was automatically generated from League of Legends Wiki data.*