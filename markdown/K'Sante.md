# K'Sante

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
| **Champion** | K'Sante |
| **Title** | the Pride of Nazumah |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2022-11-02 |
| **Release Patch** | V12.21 |
| **Latest Changes** | V25.18 |
| **Roles** | Warden, Skirmisher |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Tank |
| **Alt Type** | Fighter |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 2 |
| **Utility** | 2 |
| **Style** | 30 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $625.0$ | $+120.0$ |
| **Mana** | $320.0$ | $+60.0$ |
| **Health Regen** | $9.5$ | $+1.0$ |
| **Mana Regen** | $7.0$ | $+1.0$ |
| **Armor** | $36.0$ | $+5.2$ |
| **Magic Resist** | $30.0$ | $+2.1$ |
| **Attack Damage** | $64.0$ | $+3.5$ |
| **Attack Speed** | $0.688$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $150.0$ | $+0.0$ |
| **Base Attack Speed** | $0.688$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $50$ units | |
| **Selection Radius** | $140$ units | |
| **Selection Height** | $225$ units | |
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
| **Damage Taken** | $100.0\%$ |

## Abilities

### Passive: Dauntless Instinct

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Shield** | Special |
| **Spell Effects** | proc |
| **Parry** | Special |

**INNATE:** **K'Sante**’s abilities mark enemies hit for 4 seconds. His basic attacks against marked enemies are empowered to consume the mark on-hit, dealing 12 (+ 1 to 2 of target's **maximum** health) **bonus** physical damage. The damage based on the target's health has a minimum of 15 / 100 against minions and is capped at 20 / 105 against monsters.

**ALL OUT BONUS:** **K'Sante**’s basic attacks and ability damage, as well as *Dauntless Instinct*’s mark consumption, are empowered to deal **bonus** physical damage equal to 1% (+ 1% per 100 **bonus** armor) (+ 1% per 100 **bonus** magic resistance) of the target's **maximum** health.

**Notes:**

- Spell shield will block the mark's application, but not the mark's consumption from an attack.
- The empowered attack can be dodged, blocked, and missed while blinded but the mark will not be consumed.
- Starting the empowered attack while the mark is about to time out will not extend the mark duration, but still complete the attack and apply the bonus damage.
- All Out's bonus damage on abilities only applies to the first target hit when damaging multiple enemies (including minions and monsters).
  - For most of **K'Sante**’s spells and item effects, in the case of multiple targets being stuck on the same game tick, this refers to the closest target. For non-missile Ntofo Strikes, it instead depends on the order in which the target units were spawned into the match.
- All Out's bonus damage on abilities is applied in the same damage instance from **K'Sante**’s basic attacks and abilities.
  - This causes effects like Bone Plating and Black Cleaver Carve to be triggered only once from the respective hit.

---

### Q: Ntofo Strikes

| Attribute | Value |
|-----------|------:|
| **Range** | 465 (Q1 rectangle length, starts at K'Sante's center. Backwards also edge range from 0.) / 930 (Q3 missile maximum range) units |
| **Cast Time** | 0.45 to 0.35 seconds |
| **Width** | 100 (Q1 rectangle width) / 100 (Q3 missile width) units |
| **Cost** | 20 Mana |
| **Static Cooldown** | 3.5 to 2 by 0.1 |
| **Queue Time** | $0.25$ (All casts) seconds |
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**ACTIVE:** **K'Sante** slams his ntofo down in the target direction that deals physical damage to enemies hit and slows them by 80% for $0.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 70 / 100 / 130 / 160 / 190 (+ 40% **bonus** armor) (+ 40% **bonus** magic resistance) |

If this hits at least one enemy, **K'Sante** generates a stack for 6 seconds, stacking up to 2 times and refreshing on subsequent hits. At 2 stacks, the next *Ntofo Strikes* cast consumes them all to become empowered with a new effect.

**EMPOWERED ACTIVE:** **K'Sante** fires a shockwave in the target direction that applies the same effects to enemies hit, but also pulls them towards him over $0.65$ seconds and stuns them for $0.8$ seconds.

**ALL OUT BONUS:** *Ntofo Strikes' * cooldown is reduced by 33%, with a minimum total cooldown of $1.33$ seconds. Upon entering *All Out*, *Ntofo Strikes' * stacks are reset. *Ntofo Strikes' * cooldown is reset if **K'Sante** had 2 stacks before entering *All Out*.

*Ntofo Strikes resets **K'Sante**’s basic attack timer.*

**Notes:**

- This ability will cast from wherever the caster is at the end of the cast time.
- If *Ntofo Strikes* hits no more than one enemy champion, **K'Sante** will be ordered an attack command against them.
- The basic attack reset is not considered one for Hail of Blades.
- Temporary increases/decreases in bonus resistances will count for reducing/increasing *Ntofo Strikes** cooldown.
- Only the empowered active is a projectile.
- The following table refers for interactions while **K'Sante** is in the cast time of the third cast:

| Aspect | State / Notes |
|--------|---------------|
| **Type** | Cast |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite; Disabled: Flash, Teleport, Recall, Hexflash |

---

### W: Path Maker

| Attribute | Value |
|-----------|------:|
| **Range** | 450 (Maximum dash range, pending for test) units |
| **Cast Time** | none |
| **Speed** | 1300 (Knock back speed) / 1500 (Dash speed) / 1800 (Dash speed during All Out) units/second |
| **Cost** | 40 / 45 / 50 / 55 / 60 Mana |
| **Cooldown** | 14 / 13 / 12 / 11 / 10 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.25$ (Both for the regular and All Out version) seconds |
| **Targeting** | Direction / Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Grounded** | Special |
| **Knockdown** | True |
| **Silence** | False |

**ACTIVE:** **K'Sante** raises his ntofos defensively and prepares to dash in the target direction, charging for a minimum of $0.4$ seconds and up to 1 second. During this time, he gains displacement immunity and 30% damage reduction; additionally, *Path Maker*’s range, stun duration, and *All Out* **bonus** true damage modifier increase over the first $0.9$ seconds of the channel.

*Path Maker* can be recast within the duration, and does so automatically afterwards. *Path Maker*’s charge cannot be interrupted by crowd control.

**RECAST:** **K'Sante** dashes in the direction he targeted at the time of cast, though not through terrain, dealing physical damage to enemies he passes through, carrying them alongside him, and stunning them for 0.5 / 0.62 / 0.75 / 0.88 / 1 / 1.12 / 1.25 / 1.38 / 1.5 / 1.62 / 1.75 seconds. This damage is capped against monsters.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 45 / 75 / 105 / 135 / 165 (+ 8% (+ 2% per 100 **bonus** armor) (+ 2% per 100 **bonus** magic resistance) of target's **maximum** health) || Attribute | Value |
|-----------|------:|
| **Monster Damage Cap** | 180 / 260 / 340 / 420 / 500 |

**ALL OUT BONUS:** *Path Maker* no longer applies its knock back and stun, but its damage reduction is increased to 75%, the dash deals (true damage) 10 to 80 **bonus** true damage, and the dash speed is increased by 20%. Upon entering *All Out*, *Path Maker*’s cooldown is refreshed.

| Attribute | Value |
|-----------|------:|
| **Minimum Bonus True Damage** | 4.5 / 7.5 / 10.5 / 13.5 / 16.5 (+ 0.8% (+ 0.2% per 100 **bonus** armor) (+ 0.2% per 100 **bonus** magic resistance) of target's **maximum** health) |
| **Maximum Bonus True Damage** | 36 / 60 / 84 / 108 / 132 (+ 6.4% (+ 1.6% per 100 **bonus** armor) (+ 1.6% per 100 **bonus** magic resistance) of target's **maximum** health) |
| **Total Maximum Mixed Damage** | 81 / 135 / 189 / 243 / 297 (+ 14.4% (+ 3.6% per 100 **bonus** armor) (+ 3.6% per 100 **bonus** magic resistance) of target's **maximum** health) |

*Ntofo Strikes and All Out can be cast during the dash. Path Maker's recast can be used while affected by cast-inhibiting crowd control.*

**Scaling:**
| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 45 / 75 / 105 / 135 / 165 (+ 8% (+ 2% per 100 **bonus** armor) (+ 2% per 100 **bonus** magic resistance) of target's **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Monster Damage Cap** | 180 / 260 / 340 / 420 / 500 |

**Notes:**

- All Out’s duration is increased by 2 seconds if it were to expire during *Path Maker*’s charge.
- *Path Maker* cancels any existing movement or attack commands. However, new inputs given during the charge will be retained once the dash ends, except when the dash hits only one enemy champion. If *Path Maker* hits only one enemy champion **K'Sante** will be ordered to basic attack them afterwards.
- **K'Sante** cannot use Flash during *Path Maker*’s dash.
- *Path Maker*’s damage is not capped against monsters while empowered by *All Out*.
- *Path Maker*’s recast can be used even while grounded or rooted, but the initial cast cannot.
- *Path Maker* can be recast by issuing an attack move command (or **LMB**) but not a targeted attack command.
- The following table refers for interactions while **K'Sante** is channeling:
  - If the charge is cancelled, he will not automatically use the recast.

| Aspect | State / Notes |
|--------|---------------|
| **Type** | Charge channel |
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Disabled: All items |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite; Disabled: Flash, Teleport, Recall, Hexflash |
| **Consumables** | Disabled |
| **Interrupted by** | Death |
| **Notes** | but allows trinkets |

---

### E: Footwork

| Attribute | Value |
|-----------|------:|
| **Range** | 250 (Location-targeted maximum dash range) / 550 (Ally-targeted dash range) units |
| **Cast Time** | none |
| **Speed** | (500 (Dash speed to location) / 1250 (Dash speed to location during All Out) / 1100 (Dash speed to allies) / 1400 (Dash speed to allies during All Out)) + 100% movement speed |
| **Cost** | 45 / 50 / 55 / 60 / 65 Mana |
| **Cooldown** | 10 / 9.5 / 9 / 8.5 / 8 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Location / Unit |
| **Affects** | Self, Allies |
| **Out of Range Behavior** | If targeting a location, target at maximum range (clamped)

If targeting an ally, walk in range of the target unit to cast |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **K'Sante** dashes to the target location, though not through terrain, and grants himself a shield for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 80 / 120 / 160 / 200 / 240 (+ 15% **bonus** health) |

*Footwork* can also be cast on allies with increased range and speed, and the ability to dash through terrain. If the target ally is a champion, they receive the shield as well.

**ALL OUT BONUS:** *Footwork*’s dash speed is increased, and its cooldown is reduced by 50%.

***K'Sante** can cast any of his abilities during the dash.*

**Notes:**

- *Footwork*’s cast on allies has a forgiveness radius of 150 units.
- Dashing to the or behind will cancel current move or attack orders. Exception when attack order was given at melee range from target.
- The shield is granted instantly on cast for both **K'Sante** and the potential allied champion.

---

### R: All Out

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.4$ seconds |
| **Target Range** | 300 units |
| **Effect Radius** | 350 (Range check for pull being eligible for displacing through terrain) / 5000 (Maximum pull radius over terrain on Summoner's Rift) / Global (Pull radius over terrain on non-Summoner's Rift maps) |
| **Speed** | 2000 (Pull speed) units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Grounded** | True |

**ACTIVE:** **K'Sante** roots the target enemy champion for $0.5$ seconds (Duration unaffected by tenacity) and gains displacement immunity over the cast time. He then shatters his ntofos, pulling the target to a location that is 300 units in the cast direction from their location at the time of cast, during which they are revealed, and blinking 175 units (Beyond the target's hitbox's edge) behind that location. The target is dealt physical damage near the end of the displacement and is stunned for $0.3$ seconds once it ends.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 80 / 97.5 / 115 / 132.5 / 150 |

If there is valid map-generated terrain (Terrain that has an opposite (non-border) and is not player-generated.) 350 units in the cast direction from the target's location at the time of cast, the target is instead pulled to a location that is directly 450 units away from the edge of the other of the terrain and **K'Sante** blinks 100 units behind that location, dealing them the physical damage near their emergence from the terrain. At the end of the displacement, they remain airborne for , after which they are stunned for $0.5$ seconds. **K'Sante** strikes them after into the airborne duration to deal physical damage.

| Attribute | Value |
|-----------|------:|
| **Strike Physical Damage** | 80 / 97.5 / 115 / 132.5 / 150 (+ 5% of his **bonus** health) |

| Attribute | Value |
|-----------|------:|
| **Total Physical Damage** | 160 / 195 / 230 / 265 / 300 (+ 5% of his **bonus** health) |

After **K'Sante** blinks, he enters *All Out* for 15 seconds.

**ALL OUT:** **K'Sante** gains a health threshold equal to 65% **maximum** health which cannot be modified (By increases or decreases to maximum health) nor exceeded (By healing and health regeneration) by any means. Upon entering *All Out*, his **current** health is reduced to this threshold if it is above it. Additionally, his (armor) **base** armor and (mr) **base** magic resistance are reduced by 85% **bonus** armor and 85% **bonus** magic resistance, respectively. In return, he gains **bonus** attack speed, (lethality) 50% **bonus**-armor penetration (Affects bonus armor only), and 20% omnivamp, and modifies his basic abilities which can be cast at no cost.

Upon entering *All Out*, **K'Sante** is restored to 100% **maximum** mana. His mana then decays to 0 over the duration of the buff, after which the amount of he had prior to entering *All Out* (After paying All Out's mana cost) is restored.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 40 / 50 / 60 / 70 / 80% |

***K'Sante** retains his **maximum** health and **bonus** resistances during All Out. Health lost from gaining the health threshold is **not** restored after the effect ends. If the target is pulled over terrain that is longer than 2000 (Estimated) units, **K'Sante** will vanish during their displacement and reappear near the end of it.*

**Notes:**

- *All Out*’s cast has a forgiveness radius of er 50 units.
- If the terrain behind the target is invalid by not having an opposite side, *All Out* will not trigger its additional effects from pulling the target over terrain.
  - The pull destination in this case would be inside terrain and thus the displacement would stop at the boundary.
- **K'Sante** will restore the exact amount of mana he had before going *All Out* after the buff expires.
  - The amount does not change even if his **maximum** mana changes during the effect.
- While the target is being pulled over terrain, they are additionally attached to a missile that travels in the same trajectory as the displacement, becoming detached afterwards.
  - Removing the airborne debuff will not cause the target to detach from the missile.
- If the target resists the pull by having a spell shield or being immune to crowd control or displacement immune, **K'Sante** will still blink beyond the pull's end location as if the target were displaced.
- **K'Sante**’s strike at the end of the displacement when the target is pulled over terrain does not apply Dauntless Instinct’s mark.
- Going *All Out* does not override **K'Sante**’s attack or movement commands if the target does not collide with terrain.
  - If the target is however pulled through terrain then **K'Sante** will be ordered to basic attack the target afterwards.
- The percentage bonus armor penetration stacks additively with other sources of percentage armor penetration.
- Since **K'Sante** retains the stats he "lost", any effects that scale based on those stats will calculate based on the value prior to him going *All Out*.
  - His **base** resistances can be reduced below 0 by his **bonus** resistances.
- *All Out* will not take into account resistances gained through % bonuses (eg. Conditioning, Mountainous Vigor, etc) when reducing **K'Sante's base** resistances.
  - Jak'Sho, The Protean Voidborn Resilience specifically is taken into account.
- Gaining **bonus** health during *All Out* causes the health threshold to be briefly increased then reset back to its original value.
  - The **bonus** health is still applied to his **current** health.
- During *All Out*’s cast and displacement, the target's camera is locked and centered on their champion.
  - For **K'Sante**, his camera centers on him at the start of the cast time, then once again at the end of the cast time at the location of his blink behind the target, and it is not locked.
- *All Out*’s granted untargetability from the vanish during the target's pull of over 2000 units does not destroy in-flight projectiles.
- The following table refers for interactions while **K'Sante** is in cast time:

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Usable: Shurelya's Battlesong, Youmuu's Ghostblade, Randuin's Omen; Other items: Disabled |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite; Disabled: Flash, Teleport, Recall, Hexflash |
| **Consumables** | Usable |
| **Interrupted by** | Death (unless protected by Resurrection) |
- The following table refers for interactions while **K'Sante** is performing *All Out*’s cast:

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Disabled |
| **Items** | Disabled: All items |
| **Summoner Spells** | Usable: Barrier, Clarity, Cleanse, Exhaust, Ghost, Heal, Ignite, Smite; Disabled: Flash, Teleport, Recall, Hexflash |
| **Consumables** | Disabled |
| **Interrupted by** | Death |

---

## Patch History

### V25.18
- Dauntless Instinct
  - **Bug Fixes:** Tooltip now notes the damage floor and cap based on minions' and monsters' health ratio, respectively.
    - The correct *current* numbers are noted, but the tooltip is not set up to automatically update if they change in the future.

### V25.11
- All Out
  - **Bug Fixes:** While he is All Out and Aftershock is equipped, no longer incorrectly receives excessive damage from a champion who has gained the effects of Last Breath’s bonus-armor penetration.

### V25.09
- General
  - Updated recommended items. No longer incorrectly displays various Fighter items instead of Tank items.
- Path Maker
  - Base damage increased to 45 / 75 / 105 / 135 / 165 from 40 / 60 / 80 / 100 / 120.

### V25.06
- Path Maker
  - **Bug Fixes:** No longer fails to apply its effects when buffering it to cast shortly before a ground debuff ends.
  - **Bug Fixes:** After buffering a Footwork + Ntofo Strikes + Path Maker combo, no longer causes Path Maker to prematurely cancel shortly after casting.

### V25.05
- Dauntless Instinct
  - Mark base damage reduced to 12 from 20.
- Path Maker
  - All Out bonus true damage reduced to 10 to 80 from 10 to 100.

### V25.04
- Ntofo Strikes
  - **Bug Fixes:** Corrected SFX origin point.

### V14.23
- Footwork
  - **Bug Fixes:** Buffering *Footwork*’s cast on an ally target no longer improperly triggers on-cast effects (e.g. activating Spellblade).

### V14.22
- Ntofo Strikes
  - Base damage reduced to 70 / 100 / 130 / 160 / 190 from 80 / 110 / 140 / 170 / 200.
- Footwork
  - Location dash speed while All Out increased to 1250 from 950.

### V14.21
- Footwork
  - **Bug Fixes:** Granting an ally Moonstone Renewer *Starlit Grace*’s shield via *Footwork* no longer allows the shield to last indefinitely.

### V14.19#September 26th Hotfix|V14.19
- Stats
  - **Undocumented:** Base attack speed increased to $0.688$ from $0.625$.
  - Base armor increased to 36 from 33.
- Ntofo Strikes
  - Base damage increased to 80 / 110 / 140 / 170 / 200 from 70 / 100 / 130 / 160 / 190.
  - Damage armor ratio increased to 40% **bonus** armor from 35%.
  - Damage magic resistance ratio increased to 40% **bonus** magic resistance from 35%.
- Path Maker
  - Stun duration increased to 0.5 / 0.62 / 0.75 / 0.88 / 1 / 1.12 / 1.25 / 1.38 / 1.5 / 1.62 / 1.75 seconds from 0.5 / 0.6 / 0.7 / 0.8 / 0.9 / 1 / 1.1 / 1.2 / 1.3 / 1.4 / 1.5.
- Footwork
  - Base shield changed to 80 / 120 / 160 / 200 / 240 from 50 / 100 / 150 / 200 / 250.
  - Shield health ratio increased to 15% **bonus** health from 10%.

## Trivia

- This champion has no ability power ratio.
- K'Sante's Series 1 Eternals make the following references:
  - *Brawl Star* is a reference to the video game title of the Brawl Stars, known for being a battlefield of Brawlers against all players, K'Sante can involve being a Brawlers because he changes his role as Tank by casting his ultimate.

---
*This page was automatically generated from League of Legends Wiki data.*