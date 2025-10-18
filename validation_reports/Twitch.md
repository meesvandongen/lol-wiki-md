# Twitch

## Overview

- **Title:** Twitch
- **Resource:** Unknown

## Stats

| Stat | Base | Growth |
|------|------|--------|

## Abilities

### Passive – false

**INNATE:** **Twitch's** basic attacks on-hit apply a stack of _Deadly Venom_ for 6 seconds, refreshing on subsequent applications and stacking up to 6 times.

**DEADLY VENOM:** For each stack, the target is dealtÃ£ÂÂ 1 to 5 for 5 / 1 to 17 / formula=1 every 4 levels (+ 3% AP) true damage per second over the duration, Ã¢ÂÂ· 1*6 to 5*6 for 5 / 1 to 17 / formula=1 every 4 levels (+ 18% AP) **total** true damage over the duration, Ã£ÂÂfor a maximum ofÃ£ÂÂ 1*6 to 5*6 for 5 / 1 to 17 / formula=6 every 4 levels (+ 18% AP) true damage with each tick. Ã¢ÂÂ· 1*6*6 to 5*6*6 for 5 / 1 to 17 / formula=6 every 4 levels (+ 108% AP) **total** true damage over the duration. Ã£ÂÂThis effect is considered a poison.

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | True |
| **Spell Effects** | default |
| **Spell Shield** | False |
| **Parry** | special |

**Notes:**

- The first 5 stacks on a target are indicated each by a small mark around them, while a target affected by the maximum stacks of 6 is indicated by a single large mark above them instead.
- _Deadly Venom_ does not affect structures.
- Being applied on-hit, _Deadly Venom_ stacks will still be applied if the attack was Fiora or blocked, but not if dodged and/or missed if **Twitch** is blinded.

### Q – Ambush

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Cost** | 40 |
| **Cost Type** | Mana |
| **Cooldown** | 16 |
| **Cooldown Start** | post-effect |
| **Detection Radius** | 500 |

**ACTIVE:** After a 1-second delay, **Twitch** becomes camouflaged for a duration. Attacking or casting _Twitch_ or _Twitch_ ends _Ambush_ immediately.

During this time, **Twitch** gains 10% **bonus** movement speed, increased to 30% while facing enemy champions within a 1000-unit radius who cannot see him.

Upon breaking stealth, **Twitch** gains **bonus** attack speed for 6 seconds.

When an enemy champion dies while afflicted with _Twitch_, _Ambush's_ cooldown is reset.

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self |

**Notes:**

- Entering stealth cancels **Twitch's** current basic attack.
- _Ambush_ follows the same rules as stealth but he can still perform actions normally before entering camouflage. Activating Recall during the 1-second delay allows him to channel it while stealthed.
- If **Twitch** enters stasis during the delay, he will gain the camouflage after the stasis ends.
- Using a basic attack breaks the stealth at the start of the attack windup.

### W – Venom Cask

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Target Range** | 950 |
| **Effect Radius** | 300 |
| **Speed** | 1400 |
| **Cost** | 70 |
| **Cost Type** | Mana |
| **Cooldown** | (+13 to 9% AP) |

**ACTIVE:** **Twitch** hurls a cask of venom that explodes at the target location, applying _Twitch_ to enemies hit and granting sight of the area.

The area then becomes contaminated for 3 seconds, applying a _Twitch_ stack each second to enemies within and slowing them.

| Detail | Value |
|--------|------:|
| **Targeting** | Location |
| **Affects** | Enemies |
| **Out of Range** | walk to location |
| **Spell Shield** | True |
| **Projectile** | True |

**Notes:**

- _Venom Cask_ can apply a maximum of 4 Twitch stacks per enemy per cast.
- _Venom Cask's_ missile will fail to fire if **Twitch** is suppressed during the cast time.

### E – Contaminate

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Effect Radius** | 1200 |
| **Cost** | (+50 to 90% AP) |
| **Cost Type** | Mana |
| **Cooldown** | (+12 to 8% AP) |

**ACTIVE:** **Twitch** sends out a lethal toxin to each nearby enemy afflicted by _Twitch_, dealing them physical damage.

_Contaminate_ deals additional physical damage and 35% AP magic damage for each stack of _Twitch_ on the target.

_A nearby enemy with Twitch is required to cast this ability. The target does not have to be visible to be targeted by this ability._

| Detail | Value |
|--------|------:|
| **Targeting** | Proximity |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Spell Effects** | Aoe |
| **Spell Shield** | True |
| **Projectile** | false |

**Notes:**

- _Contaminate_ will deal the additional damage to targets based on the number of Twitch stacks they had at the start of the cast time.
- **Twitch** is given a range indicator for _Contaminate's_ radius upon infecting an enemy champion with Twitch (actual range is slightly larger than shown by the indicator).
- _Contaminate_ will not deal damage to enemies that are not within range of the ability before the cast time completes.
  - If the target moves out of range after the cast time, they are still dealt the damage.

### R – Spray and Pray

| Attribute | Value |
|-----------|------:|
| **Range** | 1100 (bolt travel distance, further increased by Rapid Firecannon) |
| **Cast Time** | none |
| **Target Range** | Twitch's attack range |
| **Width** | 120 (bolt width) |
| **Speed** | 5000 |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | 90 |
| **Cooldown Start** | on-cast |

**ACTIVE:** **Twitch** gains **bonus** attack damage and 300 **bonus** attack range for 6 seconds, during which his basic attacks are replaced by _bolts_ that travel slightly further than his attack range in a straight line, dealing damage to every enemy unit (non-champions and champions, wards and plants, turrets and structures) hit.

The _bolts_ deal key= / type=enemies hit / 110-10*x / 0 to 4 for 5 / formula=100% - 10% per enemy hit. _This is capped at 60% damage._ / changedisplay=true of the triggering attack's damage, apply on-hit effects, and can critically strike for total critical damage damage.

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Physical |
| **Spell Effects** | basic |
| **Parry** | True |
| **Projectile** | true |
| **Call For Help** | True |

**Notes:**

- The extra distance that the _bolts_ travel scales with **bonus** attack range.
- _Spray and Pray_ allows **Twitch** to outrange turrets by 50 units, allowing him to hit them without them returning fire.
- **Twitch** targets his line attack at his target's location at the _start_ of his attack windup.
  - The _bolts_ reach targets at a delay, composed of **Twitch's** windup time and distance the _bolt_ has to travel at finite speed. Even the primary target can dodge the attack if they are moving quickly enough.
- The _bolt_ projectiles will naturally pass through terrain and enemy structures (the latter will be damaged by _bolts_ even if they are not directly targeted).
- If **Twitch** is blinded before winding up the attack, the hits will _miss_ against **all** targets.
- Runaan's Hurricane Wind's Fury interacts with _Spray and Pray's_ **bonus** attack range but not with the modified missile effect (the secondary bolts will not have pass-through effects).
- Whenever the _bolts_ penetrate a target, a small elongated cloud appears at the location, which's VFX and SFX **can** be seen and heard inside the Fog of War.
- The _bolts<nowiki>'</nowiki>_ travel distance scales with the full value of **Twitch's** attack range increases such as Rapid Firecannon, but not with size increases (which only increase his effective attack range, thereby the target range).
- (Outdated as of V10.13, now can hit everything with edge range, except for turrets) The  center of a unit must be within the maximum travel distance of the _bolt_ missile, and in front of the spawn location (**Twitch's**  center) for the _bolt_ to be able to hit them.
  - Other than this condition, the _bolt_ missile has to only touch (pass within its half width of 60 units) the  edge of the unit's radius.
  - This is standard behaviour for _linear skillshots_.
- Malignance Hatefog is special cased to work with _Spray and Pray_.
- Axiom Arcanist amplifies _bolt_ damage as area of effect.<!--intended feature lmao-->

## Trivia

- Twitch was voiced by the late Doug Boyd.
  - Twitch is voiced by an unknown voice actor in the [https://www.youtube.com/watch?v=N0Uhti1lqgc Wild Rift Chat shorts].
- Twitch displaying an X when fully stacked on a target might be referencing the Black Death (the doors of those afflicted were marked as such).

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see Twitch_(Collection)._

==Patch history==

==Trivia==
```
</details>
