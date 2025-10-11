# Template checklist — expanded

Generated: 2025-10-10T15:54:19.146Z

Batch 5 of 33 — items 81..100

## Champion mastery

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AChampion%20mastery%2Fdoc.txt
- Decoded name: Template:Champion mastery/doc
- Namespace: Template
- Remainder: Champion mastery/doc
- Path parts: Template:Champion mastery / doc

```
{{Documentation subpage}}

;Usage
:A template to display [[Champion Mastery]] in a similar manner to the client. Also displays mastery score as a [[Template:Tooltip|tooltip]] if mastery score is used as an argument.

;Syntax
{{t|Champion mastery|Champion|Mastery Level (Optional)|Mastery Score (Optional)|skin{{=}}Skin (Optional)}}

Examples:
<pre>{{Champion Mastery|Aatrox}}</pre>
{{Champion mastery|Aatrox}}

<pre>{{Champion mastery|Ashe|0}}</pre>
{{Champion mastery|Ashe|0}}

<pre>{{Champion mastery|Ryze|3}}</pre>
{{Champion mastery|Ryze|3}}

<pre>{{Champion mastery|Blitzcrank|5|30000}}</pre>
{{Champion mastery|Blitzcrank|5|30000}}
```

### Notes

Mastery icon/tooltip — include as inline helper.

---

## Champion roster

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AChampion%20roster%2Fdoc.txt
- Decoded name: Template:Champion roster/doc
- Namespace: Template
- Remainder: Champion roster/doc
- Path parts: Template:Champion roster / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:Uses the getRoster function from [[Module:ChampionData]] to create a centered table the entire [[League of Legends]] roster, currently consisting of {{cmd|TotalReleasedChampions}} champions.
:Related JS: [[MediaWiki:Gadget-gridfiltering.js]]

;Usage
:{{tl|Champion roster}}
{{Champion roster}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Portal templates]]
</includeonly>
```

### Notes

Roster grid/navigation — exclude.

---

## Champion rotation archive

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AChampion%20rotation%20archive%2Fdoc.txt
- Decoded name: Template:Champion rotation archive/doc
- Namespace: Template
- Remainder: Champion rotation archive/doc
- Path parts: Template:Champion rotation archive / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
This template can be used to archive past free champion rotations. It works with 10 or 14 champions.
*The arguments <code>startdate</code> and <code>enddate</code> can be added if the "season" automatical date from [[Template:Current maintenance data]] isn't working.
*The parameter <code>notes</code> can be used to add extra information.

;Syntax
<pre>{{cra
|startdate = 
|enddate   = 
|notes     = 
|season    = 
|week      = 
|ref       = 
|champion name|champion name|...|champion name}}</pre>

;Examples
<pre>{{cra
|startdate = 18.02.2018
```

### Notes

Rotation archive template — include (historical data, low prio).

---

## Champion skin link

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AChampion%20skin%20link%2Fdoc.txt
- Decoded name: Template:Champion skin link/doc
- Namespace: Template
- Remainder: Champion skin link/doc
- Path parts: Template:Champion skin link / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
Create an in-text icon-link with tooltip that displays splash art pictures of a champion along with additional details about the latest iteration.

* Tooltip invoked from [[Module:ImageLink]]
* Tooltip design is located at [[Template:Tooltip/Skin/Lol]]
* Skin data stored at [[Module:SkinData/data]]

;Syntax
<code>{{t|csl|Champion|Skin name|optional displayed name}}</code><br> (circle=true by default)

;Examples
<code>{{tl|csl|Taric}}</code>
* {{csl|Taric}}
<code>{{tl|csl|Taric|circle{{equals}}false}}</code>
* {{csl|Taric|circle=false}}
<code>{{tl|csl|Taric|Original}}</code>
* {{csl|Taric|Original}}
```

### Notes

Icon-link + tooltip for skins — include.

---

## Champion skin trivia

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AChampion%20skin%20trivia%2Fdoc.txt
- Decoded name: Template:Champion skin trivia/doc
- Namespace: Template
- Remainder: Champion skin trivia/doc
- Path parts: Template:Champion skin trivia / doc

```
{{Documentation subpage}}

== Usage ==
A [[Template:Champion skin link]] version that should only be used on champion trivia pages. By default it has <code>{{t|csl|Champion|Skin name|32px|chromas{{equals}}true}}</code> plus a [[Template:Set]] added after it.

== Syntax ==
<code>{{t|cst|Champion|Skin name}}</code>

;Example
<code>{{tl|cst|Aatrox|Mecha}}</code>

{{cst|Aatrox|Mecha}}

<code>{{tl|cst|Aatrox|Justicar}}</code>

{{cst|Aatrox|Justicar}}

<code>{{tl|cst|Illaoi|Void Bringer}}</code>

{{cst|Illaoi|Void Bringer}}
```

### Notes

Trivia-specific skin link variant — include (lower prio).

---

## Champion skins by release cycle TOC

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AChampion%20skins%20by%20release%20cycle%20TOC%2Fdoc.txt
- Decoded name: Template:Champion skins by release cycle TOC/doc
- Namespace: Template
- Remainder: Champion skins by release cycle TOC/doc
- Path parts: Template:Champion skins by release cycle TOC / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
:Category description header.

;Syntax
:{{tl|Champion skins_by_release_season_TOC}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Navigation templates]]
</includeonly>
```

### Notes

Navigation/TOC — exclude.

---

## Champion spotlight line

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AChampion%20spotlight%20line%2Fdoc.txt
- Decoded name: Template:Champion spotlight line/doc
- Namespace: Template
- Remainder: Champion spotlight line/doc
- Path parts: Template:Champion spotlight line / doc

```
{{Documentation subpage}}

== Usage ==
Used to structure the table used on [[Champion Spotlight]].

== Syntax ==
:<code>{{t|champion spotlight line|champion|link|date}}</code>

The four parameters are:
* champion
* link: use the video identifier, the portion of the URL after "?v="
* date: in yyyy-mm-dd format

== See also ==

<includeonly>
[[Category:General wiki templates]]
</includeonly>
```

### Notes

Line used on Champion Spotlight pages — include (low prio).

---

## Champion style

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AChampion%20style%2Fdoc.txt
- Decoded name: Template:Champion style/doc
- Namespace: Template
- Remainder: Champion style/doc
- Path parts: Template:Champion style / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Creates the layout for a champion's attack-ability style in [[Template:Infobox champion]].

;Syntax
: {{t|Champion style|Positive integer}}
: Number should be between 0 to 100 for proper display.

;Examples
:{{tl|Champion style|0}}
::{{Champion style|0}}

:{{tl|Champion style|10}}
:: {{Champion style|10}}

:{{tl|Champion style|25}}
:: {{Champion style|25}}

:{{tl|Champion style|40}}
```

### Notes

Style layout for attack/ability style — include as formatting.

---

## Champion with infinite scaling

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AChampion%20with%20infinite%20scaling%2Fdoc.txt
- Decoded name: Template:Champion with infinite scaling/doc
- Namespace: Template
- Remainder: Champion with infinite scaling/doc
- Path parts: Template:Champion with infinite scaling / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
Shows the champions that have an ability that infinitely scales an effect in a comma-separated list. See [[:Category:Champion with infinite scaling]].

;Syntax
* For showcasing a specific champion, add their name as the first parameter.
<pre>
{{Champion with infinite scaling|Nasus}}
</pre>
{{Champion with infinite scaling|Nasus}}
<br><br>
* For simply showing the list of related champions, do not add any parameters.
<pre>
{{Champion with infinite scaling}}
</pre>
{{Champion with infinite scaling}}

<includeonly>
<!-- Categories and interwikis go here: -->
```

### Notes

Lists champions with infinite-scaling abilities — include.

---

## Champion with infinite scaling

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AChampion%20with%20infinite%20scaling%2Fdoc.txt
- Decoded name: Template:Champion with infinite scaling/doc
- Namespace: Template
- Remainder: Champion with infinite scaling/doc
- Path parts: Template:Champion with infinite scaling / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
Shows the champions that have an ability that infinitely scales an effect in a comma-separated list. See [[:Category:Champion with infinite scaling]].

;Syntax
* For showcasing a specific champion, add their name as the first parameter.
<pre>
{{Champion with infinite scaling|Nasus}}
</pre>
{{Champion with infinite scaling|Nasus}}
<br><br>
* For simply showing the list of related champions, do not add any parameters.
<pre>
{{Champion with infinite scaling}}
</pre>
{{Champion with infinite scaling}}

<includeonly>
<!-- Categories and interwikis go here: -->
```

### Notes

Duplicate entry (same) — include.

---

## Champion without ability power ratio

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AChampion%20without%20ability%20power%20ratio%2Fdoc.txt
- Decoded name: Template:Champion without ability power ratio/doc
- Namespace: Template
- Remainder: Champion without ability power ratio/doc
- Path parts: Template:Champion without ability power ratio / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
Shows the champions that do not have any ability power ratio on their abilities in a comma-separated list. See [[:Category:Champion without ability power ratio]].

;Syntax
<pre>
{{Champion without ability power ratio|Darius}}
</pre>
{{Champion without ability power ratio|Darius}}


<pre>
{{Champion without ability power ratio}}
</pre>
{{Champion without ability power ratio}}

;See also
[[Template:Champion with infinite scaling|Champion with infinite scaling]]

```

### Notes

Lists champions without AP ratios — include.

---

## Champions by release cycle TOC

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AChampions%20by%20release%20cycle%20TOC%2Fdoc.txt
- Decoded name: Template:Champions by release cycle TOC/doc
- Namespace: Template
- Remainder: Champions by release cycle TOC/doc
- Path parts: Template:Champions by release cycle TOC / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
:Category description header.

;Syntax
:{{tl|Champions_by_release_season_TOC}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Navigation templates]]
[[de:Template:Champions nach Veröffentlichung Saison TOC]]
</includeonly>
```

### Notes

Navigation TOC — exclude.

---

## Championtip

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AChampiontip%2Fdoc.txt
- Decoded name: Template:Championtip/doc
- Namespace: Template
- Remainder: Championtip/doc
- Path parts: Template:Championtip / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Usage
{{t|Championtip|champion}}

;Examples
:{{tl|Championtip|Ahri}}
{{Championtip|Ahri}}

:{{tl|Championtip|Quinn}}
{{Championtip|Quinn}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Tooltip templates]]
[[de:Vorlage:Championtip]]
</includeonly>
```

### Notes

Champion tooltip helper — include.

---

## Channel type

- [x] Status: done
- [x] Include decision: include

### Source (first 20 lines)

- Source filename: Template%3AChannel%20type%2Fdoc.txt
- Decoded name: Template:Channel type/doc
- Namespace: Template
- Remainder: Channel type/doc
- Path parts: Template:Channel type / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
Displays channel specifications. May also be used for any other lockout. This template allows to have default variables that are used on [[Template:Channel type table]].

;Default types
* Cast (fixed delay abilities)
* Channel (one-cast, can be interrupted by the enemy)
* Charge("two-cast" and "hold-then-release" abilities, can be interrupted by the enemy)

;Example
<pre>{{ct|cast}}</pre>creates: {{ct|cast}}


;Variables
{| class="article-table"
! Name
! Accepted parameters
! Default parameters<br/>(based on {{{1}}})
! Notes
```

### Notes

Ability/channel type definitions — include as metadata helper.

---

## Cancelled champion

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACancelled%20champion%2Fdoc.txt
- Decoded name: Template:Cancelled champion/doc
- Namespace: Template
- Remainder: Cancelled champion/doc
- Path parts: Template:Cancelled champion / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
This template was created for cancelled champions, rather than using the custom champion template.

;GalleryHelper information
: ''This template uses [[Template:GalleryHelper|GalleryHelper]].''
* Only uses 1 instance: for the champion's icon, render, or other identification images like primary concept art.
** Image parameter(s): <code>image</code> '''or''' <code>gallery</code>
** Additional caption parameter: (none)
** Placeholder image: [[:File:Champion Render.png]]

;Syntax & Example
 <nowiki>{{Cancelled champion
| name        = Averdrian
| title       = The Astral Guardian
| (GalleryHelper parameters)=
| herotype    = Mage
| alttype     = 
```

### Notes

_No notes specified._

---

## Canonical inconsistencies

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACanonical%20inconsistencies%2Fdoc.txt
- Decoded name: Template:Canonical inconsistencies/doc
- Namespace: Template
- Remainder: Canonical inconsistencies/doc
- Path parts: Template:Canonical inconsistencies / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Capital

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACapital%2Fdoc.txt
- Decoded name: Template:Capital/doc
- Namespace: Template
- Remainder: Capital/doc
- Path parts: Template:Capital / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
"Capital city" star.

{{Math symbols}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Character formatting templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Cbis

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACbis%2Fdoc.txt
- Decoded name: Template:Cbis/doc
- Namespace: Template
- Remainder: Cbis/doc
- Path parts: Template:Cbis / doc

```
{{Documentation subpage}}
;Description
:This template shortcuts usage of {{t|character background icon}} when using possessive apostrophes.

;Syntax
:Type <code>{{t|cbis|Champion}}</code> instead of <code>{{t|cbi|Champion|Champion's}}</code> at any part of the article.

:Correct:
:<code>{{tl|cbis|Cithria}}</code>
:{{cbis|Cithria}}
:<code>{{tl|cbis|Nasus}}</code>
:{{cbis|Nasus}}

:Wrong:
:<code>{{tl|cbi|Cithria}}'s</code>
:{{cbi|Cithria}}'s
:<code>{{tl|cbi|Nasus}}'</code>
:{{cbi|Nasus}}'

<includeonly>[[Category:Icon templates]]
```

### Notes

_No notes specified._

---

## CC-BY-SA

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACC-BY-SA%2Fdoc.txt
- Decoded name: Template:CC-BY-SA/doc
- Namespace: Template
- Remainder: CC-BY-SA/doc
- Path parts: Template:CC-BY-SA / doc

```
{{Documentation subpage}}
;Description
:This template is used to mark images with the [http://creativecommons.org/licenses/by-sa/3.0/ CC-BY-SA-3.0] license.
;Syntax
:Type <code>{{t|cc-by-sa-3.0|attribution details}}</code> on the image information page.
:Replace "<code>attribution details</code>" with information about the source.

<includeonly>[[Category:Image wiki templates|{{PAGENAME}}]]</includeonly>
<noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Cc-by-sa-3.0

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ACc-by-sa-3.0%2Fdoc.txt
- Decoded name: Template:Cc-by-sa-3.0/doc
- Namespace: Template
- Remainder: Cc-by-sa-3.0/doc
- Path parts: Template:Cc-by-sa-3.0 / doc

```
{{Documentation subpage}}
;Description
:This template is used to mark images with the [http://creativecommons.org/licenses/by-sa/3.0/ CC-BY-SA-3.0] license.
;Syntax
:Type <code>{{t|cc-by-sa-3.0|attribution details}}</code> on the image information page.
:Replace "<code>attribution details</code>" with information about the source.

<includeonly>[[Category:Image wiki templates|{{PAGENAME}}]]</includeonly><noinclude></noinclude>
```

### Notes

_No notes specified._

---
