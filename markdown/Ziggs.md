# Ziggs

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
| **Champion** | Ziggs |
| **Title** | the Hexplosives Expert |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2012-02-01 |
| **Release Patch** | V1.0.0.133 |
| **Latest Changes** | V25.15 |
| **Roles** | Artillery |
| **Riot Positions** | Bottom |
| **External Positions** | Middle, Bottom |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 1 |
| **Hero Type** | Mage |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 90 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $606.0$ | $+106.0$ |
| **Mana** | $480.0$ | $+23.5$ |
| **Health Regen** | $6.5$ | $+0.6$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $21.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $55.0$ | $+3.1$ |
| **Attack Speed** | $0.656$ | |
| **Movement Speed** | $325.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.656$ | |
| **Attack Speed Ratio** | $0.656$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Missile Speed** | $1500$ units/second | |
| **Acquisition Radius** | $800$ units | |
| **Gameplay Radius** | $55$ units | |
| **Pathing Radius** | $30$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $110$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $87.0\%$ |
| **Damage Taken** | $120.0\%$ |

## Abilities

### Passive: Short Fuse

| Attribute | Value |
|-----------|------:|
| **Speed** | 1500 (Attack missile speed, same as basic attacks) units/second |
| **Static Cooldown** | 12 |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | False |
| **Spell Effects** | Proc |
| **Parry** | unknown |

**INNATE:** Periodically, **Ziggs** empowers his next basic attack to deal 20 / 24 / 28 / 32 / 36 / 40 / 48 / 56 / 64 / 72 / 80 / 88 / 100 / 112 / 124 / 136 / 148 / 160 (+ 50% AP) **bonus** magic damage,「 increased by 75% against structures. ⟷ increased to 1.75*(20+4*(x-1)) for 6 / then +1.75×8*x for 6 / then +1.75×12*x (+ 87.5% AP) against structures. 」

*Short Fuse*’s cooldown is reduced by 4–6@1–13 seconds whenever **Ziggs** casts an ability.

**Notes:**

- The empowered attack will not trigger against wards.
- : *Short Fuse*’s interactions with dodging, blocking, and blinding effects.

---

### Q: Bouncing Bomb

| Attribute | Value |
|-----------|------:|
| **Range** | 1400 (Maximum range with bounces, excluding explosion radius) units |
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 850 (Cast range) units |
| **Collision Radius** | 180 (Hit detection on bounces) units |
| **Effect Radius** | 240 (Explosion radius) units |
| **Speed** | 1700 (Initial missile) / Fixed time (Bounces travel over 0.495 seconds each, regardless of distance) units/second |
| **Cost** | 50 / 55 / 60 / 65 / 70 Mana |
| **Cooldown** | 6 / 5.5 / 5 / 4.5 / 4 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Ziggs** throws a bomb to the target location that bounces forward up to two times, with the distance traveled each time being based on how far it was originally thrown.

The bomb explodes upon landing near an enemy, within terrain, or the final bounce, dealing magic damage to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 130 / 180 / 230 / 280 (+ 60 / 65 / 70 / 75 / 80% AP) |

**Notes:**

- *Bouncing Bomb* can be thrown or bounce over units/terrain.
- Spell shield will block the damage of the explosion, but the explosion can still damage other targets in the radius.
- **Ziggs** will throw the bomb from his location at the end of the cast time, towards the pre-clamped (Relevant only if cast beyond maximum range) cast location, causing the bounces to adjust their angle accordingly.

---

### W: Satchel Charge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 1000 units |
| **Effect Radius** | 325 / sight 400 units |
| **Speed** | 1750 (Missile speed) units/second |
| **Cost** | 80 Mana |
| **Cooldown** | 20 / 18 / 16 / 14 / 12 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Location / Auto |
| **Affects** | Enemies / Self |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Out of Range Behavior** | Walk in range of the target location to cast (first cast) |
| **Grounded** | Special |
| **Knockdown** | True |

**ACTIVE:** **Ziggs** hurls a charge to the target location, remaining there for 4 seconds and granting sight of the area. *Satchel Charge* can be recast within the duration, and does so automatically afterwards. If recast while in flight, the charge will explode immediately upon landing.

**RECAST:** **Ziggs** detonates the charge, causing it to explode to deal magic damage to nearby enemies and knock them back over $0.5$ seconds up to 500 units away from the center of the explosion, though not through terrain. If this hits **Ziggs**, he will dash up to 825 units away from the center.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 105 / 140 / 175 / 210 (+ 50% AP) |

Satchel Charge Turret Explosion Indicator.png The explosion also execute enemy turrets within that are below a **maximum** health threshold.

| Attribute | Value |
|-----------|------:|
| **Demolition Threshold** | 25 / 27.5 / 30 / 32.5 / 35% of turret's **maximum** health |

***Ziggs** does not dash if he is immobilized or grounded. He can cast any of his abilities during the dash.*

**Notes:**

*The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive. **Detonating the ability manually does not.
- Deals area damage to non-turrets and raw damage to turrets.
- *Satchel Charge*’s distance increases with proximity to the charge, resulting in being knocked straight up when aligned directly on top of it.
- **Ziggs** will not *dash* from *Satchel Charge*’s detonation while Teleport but he will while Recall.

---

### E: Hexplosive Minefield

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 900 units |
| **Effect Radius** | 335 (Effective minefield radius) / cr 135 (Detection and explosion radius of each individual mine) units |
| **Speed** | 1550 (Travel speed to cast location) units/second |
| **Cost** | 70 / 80 / 90 / 100 / 110 mana |
| **Cooldown** | 16 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Ziggs** scatters a cluster of 11 proximity mines over the target location that grant sight of the area for 2 seconds, arming after $0.5$ seconds (travel time of the mines spreading out plus a brief delay) and lasting for up to 10 seconds.

Each mine within the area explodes upon contact with terrain or an enemy, dealing magic damage and slowing them for $1.5$ seconds, as well as granting sight of the area around the explosion for 2 seconds. An enemy takes 40% damage from subsequent mines.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Mine** | 30 / 70 / 110 / 150 / 190 (+ 25 / 30 / 35 / 40 / 45% AP) |
| **Reduced Damage per Mine** | 12 / 28 / 44 / 60 / 76 (+ 10 / 12 / 14 / 16 / 18% AP) |
| **Maximum Total Magic Damage** | 150 / 350 / 550 / 750 / 950 (+ 125 / 150 / 175 / 200 / 225% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 10 / 20 / 30 / 40 / 50% |

**Notes:**

- The mine cluster is built of 2 rings, with 3 mines in the inner (radius pending for test) and 8 mines on the other (200 radius), all equally spaced.
  - The angular offset is randomized on each cast.
  - Because of the cr 135 detonation radius of each mine, the total possible range of a mine from the minefield hitting an enemy from the cast location is cr 335 units. It will often be slightly less as the minefield is not guaranteed to be angled the right way for this.
- Mines will not explode upon contact with player-generated terrain.
- Spell shield will block the effects of a single detonation.

---

### R: Mega Inferno Bomb

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.375$ seconds |
| **Target Range** | 5000 units |
| **Effect Radius** | 525 units |
| **Inner Radius** | 250 units |
| **Speed** | 2250 units/second |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 107.5 / 95 / 82.5 / 70 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | False |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Ziggs** catapults the *Mega Inferno Bomb* to the target location, granting sight within a 600 radius around its destination for 4 seconds.

The bomb explodes upon arrival to deal magic damage to enemies hit, reduced to 65% against those outside of the epicenter.

| Attribute | Value |
|-----------|------:|
| **Epicenter Magic Damage** | 300 / 400 / 500 / 600 / 700 (+ 100% AP) |
| **Reduced Damage** | 195 / 260 / 325 / 390 / 455 (+ 65% AP) |

**Notes:**

- Allies cannot see *Mega Inferno Bomb*’s area indicator until shortly before the blast.
- *Mega Inferno Bomb* takes $2.64$ seconds from the start of the cast time to land when *Ziggs* casts at maximum range (reduced to $1.617$ seconds from the start of the cast time when casting within 2700 (Estimated) units).
  - When cast within 2700 units, *Mega Inferno Bomb* has a fixed travel time. Beyond that, the travel time is equal to the cast distance divided by the now-fixed missile speed.
- The area reveal on the target location begins as soon as the cast time is completed and can see into brush and across terrain.

---

## Patch History

### V25.15
- Mega Inferno Bomb
  - Outer radius damage reduced to 65% from $66.6$%.
    - Outer radius base damage reduced to 195 / 325 / 455 from 200 / 333.33 / 466.67.
    - Outer radius AP ratio reduced to 65% AP from 66.67% AP.

### V25.14
- Bouncing Bomb
  - Base damage reduced to 80 / 130 / 180 / 230 / 280 from 85 / 135 / 185 / 235 / 285.
  - AP ratio changed to 60 / 65 / 70 / 75 / 80% AP from 65% AP at all ranks.
- Hexplosive Minefield
  - AP ratio changed to 25 / 30 / 35 / 40 / 45% AP from 30% AP at all ranks.
- Mega Inferno Bomb
  - Maximum base damage increased to 300 / 500 / 700 from 300 / 450 / 600.
    - Minimum base damage increased to 200 / 333.33 / 466.67 from 200 / 300 / 400.
  - Maximum AP ratio reduced to 100% AP from 110% AP.
    - Minimum AP ratio reduced to 66.67% AP from 73.33% AP.

### V25.04
- Satchel Charge
  - **Bug Fixes:** Area indicator is no longer missing after it passes and/or while it remains in Fog of War.

### V14.18
- Short Fuse
  - Structure damage reduced to 175% from 250%.
    - Base damage against structures reduced to 1.75*(20+4*(x-1)) for 6 / then +1.75×8*x for 6 / then +1.75×12*x from 2.5*(20+4*(x-1)) for 6 / then +2.5×8*x for 6 / then +2.5×12*x.
    - AP ratio against structures reduced to 87.5% AP from 125% AP.

### V14.3
- Stats
  - Base armor increased to 21 from 18.
  - Armor growth increased to $4.7$ from $4.5$.
- Satchel Charge
  - Cooldown reduced to 20 / 18 / 16 / 14 / 12 seconds from 24 / 21 / 18 / 15 / 12.

### V14.2
- Satchel Charge
  - **Bug Fixes:** Cast is no longer locked out while he is grounded or rooted.

### V13.23
- Stats
  - Base attack damage increased to 55 from 54.
- Bouncing Bomb
  - Base damage reduced to 85 / 135 / 185 / 235 / 285 from 95 / 145 / 195 / 245 / 295.
- Satchel Charge
  - Cooldown increased to 24 / 21 / 18 / 15 / 12 seconds from 20 / 18 / 16 / 14 / 12.
  - Mana cost increased to 80 from 65.

### V13.22
- Stats
  - Base armor reduced to 18 from 22.
  - Attack windup reduced to 20% from $20.833333$%.

### V13.20
- Bouncing Bomb
  - Collision radius increased to 180 units from 150.

### V13.19
- Ziggs and Ziggs
  - Satchel Charge
    - **Bug Fixes:** VFX now properly renders in front of all terrain and is thus no longer cut off when placed on slopes.

## Trivia

- Ziggs is the first (so far only) champion to have all his abilities (except innates) be the same type (ground-targeted area of effect).
- Ziggs, Blitzcrank, Caitlyn, Lissandra, Rumble, Sion, Varus, Vi, Viego, and Xerath are the only champions who can apply crowd control on themselves.
- Ziggs' dance references 'Midget Superstar'.
  - A side-by-side comparison can be seen here.
- Ziggs was a nickname for Rioter Joe 'Hephastopheles' Ziegler.
- Heimerdinger reveals that Ziggs's full name is *Zigmund*, from Proto-Germanic languages **segiz* "victory" & **mundō* "; the latter element is shared with Dr. Mundo.
- Ziggs and Nautilus were first conceived as 'Ivan the Mad Bomber'.
- Hexplosive Minefield was conceived as Ziggs' ultimate (was later replaced by Mega Inferno Bomb)
  - Mega Inferno Bomb’s icon references a nuclear weapon.
- Ziggs seems to be the mascot for featured game modes.
  - This is evident from his likeness being used on Mirror Mode profileicon.png Doom Icon of Doom profileicon.png Nemesis Draft profileicon.png that represent or commemorate them, as well as having his teaser picture be the base for the promotional art for a lot of these game modes.
- A prop of Ziggs' Hexplosive Bomb was crafted in an episode of YouTube series DIY Prop Shop.
  - This video can be viewed here.
    - There are also videos where the following are crafted:
      - Diana’s Crescent Moonblade (functional)
      - Katarina’s Daggers (functional)
      - Leona’s Zenith Blade (functional)
      - Master Yi’s Highlander' Ring Sword (functional)
      - Poppy’s Hammer of Orlon (functional)
      - Yasuo’s Last Breath' Sword (functional)
- The icon for Satchel Charge is reused for the Teamfight Tactics item Demolitionist's Charge.

---
*This page was automatically generated from League of Legends Wiki data.*