use std::str::FromStr;

use Result;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LanguageKind {
    Afar,
    Abkhaz,
    Avestan,
    Afrikaans,
    Akan,
    Amharic,
    Aragonese,
    Arabic,
    Assamese,
    Avaric,
    Aymara,
    Azerbaijani,
    Bashkir,
    Belarusian,
    Bulgarian,
    Bihari,
    Bislama,
    Bambara,
    Bengali,
    TibetanStandard,
    Breton,
    Bosnian,
    Catalan,
    Chechen,
    Chamorro,
    Corsican,
    Cree,
    Czech,
    OldChurchSlavonic,
    Chuvash,
    Welsh,
    Danish,
    German,
    Divehi,
    Dzongkha,
    Ewe,
    Greek,
    English,
    Esperanto,
    Spanish,
    Estonian,
    Basque,
    Persian,
    Fula,
    Finnish,
    Fijian,
    Faroese,
    French,
    WesternFrisian,
    Irish,
    ScottishGaelic,
    Galician,
    Guarani,
    Gujarati,
    Manx,
    Hausa,
    Hebrew,
    Hindi,
    HiriMotu,
    Croatian,
    Haitian,
    Hungarian,
    Armenian,
    Herero,
    Interlingua,
    Indonesian,
    Interlingue,
    Igbo,
    Nuosu,
    Inupiaq,
    Ido,
    Icelandic,
    Italian,
    Inuktitut,
    Japanese,
    Javanese,
    Georgian,
    Kongo,
    Kikuyu,
    Kwanyama,
    Kazakh,
    Kalaallisut,
    Khmer,
    Kannada,
    Korean,
    Kanuri,
    Kashmiri,
    Kurdish,
    Komi,
    Cornish,
    Kyrgyz,
    Latin,
    Luxembourgish,
    Ganda,
    Limburgish,
    Lingala,
    Lao,
    Lithuanian,
    LubaKatanga,
    Latvian,
    Malagasy,
    Marshallese,
    Maori,
    Macedonian,
    Malayalam,
    Mongolian,
    Marathi,
    Malay,
    Maltese,
    Burmese,
    Nauruan,
    NorwegianBokmal,
    NorthernNdebele,
    Nepali,
    Ndonga,
    Dutch,
    NorwegianNynorsk,
    Norwegian,
    SouthernNdebele,
    Navajo,
    Chichewa,
    Occitan,
    Ojibwe,
    Oromo,
    Oriya,
    Ossetian,
    EasternPunjabi,
    Pali,
    Polish,
    Pashto,
    Portuguese,
    Quechua,
    Romansh,
    Kirundi,
    Romanian,
    Russian,
    Kinyarwanda,
    Sanskrit,
    Sardinian,
    Sindhi,
    NorthernSami,
    Sango,
    Sinhalese,
    Slovak,
    Slovene,
    Samoan,
    Shona,
    Somali,
    Albanian,
    Serbian,
    Swati,
    SouthernSotho,
    Sundanese,
    Swedish,
    Swahili,
    Tamil,
    Telugu,
    Tajik,
    Thai,
    Tigrinya,
    Turkmen,
    Tagalog,
    Tswana,
    Tonga,
    Turkish,
    Tsonga,
    Tatar,
    Twi,
    Tahitian,
    Uyghur,
    Ukrainian,
    Urdu,
    Uzbek,
    Venda,
    Vietnamese,
    Volapuk,
    Walloon,
    Wolof,
    Xhosa,
    Yiddish,
    Yoruba,
    Zhuang,
    Chinese,
    Zulu,
}

impl FromStr for LanguageKind {
    type Err = String;
    fn from_str(language_code: &str) -> Result<LanguageKind> {
        match language_code.to_lowercase().as_str() {
            "aa" => Ok(LanguageKind::Afar),
            "ab" => Ok(LanguageKind::Abkhaz),
            "ae" => Ok(LanguageKind::Avestan),
            "af" => Ok(LanguageKind::Afrikaans),
            "ak" => Ok(LanguageKind::Akan),
            "am" => Ok(LanguageKind::Amharic),
            "an" => Ok(LanguageKind::Aragonese),
            "ar" => Ok(LanguageKind::Arabic),
            "as" => Ok(LanguageKind::Assamese),
            "av" => Ok(LanguageKind::Avaric),
            "ay" => Ok(LanguageKind::Aymara),
            "az" => Ok(LanguageKind::Azerbaijani),
            "ba" => Ok(LanguageKind::Bashkir),
            "be" => Ok(LanguageKind::Belarusian),
            "bg" => Ok(LanguageKind::Bulgarian),
            "bh" => Ok(LanguageKind::Bihari),
            "bi" => Ok(LanguageKind::Bislama),
            "bm" => Ok(LanguageKind::Bambara),
            "bn" => Ok(LanguageKind::Bengali),
            "bo" => Ok(LanguageKind::TibetanStandard),
            "br" => Ok(LanguageKind::Breton),
            "bs" => Ok(LanguageKind::Bosnian),
            "ca" => Ok(LanguageKind::Catalan),
            "ce" => Ok(LanguageKind::Chechen),
            "ch" => Ok(LanguageKind::Chamorro),
            "co" => Ok(LanguageKind::Corsican),
            "cr" => Ok(LanguageKind::Cree),
            "cs" => Ok(LanguageKind::Czech),
            "cu" => Ok(LanguageKind::OldChurchSlavonic),
            "cv" => Ok(LanguageKind::Chuvash),
            "cy" => Ok(LanguageKind::Welsh),
            "da" => Ok(LanguageKind::Danish),
            "de" => Ok(LanguageKind::German),
            "dv" => Ok(LanguageKind::Divehi),
            "dz" => Ok(LanguageKind::Dzongkha),
            "ee" => Ok(LanguageKind::Ewe),
            "el" => Ok(LanguageKind::Greek),
            "en" => Ok(LanguageKind::English),
            "eo" => Ok(LanguageKind::Esperanto),
            "es" => Ok(LanguageKind::Spanish),
            "et" => Ok(LanguageKind::Estonian),
            "eu" => Ok(LanguageKind::Basque),
            "fa" => Ok(LanguageKind::Persian),
            "ff" => Ok(LanguageKind::Fula),
            "fi" => Ok(LanguageKind::Finnish),
            "fj" => Ok(LanguageKind::Fijian),
            "fo" => Ok(LanguageKind::Faroese),
            "fr" => Ok(LanguageKind::French),
            "fy" => Ok(LanguageKind::WesternFrisian),
            "ga" => Ok(LanguageKind::Irish),
            "gd" => Ok(LanguageKind::ScottishGaelic),
            "gl" => Ok(LanguageKind::Galician),
            "gn" => Ok(LanguageKind::Guarani),
            "gu" => Ok(LanguageKind::Gujarati),
            "gv" => Ok(LanguageKind::Manx),
            "ha" => Ok(LanguageKind::Hausa),
            "he" => Ok(LanguageKind::Hebrew),
            "hi" => Ok(LanguageKind::Hindi),
            "ho" => Ok(LanguageKind::HiriMotu),
            "hr" => Ok(LanguageKind::Croatian),
            "ht" => Ok(LanguageKind::Haitian),
            "hu" => Ok(LanguageKind::Hungarian),
            "hy" => Ok(LanguageKind::Armenian),
            "hz" => Ok(LanguageKind::Herero),
            "ia" => Ok(LanguageKind::Interlingua),
            "id" => Ok(LanguageKind::Indonesian),
            "ie" => Ok(LanguageKind::Interlingue),
            "ig" => Ok(LanguageKind::Igbo),
            "ii" => Ok(LanguageKind::Nuosu),
            "ik" => Ok(LanguageKind::Inupiaq),
            "io" => Ok(LanguageKind::Ido),
            "is" => Ok(LanguageKind::Icelandic),
            "it" => Ok(LanguageKind::Italian),
            "iu" => Ok(LanguageKind::Inuktitut),
            "ja" => Ok(LanguageKind::Japanese),
            "jv" => Ok(LanguageKind::Javanese),
            "ka" => Ok(LanguageKind::Georgian),
            "kg" => Ok(LanguageKind::Kongo),
            "ki" => Ok(LanguageKind::Kikuyu),
            "kj" => Ok(LanguageKind::Kwanyama),
            "kk" => Ok(LanguageKind::Kazakh),
            "kl" => Ok(LanguageKind::Kalaallisut),
            "km" => Ok(LanguageKind::Khmer),
            "kn" => Ok(LanguageKind::Kannada),
            "ko" => Ok(LanguageKind::Korean),
            "kr" => Ok(LanguageKind::Kanuri),
            "ks" => Ok(LanguageKind::Kashmiri),
            "ku" => Ok(LanguageKind::Kurdish),
            "kv" => Ok(LanguageKind::Komi),
            "kw" => Ok(LanguageKind::Cornish),
            "ky" => Ok(LanguageKind::Kyrgyz),
            "la" => Ok(LanguageKind::Latin),
            "lb" => Ok(LanguageKind::Luxembourgish),
            "lg" => Ok(LanguageKind::Ganda),
            "li" => Ok(LanguageKind::Limburgish),
            "ln" => Ok(LanguageKind::Lingala),
            "lo" => Ok(LanguageKind::Lao),
            "lt" => Ok(LanguageKind::Lithuanian),
            "lu" => Ok(LanguageKind::LubaKatanga),
            "lv" => Ok(LanguageKind::Latvian),
            "mg" => Ok(LanguageKind::Malagasy),
            "mh" => Ok(LanguageKind::Marshallese),
            "mi" => Ok(LanguageKind::Maori),
            "mk" => Ok(LanguageKind::Macedonian),
            "ml" => Ok(LanguageKind::Malayalam),
            "mn" => Ok(LanguageKind::Mongolian),
            "mr" => Ok(LanguageKind::Marathi),
            "ms" => Ok(LanguageKind::Malay),
            "mt" => Ok(LanguageKind::Maltese),
            "my" => Ok(LanguageKind::Burmese),
            "na" => Ok(LanguageKind::Nauruan),
            "nb" => Ok(LanguageKind::NorwegianBokmal),
            "nd" => Ok(LanguageKind::NorthernNdebele),
            "ne" => Ok(LanguageKind::Nepali),
            "ng" => Ok(LanguageKind::Ndonga),
            "nl" => Ok(LanguageKind::Dutch),
            "nn" => Ok(LanguageKind::NorwegianNynorsk),
            "no" => Ok(LanguageKind::Norwegian),
            "nr" => Ok(LanguageKind::SouthernNdebele),
            "nv" => Ok(LanguageKind::Navajo),
            "ny" => Ok(LanguageKind::Chichewa),
            "oc" => Ok(LanguageKind::Occitan),
            "oj" => Ok(LanguageKind::Ojibwe),
            "om" => Ok(LanguageKind::Oromo),
            "or" => Ok(LanguageKind::Oriya),
            "os" => Ok(LanguageKind::Ossetian),
            "pa" => Ok(LanguageKind::EasternPunjabi),
            "pi" => Ok(LanguageKind::Pali),
            "pl" => Ok(LanguageKind::Polish),
            "ps" => Ok(LanguageKind::Pashto),
            "pt" => Ok(LanguageKind::Portuguese),
            "qu" => Ok(LanguageKind::Quechua),
            "rm" => Ok(LanguageKind::Romansh),
            "rn" => Ok(LanguageKind::Kirundi),
            "ro" => Ok(LanguageKind::Romanian),
            "ru" => Ok(LanguageKind::Russian),
            "rw" => Ok(LanguageKind::Kinyarwanda),
            "sa" => Ok(LanguageKind::Sanskrit),
            "sc" => Ok(LanguageKind::Sardinian),
            "sd" => Ok(LanguageKind::Sindhi),
            "se" => Ok(LanguageKind::NorthernSami),
            "sg" => Ok(LanguageKind::Sango),
            "si" => Ok(LanguageKind::Sinhalese),
            "sk" => Ok(LanguageKind::Slovak),
            "sl" => Ok(LanguageKind::Slovene),
            "sm" => Ok(LanguageKind::Samoan),
            "sn" => Ok(LanguageKind::Shona),
            "so" => Ok(LanguageKind::Somali),
            "sq" => Ok(LanguageKind::Albanian),
            "sr" => Ok(LanguageKind::Serbian),
            "ss" => Ok(LanguageKind::Swati),
            "st" => Ok(LanguageKind::SouthernSotho),
            "su" => Ok(LanguageKind::Sundanese),
            "sv" => Ok(LanguageKind::Swedish),
            "sw" => Ok(LanguageKind::Swahili),
            "ta" => Ok(LanguageKind::Tamil),
            "te" => Ok(LanguageKind::Telugu),
            "tg" => Ok(LanguageKind::Tajik),
            "th" => Ok(LanguageKind::Thai),
            "ti" => Ok(LanguageKind::Tigrinya),
            "tk" => Ok(LanguageKind::Turkmen),
            "tl" => Ok(LanguageKind::Tagalog),
            "tn" => Ok(LanguageKind::Tswana),
            "to" => Ok(LanguageKind::Tonga),
            "tr" => Ok(LanguageKind::Turkish),
            "ts" => Ok(LanguageKind::Tsonga),
            "tt" => Ok(LanguageKind::Tatar),
            "tw" => Ok(LanguageKind::Twi),
            "ty" => Ok(LanguageKind::Tahitian),
            "ug" => Ok(LanguageKind::Uyghur),
            "uk" => Ok(LanguageKind::Ukrainian),
            "ur" => Ok(LanguageKind::Urdu),
            "uz" => Ok(LanguageKind::Uzbek),
            "ve" => Ok(LanguageKind::Venda),
            "vi" => Ok(LanguageKind::Vietnamese),
            "vo" => Ok(LanguageKind::Volapuk),
            "wa" => Ok(LanguageKind::Walloon),
            "wo" => Ok(LanguageKind::Wolof),
            "xh" => Ok(LanguageKind::Xhosa),
            "yi" => Ok(LanguageKind::Yiddish),
            "yo" => Ok(LanguageKind::Yoruba),
            "za" => Ok(LanguageKind::Zhuang),
            "zh" => Ok(LanguageKind::Chinese),
            "zu" => Ok(LanguageKind::Zulu),
            _ => Err(format!("unknown language \"{}\"", language_code)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_languagekind() {
        assert_eq!("aa".parse::<LanguageKind>(), Ok(LanguageKind::Afar));
        assert_eq!("ab".parse::<LanguageKind>(), Ok(LanguageKind::Abkhaz));
        assert_eq!("ae".parse::<LanguageKind>(), Ok(LanguageKind::Avestan));
        assert_eq!("af".parse::<LanguageKind>(), Ok(LanguageKind::Afrikaans));
        assert_eq!("ak".parse::<LanguageKind>(), Ok(LanguageKind::Akan));
        assert_eq!("am".parse::<LanguageKind>(), Ok(LanguageKind::Amharic));
        assert_eq!("an".parse::<LanguageKind>(), Ok(LanguageKind::Aragonese));
        assert_eq!("ar".parse::<LanguageKind>(), Ok(LanguageKind::Arabic));
        assert_eq!("as".parse::<LanguageKind>(), Ok(LanguageKind::Assamese));
        assert_eq!("av".parse::<LanguageKind>(), Ok(LanguageKind::Avaric));
        assert_eq!("ay".parse::<LanguageKind>(), Ok(LanguageKind::Aymara));
        assert_eq!("az".parse::<LanguageKind>(), Ok(LanguageKind::Azerbaijani));
        assert_eq!("ba".parse::<LanguageKind>(), Ok(LanguageKind::Bashkir));
        assert_eq!("be".parse::<LanguageKind>(), Ok(LanguageKind::Belarusian));
        assert_eq!("bg".parse::<LanguageKind>(), Ok(LanguageKind::Bulgarian));
        assert_eq!("bh".parse::<LanguageKind>(), Ok(LanguageKind::Bihari));
        assert_eq!("bi".parse::<LanguageKind>(), Ok(LanguageKind::Bislama));
        assert_eq!("bm".parse::<LanguageKind>(), Ok(LanguageKind::Bambara));
        assert_eq!("bn".parse::<LanguageKind>(), Ok(LanguageKind::Bengali));
        assert_eq!("bo".parse::<LanguageKind>(), Ok(LanguageKind::TibetanStandard));
        assert_eq!("br".parse::<LanguageKind>(), Ok(LanguageKind::Breton));
        assert_eq!("bs".parse::<LanguageKind>(), Ok(LanguageKind::Bosnian));
        assert_eq!("ca".parse::<LanguageKind>(), Ok(LanguageKind::Catalan));
        assert_eq!("ce".parse::<LanguageKind>(), Ok(LanguageKind::Chechen));
        assert_eq!("ch".parse::<LanguageKind>(), Ok(LanguageKind::Chamorro));
        assert_eq!("co".parse::<LanguageKind>(), Ok(LanguageKind::Corsican));
        assert_eq!("cr".parse::<LanguageKind>(), Ok(LanguageKind::Cree));
        assert_eq!("cs".parse::<LanguageKind>(), Ok(LanguageKind::Czech));
        assert_eq!("cu".parse::<LanguageKind>(), Ok(LanguageKind::OldChurchSlavonic));
        assert_eq!("cv".parse::<LanguageKind>(), Ok(LanguageKind::Chuvash));
        assert_eq!("cy".parse::<LanguageKind>(), Ok(LanguageKind::Welsh));
        assert_eq!("da".parse::<LanguageKind>(), Ok(LanguageKind::Danish));
        assert_eq!("de".parse::<LanguageKind>(), Ok(LanguageKind::German));
        assert_eq!("dv".parse::<LanguageKind>(), Ok(LanguageKind::Divehi));
        assert_eq!("dz".parse::<LanguageKind>(), Ok(LanguageKind::Dzongkha));
        assert_eq!("ee".parse::<LanguageKind>(), Ok(LanguageKind::Ewe));
        assert_eq!("el".parse::<LanguageKind>(), Ok(LanguageKind::Greek));
        assert_eq!("en".parse::<LanguageKind>(), Ok(LanguageKind::English));
        assert_eq!("eo".parse::<LanguageKind>(), Ok(LanguageKind::Esperanto));
        assert_eq!("es".parse::<LanguageKind>(), Ok(LanguageKind::Spanish));
        assert_eq!("et".parse::<LanguageKind>(), Ok(LanguageKind::Estonian));
        assert_eq!("eu".parse::<LanguageKind>(), Ok(LanguageKind::Basque));
        assert_eq!("fa".parse::<LanguageKind>(), Ok(LanguageKind::Persian));
        assert_eq!("ff".parse::<LanguageKind>(), Ok(LanguageKind::Fula));
        assert_eq!("fi".parse::<LanguageKind>(), Ok(LanguageKind::Finnish));
        assert_eq!("fj".parse::<LanguageKind>(), Ok(LanguageKind::Fijian));
        assert_eq!("fo".parse::<LanguageKind>(), Ok(LanguageKind::Faroese));
        assert_eq!("fr".parse::<LanguageKind>(), Ok(LanguageKind::French));
        assert_eq!("fy".parse::<LanguageKind>(), Ok(LanguageKind::WesternFrisian));
        assert_eq!("ga".parse::<LanguageKind>(), Ok(LanguageKind::Irish));
        assert_eq!("gd".parse::<LanguageKind>(), Ok(LanguageKind::ScottishGaelic));
        assert_eq!("gl".parse::<LanguageKind>(), Ok(LanguageKind::Galician));
        assert_eq!("gn".parse::<LanguageKind>(), Ok(LanguageKind::Guarani));
        assert_eq!("gu".parse::<LanguageKind>(), Ok(LanguageKind::Gujarati));
        assert_eq!("gv".parse::<LanguageKind>(), Ok(LanguageKind::Manx));
        assert_eq!("ha".parse::<LanguageKind>(), Ok(LanguageKind::Hausa));
        assert_eq!("he".parse::<LanguageKind>(), Ok(LanguageKind::Hebrew));
        assert_eq!("hi".parse::<LanguageKind>(), Ok(LanguageKind::Hindi));
        assert_eq!("ho".parse::<LanguageKind>(), Ok(LanguageKind::HiriMotu));
        assert_eq!("hr".parse::<LanguageKind>(), Ok(LanguageKind::Croatian));
        assert_eq!("ht".parse::<LanguageKind>(), Ok(LanguageKind::Haitian));
        assert_eq!("hu".parse::<LanguageKind>(), Ok(LanguageKind::Hungarian));
        assert_eq!("hy".parse::<LanguageKind>(), Ok(LanguageKind::Armenian));
        assert_eq!("hz".parse::<LanguageKind>(), Ok(LanguageKind::Herero));
        assert_eq!("ia".parse::<LanguageKind>(), Ok(LanguageKind::Interlingua));
        assert_eq!("id".parse::<LanguageKind>(), Ok(LanguageKind::Indonesian));
        assert_eq!("ie".parse::<LanguageKind>(), Ok(LanguageKind::Interlingue));
        assert_eq!("ig".parse::<LanguageKind>(), Ok(LanguageKind::Igbo));
        assert_eq!("ii".parse::<LanguageKind>(), Ok(LanguageKind::Nuosu));
        assert_eq!("ik".parse::<LanguageKind>(), Ok(LanguageKind::Inupiaq));
        assert_eq!("io".parse::<LanguageKind>(), Ok(LanguageKind::Ido));
        assert_eq!("is".parse::<LanguageKind>(), Ok(LanguageKind::Icelandic));
        assert_eq!("it".parse::<LanguageKind>(), Ok(LanguageKind::Italian));
        assert_eq!("iu".parse::<LanguageKind>(), Ok(LanguageKind::Inuktitut));
        assert_eq!("ja".parse::<LanguageKind>(), Ok(LanguageKind::Japanese));
        assert_eq!("jv".parse::<LanguageKind>(), Ok(LanguageKind::Javanese));
        assert_eq!("ka".parse::<LanguageKind>(), Ok(LanguageKind::Georgian));
        assert_eq!("kg".parse::<LanguageKind>(), Ok(LanguageKind::Kongo));
        assert_eq!("ki".parse::<LanguageKind>(), Ok(LanguageKind::Kikuyu));
        assert_eq!("kj".parse::<LanguageKind>(), Ok(LanguageKind::Kwanyama));
        assert_eq!("kk".parse::<LanguageKind>(), Ok(LanguageKind::Kazakh));
        assert_eq!("kl".parse::<LanguageKind>(), Ok(LanguageKind::Kalaallisut));
        assert_eq!("km".parse::<LanguageKind>(), Ok(LanguageKind::Khmer));
        assert_eq!("kn".parse::<LanguageKind>(), Ok(LanguageKind::Kannada));
        assert_eq!("ko".parse::<LanguageKind>(), Ok(LanguageKind::Korean));
        assert_eq!("kr".parse::<LanguageKind>(), Ok(LanguageKind::Kanuri));
        assert_eq!("ks".parse::<LanguageKind>(), Ok(LanguageKind::Kashmiri));
        assert_eq!("ku".parse::<LanguageKind>(), Ok(LanguageKind::Kurdish));
        assert_eq!("kv".parse::<LanguageKind>(), Ok(LanguageKind::Komi));
        assert_eq!("kw".parse::<LanguageKind>(), Ok(LanguageKind::Cornish));
        assert_eq!("ky".parse::<LanguageKind>(), Ok(LanguageKind::Kyrgyz));
        assert_eq!("la".parse::<LanguageKind>(), Ok(LanguageKind::Latin));
        assert_eq!("lb".parse::<LanguageKind>(), Ok(LanguageKind::Luxembourgish));
        assert_eq!("lg".parse::<LanguageKind>(), Ok(LanguageKind::Ganda));
        assert_eq!("li".parse::<LanguageKind>(), Ok(LanguageKind::Limburgish));
        assert_eq!("ln".parse::<LanguageKind>(), Ok(LanguageKind::Lingala));
        assert_eq!("lo".parse::<LanguageKind>(), Ok(LanguageKind::Lao));
        assert_eq!("lt".parse::<LanguageKind>(), Ok(LanguageKind::Lithuanian));
        assert_eq!("lu".parse::<LanguageKind>(), Ok(LanguageKind::LubaKatanga));
        assert_eq!("lv".parse::<LanguageKind>(), Ok(LanguageKind::Latvian));
        assert_eq!("mg".parse::<LanguageKind>(), Ok(LanguageKind::Malagasy));
        assert_eq!("mh".parse::<LanguageKind>(), Ok(LanguageKind::Marshallese));
        assert_eq!("mi".parse::<LanguageKind>(), Ok(LanguageKind::Maori));
        assert_eq!("mk".parse::<LanguageKind>(), Ok(LanguageKind::Macedonian));
        assert_eq!("ml".parse::<LanguageKind>(), Ok(LanguageKind::Malayalam));
        assert_eq!("mn".parse::<LanguageKind>(), Ok(LanguageKind::Mongolian));
        assert_eq!("mr".parse::<LanguageKind>(), Ok(LanguageKind::Marathi));
        assert_eq!("ms".parse::<LanguageKind>(), Ok(LanguageKind::Malay));
        assert_eq!("mt".parse::<LanguageKind>(), Ok(LanguageKind::Maltese));
        assert_eq!("my".parse::<LanguageKind>(), Ok(LanguageKind::Burmese));
        assert_eq!("na".parse::<LanguageKind>(), Ok(LanguageKind::Nauruan));
        assert_eq!("nb".parse::<LanguageKind>(), Ok(LanguageKind::NorwegianBokmal));
        assert_eq!("nd".parse::<LanguageKind>(), Ok(LanguageKind::NorthernNdebele));
        assert_eq!("ne".parse::<LanguageKind>(), Ok(LanguageKind::Nepali));
        assert_eq!("ng".parse::<LanguageKind>(), Ok(LanguageKind::Ndonga));
        assert_eq!("nl".parse::<LanguageKind>(), Ok(LanguageKind::Dutch));
        assert_eq!("nn".parse::<LanguageKind>(), Ok(LanguageKind::NorwegianNynorsk));
        assert_eq!("no".parse::<LanguageKind>(), Ok(LanguageKind::Norwegian));
        assert_eq!("nr".parse::<LanguageKind>(), Ok(LanguageKind::SouthernNdebele));
        assert_eq!("nv".parse::<LanguageKind>(), Ok(LanguageKind::Navajo));
        assert_eq!("ny".parse::<LanguageKind>(), Ok(LanguageKind::Chichewa));
        assert_eq!("oc".parse::<LanguageKind>(), Ok(LanguageKind::Occitan));
        assert_eq!("oj".parse::<LanguageKind>(), Ok(LanguageKind::Ojibwe));
        assert_eq!("om".parse::<LanguageKind>(), Ok(LanguageKind::Oromo));
        assert_eq!("or".parse::<LanguageKind>(), Ok(LanguageKind::Oriya));
        assert_eq!("os".parse::<LanguageKind>(), Ok(LanguageKind::Ossetian));
        assert_eq!("pa".parse::<LanguageKind>(), Ok(LanguageKind::EasternPunjabi));
        assert_eq!("pi".parse::<LanguageKind>(), Ok(LanguageKind::Pali));
        assert_eq!("pl".parse::<LanguageKind>(), Ok(LanguageKind::Polish));
        assert_eq!("ps".parse::<LanguageKind>(), Ok(LanguageKind::Pashto));
        assert_eq!("pt".parse::<LanguageKind>(), Ok(LanguageKind::Portuguese));
        assert_eq!("qu".parse::<LanguageKind>(), Ok(LanguageKind::Quechua));
        assert_eq!("rm".parse::<LanguageKind>(), Ok(LanguageKind::Romansh));
        assert_eq!("rn".parse::<LanguageKind>(), Ok(LanguageKind::Kirundi));
        assert_eq!("ro".parse::<LanguageKind>(), Ok(LanguageKind::Romanian));
        assert_eq!("ru".parse::<LanguageKind>(), Ok(LanguageKind::Russian));
        assert_eq!("rw".parse::<LanguageKind>(), Ok(LanguageKind::Kinyarwanda));
        assert_eq!("sa".parse::<LanguageKind>(), Ok(LanguageKind::Sanskrit));
        assert_eq!("sc".parse::<LanguageKind>(), Ok(LanguageKind::Sardinian));
        assert_eq!("sd".parse::<LanguageKind>(), Ok(LanguageKind::Sindhi));
        assert_eq!("se".parse::<LanguageKind>(), Ok(LanguageKind::NorthernSami));
        assert_eq!("sg".parse::<LanguageKind>(), Ok(LanguageKind::Sango));
        assert_eq!("si".parse::<LanguageKind>(), Ok(LanguageKind::Sinhalese));
        assert_eq!("sk".parse::<LanguageKind>(), Ok(LanguageKind::Slovak));
        assert_eq!("sl".parse::<LanguageKind>(), Ok(LanguageKind::Slovene));
        assert_eq!("sm".parse::<LanguageKind>(), Ok(LanguageKind::Samoan));
        assert_eq!("sn".parse::<LanguageKind>(), Ok(LanguageKind::Shona));
        assert_eq!("so".parse::<LanguageKind>(), Ok(LanguageKind::Somali));
        assert_eq!("sq".parse::<LanguageKind>(), Ok(LanguageKind::Albanian));
        assert_eq!("sr".parse::<LanguageKind>(), Ok(LanguageKind::Serbian));
        assert_eq!("ss".parse::<LanguageKind>(), Ok(LanguageKind::Swati));
        assert_eq!("st".parse::<LanguageKind>(), Ok(LanguageKind::SouthernSotho));
        assert_eq!("su".parse::<LanguageKind>(), Ok(LanguageKind::Sundanese));
        assert_eq!("sv".parse::<LanguageKind>(), Ok(LanguageKind::Swedish));
        assert_eq!("sw".parse::<LanguageKind>(), Ok(LanguageKind::Swahili));
        assert_eq!("ta".parse::<LanguageKind>(), Ok(LanguageKind::Tamil));
        assert_eq!("te".parse::<LanguageKind>(), Ok(LanguageKind::Telugu));
        assert_eq!("tg".parse::<LanguageKind>(), Ok(LanguageKind::Tajik));
        assert_eq!("th".parse::<LanguageKind>(), Ok(LanguageKind::Thai));
        assert_eq!("ti".parse::<LanguageKind>(), Ok(LanguageKind::Tigrinya));
        assert_eq!("tk".parse::<LanguageKind>(), Ok(LanguageKind::Turkmen));
        assert_eq!("tl".parse::<LanguageKind>(), Ok(LanguageKind::Tagalog));
        assert_eq!("tn".parse::<LanguageKind>(), Ok(LanguageKind::Tswana));
        assert_eq!("to".parse::<LanguageKind>(), Ok(LanguageKind::Tonga));
        assert_eq!("tr".parse::<LanguageKind>(), Ok(LanguageKind::Turkish));
        assert_eq!("ts".parse::<LanguageKind>(), Ok(LanguageKind::Tsonga));
        assert_eq!("tt".parse::<LanguageKind>(), Ok(LanguageKind::Tatar));
        assert_eq!("tw".parse::<LanguageKind>(), Ok(LanguageKind::Twi));
        assert_eq!("ty".parse::<LanguageKind>(), Ok(LanguageKind::Tahitian));
        assert_eq!("ug".parse::<LanguageKind>(), Ok(LanguageKind::Uyghur));
        assert_eq!("uk".parse::<LanguageKind>(), Ok(LanguageKind::Ukrainian));
        assert_eq!("ur".parse::<LanguageKind>(), Ok(LanguageKind::Urdu));
        assert_eq!("uz".parse::<LanguageKind>(), Ok(LanguageKind::Uzbek));
        assert_eq!("ve".parse::<LanguageKind>(), Ok(LanguageKind::Venda));
        assert_eq!("vi".parse::<LanguageKind>(), Ok(LanguageKind::Vietnamese));
        assert_eq!("vo".parse::<LanguageKind>(), Ok(LanguageKind::Volapuk));
        assert_eq!("wa".parse::<LanguageKind>(), Ok(LanguageKind::Walloon));
        assert_eq!("wo".parse::<LanguageKind>(), Ok(LanguageKind::Wolof));
        assert_eq!("xh".parse::<LanguageKind>(), Ok(LanguageKind::Xhosa));
        assert_eq!("yi".parse::<LanguageKind>(), Ok(LanguageKind::Yiddish));
        assert_eq!("yo".parse::<LanguageKind>(), Ok(LanguageKind::Yoruba));
        assert_eq!("za".parse::<LanguageKind>(), Ok(LanguageKind::Zhuang));
        assert_eq!("zh".parse::<LanguageKind>(), Ok(LanguageKind::Chinese));
        assert_eq!("zu".parse::<LanguageKind>(), Ok(LanguageKind::Zulu));
        assert!("none".parse::<LanguageKind>().is_err());
    }

    #[test]
    fn uppercase_str_to_languagekind() {
        assert_eq!("AA".parse::<LanguageKind>(), Ok(LanguageKind::Afar));
    }
}