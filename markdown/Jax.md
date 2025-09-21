# Jax

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
| **Champion** | Jax |
| **Title** | Grandmaster at Arms |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Roles** | Skirmisher |
| **Riot Positions** | Top, Jungle |
| **External Positions** | Top, Jungle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $665.0$ | $+103.0$ | $2416.0$ |
| **Mana** | $339.0$ | $+52.0$ | $1223.0$ |
| **Health Regen** | $8.5$ | $+0.55$ | $17.9$ |
| **Mana Regen** | $8.2$ | $+0.7$ | $20.1$ |
| **Armor** | $36.0$ | $+4.2$ | $107.4$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $68.0$ | $+4.25$ | $140.2$ |
| **Attack Speed** | $0.638$ | $+3.4\%$ | $1.007$ |
| **Movement Speed** | $350.0$ | $+0.0$ | $350.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.638$ |
| **Attack Speed Ratio** | $0.638$ |
| **Bonus AS per Level** | $3.4\%$ |
| **Acquisition Radius** | $400 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $130 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Relentless Assault

**Innate:** **Jax**’s basic attacks generate stacks, up to a cap. Stacks expire one at a time.

**Innate:** ''Jax's* basic attacks generate a stack of *Relentless Assault' on-attack for $2.5$ seconds, refreshing on subsequent attacks and stacking up to 8 times. Stacks expire by one every $0.25$ seconds when the duration ends. **Relentless Assault:** For each stack, **Jax** gains *key=% *bonus attack speed*, up to a maximum of key=%. **Grandmaster at Angling:** While out-of-combat with champions and idle in the river for 10 seconds, **Jax** will occasionally catch a fish, granting him 1 gold and 1 ability power for 5 seconds. He catches a fish at an average rate of one every 15 seconds. He also has a 5% chance to catch a rare fish that grants 10 gold and 10 ability power for 5 seconds.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies, Self |

**Notes:**

- **Jax** will start fishing while idle in the river if he is in-combat with only monsters (e.g. any of the three Epic monsters).
- **Jax** catching a fish is a random event and will play a special animation.
- The total number of fish caught during the game is displayed when fishing as *Grandmaster at Angling* passive.

---

### Q: Leap Strike

**Active:** **Jax** dash to the target unit. If they are an enemy, he deals physical damage.

**Active:** **Jax** dashes to the target unit's location. If the target is an enemy and they are in range upon arrival, **Jax** deals physical damage to them. **Jax can cast any of his abilities during the dash.**

| Attribute | Value |
|-----------|-------|
| **Range** | 700 units |
| **Cooldown** | $8-6$ seconds |
| **Cast Time** | false |
| **Cost** | 65 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | single |

**Scaling:**
- **Physical Damage:** $65-225$ bonus AD)

**Notes:**

- *Leap Strike* cannot be cast on structures.
- If the target is an enemy champion, **Jax** will be ordered to basic attack them after the dash ends.
- Spell shield will block the damage, including when *empowered*.

---

### W: Empower

**Active:** **Jax** empowers his next basic attack or **Leap Strike** against an enemy to deal **bonus** magic damage.

**Active:** **Jax** empowers his next basic attack or **Leap Strike** against an enemy within 10 seconds to deal **bonus** magic damage, reduced to 50% against structures. If *Empower* is used on a basic attack, it will gain range*bonus** range* and have an uncancellable windup. *Empower basic attack reset *'Jax's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $7-3$ seconds |
| **Cast Time** | none |
| **Cost** | 30 Mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **Bonus Magic Damage:** $50-190$ (+ 60% AP)

**Notes:**

- *Empower* is applied in a separate damage instance from ''Jax's' basic attacks.
  - This causes effects like *Bone Plating* to be applied twice.

---

### E: Counter Strike

**Active:** **Jax** enters *Evasion* for a short time: a defensive stance that causes him to dodge all incoming non-turret basic attacks and take reduced damage from all area of effect abilities from champions.

*Counter Strike* can be recast within this time, and does so automatically after the duration.

**Active:** **Jax** enters *Evasion* for 2 seconds: a defensive stance that causes him to dodge all incoming non-turret basic attacks and take 25% reduced damage from all area of effect abilities sourced from champions. *Counter Strike* can be recast after 1 second, and does so automatically after the duration. **Recast:** **Jax** deals magic damage to nearby enemies, with the **total** damage increased by 20% for each attack dodged, up to a 100% increase, and stun them for 1 second.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $17-9$ seconds |
| **Cast Time** | None |
| **Cost** | $50-90$ Mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Effect Radius** | 375 units |
| **Spell Shield** | True |
| **Spell Effects** | aoe |

**Scaling:**
- **Minimum Magic Damage:** $40-160$ (+ 70% AP) (+ $3.5$% of target's **maximum* health)2-160×2$ (+ $70×2$% AP) (+ $3.5×2$% of target's
- **maximum** health)

**Notes:**

- The initial cast and the manual recast count as ability activations for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- *Counter Strike* will also dodge abilities that can trigger on-hit effects (*Parrrley*, *Mystic Shot*) which will count towards 'Counter Strike's* increased damage. There are exceptions of abilities that *Counter Strike' will not dodge but will dodge the damage from on-hit effects that they trigger (*Alpha Strike*, *Piercing Darkness*).

---

### R: Grandmaster-at-Arms

**Passive:** **Jax**’s basic attacks generate stacks. At max stacks, he consumes them to deal **bonus** magic damage.

**Active:** **Jax** swings his lantern around, dealing magic damage to nearby enemies. If this hits a champion, he gains **bonus** resistances. He applies his passive on-hit every 2 stacks instead of every 3 while active.

**Passive:** ''Jax's* basic attacks generate a stack of *Grandmaster-at-Arms* on-hit for $2.5$ seconds, refreshing on subsequent hits and stacking up to 2 times. At 2 stacks, his next basic attack on-hit is empowered to have an uncancellable windup and consume all stacks to deal **bonus** magic damage, reduced to 50% against structures. While *Grandmaster-at-Arms' is active, the empowered attack triggers at 1 stack instead. **Active:** **Jax** swings his lantern around, dealing magic damage to nearby enemies. If this hits a champion, he gains **bonus armor**, increased for each champion hit beyond the first, and **bonus magic resistance** equal to 60% of that amount as well as 10% increased for 8 seconds. '**Jax** can move during Grandmaster-at-Arms' cast time.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $110-90$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 mana |
| **Targeting** | Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Effect Radius** | 375 units |
| **Spell Shield** | Special |
| **Spell Effects** | Special |

**Scaling:**
- **Bonus Magic Damage:** $75-185$ (+ 60% AP)
- **Magic Damage:** $100-250$ (+ 100% AP)
- **Bonus Armor:** $25-75$ (+ 40% bonus AD).6-75*.6$ (+ 24% bonus AD)
- **Bonus Armor per Champion Hit:** $20-30$ (+ 10% bonus AD).6-30*.6$ (+ 6% bonus AD)

**Notes:**

- The attack bonus deals proc damage. The swing deals area damage.
- The bonus resistances are represented by a glowing aura around **Jax** that grows with each enemy champion hit beyond the first.
- The **bonus** magic damage can be dodge and block but it cannot blind.
- Spell shield will only block the bonus damage from the third attack if it is *empowered*.
- The empowered attack will trigger but not be consumed nor apply its effects against wards.
  - Stacks will still be generated when attacking them.
- Effect at cast time end

---

## Patch History

### V25.12
- *Relentless Assault*
  - Bonus attack speed per stack increased to key=% from key=%.
    - Maximum bonus attack speed increased to key=% from key=%.

### V25.11
- *Grandmaster-at-Arms*
  - **Undocumented / New Effect:** Empowered basic attack now has an uncancellable windup.

### V25.08
- *Empower*
  - **Bug Fixes:** No longer sometimes fails to apply its damage against structures.

### V25.07
- *Grandmaster-at-Arms*
  - **Bug Fixes:** After spending a second or further skill point in the ability, no longer causes the attack animations associated with the ability to stop playing.
- Stats
  - Health growth increased to 103 from 100.
  - Base mana regeneration increased to $8.2$ from $7.6$.
- *Grandmaster-at-Arms*
  - On-hit base damage increased to $75-185 3$ from $70-170 3$.

### V14.22
- *Grandmaster-at-Arms*
  - Cooldown increased to $110-90 3$ seconds from $100-80 3$.
  - Base damage reduced to $100-250 3$ from $150-350 3$.

### V14.18
- *Grandmaster-at-Arms*
  - On-hit base damage increased to $70-170 3$ from $60-160 3$.
  - Bonus armor increased to $25-75 3$ from $15-65 3$.
    - Bonus magic resistance increased to $25×0.6-75×0.6 3$ from $15×0.6-65×0.6 3$.
  - Bonus armor per subsequent target increased to $20-30 3$ from $15-25 3$.
    - Bonus magic resistance per subsequent target increased to $20×0.6-30×0.6 3$ from $15×0.6-25×0.6 3$.

### V14.14
- *Empower*
  - **Bug Fixes:** Empowered attack no longer sometimes fails to apply against towers.
- *Counter Strike*
  - Cooldown increased to $17-9$ seconds from $15-9$.
- *Grandmaster-at-Arms*
  - **Bug Fixes:** Restored empowered attack SFX when attacking structures.

### V14.10
- *Grandmaster-at-Arms*
  - **Bug Fixes:** Cast indicator now matches with the correct range of 375.
  - **Bug Fixes:** Ability power and attack damage from conversions and adaptive force is now properly included in the ratio calculations.

### V14.9
- General
  - Adjusted splash artwork for Jax.

### V14.7
- *Relentless Assault*
  - **Bug Fixes:** Restored all respective buff icons.

## Trivia

- Jax's eyes are blue.
- Jax was likely inspired by , "The Weapons Master" of the *Sword of Shannara Trilogy*. He was adept with any weapon and was undefeated in combat, though he often wielded a sword and cudgel.
- In the V1.0.0.115 (April Fools' Day) patch, *Wriggle's Lantern* was given the following joke passive, referencing **Jax**’s usage of a lamp post (which is functionally similar to a lantern) as a weapon:
  - New **Unique Passive**: Taunts nearby **Jax** (both enemy and allied).
- In the now-removed official League of Legends forums, the original icon of *image=Jax Relentless Assault old.png* was used to represent the "Off Topic Discussion" section.
- The old icon art of *Grandmaster's Might* features a stunned soldier who was directly mirrored from the original icon for *image=Time Bomb old.png*.
- Grandmaster-at-Arms, The Darkin Blade, Night Hunter, and The Relentless Storm are the only abilities in *League of Legends* that have the same name as their champion's title.
- Jax's Series 2 Eternals make the following references:
  - * Active: Jax enters Evasion...* is a reference to his ability description for *Counter Strike*.

---
*This page was automatically generated from League of Legends Wiki data.*