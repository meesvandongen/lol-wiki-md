# Diana

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
| **Champion** | Diana |
| **Title** | Scorn of the Moon |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2012-08-07 |
| **Release Patch** | V1.0.0.144 |
| **Roles** | Assassin, Diver |
| **Riot Positions** | Jungle, Middle |
| **External Positions** | Jungle, Middle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $640.0$ | $+109.0$ | $2493.0$ |
| **Mana** | $375.0$ | $+25.0$ | $800.0$ |
| **Health Regen** | $6.5$ | $+0.85$ | $20.9$ |
| **Mana Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Armor** | $31.0$ | $+4.3$ | $104.1$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $57.0$ | $+3.0$ | $108.0$ |
| **Attack Speed** | $0.625$ | $+2.0\%$ | $0.838$ |
| **Movement Speed** | $345.0$ | $+0.0$ | $345.0$ |
| **Attack Range** | $150.0$ | $+0.0$ | $150.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.694$ |
| **Bonus AS per Level** | $2.0\%$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $188.889 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Moonsilver Blade

**Innate:** **Diana** gains , increased shortly after casting an ability.

*Every third basic attack in quick succession will cleave nearby enemies, dealing magic damage.*

**Innate:** **Diana** gains . After casting an ability, this bonus is tripled to 15×3 to 35×3 for 5 seconds. **Innate - Moonsilver Blade:** ''Diana's* basic attacks generate a stack of *Moonsilver Blade* for 5 seconds, refreshing on subsequent attacks and stacking up to 2 times. At 2 stacks, **Diana** empowers her next basic attack to consume the stacks on-hit to additionally cleave nearby enemies, dealing them 20+5*(x-1) for 6–then + 25*x (+ 50% AP) magic damage. *Moonsilver Blade' deals 260% damage against monsters.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self / Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 175 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Notes:**

- *Moonsilver Blade* will affect structures.
- Each attack has a distinct animation and she will glow when the empowered attack is ready.
- Sometimes, the empowered basic attack will not consume the stacks.
  - In this case, the attack will also not deal its damage.

---

### Q: Crescent Strike

**Active:** **Diana** unleashes a bolt of lunar energy that deals magic damage to enemies hit in an arc before exploding at the target location.

*Enemies hit are marked with *Moonlight*, sight them.*

**Active:** **Diana** unleashes a bolt of lunar energy that travels in a counter-clockwise arc before exploding at the target location, granting sight of the area for $0.5$ seconds and dealing magic damage to enemies hit and afflicting them with *Moonlight* for 3 seconds, during which they are standard sight. **Lunar Rush** interacts with *Moonlight*.

| Attribute | Value |
|-----------|-------|
| **Range** | 900 units |
| **Cooldown** | $8-6$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 50 Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Speed** | 1900 / 2100 units/second |
| **Spell Shield** | Special |
| **Spell Effects** | aoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:** $70-210$ (+ 70% AP)

**Notes:**

- Even if the target spell shield the bolt, they may still be damaged by the explosion. *Moonlight* is not applied to the protected target if blocked.
- The bolt consists of two projectiles with slightly different trajectories. Effect at cast time end

---

### W: Pale Cascade

**Active:** **Diana** shield herself, creating three orbiting spheres that detonate on contact with enemies to deal magic damage in an area.

*If all the spheres detonate, she gains more shield.*

**Active:** **Diana** grants herself a shield for up to 5 seconds and creates three spheres that orbit her counterclockwise for the same duration, detonating upon contact with an enemy to deal magic damage to nearby enemies. If all three spheres detonate, 'Pale Cascade's shield is reapplied, stacking with its original shield and refreshing the duration.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $15-9$ seconds |
| **Cast Time** | none |
| **Cost** | $40-60$ mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 200 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**Scaling:**
- **Shield Strength:** $45-105$ (+ 30% AP) (+ 9%
- **bonus** health) **Magic Damage per Orb:* $20-68$ (+ 18% AP)3-68×3$ (+ 54% AP)
- **Maximum Shield Strength:** $45×2-105×2$ (+ 60% AP) (+ 18%
- **bonus** health)

**Notes:**

- On the second application of 'Pale Cascade's* shield, Shield Power will apply to both the new shield amount and the shield amount remaining from the first shield, which has already benefited from *shield power'.
  - Because of this, *shield power* effectively applies to the second shield twice, but with reduced efficiency, for up to an increase of 50% of *shield power* (maximum benefit if no damage was mitigated by the first shield). With 10% *shield power*, the second shield's total amount will be increased by up to an additional 5%, for a total of $15.5%$ **bonus** shield.
  - The formula for the total amount of shield **Diana** will receive from both shield applications is: *** *Total Shield = (((Shield Amount × (1 + Shield Power)) - Damage Blocked by Shield) + Shield Amount) × (1 + Shield Power)*

---

### E: Lunar Rush

**Active:** **Diana** dash to an enemy and deals magic damage.

*If used on a target afflicted with **Moonlight**, the cooldown is massively ah.*

**Active:** **Diana** dashes to the target enemy's location, and upon completion, she deals them magic damage and consumes **Moonlight** from all enemies. If the target is within 400 range, **Diana** will dash through their location. If *Moonlight* is consumed from the target, 'Lunar Rush's **current cooldown** is reduced to $0.25$ seconds. **Diana can cast any of her abilities during the dash.**

| Attribute | Value |
|-----------|-------|
| **Range** | 825 units |
| **Cooldown** | $22-14$ seconds |
| **Cast Time** | none |
| **Cost** | $40-60$ mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |

**Scaling:**
- **Magic Damage:** $50-130$ (+ 60% AP)

**Notes:**

- **Diana** will attempt to basic attack the target.
- 'Lunar Rush's* cooldown will also be reset if the target dies during the dash while being affected by *Moonlight'.
- *Lunar Rush* consumes *Moonlight* upon ending the dash (even if interrupt).
- *Lunar Rush* will still deal damage even if the target is untargetable by the end of the dash.
- If *Lunar Rush* is blocked by spell shield the *Moonlight* debuff is still consumed but 'Lunar Rush's cooldown is not reset.

---

### R: Moonfall

**Active:** **Diana** airborne in and slow nearby enemies.

*If she pulls an enemy champion, she calls down a beam of moonlight to strike upon the area after a delay, dealing magic damage, increased if she pulled more champions.*

**Active:** **Diana** airborne in all nearby enemies, during which they are standard sight, then slow them for 2 seconds. If an enemy champion is pulled in, she calls down a beam of moonlight to strike upon the area around her after 1 second, dealing magic damage to all nearby enemies, increased for each champion pulled beyond the first.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $100-80$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 475 / 225 units |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Slow:** $40-60$%
- **Magic Damage:** $200-400$ (+ 60% AP)

**Notes:**

- Enemies stay in the area for a maximum of approximately $0.5$ seconds while being pulled.
- cc-immune and displacement immunity still count as being "pulled in" by the ability.
- clone count as champions for increasing this ability's damage.
- 'Moonfall's additional damage is not capped at 4 champions.
- The beam of moonlight does not crash down if champions hit blocked the initial effect with spell shield.
  - Enemy champions protected by *spell shield* do not count towards the damage increase. Effect at cast time end
- A lunar phase of the moon will appear above **Diana** while she is casting *Moonfall*, each phase is based on the number of enemy champion pulled: New_Moon_Moonfall_(1_Enemy_Champion).png **New Moon:** **1** Enemy Champion Pulled Waning_Crescent_Moonfall_(2_Enemy_Champions).png **Waning Crescent:** **2** Enemy Champions Pulled Third_Quarter_Moonfall_(3_Enemy_Champions).png **Third Quarter:** **3** Enemy Champions Pulled Waning_Gibbous_Moonfall_(4_Enemy_Champions).png **Waning Gibbous:** **4** Enemy Champions Pulled Full_Moon_Moonfall_(5_Enemy_Champions).png **Full Moon:** **5** Enemy Champions Pulled

---

## Patch History

### V25.18
- *Moonfall*
  - **Bug Fixes:** Corrected damage area warning VFX, which was previously smaller than the actual hit area.

### V25.11
- *Moonsilver Blade*
  - Cleave monster damage increased to 260% from 225%.

### V25.09
- *Crescent Strike*
  - **Bug Fixes:** Missile range is no longer shorter or longer than intended if Flash is used during the cast time.
    - 'This bug fix had been noted before on patch

### V25.04
- *Moonsilver Blade*
  - Cleave monster damage reduced to 225% from 300%.
- *Crescent Strike*
  - Base damage increased to $70-210$ from $60-200$.
- *Pale Cascade*
  - Base damage per hit increased to $20-68$ from $18-66$.
    - Total base damage increased to $20×3-68×3$ from $18×3-66×3$.

### V14.9
- Stats
  - Selection radius increased to 120 units from 75.

### V14.7
- *Moonsilver Blade*
  - **Bug Fixes:** Tooltip now notes the correct duration of the buff due to 14.6 changes.

### V14.6
- Stats
  - Attack speed ratio increased to $0.694$ from $0.625$.
  - Attack speed growth reduced to 2% from $2.25$%.
- *Moonsilver Blade*
  - Bonus attack speed reduced to 15 to 35 from 15 to 40.
    - Empowered bonus attack speed reduced to 45 to 105 from 45 to 120.
  - Empowered bonus attack speed duration increased to 5 seconds from 3.
- *Pale Cascade*
  - **Bug Fixes:** The initial shield no longer lasts 1 second shorter than intended.
- *Lunar Rush*
  - **Bug Fixes:** The cooldown reset now properly takes place upon colliding with the target.

### V14.5
- *Moonsilver Blade*
  - **Bug Fixes:** Buff no longer desyncs to trigger the empowered attack every 2 or 4 basic attacks, instead of the intended 3.
- *Lunar Rush*
  - Cooldown reset upon consuming *Moonlight* reduced to $0.25$ seconds from $0.5$.
  - **Bug Fixes:** Cooldown is no longer sometimes reset a second time after only consuming *Moonlight* once.

### V14.4
- Diana
  - **Bug Fixes:** Pentakill VFX on the river terrain in Summoner's Rift is now properly rendered and no longer renders slightly higher than intended on normal elevated terrain.

### V13.14
- *Pale Cascade*
  - Base shield increased to $45-105$ from $40-100$.
  - Shield AP ratio increased to 30% AP from 25% AP.
  - Damage AP ratio per orb increased 18% AP from 15% AP.
- *Lunar Rush*
  - AP ratio increased to 60% AP from 50% AP.

## Trivia

- *Diana* derives from Proto-Indo-European language root Dyeus "to shine".
  - She shares her name with Diana). Roman goddess of the Moon.
- Diana's concept artist Michael 'IronStylus' Maurino has confirmed that Diana was intended to be lesbian when she was first created. Her forbidden relationship with Leona was later canonized in the short story "Rise with Me".
- Diana is the first champion to feature animations for when affected by both crowd control and movement speed buffs.
- *Moonfall*’s indicator above Diana references the Lunar phase.
  - 1 enemy: New moon
  - 2 enemies: Waning crescent
  - 3 enemies: Last quarter
  - 4 enemies: Waning gibbous
  - 5 enemies: Full moon
- A functional replica of Moonfall old.png was crafted in an episode of YouTube series Man At Arms: Reforged.
  - There are also videos where the following are crafted:
    - Preparation (functional)
    - Zenith Blade (functional)
    - Highlander (functional)
    - Hammer Shock (functional)
    - Steel Tempest (functional)
    - Short Fuse (prop)

---
*This page was automatically generated from League of Legends Wiki data.*