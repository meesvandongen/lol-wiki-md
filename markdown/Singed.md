# Singed

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
| **Champion** | Singed |
| **Title** | the Mad Chemist |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-04-18 |
| **Release Patch** | April 18, 2009 Patch |
| **Roles** | Specialist |
| **Riot Positions** | Top |
| **External Positions** | Top |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $650.0$ | $+96.0$ | $2282.0$ |
| **Mana** | $330.0$ | $+45.0$ | $1095.0$ |
| **Health Regen** | $9.5$ | $+0.55$ | $18.9$ |
| **Mana Regen** | $7.5$ | $+0.55$ | $16.9$ |
| **Armor** | $34.0$ | $+4.2$ | $105.4$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $63.0$ | $+3.4$ | $120.8$ |
| **Attack Speed** | $0.700$ | $+1.9\%$ | $0.926$ |
| **Movement Speed** | $345.0$ | $+0.0$ | $345.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.7$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.9\%$ |
| **Acquisition Radius** | $300 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $135 units$ |
| **Selection Height** | $175 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Noxious Slipstream

**Innate:** Whenever **Singed** moves near a champion, he gains *ms **bonus** movement speed* for a short time, which refreshes on subsequent passes and stacks up to a cap.

*This effect cannot occur on the same target more than once every few seconds.*

**Innate:** Whenever **Singed** moves near a champion, he gains a stack of *Noxious Slipstream* for 2 seconds, refreshing on subsequent passes and stacking up to 25 times. **Noxious Slipstream:** For each stack, **Singed** gains ms*bonus** movement speed*, up to a maximum of 625%. This effect cannot occur on the same target more than once every few seconds.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Effect Radius** | 225 units |

**Notes:**

- No additional details.

---

### Q: Poison Trail

**Toggle:** **Singed** continually creates a toxic cloud that shortly lingers in his wake, which continually inflicts poison to enemies within.

**Toggle:** **Singed** continually creates a toxic cloud in his wake that lingers for $3.25$ seconds. The cloud inflicts poison to enemies within. **poison The target takes magic damage every $0.25$ seconds over 2 seconds as well as upon being hit if not currently affected. Subsequent inflictions refresh the duration. **Singed** earns the kill credit of enemy minions that are poison and would die to the damage of allied minions within the time before the next damage instance.

| Attribute | Value |
|-----------|-------|
| **Cast Time** | none |
| **Cost** | 13 Mana per second |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 180 units |
| **Spell Effects** | AoEDoT |

**Scaling:**
- **Magic Damage per Tick:* $ (+ $8 (+ $(

**Notes:**

- Toggled abilities do not count as ability activations for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- When 'Poison Trail's debuff is applied instead of refreshed, the target is dealt one additional instance of the debuff at the same time as the application. This is done by delaying the actual debuff by $0.1$ seconds.
  - If the target is already affected by the debuff, its duration is refreshed to the maximum (2 seconds).
- While active after spawning, a cloud checks for valid targets who remain in or re-enter a cloud every $0.25$ seconds.
  - Valid targets already afflicted with the poison will have their debuff's duration refreshed every $0.25$ seconds if applicable.
- If **Singed** has moved fewer than 90 units since spawning the last poison cloud, it will spawn 35 units in front of him. They also only spawn every 1 second in this case.
  - When **Singed** is moving faster than that, the poison clouds spawn more frequently and on top of himself.

---

### W: Mega Adhesive

**Active:** **Singed** creates a field of adhesive at the target location for a short time that ground and slows enemies within.

**Active:** **Singed** spills a potent adhesive that lands at the target location after $0.375$ seconds, creating a field for 3 seconds that ground enemies within and slows them.

| Attribute | Value |
|-----------|-------|
| **Range** | 1000 units |
| **Cooldown** | $17-13$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-100$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Effect Radius** | 265 units |
| **Spell Shield** | False |
| **Projectile** | True |

**Scaling:**
- **Slow:** $50-70$%

**Notes:**

- The goo missile is VFX only, to convey which location Singed threw the adhesive at. It has a fixed travel time but is independent from when the zone is established. It is not destructible by effects such as *Wind Wall*.
- 'Mega Adhesive's slow and ground debuffs are each marked as non-dispellable, so they are not removed by most cleanse. Each is however allowed to be removed by cleanses that **also** grant immunity to the debuff type, such as Ragnarok.

---

### E: Fling

**Active:** **Singed** airborne the target enemy, dealing magic damage based on their **maximum** health.

*If the target lands on **Mega Adhesive**, they are briefly root.*

**Active:** **Singed** airborne the target enemy 550 units over himself over , dealing magic damage. The damage based on the target's health ratio is capped at 300 against minions and monsters. If the target lands on **Mega Adhesive*’s* area of effect after the displacement, they are root for a duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 125 units |
| **Cooldown** | $10-8$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-100$ Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Spell |

**Scaling:**
- **Magic Damage:** $50-90$
- **maximum** health) (+ 55% AP)
- **Root Duration:** $1-2$ seconds

**Notes:**

- *Fling* can throw enemies over walls (circumstances permitting).
- Slow-immune enemies will not be root when *flung* into *Mega Adhesive*.
- *Fling* is special cased to not airborne **Warwick** while **Singed** is suppression by *Infinite Duress* after the cast time.

---

### R: Insanity Potion

**Active:** **Singed** empowers himself for some time with *ap **bonus** ability power*, *armor *bonus armor*, *mr **bonus** magic resist*, *ms **bonus** movement speed*, **bonus health regen**, and **bonus mana regen**.

*During this time, **Poison Trail** additionally applies Grievous Wounds.*

**Active:** **Singed** empowers himself for 25 seconds with ap, *armor *bonus armor*, *mr **bonus** magic resistance*, *ms **bonus** movement speed*, **bonus health regeneration**, and **bonus mana regeneration**. During this time, **Poison Trail** additionally applies Grievous Wounds for 1 second, which refreshes every $0.25$ seconds while the poison persists.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 100 seconds |
| **Cast Time** | none |
| **Cost** | 100 mana |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- **Bonus Stats:** $25-95$ **HP/Mana Regenerated per $0.5$ Seconds:* $25/10-95/10$5-95×5$

**Notes:**

- No additional notes.

---

## Patch History

### V25.17
- *Poison Trail*
  - Debuff duration reduced to 2 seconds from $2.2$.
    - Maximum linger ticks reduced to 7 from 8.
      - Minimum linger damage reduced to $(20/4)*7-(60/4)*7$ (+ $(42.5/4)*7$% AP) from $(20/4)*8-(60/4)*8$ (+ $(42.5/4)*8$% AP).
  - Tooltip now includes the first damage instance in its calculations.
- *Poison Trail*
  - **Bug Fixes:** No longer unintentionally triggers (activates or stacks) item-sourced debuffs of items that **Singed** has equipped on himself, once for each enemy champion affected by the poison.
  - **Bug Fixes:** Now correctly deals damage on the debuff's application instance.

### V25.16
- *Poison Trail*
  - **Bug Fixes:** The timing of damage by casters on the red will now match the timing of damage by casters on the blue side.
  - Debuff now affects enemies once at the same time as they are hit when the debuff is applied instead of refreshed.
  - **Bug Fixes:** Re-application check cadence increased to once every $0.25$ seconds from once every $0.5$ seconds.
    - **Bug Fixes:** There is also no longer an up-to $0.25$-second delay on the re-application check (it is no longer fuzzy).
    - Both of these changes together amend cases where, depending on buff update ticks, the caster could lose out on two to three further instances of damage on any enemy.
  - Debuff duration increased to $2.2$ seconds from $2.1$.
    - Now has an internal counter to apply 8 additional instances of damage at most after the final refresh, as a safety mechanism.

### V25.08
- *Noxious Slipstream*
  - Per-target cooldown reduced to 8 seconds from 10.

### V25.07
- *Noxious Slipstream*
  - Per-target cooldown increased to 10 seconds from 8.
- *Poison Trail*
  - AP ratio per tick increased to $42.5/4$% AP from $40/4$% AP.
  - **Bug Fixes:** No longer is able to illegally kill enemies who are under the effect of Undying Rage.
- *Insanity Potion*
  - Bonus stats reduced to $25-95 3$ from $30-100 3$.
- *Poison Trail*
  - AP ratio per tick reduced to 10% AP from $10.625$% AP.

### V25.06
- Stats
  - Health growth reduced to 96 to 99.
  - Base attack speed increased to $0.7$ from $0.625$.
- *Poison Trail*
  - **New Effect:** Now grants **Singed** minion kill credit if an affected minion would die to minion damage within the time before the next damage instance.
- *Fling*
  - AP ratio reduced to 55% AP from 60% AP.

### V14.24
- *Mega Adhesive*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.20
- Stats
  - Armor growth reduced to $4.2$ from $4.7$.

### V14.17
- *Mega Adhesive*
  - **Bug Fixes:** Puddle zone no longer forms if its projectile has been blocked by projectile-destroying effects.

### V14.11
- *Poison Trail*
  - AP ratio per tick increased to $10.625$% AP from 10% AP.
- *Insanity Potion*
  - Cooldown reduced to 100 seconds at all ranks from $120-100 3$.
- *Poison Trail*
  - AP ratio per tick reduced to 10% AP from $11.25$% AP.

### V13.22
- Stats
  - Base attack speed increased to $0.625$ from $0.613$.
  - Attack speed ratio increased to $0.625$ from $0.613$.
  - Attack windup reduced to 20% from $23.614873$%.

## Trivia

- Singed was one of the first champions designed, together with **Annie**, **Lee Sin**, **Sion**, **Sivir**, and **Twisted Fate**.
  - Coincidentally, Singed is an anagram for the word design, befitting his history of being one of the earliest designed champions.
- Singed was one of the champions chosen for the Noxian pool available during the Ionia vs. Noxus match. He was one of the selected champions.
- Singed's dance references Will Smith "Jump on It" dance from The Fresh Prince of Bel-Air.
  - A side-by-side comparison can be seen here.
  - He shared this dance with **Kayle** before the latter's rework, though hers included the beginning of the dance.
- Even though *Poison Trail*’s cloud appears to expand as it fades, the area affected by the poison does not actually increase.
- Singed is one of the few champions with a special interaction between his emotes and his ability set, as *Fling* automatically causes him to laugh.
- Although Singed holds his *shield* in his left hand in the default splash art, every other skin shows it in his right hand; Singed holds his shield in his right hand in all the in-game models.
- A bottle of Singed's poison can be seen leaking in the Mac Launch preview video.
- Singed's bald figure, red and green color motifs, and his large boots make him resemble from the Mega Man/Rockman X series.
- Singed's Series 1 Eternals make the following references:
  - 'Don't Chase' is a reference to a fan-made community rule where players should never chase Singed.
- Singed's Series 2 Eternals make the following references:
  - *Gotta Go Fast!* is a reference to the first opening theme from Sonic X.

---
*This page was automatically generated from League of Legends Wiki data.*