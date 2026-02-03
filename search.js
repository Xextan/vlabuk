import json from "./words.json" with {type: "json"};
let dict = json.data;
let SERIAL_PREFIXES = ["qu","po"];
let COMPOUND_PREFIXES = ["ga","ge"];
let PREFIXES = [...SERIAL_PREFIXES,...COMPOUND_PREFIXES];
let SUFFIXES = ["ko","xo","sa","se","si","zu"];
let AFFIXES = [...PREFIXES,...SUFFIXES];
document.getElementById("pickle").addEventListener("change", function () {
  document.documentElement.classList.toggle("pickle");
});
document.getElementById("search").addEventListener("input", function () {
  let q = document.getElementById("search").value.toLowerCase();
  let r = search(q).sort((a, b) => b[1] - a[1]).map(e => htmlify(e[0]));
  document.getElementById("results").innerHTML = "";
  document.getElementById("results").append(...r);
  document.getElementById("len").innerHTML = r.length + " result" + (r.length != 1 ? "s" : "");
});
function deaccent(s) {
  return s.replace(/[‘’]/g, "'").replace(/[áãâ]/g, "a").replace(/[éẽê]/g, "e").replace(/[íĩî]/g, "i").replace(/[óõô]/g, "o").replace(/[úũû]/g, "u").replace(/[ýỹŷ]/g, "y");
}
function search(q) {
  // kinda stolen from xlasisku lol
  var results = [];
  for (const w of q.split(/\s+/)) {
    const exact = dict.find(e => deaccent(e.word.toLowerCase()) == deaccent(w));
    if (exact) {
      results.push([exact, 10]);
    }
  }
  for (const entry of dict) {
    const rgx = q.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    const regex = `(\\b|\\W|^)${rgx}(\\b|\\W|$)`;
    if (entry.def && entry.def.toLowerCase() == q) {
      results.push([entry, 2]);
    }
    if ([entry.def, entry.alignment, ...(entry.semantics || []), entry.xo, entry.ko].some(e => new RegExp(regex, "iu").test(e))) {
      results.push([entry, 1]);
    }
    if (deaccent(entry.word.toLowerCase()).startsWith(deaccent(q)) || deaccent(entry.word.toLowerCase()).includes(deaccent(q)) || (entry.semantics || []).some(e => e.toLowerCase().startsWith(q))) {
      results.push([entry, 1]);
    }
    if (entry.notes && new RegExp(regex, "iu").test(entry.notes)) {
      results.push([entry, 1]);
    }
  }
  return dedup(results);
}
function dedup(list) {
  var list = list.sort((a, b) => b[1] - a[1]);
  const logged = {};
  list = list.filter(entry => {
    if (logged[entry[0].word]) return false;
    logged[entry[0].word] = true;
    return true;
  });
  return list;
}
document.getElementById("search").dispatchEvent(new Event("input", {"bubbles": true}));
function mkelem(tag, props, children) {
  const element = document.createElement(tag);
  Object.assign(element, props);
  for (const child of children) {
    if (child) {
      element.append(child);
    }
  }
  return element;
}
function hasPrefixes(e, a) {
  return a.some(p =>
    e.word.startsWith(p)
    && !["bad", "particle"].includes(pos(dict.find(x => deaccent(x.word) == deaccent(e.word.slice(2))) ?? {word: ""}))
  );
}
function hasSuffixes(e, a) {
  return a.some(p =>
    e.word.endsWith(p)
    && !["bad", "particle"].includes(pos(dict.find(x => deaccent(x.word) == deaccent(e.word.slice(0, -2))) ?? {word: ""}))
  );
}
function pos(e) {
  if (e.word.includes(" ") || hasPrefixes(e, SERIAL_PREFIXES)) {
    return "serial";
  }
  if (hasPrefixes(e, COMPOUND_PREFIXES) || hasSuffixes(e, SUFFIXES))
    return "pseudocompound";
  if (e.gloss)
    return "compound"; // true
  if (/^.{0,2}[áéíóú]/iu.test(e.word) || (!/^[bdfgklmnpqstvxz]([aeiou][ptkln]|[bdfgklmnpqstvxz][aeiou])$/.test(e.word) && e.alignment))
    return "freeword";
  if (e.alignment)
    return "root";
  if (e.type)
    return "particle";
  return "bad";
}
function htmlify(entry) {
  let etym_content = entry.etymology ? mkelem("p", {}, [
    mkelem("span", {"className": "h"}, ["Etymology: "]),
    ...(entry.etymology.length ? entry.etymology : [entry.etymology]).map(e => typeof e == "string" ? [` ${e} `] : [
      e.lang,
      " ",
      e.link ?? true ? mkelem("a", {"href": url(e)}, [
        e.word,
        " ",
        mkelem("i", {}, [e.translit])
      ]) : mkelem("span", {}, [
        e.word,
        " ",
        mkelem("i", {}, [e.translit])
      ])
    ]).flat()
  ]) : null;
  let d = entry.derivs;
  let derivs = d ? mkelem("p", {}, [
    mkelem("span", {"className": "h"}, ["Derivations: "]),
    ...[d.qu ?? "", d.po ?? "", d.ga ?? "", d.ge ?? "", d.ko ?? "", d.xo ?? "", d.sa ?? "", d.se ?? "", d.si ?? "", d.zu ?? ""].map((e, i) => e ? [
      mkelem("br", {}, []),
      mkelem("b", {}, [derive(entry.word, AFFIXES[i])]),
      " " + e
    ] : null).flat()
  ]) : null;
  return mkelem("div", {"className": "entry " + pos(entry)}, [
    mkelem("p", {}, [
      mkelem("b", {}, [entry.word]),
      " ",
      ...[entry.type ? entry.type.split("-").map(e => mkelem("span", {"className": "type " + e}, [e])) : null].flat(),
      " ",
      entry.def,
      entry.def && entry.alignment ? " – " : null,
      entry.alignment
    ]),
    entry.notes || entry.semantics || entry.etymology || entry.derivs || entry.gloss ? mkelem("div", {"className": "more"}, [
      entry.gloss ? mkelem("p", {}, [
        mkelem("span", {"className": "h"}, ["Gloss: "]),
        mkelem("code", {}, [entry.gloss])
      ]) : null,
      entry.semantics ? mkelem("p", {}, [
        mkelem("span", {"className": "h"}, ["Keywords: "]),
        entry.semantics.join(", ")
      ]) : null,
      derivs,
      entry.notes ? mkelem("p", {}, [
        mkelem("span", {"className": "h"}, ["Notes: "]),
        entry.notes
      ]) : null,
      etym_content
    ]) : null
  ]);
}
function derive(w, a) {
  // i have no idea what these were for
  // var i = /(?<![iu])[aeiou]/.exec(w).index;
  // var v = w.match(/(?<![iu])[aeiou]/);
  if (SUFFIXES.includes(a)) {
    return w + a;
  } else {
    return a + w.replace("a", "á").replace("e", "é").replace("i", "í").replace("o", "ó").replace("u", "ú");
  }
}
function url(e) {
  var url;
  if (e.link) {
    url = e.link;
  } else if (!e.lang) {
    url = "https://en.wiktionary.org/wiki/" + (e.urlform || e.word);
  } else if (e.lang == "Loglan") {
    url = "https://mi2ebi.github.io/zalduvrai?q=" + (e.urlform || e.word.split(" ").slice(-1)[0]);
  } else if (e.lang.includes("Lojban")) {
    url = "https://sisku.org?en#" + (e.urlform || (e.word.includes("←") ? e.word.split(" ").slice(-1)[0] : e.word));
  } else if (e.lang == "Klingon") {
    url = "https://klingon.wiki/Word/" + (e.urlform || e.word);
  } else if (e.lang == "Ceqli") {
    url = "http://ceqli.pbworks.com/w/page/5455969/Ceqli-English%20Glossary";
  } else if (e.lang == "Volapük") {
    url = "http://volapük.com/VoEnDictionary-20100830.pdf";
  } else if (e.lang == "Toki Pona") {
    url = "https://linku.la/?q=" + e.word;
  } else if (e.lang == "Japanese") {
    url = "https://jisho.org/word/" + e.word;
  } else if (e.lang == "Toaq") {
    url = "https://toadua.uakci.space/#=" + e.word;
  } else if (e.lang == "Proto-Indo-European") {
    url = "https://en.wiktionary.org/wiki/Reconstruction:Proto-Indo-European/" + e.word.slice(1);
  } else if (e.lang == "Vötgil") {
    url = "http://www.ostracodfiles.com/votgil/guide.html";
  } else if (e.lang == "American Sign Language") {
    url = "https://www.signasl.org/sign/" + e.word.toLowerCase();
  } else if (e.lang == "Cherokee" && +e.urlform) {
    url = "https://www.cherokeedictionary.net/share/" + e.urlform;
  } else {
    url = "https://en.wiktionary.org/wiki/" + (e.urlform || e.word);
  }
  return url;
}
