# Kayle

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
| **Champion** | Kayle |
| **Title** | the Righteous |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Latest Changes** | V25.15 |
| **Roles** | Specialist |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 2400 |
| **Riot Points** | 260 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Alt Type** | Marksman |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 1 |
| **Mobility** | 1 |
| **Utility** | 3 |
| **Style** | 20 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $670.0$ | $+92.0$ |
| **Mana** | $330.0$ | $+50.0$ |
| **Health Regen** | $5.0$ | $+0.5$ |
| **Mana Regen** | $8.0$ | $+0.8$ |
| **Armor** | $26.0$ | $+4.2$ |
| **Magic Resist** | $22.0$ | $+1.3$ |
| **Attack Damage** | $50.0$ | $+2.5$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.667$ | |
| **Bonus AS per Level** | $1.5\%$ | |
| **Attack Windup** | $19.4\%$ | |
| **Missile Speed** | $5000$ units/second | |
| **Acquisition Radius** | $600$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $140$ units | |
| **Selection Height** | $240$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $110.0\%$ |

#### URF

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $85.0\%$ |
| **Damage Taken** | $110.0\%$ |

## Abilities

### Passive: Divine Ascent

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Effects** | AoE |
| **Projectile** | Special |
| **Parry** | False |

**INNATE:** **Kayle** ascends through four forms that each grant additional bonuses. She ascends upon spending a skill point at experience 1, 6, 11, and 16.

**LEVEL 1 - ZEALOUS:** **Kayle**’s basic attacks on-attack generate a stack of *Zeal* for 5 seconds, refreshing on subsequent attacks and stacking up to 5 times. For each stack, she gains (attack speed) 6% (+ 1% per 100 AP) **bonus** attack speed, up to a maximum of (attack speed) 30% (+ 5% per 100 AP). At max stacks, she becomes *Exalted*, gaining (ms) 10% **bonus** movement speed.

**LEVEL 6 - ARISEN:** **Kayle** becomes ranged and gains (range) 350 **bonus** attack range for a total of (range) 525.

**LEVEL 11 - AFLAME:** **Kayle** gains 10% bonus size. While **Kayle** is *Exalted*, her basic attacks on-attack also launch a *wave* of fire forward that deals 20–41@11–18 (+ 10% **bonus** AD) (+ 25% AP) magic damage to all enemies it passes through. The *wave* is affected by critical strike modifiers.

**LEVEL 16 - TRANSCENDENT:** **Kayle** gains an additional (range) 100 **bonus** attack range for a total of (range) 625, and permanently gains the full effects of *Zealous*.

**Notes:**

- The stacking **bonus** attack speed can be tracked on the HUD via a buff called **Zeal**, while the maximum stacks bonus grants the buff **Exalted**.
- Subsequent applications will also refresh the duration of all stacks, but all of them are lost when expired.
- If **Kayle** is in her attack animation after the stacks expire, she gains another $0.5$ seconds to refresh/stack **Zeal**.
- Each stack grants **bonus** attack speed based on **Kayle's ** ability power at the time the stack is generated and will not update if it is changed.
  - For example, if a stack is generated while she has 0 ability power, it'll grant 6% attack speed. If she acquired 100 ability power afterward, the next stack will grant 7%. The 2 stacks grant a total of 13% (6% + 7%) **bonus** attack speed and not 14% (7% + 7%). **Arisen**
- Basic attacks within (range) 175 units will still use **Kayle**’s melee basic attack animations and do not utilize projectiles.
  - They are still classified as ranged attacks.
  - Due to this fact, projectile-blocking effects cannot intercept basic attacks while **Kayle**’s target is within her "melee" range. **Aflame**
- On becoming **AFLAME**, **Kayle**’s bonus grows linearly over 1 second.
- Basic attacks against structures do not trigger fire waves.
- While at 4 stacks of **Zeal**, the next attack on-attack reaching the fifth stack will be empowered by **AFLAME**, releasing the first fire wave. **Transcendent**
- The **Zeal** and **Exalted** buffs are replaced by a new buff that grants the benefits of both and it is not visible on **Kayle's ** HUD.
- This buff also updates its attack speed value whenever a stat update (A stat update happens once every 0.25 seconds.) happens and ability power is changed, however, it does not track ability power from other buffs, unless they have been specifically special cased. The following buffs are not special cased yet and will not grant bonus attack speed:
  - Runes (including Adaptive Force rune shards )
  - Infernal Might **Fire Waves Details**
- A wave consists of three projectiles for the hitbox and a fourth for visuals.
- The hitbox projectiles spawn 75 units behind **Kayle**, travel through her, and are destroyed once they've traveled 850 units. Their technical details are:
  - **RANGE:** 850 (755 effective)
  - **WIDTH:** 200
  - **PROJECTILE SPEED:** 2800
  - **ANGLE:** 18° (left and right at 9° to their side, respectively)
- Projectile-blocking effects can block each projectile individually.
  - It is possible to block the visuals, but still get hit by an unblocked "invisible" hitbox projectile.
- Hitting an enemy with multiple projectiles of the same wave will not increase the damage dealt.
- The fire wave shares cast instance with the triggering basic attack.
  - The amount of Conqueror stacks gained will be 2 if the basic attack deals damage first or 2 if the wave does so.
- Keep in mind that it's still two damage instances. If one triggers Bone Plating, the other's damage will be blocked.
- Fire waves roll critical strike chance on each target hit individually.
- The distance of the fire waves scales with **bonus** attack range.
- A known issue is that despite dealing area spell damage the fire wave does not:
  - Consume Mana Charge from Tear of the Goddess and its upgrades.
  - Trigger Manaflow Band. **Interactions & Other**
- Because **Kayle** is both a melee and ranged champion, she can purchase both ranged and melee exclusive items at anytime, but their effects will function depending on her current range type; the same principle also applies to runes.
  - However, keep in mind that, prior to ascending to *Arisen*, Starfire Spellblade also changes this behaviour based on its usage.
- Waves are fired in the direction that **Kayle** is facing. Changing that during the attack windup via Flash will also change the direction of the wave.
- Each ascension has an unique animation and a voice line. *(See: Kayle's quotes)*
- The ascensions' animation will override the current one, but it does not cancel **Kayle**’s current action, nor her previous orders.

---

### Q: Radiant Blast

| Attribute | Value |
|-----------|------:|
| **Range** | 900 (Missile range only; See notes) units |
| **Cast Time** | Attack Windup Time (Cast time is equal to this period.) |
| **Width** | 150 (Missile width) units |
| **Speed** | 1600 units/second |
| **Cost** | 70 / 75 / 80 / 85 / 90 Mana |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Kayle** conjures a portal in front of her that faces the target direction, from which a celestial sword launches forward.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 100 / 140 / 180 / 220 (+ 60% **bonus** AD) (+ 50% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 26 / 32 / 38 / 44 / 50% |

The sword expands upon hitting an enemy, targets struck in the area are dealt magic damage, slowed for 2 seconds, and inflicted with 15% **reduced** (armor) armor and (magic resistance) magic resistance for 4 seconds.

**Notes:**

- This ability has a cast time and cast delay. The former represents the amount of time Kayle needs to spend to cast and is equal to her attack windup time. The latter is the amount of time the portal needs to launch the sword, which is always seconds, starting from the beginning of the cast time.
  - As implied by the previous point, the portal is created at the start of the cast time and **Kayle** does not have to be alive for it to launch the sword.
  - The portal does not follow **Kayle**’s movements, using Flash or getting displaced won't change its position nor direction either.
- The cast time can be calculated by dividing **Kayle**’s *windup percent* value ($0.193555$) with her current total attack speed.
  - *Radiant Blast*’s cast time is at base attack speed, is at most at the minimum attack speed of (attack speed) 0.2, and is at the attack speed cap of (attack speed) 3.003. The cast time can be further decreased with effects that allows the player to bypass the attack speed cap.
  - Additionally, it should be noted that due to action only taking place on whole game ticks, the cast time can only be changed in fractions of $0.033$ seconds. This also means that the lowest possible cast time is $0.033$ seconds, requiring at least (attack speed) 5.8652 attack speed.
  - Important to note that in patch V9.17, *Radiant Blast*’s cast time was changed to be a flat $0.25$ seconds, rather than scaling with the attack windup time. In patch V10.4, this was undocumentedly changed back. Whether the change to scale with windup time again was intentional or not is unknown.
- The resistance reduction is applied after the damage has been dealt.
- The expansion consists of 5 hitboxes, forming a cross-like shape centered 100 units ahead from the impact in its direction. Their technical details can be seen below and visualized on the image to the right.
  - ExplosionCenter: 100 radius
  - ExplosionLeft: 150 distance, 125 width
  - ExplosionRight: 150 distance, 125 width
  - ExplosionBackward: 100 distance, 90 width
  - ExplosionForward: 400 distance, 90 width **Interactions & Other**
- The visual effects of the explosion shows the blade and guard of **Kayle**’s swords.
- The resistance reduction can be tracked on the HUD as a debuff called **Sundered**.
- Note that the PsyOps skin still uses the old shred VFX, which also only shows for 3 seconds (effect duration is not affected). This is possibly an oversight as all existing, and at the time released Dragon Slayer, skins were updated properly in V11.18.

---

### W: Celestial Blessing

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 900 units |
| **Cost** | 70 / 75 / 80 / 85 / 90 Mana |
| **Cooldown** | 15 seconds |
| **Targeting** | Unit / Auto |
| **Affects** | Self, Allies |
| **Out of Range Behavior** | If targeting an ally, walk in range of the target unit to cast |

**ACTIVE:** **Kayle** and the target allied champion are healed and gain **bonus** movement speed for 2 seconds.

| Attribute | Value |
|-----------|------:|
| **Heal** | 55 / 80 / 105 / 130 / 155 (+ 25% AP) |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 24 / 28 / 32 / 36 / 40% (+ 8% per 100 AP) |

If cast without a valid target, or self-cast, *Celestial Blessing* will automatically target the closest allied champion in range, prioritizing the one with the lowest health.

**Notes:**

- *Celestial Blessing*’s cast time does not interrupt movement and the effects are applied at the start. **Interactions & Other**
- If cast with auto-targeting, *Celestial Blessing* may target allies who are untargetable, or allied clones that cannot be targeted by allies (such as Wukong’s Warrior Trickster).

---

### E: Starfire Spellblade

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 350 units |
| **Cooldown** | 8 / 7.5 / 7 / 6.5 / 6 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.1$ seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | True |
| **Parry** | Special |
| **Call For Help** | True |

**PASSIVE:** **Kayle** deals **bonus** magic damage on-hit.

| Attribute | Value |
|-----------|------:|
| **Passive Damage** | 15 / 20 / 25 / 30 / 35 (+ 10% **bonus** AD) (+ 20% AP) |

**ACTIVE:** **Kayle** kindles her blade, empowering her next basic attack within 6 seconds to have an uncancellable windup and deal additional **bonus** magic damage on-hit that is capped at 400 against monsters. If **Kayle** is not yet *Arisen*, this attack becomes ranged with (range) 525 range.

| Attribute | Value |
|-----------|------:|
| **Bonus Magic Damage** | 8 / 8.5 / 9 / 9.5 / 10% (+ $1.5$% per 100 AP) of target's **missing** health |

** The attack explodes upon the target, dealing its damage to surrounding enemies and applying on-hit effects. This is affected by critical strike modifiers.

*Starfire Spellblade resets **Kayle**’s basic attack timer. Starfire Spellblade does not deal bonus damage against structures.*

**Notes:**

- *Starfire Spellblade* deals:
  - The physical damage part of both the attack and the explosion deals basic damage.
  - Proc damage for the passive's on-hit effect.
  - Spell damage for the active's on-hit effect.
  - All damage dealt to secondary targets by the explosion is additionally tagged as area damage.
- The active's effect shares the same cast instance with the triggering attack (exception: Muramana, see **Interactions & Other**).
- Parry interactions:
  - Blind: The main target will never receive damage, but the secondary targets will receive the active's spell damage.
  - Block: Blocking will always negate all damage. Keep in mind that if a secondary target is outside of Spirit's Refuge’s zone, they won't block damage.
  - Dodge: Dodging will always negate all damage.
- : All parry interactions are marked for retesting because of patch V11.1 changes. **Interactions & Other**
- While the active's effect is up, a circle is visible to **Kayle**, indicating her range, however, this circle only takes into account Divine Ascent’s effect on her range, not other sources such as Rapid Firecannon.
  - The increased range (with the item's effect included) will be (range) 675 prior to ascending to **TRANSCENDENT**, after which it's (range) 725.
- To clarify the description, *Starfire Spellblade* can behave both as a ranged and melee attack based on how far the target is prior to **Kayle** ascending to Arisen, after which the attack will be always using ranged standards.
  - If the target is 200 units away or closer, the attack will be considered melee. This also uses her melee animations, not utilizing projectiles even after ascending.
  - If the target is over 200 units, the attack will be considered ranged.
- To further expand on this: Using the ranged empowered attack will change **Kayle**’s range type to ranged while the projectile is alive. This means that if the target could outrun the projectile, **Kayle** would be stuck on being ranged, although the bonuses from the active are not kept. While it's unlikely to be on this temporary ranged state for longer than half a seconds during a normal playthrough, it is possible to achieve few seconds when specific conditions are met.
- Keep in mind that while on-hit effects will be applied to all targets in the explosion, on-attack effects are only applied once per basic attack, not per target hit.
- Single-use on-hits, such as Spellblade, will be applied to the main target.
  - If the main target died prior to the projectile reaching them, or if it was a jungle plant, the effects will be applied to the closest target from the explosion's center.
- Guinsoo's Rageblade Phantom Hit can only be applied to the main target.
- Despite being one cast instance, *Starfire Spellblade* triggers Muramana Shock twice on each target, dealing normal bonus damage after the basic attack and increased bonus damage after the spell, totalling at $5.2$% **maximum** mana **bonus** physical damage.

---

### R: Divine Judgment

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Target Range** | 900 units |
| **Effect Radius** | 675 / 675 / 775 units |
| **Cost** | 100 / 75 / 50 / 25 / 0 mana |
| **Cooldown** | 160 / 140 / 120 / 100 / 80 seconds |
| **Targeting** | Unit |
| **Affects** | Allies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Kayle** grants herself or a target allied champion invulnerability for $2.5$ seconds and conjures swords of fire.

After $2.5$ seconds, she rains the swords down around the target, dealing magic damage to nearby enemies.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 200 / 250 / 300 / 350 / 400 (+ 100% **bonus** AD) (+ 70% AP) |

**Notes:**

- *Divine Judgment*’s cast time does not interrupt movement and the cast will finish even after **Kayle**’s death.
- Since internalraw damage ignores invulnerability, the target may still die via the Nexus Obelisk, but the ability will finish at their corpse's position regardless.
- *Divine Judgment* will prioritize casting on allies over **Kayle**. This means that if Kayle's and an ally's selection ranges are intercepting each other on the cursor's position, the cast will always occur on the ally.
- Prior to becoming **TRANSCENDED** via Divine Ascent, casting *Divine Judgment* refreshes the duration of Zeal stacks at the start of the cast time to ensure that it does not expire while she can't attack during the cast.
- During cast, an indicator for the damaging area can be seen around the target, however, this is not visible if there is no sight on the them.
- *Divine Judgment* can be cast on clones too, including Shapesplitter and Warrior Trickster, which are normally not targetable for allies.
- While it's an uncommon occurence, it is possible to cast *Divine Judgment* on targets who just entered resurrection or zombie state. This is only possible with very strict timing and perhaps high latency.
- It's recommended to avoid both of the afformentioned interactions, as they render the usage of *Divine Judgment* useless in many cases. **Interactions & Other**
- While the target is invulnerable, a number indicator will appear beneath them whenever they would receive damage to show the amount that was entirely mitigated. This is calculated pre-mitigation, meaning before resistances and reductions are taken into account.
- Rapid Firecannon Energized effect behaves strangely with the set basic attack range:
  - If the ability is cast while the effect is ready, the range goes from (range) 400 to (range) 385 over 4 stat updates (A stat update occurs every 0.264 seconds.) (400 -> 390 -> 387 -> 385). When the cast completes, the proper range needs 2 updates to set in ((range) 660 - (range) 675 while **ARISEN**; (range) 710 - (range) 725 while **TRANSCENDED**).
  - Gaining the effect during the cast will cause the range to cycle between (range) 525 and (range) 550 until the cast finishes, then the proper range is set after 1 update.
  - Selling the item during the cast causes the range to be set to (range) 250. Range values are outdated because of patch V11.18 changes to **Kayle**.
- The invulnerability can be tracked on the HUD via a buff called **Intervention**.

---

## Patch History

### V25.15
- General
  - Default range type changed to melee from ranged.

### V25.14
- General
  - Default range type changed to ranged from melee.
    - Starfire Spellblade attacks can still behave as melee while attacking within 200 units before becoming Arisen. This is unchanged.

### V14.22
- Divine Ascent
  - Bonus attack speed AP ratio per stack increased to 1% per 100 AP from $0.5$% per 100 AP.

### V14.21
- Celestial Blessing
  - Mana cost reduced to 70 / 75 / 80 / 85 / 90 from 90 / 100 / 110 / 120 / 130.

### V14.10
- Kayle
  - **Bug Fixes:** Properly implemented her unique HUD portrait before ascending to Aflame (level 11).

### V14.9
- Kayle
  - **Bug Fixes:** Properly implemented her unique HUD portrait before ascending to Aflame (level 11).

### V14.5
- Starfire Spellblade
  - **New Effect:** Now triggers spell effects upon dealing damage.

### V14.1#January 12th Hotfix|V14.1
- Divine Ascent
  - Bonus attack speed AP ratio per stack reduced to $0.5$% per 100 AP from 1%.

### V13.17
- Kayle
  - **Bug Fixes:** Smoke VFX during her Recall now uses the correct shape.
- Kayle
  - **Bug Fixes:** Her champion icon on the scoreboard and minimap now use the correct icon to reflect her third form at level 11.

### V13.10
- Divine Judgment
  - **Bug Fixes:** Sword VFX are no longer missing on her body during the cast animation.

## Trivia

- Kayle was the first to have her dance changed after release, (followed by Veigar) the first to have her abilities reworked three times since release, (followed by Ryze) and the first to use Chinese artwork outside Asian servers.
- Kayle - Morgana is one of seven pairs of sibling champions (the others being Cassiopeia - Katarina, Garen - Lux, Nasus - Renekton, Yasuo - Yone, Darius - Draven, and Vi - Jinx).
- During Alpha Test, she was simply called 'Judicator' and at one point she was incorrectly listed as a tank.
- Her old dance referenced Will Smith "Jump on It" dance from The Fresh Prince of Bel-Air.
  - A side-by-side comparison can be seen here.
  - She shared this dance with Singed.
- Her new dance is a reference to the dance of Elaine Benes from Seinfeld "The Little Kicks".
- In the now-removed official League of Legends forums, the original icon of Intervention was used to represent the "Mac Client" section.
- **Kayle**, Elise, Gnar, Jayce, and Nidalee are the only champions to be conditionally considered both ranged and melee.
- The projectile speed of Kayle's basic attack is the third highest in the game among all ranged attackers that utilize projectiles, at 5000.
  - The highest and second highest are Aphelios’ Severum and Yunara’s Cultivation of Spirit at 92400 and 10000, respectively.
- The first buff from her innate ( **Zeal**) shares its name with the item Zeal.
- Her dance references Elaine Benes dance from Seinfeld.
  - A side-by-side comparison can be seen here.
- In Ultimate Spellbook, **Kayle** ascends without the player's input. This is caused by the fact that the choosen Ult-ernate Summoner Spells are automatically ranked up when the required levels are reached, which also matches the levels that the player may initiate the ascensions on.

---
*This page was automatically generated from League of Legends Wiki data.*