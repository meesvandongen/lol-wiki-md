# Bel'Veth

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Bel'Veth |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $550$ | $+85$ | $1995.0$ |
| **Mana** | $350$ | $+50$ | $1200.0$ |
| **Armor** | $22$ | $+3.5$ | $81.5$ |
| **Magic Resist** | $30$ | $+0.5$ | $38.5$ |
| **Attack Damage** | $56$ | $+3.1$ | $108.7$ |
| **Attack Speed** | $0.625$ | $+3.2\%$ | $0.965$ |

## Abilities

### Passive: Death in Lavender

**Innate:* cisxp leveling up*. The damage dealt by her basic attacks (including on-hit effects) is reduced.

**Innate:** ''Bel'Veth's'* ability casts generate stacks of *Death in Lavender*, which lasts a few seconds and stacks up to a cap. While *'Bel'Veth'' has stacks, she becomes ghosted and gains . Her basic attacks consume stacks.

**Innate:** ''Bel'Veth'' attacks 36% faster than the median champion and her attack speed cap is modified to $9999.0$, but all sources of damage (including on-hit damage) dealt by her basic attacks are reduced to 75%. Her attack speed does not increase through [growth (per level)](./Champion_statistic.md#Increasing_Statistics). **Innate:** ''Bel'Veth's'* ability casts each generate 2 stacks of *Death in Lavender*, lasting for 5 seconds, refreshing on basic attacks and subsequent casts, and stacking up to 6 times. While *'Bel'Veth** has stacks, she becomes ghosted and gains *$20+(20/17)*(x-1)*(0.7025+0.0175*(x-1))$ **bonus'' attack speed*. Her basic attacks each consume 1 stack. **Innate:** Whenever ''Bel'Veth'* scores a takedown against a champion, monster, or large minion within 3 seconds of damaging them, she generates a permanent stack of *Lavender*. Large minions and monsters generate 1 stack, champions and epic monsters generate 2 stacks. **Lavender:** For each stack, *'Bel'Veth** gains $0.28 to 1.1$ **bonus'' attack speed.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | range = |
| **Cast Time** | targeting = Passive |
| **Cost** | costtype = cooldown = |

**Notes:**

- 'Death in Lavender's' takedown effect will not trigger against monsters if they were slain by an enemy.
- *Death in Lavender* stacks are indicated under ''Bel'Veth's'' [health bar](./health_bar.md).
- ''Bel'Veth's'' basic attack damage modifier will reduce the damage dealt by on-hit effects even if they aren't applied on-hit.

---

### Q: Void Surge

**Active:** **Bel'Veth** dashes in the target direction, though not through [terrain](./terrain.md), dealing physical damage to enemies she passes through.

*Void Surge* can be cast only within a cardinal direction that is off cooldown, each direction has a unique *cooldown*.

**Active:** ''Bel'Veth'* dashes in the designated direction, though not through terrain, dealing physical damage to enemies she passes through. Against the first target, *Void Surge' can critically strike for critical damage and applies on-hit effects, with on-hit damage reduced to 75% effectiveness, and at 100% effectiveness. *Void Surge* deals **bonus** physical damage to monsters and modified damage against minions. *Void Surge* can be cast only within a cardinal direction that is off cooldown, and incurs a cooldown between casts. Each cardinal direction has a unique *cooldown* that is reduced equivalent to $0.25$ per Benefits from all sources of bonus attack speed except the temporary bonus from Death in Lavender. These cooldowns reset upon [respawning](./death.md). *Void Surge basic attack reset *'Bel'Veth's'* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 1 seconds |
| **Cast Time** | None |
| **Cost** | costtype = cooldown = 1 |

**Scaling:**
- **Physical Damage:** $0-20$ (+ 100% AD)
- **Monster Bonus Physical Damage:* $55-95$*Modified Minion Damage:** $60-100$%

**Notes:**

- Applies area damage and procs basic damage required effects.
- The icon partially darkens based on the combination of dashes available. When all dashes are expended, the cooldown indicates the next available dash.
- While in *Endless Banquet*, 'Void Surge's' dash distance is extended up to 225 units when targeted across terrain.
- The basic attack reset is not considered one for *Hail of Blades*.
- *Void Surge* can be dodge and block only as the first enemy hit.

---

### W: Above and Below

**Active:** **Bel'Veth** slams her tail in the target direction that deals magic damage to enemies hit, briefly airborne and slow them.

*If this hits an enemy champion, it cdr '' dash cooldown of the target direction.*

**Active:** ''Bel'Veth'' slams her tail down in the target direction that deals magic damage to enemies hit, airborne for $0.75$ seconds, and slow them by 50% for a duration. If this hits an enemy champion, it resets '' dash *cooldown* of the target direction.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $12-8$ seconds |
| **Cast Time** | $0.5$ seconds |
| **Cost** | costtype = cooldown = $12-8$ |

**Scaling:**
- **Magic Damage:** $70-230$ (+ 100%
- *bonus AD) (+ 125% AP)
- **Slow Duration:** $1.25-2.25$ seconds

**Notes:**

- *Above and Below* can reset the cooldown of each dash from *Void Surge*.
  - More than one cooldown can be reset if enemy champions were hit within multiple directions.
  - The cooldowns that get reset are based on the relative direction to ''Bel'Veth'' at the end of the cast
  - If *Void Surge*'s dash comes off cooldown during 'Above and Below's* cast time, *'Bel'Veth'' will be able to dash twice in the same direction.

---

### E: Royal Maelstrom

**Active:** **Bel'Veth** enters a brief defensive stance, gaining *life steal* and damage reduction. During this time, ''Bel'Veth** rapidly slashes at the most wounded enemy, with the max number of slashes increased based on her **bonus'' attack speed. Each slash deals physical damage, and applies lifesteal , on-hit and spell effects based on the target's **missing** health.

*Royal Maelstrom* can be recast within the duration, and does so automatically afterwards or when ''Bel'Veth'' casts an ability.

**Active:** ''Bel'Veth'' enters a defensive stance for $1.5$ seconds, during which she is unable to move, but gains damage reduction and . While active, she rapidly slashes at the nearest enemy with the lowest **current** health percentage for up to 6 (+ 1 per Benefits from all sources of bonus attack speed) times over the duration. Each slash deals physical damage, increased by $key1=%$%, triggers on-attack effects, and applies on-hit effects and spell effects, with on-hit and spell effect damage reduced to $key1=%$% effectiveness. *Royal Maelstrom* deals 150% damage to monsters, applies lifesteal at 100% effectiveness, and is affected by critical strike modifiers. ''Bel'Veth'' cannot perform slashes while unable to declare basic attacks. *Royal Maelstrom* can be recast after $0.75$ seconds within the duration, and does so automatically after the duration or when ''Bel'Veth'' casts an ability. **Recast:** ''Bel'Veth'* ends *Royal Maelstrom'.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $20-16$ seconds |
| **Cast Time** | None |
| **Cost** | costtype = cooldown = $20-16$ |

**Scaling:**
- **Damage Reduction:** $35-55$%
- **Minimum Physical Damage per hit:* $6-10$ (+ 8% AD)4-10*4$ (+ $8*4$% AD)
- **Minimum Monster Damage per hit:** $6*1.5-10*1.5$ (+ $81.5$% AD)4*1.5-10*4*1.5$ (+ $8*4*1.5$% AD)

**Notes:**

- Applies spell damage and procs basic damage required effects.
- *Royal Maelstrom* does not prioritize champions over other units.
- ''Bel'Veth'* cannot attack structures, wards, nor [jungle plants](./jungle_plants.md) with *Royal Maelstrom'.
- If ''Bel'Veth'' is berserk or taunt, she will slash at the unit she is forced to attack.
- *Royal Maelstrom* will still perform slashes even if ''Bel'Veth'' is flee during the channel.
- Each slash's damage triggers a stack of *Conqueror*.
- Despite this ability's cooldown starts on cast, it cannot be reduced by during the effect of this ability. Cooldowns of the other basic abilities can be normally reduced during this ability, though.
- The following table refers for interactions while ''Bel'Veth'* is performing *Royal Maelstrom':

---

### R: Endless Banquet

**Passive:** Every second basic attack on the same target deals **bonus** true damage and generates a stack that increases the damage. This can stack infinitely, and expires after a short time or upon switching targets.

**Passive:** Takedowns against enemy champions and epic monsters spawn a *Void Coral for some time. Taking down uiEnhance Void Coral*.

**Passive:** ''Bel'Veth's** basic attacks on-hit apply a mark to the target for 5 seconds, refreshing on subsequent hits. Every second attack on-hit against the marked target deals ccs **bonus' true damageEndless Banquet' that increases this damage by the same value. This effect stacks infinitely, but is capped at 5 stacks against epic monsters. The mark and stacks expire upon attacking a new target. **Passive:** When ''Bel'Veth'* scores a takedown against an enemy champion or epic monster while alive, a *Void Coral is spawned from their corpse for 15 seconds. uiEnhanced Void Coral'. **Active:** ''Bel'Veth'* dashes to the target *Void Coral* to consume it over the cast time, slow nearby enemies by $type=seconds elapsed$ for the duration. She then creates an explosion at the location to deal *true damage* to enemies within, capped at 1500 versus monsters, and assumes her **True Form** for 60 seconds. *'Bel'Veth'* consumes all existing *Void Corals* at once, generating a stack of **Death in Lavender*' for each one consumed. **True Form:** ''Bel'Veth** evolves into a monster, gaining **bonus health**, **bonus movement speed** [out-of-combat](./combat_status.md), *75 **bonus** attack range*, as well as increased **total'* attack speed. **Void Surge** can dash through [terrain](./terrain.md). Consuming a *Void Coral* refreshes the duration of **True Form** and heal *'Bel'Veth'*. **Enhanced Void Coral Bonus:** **True Form** is empowered to last 180 seconds and causes *Void Remora' to spawn from allied and enemy minions that die nearby. 'A nearby Void Coral is required to cast this ability. The on-hit modifiers from *Death in Lavender*, *Void Surge* and *Royal Maelstrom* affect the **bonus** true damage applied by Endless Banquet's passive. See [Pets](./Bel'Veth.md#Pets) for details about Void Remora and Void Corals.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 1 seconds |
| **Cast Time** | 1 seconds |
| **Cost** | costtype = cooldown = 1 |

**Scaling:**
- **Bonus True Damage:** $6-10$ (+ 12%
- *bonus AD)
- **Maximum Monster Damage:** $6*5-10*5$ (+ $12*5$%
- *bonus AD)
- **True Damage:** $150-250$ (+ 100% AP) (+ 25% of target's
- **missing** health)

**Notes:**

- Applies proc damage on the bonus true damage and deals area damage on the explosion.
- 'Endless Banquet's' takedown effect will not trigger against monsters if they were slain by an enemy.
- 'Endless Banquet's' true damage based on the target's missing health is calculated *before* the base damage is dealt.
- 'True Form's* duration is displayed in *'Bel'Veth's'' resource bar, in seconds.
- 'True Form's* duration is refreshed based on the type of *Void Coral* that *'Bel'Veth'' consumes.
  - Consuming a normal *Void Coral* in a normal *True Form* state refreshes the duration to 60 seconds.
  - Consuming an enhanced *Void Coral* refreshes the duration to 180 seconds and grants the enhanced effects regardless of current *True Form* state.
  - If she consumes a normal *Void Coral* while in an enhanced *True Form* state, the current duration is extended by 60 seconds. ** The enhanced effects are not*** lost in this case.
- ''Bel'Veth'* will consume all existing *Void Corals* at once, therefore she is able to acquire the enhanced effects of *True Form* even if she does not physically consume an enhanced *Void Coral'.
- The *Void Coral* that ''Bel'Veth'' consumes herself will not expire in the process of her doing so.
- ''Bel'Veth'* will consume all other *Void Corals' upon consumption regardless of range.
- Ranking up *Endless Banquet* will update the bonuses dynamically; if ''Bel'Veth'* has *True Form' upon the level-up, she will receive the upgraded stats.
- ''Bel'Veth** gains the **bonus'* health and heal from *Endless Banquet' at the start of the cast time.
- ''Bel'Veth'* will be able to consume the target *Void Coral' even if her dash is interrupted.
  - She dashes at the start of the cast time.
- A *Void Coral* will **not** spawn when scoring a takedown against a summoned .
- The following table refers for interactions while ''Bel'Veth'' is in cast time:
  - Movement summoner spells can be buffered to cast after the cast time completes if permitted.
  - She is locked out of movement and attack actions for $0.5$ seconds after the cast time completes and no buffering of them is permitted for the entirety of her lockout.

---

## Patch History

### V25.17
- *Royal Maelstrom*
  - ***Bug Fixes:*** iisRoyal Maelstrom*. Previously, the visual indicator for direction availability still lit up as if the effect was applied and the direction was available, even though it could not actually be cast, due to the bug.

### V25.15
- *Death in Lavender*
  - Bonus attack speed per stack increased to $0.28 to 1.1$ from $0.28+0.045*(x-1) for 17$. *Now also scales until level 18 instead of 17.*
- *Void Surge*
  - Base damage reduced to $0-20$ from $10-30$.
  - Bonus monster damage increased to $55-95$ from $45-85$.

### V25.12
- *Endless Banquet*
  - Cast range increased to 450 units from 275.
  - Cast no longer seals non-movement summoner spells.
    - Movement summoner spells will buffer to cast at the end of the cast time if permitted.

### V14.18
- General
  - ***Bug Fixes:*** Certain voiceover lines will now properly trigger to play.

### V14.14
- Stats
  - Attack range reduced to 150 units from 175.
- *Endless Banquet*
  - Bonus attack range increased to 75 units from 50.

### V14.9
- *Death in Lavender*
  - Bonus attack speed per *Lavender* stack reduced to $0.28 to 1 for 17$ from $0.28 to 1 for 13$. *Now scales until level 17 instead of 13.*
- *Void Surge*
  - AD ratio reduced to 100% AD from 110% AD.
  - ***Removed:**** No longer deals 140% damage to monsters.
  - ***New Effect:*** Now deals $45-85$ **bonus** damage to monsters.

### V14.5
- *Death in Lavender*
  - ***Bug Fixes:*** Per- and takedowns now properly grant 2 Lavender stacks instead of 3.
- *Royal Maelstrom*
  - Damage reduction reduced to $35-55$% from $42-70$%.

### V13.21
- *Royal Maelstrom*
  - Cooldown reduced to $20-16$ seconds from $24-18$.

### V13.20
- *Death in Lavender*
  - Bonus attack speed from ability casts reduced to $20+(20/17)*(x-1)*(0.7025+0.0175*(x-1))$ from $25 to 50$.
- *Royal Maelstrom*
  - Damage reduction reduced to $42-70$% from 70% at all ranks.
  - Minimum base damage per hit reduced to $6-10$ from $8-16$.
    - Maximum base damage per hit reduced to $6*4-10*4$ from $8*4-16*4$.
  - Minimum AD ratio per hit increased to 8% AD from 6% AD.
    - Maximum AD ratio per hit increased to $8*4$% AD from $6*4$% AD.
  - On-hit, on-attack, and spell modifier increased to $key1=%$% from $key1=%$%.
  - ***New Effect:*** Damage can now critically strike.
- *Endless Banquet*
  - Bonus movement speed changed to $10-80 3$ from $25-75 3$.
  - Remora health changed to $20-70 3$% of minion's **maximum** health from $40-60 3$%.
- *Void Surge*
  - ***Bug Fixes:*** Directional indicators restored.

### V13.13
- *Endless Banquet*
  - ***Bug Fixes:*** No longer causes the passive mark to expire from casting *Void Surge* and *Royal Maelstrom* on different targets.

## Trivia

- 
  - In Bel'Veth's case, *I* infinitely stacks the bonus attack speed gained from *Lavender* stacks and *R* infinitely stacks the bonus true damage dealt by the passive.
- If ''Bel'Veth** has an attack speed of **$1.8$'' or higher, her attack animations will change to thrust at the target instead of swiping at them.

---
*This page was automatically generated from League of Legends Wiki data.*