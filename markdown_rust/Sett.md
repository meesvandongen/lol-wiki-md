# Sett

## Overview

- **Title:** Sett
- **Resource:** Grit

## Stats

| Stat | Base | Growth |
|------|------|--------|
| Armor | 33 | 4.7 |
| Attack Damage | 60 | 4 |
| Attack Speed | 0.625 | 1.75 |
| HP | 670 | 114 |
| HP Regen | 7 | 0.5 |
| MP | 0 | 0 |
| MP Regen | 0 | 0 |
| Magic Resist | 28 | 2.05 |
| Move Speed | 340 | 0 |
| Range | 125 | 0 |

## Advanced Stats

| Metric | Value |
|--------|-------|
| Acquisition Radius | 600 |
| Attack Delay Offset (s) | -0.0857 |
| Attack Speed Ratio | 0.625 |
| Pathing Radius | 35 |
| Selection Height | 100 |
| Selection Radius | 100 |

## Abilities

### W – false

| Attribute | Value |
|-----------|------:|
| **Range** | -25 Ã¢ÂÂ 720 (Trapezoid (and inner rectangle) length, starts behind Sett; 745 total length) |
| **Cast Time** | 0.75 |
| **Width** | 300 (Trapezoid smaller base, close to Sett) / 620 (Trapezoid larger base, farther from Sett) /  70 (Inner true damage rectangle width) |
| **Cost** | 100% |
| **Cost Type** | **Current** Grit |
| **Cooldown** | (+18 to 12% AP) |
| **Queue Time** | 0.50 |
| **Cone Radius** | 720 |
| **Cone Angle** | 54ÃÂ° |

**PASSIVE:** **Sett** stores 100% of post-mitigation damage (Damage calculated after resistances and modifiers.) taken as _Grit_ on his resource bar, up to a cap of 50% of his **maximum** health. Each instance of stored _Grit_ decays in value by 30% every second after 4 seconds.

**ACTIVE:** **Sett** charges up a strike over the cast time. Additionally, he immediately consumes all of his stored _Grit_ to grant himself a shield at the start of the cast time that is equal to the expended _Grit_ for 3 seconds, decaying in strength over the duration after 0.75 seconds.

After the cast time, he unleashes a massive blast in an area in the target direction, dealing physical damage to enemies hit; those hit in a line in the middle are dealt  instead.

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical True |
| **Spell Effects** | aoe |
| **Spell Shield** | True |
| **Projectile** | false |

**Notes:**

- _Haymaker's_ total hitbox uses the intersection of a trapezoid and a cone. Enemies are hit by _Haymaker_ only when their hitbox overlaps with both of those areas.
- The trapezoid and the rectangle both register the edge of enemies' hitbox.
- The shield is granted at the start of the cast time and will decay afterwards.
  - The buff for the shield is named _Down But Not Out_.
- **Sett** does not store _Grit_ from damage that was mitigated by shields.
- **Sett** will turn towards the target direction at the start of the cast time.
- **Sett** will glow when attaining 90% _Grit_. Casting _Haymaker_ at this amount has a different visual and audio effect. 

### E – Facebreaker

| Attribute | Value |
|-----------|------:|
| **Range** | 450 (In front and behind of Sett, center-to-edge range) |
| **Cast Time** | 0.25 |
| **Width** | 280 (Rectangle width, center-to-edge range) |
| **Cooldown** | (+16 to 10% AP) |
| **Queue Time** | 0.50 |

**ACTIVE:** **Sett** pulls in enemies at his front and back along the target direction, dealing physical damage and slowing them by 70% for 0.50 seconds. _Facebreaker_ deals 125 to 250 **bonus** physical damage to monsters.

If _Facebreaker_ affects at least one enemy on each side, all enemies are stunned for 1 second upon landing.

**_Sett** becomes unable to move or attack for 0.25 seconds after Facebreaker's cast time._

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | aoe |
| **Spell Shield** | true |
| **Projectile** | False |

**Notes:**

- **Sett** will afterwards attempt to basic attack the closest target picked up by _Facebreaker_, prioritising champions and prioritising those in front of him.
- If _Facebreaker_ does not hit an enemy, Sett is also unable to be cast for 0.25 seconds after the cast time.
- Targets will be pulled towards **Sett's** location in a straight line and rebound to 150 units from him back in the same line.
- The [spell indicator](./spell_indicator.md) is slightly shorter than the actual hitbox' rectangle length, and slightly wider than the width (note that the effects are edge-range from the hitbox rectangle).
- Targets protected by spell shields don't count toward the requirement to stun. (cast)

### R – The Show Stopper

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 400 |
| **Effect Radius** | 600 (From Sett's own final landing destination (may be a bug)) |
| **Cooldown** | (+120 to 80% AP) |
| **Queue Time** | 0.50 |

**ACTIVE:** **Sett** suppresses and reveals the target enemy champion while dashing with displacement immunity to their location and attaching them to himself upon arrival. He leaps another 600 units in the same direction to slam the target into the ground, creating a massive shockwave and quickly sliding forward 250 units beyond the impact.

Enemies within the epicenter (125 unit radius around the target's landing location, but not the true center of the area of effect) take , and other enemies hit by the shockwave take physical damage that is reduced by up to 75% based on proximity. All targets hit are slowed by 99% for 1 second.

_The leap will end and create the impact prematurely upon encountering terrain that cannot be dashed through, whether by distance or invalid space._

| Detail | Value |
|--------|------:|
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Effects** | aoe |
| **Spell Shield** | true |
| **Grounded** | true |
| **Knockdown** | False |

**Notes:**

- _The Show Stopper's_ attachment depends on the suppression; if the target does not remain suppressed by the end of the dash, they are not attached to **Sett** when he arrives to them. Similarly, if the suppression is removed, so is the attachment.
  - **Sett** will still dash to the target and leap forward from them to trigger the shockwave afterwards even if he fails to attach the target to him.
  - If the target resists the suppression by being immune to crowd control, displacement immune, or having a spell shield, **Sett** will not attach them to himself.
  - If the target removes the suppression by any means, including with an applicable cleanse effect or dispel, they will instantly detach themselves from **Sett**.
    - Neither his dash nor leap are interrupted in this case.
- **Sett's** slide dash after completing the leap dash cannot pass through terrain.
- **Sett** is displacement immune during both the initial dash and the leap after that dash, as well as for a short duration during his slide beyond on the impact.
- The target is revealed specifically while they are suppressed.
- If the target is untargetable when **Sett** completes their dash to them, _The Show Stopper_ will be cancelled immediately.
  - In this case, **Sett** will not leap forward nor attach to the target to himself and the suppression is removed from the target.
- If the target moves 2000 units or more away from their location at the time of _The Show Stopper's_ cast before **Sett** completes their dash to them, **Sett** will fail to attach the target to him.
  - The target will remain suppressed for 5 seconds if this occurs.
- _The Show Stopper_ will be cancelled if either **Sett's** dash or leap is cancelled during the ability by any means.
  - The former causes **Sett** to not leap forward nor attach to the target to himself and the suppression to be removed from the target.
  - The latter causes **Sett** to detach the target from himself instantly and the suppression to be removed from the target.
- _The Show Stopper_ will not be cancelled if the target dies or enters resurrection during the effect.
- At maximum cast range and if the dash is not shortened due to terrain in **Sett's** way, _The Show Stopper's_ movement takes 1.50 seconds, causing the slam within 1.23 seconds. (Estimated)
- If **Sett** enters resurrection right before completing his initial dash, he will still leap forward from the target.
- The "crater" VFX originates from the target's landing, which inaccurately illustrates the damage's area of effect (from Sett).

## Trivia

- Rapidly using Sett's [dance](./Controls_and_Hotkeys.md) increases how fast he does his sit ups rather than restarting the animation.<ref>[https://twitter.com/Tom_Anim/status/1205191808919068672 Whist on Sett Dance]</ref>
- Sett was teased ingame with the  [Sett's Calling Card](./Hextech_Crafting_Removed_Content.md#Bundle Tokens) loot item earnable in patch [V9.24](./V9.24.md). It was [awarded](./Missions_(League_of_Legends)_2019.md) by "Impressing Sett" (scoring first blood in PvP games) and openable from January 14, 2020. The card contained a Sett Champion Permanent.
- Sett's Series 2 [Eternals](./Eternals.md) make the following references:
  - MUDAMUDA! references Dio's eponymous [https://knowyourmeme.com/memes/stand-cries-ora-ora-ora-muda-muda-muda stand cry] from Jojo's Bizarre Adventure.

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see [Sett (Collection)](./Sett_Cosmetics.md)._

==Patch history==

==Trivia==
```
</details>
