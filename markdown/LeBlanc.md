# LeBlanc

## Table of Contents
- [Basic Information](#basic-information)
- [Statistics](#statistics)
- [Abilities](#abilities)
- [Patch History](#patch-history)
- [Trivia](#trivia)

## Basic Information

| Attribute | Value |
|-----------|-------|
| **Champion** | LeBlanc |

## Statistics

### Base Stats (Level 1-18)

| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|
| **Health** | $550$ | $+85$ | $1995.0$ |
| **Mana** | $350$ | $+50$ | $1200.0$ |
| **Armor** | $22$ | $+3.5$ | $81.5$ |
| **Magic Resist** | $30$ | $+0.5$ | $38.5$ |
| **Attack Damage** | $56$ | $+3.1$ | $108.7$ |
| **Attack Speed** | $0.625$ | $+3.2\%$ | $0.965$ |

## Abilities

### Passive: Mirror Image

**Innate:** When **LeBlanc** reaches low health, she creates a clone of herself and together they enter brief invisibility.

*The clone will run forward to a random location, and lasts for a period.*

**Innate:** Periodically, when **LeBlanc** is damaged to health*maximum** health*, she creates a clone of herself, after which they both instantly become invisibility for 1 second. The clone can cast basic attacks with no effect and lasts 8 seconds. Upon spawning, it starts moving up to 1900 units toward a random direction in front of ''LeBlanc's'' movement, determined at the moment of its spawning. *See [Pets](#Pets) for more details about *'LeBlanc's'* clone.*

**Notes:**

- The clone has a leash range.
- Using a basic attack breaks the stealth at the end of the attack windup.

---

### Q: Sigil of Malice

**Active:** **LeBlanc** projects an orb at the target enemy, dealing magic damage and marking them for a short time.

*Her next damaging ability against the marked target will consume the mark to deal magic damage.*

**Active:** **LeBlanc** projects an orb at the target enemy that deals magic damage and marks them for $3.5$ seconds. ''LeBlanc's'' next damaging ability against the marked target will consume the mark to deal the again. 'Sigil of Malice's' orb deals $10 to 146$ **bonus** magic damage against minion and refunds 100% of its *mana cost* and 30% of its remaining *cooldown* if either the orb or its mark's consumption kills the target.

| Attribute | Value |
|-----------|-------|
| **Range** | 700 units |
| **Cooldown** | 6 seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 50 Mana |

**Scaling:**
- **Magic Damage:** $65-165$ (+ 40% AP)
- **Total Magic Damage:** $65*2-165*2$ (+ $40*2$% AP)

**Notes:**

- *Sigil of Malice* and *Mimic: Sigil of Malice* can detonate each other's marks.
- The floating damage text upon the mark's detonation currently displays a real critical strike icon instead of a success indicator.

---

### W: Distortion

**Active:** **LeBlanc** dashes to the target location, dealing magic damage to nearby enemies upon arrival. She leaves behind a return pad at her original location that lasts a few seconds, during which she can recast.

**Recast:** **LeBlanc** blinks to the return pad.

**Active:** **LeBlanc** dashes to the target location, dealing magic damage to all nearby enemies upon arrival and leaving a return pad at the cast location for 4 seconds. *Distortion* can be recast after $0.2$ seconds of the dash ending for the pad's duration. **Recast:** **LeBlanc** blink to 'Distortion's' return pad, regardless of range.

| Attribute | Value |
|-----------|-------|
| **Range** | 600 / Global |
| **Cooldown** | $15-10$ seconds |
| **Cast Time** | none |
| **Cost** | $60-100$ Mana |

**Scaling:**
- **Magic Damage:** $75-235$ (+ 70% AP)

**Notes:**

- *Distortion* will be buffered and cast as soon as the cooldown ends if the player attempts to cast it within $0.5$ seconds of the cooldown ending.
  - The **recast** will be buffered and cast as soon as it becomes available if the player attempts to cast it within $0.3$ seconds of it becoming available to cast. *The initial cast counts as an ability activation for the purposes of on-cast effects such as Spellblade and triggering passive. **Recasting the ability does not. ***Mimic: Distortion* has the same properties.
- *Distortion* and *Mimic: Distortion* have independent return pads and **LeBlanc** can travel to both while they are active.
- 'Distortion's' pad duration starts once the dash ends.

---

### E: Ethereal Chains

**Active:** **LeBlanc** flings an illusory chain in the target direction that deals magic damage and tether the first enemy hit, true sight them.

*The tether lasts a brief time, after which the target is dealt magic damage, true sight, and briefly root.*

**Active:** **LeBlanc** flings an illusory chain in the target direction that deals magic damage to the first enemy it hits and forms a tether between **LeBlanc** and the target for $1.5$ seconds, during which they are true sight. If the tether is not broken by the end of its duration, it fractures to deal magic damage to the target and root them for $1.5$ seconds, during which they are true sight.

| Attribute | Value |
|-----------|-------|
| **Range** | 950 units |
| **Cooldown** | $14-11$ seconds |
| **Cast Time** | $0.25$ seconds |
| **Cost** | 50 Mana |

**Scaling:**
- **Magic Damage:** $50-130$ (+ 40% AP)
- **Magic Damage:** $80-240$ (+ 85% AP) Total Damage $50+80-130+240$ (+ $40+85$% AP)

**Notes:**

- Both the initial and the delayed damage can detonate *Sigil of Malice*.
- *Ethereal Chains* and *Mimic: Ethereal Chains* are separate debuffs, and can both be tethered to the same target without overriding each other. The root duration for either one are unchanged.
- Spell shield will block the tether's application and initial damage but not the aftereffects of one already applied.
- The tether remains for a minimum of $0.25$ seconds regardless of distance.

---

### R: Mimic

**Active:** **LeBlanc** casts a mimicked version of her most recent ability, which deals modified damage.

**Active:** **LeBlanc** casts a mimicked version of her most recently used basic ability, applying the same effects and dealing *modified magic damage. ***Mimic: Sigil of Malice* The orb deals modified damage, while the mark consumption deals double that amount. ***Mimic: Distortion* Deals modified damage. ***Mimic: Ethereal Chains* The tether's application deals modified damage, while its fracturing deals double that amount.

| Attribute | Value |
|-----------|-------|
| **Cooldown** | $45-25$ seconds |

**Scaling:**
- **Orb Magic Damage:* $70-210$ (+ 40% AP)2-2102$ (+ 80% AP)3-210*3$ (+ 120% AP)
- **Magic Damage:** $150-450$ (+ 75% AP)

**Notes:**

- *Mimic* will default to *Mimic: Sigil of Malice* if *LeBlanc* ranks up *Mimic* without having used any abilities beforehand (even if she has not yet learned *Sigil of Malice*).
- Both *Mimic: Ethereal Chains* and *Mimic: Sigil of Malice* will apply damage twice if the original ability's condition to do so is met.

---

## Patch History

### V25.18
- *Distortion* & *Mimic: Distortion*
  - **Bug Fixes:** props no longer sometimes appear during the dash.

### V25.15
- General
  - **Bug Fixes:** Cape disappearance animation now properly plays on death for certain skins.

### V25.12
- *Ethereal Chains*
  - Detonation AP ratio increased to 85% AP from 80% AP.
- *Mimic: Ethereal Chains*
  - Detonation AP ratio increased to 85% AP from 80% AP.
- *Mimic*
  - Cooldown reduced to $45-25 3$ seconds from $50-30 3$.

### V25.10
- *Mimic: Distortion*
  - **Bug Fixes:** No longer deals the non-mimicked version's damage when cast very quickly after *Distortion*.

### V25.07
- General
  - Updated ability icons.
  - Complete visual update across all skins.
    - New splash artwork for , , and .
    - Adjusted splash artwork for , , , , , , , , , and .
  - New voice-over.
  - Updated sound effects.
  - cost increased to from .
  - cost increased to from .

### V25.04
- 
  - **Bug Fixes:** Non-damaging displacement effects can no longer be prematurely canceled by using the Toggle expression during the movement.
- 
  - Updated skin border to reflect Faker's fifth World Championship title.

### V14.24
- *Sigil of Malice*
  - Base damage reduced to $65-165$ from $70-170$.
- *Mimic: Distortion*
  - **Bug Fixes:** No longer reveals the user if cast while the cursor is near an enemy champion.

### V14.17
- *Sigil of Malice*
  - AP ratio reduced to 40% AP from 45% AP.
    - Total AP ratio reduced to $40*2$% AP from $45*2$% AP.
- *Distortion*
  - AP ratio reduced to 70% AP from 75% AP.

### V14.14
- 
  - *Mirror Image*
    - **Bug Fixes:** Restored clone death SFX.

### V14.8
- *Sigil of Malice*
  - Base damage increased to $70-170$ from $65-165$.
- *Distortion*
  - Cooldown reduced to $15-10$ seconds from $18-10$.

## Trivia

- LeBlanc is the first champion to have a Loading screen and Portrait icon not focus the actual champion, in her case, focus on her *Mirror Image*.
  - The second one is **Jhin**, focus on the mirror image.
  - The same case applies for some of her skins: , , and .
- *Mimic* used to be the only ability (currently *Relentless Pursuit, AiDivine Judgement* and *Purge*) that ended up costing 0 mana when fully-ranked.
- LeBlanc is one of a few champions to have multiple textures in one skin. When she uses *Mirror Image*, her clone will be of a different colour palette which can only be seen by LeBlanc and her allies. To the opposing team, LeBlanc and her clone look exactly the same, including copies of any external buffs and item effects active at the moment the clone is spawned (Prior to patch V5.22, clones did not duplicate external buff/active item effect particles).
  - Four other champions with this feature are **Wukong** (via *Warrior Trickster), cisHallucinate*), **Nasus** (via *Fury of the Sands*) and **Malphite** (via *Granite Shield* and *Thunderclap*).
- The Black Rose might have been inspired by and/or might be referencing the Black Hand.
- Her staff appears to be inspired, partially or in full, by that of Tsukasa from .hack//SIGN.
- The name 'LeBlanc' may be a reference to Maurice Leblanc, creator of the character Arsène Lupin. Like Lupin, part of LeBlanc's character is that of a master of disguise.
- For a very short time while her 2025 Visual Update was on PBE (patch V25.07), LeBlanc was illegally able to cast her *Sigil of Malice* and *Ethereal Chains* during dash.

---
*This page was automatically generated from League of Legends Wiki data.*