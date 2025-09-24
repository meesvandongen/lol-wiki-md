# Graves

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
| **Champion** | Graves |
| **Title** | the Outlaw |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2011-10-19 |
| **Release Patch** | V1.0.0.127 |
| **Latest Changes** | V25.18 |
| **Roles** | Specialist |
| **Riot Positions** | Jungle |
| **External Positions** | Jungle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Marksman |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 1 |
| **Mobility** | 2 |
| **Utility** | 2 |
| **Style** | 20 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $625.0$ | $+106.0$ |
| **Mana** | $325.0$ | $+40.0$ |
| **Health Regen** | $8.0$ | $+0.7$ |
| **Mana Regen** | $8.0$ | $+0.7$ |
| **Armor** | $33.0$ | $+4.6$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $68.0$ | $+4.0$ |
| **Attack Speed** | $0.475$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $425.0$ | $+0.0$ |
| **Base Attack Speed** | $0.475$ | |
| **Attack Speed Ratio** | $0.49$ | |
| **Bonus AS per Level** | $3.0\%$ | |
| **Windup Modifier** | $0.1$ | |
| **Missile Speed** | $3800$ units/second | |
| **Pathing Radius** | $10$ units | |
| **Selection Radius** | $110$ units | |
| **Selection Height** | $140$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |

## Abilities

### Passive: New Destiny

| Attribute | Value |
|-----------|------:|
| **Range** | 625 (Pellet reach) / er 565 (Pellet collision reach, effective to-edge reach) units |
| **Target Range** | errange Graves |
| **Angle** | 24° / critical strike 30° |
| **Width** | 40 (Individual pellets missile width) units |
| **Speed** | / 3000 (Invisible missile, used for certain attack effects and against wards/plants) / 3400 (Invisible missile when critting) units/second |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Parry** | Special |
| **Call For Help** | True |

**INNATE - DOUBLE BARREL:** **Graves**' basic attacks fire his shotgun to consume ammunition within 2 shells. He will reload (Graves is unable to declare basic attacks during this time.) over a few seconds immediately after expending all shells or withholding leftover shells for a while. The reload time is reduced by **Graves**' level and (as) **bonus** attack speed, and can be interrupted by declaring an attack or casting an ability.

**INNATE - 12-GAUGE:** **Graves**' basic attack sprays 4 pellets in a cone, each colliding with the first enemy unit in their path, dealing 70% / 100% AD physical damage plus「 23.31% / 33.3% AD **additional** physical damage ⟷ about one-third of the damage 」for each subsequent pellet hitting the same target, up to 139.93% / 199.92% AD **total** physical damage against a single target. Pellets deal 25% reduced damage against structures.

Critical strikes spray 6 pellets over a 25% wider cone, with each pellet dealing「 increased damage, ⟷ increased damage equal to 45% **bonus** critical damage, 」resulting in「 (critical damage) 178.312% (+ Infinity Edge 23.997%) **total** damage ⟷ 249.52% / 356.48% (+ Infinity Edge 33.58% / 47.97%) AD **total** physical damage 」if all pellets hit a single target.

Pellets apply life steal at 100% effectiveness. Only the first pellet to hit each enemy applies on-hit effects.

**INNATE - BUCKSHOT:** Non-champion units hit by more than one pellet are knocked back, though not through terrain.

**Notes:**

- **Graves** takes $2.08$ seconds to reload 2 shells (when the clip is empty) and $1.3$ seconds to reload 1 shell. He must idle with 1 shell for $2.08$ seconds before he starts reloading. These values can be *improved* with **bonus** attack speed.
  - Values of **bonus** attack speed between 150% and 200% have a negative effect on **Graves' ** reload speed.
  - Attack speed growth will always have a positive effect on **Graves' ** reload speed and is not counted toward the 150% and 200% attack speed thresholds. For example, **Graves** gains 51% bonus attack speed from attack speed growth at level 18; meaning the thresholds at level 18 are at 201% and 251% respectively.
  - Maximum reload speed is 0.651 / 0.665 / 0.679 / 0.694 / 0.708 / 0.722 / 0.736 / 0.75 / 0.764 / 0.778 / 0.792 / 0.806 / 0.821 / 0.835 / 0.849 / 0.863 / 0.877 / 0.891, which equates to a reload timer of 1.54 / 1.5 / 1.47 / 1.44 / 1.41 / 1.39 / 1.36 / 1.33 / 1.31 / 1.29 / 1.26 / 1.24 / 1.22 / 1.2 / 1.18 / 1.16 / 1.14 / 1.12 seconds, which is the shortest reload time at the 150% threshold.
- **Graves** can hit multiple individual enemies with a single shot from his basic attack. The first pellet hitting an enemy:
  - Deals the highest damage out of all other pellets.
  - Applies and triggers on-hit effects.
    - "Single-use" on-hit effects such as Spellblade items will be applied to the very first enemy hit by a pellet (closest one in this case).
  - Applies basic damage, while all additional pellets deal default damage.
  - Is reduced by Warden's Mail Rock Solid.
- **Graves**' attack windup is based on such a low value that it will always take only 1 game tick (1/30th of a second), even at the lowest possible attack speed of $0.2$.
- When **Graves** has his attack range increased by 35% to (range) $573.75$ via Rapid Firecannon Sharpshooter, it has special effects on the cone spread of his basic attack:
  - The reach of the individual missiles is also increased by 35% to cr $843.75$ / er $783.75$ (Effective to-edge reach).
  - The spread of the individual missiles is *decreased* by 35% (*angle × 1/1.35*) to $17. / $22..
- Pellets will not hit wards nor jungle plants.
  - When **Graves** performs a basic attack against these units, he will fire a standard ranged homing missile at the target to successfully hit it.
    - This same missile is also used for when his basic attack is empowered by an Energized effect.
- Pellets can be dodged and blocked, but not missed while **Graves** is blinded.
  - While **Graves** is blinded, the pellets are special cased to fire in a random direction that is always away from the primary target of his basic attack.
    - Enemies within range of the pellets in their randomly fired direction will be hit as the pellets cannot be missed.
- Structures can be hit by multiple pellets, as with other valid targets.
  - If **Graves' ** basic attack critically strikes against an enemy behind their structure, and this structure is hit by all pellets, the damage dealt to the structure is increased by $33.3$%.
- On-attack effects such as Help, Pix! and Runaan's Hurricane Wind's Fury will be triggered as usual from the completion of **Graves' ** basic attack windup. They will obey their standard target-acquisition rules in spite of **Graves' ** special basic attack behavior.
  - Runaan's Hurricane Wind's Fury can still fire its bolts at secondary targets even if they were already hit by the pellets of **Graves' ** basic attack.
- Black Cleaver Carve will be applied for the physical damage dealt by each pellet, meaning that each of **Graves' ** basic attacks can apply up to 4 stacks of the effect, increased to the maximum of 5 with a critical strike.
- Sundered Sky Lightshield Strike will function normally with **Graves' ** basic attacks, causing the pellets to critically strike if the primary target was also a target of the item's effect.
  - If none of the pellets hit the target of *Lightshield Strike*’s effect, however, the item's effect will not be consumed on the target thus not triggering the heal nor applying the on-target-cooldown, but still causing **Graves' ** basic attack to critically strike as if the effect was successfully consumed.
    - His basic attacks against the marked target will be guaranteed to critically strike until the item's effect has been consumed by at least one pellet hitting them.
  - Pellets will not consume the item's effect on secondary targets that are also targets of the item's effect; they will not consume the mark of enemies near the primary target that are marked by *Lightshield Strike*.
- Randuin's Omen Resilience will reduce the critical damage dealt by every pellet, resulting in each pellet dealing less damage than if they were to not critically strike.
- Kraken Slayer Bring It Down VFX for its missile appears to fire at the primary target, but will always collide with the target hit by the first pellet.
- The in-game HUD for the champion's stat panel incorrectly displays **Graves** "reload speed" for reloading 2 shells as his attack speed, instead of displaying his actual attack speed which behaves normally like any other champion. This displayed value more accurately represents his *reloads per second*, and not *attacks per second* as the attack speed tooltip in the HUD would suggest.

---

### Q: End of the Line

| Attribute | Value |
|-----------|------:|
| **Range** | 800 (Initial shell missile range) / er 900 (Total forward range including explosion) units |
| **Cast Time** | $0.25$ seconds |
| **Width** | 80 (Initial shell missile width) / 500 (Explosion rectangle width) / 200 (Powder trail return missile width) units |
| **Cost** | 80 Mana |
| **Cooldown** | 13 / 11.25 / 9.5 / 7.75 / 6 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | Special |

**ACTIVE:** **Graves** fires a round in the target direction that deals physical damage to enemies it passes through and leaves behind a powder trail.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 45 / 65 / 85 / 105 / 125 (+ 80% **bonus** AD) |

After 2 seconds, or upon colliding with terrain, the round detonates to deal physical damage to enemies within a wide perpendicular area and in a reverse wave along the powder trail.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 85 / 120 / 155 / 190 / 225 (+ 40 / 65 / 90 / 115 / 140% **bonus** AD) |
| **Total Physical Damage** | 130 / 185 / 240 / 295 / 350 (+ 120 / 145 / 170 / 195 / 220% **bonus** AD) |

**Notes:**

- Spell shields can only block one instance of the ability's damage.
- Wind Wall will destroy the projectile entirely and mitigate all effects, including the trail on the ground.
- Unbreakable is considered to be terrain for the purposes of the projectile, triggering its second effect. Effect at cast time end

---

### W: Smoke Screen

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 950 units |
| **Effect Radius** | 200 (Canister landing damage radius) units |
| **Speed** | 1500 (Starts traveling at the start of the cast time) units/second |
| **Cost** | 70 / 75 / 80 / 85 / 90 Mana |
| **Cooldown** | 26 / 24 / 22 / 20 / 18 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |
| **Sight Reduction** | 250 (Estimated, reduces sight radius to this number) |

**ACTIVE:** **Graves** fires a smoke canister at the target location that upon impact deals magic damage to enemies within the area and slows them by 50% for $0.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 110 / 160 / 210 / 260 (+ 60% AP) |

The canister creates a cloud of smoke for 4 seconds that applies nearsight of outside the area to all enemies within every $0.25$ seconds, reducing their sight radius relative to the center.

**Notes:**

- *Smoke Screen* will nearsight untargetable units.
- Spell shields will block the instance of damage and slow, but will not mitigate the persistent area of effect.
- Crowd control immunity and cleanses have no effect on the user's sight radius reduction despite the nearsight being resisted / removed. Loss of allied vision will still be affected.
  - Removing or resisting the nearsight will prevent it from being reapplied while staying inside the area (leaving and coming back inside will not).
- The nearsight debuff applies to non-champions but it does not inherently stop them from performing actions that require sight, such as basic attacking. Effect at cast time end

---

### E: Quickdraw

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 275 (Minimum dash range) / 375 (Maximum dash range) units |
| **Cost** | 40 Mana |
| **Cooldown** | 16 / 15 / 14 / 13 / 12 seconds |
| **Targeting** | Location |
| **Affects** | Self |
| **Out of Range Behavior** | cast at max |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Graves** dashes to the target location, reloading one shell while generating a stack of *True Grit* for 4 seconds, stacking up to 8 times. Dashing towards an enemy champion generates 2 stacks of *True Grit*.

*Quickdraw*’s **current** cooldown is reduced by $0.5$ seconds for every pellet impacting an enemy.

**TRUE GRIT:** For each stack, **Graves** gains (armor) **bonus** armor. Subsequent casts of *Quickdraw* and attacks versus non-minions will refresh the duration of *True Grit*.

| Attribute | Value |
|-----------|------:|
| **Bonus Armor** | 4 / 7 / 10 / 13 / 16 |
| **Maximum Bonus Armor** | 32 / 56 / 80 / 104 / 128 |

*Quickdraw resets **Graves**' basic attack timer.*

**Notes:**

- If **Graves** dashes while reloading, he can cast any of his abilities during the dash.
- **Graves** gains the one additional stack of *True Grit* from *Quickdraw* if at any point during the dash a visible enemy champion is in front of *Graves* within er 1300 distance.
  - Enemy champions have to be targetable to grant the additional stack.
  - The additional stack of *True Grit* is gained when dashing towards clones just like actual champions.
  - This check never happens if **Graves** dashes 0 distance (impossible outside of bugs), and will not always repeat a final time at the very end of the dash. Otherwise, it occurs continously while **Graves** is dashing.
  - This works for a 180 degree sector from Graves to the Champion, as such dashing parallel to the champ will also give a bonus stack.

---

### R: Collateral Damage

| Attribute | Value |
|-----------|------:|
| **Range** | 1100 (Shell missile range) / er 1690 (Maximum range including collateral cone, estimated) units |
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 740 (Collateral end cone width, estimated) units |
| **Angle** | er 60° |
| **Width** | 200 (Shell missile width) units |
| **Speed** | 2100 (Shell missile speed. The explosion is instantaneous) |
| **Cost** | 100 Mana |
| **Cooldown** | 100 / 90 / 80 / 70 / 60 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies / Self |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Grounded** | Special |
| **Knockdown** | True |

**ACTIVE:** **Graves** fires an explosive shell in the target direction that deals physical damage to enemies hit and causes him to recoil 400 units in the opposite direction.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 275 / 350 / 425 / 500 / 575 (+ 150% **bonus** AD) |

Upon hitting an enemy champion or reaching the end of its range, the shell explodes in a cone to deal reduced damage to additional enemies.

| Attribute | Value |
|-----------|------:|
| **Reduced Damage** | 200 / 260 / 320 / 380 / 440 (+ 120% **bonus** AD) |

**Notes:**

- Applies spell damage to the target hit by the shell (including to non-champions struck before exploding) and area damage to targets within the explosion cone.
- The projectile will still explode early if the champion hit by the shell is protected by a spell shield.
- The explosion cone is anchored on the location of the shell missile when it collided with a champion, or otherwise its maximum cast range.
- The cone damage is always located 150 units behind, based off its direction
- *Collateral Damage* is not disabled while grounded or rooted, but **Graves** will not dash after the cast time while affected by them.
- **Graves** will not dash after the cast time if he is airborne. Effect at cast time end

---

## Patch History

### V25.18
- Collateral Damage
  - **Bug Fixes:** Corrected timing of cone VFX, which previously spawned slightly before the projectile hits an enemy.

### V25.14
- New Destiny
  - **Bug Fixes:** Pellets no longer fail to hit Shepherd of Souls at maximum range.

### V14.24
- End of the Line
  - Detonation bonus AD ratio reduced to 40 / 65 / 90 / 115 / 140% **bonus** AD from 40 / 70 / 100 / 130 / 160%.
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.17
- End of the Line
  - Cooldown reduced to 13 / 11.25 / 9.5 / 7.75 / 6 seconds from 13 / 11.5 / 10 / 8.5 / 7.
  - Initial base damage increased to 45 / 65 / 85 / 105 / 125 from 45 / 60 / 75 / 90 / 105.

### V14.13
- Quickdraw
  - Bonus armor per stack increased to 4 / 7 / 10 / 13 / 16 from 2 / 5 / 8 / 11 / 14.
    - Maximum bonus armor increased to 32 / 56 / 80 / 104 / 128 from 16 / 40 / 64 / 88 / 112.

### V14.9
- New Destiny
  - **Bug Fixes:** Sundered Sky Lightshield Strike is no longer consumed with no effect, and now properly triggers the critical attack.

### V14.8#April 18th Hotfix|V14.8
- Stats
  - Attack speed growth increased to 3% from $2.6$%.
- New Destiny
  - The previous change for this ability's attack speed scaling has been reverted due to unintentionally losing its intended purpose of reducing reload time at certain amounts of bonus attack speed.

### V14.8
- New Destiny
  - Increased non-growth attack speed scaling for decreasing reload time by 35%.
  - Pellet critical strike bonus damage increased to 45% from 40%.
    - Bonus damage per pellet critical strike increased to ($33.75$% + Infinity Edge) from (30% + Infinity Edge).
  - Relative **bonus** damage of critical attacks where all pellets hit increased to (critical damage) $78.3$% (+ Infinity Edge 30%) from (critical damage) $73.3$% (+ Infinity Edge $26.7$%).
    - Maximum critical strike damage increased to 249.52% / 356.48% (+Infinity Edge 41.98% / 59.97%) AD from 242.52% / 346.48% (+Infinity Edge 37.31% / 53.3%) AD.

### V14.7
- Collateral Damage
  - **Bug Fixes:** Can once again be cast while grounded.
  - **New Effect:** Grounding and root effects now prevent the dash after casting.

### V14.2#January 24th Hotfix|V14.2
- Smoke Screen
  - **Bug Fixes:** No longer grants an immensely large and recursive amount of adaptive force and hybrid resistances to champions who simultaneously have spells that echo consistent statistic updates and have the Unflinching rune equipped.

## Trivia

- Graves' jokes reference gallows humor, as per his surname *Cemetery*.
- Graves' dance references one from a country show.
  - A side-by-side comparison can be seen here.
- Graves was the second-to-last champion to have a "League Judgement" (they were discontinued after release; Varus was the last champion to receive one).
- End of the Line is among a few abilities that were named after a quote of their champion. In this case, it references his "End of the Line" quote which he had before the patch he got the ability, V5.22.
  - The others are Goes Where He Pleases and League of Draven.
- The icon for Collateral Damage is a reference to explosive hazard safety signs.
- Graves' shotgun can be seen in the game's Mac version trailer.
- Graves was the product of playerbase requests for a 'manly' ranged carry.
- In the Chinese localization, the names of all four of his active abilities are four-character phrases that allude to the criminal underworld and are often titles of gangster or martial arts films or television.
  - End of the Line becomes 人在江湖, literally "a person in river and lake"; "river and lake" is a euphemism for the underworld, and the phrase is usually used to imply a person in the underworld cannot avoid committing unsavoury actions. Secondary title of *Young and Dangerous*, a Hong Kong triad film, in Chinese.
  - Smoke Screen becomes 一手遮天, literally "covering the sky with one hand", a phrase to describe a criminal so well-connected and/or powerful they can halt justice. Secondary title of *Young and Dangerous 3*, a sequel to the above, in Chinese.
  - Quickdraw becomes 猛龙过江, literally "ferocious dragon crossing river", usually used to describe a ruthless foreign rival or criminal. Secondary title of *Young and Dangerous 2* in Chinese. It is also the Chinese name of *Way of the Dragon*, starring Bruce Lee who fights against foreign martial artists including Chuck Norris.
  - Collateral Damage becomes 龙争虎斗, literally "struggle of dragon and tiger", a phrase similar to "clash of the titans" in English. Secondary title of *Young and Dangerous 5* in Chinese. It is also the Chinese name of *Enter the Dragon*.

---
*This page was automatically generated from League of Legends Wiki data.*