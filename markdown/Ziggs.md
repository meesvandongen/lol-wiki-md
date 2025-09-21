# Ziggs

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
| **Champion** | Ziggs |
| **Title** | the Hexplosives Expert |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2012-02-01 |
| **Release Patch** | V1.0.0.133 |
| **Roles** | Artillery |
| **Riot Positions** | Bottom |
| **External Positions** | Middle, Bottom |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $606.0$ | $+106.0$ | $2408.0$ |
| **Mana** | $480.0$ | $+23.5$ | $879.5$ |
| **Health Regen** | $6.5$ | $+0.6$ | $16.7$ |
| **Mana Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Armor** | $21.0$ | $+4.7$ | $100.9$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $55.0$ | $+3.1$ | $107.7$ |
| **Attack Speed** | $0.656$ | $+2.0\%$ | $0.879$ |
| **Movement Speed** | $325.0$ | $+0.0$ | $325.0$ |
| **Attack Range** | $550.0$ | $+0.0$ | $550.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.656$ |
| **Attack Speed Ratio** | $0.656$ |
| **Bonus AS per Level** | $2.0\%$ |
| **Missile Speed** | $1500 units/second$ |
| **Acquisition Radius** | $800 units$ |
| **Gameplay Radius** | $55 units$ |
| **Pathing Radius** | $30 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $110 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Short Fuse

**Innate:** Periodically, **Ziggs**’s next basic attack will deal **bonus** magic damage, increased against structures.

*'Short Fuse's cooldown is cdr whenever **Ziggs** casts an ability.*

**Innate:** Periodically, **Ziggs** empowers his next basic attack to deal 20–160 (+ 50% AP) **bonus* magic damage, 'Short Fuse's *cooldown* is reduced by 4–6@1–13 seconds whenever **Ziggs** casts an ability.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Speed** | 1500 units/second |
| **Spell Shield** | False |
| **Spell Effects** | Proc |

**Notes:**

- The empowered attack will not trigger against wards.
- : 'Short Fuse's interactions with dodge, block, and blind effects.

---

### Q: Bouncing Bomb

**Active:** **Ziggs** throws a bomb to the target location that bounces twice forward. It will explode upon hitting an enemy, terrain, or the final bounce, dealing magic damage to nearby enemies.

**Active:** **Ziggs** throws a bomb to the target location that bounces forward up to two times, with the distance traveled each time being based on how far it was originally thrown. The bomb explodes upon landing near an enemy, within terrain, or the final bounce, dealing magic damage to nearby enemies.

| Attribute | Value |
|-----------|-------|
| **Range** | 850 units |
| **Cooldown** | $6-4$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $50-70$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Speed** | 1700 / Fixed time units/second |
| **Effect Radius** | 240 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $80-280$

**Notes:**

- *Bouncing Bomb* can be thrown or bounce over units/terrain.
- Spell shield will block the damage of the explosion, but the explosion can still damage other targets in the radius.
- **Ziggs** will throw the bomb from his location at the end of the cast time, towards the pre-clamped cast location, causing the bounces to adjust their angle accordingly.

---

### W: Satchel Charge

**Active:** **Ziggs** hurls a charge to the target location that remains for a few seconds. *Satchel Charge* can be recast within the duration, and does so automatically afterwards.

**Recast:** The charge explodes to deal magic damage and airborne nearby enemies hit. If this hits **Ziggs**, he will dash farther away.

**Active:** **Ziggs** hurls a charge to the target location, remaining there for 4 seconds and granting sight of the area. *Satchel Charge* can be recast within the duration, and does so automatically afterwards. If recast while in flight, the charge will explode immediately upon landing. **Recast:** **Ziggs** detonates the charge, causing it to explode to deal magic damage to nearby enemies and airborne over $0.5$ seconds up to 500 units away from the center of the explosion, though not through terrain. If this hits **Ziggs**, he will dash up to 825 units away from the center. Satchel Charge Turret Explosion Indicator.png The explosion also execute enemy turrets within that are below a **maximum** health threshold. **Ziggs does not dash if he is immobilize or ground. He can cast any of his abilities during the dash.**

| Attribute | Value |
|-----------|-------|
| **Range** | 1000 units |
| **Cooldown** | $20-12$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 80 Mana |
| **Targeting** | Location / Auto |
| **Affects** | Enemies / Self |
| **Damage Type** | magic |
| **Speed** | 1750 units/second |
| **Effect Radius** | 325 / sight 400 units |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $70-210$ (+ 50% AP)
- **Demolition Threshold:** $25-35$% of turret's
- **maximum** health

**Notes:**

- Deals area damage to non-turrets and raw damage to turrets.
- 'Satchel Charge's distance increases with proximity to the charge, resulting in being knocked straight up when aligned directly on top of it.
- **Ziggs** will not *dash* from 'Satchel Charge's detonation while Teleport but he will while Recall.

---

### E: Hexplosive Minefield

**Active:** **Ziggs** scatters proximity mines over the target location that remain a period. Each mine will explode upon contact with an enemy or terrain, dealing magic damage and briefly slow them.

**Active:** **Ziggs** scatters a cluster of 11 proximity mines over the target location that grant sight of the area for 2 seconds, arming after $0.5$ seconds and lasting for up to 10 seconds. Each mine within the area explodes upon contact with terrain or an enemy, dealing magic damage and slow them for $1.5$ seconds, as well as granting sight of the area around the explosion for 2 seconds. An enemy takes 40% damage from subsequent mines.

| Attribute | Value |
|-----------|-------|
| **Range** | 900 units |
| **Cooldown** | 16 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-110$ mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Speed** | 1550 units/second |
| **Effect Radius** | 335 / cr 135 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage per Mine:* $30-190$ (+ $25-45$% AP)0.4-190×0.4$ (+ $25×0.4-450.4$% AP)(1+0.4×10)-190*(1+0.4×10)$ (+ $25*(1+0.4×10)-45*(1+0.4×10)$% AP) **Slow:** $10-50$%

**Notes:**

- The mine cluster is built of 2 rings, with 3 mines in the inner (radius pending for test) and 8 mines on the other (200 radius), all equally spaced.
  - The angular offset is randomized on each cast.
  - Because of the cr 135 detonation radius of each mine, the total possible range of a mine from the minefield hitting an enemy from the cast location is cr 335 units. It will often be slightly less as the minefield is not guaranteed to be angled the right way for this.
- Mines will not explode upon contact with player-generated terrain.
- Spell shield will block the effects of a single detonation.

---

### R: Mega Inferno Bomb

**Active:** **Ziggs** catapults the *Mega Inferno Bomb* to the target location that deals magic damage to enemies hit, increased at the epicenter.

**Active:** **Ziggs** catapults the *Mega Inferno Bomb* to the target location, granting sight within a 600 radius around its destination for 4 seconds. The bomb explodes upon arrival to deal magic damage to enemies hit, reduced to 65% against those outside of the epicenter.

| Attribute | Value |
|-----------|-------|
| **Range** | 5000 units |
| **Cooldown** | $120-70$ seconds |
| **Cast Time** | $0.375$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Speed** | 2250 units/second |
| **Effect Radius** | 525 units |
| **Spell Shield** | True |
| **Spell Effects** | aoe |
| **Projectile** | False |

**Scaling:**
- **Epicenter Magic Damage:* $300-7000.65-700×0.65$ (+ $100×0.65$% AP)

**Notes:**

- Allies cannot see 'Mega Inferno Bomb's area indicator until shortly before the blast.
- *Mega Inferno Bomb* takes $2.64$ seconds from the start of the cast time to land when *Ziggs* casts at maximum range (reduced to $1.617$ seconds from the start of the cast time when casting within 2700 units).
  - When cast within 2700 units, *Mega Inferno Bomb* has a fixed travel time. Beyond that, the travel time is equal to the cast distance divided by the now-fixed missile speed.
- The area reveal on the target location begins as soon as the cast time is completed and can see into brush and across terrain.

---

## Patch History

### V25.15
- *Mega Inferno Bomb*
  - Outer radius damage reduced to 65% from $66.6$%.
    - Outer radius base damage reduced to $300×0.65-700×0.65 3$ from $300*(2/3)-700*(2/3) 3 round=2$.
    - Outer radius AP ratio reduced to $100×0.65$% AP from $100*(2/3).

### V25.14
- *Bouncing Bomb*
  - Base damage reduced to $80-280$ from $85-285$.
  - AP ratio changed to $60-80$% AP from 65% AP at all ranks.
- *Hexplosive Minefield*
  - AP ratio changed to $25-45$% AP from 30% AP at all ranks.
- *Mega Inferno Bomb*
  - Maximum base damage increased to $300-700 3$ from $300-600 3$.
    - Minimum base damage increased to $300*(2/3)-700(2/3) 3(2/3)-600*(2/3) 3$.
  - Maximum AP ratio reduced to 100% AP from 110% AP.
    - Minimum AP ratio reduced to $100*(2/3) from $110*(2/3).

### V25.04
- *Satchel Charge*
  - **Bug Fixes:** Area indicator is no longer missing after it passes and/or while it remains in Fog of War.

### V14.18
- *Short Fuse*
  - Structure damage reduced to 175% from 250%.
    - Base damage against structures reduced to 1.75*(20+4*(x-1)) for 6–then +1.75×12*x from 2.5*(20+4*(x-1)) for 6–then +2.5×12*x.
    - AP ratio against structures reduced to $50×1.75$% AP from $50×2.5$% AP.

### V14.3
- Stats
  - Base armor increased to 21 from 18.
  - Armor growth increased to $4.7$ from $4.5$.
- *Satchel Charge*
  - Cooldown reduced to $20-12$ seconds from $24-12$.

### V14.2
- *Satchel Charge*
  - **Bug Fixes:** Cast is no longer locked out while he is ground or root.

### V13.23
- Stats
  - Base attack damage increased to 55 from 54.
- *Bouncing Bomb*
  - Base damage reduced to $85-285$ from $95-295$.
- *Satchel Charge*
  - Cooldown increased to $24-12$ seconds from $20-12$.
  - Mana cost increased to 80 from 65.

### V13.22
- Stats
  - Base armor reduced to 18 from 22.
  - Attack windup reduced to 20% from $20.833333$%.

### V13.20
- *Bouncing Bomb*
  - Collision radius increased to 180 units from 150.

### V13.19
- Ziggs and Ziggs
  - *Satchel Charge*
    - **Bug Fixes:** VFX now properly renders in front of all terrain and is thus no longer cut off when placed on slopes.

## Trivia

- Ziggs is the first (so far only) champion to have all his abilities (except innates) be the same type (ground-targeted area of effect).
- Ziggs, **Blitzcrank**, **Caitlyn**, **Lissandra**, **Rumble**, **Sion**, **Varus**, **Vi**, **Viego**, and **Xerath** are the only champions who can apply crowd control on themselves.
- Ziggs' dance references 'Midget Superstar'.
  - A side-by-side comparison can be seen here.
- Ziggs was a nickname for Rioter Joe 'Hephastopheles' Ziegler.
- **Heimerdinger** reveals that Ziggs's full name is *Zigmund*, from Proto-Germanic languages **segiz* "victory" & **mundō "Tt*Dr. Mundo**.
- Ziggs and **Nautilus** were first conceived as 'Ivan the Mad Bomber'.
- *Hexplosive Minefield* was conceived as Ziggs' ultimate (was later replaced by *Mega Inferno Bomb*)
  - *Mega Inferno Bomb*’s icon references a nuclear weapon.
- Ziggs seems to be the mascot for featured game modes.
  - This is evident from his likeness being used on Mirror Mode profileicon.png Doom Icon of Doom profileicon.png Nemesis Draft profileicon.png that represent or commemorate them, as well as having his teaser picture be the base for the promotional art for a lot of these game modes.
- A prop of Ziggs' *Hexplosive Bomb* was crafted in an episode of YouTube series DIY Prop Shop.
  - This video can be viewed here.
    - There are also videos where the following are crafted:
      - Moonfall (functional)
      - Preparation (functional)
      - Zenith Blade (functional)
      - Highlander (functional)
      - Hammer Shock (functional)
      - Steel Tempest (functional)
- The icon for *Satchel Charge* is reused for the Teamfight Tactics item *Demolitionist's Charge*.

---
*This page was automatically generated from League of Legends Wiki data.*