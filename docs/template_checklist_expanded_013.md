# Template checklist — expanded

Generated: 2025-10-10T15:54:19.185Z

Batch 13 of 33 — items 241..260

## If both

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AIf%20both%2Fdoc.txt
- Decoded name: Template:If both/doc
- Namespace: Template
- Remainder: If both/doc
- Path parts: Template:If both / doc

```
{{Documentation subpage}}
<!-- PLEASE ADD CATEGORIES AND INTERWIKIS AT THE BOTTOM OF THIS PAGE -->
__NOTOC____NOEDITSECTION__

This helper template is designed to be used in other templates. It can help editors create template code that is self documenting.

=== Usage ===
*If '''a''' and '''b''' then true.
*If '''a''' nor '''b''' then false.
*If '''a''' or '''b''' then false.
It can be used to replace the construct: <tt><nowiki>{{#if:a|{{#if:b|true|false}}|false}}</nowiki></tt>

=== Examples ===
<tt>
*<nowiki>{{if both| a | b |true|false}}</nowiki> → {{if both| a | b |true|false}}
*<nowiki>{{if both| a | |true|false}}</nowiki> → {{if both| | b |true|false}}
*<nowiki>{{if both| | b |true|false}}</nowiki> → {{if both| | b |true|false}}
*<nowiki>{{if both| |  |true|false}}</nowiki> → {{if both| |  |true|false}}
</tt>

```

### Notes

_No notes specified._

---

## Ifempty

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AIfempty%2Fdoc.txt
- Decoded name: Template:Ifempty/doc
- Namespace: Template
- Remainder: Ifempty/doc
- Path parts: Template:Ifempty / doc

```
{{Documentation subpage}}
<!-- PLEASE ADD CATEGORIES AND INTERWIKIS AT THE BOTTOM OF THIS PAGE -->

This is the {{tl|ifempty}} template.

This template is used inside other templates. It takes up to nine parameters (parameters 1-9), and returns the first one that is defined and non-empty, otherwise nothing. Typical usage is like this:

:<code><nowiki>{{ifempty| {{{logo|}}} | {{{image|}}} | default.svg }}</nowiki></code>
This returns the first of the parameters ''logo'' and ''image'' that is defined and non-empty, otherwise "default.svg".

=== Background ===

The MediaWiki parameter default function doesn't return the default value for empty parameters. That is, <code><nowiki>{{{logo|default.svg}}}</nowiki></code> does not return "default.svg" if the template was called like this: <code><nowiki>{{template|logo=}}</nowiki></code>.

The usual workaround is to do like this: 
:<code><nowiki>{{#if:{{{logo|}}}| {{{logo}}} | default.svg }}</nowiki></code>

But this becomes complex when you want to check several parameters:

:<code><nowiki>{{#if:{{{logo|}}}| {{{logo}}} | {{#if:{{{image|}}} | {{{image}}} | default.svg }}}}</nowiki></code>
```

### Notes

_No notes specified._

---

## Ifnumber

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AIfnumber%2Fdoc.txt
- Decoded name: Template:Ifnumber/doc
- Namespace: Template
- Remainder: Ifnumber/doc
- Path parts: Template:Ifnumber / doc

```
{{Documentation subpage}}
<!-- PLEASE ADD CATEGORIES AND INTERWIKIS AT THE BOTTOM OF THIS PAGE -->

=== Usage ===
Returns <nowiki>{{{2|1}}} if {{{1}}} is numeric & {{{3|0}}}</nowiki> otherwise.

==Examples==
<table class="wikitable">
<tr><th>code</th><th>result</th></tr>
<tr><td><code><nowiki>{{ifnumber|1}}</nowiki></code></td><td>{{ifnumber|1}}</td></tr>
<tr><td><code><nowiki>{{ifnumber|one}}</nowiki></code></td><td>{{ifnumber|one}}</td></tr>
<tr><td><code><nowiki>{{ifnumber|1|a number|not a number}}</nowiki></code></td><td>{{ifnumber|1|a number|not a number}}</td></tr>
<tr><td><code><nowiki>{{ifnumber|one|a number|not a number}}</nowiki></code></td><td>{{ifnumber|one|a number|not a number}}</td></tr>
<tr><td><code><nowiki>{{ifnumber|1000|a number|not a number}}</nowiki></code></td><td>{{ifnumber|1000|a number|not a number}}</td></tr>
<tr><td><code><nowiki>{{ifnumber|10.00|a number|not a number}}</nowiki></code></td><td>{{ifnumber|10.00|a number|not a number}}</td></tr>
<tr><td><code><nowiki>{{ifnumber|1,000|a number|not a number}}</nowiki></code></td><td>{{ifnumber|1,000|a number|not a number}}</td></tr>
<tr><td><code><nowiki>{{ifnumber|1,000.50|a number|not a number}}</nowiki></code></td><td>{{ifnumber|1,000.50|a number|not a number}}</td></tr>
<tr><td><code><nowiki>{{ifnumber|99 balloons|a number|not a number}}</nowiki></code></td><td>{{ifnumber|99 balloons|a number|not a number}}</td></tr>
<tr><td><code><nowiki>{{ifnumber|99.9999e10|a number|not a number}}</nowiki></code></td><td>{{ifnumber|99.9999e10|a number|not a number}}</td></tr>
<tr><td><code><nowiki>{{Ifnumber|{{{val|6.B2C}}}|number|value "{{{val|6.B2C}}}" not numeric}}</nowiki>&nbsp;</code></td><td> {{Ifnumber|{{{val|6.B2C}}}|number|value "{{{val|6.B2C}}}" not numeric}}</td></tr>
```

### Notes

_No notes specified._

---

## Ifsubpage

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AIfsubpage%2Fdoc.txt
- Decoded name: Template:Ifsubpage/doc
- Namespace: Template
- Remainder: Ifsubpage/doc
- Path parts: Template:Ifsubpage / doc

```
{{Documentation subpage}}
Returns <code><nowiki>{{{1}}}</nowiki></code> if the current page is a subpage, or <code><nowiki>{{{2}}}</nowiki></code> if it is not a subpage.
<includeonly>[[Category:If-then-else templates]]</includeonly>
```

### Notes

_No notes specified._

---

## Iis

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AIis%2Fdoc.txt
- Decoded name: Template:Iis/doc
- Namespace: Template
- Remainder: Iis/doc
- Path parts: Template:Iis / doc

```
{{Documentation subpage}}
:This template shortcuts usage of {{t|item icon}} when using possessive apostrophes.

;Syntax
:Type <code>{{t|iis|Item}}</code> instead of <code>{{t|ii|Item|Item's}}</code> at any part of the article.

:Correct:
:<code>{{tl|iis|Sheen}}</code>
:{{iis|Sheen}}
:<code>{{tl|iis|Zhonya's Hourglass}}</code>
:{{iis|Zhonya's Hourglass}}

:Wrong:
:<code>{{tl|ii|Sheen}}'s</code>
:{{ii|Sheen}}'s
:<code>{{tl|ii|Zhonya's Hourglass}}'</code>
:{{ii|Zhonya's Hourglass}}'

<includeonly>[[Category:Icon templates|{{PAGENAME}}]]</includeonly>
<noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Image header

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AImage%20header%2Fdoc.txt
- Decoded name: Template:Image header/doc
- Namespace: Template
- Remainder: Image header/doc
- Path parts: Template:Image header / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
Renders a gallery displaying images with older or different versions of the same image, provided that they are uploaded with proper names.
;Syntax
<pre>
{{Image header
|basename = 
|filetype = 
|oldcount = 
|hd       = 
|s1       = 
|o1       = 
|o1desc   = 
}}
</pre>

<code>s1</code> can be set to a version with the input being the custom suffix that follows the basename, then <code>s2</code>, <code>s3</code>, etc. A description <code>s1desc</code> can also be written to replace the caption.

<code>o1</code> works in the same manner, except the input can be any file name of the same file extension. <code>o1desc</code> is an optional caption.
```

### Notes

_No notes specified._

---

## Image header/ability

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AImage%20header%2Fability%2Fdoc.txt
- Decoded name: Template:Image header/ability/doc
- Namespace: Template
- Remainder: Image header/ability/doc
- Path parts: Template:Image header / ability / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is used to automatically generate the page on [[:Category:Champion ability icons|Ability icons]]. This includes the gallery view and the categorization.

;Syntax
:<code>{{t|Image header/ability}}</code>
This code can be copied on the files page. It automatically retrieves the ability name and the champion from the pagename.

;Additional parameters
* <code>nocategories = true</code>: Suppresses the categorization and hides the <code>{{tl|Fairuse}}</code> template.
* <code>rc</code>: Necessary for ability icons on [[List of champions#scrapped champions|removed champions]]. Enter the removed champions name here.
** <code>champion</code>: Can be used in combination with <code>rc</code> to add the icon to another champions old icon categories (e.g. [[:File:Priscilla Thirst.png|Pricilla's Thirst]]).
----

;Verification
To verify, that this subtemplate is used in all files it is intended for, check [[User:TheTimebreaker/Image_header_verify#ability|this page]].

;Categorization
The categorization is externalized into [[:Module:Image header/ability]] into the function <code>categorization</code>.
```

### Notes

_No notes specified._

---

## Image header/buff

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AImage%20header%2Fbuff%2Fdoc.txt
- Decoded name: Template:Image header/buff/doc
- Namespace: Template
- Remainder: Image header/buff/doc
- Path parts: Template:Image header / buff / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is used to automatically generate the page on [[:Category:Buff icons|buff icons]]. This includes the gallery view and the categorization.

;Syntax
:<code>{{t|Image header/buff}}</code>
This code can be copied on the files page. It automatically retrieves the buff from the filename.

;Additional parameters
* <code>nocategories = true</code>: Suppresses the categorization and hides the <code>{{tl|Fairuse}}</code> template.
----

;Verification
To verify, that this subtemplate is used in all files it is intended for, check [[User:TheTimebreaker/Image_header_verify#buff|this page]].

;See also
* [[:Template:Image header]]
* [[:Module:Image header]]
** [[:Module:Image header/buff]]
```

### Notes

_No notes specified._

---

## Image header/chroma

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AImage%20header%2Fchroma%2Fdoc.txt
- Decoded name: Template:Image header/chroma/doc
- Namespace: Template
- Remainder: Image header/chroma/doc
- Path parts: Template:Image header / chroma / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is used to automatically generate the page for [[:Category:Champion chromas|skin chromas]]. This includes the gallery view and the categorization.

;Syntax
:<code>{{t|Image header/chroma}}</code>
This code can be copied on the files page. It automatically retrieves the champion's name and the skin from the pagename.

;Additional parameters
* The first parameter can be used to enter a pagename manually. ''(Should only be used for testing and not on actual file pages.)''
* <code>nocategories = true</code>: Suppresses the categorization and hides the <code>{{tl|Fairuse}}</code> template.

;Examples
----
{{Template:Image_header/chroma/doc/example|example 1}}
----
{{Template:Image_header/chroma/doc/example2|example 2}}
----
{{Template:Image_header/chroma/doc/example3|example 3}}
```

### Notes

_No notes specified._

---

## Image header/item

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AImage%20header%2Fitem%2Fdoc.txt
- Decoded name: Template:Image header/item/doc
- Namespace: Template
- Remainder: Image header/item/doc
- Path parts: Template:Image header / item / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is used to automatically generate the page on [[:Category:Item icons|Item icons]]. This includes the gallery view and the categorization.
: '''Note:''' Do not use this template more than once per page, as it goes above the template expensive call limit.

;Syntax
:<code>{{t|Image header/item}}</code>
This code can be copied on the files page. It automatically retrieves the items name from the pagename.

;Additional parameters
* The first parameter can be used to make the template use a different item's name than the one retrieved from the pagename. This is useful for renamed items.
* <code>old = item name</code>: Used, if the item has been renamed, but still exists in the game.
** In this case, the code <code><nowiki>{{Image header/item|Current name of the item|old = Old name}}</nowiki></code> should be used on all pages of the item.
* <code>other</code>: List of other files, that should be included (file extension needed). Add <code>;</code> between entries. Adds only the listed file and nothing else.
** <code>otherdesc</code>: Description of the <code>other</code> files. Add <code>;</code> between entries.
* <code>nocategories = true</code>: Suppresses the categorization and hides the <code>{{tl|Fairuse}}</code> template.

;Example
:<code><nowiki>{{Image header/item|Rabadon's Deathcap|nocategories = true}}</nowiki></code>
```

### Notes

_No notes specified._

---

## Image header/skin

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AImage%20header%2Fskin%2Fdoc.txt
- Decoded name: Template:Image header/skin/doc
- Namespace: Template
- Remainder: Image header/skin/doc
- Path parts: Template:Image header / skin / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is used to automatically generate the page on [[:Category:Champion skins|champion skin images]]. This includes the gallery view and the categorization.

;Syntax
:<code>{{t|Image header/skin}}</code>
This code can be copied on the files page. It automatically retrieves the champion's name and the skin from the pagename.

;Additional parameters
* The first parameter can be used to enter a pagename manually. ''(Should only be used for testing and not on actual file pages.)''
* <code>group = true</code>: Can be used on high definition pictures of splash arts, that are shared between multiplie champions.
* <code>nochromas = true</code>: Suppresses the chroma gallery.
;Not applicable (for changing move from fandom to official wiki)
* <code>nocategories = true</code>: Suppresses the categorization and hides the <code>{{tl|Fairuse}}</code> template.

;Examples
----
{{Image header/skin|Jax OriginalSkin.jpg|nocategories = true}}
----
```

### Notes

_No notes specified._

---

## Image header/spell

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AImage%20header%2Fspell%2Fdoc.txt
- Decoded name: Template:Image header/spell/doc
- Namespace: Template
- Remainder: Image header/spell/doc
- Path parts: Template:Image header / spell / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This template is used to automatically generate the page on [[:Category:Summoner spell icons|Summoner spell icons]]. This includes the gallery view and the categorization.

;Syntax
:<code>{{t|Image header/spell}}</code>
This code can be copied on the files page. It automatically retrieves the items name from the pagename.

;Additional parameters
* The first parameter can be used to make the template use a different spell's name than the one retrieved from the pagename. ''This should only be used on this documentation and for testing special cases!''
* <code>nocategories = true</code>: Suppresses the categorization and hides the <code>{{tl|Fairuse}}</code> template.

;Examples
''All examples include <code>nocategories = true</code>.''
----
:<code><nowiki>{{Image header/spell|Flash old3.png}}</nowiki></code>
{{Image header/spell|Flash old3.png|nocategories=true}}
----
:<code><nowiki>{{Image header/spell|Resuscitate.png}}</nowiki></code>
```

### Notes

_No notes specified._

---

## Image tabber

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AImage%20tabber%2Fdoc.txt
- Decoded name: Template:Image tabber/doc
- Namespace: Template
- Remainder: Image tabber/doc
- Path parts: Template:Image tabber / doc

```
{{Documentation subpage}}

;Syntax
<pre>
  {{Image tabber
  |title1=
  |image1=
  |image1-size=
  |content1=

  |title2=
  |image1=
  |image2-size=
  |content2=
  }}
</pre>


Optional parameters
<pre>
```

### Notes

_No notes specified._

---

## Infinity

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AInfinity%2Fdoc.txt
- Decoded name: Template:Infinity/doc
- Namespace: Template
- Remainder: Infinity/doc
- Path parts: Template:Infinity / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
Use in formulas to have proper non-breaking space placement; linebreaks will only occur after an operator, and not before.

{{Math symbols}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Character formatting templates]]
[[de:Vorlage:Grad]]
[[pt-br:Predefinição:Infinito]]
</includeonly>
```

### Notes

_No notes specified._

---

## Infobox

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AInfobox%2Fdoc.txt
- Decoded name: Template:Infobox/doc
- Namespace: Template
- Remainder: Infobox/doc
- Path parts: Template:Infobox / doc

```
{{Documentation subpage}}
{{infobox|title=Title|image=[[File:Sheen item.png]]|caption=Caption
|header1=Header 1|label1=Label 1|data1=Data 1
|label2=Label 2|data2=Data 2
|header3=Header 3|label3=Label 3|data3=Data 3
|label4=Label 4|data4=Data 4
}}

==Description==
*This template produces a [[Help:Infobox|infobox]]-style template.
*You may want to copy and modify this to create a specific infobox (e.g. character infobox).
*This template requires [[Help:Parser functions|parser functions]] to be enabled (Wikia default: on).

<includeonly>[[Category:General wiki templates|{{PAGENAME}}]][[Category:Infobox templates| ]]</includeonly><noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Infobox/Credits

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AInfobox%2FCredits%2Fdoc.txt
- Decoded name: Template:Infobox/Credits/doc
- Namespace: Template
- Remainder: Infobox/Credits/doc
- Path parts: Template:Infobox / Credits / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: Infobox for character development credits.

;Syntax
: Note that ''Project Lead'' refers to a development structure that has not been used since ~2015, when new champions were designed by small teams with a single project head. 
<pre>
{{Infobox/Credits|Character name
|concept=
|conceptcredit=
|lead=
|gameplay=
|narrative=
|artwork=
|visual=
|sound=
|voice
|guests=
```

### Notes

_No notes specified._

---

## Infobox/Pet

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AInfobox%2FPet%2Fdoc.txt
- Decoded name: Template:Infobox/Pet/doc
- Namespace: Template
- Remainder: Infobox/Pet/doc
- Path parts: Template:Infobox / Pet / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

==Usage==
<pre>
{{Infobox/Pet|<name>
|render=
|attributes=
|gold=
|exp=
|leash=
|sight=
|hp=
|healthregen=
|damage=
|abilitypower=
|damagetype=
|range=
|attackspeed=
|armor=
```

### Notes

_No notes specified._

---

## Infotip

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AInfotip%2Fdoc.txt
- Decoded name: Template:Infotip/doc
- Namespace: Template
- Remainder: Infotip/doc
- Path parts: Template:Infotip / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Usage ==
Renders the tooltip onto the page. For the seamless version, see {{t|Tip info}}.

== Syntax ==
{{t|Infotip|info}}

== Examples ==
{{tl|Infotip|Critical Strike}}
{{Infotip|Critical Strike}}
{{clr}}
{{tl|Infotip|Marksman}}
{{Infotip|Marksman}}
{{clr}}
{{tl|Infotip|physical damage}}
{{Infotip|physical damage}}

<includeonly>
```

### Notes

_No notes specified._

---

## Inspired lore

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AInspired%20lore%2Fdoc.txt
- Decoded name: Template:Inspired lore/doc
- Namespace: Template
- Remainder: Inspired lore/doc
- Path parts: Template:Inspired lore / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## IP

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AIP%2Fdoc.txt
- Decoded name: Template:IP/doc
- Namespace: Template
- Remainder: IP/doc
- Path parts: Template:IP / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce an [[Influence Point]] icon and the link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|IP|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|IP}}</code>
:* {{IP}}

:<code>{{tl|IP|100}}</code>
:* {{IP|100}}

:<code>{{tl|IP|100|3=size=30}}</code>
:* {{IP|100|size=30}}

<includeonly>
```

### Notes

_No notes specified._

---
