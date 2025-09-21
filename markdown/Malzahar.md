# Malzahar

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
| **Champion** | Malzahar |
| **Title** | the Prophet of the Void |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2010-06-01 |
| **Release Patch** | V1.0.0.86 |
| **Roles** | Battlemage |
| **Riot Positions** | Middle |
| **External Positions** | Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $580.0$ | $+101.0$ | $2297.0$ |
| **Mana** | $375.0$ | $+28.0$ | $851.0$ |
| **Health Regen** | $6.0$ | $+0.6$ | $16.2$ |
| **Mana Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Armor** | $18.0$ | $+4.7$ | $97.9$ |
| **Magic Resist** | $30.0$ | $+1.3$ | $52.1$ |
| **Attack Damage** | $55.0$ | $+3.0$ | $106.0$ |
| **Attack Speed** | $0.625$ | $+1.5\%$ | $0.784$ |
| **Movement Speed** | $335.0$ | $+0.0$ | $335.0$ |
| **Attack Range** | $500.0$ | $+0.0$ | $500.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.5\%$ |
| **Missile Speed** | $2000 units/second$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $302.778 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Void Shift

**Innate:** Periodically, **Malzahar** gains *Void Shift* until he takes non-minion damage or negates a crowd control effect, to which it then lingers for a brief time before expiring.

*'Void Shift's *cooldown* refreshes whenever **Malzahar** takes non-minion damage or is affected by a crowd control effect.*

**Innate:** Periodically, **Malzahar** gains *Void Shift* until he takes non-minion damage or negates a crowd control effect, to which it then lingers for $0.25$ seconds before expiring. **Void Shift:** **Malzahar** gains Cc-immune and 90% damage reduction. 'Void Shift's *cooldown* refreshes whenever **Malzahar** takes non-minion damage or is affected by a crowd control effect, and resets upon respawning.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- *Void Shift* will not resist allied crowd control nor will it be deactivated by them.
- *Void Shift* will not mitigate true damage but the buff will still be consumed.
  - It does not deactivate against the Nexus Obelisk.
- Shield will take priority over *Void Shift*, but the buff will still be consumed (does not reduce damage inflicted). *Void Shift* triggers even if the shield negated the damage entirely.
  - However, if **Malzahar** has a spell shield and the shield fully mitigates the damage, *Void Shift* does not trigger. The damage taken must be from an effect that cannot be negated by *spell shields*.
- Spell shield take priority over *Void Shift*. If the triggering hostile ability does not contain a crowd control effect, both *Void Shift* and the *spell shield* break simultaneously.
  - Since *spell shields* are precedent over *Void Shift*, applied crowd control from effects that cannot be blocked by *spell shields* (i.e. *Wall of Pain*) will also ignore *Void Shift* despite its cc-immune. The effect must not damage **Malzahar** for this event to take place.
- *Void Shift* will specifically not deactivate if **Malzahar** eats Honeyfruit or is within a hostile *Frozen Heart* passive aura.
- *Void Shift* will prevent *Chum the Waters* from attaching to him.

---

### Q: Call of the Void

**Active:** **Malzahar** opens two portals to the void at the target location that last a brief time, which then deal magic damage to enemies between and briefly silence them.

**Active:** **Malzahar** opens two portals to the void centered at the target location, granting sight of the area in between. After $0.4$ seconds, enemies between the portals are dealt magic damage and silence for a duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 900 units |
| **Cooldown** | 6 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-80$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 200 units |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |

**Scaling:**
- **Magic Damage:** $70-210$ (+ 55% AP)
- **Silence Duration:** $1-2$ seconds

**Notes:**

- Spell shield will prevent 'Call of the Void's refresh on *Malefic Visions*.
  - *Malefic Visions*’s debuff will also be removed.

---

### W: Void Swarm

**Passive:** **Malzahar** generates a stack of 'Zz'Rot Swarm' when he casts another ability, up to a maximum.

**Active:** **Malzahar** consumes all 'Zz'Rot Swarm* stacks to summon a Voidling at the target location. Additional *Voidlings* are then summoned for each *Zz'Rot Swarm' stack consumed.

**Passive:** **Malzahar** generates a stack of 'Zz'Rot Swarm' when he casts another ability, up to a maximum of 2. **Active:** **Malzahar** consumes all 'Zz'Rot Swarm* stacks and, after a $0.5$-second delay, summons a Voidling at the target location. Additional *Voidlings* are then summoned for each *Zz'Rot Swarm* stack consumed, with each *Voidling' being summoned $0.5$ seconds after the previous one. *Voidlings* deal magic damage with their attacks, reduced by 50% against epic monsters and increased by 200% to enemy minions infected by **Malefic Visions**. *See [Pets](#Pets) for more details about Voidlings.*

| Attribute | Value |
|-----------|-------|
| **Range** | 150 units |
| **Cooldown** | 8 seconds |
| **Cast Time** | none |
| **Cost** | $40-60$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Voidling Duration:** $8/8/9/9/10$ seconds
- **Magic Damage:** $12-20$ (+ 40% bonus AD)
- *bonus AD) (+ $20×3$% AP)

**Notes:**

- **Malzahar** loses all stacks upon death.
- 'Zz'Rot Swarm* stacks are gained per cast and not per ability activation. For example, if *Malefic Visions* is cast and the ability cancels under certain circumstances, a *Zz'Rot Swarm' stack will still be granted despite the ability's failure to complete.

---

### E: Malefic Visions

**Active:** **Malzahar** infects the target enemy's mind, continually dealing magic damage over a few seconds, which refreshes upon damaging them with **Call of the Void** or **Nether Grasp**.

*If the target dies while infected, they spread *Malefic Visions* onto the closest nearby enemy and **Malzahar** restores a portion of mana*maximum** mana*.*

**Active:** **Malzahar** infects the target enemy's mind, dealing magic damage every $0.25$ seconds over 4 seconds, which refreshes upon damaging them with **Call of the Void** or **Nether Grasp**. *Malefic Visions* executes minions if they would be damaged below health. If the target dies while infected, they spread *Malefic Visions* onto the closest nearby enemy and **Malzahar** restores mana*maximum** mana*.

| Attribute | Value |
|-----------|-------|
| **Range** | 650 units |
| **Cooldown** | $11-7$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $60-100$ Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 500 units |
| **Spell Shield** | True |
| **Spell Effects** | DoT |
| **Projectile** | False |

**Scaling:**
- **Total Magic Damage:** $80-220$ (+ 80% AP) Magic Damage Per Tick $80/16-220/16$ (+ $80/16$% AP)

**Notes:**

- No additional notes.

---

### R: Nether Grasp

**Active:** **Malzahar** tether to the target enemy champion, knockdown. The tether lasts a short time, during which he channel to suppression and true sight the target, continually dealing them magic damage.

*Additionally, a *Null Zone* is opened beneath the target's location for a few seconds, which continually deals magic damage to enemies within.*

**Active:** **Malzahar** tether to the target enemy champion, knockdown. He then channel for up to $2.5$ seconds, suppression and true sight the target and dealing them magic damage every $0.25$ seconds, sight himself in the process. **Malzahar will continue to channel as long as the tether is not broken, even if the suppression is removed.** Additionally, a *Null Zone* is opened beneath the target's location at the time of cast that lasts 5 seconds, dealing magic damage every $0.5$ seconds to enemies within, capped at 120 per tick against minions and monsters. 'Null Zone will persist even if the channel is interrupted.

| Attribute | Value |
|-----------|-------|
| **Range** | 700 units |
| **Cooldown** | $140-80$ seconds |
| **Cast Time** | $0.005$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | False |

**Scaling:**
- **Total Magic Damage:** $125-275$ (+ 80% AP) Magic Damage Per Tick $125/10-275/10$ (+ 8% AP)
- **Total Magic Damage:** $10-20$%

**Notes:**

- If a target is currently being affected by **Malefic Visions**, casting *Nether Grasp* on them will grant 2 stacks toward effects such as *Electrocute* or *Phase Rush*.
  - This is, correctly, not the case if *Nether Grasp* is cast before *Malefic Visions*.
- Applies persistent damage for the tether and persistent area damage to enemies within the *Null Zone*.
- The tether will not break if the target enters a zombie state or enters resurrection.
- The tether will continue to damage the target for its full duration even if it is broken early.
- Removing the suppression will also remove the true sight.
- **Malzahar** places himself onto the ground and interrupts airborne affecting him upon starting the channel.
- The following table refers for interactions while **Malzahar** is channel:

---

## Patch History

### V25.04
- *Malefic Visions*
  - **Bug Fixes:** No longer instantly kills Garden of Thorns.
- *Void Swarm*
  - **New Effect:** Live Voidlings now gain the level scaling bonuses when **Malzahar** levels up.
  - Tooltip now notes the damage per rank and uses it when calculating the displayed Voidling damage numbers.
  - Tooltip now properly uses stat growth instead of linear interpolation when calculating the displayed Voidling damage numbers.
- *Malefic Visions*
  - **Bug Fixes:** No longer sometimes fails to trigger *Bloodletter's Curse* Vile Decay.

### V14.24
- *Void Swarm*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.9
- Stats
  - Selection radius increased to 120 units from $88.8889007568359$.
- *Call of the Void*
  - Mana cost reduced to $60-80$ from 80 at all ranks.
- *Malefic Visions*
  - Cooldown reduced to $11-7$ seconds from $15-7$.
- Malzahar
  - *Nether Grasp*
    - **Bug Fixes:** Tether VFX now fires from his mouth as intended.

### V13.19
- *Malefic Visions*
  - **Bug Fixes:** Now properly spreads the debuff even if it was cast on a minion as it was dying.

### V13.16
- *Malefic Visions*
  - Tick rate reduced to 1 per $0.25$ seconds from 1 per $0.5$.
  - Minion execute threshold reduced to 10–30 health from 15–45 health.

### V13.9
- *Malefic Visions*
  - **Bug Fixes:** Now properly spreads when it kills a minion.

### V13.4
- *Malefic Visions*
  - **Bug Fixes:** Now properly triggers *Manaflow Band* when it spreads to an enemy champion after having done so multiple times.

### V12.22
- Stats
  - Mana growth increased to 28 from $27.5$.

### V12.21
- *Malefic Visions*
  - **New Effect:** Now executes minions below 15–45 health.

### V12.10
- Stats
  - Base health increased to 580 from 510.
  - Health growth increased to 101 from 87.
  - Armor growth increased to $4.7$ from $3.5$.
  - Magic resistance growth increased to $1.3$ from $0.5$.
- *Void Shift*
  - **Bug Fixes:** Crowd control immunity is no longer consumed by Blast Cone triggered by allies.

## Trivia

- After the Battlemage update in V6.9, Malzahar's Voidlings had their color scheme changed from Voidling to *gray with purple eyes*.
- Malzahar is one of the few champions to have a pet, the others being **Annie**, **Elise**, **Heimerdinger**, **Ivern**, **Shaco**, and **Yorick**.
- Malzahar and his *Voidlings*’s dance references U Can't Touch This by MC Hammer.
  - A side-by-side comparison for Malzahar's part can be seen here.
  - A side-by-side comparison for his *Voidlings*’s part can be seen here.
    - He shares this dance (specifically his *Voidlings*’s part) with **Jayce**.
- Upon the removal of *Zz'Rot Portal* in patch V9.23, *Gathering Swarm* stacks for *Void Swarm* was renamed to 'Zz'Rot Swarm' to honor the item.
  - This might indirectly imply that Malzahar's Voidling are a similar if not the same species as the Voidspawn that spawned from that item.
- Malzahar's Voidling had its assets reused for several rotating game modes.
  - It was given a retexture and features as a monster in Invasion.
  - Its animations were reused for the Voidling monster in Odyssey: Extraction.

---
*This page was automatically generated from League of Legends Wiki data.*