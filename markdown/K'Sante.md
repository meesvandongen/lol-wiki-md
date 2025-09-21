# K'Sante

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
| **Champion** | K'Sante |
| **Title** | the Pride of Nazumah |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2022-11-02 |
| **Release Patch** | V12.21 |
| **Roles** | Warden, Skirmisher |
| **Riot Positions** | Top |
| **External Positions** | Top |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $625.0$ | $+120.0$ | $2665.0$ |
| **Mana** | $320.0$ | $+60.0$ | $1340.0$ |
| **Health Regen** | $9.5$ | $+1.0$ | $26.5$ |
| **Mana Regen** | $7.0$ | $+1.0$ | $24.0$ |
| **Armor** | $36.0$ | $+5.2$ | $124.4$ |
| **Magic Resist** | $30.0$ | $+2.1$ | $65.7$ |
| **Attack Damage** | $64.0$ | $+3.5$ | $123.5$ |
| **Attack Speed** | $0.688$ | $+2.5\%$ | $0.980$ |
| **Movement Speed** | $330.0$ | $+0.0$ | $330.0$ |
| **Attack Range** | $150.0$ | $+0.0$ | $150.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.688$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $2.5\%$ |
| **Acquisition Radius** | $400 units$ |
| **Gameplay Radius** | $80 units$ |
| **Pathing Radius** | $50 units$ |
| **Selection Radius** | $140 units$ |
| **Selection Height** | $225 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Dauntless Instinct

**Innate:** **K'Sante**’s abilities mark enemies hit for a short time. His basic attacks on-hit consume the mark to deal **bonus** physical damage based on the target's **maximum** health.

**Innate:** ''K'Sante's' abilities mark enemies hit for 4 seconds. His basic attack against marked enemies are empowered to consume the mark on-hit, dealing 12 (+ 1 to 2 of target's **maximum** health) **bonus** physical damage. The damage based on the target's health has a minimum of 10 + 5*x against minion and is capped at 15 + 5*x against monster. **All Out Bonus:** ''K'Sante's* basic attacks and ability damage, as well as *Dauntless Instinct's mark consumption, are empowered to deal **bonus** physical damage equal to 1% .

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | Special |
| **Spell Effects** | proc |

**Notes:**

- Spell shield will block the mark's application, but not the mark's consumption from an attack.
- The empowered attack can be dodge, block, and missed while blind but the mark will not be consumed.
- Starting the empowered attack while the mark is about to time out will not extend the mark duration, but still complete the attack and apply the bonus damage.
- All Out's bonus damage on abilities only applies to the first target hit when damaging multiple enemies (including minions and monsters).
  - For most of ''K'Sante's' spells and item effects, in the case of multiple targets being stuck on the same game tick, this refers to the closest target. For non-missile *Ntofo Strikes*, it instead depends on the order in which the target units were spawned into the match.
- All Out's bonus damage on abilities is applied in the same damage instance from ''K'Sante's' basic attacks and abilities.
  - This causes effects like *Bone Plating* and *Black Cleaver* Carve to be triggered only once from the respective hit.

---

### Q: Ntofo Strikes

**Active:** **K'Sante** slams his ntofo down in the target direction that deals physical damage and briefly slow enemies hit.

*If this hits at least one enemy, he generates a stack for a few seconds. At 2 stacks, the next *Ntofo Strikes* consumes them all to become empowered with a new effect.*

**Active:** ''K'Sante'' slams his ntofo down in the target direction that deals physical damage to enemies hit and slow them by 80% for $0.5$ seconds. If this hits at least one enemy, ''K'Sante'* generates a stack for 6 seconds, stacking up to 2 times and refreshing on subsequent hits. At 2 stacks, the next *Ntofo Strikes' cast consumes them all to become empowered with a new effect. **Empowered Active:** ''K'Sante'' fires a shockwave in the target direction that applies the same effects to enemies hit, but also airborne them towards him over $0.65$ seconds and stun them for $0.8$ seconds. **All Out Bonus:** 'Ntofo Strikes' * *cooldown* is reduced by 33%, with a minimum total cooldown of $1.33$ seconds. Upon entering *All Out*, *Ntofo Strikes' * stacks are reset. *Ntofo Strikes' * *cooldown* is reset if *'K'Sante'* had 2 stacks before entering *All Out'. *Ntofo Strikes basic attack reset *'K'Sante's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cast Time** | 0.45 to 0.35 seconds |
| **Cost** | 20 Mana |
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**Scaling:**
- **Physical Damage:** $70-190$
- *bonus armor) (+ 40%
- **bonus** magic resistance)

**Notes:**

- If *Ntofo Strikes* hits no more than one enemy champion, ''K'Sante'' will be ordered an attack command against them.
- The basic attack reset is not considered one for *Hail of Blades*.
- Temporary increases/decreases in bonus resistances will count for reducing/increasing *Ntofo Strikes*' cooldown.
- Only the empowered active is a projectile.
- The following table refers for interactions while ''K'Sante'' is in the cast time of the third cast:

---

### W: Path Maker

**Active:** **K'Sante** begins to channel, during which he gains damage reduction. After a brief delay, *Path Maker* can be recast within the remaining charge duration, and does so automatically afterwards. The charge cannot be interrupt.

**Recast:** ''K'Sante'' dash in the target direction, dealing physical damage to enemies hit, airborne and stun all targets.

**Active:** ''K'Sante'* raises his ntofos defensively and prepares to dash in the target direction, channel for a minimum of $0.4$ seconds and up to 1 second. During this time, he gains displacement immunity and 30% damage reduction; additionally, *Path Maker's* range, stun duration, and **All Out*' **bonus** true damage modifier increase over the first $0.9$ seconds of the channel. *Path Maker* can be recast within the duration, and does so automatically afterwards. 'Path Maker's charge cannot be interrupt by crowd control. **Recast:** ''K'Sante'' dash in the direction he targeted at the time of cast, though not through terrain, dealing physical damage to enemies he passes through, airborne them alongside him, and stun them for type=channel time seconds. This damage is capped against monsters. **All Out Bonus:** *Path Maker* no longer applies its airborne and stun, but its damage reduction is increased to 75%, the dash deals true damage*bonus** true damage*, and the dash speed is increased by 20%. Upon entering *All Out*, 'Path Maker's *cooldown* is refreshed. '*Ntofo Strikes* and *All Out* can be cast during the dash. Path Maker's recast can be used while affected by cast-inhibiting crowd control.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $14-10$ seconds |
| **Cast Time** | none |
| **Cost** | $40-60$ Mana |
| **Targeting** | Direction / Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Speed** | 1300 / 1500 / 1800 units/second |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Physical Damage:** $45-165$
- *bonus armor) (+ 2% per 100
- **bonus** magic resistance) of target's
- **maximum** health)
- **Monster Damage Cap:** $180-500$

**Notes:**

- *All Out*’s duration is increased by 2 seconds if it were to expire during 'Path Maker's charge.
- *Path Maker* cancels any existing movement or attack commands. However, new inputs given during the charge will be retained once the dash ends, except when the dash hits only one enemy champion. If *Path Maker* hits only one enemy champion ''K'Sante'' will be ordered to basic attack them afterwards.
- ''K'Sante'* cannot use Flash during *Path Maker's dash.
- 'Path Maker's* damage is not capped against monsters while empowered by *All Out'.
- 'Path Maker's recast can be used even while ground or root, but the initial cast cannot.
- *Path Maker* can be recast by issuing an attack move command (or **LMB**) but not a targeted attack command.
- The following table refers for interactions while ''K'Sante'' is channel:
  - If the charge is cancelled, he will not automatically use the recast.

---

### E: Footwork

**Active:** **K'Sante** dash, granting himself a shield.

*If cast toward an ally, *Footwork* has increased range and can dash through terrain. If the ally is a champion, they also receive the shield.*

**Active:** ''K'Sante'' dash to the target location, though not through terrain, and grants himself a shield for 2 seconds. *Footwork* can also be cast on allies with increased range and speed, and the ability to dash through terrain. If the target ally is a champion, they receive the shield as well. **All Out Bonus:** 'Footwork's dash speed is increased, and its cooldown is reduced by 50%. '''K'Sante'* can cast any of his abilities during the dash.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $10-8$ seconds |
| **Cast Time** | none |
| **Cost** | $45-65$ Mana |
| **Targeting** | Location / Unit |
| **Affects** | Self, Allies |
| **Speed** | (500 / 1250 / 1100 / 1400) + 100% movement speed |

**Scaling:**
- **Shield Strength:** $80-240$ (+ 15%
- **bonus** health)

**Notes:**

- 'Footwork's cast on allies has a forgiveness radius of 150 units.
- Dashing to the or behind will cancel current move or attack orders. Exception when attack order was given at melee range from target.
- The shield is granted instantly on cast for both ''K'Sante'' and the potential allied champion.

---

### R: All Out

**Active:** **K'Sante** root an enemy and then shatters his tonfas, airborne, blink behind them, and dealing physical damage. If the enemy is pushed into terrain, they will be knocked over it and take additional physical damage.

*The target is stun at the end of the push, and also airborne again if they were pushed through terrain. At the end, ''K'Sante'* goes *All Out' for some time.*

**Active:** ''K'Sante'' root the target enemy champion for $0.5$ seconds and gains displacement immunity over the cast time. He then shatters his ntofos, airborne the target to a location that is 300 units in the cast direction from their location at the time of cast, during which they are standard sight, and blink 175 units behind that location. The target is dealt physical damage near the end of the displacement and is stun for $0.3$ seconds once it ends. If there is valid map-generated terrain 350 units in the cast direction from the target's location at the time of cast, the target is instead airborne to a location that is directly 450 units away from the edge of the other of the terrain and ''K'Sante* blink 100 units behind that location, dealing them the physical damage near their emergence from the terrain. At the end of the displacement, they remain airborne for into the airborne duration to deal physical damage. After ''K'Sante'* blink, he enters *All Out' for 15 seconds. **All Out:** ''K'Sante** gains a health threshold equal to *65% **maximum'* health* which cannot be modified nor exceeded by any means. Upon entering *All Out*, his **current** health is reduced to this threshold if it is above it. Additionally, his *armor **base** armor* and *mr **base** magic resistance* are reduced by 85% *bonus armor and 85% **bonus** magic resistance, respectively. In return, he gains **bonus attack speed**, lethality*bonus**-armor penetration*, and *20% omnivamp*, and modifies his ability which can be cast at no cost. Upon entering *All Out*, *'K'Sante** is restored to 100% **maximum'* mana. His mana then decays to 0 over the duration of the buff, after which the amount of Preserves the exact amount of mana, not his mana percentage at the time he had prior to entering *All Out' is restored. '''K'Sante** retains his **maximum** health and **bonus** resistances during All Out. Health lost from gaining the health threshold is **not** restored after the effect ends. If the target is pulled over terrain that is longer than 2000 units, **K'Sante'* will vanish during their displacement and reappear near the end of it.*

| Attribute | Value |
|-----------|-------|
| **Range** | 300 units |
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | $0.4$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Speed** | 2000 units/second |
| **Effect Radius** | 350 / 5000 / Global |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **Physical Damage:** $80-150$
- **Strike Physical Damage:** $80-150$ (+ 5% of his
- **bonus** health)
- **Total Physical Damage:** $80×2-150×2$ (+ 5% of his
- **bonus** health)

**Notes:**

- 'All Out's cast has a forgiveness radius of er 50 units.
- If the terrain behind the target is invalid by not having an opposite side, *All Out* will not trigger its additional effects from pulling the target over terrain.
  - The pull destination in this case would be inside terrain and thus the displacement would stop at the boundary.
- ''K'Sante'* will restore the exact amount of mana he had before going *All Out' after the buff expires.
  - The amount does not change even if his **maximum** mana changes during the effect.
- While the target is being airborne over terrain, they are additionally attach to a missile that travels in the same trajectory as the displacement, becoming detached afterwards.
  - Removing the airborne debuff will not cause the target to detach from the missile.
- If the target resists the airborne by having a spell shield or being cc-immune or displacement immune, ''K'Sante'' will still blink beyond the pull's end location as if the target were displaced.
- ''K'Sante's' strike at the end of the displacement when the target is pulled over terrain does not apply *Dauntless Instinct*’s mark.
- Going *All Out* does not override ''K'Sante's' attack or movement commands if the target does not collide with terrain.
  - If the target is however pulled through terrain then ''K'Sante'' will be ordered to basic attack the target afterwards.
- The percentage bonus armor penetration stacks additively with other sources of percentage armor penetration.
- Since ''K'Sante'* retains the stats he "lost", any effects that scale based on those stats will calculate based on the value prior to him going *All Out'.
  - His **base** resistances can be reduced below 0 by his **bonus** resistances.
- *All Out* will not take into account resistances gained through % bonuses (eg. *Conditioning*, Mountainous Vigor, etc) when reducing ''K'Sante's base'' resistances.
  - *Jak'Sho, The Protean* Voidborn Resilience specifically is taken into account.
- Gaining **bonus** health during *All Out* causes the health threshold to be briefly increased then reset back to its original value.
  - The **bonus** health is still applied to his **current** health.
- During 'All Out's cast and displacement, the target's camera is locked and centered on their champion.
  - For ''K'Sante'', his camera centers on him at the start of the cast time, then once again at the end of the cast time at the location of his blink behind the target, and it is not locked.
- 'All Out's granted untargetability from the vanish during the target's pull of over 2000 units does not homing projectile destruction in-flight projectiles.
- The following table refers for interactions while ''K'Sante'' is in cast time:
- The following table refers for interactions while ''K'Sante'* is performing *All Out's cast:

---

## Patch History

### V25.18
- *Dauntless Instinct*
  - **Bug Fixes:** Tooltip now notes the damage floor and cap based on minions' and monsters' health ratio, respectively.
    - The correct *current* numbers are noted, but the tooltip is not set up to automatically update if they change in the future.

### V25.11
- *All Out*
  - **Bug Fixes:** While he is All Out and *Aftershock* is equipped, no longer incorrectly receives excessive damage from a champion who has gained the effects of *Last Breath*’s bonus-armor penetration.

### V25.09
- General
  - Updated recommended items. No longer incorrectly displays various Fighter items instead of Tank items.
- *Path Maker*
  - Base damage increased to $45-165$ from $40-120$.

### V25.06
- *Path Maker*
  - **Bug Fixes:** No longer fails to apply its effects when buffering it to cast shortly before a ground debuff ends.
  - **Bug Fixes:** After buffering a *Footwork* + *Ntofo Strikes* + Path Maker combo, no longer causes Path Maker to prematurely cancel shortly after casting.

### V25.05
- *Dauntless Instinct*
  - Mark base damage reduced to 12 from 20.
- *Path Maker*
  - *All Out* bonus true damage reduced to key=% from key=%.

### V25.04
- *Ntofo Strikes*
  - **Bug Fixes:** Corrected SFX origin point.

### V14.23
- *Footwork*
  - **Bug Fixes:** Buffering 'Footwork's cast on an ally target no longer improperly triggers on-cast effects (e.g. activating Spellblade).

### V14.22
- *Ntofo Strikes*
  - Base damage reduced to $70-190$ from $80-200$.
- *Footwork*
  - Location dash speed while *All Out* increased to 1250 from 950.

### V14.21
- *Footwork*
  - **Bug Fixes:** Granting an ally *Moonstone Renewer* 'Starlit Grace's* shield via *Footwork' no longer allows the shield to last indefinitely.
- Stats
  - **Undocumented:** Base attack speed increased to $0.688$ from $0.625$.
  - Base armor increased to 36 from 33.
- *Ntofo Strikes*
  - Base damage increased to $80-200$ from $70-190$.
  - Damage armor ratio increased to 40% *bonus armor from 35%.
  - Damage magic resistance ratio increased to 40% **bonus** magic resistance from 35%.
- *Path Maker*
  - Stun duration increased to type=channel time seconds from type=channel time.
- *Footwork*
  - Base shield changed to $80-240$ from $50-250$.
  - Shield health ratio increased to 15% **bonus** health from 10%.

### V14.19
- Stats
  - Attack range reduced to 150 from 175.
- *Dauntless Instinct*
  - Base damage increased to 20 at all levels from 5 to 20.
  - Health ratio changed to 1 to 2 of target's **maximum** health from 1@1; 1.33@6; 1.66@11; 2@16 (@=%).
  - **Removed:*** No longer grants range**bonus** attack range* against a marked target.
  - **Removed:*** Damage is no longer converted to true damage while *All Out*.
  - **Removed:*** Damage is no longer increased by 30 to 78 while *All Out*.
  - **New Effect:** While *All Out*, ''K'Sante's** basic attacks and abilities now deal **bonus** physical damage equal to 1% .
- *Ntofo Strikes*
  - Width reduced to 100 units from 150.
  - Base damage increased to $70-190$ from $30-130$.
  - Damage armor ratio increased to 35% *bonus armor from 30%.
  - Damage magic resistance ratio increased to 35% **bonus** magic resistance from 30%.
  - **Removed:*** Damage no longer scales with 40% AD.
  - Cast time increased to 0.45 to 0.35 from 0.45 to 0.25.
  - **Removed:*** Cast time is no longer reduced by $0.08$ seconds while *All Out*.
  - Cooldown increased to 3.5 to 2 by 0.1 from 3.5 to 1.75.
  - *All Out* cooldown reduction increased to 33% from 25%.
  - **Removed:*** First cast no longer has a hitbox of 100 units around ''K'Sante''.
  - **Removed:*** No longer refreshes the cooldown of the ability upon entering *All Out*.
  - **New Effect:** Now slow enemies while *All Out*.
- *Path Maker*
  - Minimum charge time reduced to $0.4$ seconds from $0.66$.
  - Maximum range charge duration increased to $0.9$ seconds from $0.66$.
  - Base damage increased to $40-120$ from $20-100$.
  - Cooldown reduced to $14-10$ seconds from $24-16$.
  - **Removed:*** Damage no longer scales with 50% AD, 30% *bonus armor, and 30% **bonus** magic resistance.
  - Damage health ratio changed to 8% of target's **maximum** health at all ranks from $6-10$%.
  - **New Effect:** Health ratio now scales with 2% per 100 *bonus armor and 2% per 100 **bonus** magic resistance.
  - Stun duration changed to type=channel time seconds from $1.25$ at all times.
  - **New Effect:** Now deals key=% of 'Path Maker's damage as **bonus** true damage during *All Out*.
  - **Removed:*** ''K'Sante'' can no longer change the charging direction of the recast during the initial cast's channel.
    - Recast now always dashes in the direction he targeted at the time of the initial cast.
  - *All Out* damage reduction increased to 75% from 60%.
  - Monster damage cap changed to $180-500$ from 50 to 475.
- *Footwork*
  - Location dash speed reduced to 500 from 900.
    - *All Out* location dash speed reduced to 950 from 1450.
  - **New Effect:** Location dash speed, including while *All Out*, is now increased by 100% movement speed.
  - **New Effect:** While *All Out*, the base dash speed to allies is now increased to 1400.
  - Cooldown reduced to $10-8$ seconds from $10.5-8.5$.
  - **New Effect:** Cooldown is now reduced by 50% during *All Out*.
  - **Removed:*** Location dash can no longer cross terrain during *All Out*.
    - *All Out* ally dash unchanged.
  - **Removed:*** Location dash range is no longer increased to 400 while *All Out*.
  - **Removed:*** Cast no longer resets his basic attack timer.
  - **Bug Fixes:** Fixed an error on the cast animation.
- *All Out*
  - Damage type changed to physical from magic.
  - Base damage increased to $80-150 3$ from $70-150 3$.
    - Maximum base damage increased to $80×2-150×2 3$ from $70×2-150×2 3$.
  - **Removed:*** Damage no longer scales with 65% AP.
  - **New Effect:** Strike damage after pulling a target through a wall now scales with 5% of his **bonus** health.
  - Buff duration reduced to 15 seconds from 20.
  - **Removed:*** Can no longer be recast to end the effects of *All Out* early.
  - **Removed:*** No longer grants $10-40 3$ .
  - Bonus attack speed increased to $40-80 3$% from $25-45 3$%.
  - **Removed:*** No longer heals ''K'Sante'' for $15-25 3$% of damage dealt to champions for the duration.
  - **New Effect:** Now grants 20% omnivamp for the duration.
  - **New Effect:** Now grants lethality**bonus**-armor penetration* for the duration.
  - **Bug Fixes:** Now properly reduces the bonus resistances granted by *Jak'Sho, The Protean* Voidborn Resilience.

## Trivia

- This champion has no ability power ratio.
- K'Sante's Series 1 Eternals make the following references:
  - *Brawl Star* is a reference to the video game title of the Brawl Stars, known for being a battlefield of Brawlers against all players, K'Sante can involve being a Brawlers because he changes his role as Tank by casting his ultimate.

---
*This page was automatically generated from League of Legends Wiki data.*