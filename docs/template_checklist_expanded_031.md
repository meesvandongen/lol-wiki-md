# Template checklist — expanded

Generated: 2025-10-10T15:54:19.275Z

Batch 31 of 33 — items 601..620

## Expansion infobox

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AExpansion%20infobox%2Fdoc.txt
- Decoded name: Template:Expansion infobox/doc
- Namespace: Template
- Remainder: Expansion infobox/doc
- Path parts: Template:Expansion infobox / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Infobox for a {{tip|Legends of Runeterra}} expansion or {{tip|Teamfight Tactics}} set.
: The game must be specified to use some functionality.
:: For <code>game=Legends of Runeterra</code>: Adds a hatnote that links to the list of cards from the expansion based on the format <code>LoR:List of cards from <set name></code>. Adds the article under [[:Category:LoR Sets]].
:: For <code>game=Teamfight Tactics</code>: Adds a hatnote that links to the list of champions from the set based on the format <code>TFT:List of champions in <set name></code>. Adds the article under [[:Category:TFT set]].

;GalleryHelper information
: ''This template uses [[Template:GalleryHelper|GalleryHelper]].''
* Only uses 1 instance: for title art.
** Image parameter(s): <code>image</code> '''or''' <code>gallery</code>
** Additional caption parameter: (none)
** Placeholder image: (none)

== LoR example syntax ==
{{Expansion infobox
  | game=Legends of Runeterra
  | expansion=Empires of the Ascended
  | set= Empires of the Ascended
```

### Notes

_No notes specified._

---

## Game banner

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AGame%20banner%2Fdoc.txt
- Decoded name: Template:Game banner/doc
- Namespace: Template
- Remainder: Game banner/doc
- Path parts: Template:Game banner / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Displays the banner of a given champion or character, with links to their appearance in the games of the Runeterra universe.

;Notes
: This banner only checks for and links to /Background, /LoL, /TFT, /LoR, /WR, /PF, /PL subpages.
: /TFT subpages will automatically use the latest set's skin for the background.
:

;Syntax
: <code>{{t|Game banner|Champion}}</code>
: <code>{{t|Game banner|Champion|Skin|background|offset|active}}</code>

;Parameters
: '''1''' - Required; champion or character
: '''2''' - Optional; skin, defaults to original
: '''background''' - Optional; override background image
: '''offset''' - Optional; override background image's CSS
: '''active''' - Optional; override which navigation icon has an outline
```

### Notes

_No notes specified._

---

## Game navigation

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AGame%20navigation%2Fdoc.txt
- Decoded name: Template:Game navigation/doc
- Namespace: Template
- Remainder: Game navigation/doc
- Path parts: Template:Game navigation / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Introduction
* [[Template:Game navigation]] is '''not''' meant to be used directly on pages.
* Only call the template via '''[[Template:Article game navigation]]''' for convenience.
* [[Template:Game banner]] can also be used, but it is currently an obsolete implementation so the one above is encouraged.

;Description
: Creates "content island" buttons on the top left, above the page, for pages that have multiple appearances within [[Riot Games]] titles (or otherwise exist in various sections of the LoL Wiki).
: Pages must include this template on each interconnected article in order for all pages to display all titles' buttons properly.
: This template will check for pages using a standard link structure as explained in the "Link structure" section below. If a page is using a non-standard link structure in a certain namespace or a different link is desired within the same namespace, use overrides.

;Syntax
: {{t|Article game navigation|1|override A|override B|...}}
* '''1 (First parameter)''' - Optional; changes the base page (the top-level mainspace page) from the default League of Legends gameplay article. to use is not the {{tip|League of Legends}} one, changes the base page name found in the main namespace. Otherwise, defaults to the standard League of Legends gameplay article.
: '''universe''' - Optional; override Universe link
: '''lol''' - Optional; override {{tip|League of Legends}} gameplay link
: '''tft''' - Optional; override {{tip|Teamfight Tactics}} gameplay link
: '''lor''' - Optional; override {{tip|Legends of Runeterra}} gameplay link
: '''wr''' - Optional; override {{tip|Wild Rift}} gameplay link
```

### Notes

_No notes specified._

---

## Grouped ability

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AGrouped%20ability%2Fdoc.txt
- Decoded name: Template:Grouped ability/doc
- Namespace: Template
- Remainder: Grouped ability/doc
- Path parts: Template:Grouped ability / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
Displays two abilities of champions as a group of 2 abilities, provided that they are both listed in the same slot (QWER).

;Syntax
:<code>{{t|Grouped ability|champion|skill_letter}}</code>

;Example
:<code><nowiki>{{Grouped ability|Lee Sin|Q}}</nowiki></code>
{{Grouped ability|Lee Sin|Q}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Ability templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Grouped stat table

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AGrouped%20stat%20table%2Fdoc.txt
- Decoded name: Template:Grouped stat table/doc
- Namespace: Template
- Remainder: Grouped stat table/doc
- Path parts: Template:Grouped stat table / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Retrieves champion data from [[Module:ChampionData/data]] and formats the data into a table with champions grouped into rows based on the specified datatype/stat. Champion entries can be modified, and custom entries can be added to the table.

;Notes
:Champion entries
* Only released champions are displayed.
* Modifiers and notes will automatically attach to champion entries based on data from [[Module:ChampionData/data]].
* Modifiers add text next to the champion entry, using the text provided
* Notes add a <sup>note</sup> texttip next to the champion entry, using the text provided

:Custom Entries
* Custom entries must be manually updated.
* Custom entries are sorted into the table automatically. The parameter numbers do not influence the order they appear in the table.
* Don't skip numbers. If a number is skipped, all following custom entries will not show up.
* Only one entry will be displayed per champion per row/stat value.
** Custom entries have priority over champion data from [[Module:ChampionData/data]].
* (A value and a champion) or (a value and a modifier) are required for a valid entry.

```

### Notes

_No notes specified._

---

## Infobox alternate universe

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20alternate%20universe%2Fdoc.txt
- Decoded name: Template:Infobox alternate universe/doc
- Namespace: Template
- Remainder: Infobox alternate universe/doc
- Path parts: Template:Infobox alternate universe / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: Infobox for Skin Universes. Also see: [[:Category:Lore]], [[:Category:Alternate Universe]].
: Adds the article into the "Alternate Universe" category by default. This can be changed.

;Syntax
: Add the word "Unknown" to fields which ''are'' relevant to the AU, but currently unknown or unconfirmed.
<pre>
{{Infobox alternate universe
|name= Defaults to the page's name
|(GalleryHelper parameters)=
|alias= 
|by= Narrative director
|history= Establishment history
|release= Event years
|universe= If part of a super-set of skins
|related= List of related characters separated by comma plus a space
|maincategory= Add this parameter and input the type of universe to override the "Alternate Universe" category. For example, "Runeterra" will add the article into the "Runeterra" category, in case the skin set is not in an AU but part of canon lore.
```

### Notes

_No notes specified._

---

## Infobox buff

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20buff%2Fdoc.txt
- Decoded name: Template:Infobox buff/doc
- Namespace: Template
- Remainder: Infobox buff/doc
- Path parts: Template:Infobox buff / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: Infobox to present [[buff]]s in a standardized format. Can fill the infobox with data from [[:Category:Buff data templates]] if it exists and the proper name is provided.

;Syntax
* Add the parameter "nouniticon=true" to remove [[:Template:Unit icon]] from "source".
<pre>
{{Infobox buff|Name of buff
|disp_name= (override title name)
|image= (file name of icon)
|description= text
|description2= text
|duration= (number of seconds)
|source= (name of source is placed in in [[Template:Unit icon]])
|nouniticon= 
|affect= "single" or "team"
|(GalleryHelper parameters)=
}}
```

### Notes

_No notes specified._

---

## Infobox champion

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20champion%2Fdoc.txt
- Decoded name: Template:Infobox champion/doc
- Namespace: Template
- Remainder: Infobox champion/doc
- Path parts: Template:Infobox champion / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Displays the render, title, release date, last changed (patch), classes, positions, resource, range type, adaptive type, store price, crafting, ratings, style, difficulty, and champion spotlight of a given champion.
: All data is retrieved from [[Module:ChampionData/data]] with the help of [[Template:Current champion data]].

;Syntax
: <code>{{t|Infobox champion|Champion}}</code>
{{Infobox champion|Garen}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Infobox templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Infobox company

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20company%2Fdoc.txt
- Decoded name: Template:Infobox company/doc
- Namespace: Template
- Remainder: Infobox company/doc
- Path parts: Template:Infobox company / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Infobox about real-life legal entities and brands.

;Syntax
: Date format should be: Full month word {{tl|NumberSup|Day number}}, Year number
:: e.g. October {{NumberSup|16}}, 2017
<pre>
{{Infobox company
|title= Override the default title provided by the page title
|(GalleryHelper parameters)=
|type=
|industry=
|founded=
|founders=
|headquarters=
|locations=
|key_people=
|products=
```

### Notes

_No notes specified._

---

## Infobox faction

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20faction%2Fdoc.txt
- Decoded name: Template:Infobox faction/doc
- Namespace: Template
- Remainder: Infobox faction/doc
- Path parts: Template:Infobox faction / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: Infobox for regions, factions, clans, etc. See:
* [[:Category:Subpolitical factions]]
* [[:Category:Independent factions]]

;Syntax
: Add the word "Unknown" to fields which ''are'' relevant to the faction, but currently unknown or unconfirmed.
<pre>
{{Infobox faction
|name= Formal name of the faction.
|(GalleryHelper parameters)=
|leader= Name of the leader(s)
|type= e.g. Tribe, Military, Ruling Body, Nobility, etc.
|status= The status of the faction. Defaults to 'Active'.
|purpose= What is the goal of the faction or what function does it provide to the society/region it is part of.
|parent= (1) For a subpolitical faction: the geopolitical faction it is bound to (2) For an independent faction: this should be left unassigned.
|related= Related factions (excluding its parent, if applicable).
```

### Notes

_No notes specified._

---

## Infobox game

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20game%2Fdoc.txt
- Decoded name: Template:Infobox game/doc
- Namespace: Template
- Remainder: Infobox game/doc
- Path parts: Template:Infobox game / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:Infobox for all [[:Category:Games|games]] which includes minigames, browser games, metagames, etc.

;Syntax & Example
{{infobox game
| name         = Game name
| gallery      = {{GalleryHelper
    |League of Legends Cover.jpg¦Title card¦
    |File:LoL_Icon.png¦Icon¦
    }}
| developer    = Developer name
| publisher    = Publisher name
| release      = Release date 
| platform     = O.S.
| genre        = Genre of the Game
| modes        = Modes
| media        = Format
```

### Notes

_No notes specified._

---

## Infobox historical event

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20historical%20event%2Fdoc.txt
- Decoded name: Template:Infobox historical event/doc
- Namespace: Template
- Remainder: Infobox historical event/doc
- Path parts: Template:Infobox historical event / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: Infobox for historical events. See: [[Universe:Timeline|Timeline]].

;Syntax
: Add the word "Unknown" to fields which ''are'' relevant to the event, but currently unknown or unconfirmed.
<pre>
{{Infobox historical event|Name of event
|name= Override the name provided by the page title
|(GalleryHelper parameters)=
|year= Timeline year
|prev= Events that preceded this one in the timeline
|next= Events that succeeded this one in the timeline
|main= If part of a larger event, list it here
|parallel= Related events that took place at the same time in the timeline
|subs=
|included= Links to Universe lore on the wiki and/or other media 
}}
```

### Notes

_No notes specified._

---

## Infobox item old

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20item%20old%2Fdoc.txt
- Decoded name: Template:Infobox item old/doc
- Namespace: Template
- Remainder: Infobox item old/doc
- Path parts: Template:Infobox item old / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
{{infobox item old
|name    = Bob's Petal
|image   = Placeholder.svg
|caption = Bob the Flower
|type    = Basic
|effects = Adds +1 Ability Power
|passive = 10% cooldown reduction
|buy     = 350g
|sell    = 175g
|code    = 6847
}}
;Description
:This template is used to create an item infobox.
;Syntax
:Type <code>{{t|infobox item old|...}}</code> somewhere, with parameters as shown below.
;Sample output

<pre>{{infobox item old
```

### Notes

_No notes specified._

---

## Infobox item/new

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20item%2Fnew%2Fdoc.txt
- Decoded name: Template:Infobox item/new/doc
- Namespace: Template
- Remainder: Infobox item/new/doc
- Path parts: Template:Infobox item / new / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
: ''For the {{tip|Wild Rift}} template, see [[Template:WR Infobox item/new]].''
{{Infobox item/new|Rabadon's Deathcap}}
;Description
: This template adds the given item's data from [[Module:ItemData/data]] into the [[Template:Infobox item/new/var]] infobox template for items.

;Syntax
: <code>{{tl|Infobox item/new|item}}</code>

;Example
:<code>{{tl|Infobox item/new|Rabadon's Deathcap}}</code>

;See also
* [[Module:ItemData]]
* [[Template:Infobox item/new/var]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Infobox templates]]
```

### Notes

_No notes specified._

---

## Infobox item/new/var

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20item%2Fnew%2Fvar%2Fdoc.txt
- Decoded name: Template:Infobox item/new/var/doc
- Namespace: Template
- Remainder: Infobox item/new/var/doc
- Path parts: Template:Infobox item / new / var / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Infobox for items in League of Legends and Wild Rift.

;Syntax
: <code>{{tl|Infobox item/new/var|item|item parameters}}</code>
: All parameters must have parity with [[Module:ItemData]] for League of Legends and [[Module:WRItemData]] for Wild Rift.

;See also
* [[Template:Infobox item/new]]
* [[Template:WR Infobox item/new]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Infobox templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Infobox legend

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20legend%2Fdoc.txt
- Decoded name: Template:Infobox legend/doc
- Namespace: Template
- Remainder: Infobox legend/doc
- Path parts: Template:Infobox legend / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Infobox for [[TFT:Tactician|Tactician]] ([[TFT:Little Legend|Little Legends]], [[TFT:Chibi Champions|Chibi]] and [[TFT:Unbound Champions|Unbound]]) in {{tip|Teamfight Tactics}}.
: Adds the article into the category of the Little Legend's name as well as the general [[:Category:Little Legend]].

;Syntax
<pre>
{{Infobox legend
    |name=Example
    |(GalleryHelper parameters)=
    |description=Example
    |cost=Example         --- defaults to "750 RP"
}}
</pre>

;GalleryHelper information
: ''This template uses [[Template:GalleryHelper|GalleryHelper]].''
* Only uses 1 instance: usually, for the profile icon depicting the Little Legend.
** Image parameter(s): <code>image</code>
```

### Notes

_No notes specified._

---

## Infobox lore concept

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20lore%20concept%2Fdoc.txt
- Decoded name: Template:Infobox lore concept/doc
- Namespace: Template
- Remainder: Infobox lore concept/doc
- Path parts: Template:Infobox lore concept / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: Infobox for presenting client-related concepts, such as thematic mechanics, systems, or narrative elements used across articles.
: Also adds the article into the following category: [[:Category:Lore concepts]]

;Syntax
<pre>{{Infobox lore concept
|name= Override the name provided by the page title
|(GalleryHelper parameters)=
|nature= What type of concept it is
|affiliations= Other concepts or entities it's associated with
</pre>

;GalleryHelper information
: ''This template uses [[Template:GalleryHelper|GalleryHelper]].''
* Only uses 1 instance: for example image(s) of the lore concept and related concept art.
** Image parameter(s): <code>image</code> '''or''' <code>gallery</code>
** Additional caption parameter: <code>caption</code>
```

### Notes

_No notes specified._

---

## Infobox mastery

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20mastery%2Fdoc.txt
- Decoded name: Template:Infobox mastery/doc
- Namespace: Template
- Remainder: Infobox mastery/doc
- Path parts: Template:Infobox mastery / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

==Usage==
* <code>{{t|Infobox mastery|Warlord's Bloodlust}}</code>
{{Infobox mastery|Warlord's Bloodlust}}{{clr}}
* <code>{{t|Infobox mastery|Thunderlord's Decree}}</code>
{{Infobox mastery|Thunderlord's Decree}}{{clr}}
* <code>{{t|Infobox mastery|Bond of Stone}}</code>
{{Infobox mastery|Bond of Stone}}

<!-- Categories and interwikis go here: -->
<includeonly>[[Category:Mastery data templates]]</includeonly>
```

### Notes

_No notes specified._

---

## Infobox minor character

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20minor%20character%2Fdoc.txt
- Decoded name: Template:Infobox minor character/doc
- Namespace: Template
- Remainder: Infobox minor character/doc
- Path parts: Template:Infobox minor character / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: Infobox for minor character entries in [[Universe:Minor Characters|Minor Characters]] subpages.

;Syntax
<pre>
{{Infobox minor character|Lem Ajuma|image=01PZ034-full.png
|species = [[Human]]
|death = Unknown
|related = {{cbi|Ekko}}
|references = {{LoR|Sumpsnipe Scavenger}}{{,}}[[Seconds]]{{,}}[[Chronobreak]]
|background = Lem was one of {{cbis|Ekko}} friend. He was killed by a {{fi|Piltover|Piltovan}} noble.
|univ = {{univ|Runeterra Prime}}
}}
</pre>
creates:
{{Infobox minor character|Lem Ajuma|image=01PZ034-full.png
|species = [[Human]]
```

### Notes

_No notes specified._

---

## Infobox patch

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AInfobox%20patch%2Fdoc.txt
- Decoded name: Template:Infobox patch/doc
- Namespace: Template
- Remainder: Infobox patch/doc
- Path parts: Template:Infobox patch / doc

```
{{Documentation subpage}}
__NOTOC__
;Description
: Infobox for [[patch]]es of any game, in a standardized format.

;Syntax
: Date format should be: Full month word {{tl|NumberSup|Day number}}, Full year number
:: e.g. December {{NumberSup|3}}, 2024
<pre>{{Infobox patch
|Title        = (Full major version name on the Wiki, defaults to page name)
|(GalleryHelper parameters)=
|Caption      = (Patch subtitle/flavor text)
|Release      = (Release date)
|Highlights   = (Brief list of the important points of the patch)
|Related      = (Add related links)
|Prev         = (Previous patch version)
|Next         = (Next patch version)
}}</pre>

;GalleryHelper information
```

### Notes

_No notes specified._

---
