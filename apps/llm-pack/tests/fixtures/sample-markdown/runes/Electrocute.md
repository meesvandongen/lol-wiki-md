# Electrocute

Electrocute is a rune in League of Legends.

## Overview

- **Path:** Domination
- **Slot:** Keystone

## Description

**PASSIVE:** Damaging basic attacks, abilities, [item effects](./Named_item_effect.md), and [summoner spells](./summoner_spell.md), as well as the application of crowd control and damage over time effects, apply stacks against enemy champions, up to one per cast instance per champion. Applying 3 stacks to a target within a 3 second period causes them to be struck by lightning after a 0.25-second delay, dealing them 70 – 240 (based on level) (+ 10% **bonus** AD) (+ 5% AP) of either physical or magic damage.

**VARIABLE DAMAGE:** This effect deals either physical or magic damage depending on the damage contribution from your attack damage and ability power to the effect's damage formula.<li>Greater **bonus damage** from the AD ratio → Physical damage</li><li>Greater **bonus damage** from the AP ratio → Magic damage</li>If the damage contribution of AD and AP are zero or otherwise equal, the damage type defaults to magic damage.

Cooldown: 20 seconds.

> "We called them the Thunderlords, for to speak of their lightning was to invite disaster." — _Rune caption_

## Notes

- **Electrocute** deals proc damage, and thus will not trigger [spell effects](./spell_effects.md).
- **Electrocute** can trigger while [dead](./death.md).
- Blind, Cripple, Drowsy, Kinematics, Nearsight, and Stasis do not count as valid crowd control for triggering stacks.
- Becoming untargetable during the 0.25-second delay will **not** prevent the effect from occurring.
- The 3 second timer is non-refreshing - the third stack has to be applied within 3 seconds of the first.
- Ambessa is special cased to apply a stack for the suppression application and the damage dealt.
- Zed and Shadow Slash, when used together with Shadows, are special cased to generate a stack for each tick of damage.
- Nami's damage instances beyond its first are special cased to not apply a stack.
- Effects that deal proc damage do not trigger a stack unless it is also tagged as pet damage.
  - Such effects can still trigger a stack if they apply a debuff (i.e Black Cleaver).

## Trivia

- **Electrocute** is almost identical to the removed Season 2016 mastery Thunderlord's Decree. The primary distinction between the two is that persistent area of effects and multi-hit spells can no longer generate multiple stacks per cast - e.g. Ethereal Chains and Tormented Shadow.
  - In addition, **Electrocute** requires that all three abilities hit within one three-second window, while **Thunderlord's Decree** only required three seconds between each ability. This makes it harder to activate for champions with slow combos, especially champions who primarily use basic attacks.

## Patch History

### V25.S1.3

- Base damage increased to 70 – 240 (based on level) from 50 – 190 (based on level).

### V14.13

- Cooldown reduced to 20 seconds at all levels from 25 – 20 (based on level).
- Base damage changed to 50 – 190 (based on level) from 30 – 220 (based on level).

### V13.20

- Base damage increased to 30 – 220 (based on level) from 30 – 180 (based on level).
- AD ratio reduced to 10% **bonus** AD from 40% **bonus** AD.
- AP ratio reduced to 5% AP from 25% AP.

### V9.14

- July 19th Hotfix
- On-hit skills with basic attacks no longer grant 2 stacks.

### V8.16

- Base damage reduced to 30 – 180 (based on level) from 50 – 220 (based on level; formula: 40 + (10×level)).
- Bonus AD ratio reduced to 40% **bonus** AD from 50%.
- AP ratio reduced to 25% AP from 30% AP.
- Cooldown reduced to 25 – 20 (based on level) seconds from 50 – 25 (based on level; formula: 51.47 - (1.47×level)).

### V8.1

- Ally hits will no longer count towards **Electrocute** if both allies have the Keystone equipped.

### V7.23

- Allies can no longer help you activate **Electrocute** if they're running it as well.

### V7.22

- Added
- [Domination](./Domination.md) Keystone rune.
**PASSIVE:** Basic attacks and abilities generate stacks on enemy champions hit (up to one per action). If 3 stacks are received within a 3 second interval, the target takes 50 – 220 (based on level; formula: 40 + (10×level)) (+ 50% **bonus** AD) (+ 30% AP) Adaptive damage.
**ADAPTIVE:** Deals either physical or magic depending on which would deal the most damage.
**COOLDOWN:** 50 – 25 (based on level; formula: 51.47 - (1.47×level)) seconds

