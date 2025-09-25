# Bel'Veth

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Map-specific Stats](#map-specific-stats)
- [Pets](#pets)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Bel'Veth |
| **Title** | the Empress of the Void |
| **Resource** | None |
| **Range Type** | Melee |
| **Release Date** | 2022-06-09 |
| **Release Patch** | V12.11 |
| **Latest Changes** | V25.17 |
| **Roles** | Skirmisher |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 1 |
| **Style** | 0 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $610.0$ | $+99.0$ |
| **Mana** | $0.0$ | $+0.0$ |
| **Health Regen** | $6.0$ | $+0.6$ |
| **Armor** | $32.0$ | $+4.7$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $60.0$ | $+1.5$ |
| **Attack Speed** | $0.850$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $150.0$ | $+0.0$ |
| **Base Attack Speed** | $0.85$ | |
| **Attack Speed Ratio** | $0.85$ | |
| **Bonus AS per Level** | $0.0\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $110$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $95.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $100.0\%$ |

## Pets

### Void Remora

| Attribute | Value |
|-----------|------:|
| **Range** | 110 (Melee) / 550 (Ranged) units |
| **Gold** | 3 |
| **Experience** | 0 |
| **Health** | 20 / 45 / 70% of minion's **maximum** health |
| **Armor** | 0 (Based on game time for melees) |
| **Magic Resist** | 0 |
| **Damage** | 110% of minion's AD |
| **Damage Type** | Physical |
| **Attack Speed** | / |
| **Move Speed** | 505 |
| **Control** | Autonomous |
| **Targeting** | Lane Minion, but does not count towards the minion kill tracking score |
| **Spell Effects** | *Void Remora*’s basic attacks apply spell effects as basic damage. |
| **On-Hit** | *Void Remora*’s attacks are mitigated by dodge, block, and blind. |

**Abilities:**

- **Reborn:** *Void Remora* are summoned by all lane minions that are killed while **Bel'Veth** is within 1000 (Pending for test) units of them including allied ones. They emerge from the Void after $0.75$ seconds of the minion dying. Each type of minion killed spawns a certain number of *Void Remora*:
- Blue Melee Minion - 1 Melee
- Blue Caster Minion - 1 Ranged
- Blue Siege Minion - 1 Melee and 1 Ranged
- Blue Super Minion - 2 Melee and 2 Ranged
- **Replicate:** *Void Remora* are spawned with the stats that a minion would have at the time of the match.
- **Serve:** *Void Remora* automatically move down the lane in which they were spawned in and attack any enemy within their path. **Bel'Veth** receives the credit for any unit the *Void Remora* kill.

---

### Void Coral

| Attribute | Value |
|-----------|------:|
| **Control** | N/A |
| **Targeting** | N/A |

**Abilities:**

- **Consumable:** *Void Corals* are spawned from the corpses of epic monsters and champions that **Bel'Veth** has taken down. She may consume the *Void Coral* by casting Endless Banquet on it to embrace her *True Form*. They cannot be targeted by any unit but **Bel'Veth**.
- **From the Void:** *Void Corals* spawned from the corpses of Baron Nashor and Rift Herald empower **Bel'Veth**’s *True Form* with an extended duration and the ability to summon *Void Remora* from the deaths of lane minions.

---

## Abilities

### Passive: Death in Lavender

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |

**INNATE:** **Bel'Veth** attacks 36% faster than the median champion and her attack speed cap is modified to $9999.0$, but all sources of damage (including on-hit damage) dealt by her basic attacks are reduced to 75%. Her attack speed does not increase through growth (per level).

**INNATE:** **Bel'Veth**’s ability casts each generate 2 stacks of *Death in Lavender*, lasting for 5 seconds, refreshing on basic attacks and subsequent casts, and stacking up to 6 times. While **Bel'Veth** has stacks, she becomes ghosted and gains 20% / 40% **bonus** attack speed. Her basic attacks each consume 1 stack.

**INNATE:** Whenever **Bel'Veth** scores a takedown against a champion, monster, or large minion within 3 seconds of damaging them, she generates a permanent stack of *Lavender*. Large minions and monsters generate 1 stack, champions and epic monsters generate 2 stacks.


**LAVENDER:** For each stack, **Bel'Veth** gains 0.28 to 1.1 **bonus** attack speed.

**Notes:**

- *Death in Lavender*’s takedown effect will not trigger against monsters if they were slain by an enemy.
- *Death in Lavender* stacks are indicated under **Bel'Veth**’s health bar.
- **Bel'Veth**’s basic attack damage modifier will reduce the damage dealt by on-hit effects even if they aren't applied on-hit.

---

### Q: Void Surge

| Attribute | Value |
|-----------|------:|
| **Range** | 400 (Fixed dash distance) units |
| **Cast Time** | None |
| **Collision Radius** | 100 (Center-to-edge) units |
| **Speed** | 800 / 850 / 900 / 950 / 1000 units/second |
| **Cooldown** | 1 seconds |
| **Queue Time** | $0.35$ seconds |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Terrain Grace** | True |
| **Parry** | Special |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |
| **Per-Direction Cooldown** | 16 / 15 / 14 / 13 / 12 (Unaffected by ability haste) |

**ACTIVE:** **Bel'Veth** dashes in the designated direction, though not through terrain, dealing physical damage to enemies she passes through. Against the first target, *Void Surge* can critically strike for damage and applies on-hit effects, with on-hit damage reduced to 75% effectiveness, and life steal at 100% effectiveness.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 0 / 5 / 10 / 15 / 20 (+ 100% AD) |

*Void Surge* deals **bonus** physical damage to monsters and modified damage against minions.

| Attribute | Value |
|-----------|------:|
| **Monster Bonus Physical Damage** | 55 / 65 / 75 / 85 / 95 |
| **Total Monster Damage** | 55 / 70 / 85 / 100 / 115 (+ 100% AD) |

| Attribute | Value |
|-----------|------:|
| **Modified Minion Damage** | 60 / 70 / 80 / 90 / 100% |

*Void Surge* can be cast only within a cardinal direction that is off cooldown, and incurs a cooldown between casts. Each cardinal direction (Total of 4 directions) has a unique cooldown that is reduced equivalent to $0.25$ per . These cooldowns reset upon respawning.

*Void Surge resets **Bel'Veth**’s basic attack timer.*

**Notes:**

- Applies area damage and procs basic damage required effects.
- The icon partially darkens based on the combination of dashes available. When all dashes are expended, the cooldown indicates the next available dash.
- While in True Form, *Void Surge*’s dash distance is extended up to 225 (Pending for test) units when targeted across terrain.
- The basic attack reset is not considered one for Hail of Blades.
- *Void Surge* can be dodged and blocked only as the first enemy hit.

---

### W: Above and Below

| Attribute | Value |
|-----------|------:|
| **Range** | 0 - er 660 units |
| **Cast Time** | $0.5$ seconds |
| **Width** | 200 units |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Queue Time** | $0.35$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**ACTIVE:** **Bel'Veth** slams her tail down in the target direction that deals magic damage to enemies hit, knocks them up for $0.75$ seconds, and slows them by 50% for a duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 110 / 150 / 190 / 230 (+ 100% **bonus** AD) (+ 125% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow Duration** | 1.25 / 1.5 / 1.75 / 2 / 2.25 seconds |

If this hits an enemy champion, it resets *Void Surge’s* dash cooldown of the target direction.

**Notes:**

- This ability will cast from wherever the caster is at the start of the cast time.
- *Above and Below* can reset the cooldown of each dash from Void Surge.
  - More than one cooldown can be reset if enemy champions were hit within multiple directions.
  - The cooldowns that get reset are based on the relative direction to **Bel'Veth** at the end of the cast
  - If Void Surge's dash comes off cooldown during *Above and Below*’s cast time, **Bel'Veth** will be able to dash twice in the same direction.

---

### E: Royal Maelstrom

| Attribute | Value |
|-----------|------:|
| **Cast Time** | None |
| **Effect Radius** | 500 (Slash radius) units |
| **Cooldown** | 20 / 19 / 18 / 17 / 16 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.35$ seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | Special |
| **Parry** | True |
| **Call For Help** | True |

**ACTIVE:** **Bel'Veth** enters a defensive stance for $1.5$ seconds, during which she is unable to move, but gains damage reduction and (life steal) 20% (+ 100% life steal (Does not apply to the life steal gained from Royal Maelstrom)) life steal.

| Attribute | Value |
|-----------|------:|
| **Damage Reduction** | 35 / 40 / 45 / 50 / 55% |

While active, she rapidly slashes at the nearest enemy with the lowest **current** health percentage for up to 6 (+ 1 per |Benefits from all sources of bonus attack speed) times over the duration. Each slash deals physical damage, increased by 0%–300%@0–100 (@=target's **missing** health), triggers on-attack effects, and applies on-hit effects and spell effects, with on-hit and spell effect damage reduced to 8%–32%@0–100 (@=target's **missing** health) effectiveness.

| Attribute | Value |
|-----------|------:|
| **Minimum Physical Damage per hit** | 6 / 7 / 8 / 9 / 10 (+ 8% AD) |
| **Maximum Physical Damage per hit** | 24 / 28 / 32 / 36 / 40 (+ 32% AD) |

*Royal Maelstrom* deals 150% damage to monsters, applies life steal at 100% effectiveness, and is affected by critical strike modifiers. **Bel'Veth** cannot perform slashes while unable to declare basic attacks.

| Attribute | Value |
|-----------|------:|
| **Minimum Monster Damage per hit** | 9 / 10.5 / 12 / 13.5 / 15 (+ 12% AD) |
| **Maximum Monster Damage per hit** | 36 / 42 / 48 / 54 / 60 (+ 48% AD) |

*Royal Maelstrom* can be recast after $0.75$ seconds within the duration, and does so automatically after the duration or when **Bel'Veth** casts an ability.

**RECAST:** **Bel'Veth** ends *Royal Maelstrom*.

*The target does not have to be visible to be hit by this ability.*

**Notes:**

- Applies spell damage and procs basic damage required effects.
- *Royal Maelstrom* does not prioritize champions over other units.
- **Bel'Veth** cannot attack structures, wards, nor jungle plants with *Royal Maelstrom*.
- If **Bel'Veth** is berserked or taunted, she will slash at the unit she is forced to attack.
- *Royal Maelstrom* will still perform slashes even if **Bel'Veth** is feared during the channel.
- Each slash's damage triggers a stack of Conqueror.
- Despite this ability's cooldown starts on cast, it cannot be reduced by Navori Flickerblade Transcendence during the effect of this ability. Cooldowns of the other basic abilities can be normally reduced during this ability, though.
- The following table refers for interactions while **Bel'Veth** is performing *Royal Maelstrom*:

---

### R: Endless Banquet

| Attribute | Value |
|-----------|------:|
| **Range** | 450 (Void Coral targeting radius) units |
| **Cast Time** | 1 seconds |
| **Effect Radius** | 500 (Slow and explosion radius) units |
| **Cooldown** | 1 seconds |
| **Queue Time** | $0.35$ seconds |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Damage Type** | True |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Grounded** | True |
| **Knockdown** | Special |

**PASSIVE:** **Bel'Veth**’s basic attacks on-hit apply a mark to the target for 5 seconds, refreshing on subsequent hits. Every second attack on-hit against the marked target deals **bonus** true damage and generates a stack of *Endless Banquet* that increases this damage by the same value. This effect stacks infinitely, but is capped at 5 stacks against epic monsters. The mark and stacks expire upon attacking a new target.

| Attribute | Value |
|-----------|------:|
| **Bonus True Damage** | 6 / 7 / 8 / 9 / 10 (+ 12% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Maximum Monster Damage** | 30 / 35 / 40 / 45 / 50 (+ 60% **bonus** AD) |

**PASSIVE:** When **Bel'Veth** scores a takedown against an enemy champion or epic monster while alive, a *Void Coral* is spawned from their corpse for 15 seconds. Baron Nashor, the pit Rift Herald, and Voidgrub (once per spawn group) spawn an *Enhanced Void Coral*.

**ACTIVE:** **Bel'Veth** dashes to the target *Void Coral* to consume it over the cast time, slowing nearby enemies by 25% / 42% / 69% / 96% for the duration. She then creates an explosion at the location to deal true damage to enemies within, capped at 1500 versus monsters, and assumes her **TRUE FORM** for 60 seconds.

**Bel'Veth** consumes all existing *Void Corals* at once, generating a stack of *Lavender* for each one consumed.

| Attribute | Value |
|-----------|------:|
| **True Damage** | 150 / 175 / 200 / 225 / 250 (+ 100% AP) (+ 25% of target's **missing** health) |

**TRUE FORM:** **Bel'Veth** evolves into a monster, gaining **bonus** health, **bonus** movement speed out-of-combat, 75 **bonus** attack range, as well as increased **total** attack speed. *Void Surge* can dash through terrain. Consuming a *Void Coral* refreshes (See notes) the duration of **TRUE FORM** and heals **Bel'Veth**.


**ENHANCED VOID CORAL BONUS:** **TRUE FORM** is empowered to last 180 seconds and causes *Void Remora* to spawn from allied and enemy minions that die nearby.

| Attribute | Value |
|-----------|------:|
| **Bonus Health** | 100 / 125 / 150 / 175 / 200 (+ 120% **bonus** AD) (+ 90% AP) |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 10 / 27.5 / 45 / 62.5 / 80 |

| Attribute | Value |
|-----------|------:|
| **Increased Total Attack Speed** | 10 / 12.5 / 15 / 17.5 / 20% |

| Attribute | Value |
|-----------|------:|
| **Heal** | 100 / 125 / 150 / 175 / 200 (+ 120% **bonus** AD) (+ 90% AP) |

*A nearby Void Coral is required to cast this ability. The on-hit modifiers from Death in Lavender, Void Surge and Royal Maelstrom affect the **bonus** true damage applied by Endless Banquet's passive. See [Pets](#Pets) for details about Void Remora and Void Corals.*

**Notes:**

- Applies proc damage on the bonus true damage and deals area damage on the explosion.
- *Endless Banquet*’s takedown effect will not trigger against monsters if they were slain by an enemy.
- *Endless Banquet*’s true damage based on the target's missing health is calculated *before* the base damage is dealt.
- *True Form*’s duration is displayed in **Bel'Veth**’s resource bar, in seconds.
- *True Form*’s duration is refreshed based on the type of *Void Coral* that **Bel'Veth** consumes.
  - Consuming a normal *Void Coral* in a normal *True Form* state refreshes the duration to 60 seconds.
  - Consuming an enhanced *Void Coral* refreshes the duration to 180 seconds and grants the enhanced effects regardless of current *True Form* state.
  - If she consumes a normal *Void Coral* while in an enhanced *True Form* state, the current duration is extended by 60 seconds.
    - The enhanced effects are **not** lost in this case.
- **Bel'Veth** will consume all existing *Void Corals* at once, therefore she is able to acquire the enhanced effects of *True Form* even if she does not physically consume an enhanced *Void Coral*.
- The *Void Coral* that **Bel'Veth** consumes herself will not expire in the process of her doing so.
- **Bel'Veth** will consume all other *Void Corals* upon consumption regardless of range.
- Ranking up *Endless Banquet* will update the bonuses dynamically; if **Bel'Veth** has *True Form* upon the level-up, she will receive the upgraded stats.
- **Bel'Veth** gains the **bonus** health and heal from *Endless Banquet* at the start of the cast time.
- **Bel'Veth** will be able to consume the target *Void Coral* even if her dash is interrupted.
  - She dashes at the start of the cast time.
- A *Void Coral* will **not** spawn when scoring a takedown against a summoned Rift Herald. - This ability will cast from wherever the caster is at the start of the cast time.
- The following table refers for interactions while **Bel'Veth** is in cast time:
  - Movement summoner spells can be buffered to cast after the cast time completes if permitted.
  - She is locked out of movement and attack actions for $0.5$ seconds after the cast time completes and no buffering of them is permitted for the entirety of her lockout.

---

## Patch History

### V25.17
- Royal Maelstrom
  - **Bug Fixes:** Navori Flickerblade Transcendence now properly applies to Void Surge’s per-direction cooldown when triggering it via Royal Maelstrom. Previously, the visual indicator for direction availability still lit up as if the effect was applied and the direction was available, even though it could not actually be cast, due to the bug.

### V25.15
- Death in Lavender
  - Bonus attack speed per stack increased to 0.28 to 1.1 from 0.28+0.045*(x-1) for 17. *Now also scales until level 18 instead of 17.*
- Void Surge
  - Base damage reduced to 0 / 5 / 10 / 15 / 20 from 10 / 15 / 20 / 25 / 30.
  - Bonus monster damage increased to 55 / 65 / 75 / 85 / 95 from 45 / 55 / 65 / 75 / 85.

### V25.12
- Endless Banquet
  - Cast range increased to 450 units from 275.
  - Cast no longer seals non-movement summoner spells.
    - Movement summoner spells will buffer to cast at the end of the cast time if permitted.

### V14.18
- General
  - **Bug Fixes:** Certain voiceover lines will now properly trigger to play.

### V14.14
- Stats
  - Attack range reduced to 150 units from 175.
- Endless Banquet
  - Bonus attack range increased to 75 units from 50.

### V14.9
- Death in Lavender
  - Bonus attack speed per *Lavender* stack reduced to 0.28% / 0.33% / 0.37% / 0.42% / 0.46% / 0.51% / 0.55% / 0.59% / 0.64% / 0.69% / 0.73% / 0.78% / 0.82% / 0.86% / 0.91% / 0.95% / 1% from 0.28% / 0.34% / 0.4% / 0.46% / 0.52% / 0.58% / 0.64% / 0.7% / 0.76% / 0.82% / 0.88% / 0.94% / 1%. *Now scales until level 17 instead of 13.*
- Void Surge
  - AD ratio reduced to 100% AD from 110% AD.
  - **Removed:*** No longer deals 140% damage to monsters.
  - **New Effect:** Now deals 45 / 55 / 65 / 75 / 85 **bonus** damage to monsters.

### V14.5
- Death in Lavender
  - **Bug Fixes:** Per-Voidgrub and Rift Herald takedowns now properly grant 2 Lavender stacks instead of 3.
- Royal Maelstrom
  - Damage reduction reduced to 35 / 40 / 45 / 50 / 55% from 42 / 49 / 56 / 63 / 70%.

### V13.21
- Royal Maelstrom
  - Cooldown reduced to 20 / 19 / 18 / 17 / 16 seconds from 24 / 22.5 / 21 / 19.5 / 18.

### V13.20
- Death in Lavender
  - Bonus attack speed from ability casts reduced to 20% / 40% from 25 to 50.
- Royal Maelstrom
  - Damage reduction reduced to 42 / 49 / 56 / 63 / 70% from 70% at all ranks.
  - Minimum base damage per hit reduced to 6 / 7 / 8 / 9 / 10 from 8 / 10 / 12 / 14 / 16.
    - Maximum base damage per hit reduced to 24 / 28 / 32 / 36 / 40 from 32 / 40 / 48 / 56 / 64.
  - Minimum AD ratio per hit increased to 8% AD from 6% AD.
    - Maximum AD ratio per hit increased to 32% AD from 24% AD.
  - On-hit, on-attack, and spell modifier increased to 8%–32%@0–100 (@=target's **missing** health) from 6%–24%@0–100 (@=target's **missing** health).
  - **New Effect:** Damage can now critically strike.
- Endless Banquet
  - Bonus movement speed changed to 10 / 45 / 80 from 25 / 50 / 75.
  - Remora health changed to 20 / 45 / 70% of minion's **maximum** health from 40 / 50 / 60%.

### V13.15#August 3rd Hotfix|V13.15
- Void Surge
  - **Bug Fixes:** Directional indicators restored.

## Trivia

- 
  - In Bel'Veth's case, I infinitely stacks the bonus attack speed gained from *Lavender* stacks and R infinitely stacks the bonus true damage dealt by the passive.
- If **Bel'Veth** has an attack speed of **$1.8$** or higher, her attack animations will change to thrust at the target instead of swiping at them.

---
*This page was automatically generated from League of Legends Wiki data.*