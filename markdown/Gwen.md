# Gwen

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
| **Champion** | Gwen |
| **Title** | The Hallowed Seamstress |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2021-04-15 |
| **Release Patch** | V11.8 |
| **Latest Changes** | V25.17 |
| **Roles** | Skirmisher |
| **Riot Positions** | Top |
| **External Positions** | Top, Jungle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 1 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 50 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $600.0$ | $+110.0$ |
| **Mana** | $330.0$ | $+40.0$ |
| **Health Regen** | $9.0$ | $+0.9$ |
| **Mana Regen** | $7.5$ | $+0.7$ |
| **Armor** | $33.0$ | $+4.9$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $63.0$ | $+3.0$ |
| **Attack Speed** | $0.690$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $150.0$ | $+0.0$ |
| **Base Attack Speed** | $0.69$ | |
| **Attack Speed Ratio** | $0.69$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Attack Windup** | $19.7\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $120$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $102.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: A Thousand Cuts

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Effects** | proc |
| **Parry** | True |

**INNATE:** **Gwen**’s basic attacks on-hit, the center of *Snip Snip!*, and *Needlework* deal **bonus** magic damage equal to 1% (+ $0.55$% per 100 AP) of the target's **maximum** health.

*A Thousand Cuts* is modified based on the target:
- Heals **Gwen** for 50% of post-mitigation damage (Damage calculated after resistances and most modifiers.) dealt against champions, capped at 10 to 25 (+ $6.5$% AP) per instance.
- Deals an additional 8 to 30 **bonus** magic damage against minions that are **below** 40% health.
- Deals a maximum of 5 (+ 10% AP) magic damage against monsters.

**Notes:**

- The attacks do not deal the **bonus** damage against structures.

---

### Q: Snip Snip!

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Cost** | 40 mana |
| **Cooldown** | 6.5 / 5.75 / 5 / 4.25 / 3.5 seconds |
| **Queue Time** | $0.05$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic True |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |

**PASSIVE:** **Gwen**’s basic attacks generate a stack of *Snippy* on-hit for 6 seconds, stacking up to 4 times and refreshing on subsequent attacks.

**ACTIVE:** **Gwen** snips at least twice with her scissors in a cone in the target direction over the cast time, dealing magic damage per snip to all enemies within the area, with the final snip dealing increased damage. The center of each snip converts 50% of the damage to , then applies *A Thousand Cuts*. *Snip Snip!* deals 75% damage against minions, and executes them on the first damage instance if they are **below** 20% health.

| Attribute | Value |
|-----------|------:|
| **Damage per Snip** | 10 / 15 / 20 / 25 / 30 (+ 2% AP) |
| **Center Damage per Snip** | 10 / 15 / 20 / 25 / 30 (+ 2% AP)
(+ 1% (+ $0.55$% per 100 AP) of the target's **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Final Snip Damage** | 60 / 85 / 110 / 135 / 160 (+ 35% AP) |
| **Final Snip Center Damage** | 60 / 85 / 110 / 135 / 160 (+ 35% AP)
(+ 1% (+ 0.55% per 100 AP) of the target's **maximum** health) |

If **Gwen** has any *Snippy* stacks, she consumes them to snip an additional time for each.

| Attribute | Value |
|-----------|------:|
| **Minimum Damage** | 70 / 100 / 130 / 160 / 190 (+ 37% AP) |
| **Minimum Center Damage** | 70 / 100 / 130 / 160 / 190 (+ 37% AP)
(+ 2% (+ 1.1% per 100 AP) of the target's **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Maximum Damage** | 110 / 160 / 210 / 260 / 310 (+ 45% AP) |
| **Maximum Center Damage** | 110 / 160 / 210 / 260 / 310 (+ 45% AP)
(+ 6% (+ 3.3% per 100 AP) of the target's **maximum** health) |

**Notes:**

- Subsequent basic attacks on-hit also refresh the duration of *Snippy* stacks.
- The amount of *Snippy* stacks **Gwen** has is indicated below her health bar.
- Spell shield blocks only one snip.
- This ability will cast from wherever the caster is at the end of the cast time.
- The first snip happens at $0.13$ seconds (Measured, should be 0.1 seconds rounded up to 0.132 seconds), the last one at the end of the cast time.
  - Bonus snips from *Snippy* stacks each happen at $0.45$, $0.4$, $0.35$ and $0.23$ seconds into the cast time.

---

### W: Hallowed Mist

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 480 units |
| **Speed** | 2000 (Mist travel speed, teleports to Gwen if she is moving outside the zone faster than it can travel) units/second |
| **Cost** | 60 mana |
| **Cooldown** | 24 / 22.5 / 21 / 19.5 / 18 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Gwen** summons the Hallowed Mist upon her current location, lasting for 4 seconds. *Hallowed Mist* can be recast after $0.5$ seconds, and does so automatically if **Gwen** attempts to leave the area.

While inside the mist, **Gwen** becomes ghosted, gains 22 (+ 7% AP) **bonus** armor and **bonus** magic resistance and is untargetable to all enemies (except turrets and monsters) outside of the mist.

**RECAST:** **Gwen** commands the mist to move to her current location.

***Gwen** negates all attacks by monsters outside of the mist.*

**Notes:**

- *Hallowed Mist*’s untargetability does not apply to monsters nor turrets, meaning they are allowed to gain aggro against **Gwen** and attack her regardless of being in the mist or not.
  - However, **Gwen** will not take damage from monsters that hit her and are outside of the mist.
- *Hallowed Mist* does not grant untargetability against enemy pets inside the zone even if their source is not.
- All projectiles targeted at **Gwen** by enemy champions outside the mist are destroyed upon attempting to pass through the mist or being inside the mist.
  - Targeted projectiles sourced by minions or monsters will not be destroyed and can still hit **Gwen** even if they are outside of the zone.
- If **Gwen** dashes out of the area, the mist will automatically move to the dash's destination instead of **Gwen**’s location.
  - The mist will stop moving immediately if the dash is interrupted.
- The recast will move the mist's center 75 units through **Gwen**’s location at the time.
  - If she is dashing, the mist will move 75 units through her dash destination instead.
- The mist will teleport to **Gwen**’s location if she blinks a far distance.
- Enemy champions outside the mist are indicated with a broken sword.
- *Hallowed Mist* will indicate that ***Gwen is Immune*** whenever an effect attempts to hit **Gwen**.

---

### E: Skip 'n Slash

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 350 (Dash range) / 450 (Maximum dash range across terrain) units |
| **Cost** | 35 Mana |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Location |
| **Affects** | Self |
| **Damage Type** | magic |
| **Spell Effects** | proc |
| **Out of Range Behavior** | cast at max |
| **Terrain Grace** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Gwen** dashes to the target location, then empowers her basic attacks within the next 4 seconds to deal **bonus** magic damage on-hit and gain **bonus** attack speed and 75 **bonus** attack range.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage On-Hit** | 8 / 11 / 14 / 17 / 20 (+ 25% AP) |

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 20 / 35 / 50 / 65 / 80% |

**Gwen**’s first basic attack within the duration reduces *Skip 'n Slash*’s cooldown by 50%.

*Skip 'n Slash resets **Gwen**’s basic attack timer, and can be cast during any of her abilities, and vice versa.*

**Notes:**

- *Skip 'n Slash* has no minimum dash range.
- The basic attack reset is not considered one for Hail of Blades.

---

### R: Needlework

| Attribute | Value |
|-----------|------:|
| **Range** | cr -100 + 1350 (Starting 100 units behind Gwen's location, each missile travels 1350 units) |
| **Cast Time** | $0.25$ (First cast) / $0.5$ (Second and third casts) seconds |
| **Width** | 240 to 30 (-140 total width per half-second of travel time) units |
| **Speed** | 1800 (Needle missiles speed) units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.5$ (All casts) seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |
| **Projectile** | True |

**ACTIVE:** **Gwen** launches a needle in the target direction that deals magic damage to enemies hit and slows them for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Needle** | 30 / 45 / 60 / 75 / 90 (+ 8% AP) |
| **Damage with A Thousand Cuts** | 30 / 45 / 60 / 75 / 90 (+ 8% AP) (+ 1% (+ 0.55% per 100 AP) of the target's **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 30 / 37.5 / 45 / 52.5 / 60% |

*Needlework* can be recast twice more within 6 seconds of the activation at no additional cost, with a 1-second static cooldown (Unaffected by ability haste) between casts. Each subsequent cast refreshes the duration and increases the number of needles launched by 2. Needle hits beyond the first apply a slow against each individual target.

| Attribute | Value |
|-----------|------:|
| **Subsequent Slow** | 15 / 17.5 / 20 / 22.5 / 25% |

**RECAST:** **Gwen** mimics the first cast's effects, while barraging three needles on the second cast and five on the third cast.

| Attribute | Value |
|-----------|------:|
| **Second Cast Total Damage** | 90 / 135 / 180 / 225 / 270 (+ 24% AP)
(+ 3% (+ 1.65% per 100 AP) of the target's **maximum** health) |
| **Third Cast Total Damage** | 150 / 225 / 300 / 375 / 450 (+ 40% AP)
(+ 5% (+ 2.75% per 100 AP) of the target's **maximum** health) |
| **Maximum Total Damage** | 270 / 405 / 540 / 675 / 810 (+ 72% AP)
(+ 9% (+ 4.95% per 100 AP) of the target's **maximum** health) |

***Gwen** can move during Needlework's cast times.*

**Notes:**

- Dying during the cast time will prevent the remaining needles of the cast from being launched.
- The first cast's cast time is $0.25$ seconds, and the second and third's cast times are each $0.5$ seconds. **Gwen** launches the needles during the cast times.
  - For the second cast, the first needle launches after the start of the cast, the second needle launches after , and the third needle launches after the first.
  - For the third cast, the first needle launches after the start of the cast. Each additional needle launches seconds after the previous one.
- The needles of each cast will fire based on **Gwen**’s location at into each cast time (in case of the first cast, this is at the end of the cast time).
  - The needles fire from 100 units *behind* the cast location.
  - The width of all needle missiles decreases with their distance travelled, forming an approximately triangular missile hitbox.
  - Needles of the same cast all travel the same line at a slight delay after one another. The visual origins are offset (by 70 and 110 units to either from the center missile spawn location), but the actual hitting missiles all follow the central line.
- Spell shield blocks only one needle.

---

## Patch History

### V25.17
- Stats
  - Base armor reduced to 33 from 36.
  - Attack speed growth increased to $2.5$% from $2.25$%.
- Needlework
  - Initial slow reduced to 30 / 45 / 60% from 60% at all ranks.
  - Subsequent slow reduced to 15 / 20 / 25% from 25% at all ranks.

### V25.16
- Hallowed Mist
  - **Bug Fixes:** Casting *Hallowed Mist* to nullify the displacement of Yorick’s Dark Procession spawning no longer causes **Gwen** to glitch inside it when attempting to move or dash away.

### V25.12
- Stats
  - Base health reduced to 600 from 620.
- Hallowed Mist
  - Cooldown increased to 24 / 22.5 / 21 / 19.5 / 18 seconds from 22 / 21 / 20 / 19 / 18.

### V25.09
- Stats
  - Armor growth reduced to $4.9$ from $5.2$.
- A Thousand Cuts
  - Monster damage cap AP ratio reduced to 10% AP from 15% AP.
  - **Bug Fixes:** Tooltip now notes the ability's healing cap.
- Snip Snip!
  - AP ratio per small snip reduced to 2% AP from 5% AP.

### V25.08
- Skip 'n Slash
  - On-hit base damage reduced to 8 / 11 / 14 / 17 / 20 from 12 / 14 / 16 / 18 / 20.

### V25.07
- Snip Snip!
  - Final snip base damage reduced to 60 / 85 / 110 / 135 / 160 from 70 / 95 / 120 / 145 / 170.
  - **Bug Fixes:** Tooltip now notes the proper damage values instead of incorrectly noting each small snip as dealing "20% of the big snip damage".
- Hallowed Mist
  - Base bonus resistances reduced to 22 from 25.
  - Resistances AP ratio increased to 7% AP from 5% AP.
- Needlework
  - **Bug Fixes:** No longer displays debug text.

### V25.06#March 19th Hotfix|V25.06
- Stats
  - Base health reduced to 620 from 650.
  - Base armor reduced to 36 from 39.

### V25.06
- Stats
  - Base health increased to 650 from 620.
  - Health growth reduced to 110 from 115.
- A Thousand Cuts
  - AP ratio reduced to $0.55%$ per 100 AP from $0.6$% per 100 AP.
  - Base monster damage cap reduced to 5 from 10.
- Snip Snip!
  - Final snip base damage increased to 70 / 95 / 120 / 145 / 170 from 60 / 85 / 110 / 135 / 160.
- Hallowed Mist
  - Base bonus resistances changed to 25 at all ranks from 22 / 24 / 26 / 28 / 30.
  - Resistances AP ratio reduced to 5% AP from 7% AP.
- Skip 'n Slash
  - Base damage changed to 12 / 14 / 16 / 18 / 20 from 15 at all ranks.
  - AP ratio increased to 25% AP from 20% AP.
  - Cooldown reduced to 12 / 11 / 10 / 9 / 8 seconds from 13 / 12.5 / 12 / 11.5 / 11.
  - Cooldown refund changed to 50% at all ranks from 25 / 35 / 45 / 55 / 65%.
- Needlework
  - Base damage per needle reduced to 30 / 60 / 90 from 35 / 65 / 95.
  - AP ratio per needle reduced to 8% AP from 10% AP.
  - Initial slow increased to 60% at all ranks from 40 / 50 / 60%.
  - Subsequent slow increased to 25% at all ranks from 15 / 20 / 25%.

### V14.21
- A Thousand Cuts
  - AP ratio reduced to $0.6$% per 100 AP from $0.72$% per 100 AP.

### V14.17
- Needlework
  - **Bug Fixes:** No longer ignores Malignance Hatefog's per-target cooldown.

## Trivia

- Gwen is the first of three champions released in 2021 tied to Viego’s return and a continuation of the Ruined King's story.
- She's the second Heterochromia iridum champion (first being Zoe) with her right eye being teal, and the other being violet.
- Gwen will walk in her doll form for a short time if you let her transform during her joke animation.
- There is a Gwen easter egg within Ruined King: A League of Legends Story.
- Her dance is based on the PONPONPON music video.
  - A side-by-side comparison can be seen here.
- A Thousand Cuts is a play on the eponymous phrase 'death by a thousand cuts' (meaning to slowly and incrementally dismantle something,) which has been given a literal meaning through association with Gwen's signature Crafting Scissors. The phrase is itself a reference to an ancient Lingchi.
- Gwen's champion theme has elements of Viego’s mixed in.

---
*This page was automatically generated from League of Legends Wiki data.*