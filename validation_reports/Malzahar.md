# Malzahar

## Overview

- **Title:** Malzahar
- **Resource:** Unknown

## Stats

| Stat | Base | Growth |
|------|------|--------|

## Abilities

### Passive – Void Shift

| Attribute | Value |
|-----------|------:|
| **Static** | 30;24;18;12 / 1;6;11;16 |

**INNATE:** Periodically, **Malzahar** gains _Void Shift_ until he takes non-minion damage or negates a crowd control effect, to which it then lingers for 0.25 seconds before expiring. 

**VOID SHIFT:** **Malzahar** gains crowd control immunity and 90% damage reduction.

_Void Shift's_ cooldown refreshes whenever **Malzahar** takes non-minion damage or is affected by a crowd control effect, and resets upon respawning.

| Detail | Value |
|--------|------:|
| **Targeting** | Passive |
| **Affects** | Self |

**Notes:**

- _Void Shift_ will not resist allied crowd control nor will it be deactivated by them.
- _Void Shift_ will not mitigate true damage but the buff will still be consumed.
  - It does not deactivate against the Nexus Obelisk.
- Shields will take priority over _Void Shift_, but the buff will still be consumed (does not reduce damage inflicted). _Void Shift_ triggers even if the shield negated the damage entirely.
  - However, if **Malzahar** has a spell shield and the shield fully mitigates the damage, _Void Shift_ does not trigger. The damage taken must be from an effect that cannot be negated by _spell shields_.
- Spell shields take priority over _Void Shift_. If the triggering hostile ability does not contain a crowd control effect, both _Void Shift_ and the _spell shield_ break simultaneously.
  - Since _spell shields_ are precedent over _Void Shift_, applied crowd control from effects that cannot be blocked by _spell shields_ (i.e. Karthus) will also ignore _Void Shift_ despite its crowd control immunity. The effect must not damage **Malzahar** for this event to take place.
- _Void Shift_ will specifically not deactivate if **Malzahar** eats Honeyfruit or is within a hostile Frozen Heart passive aura.
- _Void Shift_ will prevent Fizz from attaching to him.

### Q – Call of the Void

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Target Range** | 900 (Cast range) |
| **Effect Radius** | 200 |
| **Cost** | (+60 to 80% AP) |
| **Cost Type** | Mana |
| **Cooldown** | 6 |

**ACTIVE:** **Malzahar** opens two portals to the void centered at the target location, granting sight of the area in between. After 0.40 seconds, enemies between the portals are dealt magic damage and silenced for a duration.

| Detail | Value |
|--------|------:|
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Out of Range** | walk to location |
| **Spell Effects** | Area of effect |
| **Spell Shield** | True |

**Notes:**

- Spell shield will prevent _Call of the Void's_ refresh on Malzahar.
  - Malzahar debuff will also be removed.

### W – false

| Attribute | Value |
|-----------|------:|
| **Cast Time** | none |
| **Target Range** | 150 |
| **Cost** | (+40 to 60% AP) |
| **Cost Type** | Mana |
| **Cooldown** | 8 |

**PASSIVE:** **Malzahar** generates a stack of _Zz'Rot Swarm_ when he casts another ability, up to a maximum of 2.

**ACTIVE:** **Malzahar** consumes all _Zz'Rot Swarm_ stacks and, after a 0.50-second delay, summons a Voidling at the target location. Additional _Voidlings_ are then summoned for each _Zz'Rot Swarm_ stack consumed, with each _Voidling_ being summoned 0.50 seconds after the previous one.

_Voidlings_ deal magic damage with their attacks, reduced by 50% against epic monsters and increased by 200% to enemy minions infected by _Malzahar_.

_See Pets for more details about Voidlings._

| Detail | Value |
|--------|------:|
| **Targeting** | Location |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Out of Range** | cast at max |
| **Spell Effects** | spellaoe |

**Notes:**

- **Malzahar** loses all stacks upon death.
- _Zz'Rot Swarm_ stacks are gained per cast and not per ability activation. For example, if Malzahar is cast and the ability cancels under certain circumstances, a _Zz'Rot Swarm_ stack will still be granted despite the ability's failure to complete.

### E – Malefic Visions

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.25 |
| **Target Range** | 650 |
| **Effect Radius** | 500 (Bounce range) |
| **Cost** | (+60 to 100% AP) |
| **Cost Type** | Mana |
| **Cooldown** | (+11 to 7% AP) |

**ACTIVE:** **Malzahar** infects the target enemy's mind, dealing magic damage every 0.25 seconds over 4 seconds, which refreshes upon damaging them with _Malzahar_ or _Malzahar_. _Malefic Visions_ executes minions if they would be damaged below 10 to 30 for 11 health.

If the target dies while infected, they spread _Malefic Visions_ onto the closest nearby enemy and **Malzahar** restores 2% of his **maximum** mana.

| Detail | Value |
|--------|------:|
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | DoT |
| **Spell Shield** | True |
| **Projectile** | false |
| **Call For Help** | True |

**Notes:**

- No additional notes.

### R – Nether Grasp

| Attribute | Value |
|-----------|------:|
| **Cast Time** | 0.01 |
| **Target Range** | 700 |
| **Tether Radius** | 1250 |
| **Cost** | 100 |
| **Cost Type** | Mana |
| **Cooldown** | (+140 to 80% AP) |
| **Cooldown Start** | on-cast |

**ACTIVE:** **Malzahar** tethers to the target enemy champion, knocking them down. He then channels for up to 2.50 seconds, suppressing and revealing the target and dealing them magic damage every 0.25 seconds, revealing himself in the process.

**_Malzahar** will continue to channel as long as the tether is not broken, even if the suppression is removed._

Additionally, a _Null Zone_ is opened beneath the target's location at the time of cast that lasts 5 seconds, dealing magic damage every 0.50 seconds to enemies within, capped at 120 per tick against minions and monsters.

_Null Zone will persist even if the channel is interrupted.

| Detail | Value |
|--------|------:|
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | Magic |
| **Spell Effects** | Special |
| **Spell Shield** | True |
| **Projectile** | False |
| **Call For Help** | true |
| **Silence** | true |

**Notes:**

- If a target is currently being affected by _Malzahar_, casting _Nether Grasp_ on them will grant 2 stacks toward effects such as Electrocute or Phase Rush.
  - This is, correctly, not the case if _Nether Grasp_ is cast before _Malefic Visions_.
- Applies persistent damage for the tether and persistent area damage to enemies within the _Null Zone_.
- The tether will not break if the target enters a zombie state or enters resurrection.
- The tether will continue to damage the target for its full duration even if it is broken early.
- Removing the suppression will also remove the reveal.
- **Malzahar** places himself onto the ground and interrupts displacements affecting him upon starting the channel.
- The following table refers for interactions while **Malzahar** is channeling: (channel)

## Trivia

- After the Battlemage update in V6.9, Malzahar's Voidlings had their color scheme changed from purple with yellow eyes to Malzahar.
- Malzahar is one of the few champions to have a pet, the others being Annie, Elise, Heimerdinger, Ivern, Shaco, and Yorick.
- Malzahar and his Malzahar dance references [https://www.youtube.com/watch?v=otCpCn0l4Wo U Can't Touch This] by MC Hammer.
  - A side-by-side comparison for Malzahar's part can be seen [https://www.youtube.com/watch?v=d9IomGyyQOA here].
  - A side-by-side comparison for his Malzahar part can be seen [https://www.youtube.com/watch?v=sblu-SYUAQA here].
    - He shares this dance (specifically his Malzahar part) with Jayce.
- Upon the removal of Zz'Rot Portal in patch V9.23, _Gathering Swarm_ stacks for Malzahar was renamed to _Zz'Rot Swarm_ to honor the item.
  - This might indirectly imply that Malzahar's voidlings are a similar if not the same species as the Voidspawn that spawned from that item.
- Malzahar's Voidling had its assets reused for several rotating_game_modes.
  - It was given a retexture and features as a monster in Invasion.
  - Its animations were reused for the file=CyteSquare.png monster in Odyssey: Extraction.

<!-- Raw excerpt (first 20 lines) -->

<details><summary>Raw excerpt</summary>

```wikitext

== Abilities ==

== Pets ==

==Champion skins==
:_This article section only contains champion skins. For all associated collection items, see Malzahar_(Collection)._

==Patch history==
```
</details>
