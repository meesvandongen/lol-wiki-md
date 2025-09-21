# Cho'Gath

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
| **Champion** | Cho'Gath |
| **Title** | the Terror of the Void |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-06-26 |
| **Release Patch** | June 26, 2009 Patch |
| **Roles** | Specialist |
| **Riot Positions** | Top |
| **External Positions** | Top |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $644.0$ | $+94.0$ | $2242.0$ |
| **Mana** | $270.0$ | $+60.0$ | $1290.0$ |
| **Health Regen** | $9.0$ | $+0.85$ | $23.4$ |
| **Mana Regen** | $7.2$ | $+0.45$ | $14.9$ |
| **Armor** | $38.0$ | $+4.5$ | $114.5$ |
| **Magic Resist** | $32.0$ | $+2.05$ | $66.8$ |
| **Attack Damage** | $69.0$ | $+4.2$ | $140.4$ |
| **Attack Speed** | $0.658$ | $+1.4\%$ | $0.819$ |
| **Movement Speed** | $345.0$ | $+0.0$ | $345.0$ |
| **Attack Range** | $125.0$ | $+0.0$ | $125.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.658$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $1.4\%$ |
| **Acquisition Radius** | $500 units$ |
| **Gameplay Radius** | $80 units$ |
| **Pathing Radius** | $50 units$ |
| **Selection Radius** | $130 units$ |
| **Selection Height** | $150 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Carnivore

**Innate:** Whenever **Cho'Gath** kills an enemy, it heal and restores *mana*.

**Innate:** Whenever ''Cho'Gath'' kills an enemy, it heals for 18 to 52 and restores .

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- *Carnivore* will also trigger when ''Cho'Gath'' destroys a turret, but not from other structures.
- *Carnivore* does not trigger upon destroying wards and possibly other similar units.
  - It does trigger upon killing a *Tentacle*. [https://www.youtube.com/watch?t=161&v=_65aFxwtPfw]

---

### Q: Rupture

**Active:** **Cho'Gath** ruptures the target location, dealing magic damage and airborne enemies within, slow them upon landing.

**Active:** ''Cho'Gath'' ruptures the target location after a delay, granting sight of the area before dealing magic damage to enemies within and airborne for 1 second, and afterwards slow them by 60% for $1.5$ seconds.

| Attribute | Value |
|-----------|-------|
| **Range** | 950 units |
| **Cooldown** | 6 seconds |
| **Cast Time** | $0.5$ seconds |
| **Cost** | 50 mana |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Effect Radius** | 250 units |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**Scaling:**
- **Magic damage:** $80-300$ (+ 100% AP)

**Notes:**

- The delay before the rupture does not include the cast time.
- The animation is visible in brush and in fog of war.
- Cleanse the airborne will not prevent the slow.
  - The slow itself can also not be blocked by spell shield, but will always be prevented if the airborne was.
- The area will still rupture even if ''Cho'Gath'' dies during the delay.

---

### W: Feral Scream

**Active:** **Cho'Gath** roars in a cone, dealing magic damage and silence enemies hit.

**Active:** ''Cho'Gath'' roars in a cone in the target direction, dealing magic damage to enemies hit. Enemy champions and Rift Scuttler hit are also silence for a duration.

| Attribute | Value |
|-----------|-------|
| **Range** | 650 units |
| **Cooldown** | $11-9$ seconds |
| **Cast Time** | $0.5$ seconds |
| **Cost** | $70-90$ mana |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | False |

**Scaling:**
- **Magic damage:** $80-280$ (+ 70% AP)
- **Silence Duration:** $1.6-2$ seconds

**Notes:**

- *Feral Scream* uses edge range for enemy targets only; Its range is *center-to-edge*.

---

### E: Vorpal Spikes

**Active:** **Cho'Gath**’s next few basic attacks launch forth deadly spikes, dealing magic damage to enemies hit based on **Feast** stacks and each target's **maximum** health, and slow them.

**Active:** ''Cho'Gath** empowers its next 3 basic attacks within 6 seconds to gain range*bonus'' range* and launch a blast of spikes on-attack in the target's direction. Enemies struck are dealt magic damage and slow by an amount that decays over $1.5$ seconds. The damage based on the target's health ratio is capped at 200 against monsters. The width of the blast of spikes increases based on ''Cho'Gath's* character modifier when they launch, including but not limited to the bonus from **Feast*' stacks. *Vorpal Spikes basic attack reset *'Cho'Gath's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $8-4$ seconds |
| **Cast Time** | none |
| **Cost** | 30 mana |
| **Targeting** | Auto |
| **Affects** | Self / Enemies |
| **Damage Type** | Magic |
| **Speed** | 1475 units/second |
| **Spell Shield** | False |
| **Spell Effects** | aoe |
| **Projectile** | True |

**Scaling:**
- **Magic Damage:* $20-100$ (+ 30% AP) (+ $2.5-3.9$% 3-100×3$ (+ 90% AP) (+ $2.5×3-3.93$% **Slow:** $30-50$%

**Notes:**

- The missiles are launched from an offset 25 units in front of Cho'Gath and towards the target's location on-attack-complete.
  - When attacking enemies within 25 units center-to-center of ''Cho'Gath'', the missile will fire in the opposite direction.
- ''Cho'Gath's* factors into the wideness of the hitbox *Vorpal Spikes' has. External modifiers (*Wild Growth*) contribute to this.
  - At certain thresholds, the missile gets replaced by a different one with a different width. At each step, the width grows by 30, except for the 3rd missile, where it only grows by 10.
  - Some of the thresholds are placed very weird.
  - The missile range is unchanged.
- The empowered attack will trigger but not apply its effects against structures.

---

### R: Feast

**Active:** **Cho'Gath** devours an enemy, dealing true damage based on ''Cho'Gath's bonus** health. If this attack kills the target, **Cho'Gath'* gains a permanent *Feast' stack that infinitely stacks.

**Active:** ''Cho'Gath** attempts to eat the target enemy, dealing them *true damage*. Against non-champions, the **base'' damage is modified. If the target is killed, ''Cho'Gath'* gains a stack of *Feast'. Only 6 stacks can be gained from non-epic monster or minion. Each stack of *Feast* increases the cast range of the ability by $2.5$, for a maximum increase of 25 at 10 stacks, and grants ''Cho'Gath** as well as *range **bonus** attack range* and increased size, capping at range*bonus'' attack range* and 100% increased size.

| Attribute | Value |
|-----------|-------|
| **Range** | 175 units |
| **Cooldown** | $80-60$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 100 mana |
| **Targeting** | Unit |
| **Affects** | Enemies / Self |
| **Damage Type** | True |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- ** health) ** Non-Champion True Damage:
- ** 1200 (+ 50% AP) (+ 10% ** bonus** health)
- **Bonus Health Per Stack:** $80-160$
- **Bonus Attack Range Per Stack:** $4.7-7.7$
- **Bonus Size Per Stack:** $(60-100)/10$%

**Notes:**

- The increased stacks additively with other increases.
- The name of the bonus health buff is *"Feast"*.
- The bonus attack range and increase cap at respectively $75/((60-100)/13) round=2$ and $100/((60-100)/10) round=2$ stacks, with the last stack being effective as the decimal part of the value (no decimal meaning 100% as effective).
- Each stack of *Feast* increases the damage by $8/12/16$.
- If an enemy champion has health below the amount of true damage *Feast* deals, they will be marked for execution. This indicator does not consider shield or invulnerability.
- Consuming any additional Voidgrub from the same spawn group will be tracked as a "monster eaten" for 'Feast's buff, but will neither grant any bonuses nor count towards the 6-stack limit.
- While not on *cooldown*, 'Feast's ability icon on the HUD can be pinged to announce its damage to monsters in the ally chat. <!--
  - Pending Test: The kill success check is after all additional instant damage applied. If the additional instant damage triggered by Feast from Cho'Gath killed the target then Chogath will still earn the stack, eg: Dark Harvest, Electrocute, Cheap Shot, etc killed the target . Non-instant damage that requires another check such as Aery, Comet, Hextech & Infernal dragon soul, Luden Echo, The Collector etc does not add however. Hextech Alternator (including Night Harvester)?? *Possibly bug: Feast has special interaction toward Shaco R Hallucination where it counted as champion stack.-->

---

## Patch History

### V25.11
- Stats
  - Armor growth reduced to $4.5$ from 5.

### V25.10
- Stats
  - Base attack speed increased to $0.658$ from $0.625$.
- *Rupture*
  - Base damage reduced to $80-300$ from $80-320$.
- *Vorpal Spikes*
  - Health ratio increased to $2.5-3.9$% of target's **maximum** health from $2.5-3.5$%.

### V25.09
- *Feast*
  - **New Effect:** While the ability is not on cooldown, its icon can now be pinged to display the damage to monsters in chat, similarly to Smite.

### V25.05
- *Rupture*
  - Base damage reduced to $80-320$ from $80-340$.
- *Feral Scream*
  - Base damage reduced to $80-280$ from $80-300$.
- *Vorpal Spikes*
  - Health ratio changed to $2.5-3.5$% of target's **maximum** health from 3% at all ranks.

### V14.14
- *Rupture*
  - Base damage increased to $80-340$ from $80-320$.
- *Vorpal Spikes*
  - Base damage changed to $20-100$ from $22-82$.

### V14.10
- *Vorpal Spikes*
  - **Removed:*** Health ratio damage is no longer capped against minions.
  - Monster damage cap increased to 200 at all ranks from $60-140$.

### V14.7
- Cho'Gath
  - **Bug Fixes:** During a basic attack, arms now follow their proper animation instead of breaking from the model's skeleton or readjusting abruptly.

### V14.6
- *Feral Scream*
  - Cooldown reduced to $11-9$ seconds from $13-9$.
- *Vorpal Spikes*
  - Base damage increased to $22-82$ from $22-70$.

### V14.2
- *Feast*
  - Bonus attack range per stack increased to $4.7-7.7 3$ from $(60-100)/13 3 round=2$.
  - **New Effect:** Cast range is now increased by $2.5$ per stack, up to 25 at 10 stacks.
  - Extended tooltip now displays the bonus range granted for basic attacks and the ability.

### V13.6
- *Carnivore*
  - Cho'Gath heal VFX is now unique from Cho'Gath.

## Trivia

- 
  - In Cho'Gath's case, *Feast* infinitely stacks his health.
- Cho'Gath was the first champion to receive a second skin (though **Corki* has two skins although one is Legacy and the other one is available).
- Cho'Gath's dance references Fantasmic! by Mickey Mouse.
- Riot created an Adobe Flash minigame dedicated for him called Cho'Gath Eats the World for April Fools' Day 2013.
  - Another minigame released on the same date, Astro Teemo, which was unlocked by playing to a certain point in this game.
- The depiction of Cho'Gath in the icon of *Feral Scream* was based on his pose in his Cho'Gath.
- In the now-removed official League of Legends forums, the original icon of *Feral Scream* was used to represent the "General Discussion" section.
- In the official League of Legends game trailer from 2009, Cho'Gath was given generic T-Rex sounds.
- In the V1.0.0.115 April Fools' Day patch;
  - The following changes regarding Cho'Gath were jokingly listed：
    - Cho'Gath now starts the game the same as if he had 6 *variant=old*.
    - *variant=old* still increases Cho'Gath's size.
    - brush can no longer stealth Cho'Gath.
  - The following joke item was listed to be added:
    - NEW Yordle Saddle: Any yordle champion can buy this new item to Cho'Gath.
- Cho'Gath was given a retexture and features as a monster called the "Big Stompy Monster" (along with its huge variant) in Invasion, a featured game mode.

---
*This page was automatically generated from League of Legends Wiki data.*