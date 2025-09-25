# Sion

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
| **Champion** | Sion |
| **Title** | The Undead Juggernaut |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Latest Changes** | V25.16 |
| **Roles** | Vanguard |
| **Riot Positions** | Top |
| **External Positions** | Top |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 1 |
| **Hero Type** | Tank |
| **Alt Type** | Fighter |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 65 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $655.0$ | $+87.0$ |
| **Mana** | $400.0$ | $+52.0$ |
| **Health Regen** | $9.0$ | $+0.8$ |
| **Mana Regen** | $8.0$ | $+0.6$ |
| **Armor** | $36.0$ | $+4.2$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $68.0$ | $+4.0$ |
| **Attack Speed** | $0.679$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.679$ | |
| **Attack Speed Ratio** | $0.679$ | |
| **Bonus AS per Level** | $1.3\%$ | |
| **Acquisition Radius** | $600$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $25.767$ units | |
| **Selection Radius** | $125$ units | |
| **Selection Height** | $155$ units | |
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
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $92.0\%$ |

## Abilities

### Passive: Death Surge

| Attribute | Value |
|-----------|------:|
| **Cooldown** | 100 (Resets upon death) seconds |
| **Targeting** | Auto |
| **Affects** | Self |

**ACTIVE:** **Sion** gains (ms) 67% (See notes, this value is currently bugged) **bonus** movement speed that decays over $2.376$ seconds.


*Once at least one ability has been learned, Death Surge can be cast using any ability key, even for the abilities that haven't been learned yet.*

**Notes:**

- *Death Surge* will grant the bonus movement speed after a brief delay.
  - The buff is intended to grant 75% decaying movement speed, but instead grants 67% movement speed decaying with -8% per stat update.
    - Precisely this leaves him at 0% / 67% / 59% / 51% / 43% / 35% / 27% / 19% / 11% / 3% / 0%8% per stat update bonus movement speed.
- *Death Surge* has a shared cooldown across each ability key, regardless of which key is used to activate it. Thus, activating *Death Surge* with any ability key will put the others on cooldown.
- *Death Surge*’s cooldown resets upon death.
- *Death Surge*’s cooldown is affected by ability haste and thus the ability is technically able to be used up to twice during a single instance of *Glory in Death*, but this isn't practical. ** *Death Surge* is not affected by Ultimate Hunter, even if cast from the ultimate ability slot.
  - Despite showing the cooldown being reduced, *Death Surge* in the ultimate ability slot will be not reduced by Axiom Arc Flux on takedown.
  - *Death Surge*’s cooldown is not reduced by Navori Flickerblade or Transcendence.

---

### Passive: Glory in Death

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | physical |
| **Spell Effects** | proc |
| **Parry** | True |

**INNATE:** Upon taking fatal damage, **Sion** enters stasis for $1.5$ seconds to reanimate into a frenzy, restoring 100% of his **maximum** health but losing 2 to 19 health every , increasing by「 70% of the **base** value ⟷ 2×0.7 to 19×0.7 」per subsequent tick. **Sion**’s health regeneration and any healing (except life steal) he receives during this time is 0% effective (See notes).

**Sion** can only move and use his basic attacks and item actives, and empowers himself with the following effects:
- (attack speed) $1.75$ attack speed minimum and maximum
- (life steal) 100% life steal
- Deals **bonus** physical damage on-hit equal to 10% of the target's **maximum** health, capped at 75 against non-champions
- Ghosting
- One cast of *Death Surge*, which replaces all of his abilities

**Sion**’s damage to structures is reduced to 40% under the effects of *Glory in Death*.

**Notes:**

**Details**
- **Sion**’s death timer starts when he dies and not when *Glory in Death* ends.
  - If **Sion** lives longer than what his death timer lasts for, he will not revive until *Glory in Death* has ended. When it ends, he instantly revives. This may result in some odd interactions and bugs at times.
- The effect that heals **Sion** back to full health at the start of *Glory in Death* cannot be modified by healing reductions and does not count towards the 'Damage Healed' statistic at the end of game lobby.
- The health loss is unaffected by Invulnerability.
- *Glory in Death* has a capped 60 seconds duration. If the player manages to survive for so long, **Sion** dies instantly, regardless of his current health.
- The healing modifier during *Glory in Death* reduces all incoming healing, it does not matter if they are sourced from **Sion** himself or an ally. This does not affect life steal.
  - This healing modifier stacks additively with Spirit Visage and does not stack with heal and shield power.
  - This means that purchasing *Spirit Visage* enables **Sion** to have regeneration and use healing effects at 25% effectiveness, and the life steal gained will be increased to 125% as expected.
- The **fixed** attack speed is achieved by lowering and increasing the maximum and minimum attack speed to (attack speed) 1.75, respectively.
  - This also means that it's unaffected by other attack speed modifiers for the duration, such as bonus attack speed or cripples.
  - However, Hail of Blades, which allows for the normal attack speed cap ((attack speed) {3.0) to be surpassed, will also break this limit. This means that while it is active, **Sion** may exceed his (attack speed) 1.75 cap, but cannot fall below it.
    - When Hail of Blades ends, the attack speed cap will revert to (attack speed) $3.0$ and not to (attack speed) 1.75, however **Sion** still can't fall below (attack speed) 1.75.
- Some item passives do not persist through death and will be lost for the initial few seconds of *Glory in Death*.
- Phase Rush cooldown is reset upon entering *Glory in Death*.
- The bonus on-hit damage granted by *Glory in Death* is unaffected by life steal, but is applied against structures that are not turrets.
  - However, it can be applied against structures that are not turrets.
- The reduction of damage against structures includes **Sion**’s attacks as well as on-hit effects that can be applied to structures, such as Demolish, Titanic Hydra and the magic damage based on AP conversion based on adaptive force.
  - It even includes true damage from sources such as Elixir of Sorcery.
- **Sion**, during *Glory in Death*, will not lose health while being carried by Fate's Call.
- *Glory in Death* does not trigger on clones of **Sion**, such as with Test of Spirit.
- *Glory in Death*’s untargetability from the stasis does not destroy in-flight projectiles.
  - Projectiles created while **Sion** is in the zombie form will still travel to him even after he dies.
- Upon death, the current cooldowns of **Sion**’s summoner spells are increased to 4 seconds. Afterwards, he is still unable to use them until he respawns.
  - However, it is possible for another lock-out effect to unlock them when their duration ends (excluding crowd control with the exception of stasis).
- After *Glory in Death* ends, the corpse of **Sion** will retain unit collision despite being dead on the ground.
- Buffs from jungle monsters that **Sion** acquired during *Glory in Death* will not be lost when the duration ends. (Crest of Cinders, Crest of Insight) **Interactions & Other**
- By surviving for the full duration of 60 seconds, **Sion** will have suffered floor(60/0.264)*(2.3 to 24.4)*(1+(floor (60/0.264)-1)/2×0.7)/1000 in health costs.
- **Sion** can also heal from spell vamp, althought the stat is currently unobtainable and it would be also unusable during *Glory in Death*, since there would be no available abilities that would apply it.
- Even though its visuals are present, Dark Harvest can never be gained from **Sion** under the effect of *Glory in Death*.
- At the start of *Glory in Death*, **Sion**’s mana drops to 0, but his (mana regeneration) mana regeneration will remain in effect. ;Zombie info

---

### Q: Decimating Smash

| Attribute | Value |
|-----------|------:|
| **Range** | 500 / 500 / 675 / 762.5 / 850 units |
| **Cast Time** | none |
| **Cost** | 45 Mana |
| **Cooldown** | 10 / 9 / 8 / 7 / 6 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |
| **Silence** | True |

**ACTIVE:** **Sion** prepares a strike in the target direction, charging for up to 2 seconds to increase *Decimating Smash*’s range over an area, knock up duration, stun duration, and damage every $0.25$ seconds, which has up to a maximum **bonus** for the **base** damage and up to a 200% **bonus** for the *scaling* damage.

| Attribute | Value |
|-----------|------:|
| **Maximum Base Damage Increase** | 125 / 158.33 / 175 / 185 / 191.67% |

| Attribute | Value |
|-----------|------:|
| **Minimum Physical Damage** | 40 / 60 / 80 / 100 / 120 (+ 40 / 50 / 60 / 70 / 80% AD) |
| **Maximum Physical Damage** | 90 / 155 / 220 / 285 / 350 (+ 120 / 150 / 180 / 210 / 240% AD) |

*Decimating Smash* can be recast within the duration, and does so automatically afterwards. If the charge is interrupted, *Decimating Smash* is placed on a (cd) 2-second cooldown.

**RECAST:** **Sion** flails his axe in the direction he targeted at the time of cast, dealing physical damage to enemies hit and slowing them by 50% for $0.25$ seconds. *Decimating Smash* deals 150% damage against monsters and 60% damage against minions.

| Attribute | Value |
|-----------|------:|
| **Minimum Monster Damage** | 60 / 90 / 120 / 150 / 180 (+ 60 / 75 / 90 / 105 / 120% AD) |
| **Maximum Monster Damage** | 135 / 232.5 / 330 / 427.5 / 525 (+ 180 / 225 / 270 / 315 / 360% AD) |
| **Minimum Minion Damage** | 24 / 36 / 48 / 60 / 72 (+ 24 / 30 / 36 / 42 / 48% AD) |
| **Maximum Minion Damage** | 54 / 93 / 132 / 171 / 210 (+ 72 / 90 / 108 / 126 / 144% AD) |

If *Decimating Smash* was charged for at least 1 second, **Sion** instead slams his axe down, dealing physical damage to enemies hit, knocking them up for 0.5–1@1–2 (@=channel time) seconds, and stunning them for 1.25–2.25@1–2 (@=channel time) seconds.

**Notes:**


- **Sion** himself remains locked out of all actions for $0.25$ seconds when releasing *Decimating Smash*.
  - Soul Furnace can be cast during this time, but Roar of the Slayer and Unstoppable Onslaught cannot and neither will they be buffered.
- Displacement immunity will also resist the application of the stun.
- Since the stun duration is longer than the airborne duration, and both start at the same time, Tenacity is still fully effective against *Decimating Smash* up to a cap.
  - This virtual cap is 60%-55.5% Tenacity based on channel time beyond 1 second. Every value below this cap will lower the CC duration with no diminished returns.
- A flash of the axe and change in brightness of the indicator on the ground indicates when the 1 second time frame is reached.
- The damage of *Decimating Smash* is calculated when it is released. Gaining AD or upgrading the ability during its channel will update the damage of the coming hit.
  - Even though the cooldown starts when the ability is released, it will not be changed by gaining or losing AH, or upgrading the ability, during the cast time.
- The indicator is visible to the enemy only if they see **Sion** himself.
  - If the channel is released before 1 second, enemies may see a particle at the far end of the hitbox, if it itself isn't outside of their vision.
  - If the channel is released after 1 second, the only remaining indication is the visual effects and audio that plays on any units the spell strikes (these play on-target), provided the enemy does have vision on those.
- Forced movement without a stun component (or the stun being cleansed previously) will not stop **Sion** from starting or continuing to channel. Its direction will not change with **Sion**’s travel, however the location will update with him.
  - The location of the indicator updates at the *beginning* of the ability, at *1 second* and at *release*.
- The indicator of the ability is slightly shorter at its front than the hitbox of the ability is for Sion, Sion and related skins and chromas, while on Sion, the indicator is slightly thinner at the front than the hitbox is, instead.
- The following table refers for interactions while **Sion** is channeling:

#### Channel Behavior (charge)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Disabled |
| **Abilities** | Soul Furnace and its recast are usable. Roar of the Slayer and Unstoppable Onslaught are disabled. This ability recasts to end channel. |
| **Items** | Disabled |
| **Summoner Spells** | Allowed / Disabled / Recasts |
| **Consumables** | Disabled |

---

### W: Soul Furnace

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 525 units |
| **Cost** | 65 / 70 / 75 / 80 / 85 Mana |
| **Cooldown** | 15 / 14 / 13 / 12 / 11 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**PASSIVE:** **Sion** gains (health) 4 **bonus** health whenever he kills an enemy (see notes for details), increased to 15 for large enemies and takedowns against enemy champions.

**ACTIVE:** **Sion** grants himself a shield that lasts for up to 6 seconds. *Soul Furnace* can be recast after 3 seconds while the shield holds, and does so automatically at the end of the duration.

| Attribute | Value |
|-----------|------:|
| **Shield Strength** | 60 / 75 / 90 / 105 / 120 (+ 40% AP) (+ 8 / 10 / 12 / 14 / 16% **maximum** health) |

**RECAST:** **Sion** consumes the shield to deal magic damage to nearby enemies, capped at 400 against minions and monsters.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 40 / 65 / 90 / 115 / 140 (+ 40% AP) (+ 14% of target's **maximum** health) |

*Both of Soul Furnace's casts can be activated during *’Sion's** other abilities.*

**Notes:**

- A buff icon in the HUD will show the current shield strength, so that the player can better calculate the timing of the second cast.
  - The initial shown value does not interact with (Heal and Shield Power) shield strength modifiers. Whenever the shield takes damage, it updates to the proper current shield amount, however. *The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive. **Detonating the ability manually does not.
- Gaining or losing AP or health, or ranking up the skill while the shield is active will not change its shield value but it will change the damage of the shield's detonation.
- The cooldown of *Soul Furnace* begins once the shield is removed.
  - Gaining or losing ability haste between the first cast and breaking of the shield will affect the resuming cooldown accordingly.
- *Soul Furnace* also automatically detonates if **Sion** is killed without the shield being destroyed, e.g. by dying to the Nexus Obelisk, *even if* Sion could not have detonated it manually yet.
- *Soul Furnace*’s bonus health can be obtained by any of **Sion**’s damage sources (basic attacks, abilities, summoner spells, item actives).
  - This does not include *last hits* transferred to **Sion** artificially.
  - If the gold efficiency of a Ruby Crystal is considered 100%, each *last hit* or *takedown* after *Soul Furnace* has been learned can be considered worth an extra $10. or 40g gold, respectively.
- **Sion** must learn *Soul Furnace* before he can stack up its passive.
- **Sion** does not gain bonus health from units that enter resurrection on-death.
- Killing the following unit types grants **Sion** 15 **bonus** health:
  - Champions (kills and assists)
  - Large minions and large monsters (including both summoned and pit Rift Herald)
  - Large pets - Explicitly Tibbers, Daisy and The Maiden
- Killing the following unit types grants **Sion** 4 **bonus** health:
  - Minions and small monsters, with the exceptions of those listed in the below category
  - Wards and other champion summoned units (e.g. Noxious Trap, Powder Keg, Dark Procession)
  - All pets, with the exceptions of those listed in the above and below categories
  - Clones
- Killing the following unit types grants **Sion** *no* **bonus** health:
  - Turrets (including Sun Disk) and other structures
  - Jungle plants (destroying which does not grant kill credit).
  - Voidmite (including both summoned and pit Voidmite)
  - Units destroyed by 'trampling' such as Zyra’s seeds, Zac’s goo or Rek'Sai’s tunnels.

---

### E: Roar of the Slayer

| Attribute | Value |
|-----------|------:|
| **Range** | 800 (Roar missile range) / 1350 (Maximum minion projectile range) units |
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 130 (Dashing minion collision radius) units |
| **Width** | 160 (Roar missile) units |
| **Speed** | 1800 units/second |
| **Cost** | 35 / 40 / 45 / 50 / 55 Mana |
| **Cooldown** | 12 / 11 / 10 / 9 / 8 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | Special |

**ACTIVE:** **Sion** bellows a shockwave in the target direction that deals magic damage to the first enemy hit, slows them for $2.5$ seconds and inflicts them with (armor penetration) 25% armor reduction for 4 seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 65 / 100 / 135 / 170 / 205 (+ 55% AP) || Attribute | Value |
|-----------|------:|
| **Slow** | 40 / 45 / 50 / 55 / 60% |

If the target is a minion or non-epic monster, they are also stunned for $0.75$ seconds and knocked back for up to 1350 units further, though not through terrain, applying *Roar of the Slayer*’s effects to enemies they pass through as well as briefly granting sight of the area around their trajectory.

**Scaling:**
| Attribute | Value |
|-----------|------:|
| **Slow** | 40 / 45 / 50 / 55 / 60% |

**Notes:**

- Minions or monsters that are flung away are immune to minion damage and take their damage once they are stopped if they would die to *Roar of the Slayer*’s damage, by either colliding with terrain, reaching the end of their trajectory, or having their displacement interrupted.
- Unlike the intial single-target projectile, a thrown minion or monster is not blocked by Wind Wall or Unbreakable.
- Gaining or losing AP, or upgrading the ability while its projectile is traveling will change its damage for every future hit. This applies to both the intial single-target projectile, as well as the minion-projectile. - This ability will cast from wherever the caster is at the end of the cast time.

---

### R: Unstoppable Onslaught

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 100 Mana |
| **Cooldown** | 140 / 120 / 100 / 80 / 60 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Direction / Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Grounded** | True |
| **Silence** | False |
| **Steer Speed** | $8.3$°/s |

**ACTIVE:** **Sion** becomes immune to crowd control and ghosted and, after a brief delay (0.05 (Estimated)), charges forward in the direction of the cursor for up to 8 seconds, during which he is able to steer at a slow turn rate while automatically navigating his movement. *Unstoppable Onslaught* can be recast after $0.4$ (Estimated) seconds during the channel.

The charge ramps up **Sion**’s at the time of cast by 40 movement speed every $0.1$ seconds, up to 950 **total** movement speed.

At the end of the charge's duration, **Sion** leaps forward 300 (Can end up being slightly shorter, but not related to to Sion's speed) units, landing after $0.55$ seconds and slamming the ground (400 radius) beneath him. **Sion** will stop upon colliding (150 units in front of Sion for the charge and point blank for the leap) with an enemy champion or terrain (Includes structures, read notes for details on player-generated terrain) during the charge, ending *Unstoppable Onslaught* with a slam. Crashing into terrain stuns **Sion** for $0.5$ seconds.

Enemies hit by **Sion**’s charge are dealt physical damage. The *base* damage increases by 0 to 500/3 for 4 and the *scaling* increases by 0%–100%@0–3 (@=channel time).

| Attribute | Value |
|-----------|------:|
| **Minimum Physical Damage** | 150 / 225 / 300 / 375 / 450 (+ 60% **bonus** AD) |
| **Maximum Physical Damage** | 400 / 600 / 800 / 1000 / 1200 (+ 120% **bonus** AD) |

Enemies, including turrets, hit by the slam are dealt the same damage and are slowed for 3 seconds. Enemies in a smaller radius (350 range, estimated) are also pulled towards **Sion** over $0.5$ seconds and become stunned after a brief delay for 0.25 to 1.75 seconds.

| Attribute | Value |
|-----------|------:|
| **Slow** | 40 / 42.5 / 45 / 47.5 / 50% |

**RECAST:** **Sion** stops charging, causing him to leap forward and slam the ground beneath him as if *Unstoppable Onslaught* had reached the end of its duration.

**Notes:**

- When **Sion** casts *Unstoppable Onslaught* a global warning can be heard.
- *Unstoppable Onslaught* does not apply the knock up nor stun to enemies from slamming on collision with terrain.
- *Unstoppable Onslaught* does not knock up non-champions hit by the charge.
- *Unstoppable Onslaught* can be activated twice **or** held down and released on **both** normal cast and quick cast. There is no other difference between normal cast and quick cast for this ability, either. *The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade. **The recast does not.
- Zeke's Convergence Frostfire Tempest and Experimental Hexplate Overdrive trigger at the end of the charge or leap.
  - Casting Death Surge will prevent the next *Unstoppable Onslaught* from triggering *Frostfire Tempest* and *Overdrive*.
- Casting *Death Surge* will postpone Axiom Arcanist cooldown refund until **Sion** casts *Unstoppable Onslaught* twice without casting *Death Surge* between the two casts.
- An enemy can only be affected by *Unstoppable Onslaught* once every 1 second.
- After reaching maximum damage and stun duration (at 3 seconds) **Sion** will transition from using his *haste* animation to his unique *charging* one.
  - Reaching the threshold on Sion changes his particle effects instead, most notably the 'Boosters' behind the vehicle.
  - The turning angle is not based on time and can easily be seen in practice when a **Sion** *charges* in-place due to bugs.
- **Sion**’s movement speed gains during the charge are unaffected by movement speed caps. Effects that would increase or decrease his movement speed do not affect him as well.
  - Nami’s I is the only known effect that can affect **Sion**’s movement speed during the *charge* and that can place him above the normal MS value. However, without further multipliers, surpassing the 950 movement speed cap will require Nami to have over 2150 ability power.
- **Sion** does not always crash with terrain immediately upon touching it, enabling the player to turn beyond his maximum turning angle.
  - Casting the ability at point blank of a terrain will most often automatically change the *charge*’s direction.
  - This is not usually possible with player-generated terrain.
- *Unstoppable Onslaught* cannot *charge* through but can *leap* over player-generated terrain.
- **Sion** loses the crowd control immunity after finishing the leap.
- **Sion** is not immune to an *allied* Bard’s Tempered Fate if it hits **Sion** during the $0.05$-second delay before he starts the channel, he will not resist the effect and will initiate the *slam* immediately while in stasis.
- *Unstoppable Onslaught* cannot strike the same non-champion twice in less than 1 second (Like most AOE spells or spells with traveling hitboxes, it places a marker buff on struck targets. This one lasts 1 second).
- Even though the cooldown of *Unstoppable Onslaught* starts at the end of the effect, gaining or losing ability haste, or upgrading the ability during the effect, will not change the resuming cooldown.
- Removing the airborne before $0.5$ seconds will **not** prevent the stun from being applied.
- Displacement immunity will also resist the application of the stun.
- **Sion** can occasionally cast *Unstoppable Onslaught* while rooted. In this case, he will stand in place until the root's debuff duration ends. During this time, the damage, stun duration and movement speed granted by the ability will still ramp up, and enemies that come into contact with him will be affected by the collision normally.
  - A similar case allows him to cast it during crowd control that would disable it. However, the channel would be interrupted almost immediately, and he will initiate the leap as well.
- Fate's Call deferment of *Unstoppable Onslaught* will still allow **Sion** to leap forward at the end of the channel.
- Moving the cursor onto HUD elements (e.g mini map.md) / champion portraits) does *not* jeopardize the cast and steering of *Unstoppable Onslaught*. The targeting will act as if the ground was targeted below the display element.
- The distance between **Sion**’s center and the center of the ground *slam* scales slightly with movement speed. This is suspected because running into a wall within half a second of *charge* time after starting at multiple thousands of movement speed shows a considerable difference, however this could also be due to a bug.
- The *leap* appears to always have the same speed regardless of **Sion**’s, at roughly 750.
- The following table refers for interactions while **Sion** is channeling:

#### Channel Behavior (channel)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Allowed |
| **Abilities** | Soul Furnace and its recast are usable. Decimating Smash and Roar of the Slayer are disabled. This ability recasts to end channel. |
| **Items** | Disabled |
| **Summoner Spells** | Disabled |
| **Consumables** | Disabled |
| **Interrupted by** | death |
| **Notes** | but cannot input movement commands. |
- If *Unstoppable Onslaught*’s channel is interrupted despite the immunity to crowd control or Decimating Smash is cast within $0.25$ seconds after starting the charge, it will also initiate the leap. If the channel ends due to collision or death, there is no leap and the effect occurs immediately.

---

## Patch History

### V25.16
- Glory in Death
  - Base health cost per tick reduced to 2 to 19 from 2.3 to 24.4.
- Roar of the Slayer
  - Armor reduction increased to 25% from 20%.
- Unstoppable Onslaught
  - Minimum bonus AD ratio increased to 60% **bonus** AD from 40%.
  - Maximum bonus AD ratio increased to 120% **bonus** AD from 80%.
  - **Bug Fixes:** Sett’s The Show Stopper can no longer illegally interrupt the charge.

### V25.15
- Unstoppable Onslaught
  - **Bug Fixes:** Now properly affects enemies casting Sett’s The Show Stopper when colliding with them. Previously, **Sion** would collide with the target and end his charge (appropriately) but apply no effects to them, which is incorrect.

### V25.11
- Decimating Smash
  - **Bug Fixes:** Issuing an attack-move command during the charge no longer incorrectly causes the ability to recast.

### V25.08
- Sion
  - Adjustments made to his model and to make his armor look, move, and feel more like armor.
  - Adjustments made to his fire and VFX elements throughout his kit.

### V25.04
- Stats
  - Base health regeneration increased to 9 from $7.5$.
  - Base armor increased to 36 from 32.

### V25.S1.1#January 9th Hotfix|V25.S1.1
- Glory in Death
  - **Bug Fixes:** Now once again properly gains Deathguard upon respawning.

### V14.19
- Decimating Smash
  - **Bug Fixes:** No longer forcibly charges the ability for its full duration when respawning and without player input if he cast Death Surge on the Q slot before dying.

### V14.14
- Glory in Death
  - **Bug Fixes:** Is no longer sometimes unable to issue movement commands after the zombie state has expired.
- Soul Furnace
  - Damage health ratio increased to 14% of target's **maximum** health at all ranks from 10 / 11 / 12 / 13 / 14%.

### V14.13
- Glory in Death
  - **Bug Fixes:** Corpse is no longer sometimes a valid movement collision target.

### V14.10
- Unstoppable Onslaught
  - **Bug Fixes:** Is no longer interrupted by Kassadin’s Null Sphere.

## Trivia

- When permanent health stacking was added to Enrage.png in the May 9, 2009 Patch, Sion became the first champion to have an ability that could infinitely stack an effect.
- Sion was one of the first champions designed, together with Annie, Lee Sin, Singed, Sivir, and Twisted Fate.
  - Of these, he was however the first and only one to recieve a full relaunch after his initial release, completely remaking the original appearance and gameplay.
- 
  - In Sion's case, Soul Furnace infinitely stacks his health.
- Sion's updated 'Champion Spotlight' was the last to feature the old Summoner's Rift map.
- Unstoppable Onslaught was inspired by Juggernaut (comics) from X-Men.
- Sion, Blitzcrank, Caitlyn, Lissandra, Rumble, Varus, Vi, Viego, Xerath, and Ziggs are the only champions who can apply crowd control on themselves.
- Glory in Death is one of three abilities which allow their casters to continue fighting after death, the others being Death Defied and Icathian Surprise.
  - Sion's passive is the only of these three that lasts a variable time, determined by his health running out.
  - Sion is also the only targetable 'Zombie-state' of the three, which was previously shared with Omen of Death, Yorick’s original ultimate.

---
*This page was automatically generated from League of Legends Wiki data.*