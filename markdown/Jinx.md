# Jinx

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
| **Champion** | Jinx |
| **Title** | the Loose Cannon |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2013-10-10 |
| **Release Patch** | V3.12 |
| **Roles** | Marksman |
| **Riot Positions** | Bottom |
| **External Positions** | Bottom |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $630.0$ | $+105.0$ | $2415.0$ |
| **Mana** | $260.0$ | $+50.0$ | $1110.0$ |
| **Health Regen** | $3.75$ | $+0.5$ | $12.2$ |
| **Mana Regen** | $6.7$ | $+1.0$ | $23.7$ |
| **Armor** | $26.0$ | $+4.7$ | $105.9$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $59.0$ | $+0.6$ | $69.2$ |
| **Attack Speed** | $0.625$ | $+1.4\%$ | $0.774$ |
| **Movement Speed** | $325.0$ | $+0.0$ | $325.0$ |
| **Attack Range** | $525.0$ | $+0.0$ | $525.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.4\%$ |
| **Attack Windup** | $16.9\%$ |
| **Acquisition Radius** | $550 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $155 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Get Excited!

**Innate:** Whenever **Jinx** scores a takedown against an enemy champion, epic monster, turret, or inhibitor shortly after damaging them, she gains a burst of ms for a few seconds.

*During this time, she also gains *as **bonus total** attack speed* and is allowed to exceed the attack speed cap.*

**Innate:** Whenever **Jinx** scores a takedown against an enemy champion, epic monster, turret, or inhibitor within 3 seconds of damaging them, she gains ms*bonus** movement speed* decaying over 6 seconds. Additionally, she is allowed to exceed the attack speed cap and gains a stack of *Get Excited!* for the same duration, stacking up to 5 times. Only takedown against enemy champion can grant stacks beyond the first. **Get Excited!:** For each stack, **Jinx** gains *25% **total** attack speed*, up to a maximum of $25×5$%.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- ''Jinx's* *attack speed cap' is increased to $90.0$ for the duration, however this value is already beyond the technical limit for attack speed.
- **Jinx** will still trigger *Get Excited* from killing a summoned Rift Herald.
- **Jinx** will fail to trigger *Get Excited* in the specific circumstance of killing Baron Nashor while more than ~2500 distance away.

---

### Q: Switcheroo!

**Toggle:** **Jinx** switches between Pow-Pow, her minigun, and Fishbones, her rocket launcher.

**Pow-Pow.png Basic attacks with **Pow-Pow** generate stacks of 'Rev'd up' for a short time. Stacks expire one at a time.

**Toggle:** **Jinx** switches between *Pow-Pow.png*, her minigun, and *Fishbones.png*, her rocket launcher. **Pow-Pow:** Basic attacks with *Pow-Pow* generate a stack of 'Rev'd up* for $2.5$ seconds, refreshing on subsequent attacks with *Pow-Pow* and stacking up to 3 times. Each stack of *Rev'd up' grants **bonus attack speed**, with all stacks beyond the first one being 50% effective. Stacks expire by one when the duration ends. **Fishbones:** Basic attacks with *Fishbones* cost *mana* on-attack to deal 110% AD *modified physical damage to the target and nearby enemies. The damage is affected by critical strike modifiers. While *Fishbones* is equipped, **Jinx** gains *range **bonus** range* but loses *10% of her *bonus attack speed*. 'Only the first attack after switching to Fishbones will benefit from Rev'd up'.

| Attribute | Value |
|-----------|-------|
| **Cast Time** | none |
| **Cost** | 20 Mana per attack with *Fishbones* |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Effect Radius** | 250 units |
| **Spell Shield** | False |
| **Spell Effects** | Special |

**Scaling:**
- **Bonus Attack Speed:** $30/2-130/2$% Attack Speed per Subsequent Stack $30/4-130/4$% Maximum Attack Speed $30-130$%
- **Bonus Range:** $80-200$

**Notes:**

- Toggle abilities do not count as ability activations for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- Despite **Jinx** starting the game using **Pow-Pow** she doesn't receive any bonus attack speed until *Switcheroo!* has been learned.
- **Fishbones** splash damage affects structures (minions will target **Jinx**) but does not affect targets if ''Jinx's' attacks are block, dodge, or blind.
- *Runaan's Hurricane* Wind's Fury bolts are special-cased to generate one **Pow-Pow** stack per enemy hit.
- Each of *Runaan's Hurricane* additional bolts apply *Fishbones'* splash damage, which stacks with each other.
- Both weapons deal basic damage to their primary target (this includes the increased damage of **Fishbones**), which applies *life steal* based on the damage dealt (post-mitigation) as usual.
- The splash damage of **Fishbones** is based on the pre-mitigation damage done to the primary target (which includes critical strikes), and is not considered *modified physical damage. It deals default damage: It doesn't apply *life steal* or on-hit effects.
  - **Fishbones** damage: (100% + 10%).
  - **Fishbones** critical strike damage: $ ((100% + 10%)×$1.75$).
  - **Fishbones** critical strike damage with *Infinity Edge*: $ ((100% + 10%)×$2.15$).
  - **Fishbones** *Runaan's Hurricane* damage: ((100% + 10%)×*Runaan's Hurricane*).
  - **Fishbones** *Runaan's Hurricane* critical strike damage: $ ((100% + 10%)×*Runaan's Hurricane*×$1.75$).
  - **Fishbones** *Runaan's Hurricane* critical strike damage with *Infinity Edge*: $ ((100% + 10%)×*Runaan's Hurricane*×$2.15$).
  - **Fishbones** potential damage on stacked enemies with *Runaan's Hurricane*: $ (110% + 60,5% + 60,5%).
  - **Fishbones** potential critical strike damage on stacked enemies with *Runaan's Hurricane*: $ ((110% + 60,5% + 60,5%)×$1.75$).
  - **Fishbones** potential critical strike damage on stacked enemies with *Runaan's Hurricane* and *Infinity Edge*: $ ((110% + 60,5% + 60,5%)×$2.15$).

---

### W: Zap!

**Active:** **Jinx** fires a shock bolt in the target direction that deals physical damage to the first enemy hit, slow and standard sight them for a short time.

**Active:** **Jinx** fires a shock blast in the target direction that deals physical damage to the first enemy it hits and standard sight and slow them for 2 seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $8/7/6/5/4$ seconds |
| **Cast Time** | 0.6–0.4@0–250 (@=*bonus attack speed) seconds |
| **Cost** | $40-60$ Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | 3300 units/second |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Projectile** | True |

**Scaling:**
- **Physical Damage:** $10-210$ (+ 140% AD)
- **Slow:** $40-80$%

**Notes:**

No additional notes.

---

### E: Flame Chompers!

**Active:** **Jinx** tosses a line of *Chompers* at the target location that explode after a few seconds, dealing magic damage.

*If an enemy champion steps on a *Chomper*, it will explode, knockdown, and briefly root them.*

**Active:** **Jinx** tosses out 3 *Chompers* centered at the target location, landing after $0.4$ seconds, arming after $0.75$ seconds, and exploding after 5 seconds to deal magic damage to nearby enemies. Each *Chomper* explodes on contact with an enemy champion, knockdown and root them for $1.5$ seconds. Enemy champions can be affected by only one *Chomper*.

| Attribute | Value |
|-----------|-------|
| **Range** | 925 units |
| **Cooldown** | $24-10$ seconds |
| **Cast Time** | none |
| **Cost** | 90 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 225 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**Scaling:**
- **Magic Damage:** $70-270$ (+ 100% AP)

**Notes:**

- *Chompers* will halt when encountering *Wind Wall* and *Unbreakable*.
- Spell shield does not negate the explosion.
- Each *Chomper* gives vision of its surrounding area.

---

### R: Super Mega Death Rocket!

**Active:** **Jinx** fires a rocket in the target direction that explodes upon the first enemy champion hit, dealing physical damage to the target and surrounding enemies.

*The damage is increased based on travel time and each target's missing health. Surrounding enemies take reduced damage.*

**Active:** **Jinx** fires a massive rocket in the target direction, briefly granting sight of its surroundings and exploding upon colliding with an enemy champion. The explosion deals physical damage to nearby enemies and grants sight of the area for 2 seconds. *Super Mega Death Rocket!* deals type=distance traveled damage. This does not affect the **bonus** damage based on the target's **missing** health. Enemies surrounding the primary target take 80% damage. The **bonus** damage based on the target's **missing** health is capped at 1200 against monsters.

| Attribute | Value |
|-----------|-------|
| **Range** | Global |
| **Cooldown** | $85-45$ seconds |
| **Cast Time** | $0.6$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Speed** | $1700 units/second |
| **Effect Radius** | 400 / sight 1000 / 1000 units |
| **Spell Shield** | Special |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**Scaling:**
- **Maximum Physical Damage:** $300-600$ (+ 155%
- *bonus AD) (+ $25-35$% of target's **missing* health)0.1$ (+ $155×0.1$%
- *bonus AD) (+ $25-35$% of target's
- **missing** health)
- **Maximum Secondary Damage:** $(300-600)*0.8$ (+ $155×0.8$%
- *bonus AD) (+ $(25-35)*0.8$% of target's **missing* health)0.8×0.1$ (+ $155×0.8×0.1$%
- *bonus AD) (+ $(25-35)*0.8$% of target's
- **missing** health)

**Notes:**

- Spell shield will not prevent the explosion.
- 'Super Mega Death Rocket's projectile has an icon on the mini-map while it is in flight. It can be seen by only **Jinx** and her allies.
- The bonus damage based on missing health is based on each unit's own missing health and not the primary target's.
- Unlike similar spells, *Super Mega Death Rocket* will not increase it's damage when ranked up while the missile is in flight.
  - This is because the base damage is fixed at a multiple of the damage the rocket initially started with.
  - The *bonus AD ratio will still update when AD is gained or lost between the cast and hit of the missile. Effect at cast time start

---

## Patch History

### V25.06
- *Super Mega Death Rocket!*
  - Minimum base damage reduced to $300×0.1-600×0.1 3$ from $325×0.1-625×0.1 3$.
    - Maximum base damage reduced to $300-600 3$ from $325-625 3$.
  - Minimum bonus AD ratio reduced to $155×0.1$% *bonus AD from $165×0.1$%.
    - Maximum bonus AD ratio reduced to 155% *bonus AD from 165%.

### V14.24
- Jinx
  - Renamed to *Arcane Enemy* from *Arcane*.

### V14.23
- Stats
  - Attack damage growth increased to $3.25$ from $2.9$.
- *Zap!*
  - AD ratio reduced to 140% AD from 160% AD.

### V14.18
- Stats
  - Attack speed growth increased to $1.4$% from 1%.

### V14.11
- Stats
  - Attack damage growth reduced to $2.9$ from $3.15$.

### V14.9
- *Super Mega Death Rocket!*
  - Cooldown increased to $85-45 3$ seconds from $70-40 3$.

### V13.21
- *Get Excited!*
  - **New Effect:** Now capped at 5 stacks.
  - **Removed:*** Epic monster and structure takedowns no longer grant stacks beyond the first.

### V13.20
- Stats
  - Health growth increased to 105 from 100.
- *Get Excited!*
  - **New Effect:** Bonus attack speed now stacks upon takedowns.
- *Super Mega Death Rocket!*
  - Cooldown reduced to $70-40 3$ seconds from $75-55 3$.
  - Minimum base damage increased to $32.5-62.5 3$ from $30-60 3$.
    - Maximum base damage increased to $325-625 3$ from $300-600 3$.
  - Minimum bonus AD ratio increased to $16.5$% *bonus AD from 15%.
    - Maximum bonus AD ratio increased to 165% *bonus AD from 150%.

### V13.11
- Stats
  - Attack speed growth reduced to 1% from $1.36$%.

### V13.9
- Stats
  - Attack damage growth reduced to $3.15$ from $3.4$.

## Trivia

- Jinx's dance is a reference to dance from *Adventure Time* animated series, specifically since the episode.
  - A side-by-side comparison can be seen here.
- *Super Mega Death Rocket!*’s blast radius displays a smiley face upon impact.
  - *Get Excited!* also displays a smiley face over Jinx upon activation.
- Jinx is the first champion whose laugh animation automatically loops.
- Jinx's hair (or scarf on Jinx) forms a heart when she dies.
- Jinx is the only champion to have all her ability names end in an exclamation mark.
- Whenever Jinx is on the opposing team, **Vi** and/or **Caitlyn** each gain the Catch me if you can! cosmetic debuff.
- On 21 August 2018, women have statistically attributed Jinx as "The most visually appealing champion in League".
- Jinx is the third champion to have one skin for each of the three big annual events of the year - Lunar Revel, Harrowing and Snowdown Showdown. The skin that gave her this distinction was Jinx in 2017.
  - The first two were **Katarina** and **Nidalee** with their Warring Kingdoms.md) skins in 2015.
  - **Annie** almost became another champion that fulfilled the criteria after Annie was released in 2013. However, Annie was not released during the Snowdown Showdown event.
- Jinx - **Vi** is one of seven pairs of sibling champions (the others being **Cassiopeia** - **Katarina**, **Kayle** - **Morgana**, **Nasus** - **Renekton**, **Garen** - **Lux**, **Yasuo** - **Yone**, and **Darius** - **Draven**).
  - Though not a pair, **Anivia**, **Ornn**, and **Volibear** are also siblings.
- Jinx's Series 2 Eternals make the following references:
  - *It Worked!* is a reference to the infamous Explosive Monkey that she created to 'save' the other family members from episode 3/Season_1/Episode_3.md) of the Arcane animated series.
- Jinx's classic splash background is a demolished version of **Vi**'s

---
*This page was automatically generated from League of Legends Wiki data.*