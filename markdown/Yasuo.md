# Yasuo

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
| **Champion** | Yasuo |
| **Title** | the Unforgiven |
| **Resource** | Flow |
| **Range Type** | Melee |
| **Release Date** | 2013-12-13 |
| **Release Patch** | V3.15 |
| **Latest Changes** | V25.14 |
| **Roles** | Skirmisher |
| **Riot Positions** | Top, Middle, Bottom |
| **External Positions** | Top, Middle, Bottom |
| **Blue Essence** | 675 |
| **Riot Points** | 585 |
| **Difficulty** | 3 |
| **Hero Type** | Fighter |
| **Alt Type** | Assassin |
| **Adaptive Type** | Physical |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 3 |
| **Utility** | 2 |
| **Style** | 20 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $590.0$ | $+110.0$ |
| **Mana** | $100.0$ | $+0.0$ |
| **Health Regen** | $6.5$ | $+0.9$ |
| **Armor** | $32.0$ | $+4.6$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $60.0$ | $+2.5$ |
| **Attack Speed** | $0.697$ | |
| **Movement Speed** | $345.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.697$ | |
| **Attack Speed Ratio** | $0.67$ | |
| **Bonus AS per Level** | $3.5\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Pathing Radius** | $32$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $180$ units | |
| **Critical Damage** | $160.0\%$ | |

### Map-specific Stats

#### URF

| Metric | Value |
|--------|------:|
| **Damage Taken** | $90.0\%$ |

## Abilities

### Passive: Way of the Wanderer

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE - INTENT:** **Yasuo**’s **total** critical strike chance is doubled from all other sources. Additionally,「 every 1% critical strike chance in excess of 100% is converted into $0.5$ **bonus** attack damage. ⟷ every 50% critical strike chance in excess of 100% is converted into 25 **bonus** attack damage. 」

**INNATE - RESOLVE:** **Yasuo** generates a stack of Flow for every 59–46@1–13 units he travels by any means. At「 100 stacks ⟷ 59×100 to 46×100 for 3 units traveled 」and upon taking damage from an enemy champion or monster, **Yasuo** consumes all Flow to grant himself a shield for 125 / 600 that lasts for 1 second.

**Notes:**

- **Yasuo** gains the shield before the damage instance; the shield will mitigate the damage that triggered it.
- **Yasuo** is able to reach 100% critical strike chance with *Way of the Wanderer*’s multiplier after obtaining a total of 50% critical strike chance.
- **Yasuo**’s secondary resource bar will display the amount of shield stored as an equivalent percentage to his Flow stacks. (Each Flow stack will add 1.25 / 6 to the bar)
  - Upon taking damage, the bar will display his current shield strength.

---

### Q: Steel Tempest

| Attribute | Value |
|-----------|------:|
| **Range** | 450 (Thrust range) / cr 1150 (Whirlwind missile range) units |
| **Cast Time** | / |
| **Effect Radius** | 215 (When used with Sweeping Blade) units |
| **Width** | 80 (Thrust) / er 180 (Whirlwind missile) units |
| **Speed** | 1200 (Whirlwind missile speed) units/second |
| **Static Cooldown** | 4 to 1.48 for 8–4/3@0–105 (@=**bonus** attack speed) |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Projectile** | Special |
| **Parry** | Special |
| **Call For Help** | Special |

**ACTIVE:** **Yasuo** thrusts his sword in a line in the target direction that deals physical damage to enemies hit, applies on-hit effects to the first enemy hit, and triggers on-attack effects once. *Steel Tempest*’s damage based on its AD ratio can critically strike for damage.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 20 / 45 / 70 / 95 / 120 (+ 105% AD) |

**STEEL WIND RISING:** If this hits at least one enemy, **Yasuo** generates a stack of *Gathering Storm* for 6 seconds, stacking up to 2 times and refreshing on subsequent hits. At 2 stacks, the next *Steel Tempest* cast consumes them all to become empowered with a new effect.

**GATHERING STORM BONUS:** **Yasuo** unleashes a whirlwind in the target direction that applies and triggers the same effects and additionally knocks up enemies hit for $0.9$ seconds.

*Steel Tempest*’s thrust will fail to hit targets after the cast time if **Yasuo** was affected by disarming crowd control during it, but the cooldown of the ability is reset to $0.1$ seconds.

If *Steel Tempest* is cast during *Sweeping Blade*, it will instead affect enemies around **Yasuo** at the end of the dash, or doing so immediately at his landing location after blinking while ending the dash prematurely. *Steel Tempest*’s cast time is not incurred in this case.

**Notes:**

- Casting Sweeping Blade when *Steel Tempest*’s remaining cooldown is less than $0.5$ seconds resets this remaining cooldown to allow casting it in combination, but adds the refunded time to *Steel Tempest*’s cooldown.
- If **Yasuo** becomes unable to cast abilities during the dash after buffering, *Steel Tempest* will not cast, but will still go on cooldown.
- The whirlwind from consuming stacks of *Gathering Storm* can strike targets whose center is behind **Yasuo**, unlike most missiles.
- The whirlwind is not released when cast during Sweeping Blade, but instead knocks up all enemies hit around **Yasuo** at the end of the dash.
- *Steel Tempest* applies basic damage to the first (closest) enemy hit and area damage to secondary enemies:
  - *Steel Tempest* will apply on-hit effects to the first target hit, but will not do so to the secondary ones.
  - *Steel Tempest* will not apply spell effects to the first target hit, but will do so to the secondary ones.
  - Spell vamp will only grant healing from the damage dealt to secondary targets, and healing is reduced to 33% effectiveness, accordingly.
  - Life steal will heal based on the damage dealt to the first target hit.
- The whirlwind is a missile and can therefore be intercepted.
- *Steel Tempest* will only draw minion aggro if the first target is a champion. Effect at cast time end
- Each parry has different interactions with this ability, whether it's the first target of *Steel Tempest* or the secondary one. In either case **Yasuo** still gains a stack of *Gathering Storm* and his whirlwind knock-up cannot be negated by parries:
  - Dodge: both first target and secondary target **do not** take damage.
  - Block: first target **does not** take damage, secondary target **does** take damage.
  - Blind: both first target and secondary target **do** take damage.
- *Steel Tempest* rolls its critical strikes individually against each enemy hit.
- Casting Last Breath during *Steel Tempest* will buffer it to cast after the cast time has completed.
  - However, if Last Breath is used on a target affected by an airborne source that is not **Yasuo**, it is cast immediately due to *Steel Tempest*’s cast time ending prematurely. *Steel Tempest* will still cast in this case. <!--14.11 fixed
- If *Steel Tempest* is buffered during *Sweeping Blade* and **Yasuo** uses Recall, Teleport or Hextech Rocketbelt, *Steel Tempest* will go on cooldown without casting. In the case of Hextech Rocketbelt **Yasuo** will also need to cast a movement command after it, else *Steel Tempest* will still trigger.
  - If, after this effect, **Yasuo** inputs a basic attack command quickly followed by a movement command, he will cast *Steel Tempest* around him and basic attack the target at the same time.
    - *Steel Tempest* will not go on cooldown.-->
- Spell shield does not prevent a stack of *Gathering Storm* from being gained.
- While at two stacks, a range indicator will be shown for the effective range of the whirlwind.

---

### W: Wind Wall

| Attribute | Value |
|-----------|------:|
| **Range** | 0 - 350 (Windwall distance (final 50 units traveled only slowly)) / 450 (Center of sight area) |
| **Cast Time** | $0.013$ seconds |
| **Effect Radius** | sight300 (Sight area radius) units |
| **Cooldown** | 25 / 23 / 21 / 19 / 17 seconds |
| **Targeting** | Direction |
| **Affects** | None |
| **Projectile** | Special |

**ACTIVE:** **Yasuo** raises a wall of wind that travels in the target direction over $0.6$ seconds (Estimated). It then drifts another 50 units over $3.4$ seconds, blocking all hostile non-turret projectiles that hit it and granting sight of its surroundings.

| Attribute | Value |
|-----------|------:|
| **Wall Width** | 320 / 390 / 460 / 530 / 600 |

**Notes:**

- *Wind Wall* grants sight in a small static area in front of the cast location.
- *Wind Wall* starts blocking projectiles on-cast (despite not being fully formed).
- *Wind Wall* travels via a missile, and thus stops upon colliding with an enemy *Wind Wall*.

---

### E: Sweeping Blade

| Attribute | Value |
|-----------|------:|
| **Range** | 475 (Dash distance) / 625 (Maximum increased dash distance across terrain) units |
| **Cast Time** | none |
| **Target Range** | 475 units |
| **Speed** | 750 + 60% movement speed |
| **Cooldown** | 0.5 / 0.4 / 0.3 / 0.2 / 0.1 (Starts after the dash ends) seconds |
| **Cooldown Start** | post-effect |
| **On-target CD Static** | 10 / 9 / 8 / 7 / 6 (Starts upon starting the dash) |
| **Queue Time** | $0.5$ seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Single target |
| **Terrain Grace** | True |
| **Call For Help** | True |
| **Grounded** | True |
| **Knockdown** | True |

**ACTIVE:** **Yasuo** dashes a fixed distance in the direction of the target enemy and becomes ghosted for 2 seconds, refreshing on subsequent casts. Upon impact, he deals magic damage to the target and generates a stack of *Ride the Wind* for 5 seconds, which refreshes on subsequent hits and stacks up to 4 times.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 85 / 100 / 115 / 130 (+ 20% **bonus** AD) (+ 60% AP) |

**RIDE THE WIND:** *Sweeping Blade*’s damage is increased by 25% per stack, up to 100% at maximum stacks.

| Attribute | Value |
|-----------|------:|
| **Bonus Damage per Stack** | 17.5 / 21.25 / 25 / 28.75 / 32.5 (+ 5% **bonus** AD) (+ 15% AP) |
| **Maximum Bonus Damage** | 70 / 85 / 100 / 115 / 130 (+ 20% **bonus** AD) (+ 60% AP) |
| **Total Combined Damage** | 140 / 170 / 200 / 230 / 260 (+ 40% **bonus** AD) (+ 120% AP) |

*Sweeping Blade* can be cast on the same target only once every few seconds.

***Yasuo** will be knocked down by any immobilizing or polymorphing crowd control during the dash.*

*Last Breath can be cast during the dash.*

**Notes:**

- If Steel Tempest has been buffered, Wind Wall may also be cast during the dash.
- *Sweeping Blade* will generate $7.5$ Flow stacks per cast that travels the maximum distance.
- **Yasuo** can use *Sweeping Blade*’s fixed dash distance to cross walls both before and after hitting his target (provided his proximity to the wall and/or target allows it).
- After *Sweeping Blade*, **Yasuo** becomes ghosted for a moment to prevent situations where he would face extreme cases of body block.
- Sleep does not count for knocking the dash down.

---

### R: Last Breath

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 1400 units |
| **Effect Radius** | 400 (Anchored around the target) units |
| **Cooldown** | 70 / 60 / 50 / 40 / 30 seconds |
| **Targeting** | Unit / Proximity |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | Single target |
| **Out of Range Behavior** | False |
| **Call For Help** | False |
| **Grounded** | True |

**ACTIVE:** **Yasuo** blinks to the opposite (Relative to Yasuo's cast location) of a visible airborne enemy champion nearest to the cursor (searches in a 1100 radius), instantly generating maximum Flow while resetting *Gathering Storm* stacks.

Upon arrival, he knocks up all nearby airborne enemy champions for 1 second, revealing them, becoming unable to act himself, and slashing them with his sword over the duration to deal physical damage thereafter. Surrounding enemy champions that become airborne during this time will also be affected by *Last Breath* through the remaining duration.

| Attribute | Value |
|-----------|------:|
| **Physical Damage** | 200 / 350 / 500 (+ 150% **bonus** AD) |

For the next 15 seconds, the damage dealt by **Yasuo**’s critical strikes ignores (lethality) 60% of the target's **bonus** armor.

*A nearby airborne enemy champion is required to cast this ability. If **Yasuo** would blink inside the attack range of an enemy turret (excluding the Nexus Obelisk), Last Breath will instead attempt to position him outside of it.*

**Notes:**

- *Last Breath* can be cast on targets suspended in mid-air by Aqua Prison’s stun.
  - The wiki has adopted the term *suspension* for this unique property due to a statement by **Yasuo**’s designer.
- The percentage bonus armor penetration stacks additively with other sources of percentage armor penetration.
- **Yasuo** gains the bonus armor penetration on-cast.
- *Last Breath* requires the target to be affected by airborne from an enemy source (from the target's perspective), and simultaneously that the target is not considered cc-immune, displacement immune, or untargetable during it.
  - *Last Breath* cannot be cast if there are no nearby enemy champions that meet the conditions.
    - Attempting to cast the ability on an enemy champion that is not airborne will indicate that they "*Must Be Airborne!*".
    - Spell shield will not prevent *Last Breath* from being cast and the target will be knocked up and dealt the damage.
      - The spell shield is still consumed in this case.
  - Self-applied or allied-applied displacements do not allow *Last Breath* to target the champion. Those applied by the *neutral* team (e.g. Dragon initial knock back) **can** be targeted.
  - An indicator will be placed towards enemy champions that meet the cast conditions within range of *Last Breath* to signify the ability can be cast.
  - The Hextech Ultimatum will prevent **Yasuo** from casting *Last Breath* on an airborne enemy champion far outside its borders.
    - **Yasuo** can only do so if the airborne enemy champion is positioned slightly outside of the borders.
- Upon *Last Breath*’s cast, the targets' facing directions snap to **Yasuo**. Afterwards, their facing directions shift in all other directions every $0.25$ seconds over the duration of the airborne.
  - The last shift snaps them to the facing direction of the second one.
  - On the Yasuo skin, these shifts in facing direction do not occur, but the targets' facing directions will still snap to **Yasuo** upon *Last Breath*’s cast.
- **Yasuo** does not need sight of enemy champions near his target to affect them with the ability.
- *Last Breath* applies its damage and removes its forced movement on the target when it ends, including if it is removed early by being cleansed.
  - This is in contrast to most other effects with airborne, which have their forced movement linger if the disabling debuff is removed and have to be overridden by another movement spell (such as Lucian’s Relentless Pursuit or Flash).
  - It behaves similar to Nami’s Aqua Prison in this but is not considered suspension.
- Terrain displacing abilities (e.g. Weaver's Wall, Pillar of Ice, Volcanic Rupture) that affect enemy champions are considered airborne, and will therefore interact with *Last Breath*.
- While *blinking*, **Yasuo** creates an untargetable *clone* of himself that cues a dashing animation (similar to Sweeping Blade’s) towards the target. This unit's rules do not follow those of a clone's. As such, it is more alike a champion, unless special cased.
- While performing *Last Breath*, **Yasuo** is locked out of performing actions.
  - The lock out will end prematurely if all targets:
    - Cleanse the knock up.
    - Move far away from **Yasuo**.
    - Leave **Yasuo**’s sight (e.g. by nearsight).
    - Become untargetable.
    - Die.
    - Enter the Realm of Death.
- The following table refers for interactions while **Yasuo** is locked out:

---

## Patch History

### V25.14
- Stats
  - Attack damage growth reduced to $2.5$ from 3.

### V25.11
- Last Breath
  - **Bug Fixes:** While he has gained the effects of the bonus-armor penetration, no longer incorrectly deals excessive damage to K'Sante who is All Out and has equipped Aftershock.
  - **Undocumented:** Percentage bonus-armor penetration now stacks additively with other sources of percentage armor penetration.

### V25.05
- Steel Tempest
  - **Bug Fixes:** While Gathering Storm stacks and Ride the Wind’s stacks are expiring or have expired very close in time to one another, *Steel Tempest* no longer fails to cast.

### V25.04
- Way of the Wanderer
  - **Removed:*** No longer grants Yasuo a critical damage penalty of 10%.
    - This penalty would also apply to Steel Tempest.
- Wind Wall
  - **Bug Fixes:** Renata Glasc’s Hostile Takeover no longer sometimes bypasses Wind Wall.

### V14.24
- Wind Wall
  - **Bug Fixes:** SFX is no longer audible through the Fog of War.

### V14.20
- Steel Tempest
  - Total critical damage penalty changed to -10% from -20%. No longer applies an *additional* critical damage penalty; only applies Way of the Wanderer’s penalty.
    - Critical strike AD ratio increased to 165.375% (175% base critical damage AD from 147% (175% base critical damage AD.
- Last Breath
  - Bonus armor penetration increased to 60% from 50%.

### V14.15
- Stats
  - Base armor increased to 32 from 30.
- Sweeping Blade
  - Base damage increased to 70 / 85 / 100 / 115 / 130 from 60 / 70 / 80 / 90 / 100.
  - Bonus damage per stack increased to 25% at all levels from 15 to 25.

### V14.11
- Steel Tempest
  - **Bug Fixes:** Using Recall during a Sweeping Blade + *Steel Tempest* combo no longer incorrectly delays the knock-up until *Recall*’s channel ends or is interrupted.

### V14.10
- Way of the Wanderer
  - Critical strike chance multiplier reduced to 2 from $2.5$.
  - Bonus attack damage per excess critical strike chance increased to $0.5$ from $0.4$.

### V14.9
- Stats
  - Selection radius increased to 120 units from 75.

## Trivia

- Yasuo - Yone is one of seven pairs of sibling champions (the others being Jinx - Vi, Cassiopeia - Katarina, Kayle - Morgana, Garen - Lux, Nasus - Renekton, and Darius - Draven).
  - Though not a pair, Anivia, Ornn, and Volibear are also siblings.
- The flute Yasuo plays in his dance emote is most likely a Shakuhachi (harmonica for Yasuo; wind controller for Yasuo).
  - His Classic 'dance' references a performance of this instrument.
    - A by comparison can be seen here.
    - Yasuo is the first champion to have an emote that will orient itself the same (dancing emote, facing the bottom-right of the screen), no matter what direction Yasuo was facing. Akali is the second.
  - The melody of his Classic dance complements Yone when the two use the dance emote simultaneously. A side-by-side comparison can be seen here.
- Yasuo's design is largely based on the Japanese Rōnin (lit. a wanderer), a samurai whose masters had died or stripped of titles & lands. Many rōnin became mercenaries, Yojimbo, or even criminals.
- The length of Yasuo's sword is about that of a tachi or even an ōdachi. However, he wears it cutting edge up like the shorter katana.
  - From Old Japanese **yasu-* "cheap, easy > peaceful" & masculine morpheme **wo*.
- Way of the Wanderer’s modified critical strike chance references Samurai gameplay portrayals in video games.
- For April Fools' Day 2017, a video showcasing a joke rework for Yasuo was posted by the official Riot Games Twitter account.
  - Way of the Wanderer was shown to have **RESOLVE'S** shield also provide crowd control immunity, similar to Morgana’s Black Shield.
  - Steel Tempest’s third cast was shown to split into three additional whirlwinds in a cone after hitting a champion, similar to Graves’ Collateral Damage.
  - Wind Wall was shown to form a pentagon of walls, similar to Thresh’s The Box.
    - Notably, this 'protection from projectiles from all angles' mechanic was later implemented by Samira’s Blade Whirl.
  - Sweeping Blade was shown to be able to target wards and champion summoned units (e.g. Demacian Standard).
  - Last Breath was shown to be able to be recast for a seemingly infinite amount of times while the affected champion is still airborne.
- A functional real-life replica of Yasuo's Steel Blade was crafted in an episode of YouTube series Man At Arms: Reforged.
  - This video can be viewed here.
    - There are also videos where the following are crafted:
      - Diana’s Crescent Moonblade (functional)
      - Katarina’s Daggers (functional)
      - Leona’s Zenith Blade (functional)
      - Master Yi’s Highlander' Ring Sword (functional)
      - Poppy’s Hammer of Orlon (functional, but impractically wieldy)
      - Ziggs’ Hexplosive Bomb (prop)
- On 25 August 2018, some results of a survey regarding champion gameplay was published by Riot Blaustoise. On the statement "One of my favorites", players ranked Yasuo 2nd in CN and 56th in NA. On "Fair to play against" they ranked him 23rd in CN and 137th in NA. Overall on "Gameplay, Visuals, Voice, etc.." they ranked him 1st in CN and 38th in NA.
- As an easter egg, in Set 4 of Teamfight Tactics, Yasuo would flash a Mastery 7 emote if he single-handedly defeats the enemy team as the only remaining unit. This references the community stereotype of Yasuo players loving to flash their Mastery in League of Legends matches.
  - A video demonstration of this can be seen here.

---
*This page was automatically generated from League of Legends Wiki data.*