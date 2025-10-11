# Template checklist — expanded

Generated: 2025-10-10T15:54:19.213Z

Batch 19 of 33 — items 361..380

## Mythic content table

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AMythic%20content%20table%2Fdoc.txt
- Decoded name: Template:Mythic content table/doc
- Namespace: Template
- Remainder: Mythic content table/doc
- Path parts: Template:Mythic content table / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Standardized table for showcasing mythic/prestige/exalted content in related pages.

;Syntax
: <nowiki>{{Mythic content table|1|2|collapse|tablewidth}}</nowiki>
* '''1''' or '''title''' (Required): Table title (is always in bold).
* '''2''' (Required): Enter a new row of [[Template:Mythic content table row]], as many as needed.
* '''collapse''' (Optional): Set to 'no' to initialize the table expanded.
* '''tablewidth''' (Optional): Default width is 100%, but may be set to a different percentage.

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Table formatting templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Mythic content table row

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AMythic%20content%20table%20row%2Fdoc.txt
- Decoded name: Template:Mythic content table row/doc
- Namespace: Template
- Remainder: Mythic content table row/doc
- Path parts: Template:Mythic content table row / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: To be used in [[Template:Mythic content table]] only. Creates a row meant for that table specifically.
: Showcases a mythic/prestige/exalted skin as well as its related content in a colored border.

;Syntax
: <nowiki>{{Mctr|1|2|3|alt3|4|alt4|5|alt5|6|alt6|7|alt7|8|alt8|9|alt9|10|alt10|11|alt11|12|alt12|hd|univ|release|comment|bordersize|bordercolor|itemsize}}</nowiki>
* Content parameters
** '''1''' (Required): Champion name
** '''2''' (Required): Skin name
** '''3''', '''4''',… '''9''' (Optional): File name of the content to display. Do not include <code>File:</code>, but include the extension
** '''alt3''', '''alt4''',… '''alt9''' (Optional): Alternate text, or caption, of each file above (respectively as numbered, e.g. alt3 corresponds to parameter 3, alt4 to 4, and so on.)
** '''hd''' (Optional): Set this parameter to 'no' to generate a link to the standard-size splash art instead of the HD one which is the default. Ideally only use this if the HD splash is missing for a long period of time.
** '''univ''': Type the name of the skin set/theme to be placed inside a [[Template:Universe icon]].
** '''release''': Release date, ideally use {{w|ISO 8601}} since the original table is sortable.
** '''comment''' (Optional): Add a comment under the release date (e.g. name of Battle Pass)
* Formatting parameters
** ''All of the following parameters default to the standard layout but can optionally be modified per individual row added.''
** '''bordersize''': (<code>#px</code>) Change the thickness of the border for each item, same one for all items.
```

### Notes

_No notes specified._

---

## Mythic gold value

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AMythic%20gold%20value%2Fdoc.txt
- Decoded name: Template:Mythic gold value/doc
- Namespace: Template
- Remainder: Mythic gold value/doc
- Path parts: Template:Mythic gold value / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is used to display the gold value of {{ig|Mythic}} passives. The '''Total Gold Value''' is stored in the variabled called <code>totalmythic</code>.

;Syntax
: <code>{{t|mgv|stats}}</code>

;Example
: <code><nowiki>{{mgv|ad|3|ah|3|msflat|3}}</nowiki></code>
{{mgv|ad|3|ah|3|msflat|3}}

: <code><nowiki>{{mgv|armpen|5|mpen|5}}</nowiki></code>
{{mgv|armpen|5|mpen|5}}

: <code><nowiki>{{mgv|Divine Sunderer|desc=true}}</nowiki></code>
{{mgv|Divine Sunderer|desc=true}}

<includeonly>
<!-- Categories and interwikis go here: -->
```

### Notes

_No notes specified._

---

## Named item effect

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ANamed%20item%20effect%2Fdoc.txt
- Decoded name: Template:Named item effect/doc
- Namespace: Template
- Remainder: Named item effect/doc
- Path parts: Template:Named item effect / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Puts a link to the [[named item effect]] page with the corresponding item effect. Add <code>wr=true</code> to link to the Wild Rift version of the page.

;Syntax
: <code>{{t|nie|named item effect|alt}}</code>

;Example
: <code>{{tl|nie|Mana Charge}}</code>
:: {{nie|Mana Charge}}

: <code>{{tl|nie|Anti-CC|Quicksilver}}</code>
:: {{nie|Anti-CC|Quicksilver}}

: <code><nowiki>{{nie|Energized|wr=true}}</nowiki></code>
:: {{nie|Energized|wr=true}}

;See also
* [[Template:Item group]]
```

### Notes

_No notes specified._

---

## Namespace detect

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ANamespace%20detect%2Fdoc.txt
- Decoded name: Template:Namespace detect/doc
- Namespace: Template
- Remainder: Namespace detect/doc
- Path parts: Template:Namespace detect / doc

```
{{documentation subpage}}
<!-- PLEASE ADD CATEGORIES AND INTERWIKIS AT THE BOTTOM OF THIS PAGE -->

This is the {{tl|namespace detect}} meta-template.

It helps other templates detect what type of page they are on.

It detects and groups all the different [[Help:Namespace|namespaces]] used on League of Legends Wiki into several types:

:'''main''' = Main/article space, as in normal Wikipedia articles.
:'''talk''' = Any talk space, such as page names that start with "Talk:", "User talk:", "File talk:" and so on.
:'''user, project, file, mediawiki, template, help, category''' = The other namespaces except the talk pages.
:'''other''' = Any namespaces that were not specified as a parameter to the template. See explanation below.

For backwards compatibility this template also understands the old name '''image''' for '''file'''. But using '''image''' is now deprecated.

'''Note!''' For most usage cases it might be better to use the simpler namespace detection templates. (See the [[#See also|see also]] section below.) Since this template is more prone to human errors such as misspelling the parameter names. 

=== Usage ===
This template takes one or more parameters named after the different page types as listed above. Like this:
```

### Notes

_No notes specified._

---

## Namespace detect see also

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ANamespace%20detect%20see%20also%2Fdoc.txt
- Decoded name: Template:Namespace detect see also/doc
- Namespace: Template
- Remainder: Namespace detect see also/doc
- Path parts: Template:Namespace detect see also / doc

```
{{Documentation subpage}}
<!-- PLEASE ADD CATEGORIES AND INTERWIKIS AT THE BOTTOM OF THIS PAGE -->
This is the {{tl|namespace detect see also}} template.

It is used in the "See also" section of the documentation of the namespace-detection templates. It takes no parameters.

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Namespace manipulation templates‎|Namespace detect see also]]
[[Category:Template namespace templates]]

</includeonly>
```

### Notes

_No notes specified._

---

## Nearby locations

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ANearby%20locations%2Fdoc.txt
- Decoded name: Template:Nearby locations/doc
- Namespace: Template
- Remainder: Nearby locations/doc
- Path parts: Template:Nearby locations / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
This template creates a compass-like navigation section. 

;Syntax
{{t|Nearby locations|current location|map|NW|N|NE|W|E|SW|S|SE}}

;Copy and paste
*Version 1
<pre>{{Nearby locations|<!--defaults to "You are here"-->
  |N=
  |W=
  |E=
  |S=
}}
</pre>
*Version 2
<pre>{{Nearby locations|<!--defaults to "You are here"-->
  |map=<!--defaults to Runeterra-->
```

### Notes

_No notes specified._

---

## Neutral

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ANeutral%2Fdoc.txt
- Decoded name: Template:Neutral/doc
- Namespace: Template
- Remainder: Neutral/doc
- Path parts: Template:Neutral / doc

```
{{Documentation subpage}}
;Description
:This template is intended to provide quicker reference to the stance various participants in a discussion have in the matter at hand. It is in a separate capacity also a general template used in voting.
;Syntax
:Type <code><nowiki>{{Neutral}}</nowiki></code> on a discussion page.

<noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Neutral buff

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ANeutral%20buff%2Fdoc.txt
- Decoded name: Template:Neutral buff/doc
- Namespace: Template
- Remainder: Neutral buff/doc
- Path parts: Template:Neutral buff / doc

```
{{Documentation subpage}}
== Usage ==
Used as a navigation footer on neutral monster buff pages.

== Syntax ==
Type {{t|neutral buff}} at the bottom of the article.

<includeonly>
[[Category:Navigation templates]]
[[de:Vorlage:Navigation Neutrale Championverbesserung]]
</includeonly>
```

### Notes

_No notes specified._

---

## Nies

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ANies%2Fdoc.txt
- Decoded name: Template:Nies/doc
- Namespace: Template
- Remainder: Nies/doc
- Path parts: Template:Nies / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: [[Template:Named item effect]] that adds a possessive case suffix.

;Example
: <code><nowiki>{{nies|Mist's Edge}}</nowiki></code>
:: {{nies|Mist's Edge}} 

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Formatting templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Non-breaking hyphen

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ANon-breaking%20hyphen%2Fdoc.txt
- Decoded name: Template:Non-breaking hyphen/doc
- Namespace: Template
- Remainder: Non-breaking hyphen/doc
- Path parts: Template:Non-breaking hyphen / doc

```
{{Documentation subpage}}
;Description
Inserts a hyphen that will connect the two adjacent words together as-if a single word, preventing them from wrapping. Useful for when adjoining numbers in table-headers. 

;Syntax
{{t|nbh}} => {{nbh}}

;See also
{{Math symbols}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Character formatting templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Nosubst

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ANosubst%2Fdoc.txt
- Decoded name: Template:Nosubst/doc
- Namespace: Template
- Remainder: Nosubst/doc
- Path parts: Template:Nosubst / doc

```
{{Documentation subpage}}
This template is to be used in sigs to circumvent Subst.

Usage: You can decrease the formatting clutter your sig leaves behind on each page by doing these things:
# Make a sig page e.g. User:YourUserName/sig
# Insert your sig code into the page
# Save
# Under [[Special:preferences|your preferences]], change your code in the Signature box to: <nowiki>{{Subst:Nosubst|User:YourUserName/sig}}</nowiki> and remember to keep the box checked. (And Save)
<includeonly>[[Category:Article management templates|{{PAGENAME}}]]</includeonly>
<noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Not done

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ANot%20done%2Fdoc.txt
- Decoded name: Template:Not done/doc
- Namespace: Template
- Remainder: Not done/doc
- Path parts: Template:Not done / doc

```
{{Documentation subpage}}
;Description
:This template is to show the outcome of a consensus as not done.
;Syntax
:Type <code><nowiki>{{not done}}</nowiki></code> on a discussion page.
<includeonly>[[Category:Community templates]]</includeonly>
<noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Note

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ANote%2Fdoc.txt
- Decoded name: Template:Note/doc
- Namespace: Template
- Remainder: Note/doc
- Path parts: Template:Note / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
This template used for ease of highlighting the word '''Note''' with optional arguments to specify if it's editor's or developer's. Should be used mainly for generic info dumb, clarification, call out incorrectly stated in-game information, official confirmations preferably with citation, etc. (Wording needs to be cleaner here.)

;Syntax
{{t|Note}} or {{t|Note|Positive integer}}
* By default the template will add the wording Note:
* A small positive integer is optional and is used for adding specific types of notes.
** Entering <code>1</code> gives "Editor's note:"
** Entering <code>2</code> givess "Developer's note:"

;Example
<code>{{tl|Note}} Claritatem</code>
* {{Note}} Claritatem
<code>{{tl|Note|1}} I don't think they thought this through.</code>
* {{Note|1}} I don't think they thought this through.
<code>{{tl|Note|2}} "We had no idea what we did." - Riot X</code>
* {{Note|2}} "We had no idea what we did." - Riot X

```

### Notes

_No notes specified._

---

## NumberSup

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ANumberSup%2Fdoc.txt
- Decoded name: Template:NumberSup/doc
- Namespace: Template
- Remainder: NumberSup/doc
- Path parts: Template:NumberSup / doc

```
{{Documentation subpage}}
This template displays a number along with its matching superscripted postfix (<sup>st</sup>, <sup>nd</sup>, <sup>rd</sup> or <sup>th</sup>).

== Usage ==
* <tt><nowiki>{{NumberSup|1}}</nowiki></tt> produces {{NumberSup|1}}
* <tt><nowiki>{{NumberSup|11}}</nowiki></tt> produces {{NumberSup|11}}
* <tt><nowiki>{{NumberSup|23}}</nowiki></tt> produces {{NumberSup|23}}
* <tt><nowiki>{{NumberSup|42}}</nowiki></tt> produces {{NumberSup|42}}
* <tt><nowiki>{{NumberSup|29}}</nowiki></tt> produces {{NumberSup|29}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Formatting templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## OE

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AOE%2Fdoc.txt
- Decoded name: Template:OE/doc
- Namespace: Template
- Remainder: OE/doc
- Path parts: Template:OE / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce an [[Hextech Crafting|Orange Essence]] icon and the link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|OE|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|OE}}</code>
:* {{OE}}

:<code>{{tl|OE|100}}</code>
:* {{OE|100}}

:<code>{{tl|OE|100|3=size=30}}</code>
:* {{OE|100|size=30}}

<includeonly>
```

### Notes

_No notes specified._

---

## OG

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AOG%2Fdoc.txt
- Decoded name: Template:OG/doc
- Namespace: Template
- Remainder: OG/doc
- Path parts: Template:OG / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce an [[WR:Hextech Crafting|Orange Gemstone]] icon and the link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|OG|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|OG}}</code>
:* {{OG}}

:<code>{{tl|OG|100}}</code>
:* {{OG|100}}

:<code>{{tl|OG|100|3=size=30}}</code>
:* {{OG|100|size=30}}

<includeonly>
```

### Notes

_No notes specified._

---

## Old lore

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AOld%20lore%2Fdoc.txt
- Decoded name: Template:Old lore/doc
- Namespace: Template
- Remainder: Old lore/doc
- Path parts: Template:Old lore / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Oppose

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AOppose%2Fdoc.txt
- Decoded name: Template:Oppose/doc
- Namespace: Template
- Remainder: Oppose/doc
- Path parts: Template:Oppose / doc

```
{{Documentation subpage}}
;Description
:This template is intended to provide quicker reference to the stance various participants in a discussion have in the matter at hand. It is in a separate capacity also a general template used in voting.
;Syntax
:Type <code><nowiki>{{Oppose}}</nowiki></code> on a discussion page.
<noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Ornn Masterwork items

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AOrnn%20Masterwork%20items%2Fdoc.txt
- Decoded name: Template:Ornn Masterwork items/doc
- Namespace: Template
- Remainder: Ornn Masterwork items/doc
- Path parts: Template:Ornn Masterwork items / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is used to show a list of {{cai|Living Forge|Ornn|Masterwork}} items and the additional stats they provide compared to their non-upgraded version, as well as the gold value of these stats.
: The data for this gets automatically retrieved from and processed by [[:Module:ItemData]].

;Syntax
To display a table listing <b>all {{cai|Living Forge|Ornn|Masterwork}} items</b>, write:
: <code>{{t|Ornn Masterwork items}}</code>
{{Ornn Masterwork items}}
----
If you want to display a specific <b>all {{cai|Living Forge|Ornn|Masterwork}} items</b>, write:
: <code>{{t|Ornn Masterwork items|item}}</code>
For example
: <code><nowiki>{{Ornn Masterwork items|Infinity Force}}</nowiki></code>
{{Ornn Masterwork items|Infinity Force}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Deprecated templates]]
```

### Notes

_No notes specified._

---
