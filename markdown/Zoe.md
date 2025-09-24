# Zoe

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
| **Champion** | Zoe |
| **Title** | the Aspect of Twilight |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2017-11-21 |
| **Release Patch** | V7.23 |
| **Latest Changes** | V25.15 |
| **Roles** | Burst |
| **Riot Positions** | Middle |
| **External Positions** | Middle |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 3 |
| **Hero Type** | Mage |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 3 |
| **Toughness** | 1 |
| **Control** | 2 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $630.0$ | $+106.0$ |
| **Mana** | $425.0$ | $+25.0$ |
| **Health Regen** | $7.5$ | $+0.6$ |
| **Mana Regen** | $8.0$ | $+0.65$ |
| **Armor** | $21.0$ | $+4.7$ |
| **Magic Resist** | $30.0$ | $+1.3$ |
| **Attack Damage** | $58.0$ | $+3.3$ |
| **Attack Speed** | $0.658$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $550.0$ | $+0.0$ |
| **Base Attack Speed** | $0.658$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.5\%$ | |
| **Missile Speed** | $1600$ units/second | |
| **Acquisition Radius** | $550$ units | |
| **Gameplay Radius** | $55$ units | |
| **Pathing Radius** | $30$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $145$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $110.0\%$ |
| **Damage Taken** | $95.0\%$ |

## Abilities

### Passive: More Sparkles!

| Attribute | Value |
|-----------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | Proc |
| **Projectile** | False |
| **Parry** | True |

**INNATE:** After casting an ability, **Zoe** empowers her next basic attack or *Spell Thief* bolt within 5 seconds to become non-projectile and deal 16 / then +4*x for 5 / then +6*x for 5 / then +8*x for 3 / then +10*x (+ 20% AP) **bonus** magic damage.

**Notes:**

- The empowered attack will trigger but not be consumed against wards.

---

### Q: Paddle Star!

| Attribute | Value |
|-----------|------:|
| **Range** | cr 800 (Initial missile) / er 800 (Redirected missile range from Zoe, continuously updating as she moves) / Global (Potential redirected missile range) |
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 180 (From collision point, both initial and redirected star, estimated) units |
| **Width** | 100 (Initial width) / 140 (After redirection) units |
| **Speed** | 1200 (Initially) / 2500 (After redirection) units/second |
| **Cost** | 40 / 45 / 50 / 55 / 60 Mana |
| **Cooldown** | 8.5 / 8 / 7.5 / 7 / 6.5 seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**ACTIVE:** **Zoe** shoots a star in the target direction that explodes upon hitting an enemy, dealing magic damage to nearby enemies hit. The **total** damage is increased by 0%@800; 25%@950; 50%@1350; 75%@1650; 100%@1950; 125%@2250; 150%@2550 (@=distance traveled).

| Attribute | Value |
|-----------|------:|
| **Minimum Magic Damage** | 2 / then +2*x for 8 / then +3*x for 4 / then +4*x (+ 50 / 80 / 110 / 140 / 170) (+ 60% AP) |
| **Maximum Magic Damage** | 2×2.5 / then +(2×2.5)*x for 8 / then +(3×2.5)*x for 4 / then +(4×2.5)*x (+ 125 / 200 / 275 / 350 / 425) (+ 150% AP) |

The star lingers at maximum range for 1 second, during which *Paddle Star!* can be recast after $0.25$ seconds in the duration.

**RECAST:** **Zoe** redirects the star in the target path, empowering it with increased speed and radius, and resetting its damage modifier on cast. The redirected star can travel until it reaches 800-units outbound from **Zoe**.

*Paddle Star!'s recast can be used while affected by cast-inhibiting crowd control.*

**Notes:**

- Both casts count as ability activations for the purposes of on-cast effects such as More Sparkles!, Spellblade and triggering Force Pulse’s passive.
- The recast radius matches the initial cast radius of the ability, but **Zoe** can move any amount of distance away from the projectile before reactivating.
- *Paddle Star!*’s effect radius is centered around the location of the missile as it collides.

---

### W: Spell Thief

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 2200 (Maximum distance from Zoe for enemy champions to drop a Spell Shard) / Global (Maximum distance from Zoe for minions slain by her to drop a Spell Shard) / 1500 (Maximum distance from Zoe for minions slain by allies to drop a Spell Shard) / er 525 (Bolt lock-on range) |
| **Cost** | 1 Spell Shard |
| **Cooldown** | $0.25$ seconds |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | magic |
| **Spell Shield** | True |
| **Spell Effects** | spell |
| **Projectile** | True |

**PASSIVE - WHEEEEE:** Whenever **Zoe** casts *Spell Thief* or a summoner spell, she gains (ms) **bonus** movement speed for a duration, and summons three bolts that orbit her for the next 10 seconds.

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed** | 30 / 40 / 50 / 60 / 70% |

| Attribute | Value |
|-----------|------:|
| **Bonus Movement Speed Duration** | 2 / 2.25 / 2.5 / 2.75 / 3 seconds |

If **Zoe** is not affected by complete crowd control, she shoots one bolt at a time at the nearest non-sleeping enemy in range, prioritizing her attack target, dealing magic damage with each bolt.

| Attribute | Value |
|-----------|------:|
| **Magic Damage Per Bolt** | 20 / 30 / 40 / 50 / 60 (+ 15% AP) |
| **Total Magic Damage** | 60 / 90 / 120 / 150 / 180 (+ 45% AP) |

**PASSIVE:** Whenever a nearby enemy champion casts a specific (See notes for applicable spells/item actives) or , they drop the corresponding **' onto the ground that grants sight over a 100-radius area and remains for 40 seconds. Enemy minions have a 10% chance to spawn with a random * that drops and remains for 20 seconds if **Zoe** kills them. Allied champions that kill them will cause the orb to drop only if **Zoe** is nearby.

**Zoe** can collect a *Spell Shard* by moving directly onto it, holding it for up to 60 seconds. Collecting a new *Spell Shard* replaces the current one.

**ACTIVE:** **Zoe** mimics the active effect of the collected *Spell Shard*.

**Notes:**

- The in-game tooltip for *Spell Thief* while **Zoe** has a *Spell Shard* uses the tooltip of the mimicked spell or item active.
- **Zoe** will cast *Spell Thief* in place of her own summoner spells if she has either of them as a collected *Spell Shard*, even if she tries to cast the summoner spell while it is on cooldown.
- Minions that can drop *Spell Shards* are marked with a tied cosmic balloon that floats above them.
- **Zoe** utilizes an attack-like animation whenever one of the bubbles hurls itself at an enemy, making it look as if she can attack while moving.
- Minions holding a *Spell Shard* can drop one of the following summoner spells or item actives:
  - **Applicable summoner spells:** Barrier, Cleanse, Exhaust, Ghost, Heal, Ignite, and Unleashed Smite.
    - All other summoner spells are not applicable for minions to drop, which includes Clarity, Hexflash, Mark / Dash, Smite, Teleport / Unleashed Teleport, and Primal Smite.
  - **Applicable item actives:** Hextech Rocketbelt, Locket of the Iron Solari, Profane Hydra, Randuin's Omen, Ravenous Hydra, Shurelya's Battlesong, Stridebreaker, Titanic Hydra, and Youmuu's Ghostblade.
- All summoner spells except Dash and Hexflash are applicable to drop as a *Spell Shard* when an enemy champion uses them.
- The recast duration of a mimicked Mark cast lasts longer than the mark duration.
  - This allows her to cast Dash while the mark is expired, though the spell will fail to trigger its effects with an attempted cast. *Spell Thief*’s passive effect is still triggered, however.
- *Spell Shards* may be collected while **Zoe** can cast Dash from a mimicked Mark cast. The new shard will replace the ability slot once Dash is used or expires.
- The following table lists item actives that can drop as a *Spell Shard*, as well as those that explicitly do not or are not applicable to drop, when an enemy champion activates them:
  - Consumable and trinket items are excluded automatically.

---

### E: Sleepy Trouble Bubble

| Attribute | Value |
|-----------|------:|
| **Range** | 800 (Initial missile range) + 50 (Bounce extra travel distance) + 250 (Bubble radius) units |
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 250 (Bubble trap radius) units |
| **Width** | 100 (Initial missile width) / 200 (Lollipop diameter at end of initial missile trajectory, also edge range) units |
| **Speed** | 1850 (Initial missile speed) units/second |
| **Cost** | 80 Mana |
| **Cooldown** | 16 / 15 / 14 / 13 / 12 seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Special |
| **Projectile** | True |

**ACTIVE:** **Zoe** kicks a bubble in the target direction that bursts upon the first enemy hit, otherwise lingering at maximum range over 1 second, then becoming a trap for 5 seconds that bursts upon contact with an enemy. The bubble can move through terrain only once, but travels the entire distance as **bonus** range, and will fall short if it would enter terrain again.

The burst deals magic damage to the target and inflicts them with drowsy for $1.4$ seconds, which gradually slows them until they fall asleep for $2.25$ seconds. While the target is asleep, they are inflicted with (magic penetration) 30% magic resistance reduction.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 110 / 150 / 190 / 230 (+ 45% AP) |

| Attribute | Value |
|-----------|------:|
| **Maximum Slow** | 10 / 15 / 20 / 25 / 30% |

The next instance of non-persistent damage that the sleeping target takes from champions, large monsters or turrets consumes the debuff to deal **bonus** true damage equal to the post-mitigation damage (Damage calculated after modifiers) dealt, capped at *Sleepy Trouble Bubble*’s damage.

| Attribute | Value |
|-----------|------:|
| **Bonus Damage Cap** | 70 / 110 / 150 / 190 / 230 (+ 45% AP) |
| **Maximum Mixed Damage** | 140 / 220 / 300 / 380 / 460 (+ 90% AP) |

**Notes:**

- Applies spell damage for the bubble and default damage for the wake-up damage.
- Spell shield will not prevent falling asleep nor block the wake-up damage.
  - The bubble's damage and drowsy application will be blocked.
- The **bonus** damage from *Sleepy Trouble Bubble* is dealt before **any** triggering damage from **Zoe** or her allies, and is attributed to **Zoe** at all times.
  - If an ally expunges the sleep and the bonus damage from *Sleepy Trouble Bubble* is enough to kill the target, **Zoe** will be credited the kill.
    - Additionally in this case, the allied champion will **not** be credited an assist at all.
- **Zoe** will gain obscured vision of the target while they're asleep if they are not visible.
- If exceptionally close to terrain, the bubble will pass through without detection and will still be able to pass through a second piece of terrain - although it sacrifices any range its already used going through the first piece.
- *Sleepy Trouble Bubble* can interact with player-generated terrain.
- At the end of the path of the initial missile, before continuing with the short bounce, *Sleepy Trouble Bubble* checks for targets in a er 100 radius to immediately collide with.
  - This "lollipop" is one of only a few that is larger than the ability's missile width.
- *Sleepy Trouble Bubble* can still be collided with during the short bounce between end of initial missile and the trap being formed.
  - This seems to use a er very small or negative collision radius, rather than normal missile collision. Effect at cast time end
  - The missile's maximum range location, or end position, is determined at the start of the cast and as such will always be fired to the same spot regardless of where **Zoe** is at the end of the cast time.
    - An applicable example of this can be seen in an interaction with her Portal Jump. If *Sleepy Trouble Bubble* is cast right before she blinks back to *Portal Jump*’s original cast position, the range of the missile will be extended for it to fire to the same end position as if she had not blinked back. This interaction occurs in this manner due to the missile's behavior as explained prior and the fact that **Zoe** blinks back with *Portal Jump* before the cast time of *Sleepy Trouble Bubble* finishes, causing the cast time to finish from *Portal Jump*’s original cast position.
  - This will still count as damage dealt by **Zoe** in the damage recap.

---

### R: Portal Jump

| Attribute | Value |
|-----------|------:|
| **Range** | 575 units |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 40 Mana |
| **Cooldown** | 11 / 9.5 / 8 / 6.5 / 5 seconds |
| **Cooldown Start** | post-effect |
| **Targeting** | Direction |
| **Affects** | Self |
| **Grounded** | True |

**ACTIVE:** After a $0.15$-second delay (From the end of the cast time, blink occurs after 0.4 seconds from the start of cast), **Zoe** blinks in the target direction and remains there for $0.95$ seconds to $1.25$ seconds (See notes), during which she sets her movement speed to a static 0 and has unobstructed vision. Afterwards, she blinks back to her casting position.

*Portal Jump resets **Zoe**’s basic attack timer. **Zoe** briefly (See notes) becomes unable to act after casting Portal Jump as well as after blinking back.*

**Notes:**

- **Zoe** will remain at the blink destination for an inconsistent period of time.
- *Portal Jump* causes **Zoe** to become unable to perform any actions for a varied ~$0.55$-$0.75$ seconds from the start of the cast time and another time for a varied ~$0.3$-$0.5$ seconds starting from right before she blinks back.
- **Zoe** is able to perform any action while at the blink destination, except practically being unable to move through normal movement due to having 0 movement speed.
- *Portal Jump*’s maximum range location, or end position, is determined at the start of the cast time and as such **Zoe** will always blink to the determined destination regardless of where she is at the end of the ability's cast time.
- *Portal Jump* will always cause **Zoe** to blink back to her casting position regardless of where she is moved to while at the destination.
  - She will still be considered to blink even if she fails to move her intended location while attached (e.g. if she were affected by The Show Stopper).
- Upon reaching her destination, **Zoe** will either raspberry at the nearest enemy champion, or lick an ice cream cone when there are no enemy champions in sight.
- **Zoe**’s return position is shifted by 25 units in the target direction from her cast location.
- The following table refers for interactions while **Zoe** is locked out during the cast time and for certain periods of time during the ability:

---

## Patch History

### V25.15
- Sleepy Trouble Bubble
  - **Bug Fixes:** Object indicator on the minimap no longer sometimes persists indefinitely even though the actual trap has expired.

### V25.14
- Paddle Star!
  - Base damage reduced to 2 / then +2*x for 8 / then +3*x for 4 / then +4*x from 7 / then +1*x for 1 / then +2*x for 9 / then +3*x for 4 / then +4*x. *Unchanged at levels 15 and above.*
- Spell Thief
  - Base damage per bolt reduced to 20 / 30 / 40 / 50 / 60 from 25 / 35 / 45 / 55 / 65.
  - AP ratio per bolt increased to 15% AP from $13.33$% AP.

### V25.08
- Paddle Star!
  - Secondary target damage increased to 100% from 80%.

### V25.S1.2
- Spell Thief
  - **Bug Fixes:** No longer displays old (pre-25.S1.1) summoner spell icons.

### V25.S1.1
- General
  - Now swaps Axiom Arcanist with Nimbus Cloak.

### V14.22
- Spell Thief
  - **Bug Fixes:** Enemy champions using Unleashed Teleport will now correctly drop its *Spell Shard*.
  - **Bug Fixes:** Shurelya's Battlesong *Spell Shard* no longer fails to activate Inspiring Speech when consumed.
  - **Bug Fixes:** Using Flash while not being under the effects of *Spell Thief*’s haste and bolts buff already no longer causes her visual position and her actual position to desync until a movement command is issued (which would subsequently cause her to appear as if "dashing" to her actual location, and remove the desync).

### V14.9
- Stats
  - Gameplay radius reduced to 55 units from 65.
  - Pathing radius reduced to 30 units from 35.
  - Selection radius reduced to 100 units from 120.

### V14.7
- Spell Thief
  - **Bug Fixes:** Profane Hydra and Titanic Hydra granted from this ability no longer play the VFX but fail to cast their effects when used.

### V14.2
- Spell Thief
  - **Bug Fixes:** Spell shard for Ravenous Hydra active no longer fails when cast.

### V13.22
- Stats
  - Base attack speed increased to $0.658$ from $0.625$.

## Trivia

- She is the first Heterochromia iridum champion (not counting Lulu). This trait is preserved in all of her skins.
- Upon jumping out of Portal Jump’s second portal, Zoe licks the ice cream cone, which she attempts to eat during her joke animation (Ctrl+1).
- **Zoe** has multiple walk cycles that change randomly over the course of the game, but players can also toggle the next one using the *Toggle* key bind (default [Ctrl] + [5]).
  - During her champion sneak peek video, Nautilus and Teemo make cameos:
    - Nautilus appears when Zoe is showing some of her dance moves.
    - Teemo shows up when Zoe falls through a portal and Zoe comes out of it.
    - At the end of her sneak peek video, Zoe is listening to Kinetic (The Crystal Method x Dada Life), one of Sona’s music tracks.
- Her title, as found on the store, "Zoe, The Aspect of Twilight" is one of the few occurrences, along with Kayn, Taliyah, Aurelion Sol, Illaoi, Elise, Sion, of having the word "the" capitalized, compared to all other champions' titles.
- Zoe has a cameo in the action role-playing game, CrossCode.
- Zoe's Series 1 Eternals make the following references:
  - *Finders Keepers* references the idiom finders keepers, losers weepers.
  - *Paddle Ball Champion* references the sport of Paddle_ball_(sport) and its similarites to Paddle Star!.

---
*This page was automatically generated from League of Legends Wiki data.*