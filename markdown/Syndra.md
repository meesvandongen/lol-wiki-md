# Syndra

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
| **Champion** | Syndra |
| **Title** | the Dark Sovereign |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2012-09-13 |
| **Release Patch** | V1.0.0.147 |
| **Latest Changes** | V25.18 |
| **Roles** | Burst |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 3 |
| **Hero Type** | Mage |
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
| **Health** | $563.0$ | $+104.0$ |
| **Mana** | $480.0$ | $+40.0$ |
| **Health Regen** | $6.5$ | $+0.6$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $25.0$ | $+4.6$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $54.0$ | $+2.9$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.0\%$ | |
| **Missile Speed** | $1800$ units/second | |
| **Acquisition Radius** | $575$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $125$ units | |
| **Selection Height** | $200$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $110.0\%$ |

## Abilities

### Passive: Transcendent

| Attribute | Value |
|-----------|------:|
| **On-target CD Static** | 8 |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** **Syndra** collects Splinters of Wrath that enhance each of her abilities based on the number of them collected, stacking up to 120 times. She gains Splinters of Wrath from the following:
- Dealing two instances of ability damage to an enemy champion within 4 seconds grants 1@1; 2@11; 3@18 Splinters. This cannot occur on the same target more than once every few seconds.
- Ranking up an ability with any skill points beyond level 1 grants 5 Splinters, up to 85.
- Killing a large minion grants 1 Splinter.

Collecting a Splinter of Wrath from an enemy restores (mana) 20 / 25 / 30 / 35 / 40 / 48 / 56 / 64 / 72 / 80 / 95 / 110 / 125 / 140 / 155 / 175 / 195 / 215 mana. Splinters of Wrath are brought to **Syndra** from enemies over $0.6$ seconds.

At 120 Splinters of Wrath, **Syndra** achieves *Transcendence*, increasing her ability power by 15%.

***Syndra** can collect Splinters of Wrath from enemies even if she is at maximum stacks. These Splinters do not grant any stacks, but will still restore mana for *’Syndra**.*

**Notes:**

- **Syndra**’s abilities gain additional visual effects when they reach their Splinter of Wrath thresholds. Her spells will also gain a red colored tint outlining the VFX.
  - *Dark Sphere* will cause fractures to appear in the earth when it is used.

---

### Q: Dark Sphere

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 800 units |
| **Effect Radius** | 210 units |
| **Cost** | 40 / 45 / 50 / 55 / 60 Mana |
| **Cooldown** | 7 seconds |
| **Recharge** | 7 (Recharge timer for Dark Sphere with the Transcendent bonus) seconds |
| **Static Cooldown** | $1.25$ (Cooldown between casts for Dark Sphere with the Transcendent bonus) |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |
| **Out of Range Behavior** | cast at max |

**ACTIVE:** **Syndra** conjures a *Dark Sphere* at the target location that appears after a $0.6$-second delay, dealing magic damage to nearby enemies. The *Dark Sphere* then remains on the ground for 6 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 75 / 110 / 145 / 180 / 215 (+ 60% AP) |

** Collecting 40 Splinters of Wrath causes **Syndra** to periodically stock a *Dark Sphere* charge, up to a maximum of 2.

**Notes:**

- *Dark Sphere* can be conjured inside terrain, but cannot be thrown there with Force of Will.
- There is no limit as to how many *Dark Spheres* can be active at a time.
- After **Syndra** gains the Transcendent bonus, the number of *Dark Sphere* charges available is represented by small Sphere icons under her health bar, visible to the player only.

---

### W: Force of Will

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 925 (Grab range) / 950 (Throw range) / 500 (Nearest target grab range) units |
| **Effect Radius** | 225 units |
| **Cost** | 60 / 70 / 80 / 90 / 100 Mana |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Unit / Location |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |
| **Projectile** | False |
| **Out of Range Behavior** | Walk in range of the target unit to cast (first cast)

Walk in range of the target location to cast (recast) |

**ACTIVE:** **Syndra** grabs the nearest *Dark Sphere*, enemy minion or non-epic monster within 500 units from the target location, and holds the target for 5 seconds. If a *Dark Sphere* is grabbed, its duration is refreshed; if a minion or monster is grabbed, it is put in stasis. She prioritizes grabbing the nearest unit, then *Dark Spheres*.

*Force of Will* can be recast while the target is being held.

**RECAST:** **Syndra** throws the grabbed target towards the target location, granting sight of their surroundings as they fall and dealing magic damage to them and nearby enemies. All targets hit are slowed by 25% for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 105 / 140 / 175 / 210 (+ 65% AP) |

** Collecting 60 Splinters of Wrath enhances *Force of Will* to deal (true damage) 12% (+ 2% per 100 AP) **bonus** true damage.

| Attribute | Value |
|-----------|------:|
| **Bonus Damage** | 8.4 / 12.6 / 16.8 / 21 / 25.2 (+ 9.2 / 9.9 / 10.6 / 11.3 / 12% AP) |
| **Total Mixed Damage** | 78.4 / 117.6 / 156.8 / 196 / 235.2 (+ 74.2 / 74.9 / 75.6 / 76.3 / 77% AP) |

**Notes:**

- The initial cast does not count as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive. **Detonating the ability manually does.
- If the ability is not recast within the duration, **Syndra** will release the unit or *Dark Sphere* that she is holding.
- *Force of Will* does not destroy in-flight projectiles for the minion or monster grabbed.
- *Force of Will* cleanses the target minion or monster grabbed from all crowd control.
- *Force of Will* prevents non-sphere targets from dying when picking them up.
- *Force of Will* cannot grab units that are being Teleport on.
- **Syndra** can grab pets, such as Jack in the Box.
- Casting Teleport or Recall will cause **Syndra** to cancel *Force of Will* autonomously and drop anything she is holding.
- Grabbing prioritizes *Dark Spheres*.
- Enemy pets that passively have an effect on their surroundings will continue to affect the area around them while they are being held. For example, Summon: Tibbers will continue to inflict burn damage on surrounding units while being held.
- Grabbing a Dark Sphere will cause the duration on it to refresh, but only on the first cast, not when it is thrown.
- *Force of Will* has additional effects when targeting the Blue Sentinel and Red Brambleback. Throwing the former will refund 10 mana to **Syndra** upon cast and will reduce the cooldown of *Force of Will* by 1 second, while throwing the latter will apply the Crest of Cinders burn to any affected units in the area of effect.
- *Force of Will*’s first cast uses a modified icon while it has the Transcendent bonus.

---

### E: Scatter the Weak

| Attribute | Value |
|-----------|------:|
| **Range** | 700 / 800 (Dark Spheres) / 1300 (Maximum push range of 1200 with 100 bonus hit range of Spheres from Syndra's location at the time they are hit) units |
| **Cast Time** | $0.25$ seconds |
| **Collision Radius** | 70 (Dark Sphere collision radius) units |
| **Angle** | 56° / 84° |
| **Width** | 120 (Individual missiles making up the cone) / 200 (Pushed spheres missile width) units |
| **Speed** | 2500 (Visible cone missiles. 2 more waves of invisible cone missiles are spawned at 1600 and 1100 speed which hit only spheres spawned into the area late) / 2000 (Pushed spheres missile speed) units/second |
| **Cost** | 50 Mana |
| **Cooldown** | 17 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |
| **Projectile** | Special |

**ACTIVE:** **Syndra** propels a wave of force in a cone in the target direction that deals magic damage to enemies hit and knocks them back for 400 units and up to 800 units away from **Syndra** based on proximity, though not through terrain.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 95 / 130 / 165 / 200 (+ 60% AP) |

*Dark Spheres* can be knocked back for 950 units and up to 1200 units away from **Syndra** based on proximity, knocking back enemies they hit over 70 units, though not through terrain. Targets hit are also stunned for $1.25$ seconds, during which they are also revealed, and dealt *Scatter the Weak*’s damage if they were not damaged by the initial cast.

** Collecting 80 Splinters of Wrath enhances *Scatter the Weak* to have an increased scatter angle and slow enemies hit by 70% for $1.25$ seconds starting after the knock back ends or if the target was hit by a pushed *Dark Sphere*, after the stun has ended.

**Notes:**

- *Scatter the Weak* is blocked by the portion of Wind Wall it collides with. *Spheres* thrown will stop upon colliding with the wall.
- *Spheres* spawned at close range of the ability are more likely to be hit.
- Enemies and spheres will be knocked into the direction directly away from **Syndra**, and the knock back has no dispersion.
- *Scatter the Weak* will knock back *Spheres* even farther if they are inside terrain, up until they reach open space. If the terrain they're inside in cannot be surpassed, the *Spheres* will be knocked in the opposite direction.
- *Scatter the Weak* will delay the expiration of a *Sphere* until after it finishes being knocked back. Effect at cast time end
- Once *Scatter the Weak* has been learned, an arrow directly in front of every Dark Sphere will appear to signify the direction the *sphere* will be knocked towards, each one corresponding to **Syndra**’s position. This is only visible to **Syndra**.

---

### R: Unleashed Power

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 675 units |
| **Effect Radius** | 3000 (Dark Sphere grab range) units |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 110 / 100 / 90 / 80 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Projectile** | True |
| **Call For Help** | True |

**PASSIVE:** *Dark Sphere* gains ability haste.

| Attribute | Value |
|-----------|------:|
| **Ability Haste** | 10 / 15 / 20 / 25 / 30 |

**ACTIVE:** **Syndra** enters a cast time, creating *3 Dark Spheres* and grabbing up to 4 nearby *Dark Spheres* that were last spawned. Afterwards, the collected *Dark Spheres* are barraged at the target enemy champion, each dealing magic damage upon hit.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Sphere** | 90 / 110 / 130 / 150 / 170 (+ 20% AP) || Attribute | Value |
|-----------|------:|
| **Minimum Magic Damage** | 270 / 330 / 390 / 450 / 510 (+ 60% AP) |
| **Maximum Magic Damage** | 630 / 770 / 910 / 1050 / 1190 (+ 140% AP) |

The *Dark Spheres* will then remain on the ground for 6 seconds.

** Collecting 100 Splinters of Wrath enhances *Unleashed Power* to execute the target if it would damage them to below 15% of their **maximum** health.

**Scaling:**
| Attribute | Value |
|-----------|------:|
| **Minimum Magic Damage** | 270 / 330 / 390 / 450 / 510 (+ 60% AP) |
| **Maximum Magic Damage** | 630 / 770 / 910 / 1050 / 1190 (+ 140% AP) |

**Notes:**

- *Unleashed Power* makes **Syndra** enter a brief cast time, but the initial cast is completed instantly. If she dies during this cast time, *Unleashed Power* will still complete the remaining effect.
- Casting *Unleashed Power* on a target that becomes untargetable while the *spheres* are in flight will not destroy them but cause them to deal no damage instead.
- *Spheres* already in flight will still travel if **Syndra**’s target dies, but any remaining *spheres* will not. However, their duration will still get reset.
- A *Dark Sphere* summoned very shortly before *Unleashed Power* **will** be used as one of the spheres that get thrown.
  - Depending on how many *spheres* are on the map and how far away the enemy is upon casting *Unleashed Power* it is possible that **Syndra** would conjure the *sphere* behind her as if she was going to throw it, but she won't.
- *Unleashed Power* will not add a *Sphere* that **Syndra** is holding with Force of Will.
- *Spheres* that are being pushed by Scatter the Weak (normally ones that were made late during the push) will not get picked up for *Unleashed Power*.
- Spell shield will only block the damage of a single *sphere*.

---

## Patch History

### V25.18
- General
  - **Bug Fixes:** Radial VFX no longer unintentionally disappears after some time.
    - The bug still persists when at 100 Splinters of Wrath or above.

### V25.08
- Force of Will
  - Slow reduced to 25% at all ranks from 25 / 30 / 35 / 40 / 45%.
- Scatter the Weak
  - Base damage reduced to 60 / 95 / 130 / 165 / 200 from 75 / 115 / 155 / 195 / 235.
  - AP ratio increased to 60% AP from 45% AP.
- Unleashed Power
  - Base damage per sphere reduced to 90 / 130 / 170 from 100 / 140 / 180.
  - AP ratio per sphere increased to 20% AP from 17% AP.

### V25.S1.3#February 6th Hotfix|V25.S1.3
- Force of Will
  - **UNDOCUMENTED / BUG FIX:** Casting now once again properly consumes the mana cost.

### V14.24
- Scatter the Weak
  - **Bug Fixes:** No longer fails to apply its damage against certain epic monsters.

### V14.22
- Force of Will
  - Base damage reduced to 70 / 105 / 140 / 175 / 210 from 70 / 110 / 150 / 190 / 230.
  - AP ratio reduced to 65% AP from 70% AP.

### V14.16
- Unleashed Power
  - Base damage per sphere increased to 100 / 140 / 180 from 90 / 130 / 170.

### V14.12
- Scatter the Weak
  - **Bug Fixes:** Is no longer able to apply the knockback and move Rek'Sai’s placed Tunnels.
  - **Bug Fixes:** Dark Sphere combo is now properly nullified by Hallowed Mist.

### V13.24#December 12th Hotfix|V13.24
- Transcendent
  - **Bug Fixes:** Now properly grants Syndra her ability power bonus when resurrected.
- Dark Sphere
  - AP ratio reduced to 60% AP from 70% AP.

### V13.24
- General
  - New splash artwork for Syndra.

### V13.22
- Stats
  - Base attack speed increased to $0.658$ from $0.625$.

## Trivia

- Syndra is the first champion to directly manipulate minions and neutral monsters to deal damage to other champions with Force of Will. The only other champion to do so is Sion with Roar of the Slayer.
- Syndra's spheres appear as red instead of blue when on the opposing team. She is the fourth champion to have team perspective based particles, with the third being Draven, the second being Cassiopeia and the first being Singed.
  - On her Atlantean and Snow Day skins, they appear purple to enemy instead.
- Using Force of Will to hit an enemy unit with the Red Brambleback will apply the debuff on them. Also, if Syndra throws the Blue Sentinel, she will be refunded 10 mana and the ability's cooldown will be lowered by 1 second.
- Syndra is the third of the dark-themed Ionian Champions. The first two being Shen & Varus, followed by Zed, Jhin, Xayah, & Kayn.
- Syndra's imprisonment under the Dream Pool shares some similarities with the Arthurian lore about Lady of the Lake; though Syndra herself is the prisoner, not the jailer.
- Syndra's dance references the choreography for the song *Hoot_(song)* by *Girls' Generation* (소녀시대: So Nyeo Shi Dae), a popular Korean girl group.
  - A side-by-side comparison can be seen here.
- Syndra was the third champion to have a difficulty rating of 100, the first being Cassiopeia, the second being Orianna, and the fourth being Draven. However, her difficulty rating has since fallen to 80. On release, her difficulty rating was 90.
- Each of Syndra's abilities gain new graphical details when leveled to max rank. For details, see below in her Skin section.
- Syndra is the second champion speculated to be **omnipotent**, or nearly so. The first one was Xerath.
- As seen in this video, Syndra appears to be able to use the balls from another Syndra.
- The fact that Unleashed Power can use up to seven balls is possibly a reference to the *Dragon Ball* franchise as there are seven Dragon Balls.
- Syndra is the second of three dark themed champions to have a light themed skin, with Varus being the first and Aatrox being the third.
- Syndra derives from Proto-Germanic **az* ("of/by/for one self"; whence English *sunder*), with possible influence from unrelated Greek *syndrome*.
  - Her brother's name, *Evard*, likewise combines two Germanic elements *īhwaz* "Taxus" & *wardaz* "guard".

---
*This page was automatically generated from League of Legends Wiki data.*