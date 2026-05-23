# Smolder

## Overview

- **Title:** Smolder
- **Resource:** Mana

## Stats

| Stat | Base | Growth |
|------|------|--------|
| Armor | 24 | 4.7 |
| Attack Damage | 60 | 2.3 |
| Attack Speed | 0.638 | 4 |
| HP | 575 | 100 |
| HP Regen | 3.75 | 0.6 |
| MP | 300 | 40 |
| MP Regen | 8.5 | 0.7 |
| Magic Resist | 30 | 1.3 |
| Move Speed | 330 | 0 |
| Range | 550 | 0 |

## Advanced Stats

| Metric | Value |
|--------|-------|
| Acquisition Radius | 900 |
| Attack Delay Offset (s) | -0.1338 |
| Attack Speed Ratio | 0.638 |
| Missile Speed | 1800 |
| Pathing Radius | 30 |
| Selection Height | 150 |
| Selection Radius | 130 |

## Abilities

### W – Smolder Achooo!

| Attribute | Value |
|-----------|------:|
| **Range** | 1500 |
| **Cast Time** | 0.35 |
| **Effect Radius** | 385 (Explosion) |
| **Width** | 115 (Sneeze) |
| **Speed** | 2000 (Initial speed) / 400 (End speed) |
| **Cost** | (+50 to 70% AP) |
| **Cost Type** | mana |
| **Cooldown** | (+14 to 10% AP) |

**ACTIVE:** **Smolder** sneezes a fiery glob in the target direction that deals physical damage to enemies hit and slows them by 35% for 1.50 seconds, slowing down in missile speed after travelling 1200 (estimated) units. Hitting an enemy champion creates an explosion that deals physical damage to nearby enemies, with subsequent explosions against the same target dealing 75% damage of the previous explosion's damage.

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | physical |
| **Spell Effects** | spellaoe |
| **Spell Shield** | special |
| **Projectile** | true |

**Notes:**

- **Smolder** can't declare basic attacks for a brief moment after the cast time.
- Spell shield can block either the glob or the explosion, but not both. <!-- Blurb -->

### E – Smolder Flap, Flap, Flap

| Attribute | Value |
|-----------|------:|
| **Range** | 700 |
| **Cast Time** | none |
| **Speed** | 1800 (Bolt speed) |
| **Cost** | 65 |
| **Cost Type** | mana |
| **Cooldown** | (+24 to 16% AP) |
| **Cooldown Start** | on-cast |

**ACTIVE:** **Smolder** takes flight for 1.25 seconds, becoming ghosted and gaining 75% **bonus** movement speed, unobstructed vision, and the ability to ignore terrain collision. While in flight, he automatically fires up to 5 (+ 1 per 100 _Smolder_ stacks) bolts at the nearest and most wounded (Lowest health percent) visible enemy, dealing physical damage with each hit and prioritizing enemy champions.

_Flap, Flap, Flap_ ends immediately if **Smolder** casts another ability or becomes immobilized.

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | physical |
| **Spell Effects** | spell |
| **Spell Shield** | special |
| **Parry** | false |
| **Grounded** | true |

**Notes:**

- **Smolder's** attack range is reduced to 0 for the duration of _Flap, Flap, Flap_.
  - Based on game engine ticks **Smolder** may not be able to declare basic attacks for up to  after the end of the effect.
- The number of additional bolts based on Smolder stacks is rounded down.
- **Smolder** reveals himself while attacking enemies, even inside terrain.
- Spell shield will block only one bolt.
- Self immobilizations such as Zhonya's Hourglass also count for ending _Flap, Flap, Flap_.
- Recall is disabled while **Smolder** is inside terrain.
- _Flap, Flap, Flap_ can interact with [player-generated terrain](./Terrain.md#Player-Generated).
- If **Smolder** is inside terrain when the effect ends, he will be placed correspondingly to the nearest valid space.
- The following table refers for interactions while **Smolder** is flying: (attack=Causes **Smolder** to move towards the attack target.) <!-- Blurb -->

### R – Smolder MMOOOMMMM!

| Attribute | Value |
|-----------|------:|
| **Range** | 4250 / -600 (Backwards range) |
| **Cast Time** | 0.75 |
| **Effect Radius** | 1000 (Vision radius) |
| **Width** | 125 (Sweetspot) |
| **Speed** | 1700 |
| **Cost** | 100 |
| **Cost Type** | mana |
| **Cooldown** | (+120 to 100% AP) |

**ACTIVE:** **Smolder** shouts for his mother to attack from above, causing her to exhale a wave of fire from behind him and towards the target direction, granting sight of its surroundings (Cannot grant sight through terrain and can only grant sight into brush when the center part goes through that brush) as it travels. The wave heals **Smolder** and deals physical damage to enemies hit, with those in the center taking  and becoming slowed by 40% for 2 seconds.

_MMOOOMMMM!_ deals 50% damage against minions and monsters.

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies, Self |
| **Damage Type** | physical |
| **Spell Effects** | spellaoe |
| **Spell Shield** | true |
| **Projectile** | true |

**Notes:**

- (cast) <!-- Blurb -->

## Trivia

  - In Smolder's case, Dragon Practice stacks gained through Smolder permanently increase the bonus damage of his basic abilities, Smolder bolt amount, and Smolder bolt amount and burn damage as it upgrades.
- Smolder's joke animation references a popular meme of the character Toothless from _How to Train Your Dragon_ series being depicted dancing.<ref>Know Your Meme Ã¢ÂÂ [https://knowyourmeme.com/memes/dancing-toothless Dancing Toothless]</ref>
- Smolder's dance animation references the 1988 movie Oliver and Company. Striking a similarity to [https://tenor.com/en-GB/view/oliver-and-company-oliver-dancing-cat-groovy-gif-5000573 Oliver's dance].

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

== Champion skins ==
:_This article section only contains champion skins. For all associated collection items, see [Smolder (Collection)](./Smolder_Cosmetics.md)._

==Trivia==
* 
** In Smolder's case, Dragon Practice stacks gained through Smolder permanently increase the bonus damage of his basic abilities, Smolder bolt amount, and Smolder bolt amount and burn damage as it upgrades.
* Smolder's joke animation references a popular meme of the character Toothless from _How to Train Your Dragon_ series being depicted dancing.<ref>Know Your Meme Ã¢ÂÂ [https://knowyourmeme.com/memes/dancing-toothless Dancing Toothless]</ref>
```
</details>
