# Hwei

## Overview

- **Title:** Hwei
- **Resource:** Mana

## Stats

| Stat | Base | Growth |
|------|------|--------|
| Armor | 21 | 4.7 |
| Attack Damage | 54 | 3.3 |
| Attack Speed | 0.69 | 2.5 |
| HP | 580 | 109 |
| HP Regen | 5.5 | 0.55 |
| MP | 480 | 30 |
| MP Regen | 7.5 | 0.75 |
| Magic Resist | 30 | 1.3 |
| Move Speed | 330 | 0 |
| Range | 550 | 0 |

## Advanced Stats

| Metric | Value |
|--------|-------|
| Acquisition Radius | 600 |
| Attack Cast Time (s) | 0.28 |
| Attack Speed Ratio | 0.658 |
| Base Attack Time (s) | 1.495 |
| Missile Speed | 2800 |
| Pathing Radius | 35 |
| Selection Height | 180 |
| Selection Radius | 135 |
| Windup % | 18.7% |

## Abilities

### Passive – Hwei Signature of the Visionary

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 285 (Center of marked champion to edge of other units) |

**INNATE:** **Hwei's** damaging [abilities](./champion_ability.md) mark enemies hit for 4 seconds. Subsequent damaging abilities against marked targets consume the mark to create an explosion beneath them, dealing 35 to 230 (+ 35% AP) **bonus** magic damage to enemies in the area after a 0.85-second delay.

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | aoe |
| **Spell Shield** | true |

**Notes:**

- The ability that consumes the mark cannot apply it on the same cast instance.
- The mark **cannot** be triggered from the same cast instance of an ability, even if the triggering ability was empowered by _Hwei_.
- The explosion occurs around the marked target from where they were when the ability damaged them to consume the mark.
  - In other words, the explosion is at the location of where they were hit, not where they are at the end of the delay.
    - The target who had their mark consumed is able to escape the area of the explosion within the delay period.
- Enemies can be damaged by multiple explosions at once.
- Spell shield will block both the mark and its consumption as well as the detonation.
- The indicator for the effect telegraphs an unusually smaller radius than it actually hits. <!-- Blurb -->

### R – R2

| Attribute | Value |
|-----------|------:|
| **Range** | 1340 (1300 missile travel distance plus 40 to-edge radius check if nothing has been hit yet) |
| **Cast Time** | none |
| **Effect Radius** | 500 (Detonation radius) |
| **Width** | 180 (Missile full width) |
| **Speed** | 1400 |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | (+140/115/80% AP) |
| **Queue Time** | 0.25 |

**ACTIVE:** **Hwei** launches a globule of pure despair in the target direction that collides with the first enemy champion hit, afflicting them with an [aura](./aura.md) that grows over 3 seconds, reveals the target, and grants sight within its radius. Enemies within are both dealt magic damage and applied a stack of _Despair_ every 0.25 seconds.

**DESPAIR:** For each stack, the target is slowed by 10% for 0.25 seconds, stacking up to 12 times.

At the end of the duration or when the target dies, the aura explodes to deal magic damage to enemies within and remove all _Despair_ stacks from affected enemies.

_Spiraling Despair can only be cast if **Hwei** has not entered a mood._

| Detail | Value |
|--------|------:|
| **Targeting** | Auto |
| **Affects** | Self |
| **Damage Type** | Magic |
| **Spell Effects** | special |
| **Spell Shield** | special |
| **Projectile** | true |

**Notes:**

- <!-- Blurb -->
- Applies persistent area damage for the aura and deals area damage for the explosion.
- Spell shield can block the aura, explosion, and the application of the first _Despair_ stack. <!-- Blurb -->

## Trivia

- **Hwei** is currently the champion with the largest number of abilities in League of Legends: 11 when excluding his subject selection spells, and 15 when including them.
- **Hwei's** kit obeys the rule of thirds in multiple aspects:
  - He has 3x3 basic abilities.
  - His passive requires two damaging abilities in order to deal a third damaging ability.
  - He studies two negative subjects and one positive subject.
- Hwei, **Hwei's** positive mood, is placed between his other two negative moods, Hwei and Hwei. This fact stems from lore implications, namely his inner turmoil, as detailed in his universe short stories: "Hwei faces the conflicting hues of Ionia" in [The Visionary](./The_Visionary.md) and "Art saves me, yet it can shatter me" in [Paintings Framed in Half-Light](./Paintings_Framed_in_Half-Light.md).
- When **Hwei** dies, his eyes, which reflect his current mood, fade into a colorless blankness, symbolizing the absence of any emotional presence in death.
- **Hwei's** name comes from Chinese, being the character **Ã¥Â½Â** (HuÃÂ¬), meaning "comet".

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

== Champion skins ==
:_This article section only contains champion skins. For all associated collection items, see [Hwei (Collection)](./Hwei_Cosmetics.md)._

== Trivia ==
* **Hwei** is currently the champion with the largest number of abilities in League of Legends: 11 when excluding his subject selection spells, and 15 when including them.
* **Hwei's** kit obeys the rule of thirds in multiple aspects:
** He has 3x3 basic abilities.
```
</details>
