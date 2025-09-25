# Darius

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
| **Champion** | Darius |
| **Title** | the Hand of Noxus |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2012-05-23 |
| **Release Patch** | V1.0.0.140 |
| **Latest Changes** | V25.14 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Alt Type** | Tank |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 55 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $652.0$ | $+114.0$ |
| **Mana** | $263.0$ | $+58.0$ |
| **Health Regen** | $10.0$ | $+0.95$ |
| **Mana Regen** | $6.6$ | $+0.35$ |
| **Armor** | $37.0$ | $+5.2$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $64.0$ | $+5.0$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $1.0\%$ | |
| **Windup Modifier** | $0.5$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $25.767$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $155$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $100.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $85.0\%$ |

## Abilities

### Passive: Hemorrhage

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | default |
| **Parry** | True |

**INNATE:** **Darius' ** damaging basic attacks and abilities apply a stack of *Hemorrhage* to enemies for 5 seconds, refreshing on subsequent applications and stacking up to 5 times.

**HEMORRHAGE:** For each stack, the target is dealt「 13 to 30 (+ 30% **bonus** AD) **total** physical damage over the duration, ⟷ 3.25 to 7.5 (+ $7.5$% **bonus** AD) physical damage every $1.25$ seconds over the duration, 」up to a maximum of「 13×5 to 30×5 (+ 150% **bonus** AD) **total** physical damage over the duration. ⟷ 3.25×5 to 7.5×5 (+ $37.5$% **bonus** AD) physical damage with each tick. 」*Hemorrhage* deals 200% damage against monsters.

When **Darius** kills a champion with *Noxian Guillotine*, or applies 5 stacks on an enemy champion, he becomes empowered with *Noxian Might* for 5 seconds.

**NOXIAN MIGHT:** **Darius** gains 30 to 75 for 10 / 85 to 105 for 3 / 130 to 230 for 5 **bonus** attack damage and instantly applies 5 *Hemorrhage* stacks through his usual means.

**Notes:**

- If Hemorrhage's **total** post-mitigation (Damage calculated after modifiers) damage is higher than the target's **current** health, a small area of sight is granted around the target for the duration of Hemorrhage and for another 2 seconds after it ends.
- *Hemorrhage* does not apply to structures nor wards.
- **Darius**' attacks and abilities have to deal damage to apply *Hemorrhage*.
  - *Hemorrhage* cannot be applied to/refreshed against invulnerable targets.
  - *Hemorrhage* **can** be applied to and refreshed against shielded targets.
  - *Hemorrhage* can also be applied on-hit, but will specifically not do so if the target is invulnerable.
- Spell shield prevents *Hemorrhage*’s application from abilities only.

---

### Q: Decimate

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 240 (Inner Radius) / 460 (Outer Radius) units |
| **Cost** | 25 / 30 / 35 / 40 / 45 mana |
| **Cooldown** | 9 / 8 / 7 / 6 / 5 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Darius** becomes ghosted for 1 second and hefts his axe for $0.75$ seconds, afterward swinging it around himself to deal physical damage to nearby enemies. Enemies within the inner radius take 35% damage.

| Attribute | Value |
|-----------|------:|
| **Physical Damage (Blade)** | 50 / 80 / 110 / 140 / 170 (+ 100 / 110 / 120 / 130 / 140% AD) |
| **Reduced Damage (Handle)** | 17.5 / 28 / 38.5 / 49 / 59.5 (+ 35 / 38.5 / 42 / 45.5 / 49% AD) |

Against champions and large monsters hit, **Darius** heals for health. Enemies hit by the inner radius do not refresh the duration of or gain a new stack of Hemorrhage.

***Darius** can move during Decimate, but is unable to declare basic attacks or cast Apprehend or Noxian Guillotine.*

**Notes:**

- *Decimate* will cancel if **Darius** enters stasis or uses Dash during the delay.
  - After the delay, *Decimate* will be placed on a 1-second cooldown.

---

### W: Crippling Strike

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 40 mana |
| **Cooldown** | 5 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Enemies, Structures |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Parry** | True |

**ACTIVE:** **Darius** empowers his next basic attack within 4 seconds to have an uncancellable windup, gain (range) 25 **bonus** range, deal **bonus** physical damage and slow the target by 90% for 1 second. This damage is affected by critical strike modifiers.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 40 / 45 / 50 / 55 / 60% AD |

If this attack kills the target, half of *Crippling Strike*’s cooldown is reduced and its mana cost is refunded.

*Crippling Strike resets **Darius' ** basic attack timer.*

**Notes:**

- *Crippling Strike* deals basic damage but will also trigger spell effects by also being tagged as spell damage.
  - This includes the basic attack itself.
- Spell shield will not block the damage.
- The attack's animation can be cancelled by casting Decimate, but the attack will still land.
- The cooldown reduction and mana refund will not trigger when killing jungle plants.

---

### E: Apprehend

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 535 units |
| **Angle** | er 50° |
| **Cost** | 70 / 60 / 50 / 40 / 30 mana |
| **Cooldown** | 26 / 23.5 / 21 / 18.5 / 16 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Spell Shield** | True |
| **Projectile** | False |

**PASSIVE:** **Darius** gains (armor penetration) armor penetration.

| Attribute | Value |
|-----------|------:|
| **Armor Penetration** | 20 / 25 / 30 / 35 / 40% |

**ACTIVE:** **Darius** sweeps his axe in a cone in the target direction, granting sight of the area for 1 second while pulling enemies hit towards him. Upon arrival, they rebound 150 (Estimated) units off of him, remaining airborne and becoming slowed by 40% for 1 second.

***Darius** is unable to move or cast Decimate or Noxian Guillotine for $0.4$ seconds after Apprehend's cast time.*

**Notes:**

- Enemies hit by *Apprehend* will quickly be dragged to **Darius** before rebounding over a longer period.
  - The slow duration starts as soon as the target rebounds off of **Darius**<!-- , meaning 50% Tenacity will leave the target not slowed at all after the airborne ends.
  - This is disregarding stat update timing, which means that the target's movement speed may still be reduced for up-to $0.25$ seconds after the slow ends-->.
  - : The rebound location is determined when the target reaches **Darius' **location.
  - The rebound's forced movement's duration is roughly $0.5$ seconds, finishing before the airborne duration ends.
  - : There is no stun applied for the same duration of the airborne, meaning brittle cannot extend the disable duration. It will not increase the slow duration in most situations because of the negative tenacity falling off before the slow is applied.
- After the pull, **Darius** will attempt to basic attack the closest pulled target, prioritizing enemy champions.
- The armor penetration stacks multiplicatively with other forms of percentage armor penetration. Effect at cast time end

#### Channel Behavior (cast)

| Aspect | State / Notes |
|--------|---------------|
| **Summoner Spells** | Allowed / Disabled |

---

### R: Noxian Guillotine

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.3667$ seconds |
| **Target Range** | 475 units |
| **Cost** | 100 / 100 / 0 mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | True |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | False |

**ACTIVE:** **Darius** attempts to execute the target enemy champion, leaping towards them to deal true damage, increased by 20px]] Hemorrhage stacks, and granting sight of the area around them for $2.5$ seconds from the start of the cast time.

| Attribute | Value |
|-----------|------:|
| **True Damage** | 125 / 187.5 / 250 / 312.5 / 375 (+ 75% **bonus** AD) |
| **Bonus Damage Per Stack** | 25 / 37.5 / 50 / 62.5 / 75 (+ 15% **bonus** AD) |
| **Maximum True Damage** | 250 / 375 / 500 / 625 / 750 (+ 150% **bonus** AD) |

If the target dies within $0.15$ seconds after being hit by *Noxian Guillotine*, **Darius** fears nearby minions and monsters for 3 seconds, during which they are gradually slowed by up to 99% over the duration. He can also recast the ability within 20 seconds at no cost, which refreshes on further executions.

At rank 3, *Noxian Guillotine* has no mana cost and recast timer.

***Darius** is unable to cast Decimate or Apprehend for $0.25$ seconds after Noxian Guillotine's cast time.*

**Notes:**

- *Noxian Guillotine*’s cooldown after the recast or recast window is calculated by taking the values at the end, and using this formula: (*Noxian Guillotine*’s Cooldown - time since first cast)×100(100 + AH).
- *Noxian Guillotine* applies Hemorrhage after the damage.
- **Darius** will only leap towards the target if it is at range, or leap backwards if it is very close.
  - This leap can pass very thin terrain and will otherwise not be able to. In any case, **Darius** will still hit the target.
  - The ability is disabled while grounded or rooted even if **Darius** is in the range in which he does not leap.
- *Noxian Guillotine* will not reset nor trigger Noxian Might if used to kill a clone or a target protected by resurrection.
- *Noxian Guillotine* will reset but not trigger Noxian Might if it finishes off a target that is in a zombie state.

---

## Patch History

### V25.14
- Decimate
  - Heal increased to 17%–51%@1–2 (@=enemies hit by the *blade*) **missing** health from 15%–45%@1–2 (@=enemies hit by the *blade*).
- Crippling Strike
  - **Bug Fixes:** Now properly rolls for a critical strike only once instead of twice.

### V25.12
- Darius
  - **Bug Fixes:** Various SFX no longer fail to play and no longer end prematurely.

### V25.11
- Darius
  - Adjusted splash art.

### V25.07
- Stats
  - Base armor reduced to 37 from 39.
- Apprehend
  - Cooldown increased to 26 / 23.5 / 21 / 18.5 / 16 seconds from 24 / 21.5 / 19 / 16.5 / 14.

### V25.06
- Hemorrhage
  - Monster damage reduced to 200% from 300%.

### V14.9
- Stats
  - Gameplay radius reduced to 65 units from 80.
  - Selection radius reduced to 120 units from 125.

### V14.7
- Darius
  - Noxian Guillotine
    - **Bug Fixes:** No longer incorrectly abruptly finishes its effects before the cast time if Darius is stunned before the ability's animation plays (and after the player uses it).

### V14.4
- Darius
  - **Bug Fixes:** First encounter VO is now properly playing as intended for Amumu, Maokai, and Sona.
- Darius
  - Crippling Strike
    - **Bug Fixes:** VFX no longer renders over impassable terrain.

### V14.2
- Decimate
  - Mana cost reduced to 25 / 30 / 35 / 40 / 45 from 30 / 35 / 40 / 45 / 50.
  - Heal increased to 15%–45%@1–2 (@=enemies hit by the *blade*) **missing** health from 13%–39%@1–2 (@=enemies hit by the *blade*).
- Apprehend
  - Armor penetration increased to 20 / 25 / 30 / 35 / 40% from 15 / 20 / 25 / 30 / 35%.

### V13.14
- Darius
  - **Bug Fixes:** Basic attack animation now uses the VFX for this skin rather than the classic one.

## Trivia

- This champion has no ability power ratio.
- Darius - Draven is one of seven pairs of sibling champions (the others being Cassiopeia - Katarina, Kayle - Morgana, Garen - Lux, Nasus - Renekton, Yasuo - Yone, and Vi - Jinx).
  - Though not a pair, Anivia, Ornn, and Volibear are also siblings.
- Darius is 6 feet 5 inches (1.96 meters) tall and weighs 266 lbs (121 kilograms).
- Darius was the first champion to have an autonomous Recall / Teleport animation.
  - He and his brother Draven are the first related champions to be released back to back.
- Darius was the first champion to receive Chroma packs for 2 of his skins, namely Darius and Darius.
- Tahm Kench says Darius' strength is 'waning' and that he could 'help him slow time's regress'.
  - This and the strands of gray hair on his head suggest Darius either is of advanced age or ages prematurely after years of exertion & combat stress.
- *Darius* is Latin transliteration, through Greek Δαρεῖος, of Old Persian ** "Upholder of Good".
  - His new background references Darius I.
    - By coincidence, ***Dar**ius* shares Proto-Indo-European language root **dʰer-* with ***Dar**ha*, the birth name of Karma in her updated lore.
- Darius' abilities had different names during development:
  - Hemorrhage used to have a movement speed buff called *Aggression*.
    - It read: *"The carnage of war enlivens Darius, increasing his movement speed."*
  - Hamstring / Noxian Tactics
  - Sudden Death
  - Noxian Might was originally called *Blood Rage* but was renamed by player request to better reflect his personality.
- Despite its name, *Crippling Strike* does not apply the cripple effect as of V5.16.

---
*This page was automatically generated from League of Legends Wiki data.*