# Senna

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
| **Champion** | Senna |
| **Title** | the Redeemer |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2019-11-10 |
| **Release Patch** | V9.22 |
| **Latest Changes** | V25.17 |
| **Roles** | Marksman, Enchanter |
| **Riot Positions** | Bottom, Support |
| **External Positions** | Bottom, Support |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Support |
| **Alt Type** | Marksman |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 3 |
| **Style** | 30 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $530.0$ | $+89.0$ |
| **Mana** | $350.0$ | $+45.0$ |
| **Health Regen** | $3.5$ | $+0.55$ |
| **Mana Regen** | $11.5$ | $+0.7$ |
| **Armor** | $25.0$ | $+4.0$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $50.0$ | $+0.0$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $600.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.4$ | |
| **Bonus AS per Level** | $2.6\%$ | |
| **Attack Windup** | $31.2\%$ | |
| **Windup Modifier** | $0.6$ | |
| **Missile Speed** | $0$ units/second | |
| **Acquisition Radius** | $800$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $115$ units | |
| **Selection Height** | $170$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $92.0\%$ |
| **Damage Taken** | $105.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $100.0\%$ |
| **Healing** | $120.0\%$ |

## Pets

### Mist Wraith

| Attribute | Value |
|-----------|------:|
| **Gold** | 8 |
| **Experience** | 0 |
| **Health** | 1 (Modified) |
| **Move Speed** | Static |
| **Control** | Autonomous |
| **Targeting** | Minion |

**Abilities:**

- **Wraith:** The *Mist Wraith* is considered an enemy unit, but targetable to **Senna** only with her basic attacks and abilities.
- **Wandering:** The *Mist Wraith*, after being spawned, disappears after 8 seconds.

---

## Abilities

### Passive: Absolution

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 2000 (Mist Wraith spawn area) units |
| **On-target CD Static** | 6@1; 5@6; 4@11 |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Spell Shield** | False |
| **Parry** | Special |

**INNATE - WEAKENED SOUL**: **Senna**’s basic attacks on-hit and abilities mark enemy champions hit for 4 seconds. Subsequent attacks or ability hits against marked targets will consume the mark to grant **Senna** a stack of Mist and deal **bonus** physical damage equal to 1 / 2 / 3 / 4 / 5 / 6 / 7 / 8 / 9 / 10 of target's **current** health, rendering the target immune to being marked again for a few seconds. This damage applies life steal at 100% effectiveness.

**INNATE:** Enemies that die near **Senna** may spawn a *Mist Wraith* from their corpse. A *Mist Wraith* lasts 8 seconds and **Senna** can basic attack or hit it with *Piercing Darkness* or *Dawning Shadow* to kill it instantly, granting her 8 gold and a stack of Mist.
- Enemy champions and large monsters will spawn a *Mist Wraith*.
- Epic monsters will spawn 2 *Mist Wraiths*.
- Minions and lesser monsters that **Senna** kills have a 10% chance to spawn a *Mist Wraith*.
- Large minions that **Senna** does not kill will always spawn a *Mist Wraith*, while lesser minions and lesser monsters that **Senna** does not kill have a 28% chance to spawn one.
- Lesser minions executed by an allied champion's Support Quest item have a $8.4$% chance to spawn a *Mist Wraith*.

**MIST**: For each stack of Mist, **Senna** gains attack damage. For every 20 stacks, she also gains attack range and critical strike chance. Additionally, every in excess of 100% is converted into $0.35$% life steal.

**INNATE - RELIC CANNON**: **Senna**’s basic attacks on-hit deal 20% AD **bonus** physical damage and grant her key=% of the target's movement speed as ms for $0.5$ seconds. This damage applies life steal at 100% effectiveness.

**Notes:**

- If a target's mark has a remaining duration less than $0.25$ seconds, and **Senna** starts her basic attack windup on the target during this time, the mark's duration will be modified to $0.75$ seconds. Subsequent windups on the marked target will refresh this modified duration if the previous attack windup did not complete.
- Despite **Senna** dealing 1 damage to *Mist Wraiths*, attacking them will calculate the attack's damage (including critical strike modifiers) and any on-hit effects for the purposes of life steal and drain effects.
- *Relic Cannon* is only applied if the attack deals more than 0 damage.
  - Hence, it is not applied if the target is invulnerable or **Senna**’s basic attack's attack damage is reduced to 0 or below.
- Dealing 0 damage is valid for marking and collecting Mist from champions, but dealing no damage at all is not.
  - Hence, **Senna** can mark and collect from invulnerable enemies.
  - Dodge prevents marking and collecting a Mist via **Senna**’s basic attacks and Q.
  - : Mist interaction with block and blind.
- Nunu spawn a wraith each.
- Enemies will not see newly spawned wraiths while **Senna** is not visible.
- A wraith can be hit by Cosmic Binding and it will interact with it the same way minions or monsters interact with Cosmic Binding but it will not take damage.
- *Mist Wraith* has a spawn animation, but is targetable immediately.
- *Mist Wraith* grants a small amount of sight around itself.
- Runaan's Hurricane secondary bolts will ignore *Mist Wraiths*.
- Attacking a *Mist Wraith* will not consume Energized.
- Excluding (Due to lack of available metric) the range increase and the bonus shielding on Dawning Shadow, one stack of Mist is worth (+8 gold from *Mist Wraith* pickup ( total)), if the critical chance is converted to life steal.
  - $0.75 AD$ is worth .
  - critical chance critical strike chance is worth .
    - life steal life steal is worth $.
- The bonus on-hit damage applies an additional stack of Black Cleaver Carve to offset **Senna**’s low attack speed. This interaction is inconsistent.
- Critical strike chance gained from Yun Tal Wildarrows Practice Makes Lethal does not interact with the life steal conversion.

---

### Q: Piercing Darkness

| Attribute | Value |
|-----------|------:|
| **Range** | 1300 units |
| **Cast Time** | 80% of **Senna**’s windup time ( at **base** attack speed) |
| **Target Range** | 600 - 1100 (Based on attack range) units |
| **Width** | 100 (Damage) / 280 (Heal) units |
| **Cost** | 70 / 80 / 90 / 100 / 110 Mana |
| **Cooldown** | 15 seconds |
| **Targeting** | Unit |
| **Affects** | Allies, Enemies, Turrets, Wards |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | AoE |
| **Projectile** | False |
| **Parry** | Unknown |

**ACTIVE:** **Senna** fires a spectral laser in the direction of the target unit, healing herself and allied champions hit, and dealing physical damage to enemies hit in a thinner line. Enemies hit are slowed by 15% (+ 15% per 100 **bonus** AD) (+ 10% per 100 AP) for a duration. The damage dealt to enemy champions applies life steal at 100% effectiveness.

| Attribute | Value |
|-----------|------:|
| **Healing** | 40 / 60 / 80 / 100 / 120 (+ 40% **bonus** AD) (+ 50% AP) |

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 30 / 60 / 90 / 120 / 150 (+ 40% **bonus** AD) |

| Attribute | Value |
|-----------|------:|
| **Slow Duration** | 1 / 1.25 / 1.5 / 1.75 / 2 seconds |

Upon being hit, enemy turrets, wards, jungle plants, Mist Wraiths and any other attack-interactive units are *considered* , while champions and turrets* (Considers all turret-specific rules) are applied on-hit effects.

*Piercing Darkness' * **current** cooldown is reduced by 1 second on-hit.

**Notes:**

- The ability's cast time respects the attack speed floor and cap.
- *Piercing Darkness* also applies most on-attack effects to one enemy champion struck, and ocassionally to two.
  - Hail of Blades will spend a stack on hitting a champion with *Piercing Darkness* but only if Hail of Blades was activated first by an auto-attack.
- **Senna** can generally target anything that is not either explicitly untargetable or herself.
  - Valid targets include anything that can be targeted by a basic attack from someone.
- *Piercing Darkness* will attempt to lead the target if it is moving but does not adjust further during cast time (enemies can dodge the laser if they change their position by a sufficient amount during the cast time).
  - It leads by (up-to?) 80 units.
- On-hit effects are applied in order of struck units' spawn IDs. This matters for effects that are consumed on-hit (e.g. Spellblade), which are applied to the target with the lowest Spawn ID (among enemy champions, this would also be the enemy first pick in Blind Pick mode).
  - "Spawn ID" is an unofficial abbreviation to describe the spawn order for all units at the beginning of games, below are some examples.
  - If **Senna** targets an enemy champion but hits at least one more enemy champion simultaneously, Press the Attack’s first stack will be applied to that target. If no champion was targeted, it will default to the champion with the lowest Spawn ID out of the ones struck by the ability. Whether the effect stacks up on a champion or not when multiple enemy champions are stuck also depends on Spawn ID.
  - If *Piercing Darkness* struck an enemy champion and turret simultaneously, with Rapid Firecannon fully charged, the passive effect will most likely proc on the turret since towers are often spawned before champions.
- For on-attack effects, they are also applied in order of struck units' spawn IDs. However, there are some specific on-attack effects that will apply to both the target of *Piercing Darkness* and the target with the lowest Spawn ID (among enemy champions, this would also be the enemy first pick in Blind Pick mode) if the two unit is not identical. Confirmed cases (all under condition that the targeted enemy champion is not having lowest Spawn ID among all struck enemy champions): **Energized effects are stacked 2 stacks **Spectral Waltz are granted 2 stacks **
- Spell shield will not prevent on-hit effects from being applied.
- : *Piercing Darkness* interaction with dodging, blocking, and blinding effects.

---

### W: Last Embrace

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1300 units |
| **Effect Radius** | 280 units |
| **Width** | 140 units |
| **Speed** | 1200 units/second |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 11 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**ACTIVE:** **Senna** throws a globule of Black Mist in the target direction that deals physical damage to the first enemy hit and attaches to them for 1 second.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 70 / 115 / 160 / 205 / 250 (+ 70% **bonus** AD) |

After the duration or when the target dies, the Black Mist spreads out of the target, rooting them and surrounding enemies for a few seconds.

| Attribute | Value |
|-----------|------:|
| **Root Duration** | 1.25 / 1.5 / 1.75 / 2 / 2.25 seconds |

**Notes:**

- *Last Embrace* will not root the primary target if they are untargetable. Effect at cast time end

---

### E: Curse of the Black Mist

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 1 seconds |
| **Effect Radius** | 400 units |
| **Cost** | 70 Mana |
| **Cooldown** | 26 / 24.5 / 23 / 21.5 / 20 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Auto |
| **Affects** | Self |
| **Detection Radius** | 400 |

**ACTIVE:** **Senna** surrounds herself with an aura of mist and gains camouflage for a duration. If **Senna** breaks the camouflage, she regains it after $1.75$ seconds without performing actions that break stealth.

| Attribute | Value |
|-----------|------:|
| **Effect Duration** | 6 / 6.5 / 7 / 7.5 / 8 seconds |

Allied champions who enter the mist gain *Wraith Form* for a duration equal to the aura's remaining duration.

**WRAITH FORM:** Gain camouflage. Enemies will have obscured vision of camouflaged *Wraiths* moving outside of the mist. *Wraith Form* can be regained if lost by breaking the camouflage after $1.75$ seconds without performing actions that break stealth. *Wraith Form* ends immediately upon **Senna**’s death.

**Senna** and allies camouflaged by *Curse of the Black Mist* also gain ms (+ 5% per 100 AP) **bonus** movement speed.

**Notes:**

- **Senna** can move while casting *Curse of the Black Mist*.
- Allies affected by the *Curse of the Blast Mist* aura receive a buff called *Cloak of Mist*.
- Obscured units appear as *Wraiths*, hence the name. The wraith model does not distinguish which champion is obscured.
  - On the mini-map.md), obscured champions will be displayed as wraith icons instead of champion icons to the enemy team.
- In-game, the 'Cloak of Mist' buff grants camouflage and the 'Wraith Form' buff obscures the unit and grants movement speed. However, 'Cloak of Mist' has the unspecified effect of negating the obscuring effects of 'Wraith Form' (i.e. the unit is not a wraith while inside the mist); the unit has the bonus movement speed even when they are not a wraith (e.g. inside the mist or an enemy is nearby); and 'Wraith Form' also grants the functionality of camouflage (i.e. the unit is treated as a camouflaged unit with regards to True Sight, Control Ward, and enemy champions).
  - Given this overly complex interaction between the two buffs granted as well as the overlapping effects, it is easier to describe the effects as a single buff that is modified in specific circumstances rather than trying to establish a new game mechanic.
  - **Senna** likewise has her aura buff and the 'Wraith Form' buff, but her 'Wraith Form' does not share gameplay functionality with other allies benefiting from Wraith Form. For **Senna**, the appearance of being a wraith is a form-swap while camouflaged (similar to Demon Shade). **Senna** will remain in her wraith form when she is detected so long as the camouflage effect isn't broken, and her wraith's appearance is very distinct from other wraiths.
- Allied champions who enter the mist and have stealth of their own will still gain the *Wraith Form* buff but they will not grant obscured vision of themselves to the enemy while out of the mist nor will they appear as a *Wraith*.
  - Allies who have their own camouflage and also have *Wraith Form* will be revealed to enemies from their stealth's detection range, not *Wraith Form*’s.
- If **Senna** gains invisibility while camouflaged by *Curse of the Black Mist*, the visual effects of her aura will be hidden while she is invisible.
- *Curse of the Black Mist* will not activate if **Senna** enters resurrection during the cast time.
- *Wraith Form* will not be granted to allies that are in a zombie state or are channeling. It will be granted to allies even if they are untargetable.
- Using a basic attack breaks the stealth at the start of the attack windup.
  - The stealth regain timer is refreshed from basic attacks at the start of their attack windup.

---

### R: Dawning Shadow

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 1 seconds |
| **Target Range** | Global |
| **Width** | 320 (Central beam) / 2400 (Wide beam) units |
| **Speed** | 20000 units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 140 / 130 / 120 / 110 / 100 seconds |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Direction |
| **Affects** | Allies / Enemies |
| **Damage Type** | physical |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | True |

**ACTIVE:** **Senna** fires two waves of spectral light in the target direction, granting sight of the area briefly along the path. The center wave deals physical damage to enemy champions hit and reveals them for 3 seconds, as well as hits all *Mist Wraiths*.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 250 / 325 / 400 / 475 / 550 (+ 115% **bonus** AD) (+ 70% AP) |

The broad wave grants a shield to **Senna** and allied champions hit for 3 seconds.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 120 / 140 / 160 / 180 / 200 (+ 50% AP)
(+ $1.5$ per Mist collected) |

**Notes:**

- **Senna** reveals herself during the cast time.

---

### Basic Attack

| Attribute | Value |
|-----------|------:|
| **Attack Range** | er Senna |
| **Speed** | N/A (Non-projectile) |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | Attack |
| **Projectile** | False |
| **Parry** | True |

**BASIC ATTACK:** **Senna** blasts the target with her relic cannon that uses a non-projectile beam, dealing 100% AD physical damage, applying on-hit effects, and triggering on-attack effects.

**Senna** has an attack ratio of $, meaning she only gains attack speed per 100% **bonus** attack speed.|% of the attack speed she would gain if her attack speed ratio was the same as her base attack speed.

Additionally, **Senna**’s base attack windup is reduced,|by 100-(0.5 to 0.35)/0.5×100,but is only reduced by % of the expected value given attack speed bonuses.

**Notes:**

- At level 18, only with the per level **bonus** attack speed, **Senna** will have $ attack speed, and will take to perform an attack compared to at level 1.

---

## Patch History

### V25.17#August 27th Hotfix|V25.17
- Stats
  - Attack speed growth reduced to $2.6$% from 3%.

### V25.17
- Stats
  - Attack speed growth increased to 3% from 2%.
- Piercing Darkness
  - Base heal increased to 40 / 60 / 80 / 100 / 120 from 40 / 55 / 70 / 85 / 100.

### V25.12
- Absolution
  - *Mist Wraith* spawn rate on minions she kills reduced to 10% from 14%.
  - **Bug Fixes:** Attacking *Mist Wraiths* no longer incorrectly consumes/activates available attack effects.

### V25.10
- Absolution
  - Critical strike chance per 20 stacks increased to 10% from 8%.
  - *Mist Wraith* spawn rate on minions she kills increased to 14% from $8.4$%.
- Piercing Darkness
  - Heal AP ratio reduced to 50% AP from 60% AP.

### V25.05
- Absolution
  - **Bug Fixes:** No longer sometimes fails to grant the guaranteed Mist Wraith from Blue Siege Minion when she kills it via a Support item's (World AtlasRunic Compass) Shared Riches charge.

### V25.S1.1
- Senna
  - **Bug Fixes:** Chromas now use the correct assets during the Homeguard animation and no longer fall back to Senna's default skin's assets.

### V14.22#November 6th Hotfix|V14.22
- Absolution
  - **Bug Fixes:** Collected *Mist Wraiths* no longer turn into Zombie Wards.

### V14.21
- Stats
  - Base armor reduced to 25 from 28.
  - Armor growth reduced to 4 from $4.7$.

### V14.19#September 25th Hotfix|V14.19
- General
  - **UNDOCUMENTED / BUG FIX:** Senna existing in the game no longer causes turrets to become invisible.

### V14.17
- Piercing Darkness
  - Base heal reduced to 40 / 55 / 70 / 85 / 100 from 40 / 60 / 80 / 100 / 120.
  - Heal AP ratio reduced to 60% AP from 80% AP.
- Dawning Shadow
  - Shield AP ratio reduced to 50% AP from 70% AP.

## Trivia

- 
  - In Senna's case, Absolution infinitely stacks her attack damage, range, lifesteal and the shield of Dawning Shadow.
- Senna is an example of a champion that was present in the lore before they were developed into a playable champion: appearing as a recurring character in Lucian’s story since his release. Another example is Lissandra, who appeared in the Journal of Justice.
  - She is also part of the alternative universes of High Noon and PROJECT without having any belonging skins before her release.
- Senna is the second champion that cannot increase a stat through leveling up, after Thresh and his armor. In her case, it is attack damage because of her passive, Absolution.
  - Bel'Veth and Briar were the next champions to be released who cannot increase a stat through leveling up. These stats are Bel'Veth's attack speed and Briar's health regeneration.
- Senna (optionally with Lucian) has a Nemesis Quest against Thresh.
  - Senna's (and Lucian's) reward is called *Purity… peace*: *"Thresh has finally been allowed to pass. This unit absorbed some of his power […]."*
- Senna has among one of the longest attack frames in game, at $0.5$ seconds, while most champions are between $0.2$ and $0.3$ seconds.
- Her name comes from Arabic سناء *Sanāʼ* "brightness, sublimity", from root *s-n-y* (> Sin (mythology) "moon" & *seneh* "burning bush").

---
*This page was automatically generated from League of Legends Wiki data.*