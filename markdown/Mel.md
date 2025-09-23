# Mel

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
| **Champion** | Mel |
| **Title** | the Soul's Reflection |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2025-01-23 |
| **Release Patch** | V25.02 |
| **Latest Changes** | V25.16 |
| **Roles** | Artillery |
| **Riot Positions** | Middle, Support |
| **External Positions** | Middle, Support |
| **Blue Essence** | 3150 |
| **Riot Points** | 975 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $630.0$ | $+93.0$ |
| **Mana** | $480.0$ | $+28.0$ |
| **Health Regen** | $6.0$ | $+0.55$ |
| **Mana Regen** | $9.0$ | $+0.9$ |
| **Armor** | $21.0$ | $+5.2$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $54.0$ | $+3.3$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.4$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Missile Speed** | $1600$ units/second | |
| **Acquisition Radius** | $625$ units | |
| **Gameplay Radius** | $65$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $85$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |

## Abilities

### Passive: Searing Brilliance

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | proc |
| **Projectile** | True |
| **Parry** | unknown |

**INNATE:** **Mel**’s damaging basic attacks and abilities apply a stack of *Overwhelm* to enemies for 5 seconds, refreshing on subsequent applications and stacking infinitely.

**OVERWHELM:** Store type=[File:Golden Eclipse.png (+ 10% AP) magic damage on the affected enemy with the first stack, reducedagainst minions. For each stack, store an additional type=[[File:Golden Eclipse.png (+ $0.75$% AP) magic damage on the target. If the total post-mitigation damage (Damage calculated after modifiers) stored exceeds the target's **current** health and shields, the next stack will consume them all to deal the damage.

**INNATE - SEARING BRILLIANCE:** **Mel**’s ability casts each generate 3 stacks of *Searing Brilliance* for 5 seconds, refreshing on subsequent casts and stacking up to 9 times. Her next basic attack consumes all stacks of *Searing Brilliance* to additionally fire an equal number of blazing projectiles at the target. Each projectile deals 8 to 25 (+ 1% AP) magic damage.

**Notes:**

- The first stack also benefits from the per-stack bonus stored damage.
- A threshold indicator for the currently stored damage is placed on a target's [health bar, as well as a mark above their head, while any *Overwhelm* stacks are active. These are visible to **Mel** and her target(s).
  - Whenever Golden Eclipse’s damage would put them below *Overwhelm*’s threshold, the mark will darken and glow more prominently.
- Just before *Overwhelm* stacks are consumed, targets have 5 additional stacks applied to them.
  - These stacks are consumed immediately.
- *Overwhelm* stacks will **not** be consumed against Undying Rage.

---

### Q: Radiant Volley

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 950 units |
| **Effect Radius** | 220 (Explosion radius) / sight 100 (Bolt sight radius) units |
| **Speed** | 4500 (Bolt speed) units/second |
| **Cost** | 70 / 80 / 90 / 100 / 110 Mana |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |
| **Spread (Between the two furthermost explosions)** | cr 60 |

**ACTIVE:** **Mel** launches a barrage of luminous bolts at the target location over $0.7$ seconds (Time between first and last explosion) which distribute evenly in the area. Each bolt explodes upon landing to deal magic damage to nearby enemies, reduced to 75% against minions.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Explosion** | 13 / 15.5 / 18 / 20.5 / 23 (+ $8.5$% AP) |
| **Minion Damage per Explosion** | 9.75 / 11.625 / 13.5 / 15.375 / 17.25 (+ 6.375% AP) |

| Attribute | Value |
|-----------|------:|
| **Number of Bolts** | 6 / 7 / 8 / 9 / 10 |

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 78 / 108.5 / 144 / 184.5 / 230 (+ 51 / 59.5 / 68 / 76.5 / 85% AP) |
| **Total Minion Damage** | 58.5 / 81.375 / 108 / 138.375 / 172.5 (+ 38.25 / 44.625 / 51 / 57.375 / 63.75% AP) |

The bolts momentarily grant sight during their travel, while the explosions grant sight over their target area. Both can see through brush and terrain.

**Notes:**

- Spell shield will only block a single explosion.
- For the given number of *Radiant Volley* bolts, the spread will result in the same individual areas being affected over the duration **invariably**.
  - Each individual missile is explicitly assigned an explosion location at a small offset from the cast location. This varies by missile, which results in their combined area of effect distributing over a total radius of [explosion radius + maximum spread] = 220 + 60 = 280 units.
    - This means that the distribution area is not random.
    - The order of bolts however, which will determine the order of individual areas to be affected per explosion, *is* random.
      - **Mel** launching the missiles in the same arcing motion for every *Radiant Volley* cast does not correlate with which missile is being fired at the time.
    - Note that the combined area of effect may not always be the maximum possible; this drawback diminishes with a higher number of bolts. Effect at cast time end

---

### W: Rebuttal

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 80 / 60 / 40 / 20 / 0 Mana |
| **Cooldown** | 35 / 32 / 29 / 26 / 23 seconds |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | Special |

**ACTIVE:** **Mel** forms a protective barrier around herself for $0.75$ seconds, becoming invulnerable to non-turret damage and gaining 30% decaying **bonus** movement speed. All hostile projectiles sourced from enemy champions that collide with the barrier will be destroyed instantly before **Mel** fires the same projectiles as her own, either toward or in the direction of the original sources, or toward herself.

Replicated projectiles have the same features (See notes) that the original ones do, but they retain a ratio of the damage that the original ones would deal, and all of their damage is converted to magic.

| Attribute | Value |
|-----------|------:|
| **Replicated Projectile Damage Modifier** | 40 / 45 / 50 / 55 / 60% (+ 5% per 100 AP) of the original damage |

**Notes:**

- *Rebuttal*’s barrier will also instantly destroy missiles that are already within its radius when the ability is cast.
- The replicated missile will mimic all of the original missile's properties, which includes its range, width, speed, acceleration, angle (if a cone), height, and sight radius.
  - Spell effects, as well as interactions with spell shield and projectile-interception effects, all vary based on the original missile that was replicated.
  - The only exception to this is the copied missile's path, or trajectory, which is based on the targeting paradigm that the original missile used when fired by its source. The replicated missile acquires the original missile source as the target.
    - If the original missile was fired by a unit or auto-targeted effect, the replicated missile will be fired as a homing missile with the target being the original missile source.
      - Healing projectiles, such as Starcall and Aria of Perseverance, will instead target **Mel**.
    - If the original missile was fired via any other targeting paradigm, the replicated missile will by default fire in the direction of the original missile source at the time of replication. This includes vector-targeted projectiles (currently only Hextech Ray), for which the origin will always be **Mel**’s center (since she cannot control the cast point).
- The replicated missile will mimic all of the functions of and copy the behavior of the original missile, unless otherwise stated. This includes every aspect and effect of the missile conforming to the attack or spell from which the missile originated.
  - The scaling of the effects of the copied missile is calculated using the original missile source's attributes (namely their stats). The damage of the missile only benefits from **Mel**’s damage modifiers and magic penetration, and not the original source's.
- The following missiles are destroyed, but not replicated:
  - Whirling Death on its way back to him.
  - Boomerang Throw on its way back to him.
  - Test of Spirit
  - Boomerang Blade on its way back to her.
  - Hijack
  - Bladecaller and Featherstorm
  - Stretching Strikes
- The following abilities will be stopped short of their trajectory without replication:
  - Emperor's Divide
  - Dark Sphere thrown with her Scatter the Weak
- **Mel** may occasionally fail to attribute Echoes of Helia damage projectile to herself.
  - This can cause the enemy to be able to kill their own allies, and gain all the kill's rewards.

---

### E: Solar Snare

| Attribute | Value |
|-----------|------:|
| **Range** | 1050 units |
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 80 (Orb radius) / 80 (Initial field radius (same as orb)) / 260 (Expanded field radius) units |
| **Speed** | 1000 units/second |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 12 / 11.5 / 11 / 10.5 / 10 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Sight radius** | 400 |

**ACTIVE:** **Mel** fires an orb in the target direction that grants sight of its surroundings (Can see through brush and terrain.) for $0.25$ seconds as it travels. Enemies hit by the orb are dealt magic damage and rooted for a duration.

| Attribute | Value |
|-----------|------:|
| **Orb Magic Damage** | 60 / 105 / 150 / 195 / 240 (+ 60% AP) |

| Attribute | Value |
|-----------|------:|
| **Orb Root Duration** | 1.25 / 1.5 / 1.75 / 2 / 2.25 seconds |

The orb emanates a field of solar radiation that additionally expands after a $0.5$-second delay. Enemies within the field are dealt magic damage and slowed by 30% every $0.125$ seconds, lingering for $0.75$ seconds after exiting.

| Attribute | Value |
|-----------|------:|
| **Field Magic Damage per Tick** | 2 / 4 / 6 / 8 / 10 (+ 1% AP) |
| **Field Magic Damage per Second** | 16 / 32 / 48 / 64 / 80 (+ 8% AP) |

Both the orb and the field of *Solar Snare* deal 50% damage against minions.

| Attribute | Value |
|-----------|------:|
| **Orb Minion Magic Damage** | 30 / 52.5 / 75 / 97.5 / 120 (+ 30% AP) |

| Attribute | Value |
|-----------|------:|
| **Field Minion Magic Damage per Tick** | 1 / 2 / 3 / 4 / 5 (+ 0.5% AP) |
| **Field Magic Damage per Second** | 8 / 16 / 24 / 32 / 40 (+ 4% AP) |

**Notes:**

- Spell shield will only block the orb (root portion).
- The initial impact of *Solar Snare* deals area damage, while the lingering damage over time deals persistent area damage.
- The slow debuff lasts for the same duration as its tick rate ($0.125$ seconds). Effect at cast time end

---

### R: Golden Eclipse

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.75$ seconds |
| **Target Range** | Global |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**PASSIVE:** *Overwhelm* stacks store more damage.

| Attribute | Value |
|-----------|------:|
| **Increased Stored Damage** | 10 / 15 / 20 / 25 / 30 |

| Attribute | Value |
|-----------|------:|
| **Stored Damage Increase per Stack** | 1 / 1.5 / 2 / 2.5 / 3 |

**ACTIVE:** **Mel** unleashes a radiant blast on all enemies affected by *Overwhelm*, dealing magic damage to each. Enemies are also revealed for 1 second from the start of the cast time.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 100 / 125 / 150 / 175 / 200 (+ 30% AP) (+ 4 / 5.5 / 7 / 8.5 / 10 (+ $3.5$% AP) per *Overwhelm* stack on the target) |

*An enemy champion affected by Overwhelm is required to cast this ability. The target does not have to be visible to be hit by this ability.*

**Notes:**

- *Golden Eclipse* can be cast even if all affected targets are untargetable.
- *Golden Eclipse* does not deal damage to enemies that are untargetable by the time the cast time completes.
- *Golden Eclipse* will not damage targets that aren't in the same realm as **Mel**.
  - *Golden Eclipse* can't be casted if the affected targets aren't in the same realm as **Mel**.
- Whenever *Golden Eclipse*’s damage would put a target below *Overwhelm’s* threshold, it will use a different icon.
- *Golden Eclipse* will display the number of total stacks affecting all enemy champions in the HUD.
- *Golden Eclipse* sets the duration of *Overwhelm* stacks to $0.75$ seconds if they would run out during the cast time.

---

## Patch History

### V25.16
- Rebuttal
  - **Bug Fixes:** Reflecting a damaging ability that subsequently last hits an Elemental Drake no longer unintentionally causes her own Ocean Dragon stack buff to duplicate and activate multiple times automatically for the remainder of the game, ignoring its normal timer.
  - **Bug Fixes:** Reflecting any damaging ability at any point no longer allows her own Ocean Dragon Soul buff to trigger multiple times, once for each damage instance received during the effect, ignoring its normal lockout.

### V25.15
- Rebuttal
  - **Bug Fixes:** After reflecting Fear Beyond Death’s missile and affecting the initial target with the effect, casting Golden Eclipse within Fear Beyond Death's mark's duration no longer causes **Mel** to fail to cast Golden Eclipse and repeatedly expend its mana over the remaining mark duration.
  - **Bug Fixes:** No longer fails to reflect Sleepy Trouble Bubble shortly after being hit by her basic attack or More Sparkles! attack.

### V25.14
- General
  - **Bug Fixes:** Restored VO for various events.

### V25.13
- Searing Brilliance
  - **Bug Fixes:** Damaging and killing an ally with any number of projectiles via an empowered attack while under the effects of Hostile Takeover no longer incorrectly credits **Mel** for the kill instead of Hostile Takeover's user.

### V25.10
- Searing Brilliance
  - **Bug Fixes:** No longer fails to apply Overwhelm stacks against Goes Where He Pleases while its immunity is active.

### V25.08
- Stats
  - Base health increased to 630 from 600.
- Searing Brilliance
  - Projectile base damage increased to 8 to 25 from 5 to 20.

### V25.05
- Rebuttal
  - **Bug Fixes:** No longer fails to destroy projectiles if the barrier is hit from certain angles.
- Solar Snare
  - **Bug Fixes:** DoT debuff no longer displays debug text.

### V25.04
- General
  - **Bug Fixes:** Fixed a bug that caused Mel's basic attacks to disable Nidalee's W.
  - Spell indicators are now gold-colored.
- Radiant Volley
  - Cast range reduced to 950 from 1000.
  - Bolt speed reduced to 4500 from 5000.
  - **Bug Fixes:** Fixed a bug that caused the cooldown to reset when killing enemy champions affected by Deadly Venom applied by Rebuttal.
- Rebuttal
  - Duration reduced to $0.75$ seconds from 1 second.
  - Mana cost increased to 80 / 60 / 40 / 20 / 0 from 60 / 45 / 30 / 15 / 0.
  - Base damage modifier reduced to 40 / 45 / 50 / 55 / 60% from 40 / 47.5 / 55 / 62.5 / 70%.
  - **Bug Fixes:** Some critical strike attacks no longer bypass the effect.
  - **Bug Fixes:** No longer always incorrectly reflects projectiles from Pix, Faerie Companion back to Pix.
- Solar Snare
  - Orb root duration reduced to 1.25 / 1.5 / 1.75 / 2 / 2.25 seconds from 1.75 / 1.875 / 2 / 2.125 / 2.25.
  - Orb damage increased to 60 / 105 / 150 / 195 / 240 from 60 / 100 / 140 / 180 / 220.
  - Orb AP ratio increased to 60% AP from 50% AP.
  - **Bug Fixes:** Both the orb and the field now properly apply as area damage instead of spell damage.
- Golden Eclipse
  - Active per-stack AP ratio increased to $3.5$% AP from $2.5$% AP.
  - **Bug Fixes:** No longer illegally resets the ability's cooldown after reflecting Whirling Death.

### V25.S1.3
- General
  - Adjusted facial expression.
  - **Bug Fixes:** Non-unique-event voicelines that trigger from Ambessa no longer fail to play more lines than one per event.
- Mel
  - Dress is now whiter and brighter.
- Rebuttal
  - **Bug Fixes:** Reflected Hawkshot no longer additionally grants vision to the original user's team.
  - **Bug Fixes:** Reflected Jhin Lotus Trap now plays the correct SFX.
  - **Bug Fixes:** If Edge of Ixtal is reflected and that missile would apply her Royal Privilege, Mel no longer receives the innate effects of Royal Privilege for the rest of the game.

### V25.S1.2#January 25th Hotfix|V25.S1.2
- Searing Brilliance
  - Overwhelm first stack AP ratio reduced to 10% AP from 25% AP.
- Radiant Volley
  - Damage per explosion reduced to 13 / 15.5 / 18 / 20.5 / 23 from 13 / 16 / 19 / 22 / 25.
    - Total damage reduced to 78 / 108.5 / 144 / 184.5 / 230 from 78 / 112 / 152 / 198 / 250.
- Golden Eclipse
  - Overwhelm first stack bonus damage reduced to 60 / 70 / 80 from 60 / 77.5 / 95.
  - Active first stack base damage reduced to 100 / 150 / 200 from 125 / 175 / 225.
  - Active first stack damage AP ratio reduced to 30% AP from 40% AP.

## Trivia

- To date, Mel is the first champion to release in Teamfight Tactics before League of Legends.
  - She was in development for League of Legends prior to her release in Teamfight Tactics. However, the Teamfight Tactics team was able to creat unique, but more simplistic art for Mel for her debut in Into the Arcane before she was finished in League of Legends.
- Rebuttal’s barrier bears functional and visual similarity to Zelda's Reflection move (called "Nayru's Love") in the fighting videogame *Super Smash Bros*. The spell (also called Nayru's Love) was originally seen in The_Legend_of_Zelda:_Ocarina_of_Time.
  - **Mel**’s game designer, Riot Emizery, confirmed in an interview that this was not intentional, but rather a serendipitous coincidence.
- Solar Snare’s bears functional and visual similarity to Anduin's Chastise from .
- Mel has a unique twirl animation for picking up a reflected dagger dropped by Bouncing Blade specifically.
- Mel's Homeguard animation resembles the typical run of the famous character from the *Naruto* manga/anime.
  - This is shared with Zed.
- Since V25.04 Mel has gold-colored spell indicators and is the only champion to date to have this.
- Killing by an enemy Ambessa as Mel grants Ambessa the *Took Long Enough* cosmetic buff that reads: *Mel has killed Ambessa. 'This is a new feeling. Pride in someone else. Finally.' - Ambessa Medarda*
  - This is a reference to DragonBall Z Abridged's Vegeta's Unyielding Rage.

---
*This page was automatically generated from League of Legends Wiki data.*