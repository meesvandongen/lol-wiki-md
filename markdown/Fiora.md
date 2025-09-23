# Fiora

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
| **Champion** | Fiora |
| **Title** | the Grand Duelist |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2012-02-29 |
| **Release Patch** | V1.0.0.135 |
| **Latest Changes** | V25.16 |
| **Roles** | Skirmisher |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 2 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 2 |
| **Style** | 20 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $620.0$ | $+99.0$ |
| **Mana** | $300.0$ | $+60.0$ |
| **Health Regen** | $8.5$ | $+0.55$ |
| **Mana Regen** | $8.0$ | $+0.7$ |
| **Armor** | $33.0$ | $+4.7$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $66.0$ | $+3.3$ |
| **Attack Speed** | $0.690$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $150.0$ | $+0.0$ |
| **Base Attack Speed** | $0.69$ | |
| **Attack Speed Ratio** | $0.69$ | |
| **Bonus AS per Level** | $3.2\%$ | |
| **Attack Windup** | $13.8\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $100$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Taken** | $95.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $110.0\%$ |
| **Healing** | $65.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $105.0\%$ |

## Abilities

### Passive: Duelist's Dance

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 1200 (If Fiora is visible) er 1350 (Distance for remaining 'near' the target) / er 250 (If Fiora is out of vision, estimated) units |
| **Angle** | 90° |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | True |
| **Spell Shield** | False |
| **Spell Effects** | Proc |
| **Parry** | True |

**INNATE:** **Fiora** identifies the Vitals of nearby visible enemy champions, marked as an arc around them in North-, East-, South- or Westward direction. Vitals take $1.75$ seconds to become targetable and linger for $13.25$ seconds afterwards while **Fiora** remains near the target, otherwise the Vital will disappear. After a Vital ends, **Fiora** identifies a new one on her target.

Dealing damage in the direction of a Vital will trigger it to deal **bonus** true damage equal to 3% (+ 4% per 100 **bonus** AD) of target's **maximum** health, heal **Fiora** for 35 to 100, and grant her type=[File:Grand Challenge.png ms that decays over $1.85$ seconds.

**Notes:**

- Vitals can be triggered by any source of damage **Fiora** deals, except those with default damage and reactive damage types.
- The Vitals are visible to all units.
- A Vital's angle is fixed to the champion's model and doesn't interact with its orientation.
- Vitals spawn directions alternate in a Northeast-Southwest pattern. Other than that, they have a 50% chance to spawn North or East, or South or West, respectively.
  - The direction of the first Vital upon **Fiora** and her enemy encountering one another in the game for the first time is always South- or Westward. This is regardless of whether Fiora is in the Blue or Red team.
- Vitals will not trigger if the attack is dodged or missed while **Fiora** is blinded.
  - Block will not prevent a trigger.
  - If the Vital is triggered by Lunge, it can be dodged, but it can't be blocked and will trigger even if **Fiora** is blinded.

---

### Q: Lunge

| Attribute | Value |
|-----------|------:|
| **Range** | 420 (Stab range) units |
| **Cast Time** | none |
| **Target Range** | 50 (Minimum dash range) / 400 (Maximum dash range) units |
| **Speed** | See notes |
| **Cost** | 20 Mana |
| **Cooldown** | 13 / 11.25 / 9.5 / 7.75 / 6 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Out of Range Behavior** | cast at max |
| **Parry** | Special |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Fiora** dashes in the target direction, then stabs a nearby enemy. *Lunge* can hit [structures and wards. Stabbing a target reduces *Lunge*’s cooldown by 50%.

The stab deals physical damage and applies on-hit effects.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 70 / 80 / 90 / 100 / 110 (+ 90 / 95 / 100 / 105 / 110% **bonus** AD) |

***Fiora** can cast any of her abilities during the dash. The target does not have to be visible to be hit by this ability, unless it is a ward.*

**Notes:**

- *Lunge*’s dash speed is modified by her **bonus** movement speed at the time of cast:
  - 1100 + 350% **bonus** movement speed, capped at 1600 maximum dash speed if bonus movement speed is positive.
  - 1100 + 300% **bonus** movement speed, capped at 850 minimum dash speed if bonus movement speed is negative.
- **Fiora** does not stab if the dash is interrupted.
  - Flash gets sealed during the dash so **Fiora** cannot interrupt herself with it. This also means she cannot buffer *Flash* during the dash for it to be cast right afterward, however.
  - **Fiora** can cast Hextech Rocketbelt during the dash, potentially interrupting it based on the timing, preventing the stab if it does.
- The dash **can** pass through walls.
  - The dash has no forgiveness mechanic to clear distant or wider walls with. Any wall wider than 400 units is not crossable.
- The stab searches for targets in its radius and prioritizes in order of:
  - Nearest enemy champion affected by Grand Challenge with a Vital facing **Fiora**.
  - Nearest enemy champion Vital facing **Fiora**.
  - Nearest enemy champion affected by Grand Challenge
  - Enemy champion with the lowest percent health out of all champions <20% remaining health.
  - Nearest enemy champion, minion, or monster within 60% of the radius (cr 252) with current health below *Lunge*’s raw damage.
  - Nearest enemy champion in the radius check.
  - Nearest champion in a er rectangular check with 160 width and 450 units length backwards from **Fiora**’s current facting direction, ignoring the normal radius check.
    - Turning **Fiora**’s facing direction during the dash (e.g. by casting Riposte) can create some very long range hits opposite to her new facing direction, especially against targets with a large gameplay radius.
  - Nearest enemy minion with current health below *Lunge*’s raw damage.
  - Any nearest enemy minion or monster.
  - Structures.
  - Visible wards.
- *Lunge* will not attempt to strike jungle plants.
- *Lunge* deals basic damage, but also triggers spell effects by also being tagged as spell damage.
  - *Lunge* cannot critically strike.
- *Lunge* won't deal damage, if the spell is dodged or blocked, but will deal damage if **Fiora** is blinded.
- *Lunge* does not apply on-attack effects apart from certain exceptions.
  - It does apply Energized as if it was an on-hit effect.
  - It does not stack Guinsoo's Rageblade nor trigger Phantom Hit.

---

### W: Riposte

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 900 units |
| **Width** | 140 units |
| **Speed** | 3200 units/second |
| **Cost** | 50 Mana |
| **Cooldown** | 24 / 22 / 20 / 18 / 16 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Fiora** enters a defensive stance for $0.75$ seconds, during which she is unable to act, prevents all incoming non-turret damage, and gains debuff immunity and crowd control immunity.

Additionally, over the first $0.5$ seconds of the duration, **Fiora** poises to strike, after which she sends a shock with her sword in a line in the target direction, dealing magic damage to all enemies hit until colliding with an enemy champion.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 110 / 150 / 190 / 230 / 270 (+ 100% AP) |

The enemy champion struck is also slowed and crippled by 25% for 2 seconds. If *Riposte* negates at least one hostile (Belonging to an enemy source) immobilizing effect, **Fiora** stuns the target for the same duration instead.

**Notes:**

- *Riposte* is not invulnerability but instead reduces any damage to her by 100% late into the damage event. Because of this, it interacts differently with effects triggered from damage:
  - Life steal and Healing based on damage dealt (Vamp) will still have effect even if the damage is ultimately ignored by *Riposte*.
  - Shields will absorb damage before *Riposte* takes effect.
  - **Fiora** cannot be executed by Death from Below during *Riposte*, but she may by Aspect of the Dragon.
- *Riposte* will not negate nearsight.
- Immobilizing effects include those from Blast Cone (only if triggered by an enemy) and epic monsters.
- (Non-crowd-control) debuff immunity encompasses debuffs from summoner spells such as Exhaust’s damage reduction and all of Ignite’s effects, as well as persistent damage and other effects.
- In the last $0.25$ seconds of *Riposte*’s duration, **Fiora** cannot be selected. This is distinct from untargetability and there are no known interactions beyond this.
- *Riposte*’s effect prioritization is as follows:
  - *Riposte* will negate effects before spell shields.
  - Black Shield will negate crowd control before *Riposte*.
    - Being hit by a crowd control while affected by Black Shield means *Riposte* will not trigger its stun empowerment condition, as it itself must negate the immobilize and not any other external source.
- Spell shield will also prevent Vitals from being triggered by *Riposte*’s shock.
- The shock missile will be fired from wherever **Fiora** is after the first $0.5$ seconds of *Riposte*.
- The following table refers for interactions while **Fiora** is performing *Riposte*:

---

### E: Bladework

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 40 Mana |
| **Cooldown** | 11 / 10 / 9 / 8 / 7 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spell |
| **Parry** | Special |

**ACTIVE:** **Fiora** empowers her next two basic attacks on-attack within 4 seconds to gain range and **bonus** attack speed.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 50 / 60 / 70 / 80 / 90% |

The first attack slows the target by 30% for 1 second but cannot critically strike. The second attack will critically strike for critical damage modifiers, including to structures.

| Attribute | Value |
|-----------|------:|
| **Critical damage** | 160 / 170 / 180 / 190 / 200% |

*Bladework resets **Fiora**’s basic attack timer.*

**Notes:**

- Spell shield will only block the slow from the first hit.
- Sundered Sky Lightshield Strike's critical damage will override *Bladework*’s if the resulting damage would be higher.
  - This does not account for any critical damage bonuses that may affect Lightshield Strike.
- *Bladework*’s first basic attack will slow even if it is dodged, blocked or missed.
- Neither *Bladework* attack will deal damage if it is dodged, blocked or missed.

---

### R: Grand Challenge

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 500 units |
| **Effect Radius** | 550 (Movement speed radius, includes target radius) / 500 (Heal zone radius, does not include target radius) units |
| **Cost** | 100 Mana |
| **Cooldown** | 110 / 100 / 90 / 80 / 70 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Spell Shield** | False |

**PASSIVE:** *Duelist's Dance’s* ms is increased.

| Attribute | Value |
|-----------|------:|
| **Additional Bonus Movement Speed** | 10 / 15 / 20 / 25 / 30% |

**ACTIVE:** **Fiora** challenges the target enemy champion for 8 seconds, highlighting all four of their Vitals after a $0.5$-second delay. While in effect, *Duelist's Dance* does not identify new Vitals on **Fiora**’s target.

While near the target, **Fiora** gains *Duelist's Dance’s* ms.

If **Fiora** triggers at least one Vital before the target dies, or triggers all four, she creates a *Victory Zone* at their location for 5 seconds, which heals **Fiora** and all allies within the area every $0.25$ seconds.

| Attribute | Value |
|-----------|------:|
| **Heal per Tick** | 18.75 / 21.875 / 25 / 28.125 / 31.25 (+ 15% **bonus** AD) |
| **Heal per Second** | 75 / 87.5 / 100 / 112.5 / 125 (+ 60% **bonus** AD) |
| **Maximum Heal** | 375 / 437.5 / 500 / 562.5 / 625 (+ 300% **bonus** AD) |

*Unlike Duelist's Dance, the Vitals will continue to linger even if **Fiora** is not nearby.*

**Notes:**

- *Grand Challenge* is dispelled instantly if targeting a decoy.
  - *Grand Challenge* can't be cast on Glory in Death.
- The damage of this ability scales with Duelist's Dance.
- Even if **Fiora** dies, the *Victory Zone* will trigger so long as she hit at least one Vital and they die before the Vitals wear off.
  - Resurrection effects will not prevent the activation of the *Victory Zone*.
- The Victory Zone will not heal untargetable units.
- **Fiora** will automatically attack the target upon casting if the target was selected while they weren't in range of the ability.

---

## Patch History

### V25.16
- Grand Challenge
  - **Bug Fixes:** *Victory Zone* no longer fails to activate if the marked enemy dies from sufficient damage by a Spellblade effect.

### V25.14
- Grand Challenge
  - *Victory Zone* heal rate increased to once every $0.25$ seconds from once every $0.5$ seconds.
  - **Bug Fixes:** *Victory Zone* VFX no longer disappears prematurely, lasting for 2 / 3 / 4 / 5 seconds instead of the proper 5.
    - *[Note: This did not prevent the actual healing from taking place, which always lasts for the full 5 seconds.]*
  - **Removed:*** *Victory Zone* heal per second is no longer modified to 40 to 100.
    - **New:*** Now heals for 75 / 100 / 125 (+ 60% **bonus** AD), **per second**.
      - 375 / 500 / 625 (+ 300% **bonus** AD) **total, over 5 seconds**.
    - **OLD:** Previously healed for <!--

### File:Fiora Grand Challenge.png|20px|link=|An icon representing Fiora's Grand Challenge
      - pp|75*(1+x)|1 to 4 by 1|type=Vitals hit|label=Total base heal|showtype=false|changedisplay=true|formula=(

### V25.06
- Lunge
  - Mana cost reduced to 20 at all ranks from 20 / 25 / 30 / 35 / 40.

### V25.04
- Riposte
  - **Bug Fixes:** Now properly mitigates various Mel abilities.

### V14.9
- Riposte
  - **Bug Fixes:** Ixtal's Impact now properly triggers the ability's stun condition.
- Bladework
  - **New Effect:** Now defaults to the highest damage effect between Sundered Sky and *Bladework*’s crit attack when they are both active at the same time.

### V14.7
- Stats
  - Base attack damage reduced to 66 from 68.

### V14.5
- Bladework
  - **New Effect:** Now triggers spell effects upon dealing damage.

### V14.4
- Riposte
  - Cripple strength reduced to 25% from 50%.

### V14.1
- Fiora, Fiora, Fiora, and Fiora
  - Duelist's Dance
    - **Bug Fixes:** Indicator visibility has been updated and aligns with her base indicator visibility.

## Trivia

- Fiora is voiced.md) by Karen Strassman, who also voices Cassiopeia, Shyvana and Zyra.
  - Karen Strassman also voices her in Legends of Runeterra.
- Fiora's fighting style references Fencing while she herself might have been inspired by Julie d'Aubigny.
  - Her taunt references the 'salute' from duel tradition.
- In her joke, Fiora draws Teemo’s face in the air.
- Her dance references Fred Astaire to the tune of Puttin' On the Ritz in Blue Skies (1946 film).
  - A side-by-side comparison can be seen here.
- *Fiora* Sabines theonym Flora (mythology), from Proto-Italic feminine suffix *-a*.
  - Her name might be referencing Fiore dei Liberi and/or Foil (fencing)
- Her house's name is a reference to Nicolo Laurent, former CEO of Riot Games. Alex 'Skribbles' Yee chose his surname without even telling him and because he was "*the only french person he knows*".
- Riposte and Fiora's Riposte/Riposte are named after the Riposte, but its effect is more akin to a Parry (fencing), alongside a Riposte.

---
*This page was automatically generated from League of Legends Wiki data.*