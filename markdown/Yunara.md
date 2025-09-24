# Yunara

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
| **Champion** | Yunara |
| **Title** | the Unbroken Faith |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2025-07-16 |
| **Release Patch** | V25.14 |
| **Latest Changes** | V25.18 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |
| **Blue Essence** | 3150 |
| **Riot Points** | 975 |
| **Difficulty** | 2 |
| **Hero Type** | Marksman |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 2 |
| **Utility** | 0 |
| **Style** | 35 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $590.0$ | $+110.0$ |
| **Mana** | $275.0$ | $+45.0$ |
| **Health Regen** | $4.0$ | $+0.55$ |
| **Mana Regen** | $7.5$ | $+0.75$ |
| **Armor** | $25.0$ | $+4.4$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $55.0$ | $+2.5$ |
| **Attack Speed** | $0.650$ | |
| **Movement Speed** | $325.0$ | $+0.0$ |
| **Attack Range** | $575.0$ | $+0.0$ |
| **Base Attack Speed** | $0.65$ | |
| **Attack Speed Ratio** | $0.65$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Attack Windup** | $16.3\%$ | |
| **Missile Speed** | $2500$ units/second | |
| **Acquisition Radius** | $800$ units | |
| **Gameplay Radius** | $65$ units | |
| **Pathing Radius** | $30$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $125$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Damage Taken** | $100.0\%$ |
| **Healing** | $100.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $100.0\%$ |
| **Healing** | $100.0\%$ |

## Abilities

### Passive: Vow of the First Lands

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Effects** | proc |
| **Parry** | unknown |

**INNATE:** **Yunara**’s critical strikes deal 10% (+ 10% per 100 AP) **bonus** magic damage.

**Notes:**

No additional notes.

---

### Q: Cultivation of Spirit

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 300 units |
| **Speed** | 10000 (Empowered attack missile speed) / 2000 (Spread attack missile speed) units/second |
| **Cost** | 30 Mana + 8 Unleash Stacks |
| **Queue Time** | $0.05$ seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical Magic |
| **Spell Shield** | False |
| **Spell Effects** | Special |
| **Projectile** | unknown |
| **Parry** | unknown |

**PASSIVE:** **Yunara**’s basic attacks deal **bonus** magic damage on-hit. While this ability is inactive, her basic attacks on-attack generate a stack of *Unleash* for 6 seconds, increased to 2 stacks against champions. The stacks refresh on subsequent attacks, stack up to 8 times, and expire by one every $0.5$ seconds after the duration.

| Attribute | Value |
|-----------|------:|
| **Passive Bonus Magic Damage** | 5 / 10 / 15 / 20 / 25 (+ 20% AP) |

**ACTIVE:** **Yunara** unleashes for 5 seconds: gaining **bonus** attack speed, dealing **additional bonus** magic damage on-hit, and augmenting her basic attacks to fly faster, have a 40% lower windup time, and spread to each enemy near the target, dealing 30% AD physical damage.
Each spread attack applies life steal and on-hit effects, with on-hit damage reduced to 30% effectiveness, and will critically strike if the triggering attack does. Spread attacks deal 250% damage against minions below 30% **maximum** health.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 25 / 35 / 45 / 55 / 65% |

| Attribute | Value |
|-----------|------:|
| **Active Bonus Magic Damage** | 5 / 10 / 15 / 20 / 25 (+ 20% AP) |
| **Combined Bonus Magic Damage** | 10 / 20 / 30 / 40 / 50 (+ 40% AP) |

| Attribute | Value |
|-----------|------:|
| **Active Increased Minion Damage** | 12.5 / 25 / 37.5 / 50 / 62.5 (+ 50% AP) |
| **Combined Increased Minion Damage** | 25 / 50 / 75 / 100 / 125 (+ 100% AP) |

*Cultivation of Spirit resets **Yunara**’s basic attack timer. Targets do not have to be visible to be hit by the spread attacks.*

**Notes:**

- *Cultivation of Spirit*’s damage classifications:
  - Proc damage for the passive and active on-hit damage.
  - Basic damage for the spread attack damage.
- Spread attacks will still deal damage even if **Yunara** misses her basic attack due to blind.
- Spread attack interaction with parrying effects (dodge, block)
- Spread attack interaction with projectile-interception effects (E, W, W, W)
- There is no limit to the number of enemies the empowered attacks can spread to.
- **Yunara**’s secondary resource bar gives various context for *Cultivation of Spirit*. It is visible to the player only.
  - While *Cultivation of Spirit* is not active, the bar shows her current *Unleash* stacks. It is colored white while below maximum stacks and yellow while at maximum stacks.
  - After *Cultivation of Spirit* is cast, the bar counts down the buff's duration.
    - This is also true during Transcend One's Self, which means that the bar effectively also counts down the duration of her *Transcendent State*.
- If **Yunara** begins an attack windup toward the end of the active effects' duration and its launch would take place after it, the effects' duration is extended equal to the attack's windup time, once at most.
  - Since the buff ends right after launching *Cultivation of Spirit*’s attack and its effects have already been determined, this allows it to both spread to nearby enemies and generate a stack of *Unleash*.
- Each bolt from Runaan's Hurricane Wind's Fury may trigger its own spread attack (despite the spread attacks not being an on-hit effect).

---

### W: Arc of Judgment

| Attribute | Value |
|-----------|------:|
| **Range** | 1150 units |
| **Cast Time** | 0.45–0.225@0–100 (@=**bonus** attack speed) seconds |
| **Effect Radius** | 150 (Expanded bead radius) units |
| **Width** | 120 units |
| **Speed** | 2150 (Normal missile speed) / 150 (Lingering missile speed) units/second |
| **Cost** | 60 mana |
| **Cooldown** | 10 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |

**ACTIVE:** **Yunara** launches a spinning prayer bead in the target direction that briefly grants sight of its surroundings (Cannot see through terrain nor brush) as it travels. The bead slows down significantly while colliding with an enemy, lingering as it travels and resetting its remaining duration; upon reaching maximum range, it will expand and linger in place for 1 second regardless of enemies hit.

The initial hit against each enemy before the bead's expansion deals magic damage and slows them by 99% decaying over $1.5$ seconds. While lingering, the bead deals magic damage to nearby enemies every $0.25$ seconds.

| Attribute | Value |
|-----------|------:|
| **Initial Magic Damage** | 5 / 30 / 55 / 80 / 105 (+ 85% AD) (+ 50% AP) |

| Attribute | Value |
|-----------|------:|
| **Linger Magic Damage per Tick** | 1.25 / 5 / 8.75 / 12.5 / 16.25 (+ 10% AD) (+ 6.25% AP) |

| Attribute | Value |
|-----------|------:|
| **Total Expanded Damage** | 5 / 20 / 35 / 50 / 65 (+ 40% AD) (+ 25% AP) |

*Arc of Judgment* deals 50%@1; 75%@9; 100%@13 damage against minions and executes them if they would be killed by 6 more damage instances from the linger effect.

**Notes:**

- Applies area damage for the initial hit and persistent area damage for the lingering effect.
- Spell shield can block either the initial hit or one damage tick of the lingering effect.Effect at cast time end
- *Arc of Judgment* can hit an enemy more than once with the initial damage as well with the lingering effect (both while not expanded and while expanded).
  - The bead can hit an enemy multiple times if they collide with it again while it is in flight.

---

### W: Arc of Ruin

| Attribute | Value |
|-----------|------:|
| **Range** | 1150 units |
| **Cast Time** | 0.6–0.45@0–100 (@=**bonus** attack speed) seconds |
| **Width** | 120 units |
| **Cooldown** | 10 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**ACTIVE:** **Yunara** fires a beam of spirit magic in the target direction that deals 50–350@1–3 (@=[[File:Yunara Transcend One's Self.png) (+ 175% AD) (+ 75% AP) magic damage to enemies hit and slows them by 99% decaying over 1 second, as well as granting sight of the surrounding area (Cannot see through terrain nor brush).

*This ability benefits from ultimate ability effects (See notes) and is the upgraded version of Arc of Judgment during Transcendent State.*

**Notes:**

- *Arc of Ruin* grants sight of its surroundings during the cast time and for $0.3$ seconds (Estimated) afterwards.Effect at cast time end
- *Arc of Ruin* does not trigger ultimate *cast* effects, such as Experimental Hexplate Overdrive, Zeke's Convergence Frostfire Tempest and possessive=true increased movement speed. This is intended.

---

### E: Kanmei's Steps

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 2000 (Enemy champion check) units |
| **Cost** | 40 Mana |
| **Cooldown** | $7.5$ seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.05$ seconds |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Yunara** becomes ghosted and gains **bonus** movement speed decaying over $1.5$ seconds. The movement speed is 50% more effective while facing a nearby visible enemy champion.

| Attribute | Value |
|-----------|------:|
| **Bonus Move Speed** | 30 / 35 / 40 / 45 / 50% |
| **Increased Bonus Move Speed** | 45 / 52.5 / 60 / 67.5 / 75% |

**Notes:**

No additional notes.

---

### E: Untouchable Shadow

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 225 (Minimum dash range) – 450 (Maximum dash range) / 550 (Maximum extended dash range through terrain) units |
| **Speed** | 1350–1650@1–3 (@=[[File:Yunara Transcend One's Self.png) |
| **Cooldown** | $7.5$ seconds |
| **Queue Time** | 0.05 seconds |
| **Targeting** | Location |
| **Affects** | Self |
| **Out of Range Behavior** | cast at max |
| **Terrain Grace** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Yunara** dashes in the target direction.

*Untouchable Shadow resets **Yunara**’s basic attack timer. Arc of Ruin can be cast during the dash. This ability benefits from ultimate ability effects (See notes) and is the upgraded version of Kanmei's Steps during Transcendent State.*

**Notes:**

- *Untouchable Shadow* does not trigger ultimate *cast* effects, such as Experimental Hexplate Overdrive, Zeke's Convergence Frostfire Tempest and possessive=true increased movement speed. This is intended.

---

### R: Transcend One's Self

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Cooldown** | 100 / 95 / 90 / 85 / 80 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self |

**PASSIVE:** *Arc of Ruin’s* base damage and *Untouchable Shadow’s* dash speed scale with *Transcend One's Self*’s rank.

| Attribute | Value |
|-----------|------:|
| ** Arc of Ruin Base Damage |

| Attribute | Value |
|-----------|------:|
| ** Untouchable Shadow Dash Speed |

**ACTIVE:** **Yunara** enters *Transcendent State* for 15 seconds, during which each of her basic abilities is empowered:
 Automatically becomes active at no cost, with the duration increased to *Transcendent State*’s.
 Upgraded into *Arc of Ruin*, which has no cost and has its **remaining** cooldown reduced by 80% upon both entering and exiting the state.
 Upgraded into *Untouchable Shadow*, which has no cost and has its cooldown reset upon both entering and exiting the state.

**Scaling:**
| Attribute | Value |
|-----------|------:|
| **[[File: Yunara Untouchable Shadow.png** | 20px |
| **border** | link=]] Untouchable Shadow Dash Speed |

**Notes:**

- An ongoing *Kanmei's Steps’* effect is not lost when quickly replaced by *Untouchable Shadow*.
- Cultivation of Spirit is cast alongside *Transcend One's Self*. If the former was not already active, this will also grant its attack reset.
  - The attack reset does **not** originate from *Transcend One's Self* nor is *Transcend One's Self* tagged as granting one itself (e.g. for the purposes of Hail of Blades).
- *Transcend One's Self* cannot be cast again while active.

---

## Patch History

### V25.18#September 11th Hotfix|V25.18
- Stats
  - Base health increased to 590 from 575.
  - Base attack damage increased to 55 from 53.

### V25.18
- Stats
  - Base health reduced to 575 from 600.
  - Base attack damage reduced to 53 from 56.
- Cultivation of Spirit
  - **Bug Fixes:** Remedied multiple issues related to using Cleave items (Tiamat & upgrades).
- Kanmei's Steps
  - Bonus movement speed reduced to 30 / 35 / 40 / 45 / 50% from 50 / 55 / 60 / 65 / 70%.
  - Duration reduced to $1.5$ seconds at all ranks from 1.5 / 1.75 / 2 / 2.25 / 2.5.

### V25.17
- Cultivation of Spirit
  - **Bug Fixes:** On-hit damage on spread attacks caused by Runaan's Hurricane Wind's Fury bolts that hit the primary target now properly apply at reduced effectiveness instead of full effectiveness (as if a primary attack hit them).
  - **Bug Fixes:** Runaan's Hurricane Wind's Fury bolt damage is no longer incorrectly also modified by *Cultivation of Spirit*’s reduced on-hit effectiveness on each of the bolts' primary targets.
  - **Bug Fixes:** Item on-hit effects no longer fail to deal damage on valid targets when a sufficiently large number of spread attacks are taking place simultaneously.
  - **Bug Fixes:** Item on-hit effects which stack now gain and consume stacks at the correct timing and can no longer sometimes trigger twice per individual hit when a sufficiently large number of spread attacks are taking place simultaneously.

### V25.16
- Cultivation of Spirit
  - **Bug Fixes:** No longer applies spell effects against the main target while active.
  - **Bug Fixes:** VFX no longer splits to invalid targets that are within the spread radius.
    - *[Note: This has no gameplay effect.]*

### V25.15
- General
  - Adjusted attack animations to better incorporate her beads.
  - Added more visual effects to Transcend One's Self to better indicate that its effects are active.
- Vow of the First Lands
  - **Bug Fixes:** Shadowflame Cinderbloom now properly triggers on the ability's damage.
- Cultivation of Spirit
  - **Bug Fixes:** Is now correctly tagged as a basic attack reset for the purposes of Hail of Blades.
  - **UNDOCUMENTED / BUG FIX:** Spread attack no longer applies spell effects.
- Arc of Judgment
  - **Bug Fixes:** In the Chinese Simplified and Traditional localizations, no longer displays incorrect values.
- Untouchable Shadow
  - **Bug Fixes:** Is now correctly tagged as a basic attack reset for the purposes of Hail of Blades.
- Transcend One's Self
  - **Bug Fixes:** Axiom Arc Flux no longer reduces its cooldown twice upon triggering the effect.

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
- Vow of the First Lands - Innate
  - **INNATE:** **Yunara**’s critical strikes deal 10% (+ 10% per 100 AP) **bonus** magic damage.
- Cultivation of Spirit - Q
  - **PASSIVE:** **Yunara**’s basic attacks deal 5 / 10 / 15 / 20 / 25 (+ 20% AP) **bonus** magic damage on-hit. While this ability is inactive, her basic attacks on-attack generate a stack of *Unleash* for 6 seconds, increased to 2 stacks against champions. The stacks refresh on subsequent attacks, stack up to 8 times, and expire by one every $0.5$ seconds after the duration.
  - **ACTIVE:** **Yunara** unleashes for 5 seconds: gaining 25 / 35 / 45 / 55 / 65% **bonus** attack speed, dealing 5 / 10 / 15 / 20 / 25 (+ 20% AP) **additional bonus** magic damage on-hit, and augmenting her basic attacks to fly faster, have a 40% lower windup time, and spread to each enemy near the target, dealing 30% AD physical damage.
  - Each spread attack life steal applies lifesteal and on-hit effects (30% for on-hit damage), and will critically strike if the triggering attack does. Spread attacks deal 250% damage against minions below 30% **maximum** health.
  - **Cultivation of Spirit resets **Yunara**’s basic attack timer. Targets do not have to be visible to be hit by this ability.*
  - **COST:** 30 mana + 8 Unleash Stacks.
  - **CAST TIME:** None, **EFFECT RADIUS:** 300, **SPEED:** 10000 (Empowered attack missile speed) / 2000 (Spread attack missile speed), **QUEUE THRESHOLD:** $0.05$ seconds.
- Arc of Judgment - W
  - **ACTIVE:** **Yunara** launches a spinning prayer bead in the target direction that briefly grants sight of its surroundings (Cannot see through terrain nor brush) as it travels. The bead slows down significantly while colliding with an enemy, lingering as it travels and resetting its remaining duration; upon reaching maximum range, it will expand and linger in place for 1 second regardless of enemies hit.
  - The initial hit against each enemy before the bead's expansion deals 5 / 30 / 55 / 80 / 105 (+ 85% AD) (+ 50% AP) magic damage and slows them by 99% decaying over $1.5$ seconds. While lingering, the bead deals 1.25 / 5 / 8.75 / 12.5 / 16.25 (+ 10% AD) (+ 6.25% AP) magic damage to nearby enemies every $0.25$ seconds.
  - *Arc of Judgment* deals 50%@1; 75%@9; 100%@13 damage against minions and executes them if they would be killed by 6 more damage instances from the linger effect.
  - **COST:** 60 mana.
  - **COOLDOWN:** 10 seconds.
  - **CAST TIME:** 0.45–0.225@0–100 (@=**bonus** attack speed), **EFFECT RADIUS:** 150 (Expansion), **RANGE:** 1150, **WIDTH:** 120, **SPEED:** 1150.
  - While under the effects of Transcend One's Self, this ability becomes Arc of Ruin:
    - **ACTIVE:** **Yunara** fires a beam of spirit magic in the target direction that deals pp|50 to 350 for 3/1 to 3|type

### File:Yunara Transcend One's Self.png|20px|border|link=
    - *This ability benefits from ultimate ability effects and is the upgraded version of Arc of Judgment during Transcendent State.*
    - **COOLDOWN:** 10 seconds.
    - **CAST TIME:** 0.6–0.45@0–100 (@=**bonus** attack speed), **RANGE:** er 1150, **WIDTH:** 120.
- Kanmei's Steps - E
  - **ACTIVE:** **Yunara** becomes ghosted and gains 50 / 55 / 60 / 65 / 70% **bonus** movement speed decaying over 1.5 / 1.75 / 2 / 2.25 / 2.5 seconds. The movement speed is 50% more effective while facing a nearby visible enemy champion.
  - **COST:** 40 mana.
  - **COOLDOWN:** $7.5$ seconds (starts on cast).
  - **CAST TIME:** None, **EFFECT RADIUS:** cr 2000 (Enemy champion check), **QUEUE THRESHOLD:** $0.05$ seconds.
  - While under the effects of Transcend One's Self, this ability becomes Untouchable Shadow:
    - **ACTIVE:** **Yunara** dashes in the target direction.
    - **Untouchable Shadow resets **Yunara**’s basic attack timer. Arc of Ruin can be cast during the dash. This ability benefits from ultimate ability effects and is the upgraded version of Kanmei's Steps during Transcendent State.*
    - **COOLDOWN:** $7.5$ seconds.
    - **CAST TIME:** None, **TARGET RANGE:** 225 (Minimum dash range) – 450 (Maximum dash range) / 550 (Maximum extended dash range through terrain), **SPEED:** pp|1350 to 1650 for 3/1 to 3|type

### File:Yunara Transcend One's Self.png|20px|border|link=
- Transcend One's Self - R
  - **PASSIVE:** *Arc of Ruin’s* base damage and *Untouchable Shadow’s* dash speed scale with *Transcend One's Self*’s rank.
  - **ACTIVE:** **Yunara** enters *Transcendent State* for 15 seconds, during which each of her basic abilities is empowered:
    - **CULTIVATION OF SPIRIT:** Automatically becomes active at no cost, with the duration increased to *Transcendent State*’s.
    - **ARC OF JUDGMENT:** Upgraded into *Arc of Ruin*, which has no cost and has its **remaining** cooldown reduced by 80% upon both entering and exiting the state.
    - ***KANMEI'S STEPS:*** Upgraded into *Untouchable Shadow*, which has no cost and has its cooldown reset upon both entering and exiting the state.
  - **COST:** 100 mana.
  - **COOLDOWN:** 100 / 90 / 80 seconds.
  - **CAST TIME:** None.

## Trivia

- Her buff flavor text reads:
  - *Too Early* (Cultivation of Spirit active): *"You're a thousand years too early for this fight." — Yunara*
    - This is a reference to a common trope in media.
  - *The Wonderful and Dynamic Prayer Bead* (Arc of Judgment slow): *"Take this prayer bead and eat it." — Yunara*
    - This might be a reference to a popular meme from the *Death Note* anime.
  - *Must go Fast!* (Kanmei's Steps active): *You thought you could outrun me! — Yunara*
    - This might be referencing Sonic_X#Popularity_and_cultural_impact.
  - *Even Further Beyond* (Transcend One's Self active): *Congratulations, you work for The Kinkou Order now. — Yunara*
    - This is a reference to Goku's iconic line from the *Dragon Ball Z* anime.
  - *Going Somewhere?* (Arc of Ruin slow): *"All according to Kinkou-ku!" — Yunara*
    - This is a reference to a popular meme from a *Death Note* fansub.
  - *Keep Up!* (Untouchable Shadow dash): *"Every journey begins with a single step. But a fight begins when you catch me." — Yunara*
- Her dance is a reference to the Season 2 opening for *Mashle*.
  - A side-by-side comparison can be viewed here.
- **Yunara**’s *Cultivation of Spirit* attack has the second highest missile speed on an attack out of any champion, at 10000.
  - Aphelios’ Severum attacks have the highest speed, at 92400.
- Her exclusive Augment in Arena, **Quest: Three Sacred Treasures**, is named after the Imperial Regalia of Japan.
  - Its effect is based on a version of Arc of Ruin during development that deals physical and magic damage.

---
*This page was automatically generated from League of Legends Wiki data.*