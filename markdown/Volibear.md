# Volibear

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
| **Champion** | Volibear |
| **Title** | the Relentless Storm |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2011-11-29 |
| **Release Patch** | V1.0.0.130 |
| **Latest Changes** | V25.18 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top |
| **External Positions** | Top, Jungle |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 1 |
| **Hero Type** | Fighter |
| **Alt Type** | Tank |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 30 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $650.0$ | $+104.0$ |
| **Mana** | $350.0$ | $+70.0$ |
| **Health Regen** | $9.0$ | $+0.75$ |
| **Mana Regen** | $6.25$ | $+0.5$ |
| **Armor** | $31.0$ | $+5.2$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $60.0$ | $+3.5$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $150.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.7$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $50$ units | |
| **Selection Radius** | $125$ units | |
| **Selection Height** | $180$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $110.0\%$ |
| **Healing** | $80.0\%$ |

## Abilities

### Passive: The Relentless Storm

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 450 (Maximum range to chain between targets) units |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | periodic |
| **Projectile** | False |
| **Parry** | Special |

**INNATE:** Whenever **Volibear** damages at least one enemy with a basic attack or ability, he generates a stack of *The Relentless Storm* for 6 seconds, refreshing on subsequent damage and stacking up to 5 times. At 5 stacks, **Volibear** gains *Lightning Claws*.

**THE RELENTLESS STORM:** For each stack, **Volibear** gains (attack speed) 5% (+ 3% per 100 AP) **bonus** attack speed, up to 25% (+ 15% per 100 AP).

**LIGHTNING CLAWS:** **Volibear**’s claws ignite with lightning, empowering his basic attacks on-hit to deal 10+1*x for 3 / then + 2*x for 3 / then + 3*x for 7 / then + 4*x (+ 50% AP) **bonus** magic damage to the target and the nearest visible enemy within 450 units of the target, chaining up to 4 subsequent targets.

**Notes:**

- **Volibear**’s spikes visually grow as *The Relentless Storm* stacks.
- Stacks of *The Relentless Storm* are also granted on-hit, but will specifically not be if the target is invulnerable.
- While at four stacks, the next attack or ability on-hit reaching the fifth stack will be empowered by *Lightning Claws*.
- A stack of *The Relentless Storm* is not gained if the attack is dodged and/or missed if **Volibear** is blinded. A stack is granted even if the attack is blocked. In all cases, *Lightning Claws* will not apply (on-hit damage is parried and the bounce is prevented).
  - Since Frenzied Maul cannot be missed while **Volibear** is blinded, gaining stacks from *The Relentless Storm* and applying *Lighting Claws* from the ability will not be prevented from that parry effect.
- **Volibear** has a hidden passive that grants him 1 armor for every enemy Zilean within 800 range of him.
  - Likewise, Zilean gains 1 ability power for every nearby **Volibear**.
  - Neither **Volibear** nor Zilean need sight of one another to gain these bonuses.
- The empowered attacks do not affect structures nor wards.

---

### Q: Thundering Smash

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 2000 (Enemy champion check) units |
| **Cost** | 50 Mana |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Parry** | True |
| **Call For Help** | True |
| **Grounded** | False |
| **Knockdown** | Special |
| **Windup Time** | $0.4$ s (- $0.1$ per 100% **bonus** attack speed) |

**ACTIVE:** **Volibear** drops on all fours, becoming ghosted and gaining **bonus** movement speed for 4 seconds, doubled while facing a nearby visible enemy champion.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 12 / 16 / 20 / 24 / 28% |
| **Increased Bonus Movement Speed** | 24 / 32 / 40 / 48 / 56% |

During this time, **Volibear**’s next basic attack is empowered to have an uncancellable windup, gain 25 **bonus** range, and , dealing **bonus** physical damage and stunning them for 1 second. This damage applies life steal at 100% effectiveness.

| Attribute | Value |
|-----------|------:|
| **Bonus Physical Damage** | 10 / 30 / 50 / 70 / 90 (+ 120% **bonus** AD) |

If **Volibear** becomes immobilized or polymorphed by an enemy during *Thundering Smash*, the effect ends prematurely and the cooldown is reset.

*Thundering Smash resets **Volibear**’s basic attack timer.*

**Notes:**

- *Thundering Smash*’s bonus damage cannot critically strike; the base damage of the attack **can**, however.
  - Despite this, Randuin's Omen Resilience reduces *Thundering Smash*’s total damage.
- *Thundering Smash* deals basic damage, but also triggers spell effects by also being tagged as spell damage.
- *Thundering Smash* will still apply its effects to the target even if the dash is interrupted, but not if he goes into resurrection.
- *Thundering Smash*’s attack triggers against structures and wards, consuming the effect but dealing its bonus damage, if applicable.
- The windup for the attack completes even if the target becomes untargetable but the stun and damage do not apply.
- The player's screen will flash red briefly and cue a sound effect when **Volibear** becomes immobilized while *Thundering Smash* is active.
- *Thundering Smash*’s attack does not put **Volibear**’s basic attack on cooldown.
  - This results in the same functionality as Leona’s Shield of Daybreak, effectively being a double attack reset.
- If Thundering Smash's buff appears as the first on the buff bar, the empowered attack will fail against structures.

---

### W: Frenzied Maul

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 325 (Standard Range) / 350 (Enhanced Range) units |
| **Cost** | 30 / 35 / 40 / 45 / 50 Mana |
| **Cooldown** | 5 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Parry** | Special |
| **Call For Help** | True |

**ACTIVE:** **Volibear** slashes the target enemy with his claws to deal physical damage, apply on-hit effects, trigger on-attack effects, and mark the target *Wounded* for 8 seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 5 / 30 / 55 / 80 / 105 (+ 100% AD) (+ 6% of his **bonus** health) |

**WOUNDED BONUS:** If the target is already *Wounded*, **Volibear** takes a bite out of them instead, dealing 50% (+ 15% per 100 **bonus** AD) increased damage and healing himself. The heal is halved against minions.

| Attribute | Value |
|-----------|------:|
| **Heal** | 20 / 35 / 50 / 65 / 80 (+ 8 / 11 / 14 / 17 / 20% of his **missing** health) |
| **Minion Heal** | 10 / 17.5 / 25 / 32.5 / 40 (+ 4 / 5.5 / 7 / 8.5 / 10% of his **missing** health) |

*Frenzied Maul* applies life steal at 100% effectiveness.

**Notes:**

- **Volibear** will be ordered to basic attack the target after casting *Frenzied Maul*.
- *Frenzied Maul* deals basic damage, but also triggers spell effects by also being tagged as spell damage.
- *Frenzied Maul*’s bite heals even if it is blocked by spell shield.
- *Frenzied Maul*’s strike can be dodged and blocked, but it cannot miss if **Volibear** is blinded. The *Wound* mark does not apply if dodged, but will otherwise do so regardless.
  - The bite can be blocked but **Volibear** still heals. It does not heal nor deal damage if the bite is dodged. The bite cannot miss if **Volibear** is blinded.
- *Frenzied Maul* deals bonus damage and heals if the target is still *Wounded* after the cast time. If the mark wears off before the cast time completes, the ability's animation will appear as if the bite was applied but there is no bonus damage or heal.

---

### E: Sky Splitter

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 1200 units |
| **Effect Radius** | 325 (Damage radius) / 425 (Shield granting radius) units |
| **Cost** | 60 Mana |
| **Cooldown** | 14 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Location |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Volibear** summons a lightning bolt to strike at the target location after a 2-second delay, granting sight of the area for 1 second after the first second of the delay. If **Volibear** is within the strike, he gains a shield equal to 14% of his **maximum** health (+ 75% AP) for 3 seconds.

The bolt deals magic damage to enemies hit, capped at 650 against non-champions, and slows them by 40% for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 110 / 140 / 170 / 200 (+ 80% AP) (+ 11 / 12 / 13 / 14 / 15% of target's **maximum** health) |

**Notes:**

- Enemies cannot see the location of the cast for the first second, but they can already tell that the spell is underway by noticing **Volibear**’s cast animation.
  - Volibear can cast *Sky Splitter* during the movement speed boost from *Thundering Smash* to prevent the animation from playing.
- **Volibear** will receive the shield even if he is untargetable.

---

### R: Stormbringer

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 700 units |
| **Effect Radius** | 300 (Damage radius) / 500 (Slow radius) / 700 (Turret damage and disable radius) units |
| **Speed** | 750 (Dash speed) units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 160 / 147.5 / 135 / 122.5 / 110 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Location |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Knockdown** | False |

**ACTIVE:** **Volibear** gains *Stormbringer* for 12 seconds and leaps to the target location with displacement immunity over 1 second, granting sight of the area in a 500 radius during the travel.

**STORMBRINGER:** **Volibear** gains ghosting, **bonus** health, 50 **bonus** attack range, 25 increased range on *Frenzied Maul*, and 35% increased size.

| Attribute | Value |
|-----------|------:|
| **Bonus Health** | 175 / 262.5 / 350 / 437.5 / 525 |

**Volibear** impacts (centered on Volibear's location when the dash ends in any way) after 1 second, slowing nearby enemies by 50% decaying over 1 second. Enemies within the epicenter are also dealt physical damage.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 300 / 400 / 500 / 600 / 700 (+ 250% **bonus** AD) (+ 125% AP) |

**Volibear** also disables enemy turrets in an area for a duration, rendering them unable to attack, as well as dealing them the same damage.

| Attribute | Value |
|-----------|------:|
| **Turret Disable Duration** | 2 / 2.5 / 3 / 3.5 / 4 seconds |

*Thundering Smash’s duration will be paused for the leap.*

**Notes:**

- *Stormbringer* will disable any turret that is not the Nexus Obelisk, even if it is untargetable.
  - *Stormbringer* will not deal damage to untargetable turrets.
- **Volibear** grows to his new over $1.25$ seconds, starting $0.25$ seconds after landing, and shrinks back to his normal over $0.5$ seconds after *Stormbinger*’s status ends, respectively.
- **Volibear** leaps over 1 second regardless of distance or movement speed.
  - **Volibear** impacts immediately at his current location if *Stormbringer* is cast over terrain he cannot pass through (due to not being able to cover the distance required).
- The additional bonuses are granted on-cast.
- The turret disable debuff is named *Ohmwrecker*.
- Disabling a turret does not prevent aggro of the current target it is locked onto. The increased turret shot damage from Ohmwrecker (Turret Item) is also reset.
  - Turrets maintain the same targeting behavior even when disabled; damaging an enemy champion will still draw turret aggro. However, if the turret's desired target leaves range or has become an invalid target, it will lock onto the most previous target it was going to attack prior to becoming disabled, or instead, find a new one if that condition is not applicable.
- The following table refers for interactions while **Volibear** is dashing:

---

## Patch History

### V25.18
- The Relentless Storm
  - Attack speed AP ratio per stack reduced to 3% per 100 AP from 4% per 100 AP.
    - Maximum stacks attack speed AP ratio reduced to 15% per 100 AP from 20% per 100 AP.

### V25.07
- Frenzied Maul
  - **New Effect:** Wounded bonus damage now scales with 15% per 100 **bonus** AD.

### V14.21
- Sky Splitter
  - Cooldown increased to 14 from 12.

### V14.10
- Thundering Smash
  - **Removed:*** The empowered attack is no longer able to make Volibear dash over thin enough walls if his target on the other is within his attack range.

### V14.8
- The Relentless Storm
  - **Bug Fixes:** Adaptive and converted stats are now considered for this ability's scalings.

### V14.7
- Thundering Smash
  - **Removed:*** Bonus damage is no longer affected by critical strike modifiers.
- Stormbringer
  - Cooldown increased to 160 / 135 / 110 seconds from 140 / 120 / 100.

### V14.6
- Thundering Smash
  - Bonus movement speed reduced to 12 / 16 / 20 / 24 / 28% from 12 / 17 / 22 / 27 / 32%.
    - Increased bonus movement speed reduced to 24 / 32 / 40 / 48 / 56% from 24 / 34 / 44 / 54 / 64%.
- Frenzied Maul
  - **Bug Fixes:** No longer unintentionally applies the mark or heal when dodged.
- Stormbringer
  - Cooldown increased to 140 / 120 / 100 seconds from 130 / 115 / 100.

### V14.5
- Thundering Smash
  - **Bug Fixes:** Now properly draws the aggro of enemy minions towards the caster if it is used against an enemy champion.
- Frenzied Maul
  - **Bug Fixes:** Now properly draws the aggro of enemy minions towards the caster if it is used against an enemy champion.

### V14.4
- Thundering Smash
  - Bonus movement speed increased to 12 / 17 / 22 / 27 / 32% from 8 / 12 / 16 / 20 / 24%.
    - Increased bonus movement speed increased to 24 / 34 / 44 / 54 / 64% from 16 / 24 / 32 / 40 / 48%.
- Sky Splitter
  - Cooldown reduced to 12 seconds from 13.
- Stormbringer
  - Cooldown reduced to 130 / 115 / 100 seconds from 160 / 140 / 120.
  - Tower disable duration reduced to 2 / 3 / 4 seconds from 3 / 4 / 5.
  - Disable immunity during the dash changed to displacement immunity from total crowd control immunity.

### V14.2
- Thundering Smash
  - **New Effect:** Empowered attack now gains 25 bonus attack range.

## Trivia

- Volibear was the first champion released in Season Two.
  - He was named after Travis 'Volibar' George, from a long running forum meme of him supporting armored bears in the League.
  - *Volibar* comes from Basque "mill" and *ibar* "valley".
- This demi-god's preferred epithet *Valhir* is possibly based on unattested Old Norse *** "fire of the battle-slain"; *valr* "battle-slain" is also found in Valhalla "hall of the battle-slain" and Valkyrie "chooser of the battle-slain".
  - His other epithet *the Volibear* possibly also contains a variant of *valr*.
- His Ursine tribe possibly references on real-world bear worship and the norse Berserker.
- Volibear might have been inspired by the *Panserbjørne* (Danish/Norwegian: 'armored bears') from His Dark Materials by Philip Pullman.
- Volibear’s The Relentless Storm, Aatrox’s The Darkin Blade, Jax’s Grandmaster-at-Arms, and Vayne’s Night Hunter are the only abilities in *League of Legends* that have the same name as their champion's title.
- Volibear's dance both before and after his visual update references Sam B's dance in America's Got Talent.
  - A side-by-side comparison from before his update can be seen here.
  - A side-by-side comparison from after his update can be seen here.
- In Volibear's 'Champion Spotlight', his release skin names were erroneously used to label the opposite one (i.e. 'Volibear' was on 'Volibear' and vice versa).
- Rolling Thunder might be referencing the eponymous piece by Henry Fillmore and/or Operation Rolling Thunder from the Vietnam War.

---
*This page was automatically generated from League of Legends Wiki data.*