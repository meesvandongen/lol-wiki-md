# Jarvan_IV

## Table of Contents
- [Basic Information](#basic-information)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Jarvan IV |

## Abilities

### Passive: Martial Cadence

**Innate:** **Jarvan IV**’s basic attacks deal **bonus** physical damage equal to a portion of the target's **current** health.

*This effect cannot occur on the same target more than once every few seconds.*

**Innate:** ''Jarvan IV's** basic attacks are empowered to deal **bonus'' physical damage equal to 8% of the target's **current** health, with a minimum threshold of 20 and capped at 400 against non-champion targets. This effect cannot occur on the same target more than once every few seconds.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | False |
| **Spell Effects** | Proc |

**Notes:**

- 'Martial Cadence's damage is based on the current health of the target before any damage of the attack has been dealt.
- 'Martial Cadence's damage is dealt after the main attack damage.
  - If the target dies to the main attack damage, the additional damage will not be dealt.
- The empowered attack will not trigger against structures.

---

### Q: Dragon Strike

**Active:** **Jarvan IV** extends his lance in the target direction, dealing physical damage to enemies hit and lethality for a short time.

*If the lance connects with a deployed **Demacian Standard**, he will dash to its location, airborne enemies he passes by.*

**Active:** **Jarvan IV** extends his lance in the target direction, dealing physical damage to enemies hit and inflicting them with armor penetration for 3 seconds. If the lance connects with a deployed **Demacian Standard**, **Jarvan IV** dashes to its location, airborne nearby enemies around him and along his path for $0.75$ seconds. **Jarvan IV can cast any of his abilities during the dash.**

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $10-6$ seconds |
| **Cast Time** | $0.4$ seconds |
| **Cost** | $45-65$ Mana |
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |

**Scaling:**
- **Physical Damage:** $90-250$ bonus AD)
- **Armor Reduction:** $10-26$% of target's armor

**Notes:**

- The armor reduction is applied after the damage. The damage dealt does not benefit from it in this case. Effect at cast time end
- *Dragon Strike* will still pull **Jarvan IV** to *Demacian Standard* even if he is immobilize or ground.
- Flash will interrupt the dash to a *Demacian Standard* but **Jarvan IV** will still airborne and damage enemies he comes in contact with at the new location.
  - Casting *Flash* right before the *dash* begins will cause **Jarvan IV** to be pulled to the flag from the new position.
  - Enemies already hit by *Dragon Strike* cannot be affected more than once.
- The airborne also slightly moves the target, landing 50-100 units in a random direction.

---

### W: Golden Aegis

**Active:** **Jarvan IV** slow nearby enemies for a short time.

*He will also shield himself for a few seconds, with the strength increased if he hit more enemy champions.*

**Active:** **Jarvan IV** slow all nearby enemies for 2 seconds. **Jarvan IV** also grants himself a shield for 4 seconds, increased by $1.3$% of his **maximum** health for each enemy champion hit by *Golden Aegis*.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 9 seconds |
| **Cast Time** | none |
| **Cost** | 30 Mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Effect Radius** | 600 units |
| **Spell Shield** | True |

**Scaling:**
- **Slow:** $15-35$%
- **Shield Strength:** $60-140$ (+ 70% bonus AD)

**Notes:**

- *Golden Aegis* deals no damage and thus does **not** trigger turret aggro, or effects such as *Elixir of Sorcery* and activation.
- *Golden Aegis* will activate combat status despite not dealing any damage.

---

### E: Demacian Standard

**Passive:** **Jarvan IV** gains **bonus attack speed**.

**Active:** **Jarvan IV** plants a flag at the target location that deals magic damage to nearby enemies.

**Passive:** **Jarvan IV** gains *attack speed *bonus attack speed*. **Active:** **Jarvan IV** throws a Demacian flag to the target location, dealing magic damage to enemies within the area. The flag then remains for 8 seconds, granting sight of its surroundings and providing an aura that grants 'Demacian Standard's passive *bonus attack speed to all nearby allied champions as well as **Jarvan IV** himself.

| Attribute | Value |
|-----------|-------|
| **Range** | 860 units |
| **Cooldown** | $12-10$ seconds |
| **Cast Time** | none |
| **Cost** | 55 Mana |
| **Targeting** | Location |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 200 / 1000 / sight700 units |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |

**Scaling:**
- **Bonus Attack Speed:** $20-30$%
- **Magic Damage:** $80-240$ (+ 80% AP)

**Notes:**

- The attack speed granted from a deployed *Demacian Standard* stacks additively with 'Demacian Standard's* passive permanent bonus attack speed to **Jarvan IV**, but the aura from multiple *Demacian Standards' does not stack.
  - **Jarvan IV** gains a total of $20×2-30×2$% bonus attack speed from *Demacian Standard* while near the flag.
- The flag has a $13.6$ pathing radius and a 65 gameplay radius.
  - The gameplay radius matters to *Dragon Strike*.
- *Demacian Standard* can be targeted by allied abilities (Teleport, *Safeguard*). The target type is the same as a minion.
- *Demacian Standard* is untargetable to enemies.

---

### R: Cataclysm

**Active:** **Jarvan IV** dash displacement immune to the target enemy champion's location, dealing physical damage to nearby enemies upon impact.

*He then erects a circle of terrain that lasts for a short time.*

**Active:** **Jarvan IV** dash with displacement immunity to the target enemy champion's location over $0.35$ seconds, dealing physical damage to all nearby enemies upon arrival. Upon impact, he creates a circle of impassable terrain that airborne enemies within the perimeter over $0.15$ seconds, knocking them out of the circle if they are on the outer edge and pulling them in otherwise. The terrain lasts for $3.5$ seconds and grants sight of the area. *Cataclysm* can be recast after $0.75$ seconds while the terrain is present. **Recast:** **Jarvan IV** destroys the terrain created by *Cataclysm*.

| Attribute | Value |
|-----------|-------|
| **Range** | 650 units |
| **Cooldown** | $120-90$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Unit |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Speed** | Varies |
| **Effect Radius** | 350 / sight1700 units |
| **Spell Shield** | True |
| **Spell Effects** | Aoe |

**Scaling:**
- **Physical Damage:** $200-450$ bonus AD)

**Notes:**

- *Cataclysm* will not deal damage, generate terrain, nor refund its cooldown if **Jarvan IV** is killed while dash.
- The perimeter is made up of 24 units with 95 pathing radius, 65 gameplay radius and 1350 sight radius each. Only every other unit is rendered visible.
- *Searing Charge* will not destroy the whole but only the part that **Ornn** collides with.

---

## Patch History

### V25.16
- Stats
  - Armor growth reduced to $4.6$ from $5.2$.
- *Dragon Strike*
  - **Undocumented / Bug Fix:** AD ratio is now properly 145% *bonus AD in accordance with the intended change in the patch's release, instead of only being present in the tooltip.

### V25.06
- *Dragon Strike*
  - Base damage increased to $90-250$ from $80-240$.
  - Bonus AD ratio increased to 145% *bonus AD from 140%.

### V14.24
- *Demacian Standard*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.18
- *Martial Cadence*
  - Health ratio increased to 8% of target's **current** health from 7%.

### V14.10
- *Martial Cadence*
  - Damage cap no longer applies to champions.

### V14.8
- *Martial Cadence*
  - Health ratio increased to 7% of target's **current** health from 6%.

### V14.5
- Stats
  - Armor growth increased to $5.2$ from $4.8$.
- *Dragon Strike*
  - Cooldown reduced to $10-6$ seconds from $10-8$.

### V14.1
- *Cataclysm*
  - Minimum recast time reduced to $0.75$ seconds from 1.

### V13.23
- *Dragon Strike*
  - Cooldown increased to $10-8$ seconds from $10-6$.

### V13.18
- General
  - Updated ability icons.
- *Martial Cadence*
  - Health ratio reduced to 6% of target's **current** health from 8%.
- *Cataclysm*
  - **Bug Fixes:** Summoned terrain now applies airborne to enemies knocked by being within the parameters.

## Trivia

- Jarvan IV was first mentioned in **Urgot**’s lore.
- A tab for Jarvan IV can be faintly seen at the top during **Maokai**’s Art Spotlight.
- In-universe Demacian naming convention does not mirror real-life's European royalty and nobility's naming convention; as the current Jarvan should not have gotten the regnal number ***IV*** before his coronation.
- The terrain created by *Cataclysm* is made of 24 minions to make sure that no one can squeeze through the wall.
- The old visual effects of *Golden Aegis* was once made up of little ghosts due being a recolored copy of *Death's Caress*.
- Jarvan IV's Series 1 and 2 Eternals make the following references:
  - 'I'm Helping* and *Still Helping!' is a reference to the I'm Jarvan, I'm Helping meme.

---
*This page was automatically generated from League of Legends Wiki data.*