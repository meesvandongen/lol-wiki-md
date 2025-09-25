# Jarvan_IV

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Jarvan IV |

## Abilities

### Passive: Martial Cadence

| Attribute | Value |
|-----------|------:|
| **On-target CD Static** | 6–3@1–16 |
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | Proc |
| **Parry** | True |

**INNATE:** **Jarvan IV**’s basic attacks are empowered to deal **bonus** physical damage equal to 8% of the target's **current** health, with a minimum threshold of 20 and capped at 400 against non-champion targets.

This effect cannot occur on the same target more than once every few seconds.

**Notes:**

- *Martial Cadence*’s damage is based on the current health of the target before any damage of the attack has been dealt.
- *Martial Cadence*’s damage is dealt after the main attack damage.
  - If the target dies to the main attack damage, the additional damage will not be dealt.
- The empowered attack will not trigger against structures.

---

### Q: Dragon Strike

| Attribute | Value |
|-----------|------:|
| **Range** | 65 (Minimum distance) - 785 (Maximum range) / er 0 (Flag detection minimum distance) - 850 (Flag detection maximum range) units |
| **Cast Time** | $0.4$ seconds |
| **Collision Radius** | 180 (Dash collision radius) units |
| **Width** | 136 (Damage area width) / er 300 (Flag detection width) units |
| **Cost** | 45 / 50 / 55 / 60 / 65 Mana |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 seconds |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |
| **Grounded** | False |
| **Knockdown** | True |

**ACTIVE:** **Jarvan IV** extends his lance in the target direction, dealing physical damage to enemies hit and inflicting them with (armor penetration) armor reduction for 3 seconds.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 90 / 130 / 170 / 210 / 250 (+ 145% **bonus** AD) || Attribute | Value |
|-----------|------:|
| **Armor Reduction** | 10 / 14 / 18 / 22 / 26% of target's armor |

If the lance connects with a deployed *Demacian Standard*, **Jarvan IV** dashes to its location, knocking up nearby enemies around him and along his path for $0.75$ seconds.

***Jarvan IV** can cast any of his abilities during the dash.*

**Notes:**

- The armor reduction is applied after the damage. The damage dealt does not benefit from it in this case.
- This ability will cast from wherever the caster is at the end of the cast time.
- *Dragon Strike* will still pull **Jarvan IV** to Demacian Standard even if he is immobilized or grounded.
- Flash will interrupt the dash to a Demacian Standard but **Jarvan IV** will still knock up and damage enemies he comes in contact with at the new location.
  - Casting *Flash* right before the *dash* begins will cause **Jarvan IV** to be pulled to the flag from the new position.
  - Enemies already hit by *Dragon Strike* cannot be affected more than once.
- The knockup also slightly moves the target, landing 50-100 units in a random direction.

---

### W: Golden Aegis

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 600 units |
| **Cost** | 30 Mana |
| **Cooldown** | 9 seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Spell Shield** | True |

**ACTIVE:** **Jarvan IV** slows all nearby enemies for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Slow** | 15 / 20 / 25 / 30 / 35% |

**Jarvan IV** also grants himself a shield for 4 seconds, increased by $1.3$% of his **maximum** health for each enemy champion hit by *Golden Aegis*.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 60 / 80 / 100 / 120 / 140 (+ 70% **bonus** AD) |

**Notes:**

- *Golden Aegis* deals no damage and thus does **not** trigger turret aggro, or effects such as Elixir of Sorcery and Sudden Impact’s activation.
- *Golden Aegis* will activate combat status despite not dealing any damage.

---

### E: Demacian Standard

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 860 units |
| **Effect Radius** | 200 (Impact radius) / 1000 (Aura radius) / sight700 (Sight range) units |
| **Cost** | 55 Mana |
| **Cooldown** | 12 / 11.5 / 11 / 10.5 / 10 seconds |
| **Targeting** | Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |
| **Out of Range Behavior** | cast at max |

**PASSIVE:** **Jarvan IV** gains (attack speed) **bonus** attack speed.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 20 / 22.5 / 25 / 27.5 / 30% |

**ACTIVE:** **Jarvan IV** throws a Demacian flag to the target location, dealing magic damage to enemies within the area.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 120 / 160 / 200 / 240 (+ 80% AP) |

The flag then remains for 8 seconds, granting sight of its surroundings and providing an aura that grants *Demacian Standard*’s passive **bonus** attack speed to all nearby allied champions as well as **Jarvan IV** himself (See notes).

**Notes:**

- The attack speed granted from a deployed *Demacian Standard* stacks additively with *Demacian Standard*’s passive permanent bonus attack speed to **Jarvan IV**, but the aura from multiple *Demacian Standards* does not stack.
  - **Jarvan IV** gains a total of 40 / 45 / 50 / 55 / 60% bonus attack speed from *Demacian Standard* while near the flag.
- The flag has a $13.6$ pathing radius and a 65 gameplay radius.
  - The gameplay radius matters to Dragon Strike.
- *Demacian Standard* can be targeted by allied abilities (Teleport, Safeguard). The target type is the same as a minion.
- *Demacian Standard* is untargetable to enemies.

---

### R: Cataclysm

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 650 units |
| **Effect Radius** | 350 (Terrain creation radius) / sight1700 (Sight range (from center of the whole structure)) units |
| **Speed** | Varies (Dash speed based on cast distance) |
| **Cost** | 100 Mana |
| **Cooldown** | 120 / 112.5 / 105 / 97.5 / 90 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | False |
| **Impassable perimeter** | pathing radius 255 (Inner radius) to 445 (Outer radius) |

**ACTIVE:** **Jarvan IV** leaps with displacement immunity to the target enemy champion's location over $0.35$ seconds, dealing physical damage to all nearby enemies upon arrival.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 200 / 262.5 / 325 / 387.5 / 450 (+ 180% **bonus** AD) |

Upon impact, he creates a circle of impassable terrain that knocks aside enemies within the perimeter over $0.15$ seconds, knocking them out of the circle if they are on the outer edge and pulling them in otherwise. The terrain lasts for $3.5$ seconds and grants sight of the area. *Cataclysm* can be recast after $0.75$ seconds while the terrain is present.

**RECAST:** **Jarvan IV** destroys the terrain created by *Cataclysm*.

**Notes:**

*The initial cast and the manual recast count as ability activations for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive. **If the ability is not manually recasted, the secondary effect will trigger without being considered as an ability activation.
- *Cataclysm* will not deal damage, generate terrain, nor refund its cooldown if **Jarvan IV** is killed while leaping.
- The perimeter is made up of 24 units with 95 pathing radius, 65 gameplay radius and 1350 sight radius each. Only every other unit is rendered visible.
- Searing Charge will not destroy the whole but only the part that Ornn collides with.

---

## Patch History

### V25.16
- Stats
  - Armor growth reduced to $4.6$ from $5.2$.

### V25.06#March 19th Hotfix|V25.06
- Dragon Strike
  - **UNDOCUMENTED / BUG FIX:** AD ratio is now properly 145% **bonus** AD in accordance with the intended change in the patch's release, instead of only being present in the tooltip.

### V25.06
- Dragon Strike
  - Base damage increased to 90 / 130 / 170 / 210 / 250 from 80 / 120 / 160 / 200 / 240.
  - Bonus AD ratio increased to 145% **bonus** AD from 140%.

### V14.24
- Demacian Standard
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.18
- Martial Cadence
  - Health ratio increased to 8% of target's **current** health from 7%.

### V14.10
- Martial Cadence
  - Damage cap no longer applies to champions.

### V14.8
- Martial Cadence
  - Health ratio increased to 7% of target's **current** health from 6%.

### V14.5
- Stats
  - Armor growth increased to $5.2$ from $4.8$.
- Dragon Strike
  - Cooldown reduced to 10 / 9 / 8 / 7 / 6 seconds from 10 / 9.5 / 9 / 8.5 / 8.

### V14.1
- Cataclysm
  - Minimum recast time reduced to $0.75$ seconds from 1.

### V13.23
- Dragon Strike
  - Cooldown increased to 10 / 9.5 / 9 / 8.5 / 8 seconds from 10 / 9 / 8 / 7 / 6.

## Trivia

- Jarvan IV was first mentioned in Urgot’s lore.
- A tab for Jarvan IV can be faintly seen at the top during Maokai’s Art Spotlight.
- In-universe Demacian naming convention does not mirror real-life's European royalty and nobility's naming convention; as the current Jarvan should not have gotten the regnal number ***IV*** before his coronation.
- The terrain created by Cataclysm is made of 24 minions to make sure that no one can squeeze through the wall.
- The old visual effects of Golden Aegis was once made up of little ghosts due being a recolored copy of Death's Caress.
- Jarvan IV's Series 1 and 2 Eternals make the following references:
  - *I'm Helping* and *Still Helping!* is a reference to the I'm Jarvan, I'm Helping meme.

---
*This page was automatically generated from League of Legends Wiki data.*