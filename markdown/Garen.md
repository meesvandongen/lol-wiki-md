# Garen

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
  - [Advanced Stats](#advanced-stats)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | Garen |
| **Title** | The Might of Demacia |
| **Resource** | None |
| **Range Type** | Melee |
| **Release Date** | 2010-04-27 |
| **Release Patch** | V1.0.0.83 |
| **Roles** | Juggernaut |
| **Riot Positions** | Top |
| **External Positions** | Top |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $690.0$ | $+98.0$ | $2356.0$ |
| **Mana** | $0.0$ | $+0.0$ | $0.0$ |
| **Health Regen** | $8.0$ | $+0.5$ | $16.5$ |
| **Armor** | $38.0$ | $+4.2$ | $109.4$ |
| **Magic Resist** | $32.0$ | $+1.55$ | $58.4$ |
| **Attack Damage** | $69.0$ | $+4.5$ | $145.5$ |
| **Attack Speed** | $0.625$ | $+3.6\%$ | $1.013$ |
| **Movement Speed** | $340.0$ | $+0.0$ | $340.0$ |
| **Attack Range** | $175.0$ | $+0.0$ | $175.0$ |

### Advanced Stats

| Metric | Value |
|--------|-------|
| **Base Attack Speed** | $0.625$ |
| **Attack Speed Ratio** | $0.625$ |
| **Bonus AS per Level** | $3.6\%$ |
| **Windup Modifier** | $0.5$ |
| **Acquisition Radius** | $600 units$ |
| **Pathing Radius** | $35 units$ |
| **Selection Radius** | $120 units$ |
| **Selection Height** | $188.889 units$ |
| **Critical Damage** | $175.0\%$ |

## Abilities

### Passive: Perseverance

**Innate:** **Garen** continually health regeneration a portion of his *health **maximum** health*.

*This ability is temporarily disabled if he is attacked by epic monsters, enemy turrets, or enemy champions.*

**Innate:** **Garen** health regeneration an additional every 5 seconds. key=% of his every $0.5$ seconds. *Perseverance* is lost for 8 seconds if **Garen** takes damage from champion, epic monster, or turret, or if he is hit by an enemy ability or affected by an enemy summoner spell, refreshing on subsequent damage and hits taken from them.

| Attribute | Value |
|-----------|-------|
| **Targeting** | Passive |
| **Affects** | Self |
| **Spell Effects** | Heal |

**Notes:**

- *Perseverance* does not go on nor refresh its cooldown from damage which was fully prevented by a shield or invulnerability.

---

### Q: Decisive Strike

**Active:** **Garen** gains *ms **bonus** movement speed* and cleanse from slow.

*His next basic attack will lunge and deal **bonus** physical damage, silence the target.*

**Active:** **Garen** cleanse himself of all slow and gains ms*bonus** movement speed* for a duration. Additionally, **Garen** empowers his next basic attack within $4.5$ seconds to have an uncancellable windup, lunge at the target, deal **bonus** physical damage, and silence them for $1.5$ seconds. *Decisive Strike basic attack reset *'Garen's* basic attack timer.*

| Attribute | Value |
|-----------|-------|
| **Cooldown** | 8 seconds |
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Self, Enemies |
| **Damage Type** | Physical |
| **Spell Shield** | Special |
| **Spell Effects** | spell |

**Scaling:**
- **Movement Speed Duration:** $1-3.6$
- **Bonus Physical Damage:** $30-150$ (+ 50% AD)

**Notes:**

- The enhanced attack will apply other on-hit effects and can critical strike as normal (the bonus damage does not).
  - The bonus damage will also apply .
  - The bonus damage is applied to structures.
- *Decisive Strike* locks **Garen** out of using basic attacks for a short period of time. This is shortened with .
- The enhanced attack will still complete and hit the target even if they become untargetable during the attack windup.
- *Decisive Strike* will not prevent the reapplication of slow (e.g. the persistent slow from *Pillar of Ice* will be immediately reapplied after removal if **Garen** is still within its area of effect).
- Spell shield will only negate the silence.
- 'Decisive Strike's lunge can pass very thin terrain and will otherwise not be able to. In any case, **Garen** will still hit his target.

---

### W: Courage

**Passive:** **Garen** permanently gains **bonus** and magic resistance by killing enemies, up to a cap. Upon reaching the cap, he gains another permanent boost to these defenses.

**Active:** **Garen** reduces incoming damage for a few seconds. He also briefly gains a shield and increased tenacity.

**Passive:** Whenever **Garen** kills an enemy, he generates a stack of *Courage*, stacking up to 150 times. **Courage:** For each stack, **Garen** gains armorbonus armor* and mr*bonus** magic resistance*, up to a maximum of 30 **bonus** resistances each. **Active:** **Garen** reduces incoming damage for 4 seconds. For the first $0.75$ seconds, **Garen** additionally grants himself a shield and 60% tenacity.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $22-12$ seconds |
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Self |

**Scaling:**
- **Damage Reduction:** $25-41$%
- **Shield Strength:** $65-145$ (+ 18%
- **bonus** health)

**Notes:**

- 'Courage's* bonus resistances are only gained when it is ranked up at least once, but *Courage's passive is retroactive for units killed before it was learned.
- 'Courage's* **bonus** resistances can be obtained by any of *'Garen's' damage sources (basic attacks, abilities, summoner spells, runes, item actives).
  - This does not include *last hits* transferred to **Garen** artificially.
- The kills on these enemy unit types count towards *Courage* stacks:
  - champion (Only kills, assists don't grant stacks)
  - All minions and monster
  - Champion summoned units (e.g. *Noxious Trap*, *Powder Keg*, *Dark Procession*)
  - All pet and clone. *** The bonus is equal for all these units; *large* units do not grant a larger bonus.
- The kills on these enemy unit types do **not** count towards *Courage* stacks:
  - Wards.
  - Turrets (including *Sun Disk*) and other structures
  - Jungle plants.
  - Units destroyed by 'trampling' such as Rampant Growth, Cell Division or Tunnel.
- *Courage* will only grant bonus resistances from units killed by **Garen** himself. *Eye of the Herald* does not count.
- The Tenacity increase stacks additively with the *tenacity reduction* from Brittle, but multiplicatively with other sources.

---

### E: Judgment

**Active:** **Garen** rapidly spins with his sword for a short time while ghosted, continually dealing physical damage to nearby enemies. He spins more based on his *as *bonus attack speed*.

*The nearest enemy is dealt increased damage. Enemy champions hit by enough spins will have armor penetration.*

**Active:** **Garen** rapidly spins his sword around himself 7 (+ 1 per Only bonus attack speed from item stats and growth (per level)) times over 3 seconds, becoming unable to declare basic attacks but gaining ghosting and dealing physical damage to nearby enemies periodically. *Judgment* can be recast after 1 second while active, and does so automatically after it ends. Enemy champions hit 6 times by *Judgment* are inflicted with armor penetration for 6 seconds, with the duration refreshing upon the 7th hit and every 6th hit thereafter. *Judgment* deals 25% increased damage against the nearest enemy hit. **Recast:** **Garen** ends *Judgment*. *Judgment can critical strike for

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $9-6$ seconds |
| **Cast Time** | none |
| **Targeting** | Auto |
| **Affects** | Enemies |
| **Damage Type** | Physical |
| **Effect Radius** | 325 units |
| **Spell Effects** | aoedot |

**Scaling:**
- **Physical Damage Per Spin:** $4-16$ (+ $38-50$% AD)
- **Increased Damage Per Spin:** $4×1.25-16×1.25$ (+ $38×1.25-50×1.25$% AD)

**Notes:**

- Deactivating the ability manually does not count as an ability activation for the purposes of on-cast effects such as Spellblade and triggering *Force Pulse*’s passive.
- Each spin triggers a stack of *Conqueror*.
- For the purpose of moving closer to an enemy when right clicking them, *Judgment* also reduces ''Garen's' attack range for the duration.
- *Judgment* cancels ''Garen's' last movement command upon ending.
- Each spin deals damage simultaneously to all enemies within its range when the spin is completed.
- The spin animation is capped at once per $0.2$ seconds, but the amount of ticks will keep on scaling.
- The armor reduction stacks multiplicatively with other percentage armor penetration effects.

---

### R: Demacian Justice

**Active:** **Garen** strikes a lethal blow to an enemy champion that deals true damage based on the target's **missing** health.

**Active:** **Garen** calls upon the might of *Demacia* onto the target enemy champion, dealing them *true damage* as well as true sight them for 1 second at the start of the cast time.

| Attribute | Value |
|-----------|-------|
| **Range** | 400 units |
| **Cooldown** | $120-80$ seconds |
| **Cast Time** | $0.435$ seconds |
| **Targeting** | Unit |
| **Affects** | Enemies |
| **Damage Type** | True |
| **Spell Shield** | True |
| **Spell Effects** | spell |

**Scaling:**
- **True Damage:** $ (+ $% of target's
- **missing** health)

**Notes:**

- The target will die if they're at least under $(*100 to 100)/100)*maximum** health + $(*100 to 100)/100)*flat** health on top of that. This does not count effects that mitigate true damage, such as shield.

---

## Patch History

### V25.18
- *Judgment*
  - AD ratio per spin increased to $38-50$% AD from $36-48$% AD.

### V25.12
- *Judgment*
  - Base damage per spin reduced to $4-16$ from $4-20$.
  - AD ratio per spin increased to $36-48$% AD from $36-44$% AD.
  - **Bug Fixes:** Armor reduction across separate casts of Judgment is now properly refreshed every 6th spin within the debuff's duration instead of requiring.

### V25.11
- General
  - Updated recommended items.
- *Perseverance*
  - **Bug Fixes:** Tooltip now displays one decimal place in the health ratio instead of none/rounding to the nearest integer which would be slightly inaccurate. *(Actual value was not affected.)*
- *Courage*
  - Damage reduction changed to $25-41$% from 30% at all ranks.
  - Cooldown reduced to $22-12$ from $23-15$.
  - **Removed:*** No longer grants 10% *bonus armor and 10% **bonus** magic resistance at maximum stacks.
  - Tooltip now displays the amount of armor and magic resistance gained before explaining the stacking mechanic.
  - **Bug Fixes:** Restored success VFX particles when mitigating crowd control with the gained tenacity.
- *Judgment*
  - AD ratio per spin increased to $36-44$% AD from $36-40$% AD.
  - *Judgment* is now modified by 80% of the caster's total critical damage instead of 150% of 'Judgment's base damage.
    - Base critical damage reduced to $0.8×175$% from 150%.
    - *Infinity Edge* bonus critical damage reduced to $40×0.8$% from 40%.
  - Cooldown reduced to $9-6$ from 9 at all ranks.
  - **Removed:*** No longer has an additional base damage per spin of 0.8*(x-1) for 9–then + 0.2*x.
  - **Removed:*** Recast no longer reduces its cooldown by its remaining duration.
  - **Removed:*** No longer deals 150% damage against non-epic monsters.
  - **Bug Fixes:** Duration no longer sometimes illegally extends by up to $0.25$ seconds (depending on game tick), causing another spin beyond the designated limit.
  - **Bug Fixes:** Cooldown no longer sometimes improperly extends by up to $0.25$ seconds (depending on game tick) when recasting to end the ability early.
  - **Bug Fixes:** Tooltip now properly calculates critical damage.
- *Demacian Justice*
  - Base damage reduced to $150-350 3$ from $150-450 3$.

### V25.08
- *Decisive Strike*
  - **Bug Fixes:** No longer causes the target to illegally survive at 0 health if they had a method to resurrection active and blocked the applicable effect with a spell shield.

### V25.06
- *Judgment*
  - Base critical damage reduced to 150% from 175%.

### V14.9
- Stats
  - Selection radius increased to 120 units from 75.

### V14.2
- *Courage*
  - Damage reduction duration changed to 4 seconds at all ranks from $2-5$.
- *Judgment*
  - AD ratio per spin increased to $36-40$% AD from $32-40$% AD.

### V13.8
- Stats
  - Base armor increased to 38 from 36.
  - Base attack damage increased to 69 from 66.

### V13.5
- General
  - **Bug Fixes:** Garen now has a separate SFX when attacking turrets.

### V13.4
- General
  - New splash artwork for Garen.

## Trivia

- Garen was featured in Battle Training before the tutorials' first revamp.
- His dance is a reference to Bye Bye Bye by NSYNC.
  - A side-by-side comparison can be seen here.
  - He used to share this dance with **Tryndamere** before they both got reworked.
- **Wukong**’s abilities might have been based on Garen's kit. (*Decisive Strike* / *Crushing Blow*, *Judgment* / *Cyclone*)
- Garen was the first to have his price reduced twice, others being **Miss Fortune** and **Warwick**.
- The original icon of *image=Perseverance old.png* features a silhouette of Garen's pose from his first Garen.
- In the now-removed official League of Legends forums, the original icon of *image=Perseverance old.png* was used to represent the "Public Beta Environment" section.
- *Garen* Գարեն is an Armenian name, shortened from Old Armenian Գարեգին *Garegin*;
  - Hrachia Adjarian interprets this name as ", though .
  - Interestingly, Garen's and **Lux**’s parents both bear Pre-Greek substrate names: Tt < Πέτρος "stone" & Tt < Ᾰ̓γᾰθή "good, fit, noble".
- This champion has no ability power ratio.
- Garen - **Lux** is one of seven pairs of sibling champions (the others being **Cassiopeia** - **Katarina**, **Kayle** - **Morgana**, **Nasus** - **Renekton**, **Yasuo** - **Yone**, **Darius** - **Draven**, and **Vi** - **Jinx**).
  - Though not a pair, **Anivia**, **Ornn**, and **Volibear** are also siblings.
- Garen's dance references Johnny Bravo dance.
  - A side-by-side comparison can be seen here.
- Contrary to popular belief, Garen yells **"Demacia!"** when using *Courage* and not during *Judgment* or *Demacian Justice*.
  - Players using these abilities simultaneously led to this confusion.
- The bright and shimmery tonality on Garen's *Demacian Justice* came from a closely mic'd tuning fork and a finger cymbal hitting a broadsword.
- **"Spin To Win!"** comes from how easily it was for Garen to get kills while using it.
  - It earned meme status when animated with Ievan Polkka by Loituma playing in the background.
    - When using *Judgment* Garen earns a cosmetic buff that reads *"Garen is spinning to win"*.
      - The phrase is now associated to every spinning ability (e.g. *Counter Strike*, *Death Lotus*, *Powerball*, *Spinning Slash*, and *Cyclone*)
- Garen, **Zed**, **Sona**, and **Vi** were 'targeted' by **Jhin**. This is not canon, so much as a teaser for the general theme of **Jhin**, as **Zed** is the only character with any connection.
  - After Garen was shot, his champion icon on his League of Legends website page and champion list were updated to a gif with what seems to be shards of some sort floating out.

---
*This page was automatically generated from League of Legends Wiki data.*