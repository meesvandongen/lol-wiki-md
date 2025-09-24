# Singed

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
| **Champion** | Singed |
| **Title** | the Mad Chemist |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-04-18 |
| **Release Patch** | April 18, 2009 Patch |
| **Latest Changes** | V25.17 |
| **Roles** | Specialist |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 2400 |
| **Riot Points** | 260 |
| **Difficulty** | 2 |
| **Hero Type** | Tank |
| **Alt Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $650.0$ | $+96.0$ |
| **Mana** | $330.0$ | $+45.0$ |
| **Health Regen** | $9.5$ | $+0.55$ |
| **Mana Regen** | $7.5$ | $+0.55$ |
| **Armor** | $34.0$ | $+4.2$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $63.0$ | $+3.4$ |
| **Attack Speed** | $0.700$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.7$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $1.9\%$ | |
| **Acquisition Radius** | $300$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $135$ units | |
| **Selection Height** | $175$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $108.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Noxious Slipstream

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 225 (Center-to-edge) units |
| **On-target CD Static** | 8 |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** Whenever **Singed** moves near a champion, he gains a stack of *Noxious Slipstream* for 2 seconds, refreshing on subsequent passes and stacking up to 25 times.

**NOXIOUS SLIPSTREAM:** For each stack, **Singed** gains (ms) 25% **bonus** movement speed, up to a maximum of 625%.

This effect cannot occur on the same target more than once every few seconds.

**Notes:**

- No additional details.

---

### Q: Poison Trail

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 180 (Individual gas clouds) units |
| **Cost** | 13 Mana per second |
| **Static Cooldown** | 1 |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | AoEDoT |

**TOGGLE:** **Singed** continually creates a toxic cloud in his wake that lingers for $3.25$ seconds. The cloud inflicts poison to enemies within.

** The target takes magic damage every $0.25$ seconds over 2 seconds as well as upon being hit if not currently affected. Subsequent inflictions refresh the duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Tick** | 5 / 7.5 / 10 / 12.5 / 15 (+ 10.625% AP) |
| **Magic Damage per Second** | 20 / 30 / 40 / 50 / 60 (+ $42.5$% AP) |
| **Minimum Magic Damage** | 40 / 60 / 80 / 100 / 120 (+ 85% AP) |

**Singed** earns the kill credit of enemy minions that are poisoned and would die to the damage of allied minions within the time before the next damage instance.

**Notes:**

- Toggled abilities do not count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- When *Poison Trail*’s debuff is applied instead of refreshed, the target is dealt one additional instance of the debuff at the same time as the application. This is done by delaying the actual debuff by $0.1$ seconds.
  - If the target is already affected by the debuff, its duration is refreshed to the maximum (2 seconds).
- While active after spawning, a cloud checks for valid targets who remain in or re-enter a cloud every $0.25$ seconds.
  - Valid targets already afflicted with the poison will have their debuff's duration refreshed every $0.25$ seconds if applicable.
- If **Singed** has moved fewer than 90 units since spawning the last poison cloud, it will spawn 35 units in front of him. They also only spawn every 1 second in this case.
  - When **Singed** is moving faster than that, the poison clouds spawn more frequently and on top of himself.

---

### W: Mega Adhesive

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1000 units |
| **Effect Radius** | 265 units |
| **Cost** | 60 / 70 / 80 / 90 / 100 Mana |
| **Cooldown** | 17 / 16 / 15 / 14 / 13 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Spell Shield** | False |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Singed** spills a potent adhesive that lands at the target location after $0.375$ seconds, creating a field for 3 seconds that grounds enemies within and slows them.

| Attribute | Value |
|-----------|------:|
| **Slow** | 50 / 55 / 60 / 65 / 70% |

**Notes:**

- The goo missile is VFX only, to convey which location Singed threw the adhesive at. It has a fixed travel time but is independent from when the zone is established. It is not destructible by effects such as Wind Wall.
- *Mega Adhesive*’s slow and ground debuffs are each marked as non-dispellable, so they are not removed by most cleanses. Each is however allowed to be removed by cleanses that **also** grant immunity to the debuff type, such as Ragnarok.

---

### E: Fling

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 125 units |
| **Cost** | 60 / 70 / 80 / 90 / 100 Mana |
| **Cooldown** | 10 / 9.5 / 9 / 8.5 / 8 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Spell |
| **Call For Help** | True |

**ACTIVE:** **Singed** flings the target enemy 550 units over himself over , dealing magic damage. The damage based on the target's health ratio is capped at 300 against minions and monsters.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 50 / 60 / 70 / 80 / 90 (+ 6 / 6.5 / 7 / 7.5 / 8% of target's **maximum** health) (+ 55% AP) |

If the target lands on *Mega Adhesive’s* area of effect after the displacement, they are rooted for a duration.

| Attribute | Value |
|-----------|------:|
| **Root Duration** | 1 / 1.25 / 1.5 / 1.75 / 2 seconds |

**Notes:**

- *Fling* can throw enemies over walls (circumstances permitting).
- Slow-immune enemies will not be rooted when *flung* into Mega Adhesive.
- *Fling* is special cased to not fling Warwick while **Singed** is suppressed by Infinite Duress after the cast time.

---

### R: Insanity Potion

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 100 mana |
| **Cooldown** | 100 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Singed** empowers himself for 25 seconds with (ap) ability power, (armor) **bonus** armor, (mr) **bonus** magic resistance, (ms) **bonus** movement speed, **bonus** health regeneration, and **bonus** mana regeneration.

| Attribute | Value |
|-----------|------:|
| **Bonus Stats** | 25 / 42.5 / 60 / 77.5 / 95 |

| Attribute | Value |
|-----------|------:|
| **HP/Mana Regenerated per $0.5$ Seconds** | 2.5 / 4.25 / 6 / 7.75 / 9.5 |
| **Total HP/Mana Regeneration (per 5 Seconds)** | 125 / 212.5 / 300 / 387.5 / 475 |

During this time, *Poison Trail* additionally applies Grievous Wounds for 1 second, which refreshes every $0.25$ seconds while the poison persists.

**Notes:**

- No additional notes.

---

## Patch History

### V25.17
- Poison Trail
  - Debuff duration reduced to 2 seconds from $2.2$.
    - Maximum linger ticks reduced to 7 from 8.
      - Minimum linger damage reduced to 35 / 52.5 / 70 / 87.5 / 105 (+ 74.375% AP) from 40 / 60 / 80 / 100 / 120 (+ 85% AP).
  - Tooltip now includes the first damage instance in its calculations.

### V25.16#August 13th Hotfix|V25.16
- Poison Trail
  - **Bug Fixes:** No longer unintentionally triggers (activates or stacks) item-sourced debuffs of items that **Singed** has equipped on himself, once for each enemy champion affected by the poison.
  - **Bug Fixes:** Now correctly deals damage on the debuff's application instance.

### V25.16
- Poison Trail
  - **Bug Fixes:** The timing of damage by casters on the red will now match the timing of damage by casters on the blue side.
  - Debuff now affects enemies once at the same time as they are hit when the debuff is applied instead of refreshed.
  - **Bug Fixes:** Re-application check cadence increased to once every $0.25$ seconds from once every $0.5$ seconds.
    - **Bug Fixes:** There is also no longer an up-to $0.25$-second delay on the re-application check (it is no longer fuzzy).
    - Both of these changes together amend cases where, depending on buff update ticks, the caster could lose out on two to three further instances of damage on any enemy.
  - Debuff duration increased to $2.2$ seconds from $2.1$.
    - Now has an internal counter to apply 8 additional instances of damage at most after the final refresh, as a safety mechanism.

### V25.08
- Noxious Slipstream
  - Per-target cooldown reduced to 8 seconds from 10.

### V25.07
- Noxious Slipstream
  - Per-target cooldown increased to 10 seconds from 8.
- Poison Trail
  - AP ratio per tick increased to 10.625% AP from 10% AP.
  - **Bug Fixes:** No longer is able to illegally kill enemies who are under the effect of Undying Rage.
- Insanity Potion
  - Bonus stats reduced to 25 / 60 / 95 from 30 / 65 / 100.

### V25.06#March 20th Hotfix|V25.06
- Poison Trail
  - AP ratio per tick reduced to 10% AP from $10.625$% AP.

### V25.06
- Stats
  - Health growth reduced to 96 to 99.
  - Base attack speed increased to $0.7$ from $0.625$.
- Poison Trail
  - **New Effect:** Now grants **Singed** minion kill credit if an affected minion would die to minion damage within the time before the next damage instance.
- Fling
  - AP ratio reduced to 55% AP from 60% AP.

### V14.24
- Mega Adhesive
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.20
- Stats
  - Armor growth reduced to $4.2$ from $4.7$.

### V14.17
- Mega Adhesive
  - **Bug Fixes:** Puddle zone no longer forms if its projectile has been blocked by projectile-destroying effects.

## Trivia

- Singed was one of the first champions designed, together with Annie, Lee Sin, Sion, Sivir, and Twisted Fate.
  - Coincidentally, Singed is an anagram for the word design, befitting his history of being one of the earliest designed champions.
- Singed was one of the champions chosen for the Noxian pool available during the Ionia vs. Noxus match. He was one of the selected champions.
- Singed's dance references Will Smith "Jump on It" dance from The Fresh Prince of Bel-Air.
  - A side-by-side comparison can be seen here.
  - He shared this dance with Kayle before the latter's rework, though hers included the beginning of the dance.
- Even though Poison Trail’s cloud appears to expand as it fades, the area affected by the poison does not actually increase.
- Singed is one of the few champions with a special interaction between his emotes and his ability set, as Fling automatically causes him to laugh.
- Although Singed holds his shield in his left hand in the default splash art, every other skin shows it in his right hand; Singed holds his shield in his right hand in all the in-game models.
- A bottle of Singed's poison can be seen leaking in the Mac Launch preview video.
- Singed's bald figure, red and green color motifs, and his large boots make him resemble from the Mega Man/Rockman X series.
- Singed's Series 1 Eternals make the following references:
  - *Don't Chase* is a reference to a fan-made community rule where players should never chase Singed.
- Singed's Series 2 Eternals make the following references:
  - *Gotta Go Fast!* is a reference to the first opening theme from Sonic X.

---
*This page was automatically generated from League of Legends Wiki data.*