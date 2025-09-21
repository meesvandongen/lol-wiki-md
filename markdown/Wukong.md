# Wukong

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
| **Champion** | Wukong |
| **Title** | the Monkey King |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2011-07-26 |
| **Release Patch** | V1.0.0.122 |
| **Roles** | Diver |
| **Riot Positions** | Jungle |
| **External Positions** | Top, Jungle |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $610.0$ | $+99.0$ | $2293.0$ |
| **Mana** | $330.0$ | $+65.0$ | $1435.0$ |
| **Health Regen** | $3.5$ | $+0.65$ | $14.6$ |
| **Mana Regen** | $8.0$ | $+0.8$ | $21.6$ |
| **Armor** | $31.0$ | $+4.7$ | $110.9$ |
| **Magic Resist** | $28.0$ | $+2.05$ | $62.8$ |
| **Attack Damage** | $66.0$ | $+3.5$ | $125.5$ |
| **Attack Speed** | $0.690$ | $+3.0\%$ | $1.042$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $175.0$ | $+0.0$ | $175.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.69$ |
| **Attack Speed Ratio** | $0.658$ |
| **Bonus AS per Level** | $3.0\%$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $100 units$ |
| **Selection Height** | $122.222 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Stone Skin

**Innate:** **Wukong** gains *armor* based on level and equal to a portion of his **maximum** health.

*Whenever **Wukong** or his **clone** damage an enemy champion or monster, he generates a stack of *Strength of Stone*, which stacks up to 5 times.*

**Innate:** **Wukong** gains armor*bonus* armor and an additional every 5 seconds. $$0.35/10$$% of his every $0.5$ seconds. Whenever **Wukong** or his **clone** damage an enemy champion or monster, he generates a stack of *Strength of Stone* for 5 seconds, refreshing on subsequent hits and stacking up to 5 times. Stacks expire by one every second when the duration ends. **Strength of Stone:* For each stack,up to a maximum of 6*(1+1×5) to 10*(1+1×5) **bonus* armor and total additional regeneration of

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

No additional notes.

---

### Q: Crushing Blow

**Active:** **Wukong**’s next basic attack within a few seconds will gain *range **bonus** range*, deal **bonus** physical damage, and inflict armor penetration for a short time.

*The cooldown is cdr whenever **Wukong** or his **clone** damage an enemy.*

**Active:** **Wukong** empowers his next basic attack within 5 seconds to have an uncancellable windup, gain *range **bonus** range*, deal **bonus** physical damage, and inflict armor penetration for 3 seconds. 'Crushing Blow's* *cooldown* is reduced by $0.5$ seconds whenever **Wukong** or his **clone*' damage an enemy. *Crushing Blow basic attack reset *'Wukong's' and his *clone's* basic attack timer. Other abilities can be used during the attack's animation.'

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $8-6$ seconds |
| **Cast Time** | none |
| **Cost** | 20 Mana |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **Bonus Range:** $135-175$
- **Bonus Physical Damage:** $20-120$ (+ 55% bonus AD)
- **Armor Reduction:** $10-30$% of target's armor

**Notes:**

- *Crushing Blow* triggers on turret and ward but the armor reduction is not applied.
- Issuing a Hold command (default **J**) or Stop command (default **S**) after 'Crushing Blow's attack has started will cancel the animation, however the attack will still be launched and **Wukong** will be unable to move until that point.

---

### W: Warrior Trickster

**Active:** **Wukong** enters brief invisibility and dash to the target location, leaving behind a clone that remains for a short time.

*The clone will basic attack autonomously and gain the effects of his other abilities, as well as cast **Cyclone**.*

**Active:** **Wukong** enters invisibility for 1 second and dash in the target direction, leaving behind a clone of himself at his casting position for $3.25$ seconds. The clone is untargetable to allies and can basic attack autonomously, prioritizing the last enemy **Wukong** damaged. It can also gain the effects of **Crushing Blow** and **Nimbus Strike*’s* *bonus attack speed, and casts **Cyclone** whenever **Wukong** does. The clone deals reduced damage. *See [Pets](#Pets) for more details about *'Wukong's** clone. **Wukong'* can cast any of his abilities during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | 100 / 300 units |
| **Cooldown** | $20-16$ seconds |
| **Cast Time** | none |
| **Cost** | $60-40$ Mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Speed** | 900 units/second |
| **Effect Radius** | *Clone's attack range* |
| **Spell Shield** | True |
| **Spell Effects** | pet |

**Scaling:**
- **Clone Outgoing Damage:** $35-55$%

**Notes:**

- Casting *Warrior Trickster* during an ability's effects will transfer them to the clone.
- Turret shots instantly kill the clone.
- The clone is immune to stasis and Blast Cone.
- The clone can still be targeted by allied auto-targeted effects (e.g. Heal, *Celestial Blessing*).
- Using a basic attack breaks the stealth at the end of the attack windup.

---

### E: Nimbus Strike

**Active:** **Wukong** dashes to the target enemy and sends untargetable clones to briefly attack up to two surrounding enemies, dealing magic damage.

*Upon arrival, **Wukong** gains **bonus attack speed** for a few seconds.*

**Active:** **Wukong** dashes to the target enemy's location and sends out untargetable *clones* to strike up to two additional enemies near the target's location, dealing magic damage. Upon arrival, **Wukong** gains **bonus attack speed** for 5 seconds. **Crushing Blow* can be cast during the dash.*

| Attribute | Value |
|-----------|-------|
| **Range** | 625 units |
| **Cooldown** | $10-7$ seconds |
| **Cast Time** | none |
| **Cost** | $30-50$ Mana |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | magic |
| **Effect Radius** | $187.5$ units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Scaling:**
- **Magic Damage:** $80-200$ (+ 100% AP)
- **Bonus Attack Speed:** $35-55$%

**Notes:**

- The *clones* can block non-targeted attacks or abilities for $0.25$ seconds after hitting their targets.
- Casting *Nimbus Strike* interrupts the attack windup of enemies targeting **Wukong**.
- *Nimbus Strike* has an offset distance of 75 units from the target dashed to.
- The ability applies a stack of *Electrocute* / *Phase Rush* even if the target is untargetable.

---

### R: Cyclone

**Active:** **Wukong** spins his staff around for a short time, becoming ghosted and gaining *ms **bonus** movement speed*. The staff continually deals physical damage to enemies hit based on their **maximum** health, and can briefly airborne once.

*Cyclone* can be recast within the duration, and does so automatically afterwards.

**Active:** **Wukong** spins his staff around for up to 2 seconds, becoming ghosted and gaining ms*bonus** movement speed*. The staff deals physical damage every $0.25$ seconds to enemies hit, and can airborne once for $0.6$ seconds. *Cyclone* will not airborne enemies who were already hit by either **Wukong** or his **clone** within the same cast. *Cyclone* can be recast after $0.5$ seconds within the duration, and does so automatically afterwards or by casting another ability. **Recast:** **Wukong** ends *Cyclone*. After 1 second, he can cast *Cyclone* a second time within 8 seconds of the first cast. **Nimbus Strike*’s* *bonus attack speed duration is continuously refreshed while spinning. 'Cyclone's damage is capped at 200@6; 400@11; 600@16 per second against monsters.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $130-90$ seconds |
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Targeting** | Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | physical |
| **Effect Radius** | $162.5$ units |
| **Spell Shield** | Special |
| **Spell Effects** | aoedot |

**Scaling:**
- **Physical Damage Per Tick:** $8/8-16/8$% of target's **maximum* health (+ $275/8$% AD)*maximum** health (+ 275% AD)
- **Maximum Total Physical Damage:** $8×2-16×2$% of target's
- **maximum** health (+ $275×2$% AD)

**Notes:**

- *Cyclone* is special cased to grant stacks of *Conqueror* for each tick of damage.
- Casting a basic ability will always interrupt ''Wukong's* *Cyclone*. A clone can be interrupted by casting *Crushing Blow*, but only if the clone was made during the current *Cyclone' cast and only for that cast. This means if both **Wukong** and his clone are spinning, the clone cannot be interrupted.
- 'Cyclone's remaining duration always stays the same even after the clone replicated the ability.
- During *Cyclone*, **Wukong** and his clone's attack ranges are reduced.
- Spell shield will block one instance of damage.
  - Blocking the airborne will still grant immunity to it from either of 'Cyclone's casts.

---

## Patch History

### V25.16
- *Nimbus Strike*
  - **Bug Fixes:** No longer interrupts the attack windup of enemies targeting **Wukong** if the *Auto Attack* option is not enabled.
- *Crushing Blow*
  - **Bug Fixes:** Attacking a ward no longer briefly reveals the user globally.
- *Warrior Trickster*
  - Cooldown increased to $20-16$ seconds from $18-14$.
- *Nimbus Strike*
  - Bonus attack speed reduced to $35-55$% from $40-60$%.

### V14.24
- *Warrior Trickster*
  - **Bug Fixes:** Clone death no longer triggers jungle invasion pings.

### V14.22
- Stats
  - Base mana increased to 330 from 300.
- *Warrior Trickster*
  - Cooldown reduced to $18-14$ seconds from $22-14$.

### V14.17
- *Crushing Blow*
  - **Bug Fixes:** Casting now properly reveals a Fog of War bubble around himself and any clone (with the same effects as attacking).

### V14.16
- *Stone Skin*
  - Bonus armor per stack increased to 6 to 10 from 5 to 9.
    - Maximum bonus armor increased to 6×6 to 10×6 from 5×6 to 9×6
- *Nimbus Strike*
  - Bonus attack speed increased to $40-60$% from $35-55$%.

### V14.11
- *Warrior Trickster*
  - **Bug Fixes:** *Statikk Shiv* *Electroshock* no longer unintentionally triggers from the clone expiring if the enemy who owns the item hit it at any point during its active time.

### V14.5
- *Stone Skin*
  - Maximum stacks reduced to 5 from 10.
  - Bonus effects per stack increased to 100% from 50%.
  - **New Effect:** Stacks now fall off at a rate of 1 per second after the duration expires.

### V14.3
- *Crushing Blow*
  - Bonus range increased to $135-175$ from $75-175$.
  - Bonus AD ratio increased to 55% *bonus AD from 45%.
- *Nimbus Strike*
  - Cooldown reduced to $10-7$ seconds from $10-8$.

### V13.16
- Stats
  - Mana regeneration growth increased to $0.8$ from $0.65$.
- *Crushing Blow*
  - Mana cost reduced to 20 from 40.
- *Warrior Trickster*
  - Mana cost reduced to $60-40$ from $80-40$.

### V13.14
- *Warrior Trickster*
  - **Undocumented/Bug Fix:** Pet can now consistently be commanded to attack inhibitors and the Nexus via their controller spells or automatic attack priorities.

## Trivia

- Wukong is based on Sun Wukong from the Chinese novel Journey to the West.
- Coincidentally, much of **Garen**’s kit was a possible basis for Wukong, mainly due to both characters having:
  - A "Q" ability that originally stated its scaling to be over 100% in attack damage as an auto-attack booster and reset with an extra effect (*Decisive Strike* silences with a movement speed boost while Crushing Blow reduces armor by a percentage), and applies on-hit effects as well.
  - A passive ability that increases their defensive stats (with Garen's being a normal ability with not only percentage defensive boosts, but also with a damage reduction active while Wukong's only being a standard passive with flat defense/HP boosts).
  - Point-and-click spells; Garen's can only target one champion with it and acts as his ultimate; Wukong's can hit two extra targets in range, moves him to said target(s) and acts as a normal ability. Both versions at some point have interacted with magic damage (Demacian Justice originally inflicted bonus magic damage while Nimbus Strike now scales with magic damage and ability power as of his gameplay update).
  - *Cyclone* and *Judgment* involve a very similar spinning animation and near-identical mechanics in the way their hits are racked up, how they provide bonus movement speed during cast time, and cancellation. A number of certain item actives and summoner spells can even be used during their cast times, though depending on the champion there are exclusive item actives only available to them during such abilities, as Wukong before his update could not activate *Tiamat* / *Ravenous Hydra* active during *Cyclone*. *Cyclone* also cannot critically strike, unlike *Judgment*.
    - Likewise, both their ultimates (*Demacian Justice* and *Cyclone*) have HP-based-damage-scaling against their targets.
- The name of *Cyclone* on the Chinese Servers, 大鬧天宮 (Traditional Chinese characters) 大闹天宫 (Simplified Chinese characters) (*Dànào Tiāngōng*, lit. *Greatly Wrecking the Heavenly Palace*) references the Monkey King's rebellion against the Jade Emperor & other heavenly government's officials, until Buddha sealed him the Five Finger Mountain (*Wǔzhǐshān*, 五指山) for five centuries, before embarking on his westward journey to redeem himself.
- Other MOBA than *League of Legends* (such as *Heroes of Newerth*, *Smite*, and *Dota 2*) also feature a character based on Sun Wukong. These expies of the Monkey King (including 'Smite's pre-worked Sun Wukong)) have similar abilities and playstyles (mainly being damaging tanks with assassin potential, and having very unique tricks with their abilities).
- Wukong is one of a few champions to have multiple textures in one skin. When he uses *Warrior Trickster*, his clone will be of a different colour palette which can only be seen by Wukong and his allies. To the opposing team, the two Wukongs look exactly the same, including copies of any external buffs and item effects active at the moment the clone is spawned (Prior to patch V5.22, clones did not duplicate external buff/active item effect particles).
  - Four other champions with this feature are **LeBlanc** (via *Mirror Image*),**Shaco**’s (via *Hallucinate*), **Nasus** (via *Fury of the Sands*) and **Malphite** (via *Granite Shield* and *Thunderclap*).
- Wukong's critical strike animation is a possible nod to a famous pole-vaulting kick used by many incarnations of the original Sun Wukong, especially in the TV drama adaptations of *Journey to the West*, and his *Warriors Orochi / Musou Orochi* incarnation as well.
- When a Wukong Bot uses *Warrior Trickster*, the decoy is named "MonkeyKing Bot".
- Wukong is the only champion with two Lunar Revel skins.
- Before his gameplay update, *Cyclone* was originally tied as one of the highest AD ratio abilities in the game.
- Wukong's Series 2 Eternals make the following references:
  - *Monke Strong* is a reference to the Return to Monke meme.

---
*This page was automatically generated from League of Legends Wiki data.*