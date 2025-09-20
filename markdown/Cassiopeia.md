# Cassiopeia

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Cassiopeia |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $550$ | $+85$ | $1995.0$ |
| **Mana** | $350$ | $+50$ | $1200.0$ |
| **Armor** | $22$ | $+3.5$ | $81.5$ |
| **Magic Resist** | $30$ | $+0.5$ | $38.5$ |
| **Attack Damage** | $56$ | $+3.1$ | $108.7$ |
| **Attack Speed** | $0.625$ | $+3.2\%$ | $0.965$ |

## Abilities

### Passive: Serpentine Grace

**Innate:** **Cassiopeia** gains ms per level, but she cannot purchase *Boots* items.

**Innate:** **Cassiopeia** gains $4 to 72$ *ms **bonus** movement speed*, but she cannot purchase [Boots items](./Item.md#Boots_tiers). **[Blessing of Noxus](./Blessing_of_Noxus.md) Bonus:** 'Serpentine Grace's' **bonus** movement speed is increased by $1 to 18$, for a total of $4+1 to 72+18$.

**Notes:**

- The movement speed from *Serpentine Grace* is worth per level, up to a maximum of at level 18.
- Without other movement speed modifiers taken into account, *Serpentine Grace* grants **Cassiopeia* a total of $4+ccd$ to 72+ccdms movement speed*.
- As *Magical Footwear* grants boots, the rune will be replaced with *Cash Back*.

---

### Q: Noxious Blast

**Active:** **Cassiopeia** blasts an area with Poison after a brief delay, dealing magic damage over time to enemies hit. She gains *ms **bonus** movement speed* if she hits an enemy champion.

**Active:** **Cassiopeia** creates a blast at the target location that explodes after a $0.4$-second delay. Enemies within the blast are poison for 3 seconds, taking magic damage every $3/7 round=3$ seconds over the duration. If *Noxious Blast* hits an enemy champion, **Cassiopeia** gains *ms **bonus** movement speed* that decays over 3 seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 850 units |
| **Cooldown** | $3.5$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $50-70$ mana |

**Scaling:**
- **Total Magic Damage:** $75-215$ (+ 65% AP) Magic Damage Per Tick $75/7-215/7 round=2$ (+ $65/7
- **Bonus Movement Speed:** $30-50$%

**Notes:**

- *Noxious Blast* always uses [quick cast](./quick_cast.md), regardless of player settings.
- The damage inflicted by *Noxious Blast* is calculated as follows: damage / 7 ticks rounded down to the nearest integer.
  - For example: with 0 *bonus AP *Noxious Blast* inflicts 75 damages over 7 ticks of damage, so 75/7 (≈10.71) deals first of all 10 damage, then (275/7) 10 (≈11.43) deals in a second time 11 damages, then (375/7) 10 11 (≈11.14) deals in a third time 11 damages, and so on.
- The damage displayed for *Noxious Blast* DOT ([Damage_over_time](./Damage_over_time.md)) always remains constant and corresponds to the previously mentioned formula: damage / 7 ticks rounded down to the nearest integer. It's important not to confuse the consistent displayed damage with the actual damage, which is updated with each tick, see [tick and updates](./tick_and_updates.md).
  - With the same example as above, the damage displayed is always 10.
- Occasionally, *Noxious Blast* may not deal the displayed damage. This discrepancy occurs when the calculated damage falls between 0.5 and 1. In such cases, rounding to the nearest integer, specifically rounding up, creates a 1-point difference. It's important to note that the damage is rounded down to the nearest integer.
  - For example: with a , *Noxious Blast* shows $75 + 90% of 45$ = 115,5 magic damage so rounded to the nearest integer $116 magic damage$, but really deals using the formula above $115 magic damage$.
- When **Cassiopeia** has precisely 20 *bonus AP or 40 *bonus AP. In this scenario, *Noxious Blast* may inflict 1 less damage than intended. Importantly, this bug occurs independently of rounding considerations.
- Enemies within the blast of *Noxious Blast* are poison for something between $3.2$ and $3.3$ seconds.

---

### W: Miasma

**Active:** **Cassiopeia** spews several clouds of Poison that linger, continually dealing magic damage to enemies within. They are also slow and ground.

*Grounded enemies cannot use Movement abilities.*

**Active:** **Cassiopeia** spews forth 7 bolts of venom in an arc at the target location, creating toxic clouds at the area for 5 seconds. Enemies within the clouds are poison to take magic damage every $5/19 round=3$ seconds and become ground and slow by an amount that decays over the area's duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 700 units |
| **Cooldown** | $24-16$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-90$ Mana |

**Scaling:**
- **Magic Damage Per Second:* $20-40$ (+ 10% AP)*Slow:** $40-80$%

**Notes:**

- 'Miasma's' slow and ground debuffs are each marked as non-dispellable, so they are not removed by most cleanse. Each is however allowed to be removed by cleanses that **also** grant immunity to the debuff type, such as .
- If a target becomes untargetable while affected by the ground, the debuff will refresh to $0.25$ seconds.
- *Wind Wall* will block the portion of *Miasma* it destroys.
- *Miasma* inflicts $5-10$ (+ $2.5$% AP) magic damage per tick, and in fact because there is 19 ticks and not 20 ticks, the **total** magic damage is currently not $100-200$ but $95-190$ (+ $47.5$% AP).
  - Sometimes *Miasma* randomly inflicts only 18 ticks of damage.

---

### E: Twin Fang

**Active:** **Cassiopeia** launches an attack at the target enemy that deals magic damage. If this attack kills the target, the *mana cost* is refunded.

*Against a Poison target, the attack deals increased damage and heal her.*

**Active:** **Cassiopeia** launches her fangs at the target enemy that deal $52 to 120$ (+ 10% AP) magic damage. If this kills the target, 'Twin Fang's' is refunded. Against a poison target, *Twin Fang* deals **bonus** magic damage and heals **Cassiopeia**. The heal is reduced by 75% against minions and small and medium monsters.

| Attribute | Value |
|-----------|-------|
| **Range** | 700 units |
| **Cooldown** | $0.75$ seconds |
| **Cast Time** | $0.125$ seconds |
| **Cost** | 40 Mana |

**Scaling:**
- **Bonus Magic Damage:* $20-112$ (+ 55% AP)*Heal:* $10-16$% AP0.25-16*0.25$% AP

**Notes:**

- Upon reaching the target, *Twin Fang* will wait for an ongoing *Noxious Blast* to explode before dealing its damage.
- 'Twin Fang's' damage against poisoned targets:
  - When maxed first: $52+20-84+100 to 120+100$ (+ 65% AP).
  - When maxed second: $52+20-100+100 to 120+100$ (+ 65% AP).
  - When maxed last: $52+20 to 100+20-120+100$ (+ 65% AP).
- 'Twin Fang's* cast indicator is incorrectly adding her own radius to the range like an edge range ability, and is therefore slightly larger than the actual cast range *Twin Fang' can cast at.
- Using *Twin Fang* on an out-of-range target after completing a movement order causes **Cassiopeia** to move back to her original location after casting, if no other order is issued during the entire process.
  - This may be a consequence of *Twing Fang* being a "Walk in range of the target" ability that does not interrupt movement commands.

---

### R: Petrifying Gaze

**Active:** **Cassiopeia** blasts in a cone, dealing magic damage to enemies hit and stun any that are facing her, while slow any others with their back turned.

**Active:** **Cassiopeia** blasts enemies in a cone in the target direction, dealing magic damage to enemies struck within and slow them by 40% for 2 seconds. Enemies with their facing direction towards her are instead stun for the same duration.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | $0.5$ seconds |
| **Cost** | 100 mana |

**Scaling:**
- **Magic Damage:** $150-350$ (+ 50% AP)

**Notes:**

- **Cassiopeia** will turn to face the target direction over the cast time.
  - 'Petrifying Gaze's* target direction will change if *'Cassiopeia's'' facing direction changes during the cast time (i.e. through airborne). *** The visual effect will not follow changes in ''Cassiopeia's'* facing direction and may not portray *Petrifying Gaze's' target direction accurately.
- The facing direction of champions whose abilities/animations cause them to lock their facing or spin is always the direction they are moving in. For abilities that cause the champion to spin in place (e.g. *Death Lotus*, *Judgment*) it will factor the direction they were facing on cast.
  - is an exception and his facing direction is actually considered to be the direction he is facing.

---

## Patch History

### V25.18
- Stats
  - Base mana increased to 450 from 400.
- *Twin Fang*
  - Poisoned bonus damage increased to $20-112$ from $20-100$.
- *Noxious Blast*
  - Total AP ratio reduced to 65% AP from 70% AP.
- *Miasma*
  - AP ratio per second reduced to 10% AP from 15% AP.
- *Serpentine Grace*
  - Blessing of Noxus bonus movement speed reduced to $1 to 18$ from $2 to 36$.
- *Serpentine Grace*
  - ***New Effect:*** Now grants an additional $2 to 36$ **bonus** movement speed if her team gets *Blessing of Noxus*.

### V14.16
- *Twin Fang*
  - ***Bug Fixes:*** Heal is now properly reduced against monsters.

### V14.15
- Stats
  - Base mana increased to 400 from 350.
  - Mana growth reduced to 40 from 60.
- *Noxious Blast*
  - Mana cost reduced to $50-70$ from $50-90$.
  - AP ratio per tick reduced to $70/7$% AP from $90/7$% AP.
    - Total AP ratio reduced to 70% AP from 90% AP.
- *Miasma*
  - Mana cost reduced to $70-90$ from $70-110$.
- *Twin Fang*
  - Mana cost reduced to 40 at all ranks from $50-42$.
  - Poison bonus AP ratio reduced to 55% AP from 60% AP.

### V14.10
- General
  - *Magical Footwear* is now replaced by *Cash Back* instead of *Triple Tonic*.

### V13.22
- Stats
  - Basic attack missile speed increased to 1500 from 1200.

### V12.23
- General
  - ***Bug Fixes:*** Basic attack hit VFX are now properly visible.

### V12.10
- Stats
  - Base health increased to 630 from 560.
  - Health growth increased to 104 from 90.
  - Armor growth increased to $4.7$ from $3.5$.
  - Magic resistance growth increased to $1.3$ from $0.5$.
- *Twin Fang*
  - Heal AP ratio reduced to $10-16$% AP from $12-20$% AP.

### V11.15
- *Twin Fang*
  - Bonus base damage increased to $20-100$ from $10-90$.

### V11.14
- *Miasma*
  - **Undocumented/Bug fix:** Grounded debuff can once again be cleansed by .

### V11.13
- *Miasma*
  - ***Bug Fixes:*** Grounded debuff is now non-dispellable.
    - This prevents the debuff from being briefly removed by cleanse even while the player remains in the AoE.
    - Because of this, fails to dispell and become immune to the effect for the remaining duration unless he leaves and reenters it.

## Trivia

- Cassiopeia was the last champion released in 2010 as well as the third one to feature an '[Art Spotlight](./Art_Spotlight.md)' (hers in particular shows how her taunt was animated).
  - Cassiopeia's basic attack particles were snakes at release, as seen in [her](https://www.youtube.com/watch?v=neQNvEyuhPU) [Champion Spotlight](./Champion_Spotlight.md).
- Cassiopeia - **Katarina** is one of seven pairs of sibling champions (the others being **Kayle** - **Morgana**, **Garen** - **Lux**, **Nasus** - **Renekton**, **Yasuo** - **Yone**, **Darius** - **Draven**, and **Vi** - **Jinx**).
  - Though not a pair, **Anivia**, **Ornn**, and **Volibear** are also siblings.
- *Twin Fang* directly benefits from allied poison abilities in the same fashion *Last Breath* does with displacement ones.
- Her name's first element derives from Hebrew קְצִיעָה‏ *qeṣiah* cinnamomum cassia:
  - Possibly from Semitic root *K-S* "cut", as the barks were scrapped from the tree's trunk;
  - Or borrowed from Old Chinese 桂 **kʷeːs* (> Mandarin *guì* /kʷei̯˥˩/) "osmanthus fragrans", by spice-merchants who imported the barks;
    - the suffix *-opeia* is etymologically uncertain.
- The inspiration for Cassiopeia comes from mainly three sources:
  - Greek mythology
  - #She shares names with Cassiopeia (constellation) charted by Ptolemy as well as Cassiopeia (mythology) from Greek Mythology.
  - #*Miasma* references Miasma (Greek mythology).
  - #She resembles a Gorgon (with *Petrifying Gaze* and her death animation referencing Medusa) as well as a Drakaina (mythology).
  - Plotelamic kingdom
  - #Her seductress background references Cleopatra VII.
  - #Hers is the [the Belly Dance.](http://www.youtube.com/watch?v=8YQM_zfYO7Q)
  - Indian Nāga.

---
*This page was automatically generated from League of Legends Wiki data.*