# Bard

## Overview

- **Title:** Bard
- **Resource:** Mana

## Stats

| Stat | Base | Growth |
|------|------|--------|
| Armor | 34 | 5 |
| Attack Damage | 52 | 3 |
| Attack Speed | 0.658 | 2 |
| HP | 630 | 103 |
| HP Regen | 5.5 | 0.55 |
| MP | 350 | 50 |
| MP Regen | 6 | 0.45 |
| Magic Resist | 30 | 1.3 |
| Move Speed | 335 | 0 |
| Range | 500 | 0 |

## Advanced Stats

| Metric | Value |
|--------|-------|
| Acquisition Radius | 600 |
| Attack Cast Time (s) | 0.3 |
| Attack Delay Offset (s) | -0.1 |
| Attack Speed Ratio | 0.658 |
| Base Attack Time (s) | 1.6 |
| Gameplay Radius | 80 |
| Missile Speed | 2000 |
| Pathing Radius | 35 |
| Selection Height | 302.7778 |
| Selection Radius | 130 |
| Windup % | 18.8% |

## Abilities

### Passive – Traveler's Call

| Attribute | Value |
|-----------|------:|
| **Static** | 50 (Chime spawn rate) / changedisplay=true / type=number of Chimes / 8 to 4 for 5 / 0;20;40;55;70 (Meep spawn rate) |

**INNATE - ANCIENT CHIMES:** **Bard's** presence causes sacred  Chimes to appear at random locations on the map, lingering for up to 10 minutes. Collecting a Chime restores 12% **maximum** mana and grants **Bard** 20 + (_1 per minute after 5 minutes_) experience, as well as 24% **bonus** movement speed [out of combat](./combat_status.md) for 20 seconds, which stacks up to 10 times. Each Chime collected beyond the first instead grants an additional 14% **bonus** movement speed, up to a total of 150% **bonus** movement speed. **Bard** empowers his Meeps each time he collects 5 Chimes.

**INNATE - MEEPS:** **Bard's** presence attracts Meeps, which are small spirits that come to his side. While he has Meeps, his basic attacks are empowered to each consume a Meep on-attack to deal 35 (_+ 10 per 5 Chimes collected_) (+ 40% AP) **bonus** magic damage.
* At 5 Chimes, Meeps slow damaged enemies by type=number of Chimes / key= / 25 to 75 for 6 / 5;25;45;60;75;85 for 1 second.
* At 15 Chimes, Meeps deal the damage to enemies within 150 units of the primary target as well as those in a cone behind them.
* At 35 Chimes, the cone's radius increases.

**Bard** can have up to type=number of Chimes / 1 to 9 for 9 / 0;10;30;50;65;80;90;95;100 Meeps at a time.

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | mixed |
| **Spell Shield** | Special |
| **Parry** | False |
| **Projectile** | True |

**Notes:**

- Chime spawns are weighted towards the areas of the map that **Bard** is currently present in and spawn in pairs, with the only exception being the second and third spawns on Summoner's Rift, which spawn only one.
  - Chimes will not spawn in either team's base and will not spawn in the enemy team's jungle before the 5 minute mark.
- The first Chime pair spawns at:
  - **0:15** (45 seconds before Ambient game start.) in all Summoner's Rift modes.
  - **0:50** (5 seconds after Ambient game start.) in all Howling Abyss modes.
- The damage from Meeps:
  - Applies as spell damage on the primary target.
  - Applies as area damage on secondary target(s).
- Meeps grant additional stacks of Electrocute, Phase Rush and Conqueror.
- Only **Bard** can see a Chime that is in the [fog of war](./fog_of_war.md). All other players must have sight of a chime for it to be visible to them.
  - Only **Bard** can see a minimap icon for an uncollected Chime, regardless of whether other players have vision on it or not. The minimap icon will turn red when the chime is one minute away from expiration.
  - **Bard** cannot see Chimes while nearsighted.
- Only Meep cone attacks are absorbed by spell shields. Those that deal damage together with the basic attack don't.
- If a Meep empowered auto attack is dodged, blocked or when **Bard** is blinded, the basic attack itself will deal no damage, but the Meep damage will still be applied.
- The empowered attack will not trigger against [structures](./structures.md) and [ward](./ward.md)s. | Minimum Time | Chimes | Effect | Base Damage | | --- | --- | --- | --- | | 3:20 | 5 | Meeps now slow by 25%. | 49 | | 5:00 | 10 | Meep limit increased to 2. | 63 | | 7:30 | 15 | Meeps now deal splash damage. | 77 | | 9:10 | 20 | Recharge time reduced to 7 seconds. | 91 | | 11:40 | 25 | Slow increased to 35%. | 105 | | 13:20 | 30 | Meep limit increased to 3. | 119 | | 15:50 | 35 | Splash damage area increased. | 133 | | 17:30 | 40 | Recharge time reduced to 6 seconds. | 147 | | 20:00 | 45 | Slow increased to 45%. | 161 | | 21:40 | 50 | Meep limit increased to 4. | 175 | | 24:10 | 55 | Recharge time reduced to 5 seconds. | 189 | | 25:50 | 60 | Slow increased to 55%. | 203 | | 28:20 | 65 | Meep limit increased to 5. | 217 | | 30:00 | 70 | Recharge time reduced to 4 seconds. | 231 | | 32:30 | 75 | Slow increased to 65%. | 245 | | 34:10 | 80 | Meep limit increased to 6. | 259 | | 36:40 | 85 | Slow increased to 75%. | 273 | | 38:20 | 90 | Meep limit increased to 7. | 287 | | 40:50 | 95 | Meep limit increased to 8. | 301 | | 42:30 | 100 | Meep limit increased to 9. | 315 | Every additional 5 chimes collected beyond 100 grant +14 bonus damage.

### Q – Cosmic Binding

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Target Range** | 850 |
| **Width** | 120 |
| **Speed** | 1500 |
| **Cost** | 60 |
| **Cost Type** | Mana |
| **Cooldown** | (+11 to 7% AP) |

**ACTIVE:** **Bard** fires an energy bolt in the target direction that deals magic damage to the first enemy hit and slows them by 60% for a duration.

After _Cosmic Binding_ hits an enemy, the bolt continues behind them for 300 units (Estimated). If the bolt hits terrain or a second enemy, it stuns both targets for the same duration, dealing the same damage to the secondary target.

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | aoe |
| **Spell Shield** | Special |
| **Projectile** | True |

**Notes:**

- _Cosmic Binding's_ interaction with spell shields:
  - If the first target has a spell shield, they will block the damage but the bolt will still pass through them.
    - If the bolt hits a second target, only they will be stunned.
    - If the bolt hits terrain, the first target will be stunned.
  - If the second target has a spell shield, they will block the stun and damage.
    - The first target will still be stunned.
  - If the first target gains a spell shield after being hit, the stun will not be blocked.
- The stun will apply even if the first target becomes untargetable.
- _Cosmic Binding_ can interact with [player-generated terrain](./Terrain.md#Player-Generated).
- The stun is still applied even if the first or second target dies to the ability itself or before the projectile can bind them.
- _Cosmic Binding_ does not grant sight of enemies hit, but a hit can be seen and heard to indicate that at least one enemy is there.

### W – Caretaker's Shrine

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Target Range** | 800 |
| **Cost** | 70 |
| **Cost Type** | Mana |
| **Recharge** | 18 |

**ACTIVE:** **Bard** conjures a shrine at the target location that grants sight of its surroundings for 1 second and gathers power over 5 seconds. Up to 3 shrines may be active at a time. Shrines may last indefinitely until being consumed by a champion moving over them.

If the champion is an ally or **Bard** himself, they are healed for an amount based on the shrine's power and gain **bonus** movement speed decaying over 1.50 seconds. The **bonus** movement speed can [stack](./stack.md) with subsequent shrines triggered. If the champion is an enemy, the shrine is destroyed.

**Bard** periodically [stock](./stock.md)s a _Caretaker's Shrine_ charge, up to a maximum of 2.

| Detail | Value |
|--------|------:|
| **Targeting** | [Location](./Location-targeted.md) / [Unit](./Unit-targeted.md) |
| **Affects** | Allies |
| **Out of Range** | If targeting a location, walk in range of the target location to cast

If targeting an ally, walk in range of the target unit to cast |

**Notes:**

- _Caretaker's Shrine_ can be cast directly on an ally and it will be treated as a [unit-targeted](./unit-targeted.md) ability for effects such as Guardian. This version also has a forgiveness radius of 175 units.
  - If **Bard** has 3 _shrines_ on the map and creates a new one that is cast directly on himself or an ally, the oldest shrine will not be destroyed. This is because the _shrine_ is used as an actual targeted heal on himself or someone else, and not a permanent shrine, so the other 3 may stay.
  - The [spell indicator](./spell_indicator.md) of _Caretaker's Shrine_ shows the area in which it can be used as a targeted heal. After a shrine has been placed on the ground its activation area becomes smaller: allies or enemies have to step directly on the shrine to use it or destroy it.
- Leveling up _Caretaker's Shrine_ will update the healing of existing shrines.

### E – Magical Journey

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Target Range** | 900 |
| **Speed** | 900 (Enemy travel speed) / 1197 (Bard and ally travel speed) |
| **Cost** | 30 |
| **Cost Type** | Mana |
| **Cooldown** | (+22 to 16% AP) |
| **Cooldown Start** | on-cast |

**ACTIVE:** **Bard** opens a one-way magical corridor through a piece of [terrain](./terrain.md) in the target direction for 10 seconds.

A champion can pass through the corridor by selecting it while within proximity of it, becoming revealed during the travel. **Bard** and allies travel through the portal at 33% increased speed.

_A valid piece of terrain within the target range is required to cast this ability. Magical Journey cannot be taken while immobilized or grounded._

| Detail | Value |
|--------|------:|
| **Targeting** | Direction |
| **Affects** | Self, Allies, Enemies |
| **Out of Range** | The ability will not cast even when targeting a valid piece of terrain that is outside of the target range |
| **Projectile** | False |
| **Grounded** | special |
| **Knockdown** | True |

**Notes:**

- Terrain also includes [structures](./structures.md).
- _Magical Journey's_ tunnel has a max range of 2600 units (about two screens long)<ref>[http://boards.na.leagueoflegends.com/en/c/gameplay-balance/Ev3qQR4G-bard-qa?show=flat&comment=00dc0001/ Bard Q&A comment by Rabid Llama]</ref>.
- _Magical Journey_ will score assists for **Bard** if an ally that uses it scores a [kill](./kill.md) or [assist](./assist.md) shortly after.
- **Bard** can cast _Magical Journey_ while grounded or rooted.
- There is no limit to how many times a given instance of _Magical Journey_ may be used.
- _Magical Journey_ cannot interact with player-created terrain due to its short lifespan.<ref>[http://boards.na.leagueoflegends.com/en/c/gameplay-balance/Ev3qQR4G-bard-qa?show=flat&comment=001f0000/ Bard Q&A comment by Rabid Llama]</ref>
- Any abilities that continue during normal movement, such as [point blank area of effects](./area_of_effect.md) and various forms of stealth, will continue during the travel within _Magical Journey_.
- If a champion's dash is interrupted while inside terrain using _Magical Journey_, they will be immediately displaced out to the nearest open space, but are not rendered airborne.
- Taking _Magical Journey_ is considered a dash, and so will interact with effects such as Sudden Impact and knockdown.

### R – Tempered Fate

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.50 |
| **Target Range** | 3400 |
| **Effect Radius** | 350 |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | (+110/95/80% AP) |

**ACTIVE:** **Bard** sends magical energy arcing to the target location, granting sight of the area during travel. Upon impact, it puts all units within into stasis for 2.50 seconds, as well as stunning all enemy champions, minions, and turrets struck for the same duration. Enemies hit are revealed for the duration.

_Epic monsters and turrets are affected by Tempered Fate despite being immune to crowd control._

| Detail | Value |
|--------|------:|
| **Targeting** | Location |
| **Out of Range** | walk to location |
| **Spell Shield** | True |
| **Projectile** | False |

**Notes:**

- _Tempered Fate's_ stun is affected by Tenacity.
  - The stasis is unaffected.
- _Tempered Fate_ will affect all _targetable_ champions, minions, monsters, turrets, wards, and [jungle plants](./jungle_plants.md).
- _Tempered Fate_ has no effect on enemies that have displacement immunity or total crowd control immunity, and allies that are immune from using an ability that preloads UnstoppableForceMarker.
  - The only exceptions are the crowd control immunity that turrets and epic monsters have.
- All champions (including allies) during the stasis are prohibited from activating cleansing effects for its duration.
  - It will not disable  Quicksilver and instead put it on a 3-second cooldown.
- _Tempered Fate_ deals 0 proc true damage, which triggers [in-combat](./combat_status.md) effects such as drawing turret and [monster aggression](./Monster.md#Monster_Behavior), Sudden Impact or applying Elixir of Sorcery.
  - Elixir of Sorcery deals damage before the target is put into stasis.
- _Tempered Fate's_ travel time varies between ~0.65 at point blank and ~1.80 at max range, resulting in an overall delay of ~1.15-2.30 seconds including the cast time depending on distance covered.

## Trivia

  - In Bard's case, Bard infinitely stacks the base magic damage of his Bard.
- Curiously enough, The Wonder Above is the antithesis of The Terror Beneath, going so far as to both having a portal-opening ability (Bard and Rek'Sai) as well as an enemy-destructible object-placing one (Bard and Rek'Sai)

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see [Bard (Collection)](./Bard_Cosmetics.md)._

==Patch history==

==Trivia==
* 
```
</details>
