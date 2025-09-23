# Malzahar

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
| **Champion** | Malzahar |
| **Title** | the Prophet of the Void |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2010-06-01 |
| **Release Patch** | V1.0.0.86 |
| **Latest Changes** | V25.04 |
| **Roles** | Battlemage |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 1 |
| **Hero Type** | Mage |
| **Alt Type** | Assassin |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 2 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $580.0$ | $+101.0$ |
| **Mana** | $375.0$ | $+28.0$ |
| **Health Regen** | $6.0$ | $+0.6$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $18.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $55.0$ | $+3.0$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $500.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $1.5\%$ | |
| **Missile Speed** | $2000$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $302.778$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $90.0\%$ |
| **Damage Taken** | $110.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $92.0\%$ |
| **Damage Taken** | $108.0\%$ |

## Pets

### Voidling

| Attribute | Value |
|-----------|------:|
| **Range** | 100 units |
| **Gold** | 2 |
| **Experience** | 0 |
| **Health** | 65+8*(x-1)*(0.7025+0.0175*(x-1)) |
| **Armor** | 16+2*(x-1)*(0.7025+0.0175*(x-1)) |
| **Magic Resist** | 8+1*(x-1)*(0.7025+0.0175*(x-1)) |
| **Damage** | 5+3.5*(x-1)*(0.7025+0.0175*(x-1)) (+ 40% **bonus** AD) (+ 20% AP) |
| **Damage Type** | magic |
| **Attack Speed** | 0.665*(1+0.02*(x-1)*(0.7025+0.0175*(x-1))) attack speed |
| **Move Speed** | 400 |
| **Control** | Prioritize attacking Nether Grasp’s target and enemies (champions have a higher priority than minions) infected by Malefic Visions, or the nearest target. |
| **Targeting** | Minion, does not count towards the minion kill tracking score |
| **Spell Effects** | *Voidlings' * basic attacks apply spell effects as area damage. |
| **On-Hit** | *Voidlings' * attacks can be mitigated by block, dodge, and blind, as well as stopped by Disarm. |

**Abilities:**

- **Void Essence:** The Voidling deals 50% reduced damage to epic monsters and 200% increased damage to enemy minions infected by Malefic Visions.

---

## Abilities

### Passive: Void Shift

| Attribute | Value |
|-----------|------:|
| **Static Cooldown** | 30@1; 24@6; 18@11; 12@16 |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE:** Periodically, **Malzahar** gains *Void Shift* until he takes non-minion damage or negates a crowd control effect, to which it then lingers for $0.25$ seconds before expiring. 

**VOID SHIFT:** **Malzahar** gains crowd control immunity and 90% damage reduction.

*Void Shift*’s cooldown refreshes whenever **Malzahar** takes non-minion damage or is affected by a crowd control effect, and resets upon respawning.

**Notes:**

- *Void Shift* will not resist allied crowd control nor will it be deactivated by them.
- *Void Shift* will not mitigate true damage but the buff will still be consumed.
  - It does not deactivate against the Nexus Obelisk.
- Shields will take priority over *Void Shift*, but the buff will still be consumed (does not reduce damage inflicted). *Void Shift* triggers even if the shield negated the damage entirely.
  - However, if **Malzahar** has a spell shield and the shield fully mitigates the damage, *Void Shift* does not trigger. The damage taken must be from an effect that cannot be negated by *spell shields*.
- Spell shields take priority over *Void Shift*. If the triggering hostile ability does not contain a crowd control effect, both *Void Shift* and the *spell shield* break simultaneously.
  - Since *spell shields* are precedent over *Void Shift*, applied crowd control from effects that cannot be blocked by *spell shields* (i.e. Wall of Pain) will also ignore *Void Shift* despite its crowd control immunity. The effect must not damage **Malzahar** for this event to take place.
- *Void Shift* will specifically not deactivate if **Malzahar** eats Honeyfruit or is within a hostile Frozen Heart passive aura.
- *Void Shift* will prevent Chum the Waters from attaching to him.

---

### Q: Call of the Void

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 900 (Cast range) units |
| **Effect Radius** | 200 units |
| **Cost** | 60 / 65 / 70 / 75 / 80 Mana |
| **Cooldown** | 6 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Malzahar** opens two portals to the void centered at the target location, granting sight of the area in between. After $0.4$ seconds, enemies between the portals are dealt magic damage and silenced for a duration.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 105 / 140 / 175 / 210 (+ 55% AP) |

| Attribute | Value |
|-----------|------:|
| **Silence Duration** | 1 / 1.25 / 1.5 / 1.75 / 2 seconds |

**Notes:**

- Spell shield will prevent *Call of the Void*’s refresh on Malefic Visions.
  - Malefic Visions’s debuff will also be removed.

---

### W: Void Swarm

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 150 units |
| **Cost** | 40 / 45 / 50 / 55 / 60 Mana |
| **Cooldown** | 8 seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | cast at max |

**PASSIVE:** **Malzahar** generates a stack of *Zz'Rot Swarm* when he casts another ability, up to a maximum of 2.

**ACTIVE:** **Malzahar** consumes all *Zz'Rot Swarm* stacks and, after a $0.5$-second delay, summons a Voidling at the target location. Additional *Voidlings* are then summoned for each *Zz'Rot Swarm* stack consumed, with each *Voidling* being summoned $0.5$ seconds after the previous one.

| Attribute | Value |
|-----------|------:|
| **Voidling Duration** | 8 / 8 / 9 / 9 / 10 seconds |

*Voidlings* deal magic damage with their attacks, reduced by 50% against epic monsters and increased by 200% to enemy minions infected by *Malefic Visions*.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 5+3.5*(x-1)*(0.7025+0.0175*(x-1)) (+ 12 / 14 / 16 / 18 / 20) (+ 40% **bonus** AD) (+ 20% AP) |
| **Minion Damage** | (5+3.5*(x-1)*(0.7025+0.0175*(x-1)))*3 (+ 36 / 42 / 48 / 54 / 60) (+ 120% **bonus** AD) (+ 60% AP) |

*See [Pets](#Pets) for more details about Voidlings.*

**Notes:**

- **Malzahar** loses all stacks upon death.
- *Zz'Rot Swarm* stacks are gained per cast and not per ability activation. For example, if Malefic Visions is cast and the ability cancels under certain circumstances, a *Zz'Rot Swarm* stack will still be granted despite the ability's failure to complete.

---

### E: Malefic Visions

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 650 units |
| **Effect Radius** | 500 (Bounce range) units |
| **Cost** | 60 / 70 / 80 / 90 / 100 Mana |
| **Cooldown** | 11 / 10 / 9 / 8 / 7 seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | DoT |
| **Projectile** | False |
| **Call For Help** | True |

**ACTIVE:** **Malzahar** infects the target enemy's mind, dealing magic damage every $0.25$ seconds over 4 seconds, which refreshes upon damaging them with *Call of the Void* or *Nether Grasp*. *Malefic Visions* executes minions if they would be damaged below health.

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 80 / 115 / 150 / 185 / 220 (+ 80% AP) |
| **Magic Damage Per Tick** | 5 / 7.1875 / 9.375 / 11.5625 / 13.75 (+ 5% AP) |

If the target dies while infected, they spread *Malefic Visions* onto the closest nearby enemy and **Malzahar** restores mana.

**Notes:**

- No additional notes.

---

### R: Nether Grasp

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.005$ seconds |
| **Target Range** | 700 units |
| **Tether Radius** | 1250 units |
| **Cost** | 100 Mana |
| **Cooldown** | 140 / 125 / 110 / 95 / 80 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | False |
| **Call For Help** | True |
| **Silence** | True |

**ACTIVE:** **Malzahar** tethers to the target enemy champion, knocking them down. He then channels for up to $2.5$ seconds, suppressing and revealing the target and dealing them magic damage every $0.25$ seconds, revealing himself in the process.

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 125 / 162.5 / 200 / 237.5 / 275 (+ 80% AP) |
| **Magic Damage Per Tick** | 12.5 / 16.25 / 20 / 23.75 / 27.5 (+ 8% AP) |

***Malzahar** will continue to channel as long as the tether is not broken, even if the suppression is removed.*

Additionally, a *Null Zone* is opened beneath the target's location at the time of cast that lasts 5 seconds, dealing magic damage every $0.5$ seconds to enemies within, capped at 120 per tick against minions and monsters.

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 10 / 12.5 / 15 / 17.5 / 20% (+ $2.5$% per 100 AP) of target's **maximum** health |
| **Magic Damage Per Tick** | 1 / 1.25 / 1.5 / 1.75 / 2% (+ $0.25$% per 100 AP) of target's **maximum** health |

*Null Zone will persist even if the channel is interrupted.

**Notes:**

- If a target is currently being affected by *Malefic Visions*, casting *Nether Grasp* on them will grant 2 stacks toward effects such as Electrocute or Phase Rush.
  - This is, correctly, not the case if *Nether Grasp* is cast before *Malefic Visions*.
- Applies persistent damage for the tether and persistent area damage to enemies within the *Null Zone*.
- The tether will not break if the target enters a zombie state or enters resurrection.
- The tether will continue to damage the target for its full duration even if it is broken early.
- Removing the suppression will also remove the reveal.
- **Malzahar** places himself onto the ground and interrupts displacements affecting him upon starting the channel.
- The following table refers for interactions while **Malzahar** is channeling:

---

## Patch History

### V25.04
- Malefic Visions
  - **Bug Fixes:** No longer instantly kills Garden of Thorns.

### V25.S1.2
- Void Swarm
  - **New Effect:** Live Voidlings now gain the level scaling bonuses when **Malzahar** levels up.
  - Tooltip now notes the damage per rank and uses it when calculating the displayed Voidling damage numbers.
  - Tooltip now properly uses stat growth instead of linear interpolation when calculating the displayed Voidling damage numbers.
- Malefic Visions
  - **Bug Fixes:** No longer sometimes fails to trigger Bloodletter's Curse Vile Decay.

### V14.24
- Void Swarm
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.9
- Stats
  - Selection radius increased to 120 units from $88.8889007568359$.
- Call of the Void
  - Mana cost reduced to 60 / 65 / 70 / 75 / 80 from 80 at all ranks.
- Malefic Visions
  - Cooldown reduced to 11 / 10 / 9 / 8 / 7 seconds from 15 / 13 / 11 / 9 / 7.
- Malzahar
  - Nether Grasp
    - **Bug Fixes:** Tether VFX now fires from his mouth as intended.

### V13.19
- Malefic Visions
  - **Bug Fixes:** Now properly spreads the debuff even if it was cast on a minion as it was dying.

### V13.16
- Malefic Visions
  - Tick rate reduced to 1 per $0.25$ seconds from 1 per $0.5$.
  - Minion execute threshold reduced to 10 / 12 / 14 / 16 / 18 / 20 / 22 / 24 / 26 / 28 / 30 health from 15 / 18 / 21 / 24 / 27 / 30 / 33 / 36 / 39 / 42 / 45 health.

### V13.9
- Malefic Visions
  - **Bug Fixes:** Now properly spreads when it kills a minion.

### V13.4
- Malefic Visions
  - **Bug Fixes:** Now properly triggers Manaflow Band when it spreads to an enemy champion after having done so multiple times.

### V12.22
- Stats
  - Mana growth increased to 28 from $27.5$.

### V12.21
- Malefic Visions
  - **New Effect:** Now executes minions below 15 / 18 / 21 / 24 / 27 / 30 / 33 / 36 / 39 / 42 / 45 health.

## Trivia

- After the Battlemage update in V6.9, Malzahar's Voidlings had their color scheme changed from Voidling to gray with purple eyes.
- Malzahar is one of the few champions to have a pet, the others being Annie, Elise, Heimerdinger, Ivern, Shaco, and Yorick.
- Malzahar and his Voidlings’s dance references U Can't Touch This by MC Hammer.
  - A side-by-side comparison for Malzahar's part can be seen here.
  - A side-by-side comparison for his Voidlings’s part can be seen here.
    - He shares this dance (specifically his Voidlings’s part) with Jayce.
- Upon the removal of Zz'Rot Portal in patch V9.23, *Gathering Swarm* stacks for Void Swarm was renamed to *Zz'Rot Swarm* to honor the item.
  - This might indirectly imply that Malzahar's Voidling are a similar if not the same species as the Voidspawn that spawned from that item.
- Malzahar's Voidling had its assets reused for several rotating game modes.
  - It was given a retexture and features as a monster in Invasion.
  - Its animations were reused for the Voidling monster in Odyssey: Extraction.

---
*This page was automatically generated from League of Legends Wiki data.*