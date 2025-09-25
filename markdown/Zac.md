# Zac

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Map-specific Stats](#map-specific-stats)
- [Pets](#pets)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|------:|
| **Champion** | Zac |
| **Title** | the Secret Weapon |
| **Resource** | Health |
| **Range Type** | Melee |
| **Release Date** | 2013-03-29 |
| **Release Patch** | V3.5 |
| **Latest Changes** | V25.18 |
| **Roles** | Vanguard |
| **Riot Positions** | Jungle |
| **External Positions** | Top, Jungle, Support |
| **Blue Essence** | 2400 |
| **Riot Points** | 880 |
| **Difficulty** | 1 |
| **Hero Type** | Tank |
| **Alt Type** | Fighter |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 3 |
| **Control** | 3 |
| **Mobility** | 2 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $685.0$ | $+109.0$ |
| **Mana** | $0.0$ | $+0.0$ |
| **Health Regen** | $5.0$ | $+0.5$ |
| **Armor** | $33.0$ | $+4.7$ |
| **Magic Resist** | $32.0$ | $+2.05$ |
| **Attack Damage** | $60.0$ | $+3.4$ |
| **Attack Speed** | $0.736$ | |
| **Movement Speed** | $340.0$ | $+0.0$ |
| **Attack Range** | $175.0$ | $+0.0$ |
| **Base Attack Speed** | $0.736$ | |
| **Attack Speed Ratio** | $0.638$ | |
| **Bonus AS per Level** | $1.6\%$ | |
| **Attack Windup** | $23.2\%$ | |
| **Acquisition Radius** | $400$ units | |
| **Gameplay Radius** | $80$ units | |
| **Pathing Radius** | $43.075$ units | |
| **Selection Radius** | $140$ units | |
| **Selection Height** | $100$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Taken** | $96.0\%$ |
| **Healing** | $110.0\%$ |

## Pets

### Bloblet

| Attribute | Value |
|-----------|------:|
| **Gold** | 0 |
| **Experience** | 0 |
| **Health** | 12% (Plus an additional 1 health) of **Zac**’s **maximum** health |
| **Armor** | 50% of **Zac**’s armor |
| **Magic Resist** | 50% of **Zac**’s magic resistance |
| **CC Resist** | Immune to crowd control (except stasis, still able to move) |
| **Move Speed** | Variable |
| **Control** | Autonomous |
| **Targeting** | Minion
- *Bloblets* can be targeted by Smite and Force of Will.
  - A grabbed *Bloblet*, while visually separated from the others, will not delay **Zac**’s revival or cause him to recombine with less **current** health.
- *Bloblets* can be targeted by Teleport.
- Taste Their Fear considers *Bloblets* 'isolated' when being the last one alive or after having merged with the others right before **Zac** recombines (A *Bloblet* is not considered 'isolated' when close enough to the others)
- Turrets will prioritize *Bloblets* over other minions. |

---

## Abilities

### Passive: Cell Division

| Attribute | Value |
|-----------|------:|
| **Range** | 700 (Maximum straight-line bloblet split range, estimated. Bloblets will not go over terrain if their pathfinding length exceeds this range.) units |
| **Static Cooldown** | 300 |
| **Targeting** | Passive |
| **Affects** | Self |

**INNATE - THE SECRET WEAPON:** **Zac** gains increased percentage size equal to $1.5$% of his **bonus** health, capped at 35% increased at $ **bonus** health. **Zac**’s also reduces based on his **missing** health, down to 70% **total** while at 0% health.

**INNATE - GOO:** Whenever **Zac** damages at least one enemy with an ability, he sheds a number of chunks of himself to a nearby location, landing towards nearby visible enemy champions and lasting for 6 seconds.
- *Stretching Strikes*: 1 chunk for each strike hit (excluding the collision) on different targets, up to 2
- *Unstable Matter*: 1 chunk if it hits at least one target
- *Elastic Slingshot*: 1 chunk for each enemy champion hit
- *Let's Bounce!*: 1 chunk for each bounce that hits an enemy champion, up to 4

Both **Zac** and enemy champions can interact with a chunk by being within 50 units of it: an enemy will destroy it, while **Zac** will consume it to heal for 4 to 8 of his **maximum** health.

**INNATE - CELL DIVISION:** Periodically, upon taking fatal damage, **Zac** enters resurrection for 8@1; 7@6; 6@10; 5@13; 4@17 seconds, instantly restoring 50% of his **maximum** health and becoming untargetable and unable to act for the duration. **Zac** then splits into four uncontrollable *[bloblets](#Pets)* away from his location, though stopping at terrain. After $1.5$ seconds of the *bloblets* being untargetable, they start moving toward **Zac**’s location over the remaining duration (Regardless of distance). Any damage the *bloblets* receive, excluding damage dealt in excess of each *bloblet*’s **total maximum** health, is also redirected to **Zac** as true damage. While in resurrection, **Zac** cannot take damage from sources other than the redirected damage from his *bloblets*.

After the duration, **Zac** is revived with 10 – 50% (Based on his bloblets' total remaining health. Reaches the maximum amount at full health on all bloblets) **maximum** health. **Zac** will die once all bloblets are killed.

*See [Pets](#Pets) for stats and details for bloblets.*

**Notes:**

- The damage directed to **Zac** is internal raw damage.
- **Zac**’s health when the *bloblets* are sent out is set to what it would be if all *bloblets* recombined, and is actively updated when the *bloblets* take damage.
- **Zac**’s increased does not consider his base stats.
- *Cell Division*’s *chunks* spawn in two versions:
  - **CONTESTABLE:** They will fall to the ground at a point which is roughly equidistant from **Zac** and the nearest visible enemy champion within 1000 range of him (dropping slightly in **Zac**’s favor). The location at which they fall to the ground is influenced by the direction in which **Zac** is currently facing. Cannot be interacted with for the first $0.25$ seconds (except during Let's Bounce!).
  - **UNCONTESTABLE:** They will fall to the ground near **Zac** and can be picked up immediately (version used by default).
- **Zac**’s untargetability upon triggering *Cell Division* does not destroy in-flight projectiles.
- Chronoshift and Guardian Angel will take priority over *Cell Division*.
- Spirit Visage and Heal Power increases the healing of both *chunks* and *bloblets*.
- Ryze’s Realm Warp is able to individually teleport **Zac** or his *bloblets* during *Cell Division*.
  - If **Zac** is transported to a new location before the *bloblets* landed, they will travel to form at his new position, adjusting their speed accordingly. If *Cell Division*’s duration permits during this time, **Zac** will be revived even if the *bloblets* fail to physically recombine.
- *Cell Division*’s *bloblets* can be targeted by Teleport and Syndra’s Force of Will.
  - Other *bloblets* are not interrupted by the conditions above and will continue to recombine instinctively.
- If **Zac**’s health is higher than it is supposed to be when the *bloblets* converge on him, his health will be set to the correct value.
- The following table refers for interactions while **Zac** is in resurrection:

---

### Q: Stretching Strikes

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.33$ (Cast time for the activation) seconds |
| **Target Range** | 800 (Initial stretch missile range) / er 80 (Lollipop at end) units |
| **Tether Radius** | 900 units |
| **Width** | 160 (Initial stretch missile width) units |
| **Speed** | 2800 (Initial stretch missile speed) units/second |
| **Cost** | 8% **Current** Health |
| **Cooldown** | 14 / 12.5 / 11 / 9.5 / 8 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.3$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Single target |
| **Projectile** | True |
| **Parry** | True |

**ACTIVE:** **Zac** stretches his left arm in the target direction that catches the first enemy hit, dealing magic damage, slowing them by 40% for $0.5$ seconds, and forming a tether between **Zac** and the target for 2 seconds, during which they are revealed.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 40 / 55 / 70 / 85 / 100 (+ 30% AP) (+ 3% of **Zac**’s **maximum** health) |
| **Total Magic Damage** | 80 / 110 / 140 / 170 / 200 (+ 60% AP) (+ 6% of **Zac**’s **maximum** health) |

While the tether persists, **Zac**’s next basic attack is replaced by a second *Stretching Strike*, empowering it to have a $0.25$-second cast time and gain 25 **bonus** attack range. This attack cannot critically strike.

If the two *Stretching Strikes* affect different targets, both are rooted for $0.5$ (Estimated) seconds while the secondary target (Target hit by the empowered attack (or second Stretching Strike in this context)) is dealt the initial magic damage and applied the same slow. After a $0.4$-second delay (After the attack's cast time), **Zac** displaces (Only move block, does not stun the target's actions (disables attacking and movement only)) them toward each other over 300 units, though not through terrain.

If the two targets are within 700 units of each other, they are instead slammed together through the displacement. Upon impact of each other, both targets are knocked up and stunned for $0.25$ seconds and all enemies within the impact are dealt the initial magic damage and applied the same slow.

*Both Stretching Strikes (the cast and the empowered attack) reset **Zac**’s basic attack timer. **Zac** is unable to move or attack while his left arm is in flight.*

**Notes:**

- If both *Stretching Strikes* affect the same target, the target will be dealt the damage and applied the slow for both hits.
- The following features were added in patch V7.10 to increase *Stretching Strikes** reliability and usability:
  - If the second attack's target would die from the damage, it won't be dealt until they are slammed to the first target.
  - If the first target is standing on top of other enemies, the second hit will reliably choose a different target from the first.
  - Non-champion units that are affected by *Stretching Strikes** displacement are prevented from taking any damage until they are slammed together. Note that this often confuses players who are trying to secure monsters via Smite while **Zac** is slamming the wanted target, resulting in *Smite* dealing no damage to it. It is currently unknown whatever the effects of this feature are intentional or not.
- Spell shield's interactions with *Stretching Strikes*:
  - The first *Stretching Strike* will have its damage, slow, and tether application all blocked.
  - If as the primary target and affected by the tether already from the first *Stretching Strike*:
    - The damage and slow from the empowered attack if targeted again by the second *Stretching Strike* will be blocked.
    - The damage, slow, stun, and knock up from the impact of the second *Stretching Strike* hitting a different target will all be blocked, but not the root and the displacement.
  - If as the primary target of the second *Stretching Strike*, the root will be blocked, but the displacement, damage, slow, stun, and knock up afterwards will not.
  - If as the secondary target of the impact, the damage and slow will be blocked.
- The second *Stretching Strike* does not trigger when attacking Illaoi’s tentacles, Gangplank’s kegs.
  - Does it trigger against blind, dodge, block, stealthed trap, farsight ward?
- If the tethered target is untargetable during the second *Stretching Strike* via basic attack, they are completely unaffected by the ability.
- Displacement immunity will also resist the application of the stun.
- This ability will cast from wherever the caster is at the end of the cast time.
- If the target becomes untargetable, dies, or is too far away during the empowered attack's cast time, it is cancelled but not consumed.

#### Channel Behavior (cast)

| Aspect | State / Notes |
|--------|---------------|
| **Summoner Spells** | Allowed / Disabled |

---

### W: Unstable Matter

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 350 units |
| **Cost** | 4% **current** Health |
| **Cooldown** | 5 seconds |
| **Queue Time** | $0.1$ seconds |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |

**ACTIVE:** **Zac** explodes to deal magic damage to nearby enemies, capped against minions and monsters.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 40 / 50 / 60 / 70 / 80 (+ 4 / 5 / 6 / 7 / 8% (+ 3% per 100 AP) of target's **maximum** health) |

| Attribute | Value |
|-----------|------:|
| **Capped Non-Champion Damage** | 240 / 250 / 260 / 270 / 280 |

**Zac** grants ghosting to aggroed monsters hit for 5 seconds.

*Unstable Matter*’s **current** cooldown is reduced by 1 second whenever **Zac** collects a *Cell Division* chunk.

**Notes:**

- If *Unstable Matter* is the cause of monster aggro, the monsters will not receive ghosting.
  - Subsequent uses of *Unstable Matter* within 5 seconds after the previous will not grant ghosting either.

---

### E: Elastic Slingshot

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 1200 / 1350 / 1500 / 1650 / 1800 units |
| **Effect Radius** | 250 units |
| **Speed** | 1350 (Dash speed) units/second |
| **Cost** | 4% **Current** Health |
| **Cooldown** | 22 / 19 / 16 / 13 / 10 seconds |
| **Cooldown Start** | post-effect |
| **Queue Time** | $0.3$ seconds |
| **Targeting** | Direction / Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Out of Range Behavior** | Target at maximum range (recast clamped) |
| **Grounded** | True |
| **Knockdown** | True |
| **Silence** | True |

**ACTIVE:** **Zac** charges for up to $4.5$ seconds to increase *Elastic Slingshot*’s range over a cone in the target direction.

| Attribute | Value |
|-----------|------:|
| **Maximum Range Channel Duration** | 0.9 / 1 / 1.1 / 1.2 / 1.3 seconds |

*Elastic Slingshot* can be recast within the duration. If **Zac** cancels (See notes) the channel himself, or the charge completes without reactivation, 50% of *Elastic Slingshot*’s health cost and cooldown are refunded.

**RECAST:** **Zac** leaps to the target location within the boundaries of the cone. Upon landing, he deals magic damage to nearby enemies and knocks them up and stuns them for $0.5$ seconds, increased to 1 second if *Elastic Slingshot* was charged for more than 1 second.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 105 / 150 / 195 / 240 (+ 80% AP) |

**Notes:**

- **QUICK CASTING:** *Elastic Slingshot*’s channel begins when the key is pressed and held, launching **Zac** towards the cursor when released.
- *Elastic Slingshot* will not knock up if the targeted location was directly on top of **Zac** (in which case he will not dash at all).
  - The knock up will occasionally end early if the targeted location was slightly away from **Zac**.
- Displacement immunity will also resist the application of the stun.
- The following table refers for interactions while **Zac** is channeling:
  - Recall is disabled for the first $0.9$ seconds and otherwise interrupts the channel if used.

#### Channel Behavior (charge)

| Aspect | State / Notes |
|--------|---------------|
| **Attacking** | Disabled |
| **Movement** | Interrupts |
| **Abilities** | Stretching Strikes and Unstable Matter interrupt. Let's Bounce! initiates the recast and is buffered to cast itself at the end of the dash. |
| **Items** | Interrupts / Allowed / Disabled |
| **Summoner Spells** | Allowed / Disabled / Interrupts / Recasts |
| **Interrupted by** | silence, death, root, ground |
| **Notes** | Disabled for the first $0.9$ seconds otherwise |

---

### R: Let's Bounce!

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.3$ seconds |
| **Effect Radius** | 300 (Damage and knockback radius) units |
| **Cooldown** | 120 / 112.5 / 105 / 97.5 / 90 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.3$ seconds |
| **Targeting** | Auto |
| **Affects** | Enemies, Self |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Area of effect |

**ACTIVE:** **Zac** bounces after the cast time, then bounces 3 additional times each second over 3 seconds. Each bounce deals magic damage to enemies hit, knocks them back over 1 second, and slows them by 20% for the same duration. An enemy can be hit by multiple bounces, but ones beyond the first deal 50% damage to them and do not apply the knock back.

| Attribute | Value |
|-----------|------:|
| **Primary Magic Damage** | 140 / 175 / 210 / 245 / 280 (+ 40% AP) |
| **Reduced Magic Damage** | 70 / 87.5 / 105 / 122.5 / 140 (+ 20% AP) |

| Attribute | Value |
|-----------|------:|
| **Total Magic Damage** | 350 / 437.5 / 525 / 612.5 / 700 (+ 100% AP) |

**Zac** can move during *Let's Bounce!*, gaining ghosting and (ms) 20%–50%@0–3 (@=duration) **bonus** movement speed, but becomes unable to declare basic attacks, *Stretching Strikes*, and *Elastic Slingshot*.

**Notes:**

- Spell shield only blocks a single instance of damage.
- Entering stasis will end *Let's Bounce!* prematurely.
- While *Let's Bounce!* is active **Zac** consumes *Chunks* within cr 300 range.
- The following table refers for interactions while **Zac** is performing *Let's Bounce!*: and are disabled. is usable.|items=true|consume=true|spells=true,true,true,false,true|interrupts=death

---

## Patch History

### V25.18
- Let's Bounce!
  - **New Effect:** Now grants ghosting during the effect.

### V25.16
- Let's Bounce!
  - **UNDOCUMENTED / BUG FIX:** Tooltip damage calculations now properly take into account his ability power (instead of incorrectly adding the ability power ratio values alone).

### V25.13
- Cell Division
  - **Bug Fixes:** Dying to Nexus Obelisk damage no longer triggers *Cell Division*’s resurrection.

### V14.23
- Stretching Strikes
  - Health ratio reduced to 3% of his **maximum** health from 4%.

### V14.13
- Cell Division
  - Heal changed to 4 to 8 **maximum** health from pp|type

### V14.9
- Stats
  - Selection radius increased to 140 units from 90.

### V14.8
- Stats
  - Base health regeneration reduced to 5 from 8.

### V14.5
- Unstable Matter
  - Base damage changed to 40 / 50 / 60 / 70 / 80 from 35 / 50 / 65 / 80 / 95.

### V14.4
- Stretching Strikes
  - Tether range reduced to 900 units from 1000.
  - Tether range type changed to edge from center.
  - Tether now innately checks every $0.05$ seconds for valid range conditions, and no longer waits for every 8th server tick.

### V14.2
- Elastic Slingshot
  - **Bug Fixes:** No longer has an error in its internal cast range values that sometimes caused it to cast backwards at rank 5 of the ability while Clamp Cast Targeting was enabled.

## Trivia

- Zac stands for **Z**aun **A**morphous **C**ombatant.
- Zac made a cameo in Riven artwork and maybe even in the game's Mac Version trailer (all before he was released).
- Zac is the only champion that can be damaged by regular Smite (due to Cell Division’s chunks being coded as minions) and to have 2 laugh animations.
- Let's Bounce! references Tigger from Winnie-the-Pooh (book) by A. A. Milne.
- The pitch level of his voice changes based on how much health he currently has.
- Zac is one of the champions who use health as a resource for their abilities, the other five being Briar, Dr. Mundo, Olaf, Soraka, and Vladimir.

---
*This page was automatically generated from League of Legends Wiki data.*