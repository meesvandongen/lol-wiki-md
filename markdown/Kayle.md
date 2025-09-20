# Kayle

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Kayle |

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

### Passive: Divine Ascent

**Innate:** **Kayle** ascends through four forms that each grant additional bonuses: * **Level 1 - Zealous:** ''Kayle's** basic attacks on-attack grant her stack as, and at maximum stacks she also gains ms. * **Level 6 - Arisen:** **Kayle** gains range and becomes a ranged champion. * **Level 11 - Aflame:** While at maximum stacks, **Kayle's** basic attacks launches a fire wave that deals magic damage. * **Level 16 - Transcendent:** **Kayle'' is permanently at maximum stacks, in addition to gaining even more range.

**Innate:** **Kayle** ascends through four forms that each grant additional bonuses. She ascends upon spending a skill point at 1, 6, 11, and 16. **Level 1 - Zealous:** ''Kayle's'* basic attacks on-attack generate a stack of *Zeal* for 5 seconds, refreshing on subsequent attacks and stacking up to 5 times. For each stack, she gains attack speedbonus attack speed*, up to a maximum of attack speed (+ 5% per 100 AP). At max stacks, she becomes *Exalted', gaining ms*bonus** movement speed*. **Level 6 - Arisen:** **Kayle** becomes ranged and gains range*bonus** attack range* for a total of range. **Level 11 - Aflame:** **Kayle** gains 10% bonus size. While **Kayle** is *Exalted*, her basic attacks on-attack also launch a *wave* of fire forward that deals $20 to 41 for 8$ (+ 10% *bonus AD) (+ 25% AP) magic damage to all enemies it passes through. The *wave* is affected by critical strike modifiers. **Level 16 - Transcendent:** **Kayle** gains an additional range*bonus** attack range* for a total of range, and permanently gains the full effects of *Zealous*.

**Notes:**

- The stacking *bonus attack speed can be tracked on the HUD via a buff called **Zeal**, while the maximum stacks bonus grants the buff **Exalted**.
- Subsequent applications will also refresh the duration of all stacks, but all of them are lost when expired.
- If **Kayle** is in her attack animation after the stacks expire, she gains another $0.5$ seconds to refresh/stack **Zeal**.
- Each stack grants *bonus attack speed based on ''Kayle's '' ability power at the time the stack is generated and will not update if it is changed.
  - For example, if a stack is generated while she has 0 ability power, it'll grant 6% attack speed. If she acquired 100 ability power afterward, the next stack will grant 7%. The 2 stacks grant a total of 13% *bonus attack speed and not 14%. **Arisen**
- Basic attacks within range will still use ''Kayle's'' melee basic attack animations and do not utilize projectile.
  - They are still classified as ranged attacks.
  - Due to this fact, projectile-blocking effects cannot intercept basic attacks while ''Kayle's'' target is within her "melee" range. **Aflame**
- On becoming **Aflame**, ''Kayle's'' bonus grows linearly over 1 second.
- Basic attacks against [structures](./structures.md) do not trigger fire waves.
- While at 4 stacks of **Zeal**, the next attack on-attack reaching the fifth stack will be empowered by **Aflame**, releasing the first fire wave. **Transcendent**
- The **Zeal** and **Exalted** buffs are replaced by a new buff that grants the benefits of both and it is not visible on ''Kayle's '' HUD.
- This buff also updates its attack speed value whenever a stat update happens and ability power is changed, however, it does not track ability power from other buffs, unless they have been specifically special cased. The following buffs are not special cased yet and will not grant bonus attack speed:
  - [Rune](./Rune.md)s (including [Adaptive Force rune shards](./Runes_Reforged.md#Shards) )
  - **Fire Waves Details**
- A wave consists of three projectiles for the hitbox and a fourth for visuals.
- The hitbox projectiles spawn 75 units behind **Kayle**, travel through her, and are destroyed once they've traveled 850 units. Their technical details are:
  - **Range:** cr (755 effective)
  - **Width:** er
  - **Projectile Speed:** 2800
  - **Angle:** 18 (left and right at 9 to their side, respectively)
- Projectile-blocking effects can block each projectile individually.
  - It is possible to block the visuals, but still get hit by an unblocked "invisible" hitbox projectile.
- Hitting an enemy with multiple projectiles of the same wave will not increase the damage dealt.
- The fire wave shares cast instance with the triggering basic attack.
  - The amount of *Conqueror* stacks gained will be if the basic attack deals damage first or 2 if the wave does so.
- Keep in mind that it's still two damage instances. If one triggers *Bone Plating*, the other's damage will be blocked.
- Fire waves roll critical strike on each target hit individually.
- The distance of the fire waves scales with **bonus** attack range.
- A known issue is that despite dealing area damage the fire wave does not:
  - Consume from *Tear of the Goddess* and its upgrades.
  - Trigger *Manaflow Band*. **Interactions & Other**
- Because **Kayle** is both a melee and ranged champion, she can purchase both ranged and melee exclusive items at anytime, but their effects will function depending on her current range type; the same principle also applies to [runes](./runes.md).
  - However, keep in mind that, prior to ascending to *Arisen*, *Starfire Spellblade* also changes this behaviour based on its usage.
- Waves are fired in the direction that **Kayle** is facing. Changing that during the attack windup via will also change the direction of the wave.
- Each ascension has an unique animation and a voice line. '([See: Kayle's quotes](./Kayle/LoL/Audio.md))'
- The ascensions' animation will override the current one, but it does not cancel ''Kayle's'' current action, nor her previous orders.

---

### Q: Radiant Blast

**Active:** **Kayle** summons a celestial sword that travels forward and expands upon hitting a target, dealing magic damage, slow, and reducing the armor and magic resistance of all targets struck.

**Active:** **Kayle** conjures a portal in front of her that faces the target direction, from which a celestial sword launches forward. The sword expands upon hitting an enemy, targets struck in the area are dealt magic damage, slow for 2 seconds, and inflicted with 15% **reduced** armor and magic resistance for 4 seconds.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $12-8$ seconds |
| **Cast Time** | Attack Windup Time |
| **Cost** | $70-90$ Mana |

**Scaling:**
- **Magic Damage:** $60-220$ (+ 60%
- *bonus AD) (+ 50% AP)
- **Slow:** $26-50$%

**Notes:**

- This ability has a cast time and cast delay. The former represents the amount of time Kayle needs to spend to cast and is equal to her attack windup time. The latter is the amount of time the portal needs to launch the sword, which is always seconds, starting from the beginning of the cast time.
  - As implied by the previous point, the portal is created at the start of the cast time and **Kayle** does not have to be alive for it to launch the sword.
  - The portal does not follow ''Kayle's'' movements, using or getting displaced won't change its position nor direction either.
- The cast time can be calculated by dividing ''Kayle's'* *windup percent' value ($0.193555$) with her current total attack speed.
  - 'Radiant Blast's' cast time is at base attack speed, is at most at the minimum attack speed of attack speed, and is at the attack speed cap of attack speed. The cast time can be further decreased with effects that allows the player to bypass the attack speed cap.
  - Additionally, it should be noted that due to action only taking place on whole game ticks, the cast time can only be changed in fractions of $0.033$ seconds. This also means that the lowest possible cast time is $0.033$ seconds, requiring at least attack speed.
  - Important to note that in patch [V9.17](./V9.17.md), 'Radiant Blast's' cast time was changed to be a flat $0.25$ seconds, rather than scaling with the attack windup time. In patch [V10.4](./V10.4.md), this was undocumentedly changed back. Whether the change to scale with windup time again was intentional or not is unknown.
- The resistance reduction is applied after the damage has been dealt.
- The expansion consists of 5 hitboxes, forming a cross-like shape centered 100 units ahead from the impact in its direction. Their technical details can be seen below and visualized on the image to the right.
  - ExplosionCenter: 100 radius
  - ExplosionLeft: 150 distance, 125 width
  - ExplosionRight: 150 distance, 125 width
  - ExplosionBackward: 100 distance, 90 width
  - ExplosionForward: 400 distance, 90 width **Interactions & Other**
- The visual effects of the explosion shows the blade and guard of ''Kayle's'' swords.
- The resistance reduction can be tracked on the HUD as a debuff called **Sundered**.
- Note that the PsyOps skin still uses the old shred VFX, which also only shows for 3 seconds (effect duration is not affected). This is possibly an oversight as all existing, and at the time released Dragon Slayer, skins were updated properly in [V11.18](./V11.18.md).

---

### W: Celestial Blessing

**Active:** **Kayle** herself and an allied champion. Additionally, both of them gains *move speed* for a brief time.

**Active:** **Kayle** and the target allied champion are heal and gain **bonus movement speed** for 2 seconds. If cast without a valid target, or [self-cast](./self-targeted.md), *Celestial Blessing* will automatically target the closest allied champion in range, prioritizing the one with the lowest *health*.

| Attribute | Value |
|-----------|-------|
| **Range** | 900 units |
| **Cooldown** | 15 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | $70-90$ Mana |

**Scaling:**
- **Heal:** $55-155$ (+ 25% AP)
- **Bonus Movement Speed:** $24-40$% (+ 8% per 100 AP)

**Notes:**

- 'Celestial Blessing's' cast time does not interrupt movement and the effects are applied at the start. **Interactions & Other**
- If cast with [auto-targeting](./Auto-targeted.md), *Celestial Blessing* may target allies who are untargetable, or allied clones that cannot be targeted by allies (such as ).

---

### E: Starfire Spellblade

**Passive:** **Kayle** deals **bonus** magic damage on-hit.

**Active:** **Kayle** kindles her blade, causing her next basic attack to deal additional **bonus** magic damage based on the target's **missing** health.

**Passive:** **Kayle** deals **bonus** magic damage on-hit. **Active:** **Kayle** kindles her blade, empowering her next [basic attack](./basic_attack.md) within 6 seconds to have an uncancellable windup and deal additional **bonus** magic damage on-hit that is capped at 400 against monsters. If **Kayle** is not yet **Divine Ascent**, this attack becomes ranged with range. ***Divine Ascent* The attack explodes upon the target, dealing its damage to surrounding enemies and applying on-hit effects. This is affected by critical strike modifiers. *Starfire Spellblade basic attack reset *'Kayle's'* basic attack timer. Starfire Spellblade does not deal bonus damage against [structures](./structure.md).*

| Attribute | Value |
|-----------|-------|
| **Range** | radius= cr 350 units |
| **Cooldown** | $8-6$ seconds |
| **Cast Time** | none |
| **Cost** | costtype = speed = |

**Scaling:**
- **Passive Damage:** $15-35$ (+ 10%
- *bonus AD) (+ 20% AP)
- **Bonus Magic Damage:** $8-10$%

**Notes:**

- *Starfire Spellblade* deals:
  - The physical damage part of both the attack and the explosion deals basic damage.
  - for the passive's on-hit effect.
  - for the active's on-hit effect.
  - All damage dealt to secondary targets by the explosion is additionally tagged as area damage.
- The active's effect shares the same cast instance with the triggering attack (exception: *Muramana*, see **Interactions & Other**).
- Parry interactions:
  - Blind: The main target will never receive damage, but the secondary targets will receive the active's spell damage.
  - Block: Blocking will always negate all damage. Keep in mind that if a secondary target is outside of zone, they won't block damage.
  - Dodge: Dodging will always negate all damage.
- : All parry interactions are marked for retesting because of patch [V11.1](./V11.1.md) changes. **Interactions & Other**
- While the active's effect is up, a circle is visible to **Kayle**, indicating her *range, however, this circle only takes into account aisRapid Firecannon*.
  - The increased range (with the item's effect included) will be range prior to ascending to **Transcendent**, after which it's range.
- To clarify the description, *Starfire Spellblade* can behave both as a ranged and melee attack based on how far the target is prior to **Kayle** ascending to *Divine Ascent*, after which the attack will be always using ranged standards.
  - If the target is 200 units away or closer, the attack will be considered melee. This also uses her melee animations, not utilizing projectiles even after ascending.
  - If the target is over 200 units, the attack will be considered ranged.
- To further expand on this: Using the ranged empowered attack will change ''Kayle's** range type to ranged while the projectile is alive. This means that if the target could outrun the projectile, **Kayle'' would be stuck on being ranged, although the bonuses from the active are not kept. While it's unlikely to be on this temporary ranged state for longer than half a seconds during a normal playthrough, it is possible to achieve few seconds when specific conditions are met.
- Keep in mind that while on-hit effects will be applied to all targets in the explosion, on-attack effects are only applied once per basic attack, not per target hit.
- Single-use on-hits, such as Spellblade, will be applied to the main target.
  - If the main target died prior to the projectile reaching them, or if it was a [jungle plant](./Jungle_plants.md), the effects will be applied to the closest target from the explosion's center.
- can only be applied to the main target.
- Despite being one cast instance, *Starfire Spellblade triggers **bonus** physical damage physical damage.

---

### R: Divine Judgment

**Active:** **Kayle** grants herself or an allied champion invulnerable for the next few seconds, and conjures flaming swords around herself.

*At the end of the duration, the swords rain down around the target, dealing magic damage to enemies hit.*

**Active:** **Kayle** grants herself or a target allied champion invulnerable for $2.5$ seconds and conjures swords of fire. After $2.5$ seconds, she rains the swords down around the target, dealing magic damage to nearby enemies. *Divine Judgment will prioritize casting on allies over **Kayle**.* This part has been moved to notes and received some additional notes. -->

| Attribute | Value |
|-----------|-------|
| **Range** | 900 units |
| **Cooldown** | $160-80$ seconds |
| **Cast Time** | $0.5$ seconds |
| **Cost** | $100-0$ mana |

**Scaling:**
- **Magic Damage:** $200-400$ (+ 100%
- *bonus AD) (+ 70% AP)

**Notes:**

- 'Divine Judgment's* cast time does not interrupt movement and the cast will finish even after *'Kayle's'' death.
- Since raw damage ignores invulnerability, the target may still die via the Nexus Obelisk, but the ability will finish at their corpse's position regardless.
- *Divine Judgment* will prioritize casting on allies over **Kayle**. This means that if Kayle's and an ally's selection ranges are intercepting each other on the cursor's position, the cast will always occur on the ally.
- Prior to becoming **Transcended** via *Divine Ascent*, casting *Divine Judgment* refreshes the duration of *Divine Ascent 2* stacks at the start of the cast time to ensure that it does not expire while she can't attack during the cast.
- During cast, an indicator for the damaging area can be seen around the target, however, this is not visible if there is no sight on the them.
- *Divine Judgment* can be cast on clone too, including *Shapesplitter* and *Warrior Trickster*, which are normally not targetable for allies.
- While it's an uncommon occurence, it is possible to cast *Divine Judgment* on targets who just entered resurrection or zombie state. This is only possible with very strict timing and perhaps high latency.
- It's recommended to avoid both of the afformentioned interactions, as they render the usage of *Divine Judgment* useless in many cases. **Interactions & Other**
- While the target is invulnerable, a number indicator will appear beneath them whenever they would receive damage to show the amount that was entirely mitigated. This is calculated pre-mitigation, meaning before resistances and reductions are taken into account.
- effect behaves strangely with the set basic attack range:
  - If the ability is cast while the effect is ready, the range goes from range to range over 4 stat updates (400 -> 390 -> 387 -> 385). When the cast completes, the proper range needs 2 updates to set in (range - range while **Arisen**; range - range while **Transcended**).
  - Gaining the effect during the cast will cause the range to cycle between range and range until the cast finishes, then the proper range is set after 1 update.
  - Selling the item during the cast causes the range to be set to range. Range values are outdated because of patch [V11.18](./V11.18.md) changes to **Kayle**.
- The invulnerability can be tracked on the HUD via a buff called **Intervention**.

---

## Patch History

### V25.15
- General
  - Default range type changed to melee from ranged.

### V25.14
- General
  - Default range type changed to ranged from melee.
    - *Starfire Spellblade* attacks can still behave as melee while attacking within 200 units before becoming Arisen. This is unchanged.

### V14.22
- *Divine Ascent*
  - Bonus attack speed AP ratio per stack increased to 1% per 100 AP from $0.5$% per 100 AP.

### V14.21
- *Celestial Blessing*
  - Mana cost reduced to $70-90$ from $90-130$.

### V14.10
- 
  - ***Bug Fixes:*** Properly implemented her unique HUD portrait before ascending to *Divine Ascent* (level 11).

### V14.9
- 
  - ***Bug Fixes:*** Properly implemented her unique HUD portrait before ascending to *Divine Ascent* (level 11).

### V14.5
- *Starfire Spellblade*
  - ***New Effect:*** Now triggers spell effects upon dealing damage.
- *Divine Ascent*
  - Bonus attack speed AP ratio per stack reduced to $0.5$% per 100 AP from 1%.

### V13.17
- 
  - ***Bug Fixes:*** Smoke VFX during her now uses the correct shape.
- 
  - ***Bug Fixes:*** Her champion icon on the scoreboard and minimap now use the correct icon to reflect her third form at level 11.

### V13.10
- *Divine Judgment*
  - ***Bug Fixes:*** Sword VFX are no longer missing on her body during the cast animation.

### V13.9
- General
  - ***Bug Fixes:*** Several SFX and animations now properly play.
- *Divine Ascent*
  - Aflame base damage changed to $20 to 41 for 8$ from $type=[[File:Starfire Spellblade.png$.
- *Starfire Spellblade*
  - Passive AP ratio reduced to 20% AP from 25% AP.
- *Divine Judgment*
  - Cast time reduced to $0.5$ seconds from $1.5$.
  - Area of effect delay increased to $2.5$ seconds from $1.5$ seconds.
  - Duration changed to $2.5$ seconds at all ranks from $2-3 3$.
  - Effect radius increased to $675/675/775$ units from 500 at all ranks.
  - Base damage reduced to $200-400 3$ from $200-500 3$.
  - AP ratio reduced to 70% AP from 80% AP.
  - ***Removed:**** No longer reduces her attack range to 400 if she self-casts.

## Trivia

- Kayle was the first to have her dance changed after release, (followed by **Veigar**) the first to have her abilities reworked three times since release, (followed by **Ryze**) and the first to use Chinese artwork outside Asian servers.
- Kayle - **Morgana** is one of seven pairs of sibling champions (the others being **Cassiopeia** - **Katarina**, **Garen** - **Lux**, **Nasus** - **Renekton**, **Yasuo** - **Yone**, **Darius** - **Draven**, and **Vi** - **Jinx**).
  - Though not a pair, **Anivia**, **Ornn**, and **Volibear** are also siblings.
- During [Alpha Test](./Alpha_Test.md), she was simply called 'Judicator' and at one point she was incorrectly listed as a tank.
- Her old dance referenced Will Smith ["Jump on It"](https://www.youtube.com/watch?v=GCFRJEjM3fc) dance from The Fresh Prince of Bel-Air.
  - A side-by-side comparison can be seen [here](http://www.youtube.com/watch?v=FtsJLKw5aWA).
  - She shared this dance with **Singed**.
- Her new dance is a reference to the dance of Elaine Benes from [Seinfeld "The Little Kicks"](https://en.wikipedia.org/wiki/The_Little_Kicks).
- In the now-removed official League of Legends [forums](https://web.archive.org/web/20130726165848/http://forums.na.leagueoflegends.com/board/), the original icon of *Intervention* was used to represent the "Mac Client" section.
- **Kayle**, **Elise**, **Gnar**, **Jayce**, and **Nidalee** are the only champions to be conditionally considered both ranged and melee.
- The projectile speed of Kayle's basic attack is the third highest in the game among all ranged attackers that utilize projectiles, at 5000.
  - The highest and second highest are and at 92400 and 10000, respectively.
- The first buff from her innate ( **Zeal**) shares its name with the item *Zeal*.
- Her dance references Elaine Benes [dance](https://www.youtube.com/watch?v=HQu_NLRvULM) from Seinfeld.
  - A side-by-side comparison can be seen [here](https://youtu.be/Db6HGmrm9RM?t=50).
- In Ultimate Spellbook, **Kayle** ascends without the player's input. This is caused by the fact that the choosen Ult-ernate Summoner Spells are automatically ranked up when the required levels are reached, which also matches the levels that the player may initiate the ascensions on.

---
*This page was automatically generated from League of Legends Wiki data.*