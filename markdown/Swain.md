# Swain

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
| **Champion** | Swain |
| **Title** | the Noxian Grand General |
| **Resource** | Mana |
| **Range Type** | Ranged |
| **Release Date** | 2010-10-05 |
| **Release Patch** | V1.0.0.102 |
| **Latest Changes** | V25.12 |
| **Roles** | Battlemage |
| **Riot Positions** | Middle, Support |
| **External Positions** | Middle, Support, Bottom |
| **Blue Essence** | 1575 |
| **Riot Points** | 790 |
| **Difficulty** | 2 |
| **Hero Type** | Mage |
| **Alt Type** | Support |
| **Adaptive Type** | Magic |
| **Damage** | 2 |
| **Toughness** | 2 |
| **Control** | 2 |
| **Mobility** | 1 |
| **Utility** | 1 |
| **Style** | 100 |

## Statistics

### Base Stats

| Stat | Base | Growth |
|------|-----:|-------:|
| **Health** | $595.0$ | $+99.0$ |
| **Mana** | $400.0$ | $+29.0$ |
| **Health Regen** | $3.0$ | $+0.5$ |
| **Mana Regen** | $10.0$ | $+0.8$ |
| **Armor** | $25.0$ | $+4.7$ |
| **Magic Resist** | $31.0$ | $+1.55$ |
| **Attack Damage** | $58.0$ | $+2.7$ |
| **Attack Speed** | $0.625$ | |
| **Movement Speed** | $330.0$ | $+0.0$ |
| **Attack Range** | $525.0$ | $+0.0$ |
| **Base Attack Speed** | $0.625$ | |
| **Attack Speed Ratio** | $0.625$ | |
| **Bonus AS per Level** | $2.1\%$ | |
| **Missile Speed** | $1800$ units/second | |
| **Acquisition Radius** | $525$ units | |
| **Pathing Radius** | $35$ units | |
| **Selection Radius** | $100$ units | |
| **Selection Height** | $148.889$ units | |
| **Critical Damage** | $175.0\%$ | |

### Map-specific Stats

#### ARAM

| Metric | Value |
|--------|------:|
| **Damage Dealt** | $95.0\%$ |
| **Damage Taken** | $115.0\%$ |
| **Healing** | $80.0\%$ |

## Abilities

### Passive: Ravenous Flock

| Attribute | Value |
|-----------|------:|
| **Effect Radius** | 1100 (Nearby Soul Fragment pickup range) / sight 250 (Returning raven sight radius) units |
| **Speed** | 1200 (Raven missile speed) units/second |
| **Targeting** | Passive |
| **Affects** | Self, Enemies |
| **Projectile** | False |

**INNATE:** Enemy champions that die will leave behind a Soul Fragment for 16 seconds. **Swain**’s ravens will collect nearby dropped Soul Fragments, as well as the Soul Fragments out of champions hit by *Vision of Empire* or *Nevermove*’s recast. Once a raven has collected a Soul Fragment, it will fly towards **Swain** to grant it to him, while also granting sight of its surroundings during the travel.


**SOUL FRAGMENT:** For each stack, **Swain** gains 15 **bonus** health permanently. Upon claiming the fragment, he will heal for 3 to 6 of his **maximum** health.

**Notes:**

- Dropped Soul Fragments can be collected after a 1-second delay.
- The ravens will only be seen by enemies if they have vision of **Swain**.

---

### Q: Death's Hand

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 725 units |
| **Angle** | 32° |
| **Cost** | 40 / 45 / 50 / 55 / 60 Mana |
| **Cooldown** | 7 / 6 / 5 / 4 / 3 seconds |
| **Queue Time** | $0.3$ seconds |
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | Area of effect |
| **Projectile** | False |

**ACTIVE:** **Swain** unleashes five bolts of eldritch power over in a cone in the target direction that deal magic damage to enemies hit. Subsequent bolts against an enemy deal 25% **bonus** damage.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 60 / 90 / 120 / 150 / 180 (+ 45% AP) |

| Attribute | Value |
|-----------|------:|
| **Bonus Damage Per Bolt** | 15 / 22.5 / 30 / 37.5 / 45 (+ 11.25% AP) |
| **Total Damage** | 120 / 180 / 240 / 300 / 360 (+ 90% AP) |

**Notes:**

- *Death's Hand* will still cast and deal damage if **Swain** dies during the cast time.
- This ability will cast from wherever the caster is at the end of the cast time.
- The total damage to a single target will be split evenly among each bolt that hits, meaning that hitting with more bolts will deal less damage per hit (but more overall damage).
- Spell shield will block all bolts.

---

### W: Vision of Empire

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 5500 / 6000 / 6500 / 7000 / 7500 units |
| **Effect Radius** | 325 units |
| **Cost** | 60 / 65 / 70 / 75 / 80 Mana |
| **Cooldown** | 22 / 21 / 20 / 19 / 18 seconds |
| **Queue Time** | $0.3$ seconds |
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | spellaoe |
| **Out of Range Behavior** | walk to location |

**ACTIVE:** **Swain** summons a demonic eye at the target location that grants sight of the area for 2 seconds and explodes after $1.5$ seconds, dealing magic damage to enemies within, halved against minions, and slowing them for $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 70 / 105 / 140 / 175 / 210 (+ 60% AP) |
| **Minion Damage** | 35 / 52.5 / 70 / 87.5 / 105 (+ 30% AP) |

| Attribute | Value |
|-----------|------:|
| **Slow** | 50 / 55 / 60 / 65 / 70% |

Enemy champions hit are also revealed for 6 seconds and have a Soul Fragment collected from them.

**Notes:**

- Enemy champions protected by a spell shield do not have a Soul Fragment collected from them but are revealed anyway.
- *Vision of Empire* will grant Soul Fragments from hitting clones.
- **Swain** will not receive the Soul Fragments if *Vision of Empire* hits enemy champions while he is dead.
- *Vision of Empire* is visible and audible to enemies through the fog of war.
- Hitting an enemy champion with *Vision of Empire* collects nearby Soul Fragments.

---

### E: Nevermove

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Target Range** | 850 units |
| **Effect Radius** | 100 (Detonation radius) / Global (Recast range for rooted targets) |
| **Width** | 180 units |
| **Speed** | 1125 to 1800 / 2000 to 2800 units/second |
| **Cost** | 60 / 65 / 70 / 75 / 80 Mana |
| **Cooldown** | 12 / 11.5 / 11 / 10.5 / 10 seconds |
| **Cooldown Start** | on-cast |
| **Queue Time** | $0.3$ (Initial cast) seconds |
| **Targeting** | Direction / Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | Special |
| **Spell Effects** | Area of effect |
| **Projectile** | True |

**ACTIVE:** **Swain** launches a demonic wave in the target direction.

At maximum range, the wave homes back to **Swain** and detonates upon the first enemy hit, dealing magic damage to nearby enemies and rooting them for $1.5$ seconds, during which they are revealed. *Nevermove* can be recast while champions hit are rooted.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 80 / 120 / 160 / 200 / 240 (+ 70% AP) |

**RECAST:** **Swain** pulls the rooted champions over 290 units and collects a Soul Fragment from each of them. If the targets would be brought past **Swain**, they are instead knocked back the excess distance.

**Notes:**

- *Nevermove*’s missiles have their speed update to accelerate at specific times by the server. The client applies these speed increases after additional specified delays, while the hitbox behaviour itself only uses the timings noted above.
- *Nevermove*’s recast can be used during the cast time of other spells.
- Spell shield will block either the detonation on the first cast or the pull from the recast.
  - If the recast is blocked, the target will also prevent a Soul Fragment being collected from them.
- If the root is not applied to or is removed from all champions, the recast does not become available.
- The pull will interrupt channels at the very beginning, but does not keep the target stunned for the rest of the movement.
  - Channels can be started successfully during the pull, provided the previous crowd control does not prevent them still.

- This ability will cast from wherever the caster is at the start of the cast time.

---

### R: Demonflare

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.25$ seconds |
| **Effect Radius** | 675 units |
| **Width** | 60 (Individual missiles) units |
| **Speed** | 3000 (Shockwave speed) units/second |
| **Static Cooldown** | 8 |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | True |
| **Spell Effects** | spellaoe |
| **Projectile** | True |

**ACTIVE:** **Swain** releases a nova of soulfire that deals magic damage to nearby enemies and slows them by 75%, decaying over $1.5$ seconds.

| Attribute | Value |
|-----------|------:|
| **Magic Damage** | 150 / 200 / 250 / 300 / 350 (+ 50% AP) |

***Swain** can move during Demonflare's cast time.*

**Notes:**

- *Demon Flare* counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering Force Pulse’s passive.
- *Demon Flare* passes through champions. Effects like Yasuo’s Wind Wall and Braum’s Unbreakable will block the portion of the nova they intercept.
- **Swain** can move during *Demonflare*’s cast time.
- *Demonflare* will fire from wherever **Swain** is at the end of the cast time.

---

### R: Demonic Ascension

| Attribute | Value |
|-----------|------:|
| **Cast Time** | $0.5$ seconds |
| **Effect Radius** | 650 units |
| **Cost** | 100 Mana |
| **Cooldown** | 120 (Starts on cast) seconds |
| **Cooldown Start** | on-cast |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Magic |
| **Spell Shield** | False |
| **Spell Effects** | aoedot |
| **Projectile** | False |

**ACTIVE:** **Swain** frees the demon within himself, gaining *Demonic Ascension* and generating 50 Demonic Energy. *Demonic Ascension* is maintained with Demonic Energy, which decays by 5 every $0.5$ seconds, increased to $7.5$ after 5 seconds have elapsed, and is lost once all Demonic Energy is depleted to 0. **Swain** generates 10 Demonic Energy every $0.5$ seconds while draining from at least one enemy champion, and fully restores it to the maximum of 50 upon scoring a champion takedown.

**DEMONIC ASCENSION:** **Swain** is ghosted and drains the lifeforce of nearby enemies, both dealing magic damage and healing himself every $0.5$ seconds per target affected. The heal is reduced by 90% against minions and monsters.

| Attribute | Value |
|-----------|------:|
| **Magic Damage per Tick** | 7.5 / 10 / 12.5 / 15 / 17.5 (+ 2.5% AP) |

| Attribute | Value |
|-----------|------:|
| **Heal per Tick** | 7.5 / 11.25 / 15 / 18.75 / 22.5 (+ 2.5% AP) (+ 0.75% of his **bonus** health) |
| **Reduced Heal per Tick** | 0.75 / 1.125 / 1.5 / 1.875 / 2.25 (+ 0.25% AP) (+ 0.075% of his **bonus** health) |

After 2 seconds, **Swain** gains the ability to cast *Demonflare*, up to 100 times, and does so automatically when *Demonic Ascension* expires.

*The targets do not have to be visible to be hit by this ability.*

***Swain** can move during Demonic Ascension's cast time.*

**Notes:**

- Demonflare will not automatically cast at the end of *Demonic Ascension* if it occurs due to **Swain**’s death.

---

## Patch History

### V25.12
- Nevermove
  - **Bug Fixes:** If the missile hits a target during Fate Sealed’s cast time, Nevermove's displacement is no longer illegally able to return Fate Sealed's caster to their initial cast position after the cast time has completed.

### V25.S1.3
- Ravenous Flock
  - Bonus health per Soul Fragment increased to 15 from 12.
- Nevermove
  - AP ratio increased to 70% AP from 60% AP.

### V14.24
- Stats
  - Base armor reduced to 25 from 26.
  - Armor growth reduced to $4.7$ from $5.2$.

### V14.22
- Stats
  - Base magic resistance increased to 31 from 30.
  - Magic resistance growth increased to $1.55$ from $1.3$.
- Death's Hand
  - Base damage increased to 60 / 90 / 120 / 150 / 180 from 60 / 85 / 110 / 135 / 160.
    - Bonus base damage per bolt increased to 15 / 22.5 / 30 / 37.5 / 45 from 15 / 21.25 / 27.5 / 33.75 / 40.
    - Maximum base damage increased to 120 / 180 / 240 / 300 / 360 from 120 / 170 / 220 / 270 / 320.
- Nevermove
  - Cooldown reduced to 12 / 11.5 / 11 / 10.5 / 10 from 14 / 13 / 12 / 11 / 10.
- Demonic Ascension
  - Heal health ratio per second increased to $1.5$% **bonus** health from $1.25$% **bonus** health.
  - Non-champion heal health ratio per second increased to $0.15%$ **bonus** health from $0.125%$ **bonus** health.

### V14.21
- Stats
  - Base health regeneration reduced to 3 from 7.
  - Health regeneration growth reduced to $0.5$ from $0.65$.
  - Base mana reduced to 400 from 468.
  - Base mana regeneration increased to 10 from 8.
- Ravenous Flock
  - Healing changed to 3 to 6 of his **maximum** health from 3%–6%@1–16. *Now scales linearly instead of at breakpoints.*
- Death's Hand
  - Mana cost reduced to 40 / 45 / 50 / 55 / 60 from 45 / 50 / 55 / 60 / 65.
  - Base damage changed to 60 / 85 / 110 / 135 / 160 from 65 / 85 / 105 / 125 / 145.
  - Bonus base damage per bolt reduced to 15 / 21.25 / 27.5 / 33.75 / 40 from 15 / 25 / 35 / 45 / 55. *Now always 25% damage.*
    - Maximum base damage reduced to 120 / 170 / 220 / 270 / 320 from 125 / 185 / 245 / 305 / 365. *Now always 200% damage.*
  - AP ratio increased to 45% AP from 40% AP.
  - Bonus AP ratio per bolt increased to $11.25$% AP from 10% AP.
    - Maximum AP ratio increased to 90% AP from 80% AP.
- Vision of Empire
  - Base damage reduced to 70 / 105 / 140 / 175 / 210 from 80 / 115 / 150 / 185 / 220.
    - Minion base damage reduced to 35 / 52.5 / 70 / 87.5 / 105 from 40 / 57.5 / 75 / 92.5 / 110.
  - AP ratio increased to 60% AP from 55% AP.
    - Minion AP ratio increased to 30% AP from $27.5$% AP.
  - Slow strength increased to 50 / 55 / 60 / 65 / 70% from 25 / 35 / 45 / 55 / 65%.
  - Slow duration reduced to $1.5$ seconds from $2.5$.
  - Reveal duration changed to 6 seconds at all ranks from 4 / 5 / 6 / 7 / 8.
- Nevermove
  - Cooldown increased to 14 / 13 / 12 / 11 / 10 from 10 at all ranks.
  - Mana cost increased to 60 / 65 / 70 / 75 / 80 from 50 at all ranks.
  - **Removed:*** Outgoing missile no longer deals 35 / 70 / 105 / 140 / 175 (+ 25% AP) magic damage.
  - Detonation base damage increased to 80 / 120 / 160 / 200 / 240 from 35 / 45 / 55 / 65 / 75.
  - Detonation AP ratio increased to 60% AP from 25% AP.
  - Incoming missile width increased to 180 from 170.
  - Outgoing missile speed changed to 1125 to 1800 from 935–2735@0–0 (@=seconds). *Time to reach maximum range increased to $0.585$ seconds from $0.562$.
  - Incoming missile speed increased to 2000 to 2800 from 600–2200@0–0 (@=seconds). *Time to reach maximum range reduced to $0.4$ seconds from $0.6$.
  - **Bug Fixes:** No longer displays root VFX on a rooted target if the target dies to its damage.
- Demonic Ascension
  - Cooldown increased to 120 seconds at all ranks from 100 / 80 / 60.
  - **New Effect:** Cooldown now starts on-cast instead of post-effect.
  - **Removed:*** No longer reduces Nevermove’s cooldown by 20% while in Demonic Ascension.
  - Base damage per second reduced to 15 / 25 / 35 from 20 / 40 / 60.
  - Damage per second AP ratio reduced to 5% AP from 10% AP.
  - Base heal per second increased to 15 / 30 / 45 from 15 / 27.5 / 40.
    - Non-champion base heal per second increased to 1.5 / 3 / 4.5 from 1.5 / 2.75 / 4.
  - Heal AP ratio per second reduced to 5% AP from 18% AP.
    - Non-champion heal AP ratio per second reduced to $0.5$% AP from $1.8$% AP.
  - **New Effect:** Heal now scales with $1.25$% **bonus** health, reduced to $0.125%$ **bonus** health against non-champions.
- Demonflare
  - **New Effect:** Can now be recast up to 100 times.
  - **New Effect:** Now has a cooldown of 8 seconds between recasts, unaffected by ability haste.
    - Initial recast timer unchanged.
  - Base damage increased to 150 / 250 / 350 from 150 / 225 / 300.
  - AP ratio reduced to 50% AP from 60% AP.
  - Slow strength increased to 75% from 60%.

### V14.18
- Demonic Ascension
  - **Bug Fixes:** Healing is now properly reduced against Voidmite.

### V14.2
- Ravenous Flock
  - Now displays the bonus health gained from *Soul Fragments* in the tooltip and passive icon.

### V13.9
- Death's Hand
  - Base damage increased to 65 / 85 / 105 / 125 / 145 from 60 / 80 / 100 / 120 / 140.
  - Bonus base damage per bolt increased to 15 / 25 / 35 / 45 / 55 from 12 / 22 / 32 / 42 / 52.
    - Maximum base damage increased to 125 / 185 / 245 / 305 / 365 from 108 / 168 / 228 / 288 / 348.
  - AP ratio increased to 40% AP from 38% AP.
  - Bonus AP ratio per bolt increased to 10% AP from 8% AP.
    - Maximum AP ratio increased to 80% AP from 70% AP.

### V12.22
- Stats
  - Mana growth increased to 29 from $28.5$.

### V12.15
- Nevermove
  - **Bug Fixes:** Fixed a bug where the recast could pull targets that were in stasis.

## Trivia

- 
  - In Swain's case, Ravenous Flock infinitely stacks his health.
- **Swain's **Vision of Empire is the longest ranged basic ability in League of Legends, at 7500 units.
  - It used to be tied with Comet of Legend before **Aurelion Sol**’s CGU.
  - It used to be the longest limited-range ability, but was overtaken by Certain Death.
- While dancing, he whistles a tune from his theme.
  - His dance references Fred Astaire Top Hat, White Tie & Tails dance.
    - A side-by-side comparison can be seen here.
- While playing a game on Summoner's Rift, Swain's flock of Ravens (depending on the skin it could also be Snow Owls, Macaws, Demon birds, or Dragons) can be seen around the map.
- His mannerisms, quotes and the "Grand General" persona are reminiscent of from .
- At the start of the game, there is a chance of thematic music with ravens in the background.
- His demonic left arm could be a reference to in the . After losing his left arm, Murtaugh discovered he had a third "karma" arm, which was extremely powerful and capable of causing mass destruction. Murtaugh pursues his goals with similar ruthlessness as Swain.
- The sigil that appears beneath Swain when his ultimate ability is cast has three runes and three eyes inscribed into it. This is possibly a reference to the Three-eyed Raven from the book series A Song of Ice and Fire, considering both characters possess the ability to see events occurring around the world through magical means.
  - The sigil used in Swain, Swain, Swain, and Swain is an edited version of the sigil that appears when Galio uses his ultimate ability.
    - Swain uses the unedited of the sigil.
  - The head of Raum, Swain's demon, shortly appears above Swain when he uses his ultimate ability.
- The name Swain is derived from the Old Norse *sveinn* for "boy".
- Swain's raven is named Beatrice.
  - Beatrice can be seen on the Crystal Scar. She will land on the barrier near the Boneyard node for a short time and then fly off.
- Swain's Ravenous Flock old.png transformation has a similar appearance to Tzeentch's daemon servants, the Lords of Change, from the Warhammer franchise.
- The icon for Ravenous Flock old.png is similar to the old one for Tides of Blood old.png.
- The Art Spotlight for his first splash art is the first one ever released.
- If Swain dies while casting Decrepify, there will be two Beatrices on the battlefield, one attached to the laser and one on his back when he falls to the ground.
- The ending quote to Swain's background is based on Joseph Heller's novel *Catch-22*.
- In the League of Legends, Swain was unable to compete in the Ionia Versus Noxus Grudge Match due to his recent induction into Noxian High Command, thus needing to take time away from the League to put all his affairs in order.
- In the 2014 version of the Ultra Rapid Fire featured game mode, Ravenous Flock old.png would toggle-off once its mana cost scaled beyond Swain's maximum mana. This was the only ability affected by the restriction of a champion requiring sufficient maximum mana in order to activate an ability within the game mode.

---
*This page was automatically generated from League of Legends Wiki data.*