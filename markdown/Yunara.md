# Yunara

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
| **Champion** | Yunara |
| **Title** | the Unbroken Faith |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2025-07-16 |
| **Release Patch** | V25.14 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $590.0$ | $+110.0$ | $2460.0$ |
| **Mana** | $275.0$ | $+45.0$ | $1040.0$ |
| **Health Regen** | $4.0$ | $+0.55$ | $13.4$ |
| **Mana Regen** | $7.5$ | $+0.75$ | $20.2$ |
| **Armor** | $25.0$ | $+4.4$ | $99.8$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $55.0$ | $+2.5$ | $97.5$ |
| **Attack Speed** | $0.650$ | $+2.0\%$ | $0.871$ |
| **Movement Speed** | $325.0$ | $+0.0$ | $325.0$ |
| **Attack Range** | $575.0$ | $+0.0$ | $575.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.65$ |
| **Attack Speed Ratio** | $0.65$ |
| **Bonus AS per Level** | $2.0\%$ |
| **Attack Windup** | $16.3\%$ |
| **Missile Speed** | $2500 units/second$ |
| **Acquisition Radius** | $800 units$ |
| **Gameplay Radius** | $65 units$ |
| **Pathing Radius** | $30 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $125 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Vow of the First Lands

**Innate:** **Yunara**’s critical strike deal **bonus** magic damage.

**Innate:** ''Yunara's** critical strike deal 10% (+ 10% per 100 AP) **bonus'' magic damage.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Effects** | proc |

**Notes:**

No additional notes.

---

### Q: Cultivation of Spirit

**Passive:** **Yunara**’s basic attacks deal **bonus** magic damage on-hit and generate stacks of *Unleash* on-attack, stacking up to a cap upon which she can cast *Cultivation of Spirit*.

**Active:** **Yunara** unleashes for a few seconds: her basic attacks gain , deal **additional bonus** magic damage, and spread to deal physical damage to enemies near the target.

**Passive:** ''Yunara's** basic attacks deal **bonus'* magic damage on-hit. While this ability is inactive, her basic attacks on-attack generate a stack of *Unleash' for 6 seconds, increased to 2 stacks against champions. The stacks refresh on subsequent attacks, stack up to 8 times, and expire by one every $0.5$ seconds after the duration. **Active:** **Yunara** unleashes for 5 seconds: gaining **bonus attack speed**, dealing **additional bonus** magic damage on-hit, and augmenting her basic attacks to fly faster, have a 40% lower attack windup time, and spread to each enemy near the target, dealing 30% AD physical damage. Each spread attack applies and on-hit effects, with on-hit damage reduced to 30% effectiveness, and will critically strike if the triggering attack does. Spread attacks deal 250% damage against minions below 30% **maximum** health. *Cultivation of Spirit basic attack reset *'Yunara's* basic attack timer. Targets do not have to be sight to be hit by the spread attacks.*

| Attribute | Value |
|-----------|-------|
| **Cast Time** | none |
| **Cost** | 30 Mana + 8 Unleash Stacks |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical Magic |
| **Speed** | 10000 / 2000 units/second |
| **Effect Radius** | 300 units |
| **Spell Shield** | False |
| **Spell Effects** | Special |
| **Projectile** | unknown |

**Scaling:**
- **Passive Bonus Magic Damage:** $ (+
- **Bonus Attack Speed:** $25-65$% **Active Bonus Magic Damage:* $ (+ *Active Increased Minion Damage:** $ (+ 2.5 to (+)*2.5 (+ % AP)

**Notes:**

- 'Cultivation of Spirit's damage classifications:
  - Proc damage for the passive and active on-hit damage.
  - Basic damage for the spread attack damage.
- Spread attacks will still deal damage even if **Yunara** misses her basic attack due to blind.
- Spread attack interaction with parrying effects (dodge, block)
- Spread attack interaction with projectile-interception effects (*E*, *W*, *W*, *W*)
- There is no limit to the number of enemies the empowered attacks can spread to.
- ''Yunara's* secondary resource bar gives various context for *Cultivation of Spirit'. It is visible to the player only.
  - While *Cultivation of Spirit* is not active, the bar shows her current *Unleash* stacks. It is colored white while below maximum stacks and yellow while at maximum stacks.
  - After *Cultivation of Spirit* is cast, the bar counts down the buff's duration. *** This is also true during *Transcend One's Self*, which means that the bar effectively also counts down the duration of her *Transcendent State*.
- If **Yunara** begins an attack windup toward the end of the active effects' duration and its launch would take place after it, the effects' duration is extended equal to the attack's windup time, once at most.
  - Since the buff ends right after launching 'Cultivation of Spirit's* attack and its effects have already been determined, this allows it to both spread to nearby enemies and generate a stack of *Unleash'.
- Each bolt from *Runaan's Hurricane* Wind's Fury may trigger its own spread attack (despite the spread attacks not being an on-hit effect).

---

### W: Arc of Judgment

**Active:** **Yunara** launches a spinning prayer bead that briefly slows down upon hitting an enemy. The initial hit against each enemy deals magic damage and slows the target, and the bead continually deals magic damage to nearby enemies.

*Upon reaching maximum range, the bead expands and lingers briefly.*

**Active:** **Yunara** launches a spinning prayer bead in the target direction that briefly grants sight of its surroundings as it travels. The bead slows down significantly while colliding with an enemy, lingering as it travels and resetting its remaining duration; upon reaching maximum range, it will expand and linger in place for 1 second regardless of enemies hit. The initial hit against each enemy before the bead's expansion deals magic damage and slow them by 99% decaying over $1.5$ seconds. While lingering, the bead deals magic damage to nearby enemies every $0.25$ seconds. *Arc of Judgment* deals 50@1; 75@9; 100@13 (@=%) damage against minions and execute them if they would be killed by 6 more damage instances from the linger effect.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 10 seconds |
| **Cast Time** | 0.45–0.225@0–100 (@=*bonus attack speed) seconds |
| **Cost** | 60 mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Speed** | 2150 / 150 units/second |
| **Effect Radius** | 150 units |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |

**Scaling:**
- **Initial Magic Damage:** $5-105$ (+ 85% AD) (+ 50% AP)
- **Linger Magic Damage per Tick:** $5/4-65/4$ (+ $40/4$% AD) (+ $25/4$% AP)
- **Total Expanded Damage:** $(5/4)*4-(65/4)*4$ (+ $(40/4)*4$% AD) (+ $(25/4)*4$% AP)

**Notes:**

- Applies area damage for the initial hit and persistent area damage for the lingering effect.
- Spell shield can block either the initial hit or one damage tick of the lingering effect.Effect at cast time end
- *Arc of Judgment* can hit an enemy more than once with the initial damage as well with the lingering effect (both while not expanded and while expanded).
  - The bead can hit an enemy multiple times if they collide with it again while it is in flight.

---

### W: Arc of Ruin

**Active:** **Yunara** fires a beam of spirit magic in the target direction that deals magic damage to enemies and slows them for a short duration.

**Active:** **Yunara** fires a beam of spirit magic in the target direction that deals 50–350@1–3 (@=[[File:Yunara Transcend One's Self.png) (+ 175% AD) (+ 75% AP) magic damage to enemies hit and slow them by 99% decaying over 1 second, as well as granting sight of the surrounding area. *This ability benefits from ultimate ability effects and is the upgraded version of *Arc of Judgment* during *Transcendent State*.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 10 seconds |
| **Cast Time** | 0.6–0.45@0–100 (@=*bonus attack speed) seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Notes:**

- *Arc of Ruin* grants sight of its surroundings during the cast time and for $0.3$ seconds afterwards.Effect at cast time end
- *Arc of Ruin* does not trigger ultimate *cast* effects, such as *Experimental Hexplate* Overdrive, *Zeke's Convergence* Frostfire Tempest and possessive=true increased movement speed. This is intended.

---

### E: Kanmei's Steps

**Active:** **Yunara** briefly becomes ghosted and gains decaying *ms **bonus** move speed*, increased while facing an enemy champion.

**Active:** **Yunara** becomes ghosted and gains **bonus movement speed** decaying over $1.5$ seconds. The movement speed is 50% more effective while facing a nearby sight enemy champion.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $7.5$ seconds |
| **Cast Time** | none |
| **Cost** | 40 Mana |
| **Targeting** | Auto |
| **Affects** | Self |
| **Effect Radius** | 2000 units |

**Scaling:**
- **Bonus Move Speed:* $30-50$%1.5-50×1.5$%

**Notes:**

No additional notes.

---

### E: Untouchable Shadow

**Active:** **Yunara** dashes in the target direction.

**Active:** **Yunara** dashes in the target direction. *Untouchable Shadow basic attack reset *'Yunara's' basic attack timer. *Arc of Ruin* can be cast during the dash. This ability benefits from ultimate ability effects and is the upgraded version of *Kanmei's Steps* during *Transcendent State*.'

| Attribute | Value |
|-----------|-------|
| **Range** | 225 – 450 / 550 units |
| **Cooldown** | $7.5$ seconds |
| **Cast Time** | none |
| **Targeting** | Location |
| **Affects** | Self |
| **Speed** | 1350–1650@1–3 (@=[[File:Yunara Transcend One's Self.png) |

**Notes:**

- *Untouchable Shadow* does not trigger ultimate *cast* effects, such as *Experimental Hexplate* Overdrive, *Zeke's Convergence* Frostfire Tempest and possessive=true increased movement speed. This is intended.

---

### R: Transcend One's Self

**Passive:** **Arc of Ruin*’s* damage and *Untouchable Shadow*’s dash speed are increased.

**Active:** **Yunara** enters *Transcendent State* for some time.

**Passive:** **Arc of Ruin*’s* base damage and **Untouchable Shadow*’s* dash speed scale with 'Transcend One's Self's rank. **Active:** **Yunara** enters *Transcendent State* for 15 seconds, during which each of her basic abilities is empowered: Automatically becomes active at no cost, with the duration increased to 'Transcendent State's*. Upgraded into **Arc of Ruin*', which has no cost and has its **remaining cooldown** reduced by 80% upon both entering and exiting the state. Upgraded into **Untouchable Shadow**, which has no cost and has its *cooldown* reset upon both entering and exiting the state.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $100-80$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- * Arc of Ruin Base Damage* Untouchable Shadow Dash Speed $1350-1650$

**Notes:**

- An ongoing '*Kanmei's Steps*’s* effect is not lost when quickly replaced by **Untouchable Shadow*'.
- *Cultivation of Spirit* is cast alongside 'Transcend One's Self'. If the former was not already active, this will also grant its basic attack reset.
  - The attack reset does **not** originate from 'Transcend One's Self* nor is *Transcend One's Self' tagged as granting one itself (e.g. for the purposes of *Hail of Blades*).
- 'Transcend One's Self' cannot be cast again while active.

---

## Patch History

### V25.18
- Stats
  - Base health reduced to 575 from 600.
  - Base attack damage reduced to 53 from 56.
- *Cultivation of Spirit*
  - **Bug Fixes:** Remedied multiple issues related to using Cleave items (*Tiamat* & upgrades).
- *Kanmei's Steps*
  - Bonus movement speed reduced to $30-50$% from $50-70$%.
  - Duration reduced to $1.5$ seconds at all ranks from $1.5-2.5$.

### V25.17
- *Cultivation of Spirit*
  - **Bug Fixes:** On-hit damage on spread attacks caused by *Runaan's Hurricane* Wind's Fury bolts that hit the primary target now properly apply at reduced effectiveness instead of full effectiveness (as if a primary attack hit them).
  - **Bug Fixes:** *Runaan's Hurricane* Wind's Fury bolt damage is no longer incorrectly also modified by 'Cultivation of Spirit's reduced on-hit effectiveness on each of the bolts' primary targets.
  - **Bug Fixes:** Item on-hit effects no longer fail to deal damage on valid targets when a sufficiently large number of spread attacks are taking place simultaneously.
  - **Bug Fixes:** Item on-hit effects which stack now gain and consume stacks at the correct timing and can no longer sometimes trigger twice per individual hit when a sufficiently large number of spread attacks are taking place simultaneously.

### V25.16
- *Cultivation of Spirit*
  - **Bug Fixes:** No longer applies spell effects against the main target while active.
  - **Bug Fixes:** VFX no longer splits to invalid targets that are within the spread radius.
    - *[Note: This has no gameplay effect.]*

### V25.15
- General
  - Adjusted attack animations to better incorporate her beads.
  - Added more visual effects to *Transcend One's Self* to better indicate that its effects are active.
- *Vow of the First Lands*
  - **Bug Fixes:** *Shadowflame* Cinderbloom now properly triggers on the ability's damage.
- *Cultivation of Spirit*
  - **Bug Fixes:** Is now correctly tagged as a basic attack reset for the purposes of *Hail of Blades*.
  - **Undocumented / Bug Fix:** Spread attack no longer applies spell effects.
- *Arc of Judgment*
  - **Bug Fixes:** In the Chinese Simplified and Traditional localizations, no longer displays incorrect values.
- *Untouchable Shadow*
  - **Bug Fixes:** Is now correctly tagged as a basic attack reset for the purposes of *Hail of Blades*.
- *Transcend One's Self*
  - **Bug Fixes:** *Axiom Arc* Flux no longer reduces its cooldown twice upon triggering the effect.

### V25.14
- Stats
  - Health: 600 (+ 110)
  - Health regeneration: 4 (+ $0.55$)
  - Mana: 275 (+ 45)
  - Mana regeneration: $7.5$ (+ $0.75$)
  - Armor: 25 (+ $4.4$)
  - Magic resistance: 30 (+ $1.3$)
  - Attack damage: 56 (+ $2.5$)
  - Attack speed: $0.65$ (+ 2%, $0.65$ ratio)
  - Movement speed: 325
  - Basic attack range: 575
  - Gameplay radius: 65
  - Pathing radius: 30
  - Selection radius: 100
  - Selection height: 125
  - Acquisition radius: 800
- *Vow of the First Lands* - Innate
  - **Innate:** ''Yunara's** critical strike deal 10% (+ 10% per 100 AP) **bonus'' magic damage.
- *Cultivation of Spirit* - Q
  - **Passive:** ''Yunara's** basic attacks deal $5-25$ (+ 20% AP) **bonus'* magic damage on-hit. While this ability is inactive, her basic attacks on-attack generate a stack of *Unleash' for 6 seconds, increased to 2 stacks against champions. The stacks refresh on subsequent attacks, stack up to 8 times, and expire by one every $0.5$ seconds after the duration.
  - **Active:** **Yunara** unleashes for 5 seconds: gaining $25-65$% **bonus attack speed**, dealing $5-25$ (+ 20% AP) **additional bonus** magic damage on-hit, and augmenting her basic attacks to fly faster, have a 40% lower attack windup time, and spread to each enemy near the target, dealing 30% AD physical damage.
  - Each spread attack life steal applies lifesteal and on-hit effects (30% for on-hit damage), and will critically strike if the triggering attack does. Spread attacks deal 250% damage against minions below 30% **maximum** health.
  - *Cultivation of Spirit basic attack reset *'Yunara's* basic attack timer. Targets do not have to be sight to be hit by this ability.*
  - **Cost:** 30 mana + 8 Unleash Stacks.
  - **Cast Time:** None, **Effect Radius:** 300, **Speed:** 10000 / 2000, **Queue Threshold:** $0.05$ seconds.
- *Arc of Judgment* - W
  - **Active:** **Yunara** launches a spinning prayer bead in the target direction that briefly grants sight of its surroundings as it travels. The bead slows down significantly while colliding with an enemy, lingering as it travels and resetting its remaining duration; upon reaching maximum range, it will expand and linger in place for 1 second regardless of enemies hit.
  - The initial hit against each enemy before the bead's expansion deals $5-105$ (+ 85% AD) (+ 50% AP) magic damage and slow them by 99% decaying over $1.5$ seconds. While lingering, the bead deals $5/4-65/4$ (+ $40/4$% AD) (+ $25/4$% AP) magic damage to nearby enemies every $0.25$ seconds.
  - *Arc of Judgment* deals 50@1; 75@9; 100@13 (@=%) damage against minions and execute them if they would be killed by 6 more damage instances from the linger effect.
  - **Cost:** 60 mana.
  - **Cooldown:** 10 seconds.
  - **Cast Time:** 0.45–0.225@0–100 (@=*bonus attack speed), **Effect Radius:** 150, **Range:** 1150, **Width:** 120, **Speed:** 1150.
  - While under the effects of *Transcend One's Self*, this ability becomes *Arc of Ruin*:
    - **Active:** **Yunara** fires a beam of spirit magic in the target direction that deals 50–350@1–3 (@=[File:Yunara Transcend One's Self.png) (+ 175% AD) (+ 75% AP) magic damage to enemies hit and slow them by 99% decaying over 1 second, as well as granting sight of the surrounding area.
    - *This ability benefits from ultimate ability effects and is the upgraded version of *Arc of Judgment* during *Transcendent State*.*
    - **Cooldown:** 10 seconds.
    - **Cast Time:** 0.6–0.45@0–100 (@=*bonus attack speed), **Range:** er 1150, **Width:** 120.
- *Kanmei's Steps* - E
  - **Active:** **Yunara** becomes ghosted and gains $50-70$% **bonus movement speed** decaying over $1.5-2.5$ seconds. The movement speed is 50% more effective while facing a nearby sight enemy champion.
  - **Cost:** 40 mana.
  - **Cooldown:** $7.5$ seconds (starts on cast).
  - **Cast Time:** None, **Effect Radius:** cr 2000, **Queue Threshold:** $0.05$ seconds.
  - While under the effects of *Transcend One's Self*, this ability becomes *Untouchable Shadow*:
    - **Active:** **Yunara** dashes in the target direction.
    - *Untouchable Shadow basic attack reset *'Yunara's' basic attack timer. *Arc of Ruin* can be cast during the dash. This ability benefits from ultimate ability effects and is the upgraded version of *Kanmei's Steps* during *Transcendent State*.'
    - **Cooldown:** $7.5$ seconds.
    - **Cast Time:** None, **Target Range:** 225 – 450 / 550, **Speed:** 1350–1650@1–3 (@=[[File:Yunara Transcend One's Self.png), **Queue Threshold:** $0.05$ seconds.
- *Transcend One's Self* - R
  - **Passive:** **Arc of Ruin*’s* base damage and **Untouchable Shadow*’s* dash speed scale with 'Transcend One's Self's rank.
  - **Active:** **Yunara** enters *Transcendent State* for 15 seconds, during which each of her basic abilities is empowered:
    - ***Cultivation of Spirit* Automatically becomes active at no cost, with the duration increased to 'Transcendent State's.
    - ***Arc of Judgment* Upgraded into **Arc of Ruin**, which has no cost and has its **remaining cooldown** reduced by 80% upon both entering and exiting the state.
    - ***Kanmei's Steps* Upgraded into **Untouchable Shadow**, which has no cost and has its *cooldown* reset upon both entering and exiting the state.
  - **Cost:** 100 mana.
  - **Cooldown:** $100-80 3$ seconds.
  - **Cast Time:** None.

## Trivia

- Her buff flavor text reads:
  - *Too Early* (*Cultivation of Spirit* active): '"You're a thousand years too early for this fight." — Yunara'
    - This is a reference to [a common trope in media.
  - *The Wonderful and Dynamic Prayer Bead* (*Arc of Judgment* slow): *"Take this prayer bead and eat it." — Yunara*
    - This might be a reference to a popular meme from the *Death Note* anime.
  - *Must go Fast!* (*Kanmei's Steps* active): *You thought you could outrun me! — Yunara*
    - This might be referencing Sonic_X#Popularity_and_cultural_impact.
  - *Even Further Beyond* (*Transcend One's Self* active): *Congratulations, you work for The Kinkou Order now. — Yunara*
    - This is a reference to Goku's iconic line from the *Dragon Ball Z* anime.
  - *Going Somewhere?* (*Arc of Ruin* slow): *"All according to Kinkou-ku!" — Yunara*
    - This is a reference to a popular meme from a *Death Note* fansub.
  - *Keep Up!* (*Untouchable Shadow* dash): *"Every journey begins with a single step. But a fight begins when you catch me." — Yunara*
- Her dance is a reference to the Season 2 opening for *Mashle*.
  - A side-by-side comparison can be viewed here.
- ''Yunara's* **Cultivation of Spirit*' attack has the second highest missile speed on an attack out of any champion, at 10000.
  - Severum attacks have the highest speed, at 92400.
- Her exclusive Augment in Arena, **Quest: Three Sacred Treasures**, is named after the Imperial Regalia of Japan.
  - Its effect is based on a version of *Arc of Ruin* during development that deals physical and magic damage.

---
*This page was automatically generated from League of Legends Wiki data.*