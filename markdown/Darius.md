# Darius

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
| **Champion** | Darius |
| **Title** | the Hand of Noxus |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2012-05-23 |
| **Release Patch** | V1.0.0.140 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top |
| **External Positions** | Top |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $652.0$ | $+114.0$ | $2590.0$ |
| **Mana** | $263.0$ | $+58.0$ | $1249.0$ |
| **Health Regen** | $10.0$ | $+0.95$ | $26.1$ |
| **Mana Regen** | $6.6$ | $+0.35$ | $12.5$ |
| **Armor** | $37.0$ | $+5.2$ | $125.4$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $64.0$ | $+5.0$ | $149.0$ |
| **Attack Speed** | $0.625$ | $+1.0\%$ | $0.731$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $175.0$ | $+0.0$ | $175.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.0\%$ |
| **Windup Modifier** | $0.5$ |
| **Acquisition Radius** | $400 units$ |
| **Pathing Radius** | $25.767 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $155 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Hemorrhage

**Innate:** **Darius**’s basic attacks and damaging abilities apply a stack of *Hemorrhage*, stacking up to a cap.

**Darius** becomes empowered with **Noxian Might** for a few seconds whenever he kills an enemy with **Noxian Guillotine** or applies maximum stacks to an enemy champion.

**Innate:** ''Darius'* damaging basic attacks and abilities apply a stack of *Hemorrhage' to enemies for 5 seconds, refreshing on subsequent applications and stacking up to 5 times. **Hemorrhage:* For each stack, the target is dealtup to a maximum of*Hemorrhage* deals 200% damage against monsters. When **Darius** kills a champion with **Noxian Guillotine**, or applies 5 stacks on an enemy champion, he becomes empowered with *Noxian Might* for 5 seconds. **Noxian Might:** **Darius** gains 30 to 75 for 10–130 to 230 for 5 **bonus attack damage** and instantly applies 5 *Hemorrhage* stacks through his usual means.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | default |

**Notes:**

- If Hemorrhage's **total** post-mitigation damage is higher than the target's **current** health, a small area of sight is granted around the target for the duration of Hemorrhage and for another 2 seconds after it ends.
- *Hemorrhage* does not apply to structures nor wards.
- **Darius**' attacks and abilities have to deal damage to apply *Hemorrhage*.
  - *Hemorrhage* cannot be applied to/refreshed against invulnerable targets.
  - *Hemorrhage* **can** be applied to and refreshed against shield targets.
  - *Hemorrhage* can also be applied on-hit, but will specifically not do so if the target is invulnerable.
- Spell shield prevents 'Hemorrhage's application from abilities only.

---

### Q: Decimate

**Active:** **Darius** briefly winds up to spin his axe in a circle while ghosted, dealing physical damage to enemies hit.

*When **Darius** hits a champion with the blade, he applies a *Hemorrhage* stack and heals based on his **missing** health, multiplied by the number of champions hit up to 3.*

**Active:** **Darius** becomes ghosted for 1 second and hefts his axe for $0.75$ seconds, afterward swinging it around himself to deal physical damage to nearby enemies. Enemies within the inner radius take 35% damage. Against champions and large monsters hit, **Darius** heal for . Enemies hit by the inner radius do not refresh the duration of or gain a new stack of *Hemorrhage*. **Darius can move during Decimate, but is unable to declare basic attacks or cast **Apprehend* or *Noxian Guillotine*.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $9-5$ seconds |
| **Cast Time** | none |
| **Cost** | $25-45$ mana |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 240 / 460 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Physical Damage (Blade):* $50-170$ (+ $100-140$% AD)0.35-170×0.35$ (+ $100×0.35-140×0.35$% AD)

**Notes:**

- *Decimate* will cancel if **Darius** enters stasis (buff) or uses Dash during the delay.
  - After the delay, *Decimate* will be placed on a 1-second cooldown.

---

### W: Crippling Strike

**Active:** **Darius**’s next basic attack within a few seconds deals increased physical damage and slow the target.

*If the empowered attack kills the target, the cooldown is ah and the is refunded.*

**Active:** **Darius** empowers his next basic attack within 4 seconds to have an uncancellable windup, gain range*bonus** range*, deal **bonus** physical damage and slow the target by 90% for 1 second. This damage is affected by critical strike modifiers. If this attack kills the target, half of 'Crippling Strike's *cooldown* is reduced and its is refunded. *Crippling Strike basic attack reset *'Darius'* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 5 seconds |
| **Cast Time** | none |
| **Cost** | 40 mana |
| **Targeting** | Auto |
| **Affects** | Enemies, Structures |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | Special |

**Scaling:**
- **Bonus Physical Damage:** $40-60$% AD

**Notes:**

- *Crippling Strike* deals basic damage but will also trigger spell effects by also being tagged as spell damage.
  - This includes the basic attack itself.
- Spell shield will not block the damage.
- The attack's animation can be cancelled by casting *Decimate*, but the attack will still land.
- The cooldown reduction and mana refund will not trigger when killing jungle plants.

---

### E: Apprehend

**Passive:** **Darius** gains .

**Active:** **Darius** sweeps in a cone in the target direction, slow and airborne enemies hit towards him.

**Passive:** **Darius** gains armor penetration. **Active:** **Darius** sweeps his axe in a cone in the target direction, granting sight of the area for 1 second while airborne enemies hit towards him. Upon arrival, they rebound 150 units off of him, remaining airborne and becoming slow by 40% for 1 second. '**Darius** is unable to move or cast *Decimate* or *Noxian Guillotine* for $0.4$ seconds after Apprehend's cast time.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $26-16$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-30$ mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Effect Radius** | 535 units |
| **Spell Shield** | True |
| **Projectile** | False |

**Scaling:**
- **Armor Penetration:** $20-40$%

**Notes:**

- Enemies hit by *Apprehend* will quickly be dragged to **Darius** before rebounding over a longer period.
  - The slow duration starts as soon as the target rebounds off of **Darius**<!-- , meaning 50% Tenacity will leave the target not slowed at all after the airborne ends.
  - This is disregarding stat update timing, which means that the target's movement speed may still be reduced for up-to $0.25$ seconds after the slow ends-->.
  - : The rebound location is determined when the target reaches ''Darius''location.
  - The rebound's forced movement's duration is roughly $0.5$ seconds, finishing before the airborne duration ends.
  - : There is no stun applied for the same duration of the airborne, meaning brittle cannot extend the disable duration. It will not increase the slow duration in most situations because of the negative tenacity falling off before the slow is applied.
- After the pull, **Darius** will attempt to basic attack the closest pulled target, prioritizing enemy champions.
- The armor penetration stacks multiplicatively with other forms of percentage armor penetration. Effect at cast time end

---

### R: Noxian Guillotine

**Active:** **Darius** lunge to an enemy champion and strikes a lethal blow, dealing true damage based on the target's current **Hemorrhage** stacks.

*If the target dies very briefly afterwards, 'Noxian Guillotine's cooldown is temporarily ah, and nearby minions and monsters are fear and slow.*

**Active:** **Darius** attempts to execute the target enemy champion, lunge towards them to deal *true damage*, increased by type=target's [File:Hemorrhage.png, and granting sight of the area around them for $2.5$ seconds from the start of the cast time. If the target dies within $0.15$ seconds after being hit by *Noxian Guillotine*, **Darius** fear nearby minions and monsters for 3 seconds, during which they are gradually slow by up to 99% over the duration. He can also recast the ability within 20 seconds at no cost, which refreshes on further executions. At rank 3, *Noxian Guillotine* has no *mana cost* and recast timer. '**Darius** is unable to cast *Decimate* or *Apprehend* for $0.25$ seconds after Noxian Guillotine's cast time.'

| Attribute | Value |
|-----------|-------|
| **Range** | 475 units |
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | $0.3667$ seconds |
| **Cost** | $100/100/0$ mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | True |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **True Damage:** $125-375$ (+ 75% bonus AD)0.2-375×0.2$ (+ 15% bonus AD)2-375×2$ (+ 150% bonus AD)

**Notes:**

- 'Noxian Guillotine's* cooldown after the recast or recast window is calculated by taking the values at the end, and using this formula: (*Noxian Guillotine's Cooldown - time since first cast)×100(100 + *AH*).
- *Noxian Guillotine* applies *Hemorrhage* after the damage.
- **Darius** will only leap towards the target if it is at range, or leap backwards if it is very close.
  - This leap can pass very thin terrain and will otherwise not be able to. In any case, **Darius** will still hit the target.
  - The ability is disabled while ground or root even if **Darius** is in the range in which he does not leap.
- *Noxian Guillotine* will not reset nor trigger *Noxian Might* if used to kill a clone or a target protected by resurrection.
- *Noxian Guillotine* will reset but not trigger *Noxian Might* if it finishes off a target that is in a zombie state.

---

## Patch History

### V25.14
- *Decimate*
  - Heal increased to changedisplay=true **missing** health from changedisplay=true.
- *Crippling Strike*
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
- *Apprehend*
  - Cooldown increased to $26-16$ seconds from $24-14$.

### V25.06
- *Hemorrhage*
  - Monster damage reduced to 200% from 300%.

### V14.9
- Stats
  - Gameplay radius reduced to 65 units from 80.
  - Selection radius reduced to 120 units from 125.

### V14.7
- Darius
  - *Noxian Guillotine*
    - **Bug Fixes:** No longer incorrectly abruptly finishes its effects before the cast time if Darius is stunned before the ability's animation plays (and after the player uses it).

### V14.4
- Darius
  - **Bug Fixes:** First encounter VO is now properly playing as intended for **Amumu**, **Maokai**, and **Sona**.
- Darius
  - *Crippling Strike*
    - **Bug Fixes:** VFX no longer renders over impassable terrain.

### V14.2
- *Decimate*
  - Mana cost reduced to $25-45$ from $30-50$.
  - Heal increased to changedisplay=true **missing** health from changedisplay=true.
- *Apprehend*
  - Armor penetration increased to $20-40$% from $15-35$%.

### V13.14
- Darius
  - **Bug Fixes:** Basic attack animation now uses the VFX for this skin rather than the classic one.

## Trivia

- This champion has no ability power ratio.
- Darius - **Draven** is one of seven pairs of sibling champions (the others being **Cassiopeia** - **Katarina**, **Kayle** - **Morgana**, **Garen** - **Lux**, **Nasus** - **Renekton**, **Yasuo** - **Yone**, and **Vi** - **Jinx**).
  - Though not a pair, **Anivia**, **Ornn**, and **Volibear** are also siblings.
- Darius is 6 feet 5 inches (1.96 meters) tall and weighs 266 lbs (121 kilograms).
- Darius was the first champion to have an autonomous Recall / Teleport animation.
  - He and his brother **Draven** are the first related champions to be released back to back.
- Darius was the first champion to receive [Chroma packs for 2 of his skins, namely Darius and Darius.
- **Tahm Kench** says Darius' strength is 'waning' and that he could 'help him slow time's regress'.
  - This and the strands of gray hair on his head suggest Darius either is of advanced age or ages prematurely after years of exertion & combat stress.
- *Darius is Latin transliteration, through Greek Δαρεῖος, of Old Persian '' "Upholder of Good".
  - His new background references Darius I.
    - By coincidence, **Darius** shares Proto-Indo-European language root **dʰer-* with **Darha**, the birth name of **Karma** in her updated lore.
- Darius' abilities had different names during development:
  - *Hemorrhage* used to have a movement speed buff called *Aggression*.
    - It read: *"The carnage of war enlivens Darius, increasing his movement speed."*
  - *Hamstring / Noxian Tactics*
  - *Sudden Death*
  - *Noxian Might* was originally called *Blood Rage* but was renamed by player request to better reflect his personality.
- Despite its name, *Crippling Strike* does not apply the cripple effect as of V5.16.

---
*This page was automatically generated from League of Legends Wiki data.*