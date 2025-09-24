# Warwick

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
| **Champion** | Warwick |
| **Title** | the Uncaged Wrath of Zaun |
| **Resource** | Mana |
| **Range Type** | Melee |
| **Release Date** | 2009-02-21 |
| **Release Patch** | Alpha Week 2 |
| **Latest Changes** | V25.16 |
| **Roles** | Diver |
| **Riot Positions** | Jungle |
| **External Positions** | Top, Jungle |
| **Blue Essence** | 225 |
| **Riot Points** | 260 |
| **Difficulty** | 1 |
| **Hero Type** | Fighter |
| **Alt Type** | Tank |
| **Adaptive Type** | Physical |
| **Damage** | 2 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 30 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $620.0$ | $+99.0$ |
| **Mana** | $280.0$ | $+35.0$ |
| **Health Regen** | $4.0$ | $+0.75$ |
| **Mana Regen** | $7.45$ | $+0.6$ |
| **Armor** | $33.0$ | $+4.4$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $65.0$ | $+2.75$ |
| **Attack Speed** | $0.638$ | |
| **Movement Speed** | $335.0$ | $+0.0$ |
| **Attack Range** | $125.0$ | $+0.0$ |
| **Base Attack Speed** | $0.638$ | |
| **Attack Speed Ratio** | $0.638$ | |
| **Bonus AS per Level** | $2.3\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Gameplay Radius** | $74.75$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $120$ units | |
| **Selection Height** | $191.667$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $105.0\%$ |
| **Damage Taken** | $95.0\%$ |
| **Healing** | $100.0\%$ |

#### Ultimate Spellbook

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |

## Abilities

### Passive: Eternal Hunger

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | Proc |
| **Parry** | True |

**INNATE:** **Warwick** deals 6 to 46 (+ 15% **bonus** AD) (+ 10% AP) **bonus** magic damage on-hit.


While below 50% **maximum** health, **Warwick** also heals for 100% of the post-mitigation damage (Damage calculated after resistances and modifiers.) dealt by *Eternal Hunger*, increased to 250% while below 25% **maximum** health.

**Notes:**

- *Eternal Hunger* does not affect:
  - structures
  - wards (except effigies),
  - Gangplank’s kegs
- *Eternal Hunger* does affect:
  - Illaoi’s Tentacles [https://www.youtube.com/watch?v=Sk43knTvpZ4&t=388s] Other units with modified health behaviour.

---

### Q: Jaws of the Beast

| Attribute | Value |
|-----------|------:|
| **Range** | Global (Charge tracking/following range) |
| **Cast Time** | none (See notes) |
| **Target Range** | 365 units |
| **Cost** | 80 / 85 / 90 / 95 / 100 Mana |
| **Cooldown** | 8 / 7.5 / 7 / 6.5 / 6 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Unit |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Parry** | True |
| **Call For Help** | True |
| **Grounded** | Special |
| **Knockdown** | Special |
| **Silence** | True |

**ACTIVE:** **Warwick** lunges at the target enemy over and bites them, dealing magic damage, healing himself for a percentage of the post-mitigation damage (Damage calculated after resistances and most modifiers.) dealt, applying on-hit effects and life steal at 100% effectiveness, and triggering on-attack effects. The damage based on the target's health ratio is capped against monsters.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 6 / 7 / 8 / 9 / 10% of target's **maximum** health (+ 120% AD) (+ 100% AP) |
| **Maximum Monster Damage** | 150 / 165 / 180 / 195 / 210 (+ 120% AD) (+ 100% AP) |

| Attribute | Value |
|-----------|------:|
| **Healing Percentage** | 25 / 37.5 / 50 / 62.5 / 75% |

While *Jaws of the Beast* is active, **Warwick** is displacement immune and clamps his jaw on the target, following **all** of their movement. The ability can be held for additional effects.

**HOLD:** **Warwick** performs the bite, and then begins to charge for $0.5 seconds$, extended if the target is under effects of a dash or displacement. During the charge, he leaps behind the target.

**Notes:**

- *Jaws of the Beast* deals basic damage, but also triggers spell effects by also being tagged as spell damage.
- *Jaws of the Beast* has a cast *delay*, which is the amount of time required for the bite to occur from the start of the cast. This should not be confused with a cast *time*.
  - If the cast started beforehand, both the damage and the heal occur even after **Warwick**’s death, or during resurrection.
- If the target dies prior to **Warwick** reaching them, *Jaws of the Beast* refunds its cooldown when tapped. The ability can be still held, however, **Warwick** is no longer eligible for the refund in this case. 
  The refund can occasionally fail.
- The healing is calculated only from *Jaws of the Beast*’s damage, while additional effects, such as on-hits, are not.
- While the bite always occurs after the same delay, the time spent lunging does not, which depends on the range *Jaws of the Beast* was cast away from the target, up to $0.5$ seconds when cast at maximum range.
  - This does not affect the lockout timer, which is the same as the bite's delay.
- The lunge always places *Warwick* to his attack range to the center of his target, while the dash will place him up to the edge of his attack range from the target's edge.
- It has been special cased so **Warwick** does not follow an enemy's Recall or a Sion that revives from I.
 This may occasionally fail. Also note that as of currently, if *Warwick* manages to bite the target in those conditions, he will almost always end up following them to their base.
- *Jaws of the Beast*’s channel cannot be initiated while grounded or rooted. **Warwick** can only use the normal cast.
  - The channel will **not** be interrupted by either of the two effects, despite the lock-on being a movement channel. **Interactions & Other**
- In order to hit the cap, the target monster needs to have at least (150 to 210)/((6 to 10)/100) health.
- *Jaws of the Beast* is the only ability in the game that combines channeling with displacement immunity. Every interrupting crowd control effect that is not resisted by the displacement immunity will prematurely stop the dash.
  - The bite can still occur if cast before the crowd control application, but **Warwick** cannot start channeling.
  - Abilities that apply both stun and airborne at the same time cause **Warwick** to ignore the displacement, but prematurely stop the channel and the dash.
  - The only exception to this is Sion’s I.
- **Warwick** will follow enemies that move via attachment effects but the lock-on will **not** be extended.
- While *Jaws of the Beast* does not use Quick Casting by default, click-and-hold is comparable to the input required to use Bone Skewer or Comet Spear. It does not accept two-click input (e.g. First Cast and Recast) that abilities like Piercing Arrow or Decimating Smash use in Standard Casting.
- **Warwick** will attempt to basic attack the target after a short delay.
- The following table refers for interactions while **Warwick** is channeling:
  - Flash, Teleport, Hexflash, and Recall are disabled during the first $0.4$ seconds of the channel. **Known Issues**
- *Jaws of the Beast* will also refund its cooldown if it kills the target after being held for exactly its bite delay.
- The ability may briefly cast on a dead target, with the ability going on cooldown.

---

### W: Blood Hunt

| Attribute | Value |
|-----------|------:|
| **Range** | Global (Passive detection of wounded enemies) |
| **Cast Time** | $0.5$ seconds |
| **Effect Radius** | 4000 (Search radius for target) units |
| **Cost** | 55 Mana |
| **Cooldown** | 80 / 70 / 60 / 50 / 40 seconds |
| **Targeting** | Auto |
| **Affects** | Self / Enemies |
| **Spell Shield** | False |

**PASSIVE:** **Warwick** gains **bonus** attack speed for $1.25$ seconds upon damaging a target to or while they are below (health) 50% of their **maximum** health with a basic attack or ability damage, refreshing with subsequent damaging basic attacks or ability damage to enemies below the threshold. If **Warwick** does not have the **bonus** attack speed already, he also gains it upon initiating an attack windup against an enemy below the threshold.

| Attribute | Value |
|-----------|------:|
| **Bonus Attack Speed** | 70 / 80 / 90 / 100 / 110% |

**PASSIVE:** **Warwick** senses all enemy champions on the map who are damaged below (health) 50% of their **maximum** health by him or an allied source, marking them with *Blood Hunt* until they (health regeneration) regenerate or heal above this threshold again. He sees trails leading toward them and gains (ms) **bonus** movement speed while following a trail. This bonus is lost for $0.5$ seconds upon entering champion combat, but will build up again over $3.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 35 / 42.5 / 50 / 57.5 / 65% |

The attack speed and movement speed bonuses granted by both passives are doubled against enemies who are below (health) 25% of their **maximum** health.

| Attribute | Value |
|-----------|------:|
| **Increased Attack Speed** | 140 / 160 / 180 / 200 / 220% |

| Attribute | Value |
|-----------|------:|
| **Increased Movement Speed** | 70 / 85 / 100 / 115 / 130% |

**ACTIVE:** **Warwick** senses the nearest enemy champion in the area at the start of the cast time, marking them with *Blood Hunt* for 8 seconds, gaining both passive bonuses against them regardless of their (health) **current** health.

*Blood Hunt*’s cooldown is reduced by 30% if no enemy champions are found within range.

**Notes:**

- The initiation of the attack windup to gain the bonus attack speed is only for *gaining* the bonus; it does not refresh the duration of the attack speed buff if **Warwick** has it already.
- If *Blood Hunt* was cast with no enemy champions within range, the ability incurs its cast time then goes on cooldown without marking any target.
- As mentioned by the description, *Blood Hunt* only marks enemy champions if they are damaged below the thresholds by an allied source. This includes **Warwick** himself, champions, minions, and turrets.
  - The only exception is the allied Nexus Obelisk, because that deals internalraw damage, which does not trigger damage-related events such as *Blood Hunt*’s marking.
- Note that monsters are neutral units.
- *Blood Hunt* can detect and target decoys.
- *Blood Hunt* will occur from wherever **Warwick** was at the start of the cast time.
- *Blood Hunt* has an indicator that appears above the target(s) when they are being hunted. Some skins have a unique indicator. Warwick Blood Hunt Mark.png | Classic PROJECT Warwick Blood Hunt Mark.png | PROJECT Lunar Guardian Warwick Blood Hunt Mark.png | Lunar Guardian Old God Warwick Blood Hunt Mark.png | Old God Winterblessed Warwick Blood Hunt Mark.png | Winterblessed Prestige Winterblessed Warwick Blood Hunt Mark.png | Prestige Winterblessed Arcane Vander Warwick Blood Hunt Mark.png | Arcane Vander
- Some of Warwick's have a unique color variation: Old God Old God Warwick Ruby Blood Hunt Mark.png | Ruby Old God Warwick Citrine Blood Hunt Mark.png | Citrine Old God Warwick Emerald Blood Hunt Mark.png | Emerald Old God Warwick Sapphire Blood Hunt Mark.png | Sapphire Old God Warwick Rose Quartz Blood Hunt Mark.png | Rose Quartz Old God Warwick Obsidian Blood Hunt Mark.png | Obsidian Old God Warwick Aquamarine Blood Hunt Mark.png | Aquamarine Old God Warwick Pearl Blood Hunt Mark.png | Pearl Old God Warwick Dark Ritual Blood Hunt Mark.png | Dark Ritual

---

### E: Primal Howl

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Effect Radius** | 375 (Recast effect radius) units |
| **Cost** | 40 Mana |
| **Cooldown** | 15 / 14 / 13 / 12 / 11 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Spell Shield** | True |

**ACTIVE:** **Warwick** gains damage reduction for up to $2.5$ seconds. *Primal Howl* can be recast after 1 second, and does so automatically after the duration.

| Attribute | Value |
|-----------|------:|
| **Damage Reduction** | 35 / 40 / 45 / 50 / 55% |

**RECAST:** **Warwick** howls, ending *Primal Howl*’s effects and fearing nearby enemies for 1 second, slowing them by 90%.

Starting *Infinite Duress’* channel while *Primal Howl* is active will initiate the recast without ending the damage reduction buff prematurely.

**Notes:**

- The recast's howling animation has a $0.25$-second lockout time.
  - This will also cancel **Warwick**’s basic attack.
  - **Warwick** cannot attack or cast abilities during the animation, instead the last input will be buffered to play after it ends.
- **Warwick** will automatically recast if he enters resurrection.

---

### R: Infinite Duress

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.1$ seconds |
| **Collision Radius** | 205 units |
| **Cost** | 100 Mana |
| **Cooldown** | 110 / 100 / 90 / 80 / 70 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Special |
| **Parry** | True |
| **Grounded** | True |
| **Knockdown** | False |
| **Silence** | True |

**ACTIVE:** **Warwick** leaps in the target direction with crowd control immunity, stopping upon hitting an enemy champion. He then knocks them down and channels for up to $1.5$ seconds to suppress, reveal, and deal magic damage every $0.25$ seconds, revealing himself in the process.

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 175 / 262.5 / 350 / 437.5 / 525 (+ 167% **bonus** AD) |

**Warwick** heals for 100% of all post-mitigation damage (Damage calculated after resistances and modifiers.) he deals to the target during *Infinite Duress*.

*Infinite Duress* applies on-hit effects and life steal at 100% effectiveness, as well as triggers on-attack effects, 3 times.

*Primal Howl can be cast during the dash.*

**Notes:**

- *Infinite Duress* deals basic damage, but also triggers spell effects by also being tagged as spell damage.
- The minimum range of the ability is 275 units, based on the minimum movement speed that a champion can have (110) while still being able to move. The "base" range of this ability is $837.5$ because of his 335 base movement speed.
  - Considering movement speed soft caps, with Blood Hunt, this range increases to 1112 / 1162.25 / 1203.12 / 1234.53 / 1265.94, further increased to 1286.88 / 1349.69 / 1412.5 / 1475.31 / 1538.12 toward enemy champions below 25% health.
- *Infinite Duress* deals a total of 6 damage instances:
  - The 1st, 3rd, and 5th ticks deals 2/9ths of the total damage as a single instance while applying both spell and on-hit effects.
  - The 2nd, 4th, and 6th ticks deal 1/9ths of the total damage as a single instance while applying spell effects but not on-hit effects.
- If **Warwick** does not collide with a champion, he will slide a fixed distance (estimated 150 units) past the maximum range during which he is no longer crowd control immune and unable to declare basic attacks or cast movement abilities, including Flash, but being able to cast Jaws of the Beast and Blood Hunt, with the former also interrupting the slide.
  - **Warwick** is able to cast Primal Howl at any time during the dash.
- Activating Zhonya's Hourglass during the dash interrupts it.
- Each hit counts as a separate hit for effects such as Electrocute and Eclipse Ever Rising Moon.
- **Warwick** gains the cc-immunity at the start of the cast time.
- Cleansing the suppression does not interrupt the channel and **Warwick** will continue to strike but not deal any damage.
  - *Infinite Duress* is special cased to have its channel interrupted by Fate's Call.
- If the target is crowd control immune or protected by spell shield upon collision, **Warwick** will not start the channel.
- If the target is in a zombie state, **Warwick** will stop the channel after ~$0.5$ seconds of hitting them.
- *Infinite Duress* covers **Warwick**’s hitbox, meaning he will collide with enemy champions that are particularly close to him.
- The following table refers for interactions while **Warwick** is in cast time:
- The following table refers for interactions while **Warwick** is channeling:

---

## Patch History

### V25.16
- Stats
  - Base attack damage increased to 65 from 60.
- Eternal Hunger
  - On-hit base damage reduced to 6 to 46 from 12 to 46.

### V25.08#April 17th Hotfix|V25.08
- Blood Hunt
  - **Bug Fixes:** If Warwick dies under an enemy turret's range while an enemy is marked, no longer causes the mark's phantom unit to become targetable to that enemy turret at his death location after he respawns, priority permitting, which can persist permanently including across subsequent respawns and causes the turret to never be able to change its target for the remainder of the game.
    - Warwick has been re-enabled.

### V25.05
- General
  - **Bug Fixes:** Restored First Encounter VO against Soraka and Urgot.

### V25.04
- General
  - **Bug Fixes:** Corrected various VO errors.
- Jaws of the Beast
  - Mana cost increased to 80 / 85 / 90 / 95 / 100 from 70 / 75 / 80 / 85 / 90.

### V25.S1.1
- Blood Hunt
  - **Bug Fixes:** No longer grants bonuses while no skill points have been spent for the ability.

### V14.24#December 18th Hotfix|V14.24
- Stats
  - Base attack damage reduced to 60 from 63.
  - Attack damage growth reduced to $2.75$ from 3.

### V14.24#December 12th Hotfix|V14.24
- Jaws of the Beast
  - Cooldown increased to 8 / 7.5 / 7 / 6.5 / 6 from 6 at all ranks.
  - Mana cost increased to 70 / 75 / 80 / 85 / 90 from 50 / 60 / 70 / 80 / 90.
- Blood Hunt
  - Attack speed duration reduced to $1.25$ seconds from $2.5$.

### V14.24
- Stats
  - Base attack damage reduced to 63 from 65.
- Jaws of the Beast
  - Target range increased to 365 from 350.
- Blood Hunt
  - Bonus attack speed duration increased to $2.5$ seconds from $0.75$.
  - **Removed:*** Basic attacks on-hit and ability hits no longer grant the bonus attack speed.
  - **New Effect:** Damaging basic attacks and ability damage now grant the bonus attack speed both when damaging the target to below the threshold and while they are already below it.
  - **New Effect:** Basic attacks against enemies below the threshold now grant the bonus attack speed at the start of the attack windup if he does not have the bonus already.
    - *Duration of the attack speed buff cannot be refreshed this way.*
  - **New Effect:** Now displays duration on the status bar.
  - **Bug Fixes:** Audio for hunting low health targets marked by *Blood Hunt* no longer fails to activate upon them reaching the threshold.

### V14.22
- Stats
  - Now starts the game with a +15% modifier.
    - Starting gameplay radius increased to $74.75$ units from $65$.
- Blood Hunt
  - Mana cost reduced to 55 from 70.
  - Cooldown reduced to 80 / 70 / 60 / 50 / 40 seconds from 100 / 85 / 70 / 55 / 40.
  - Enhanced attack speed and movement speed modifier reduced to 200% from 250%.
    - Enhanced bonus attack speed reduced to 140 / 160 / 180 / 200 / 220% from 175 / 200 / 225 / 250 / 275%.
    - Enhanced bonus movement speed reduced to 70 / 85 / 100 / 115 / 130% from 87.5 / 106.25 / 125 / 143.75 / 162.5%.
  - Threshold for enhanced bonuses increased to 25% of target's **maximum** health from 20%.
  - **New Effect:** Passive bonus attack speed is now also granted by ability hits against enemies below the threshold, instead of only basic attacks.
  - **New Effect:** Passive bonus attack speed now lingers for $1.1$ seconds after being gained.
  - **Removed:*** No longer has a separate animation for when no targets are found within range.
  - **Removed:*** No longer is placed on a 3-second static cooldown while in combat with champions.
  - **Removed:*** No longer is locked out of being cast while in champion combat.
  - **Removed:*** Cooldown no longer elapses twice as quickly while out of combat with champions.
  - **New Effect:** Cooldown is now reduced to 70% of the total if no enemy champions are found within range.
- Primal Howl
  - Duration of basic attack lockout after recasting reduced to $0.25$ seconds from $0.5$.
- Infinite Duress
  - **Undocumented:** Radius increased to 205 units from 155.
  - Width of the cast indicator increased to 125 units from 50.
  - **New Effect:** Now only collides with champions in front of him.
  - **Removed:*** No longer collides with targets whose center are behind him.

### V14.9
- Stats
  - Gameplay radius increased to 65 units from 55.
  - Selection radius increased to 120 units from $111.11109924316406$.

## Trivia

- Warwick's dance references Thriller by Michael Jackson.
  - A side-by-side comparison can be seen here.
- Warwick was the second champion to have his price reduced twice (the others being Garen and Miss Fortune).
- Warwick shares a name with a Warwick, where it means "abode by the weir".
  - However, the town's name is pronounced with the second w being silent.
- His ability icons depicted him in his Warwick skin's color scheme rather than his Warwick skin's.
- In the now-removed official League of Legends forums, the icon of Hunter's Call.png was used to represent the "Announcements" section.
- Following his rework, Warwick retains his dance reference to Thriller by Michael Jackson.
  - A side-by-side comparison can be seen here.
  - He shares this dance with Brand.
- Warwick is displayed when typing 'Urf' in Champion Select.
  - This is a reference to his Warwick skin (see more details in the "Relations" section below). However, as of his current rework, recent origins behind the skin is unknown.
- Warwick, Vayne and Trundle are the only champions without damaging area-of-effect abilities.

---
*This page was automatically generated from League of Legends Wiki data.*