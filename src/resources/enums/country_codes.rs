use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum CountryCodes {
    #[serde(rename = "AL")]
    Albania,
    #[serde(rename = "DZ")]
    Algeria,
    #[serde(rename = "AD")]
    Andorra,
    #[serde(rename = "AO")]
    Angola,
    #[serde(rename = "AI")]
    Anguilla,
    #[serde(rename = "AG")]
    AntiguaAndBarbuda,
    #[serde(rename = "AR")]
    Argentina,
    #[serde(rename = "AM")]
    Armenia,
    #[serde(rename = "AW")]
    Aruba,
    #[serde(rename = "AU")]
    Australia,
    #[serde(rename = "AT")]
    Austria,
    #[serde(rename = "AZ")]
    Azerbaijan,
    #[serde(rename = "BS")]
    Bahamas,
    #[serde(rename = "BH")]
    Bahrain,
    #[serde(rename = "BB")]
    Barbados,
    #[serde(rename = "BY")]
    Belarus,
    #[serde(rename = "BE")]
    Belgium,
    #[serde(rename = "BZ")]
    Belize,
    #[serde(rename = "BJ")]
    Benin,
    #[serde(rename = "BM")]
    Bermuda,
    #[serde(rename = "BT")]
    Bhutan,
    #[serde(rename = "BO")]
    Bolivia,
    #[serde(rename = "BA")]
    BosniaAndHerzegovina,
    #[serde(rename = "BW")]
    Botswana,
    #[serde(rename = "BR")]
    Brazil,
    #[serde(rename = "VG")]
    BritishVirginIslands,
    #[serde(rename = "BN")]
    Brunei,
    #[serde(rename = "BG")]
    Bulgaria,
    #[serde(rename = "BF")]
    BurkinaFaso,
    #[serde(rename = "BI")]
    Burundi,
    #[serde(rename = "KH")]
    Cambodia,
    #[serde(rename = "CM")]
    Cameroon,
    #[serde(rename = "CA")]
    Canada,
    #[serde(rename = "CV")]
    CapeVerde,
    #[serde(rename = "KY")]
    CaymanIslands,
    #[serde(rename = "TD")]
    Chad,
    #[serde(rename = "CL")]
    Chile,
    #[serde(rename = "C2")]
    China,
    #[serde(rename = "CO")]
    Colombia,
    #[serde(rename = "KM")]
    Comoros,
    #[serde(rename = "CG")]
    CongoBrazzaville,
    #[serde(rename = "CD")]
    CongoKinshasa,
    #[serde(rename = "CK")]
    CookIslands,
    #[serde(rename = "CR")]
    CostaRica,
    #[serde(rename = "CI")]
    CoteDIvoire,
    #[serde(rename = "HR")]
    Croatia,
    #[serde(rename = "CY")]
    Cyprus,
    #[serde(rename = "CZ")]
    CzechRepublic,
    #[serde(rename = "DK")]
    Denmark,
    #[serde(rename = "DJ")]
    Djibouti,
    #[serde(rename = "DM")]
    Dominica,
    #[serde(rename = "DO")]
    DominicanRepublic,
    #[serde(rename = "EC")]
    Ecuador,
    #[serde(rename = "EG")]
    Egypt,
    #[serde(rename = "SV")]
    ElSalvador,
    #[serde(rename = "ER")]
    Eritrea,
    #[serde(rename = "EE")]
    Estonia,
    #[serde(rename = "ET")]
    Ethiopia,
    #[serde(rename = "FK")]
    FalklandIslands,
    #[serde(rename = "FO")]
    FaroeIslands,
    #[serde(rename = "FJ")]
    Fiji,
    #[serde(rename = "FI")]
    Finland,
    #[serde(rename = "FR")]
    France,
    #[serde(rename = "GF")]
    FrenchGuiana,
    #[serde(rename = "PF")]
    FrenchPolynesia,
    #[serde(rename = "GA")]
    Gabon,
    #[serde(rename = "GM")]
    Gambia,
    #[serde(rename = "GE")]
    Georgia,
    #[default]
    #[serde(rename = "DE")]
    Germany,
    #[serde(rename = "GI")]
    Gibraltar,
    #[serde(rename = "GR")]
    Greece,
    #[serde(rename = "GL")]
    Greenland,
    #[serde(rename = "GD")]
    Grenada,
    #[serde(rename = "GP")]
    Guadeloupe,
    #[serde(rename = "GT")]
    Guatemala,
    #[serde(rename = "GN")]
    Guinea,
    #[serde(rename = "GW")]
    GuineaBissau,
    #[serde(rename = "GY")]
    Guyana,
    #[serde(rename = "HN")]
    Honduras,
    #[serde(rename = "HK")]
    HongKongSARChina,
    #[serde(rename = "HU")]
    Hungary,
    #[serde(rename = "IS")]
    Iceland,
    #[serde(rename = "IN")]
    India,
    #[serde(rename = "ID")]
    Indonesia,
    #[serde(rename = "IE")]
    Ireland,
    #[serde(rename = "IL")]
    Israel,
    #[serde(rename = "IT")]
    Italy,
    #[serde(rename = "JM")]
    Jamaica,
    #[serde(rename = "JP")]
    Japan,
    #[serde(rename = "JO")]
    Jordan,
    #[serde(rename = "KZ")]
    Kazakhstan,
    #[serde(rename = "KE")]
    Kenya,
    #[serde(rename = "KI")]
    Kiribati,
    #[serde(rename = "KW")]
    Kuwait,
    #[serde(rename = "KG")]
    Kyrgyzstan,
    #[serde(rename = "LA")]
    Laos,
    #[serde(rename = "LV")]
    Latvia,
    #[serde(rename = "LS")]
    Lesotho,
    #[serde(rename = "LI")]
    Liechtenstein,
    #[serde(rename = "LT")]
    Lithuania,
    #[serde(rename = "LU")]
    Luxembourg,
    #[serde(rename = "MK")]
    Macedonia,
    #[serde(rename = "MG")]
    Madagascar,
    #[serde(rename = "MW")]
    Malawi,
    #[serde(rename = "MY")]
    Malaysia,
    #[serde(rename = "MV")]
    Maldives,
    #[serde(rename = "ML")]
    Mali,
    #[serde(rename = "MT")]
    Malta,
    #[serde(rename = "MH")]
    MarshallIslands,
    #[serde(rename = "MQ")]
    Martinique,
    #[serde(rename = "MR")]
    Mauritania,
    #[serde(rename = "MU")]
    Mauritius,
    #[serde(rename = "YT")]
    Mayotte,
    #[serde(rename = "MX")]
    Mexico,
    #[serde(rename = "FM")]
    Micronesia,
    #[serde(rename = "MD")]
    Moldova,
    #[serde(rename = "MC")]
    Monaco,
    #[serde(rename = "MN")]
    Mongolia,
    #[serde(rename = "ME")]
    Montenegro,
    #[serde(rename = "MS")]
    Montserrat,
    #[serde(rename = "MA")]
    Morocco,
    #[serde(rename = "MZ")]
    Mozambique,
    #[serde(rename = "NA")]
    Namibia,
    #[serde(rename = "NR")]
    Nauru,
    #[serde(rename = "NP")]
    Nepal,
    #[serde(rename = "NL")]
    Netherlands,
    #[serde(rename = "NC")]
    NewCaledonia,
    #[serde(rename = "NZ")]
    NewZealand,
    #[serde(rename = "NI")]
    Nicaragua,
    #[serde(rename = "NE")]
    Niger,
    #[serde(rename = "NG")]
    Nigeria,
    #[serde(rename = "NU")]
    Niue,
    #[serde(rename = "NF")]
    NorfolkIsland,
    #[serde(rename = "NO")]
    Norway,
    #[serde(rename = "OM")]
    Oman,
    #[serde(rename = "PW")]
    Palau,
    #[serde(rename = "PA")]
    Panama,
    #[serde(rename = "PG")]
    PapuaNewGuinea,
    #[serde(rename = "PY")]
    Paraguay,
    #[serde(rename = "PE")]
    Peru,
    #[serde(rename = "PH")]
    Philippines,
    #[serde(rename = "PN")]
    PitcairnIslands,
    #[serde(rename = "PL")]
    Poland,
    #[serde(rename = "PT")]
    Portugal,
    #[serde(rename = "QA")]
    Qatar,
    #[serde(rename = "RE")]
    Reunion,
    #[serde(rename = "RO")]
    Romania,
    #[serde(rename = "RU")]
    Russia,
    #[serde(rename = "RW")]
    Rwanda,
    #[serde(rename = "WS")]
    Samoa,
    #[serde(rename = "SM")]
    SanMarino,
    #[serde(rename = "ST")]
    SaoTomeAndPrincipe,
    #[serde(rename = "SA")]
    SaudiArabia,
    #[serde(rename = "SN")]
    Senegal,
    #[serde(rename = "RS")]
    Serbia,
    #[serde(rename = "SC")]
    Seychelles,
    #[serde(rename = "SL")]
    SierraLeone,
    #[serde(rename = "ed")]
    Singapore,
    #[serde(rename = "SK")]
    Slovakia,
    #[serde(rename = "SI")]
    Slovenia,
    #[serde(rename = "SB")]
    SolomonIslands,
    #[serde(rename = "SO")]
    Somalia,
    #[serde(rename = "ZA")]
    SouthAfrica,
    #[serde(rename = "KR")]
    SouthKorea,
    #[serde(rename = "ES")]
    Spain,
    #[serde(rename = "LK")]
    SriLanka,
    #[serde(rename = "SH")]
    StHelena,
    #[serde(rename = "KN")]
    StKittsAndNevis,
    #[serde(rename = "LC")]
    StLucia,
    #[serde(rename = "PM")]
    StPierreAndMiquelon,
    #[serde(rename = "VC")]
    StVincentAndGrenadines,
    #[serde(rename = "SR")]
    Suriname,
    #[serde(rename = "SJ")]
    SvalbardAndJanMayen,
    #[serde(rename = "SZ")]
    Swaziland,
    #[serde(rename = "SE")]
    Sweden,
    #[serde(rename = "CH")]
    Switzerland,
    #[serde(rename = "TW")]
    Taiwan,
    #[serde(rename = "TJ")]
    Tajikistan,
    #[serde(rename = "TZ")]
    Tanzania,
    #[serde(rename = "TH")]
    Thailand,
    #[serde(rename = "TG")]
    Togo,
    #[serde(rename = "TO")]
    Tonga,
    #[serde(rename = "TT")]
    TrinidadAndTobago,
    #[serde(rename = "TN")]
    Tunisia,
    #[serde(rename = "TM")]
    Turkmenistan,
    #[serde(rename = "TC")]
    TurksAndCaicosIslands,
    #[serde(rename = "TV")]
    Tuvalu,
    #[serde(rename = "UG")]
    Uganda,
    #[serde(rename = "UA")]
    Ukraine,
    #[serde(rename = "al")]
    UnitedArabEmirates,
    #[serde(rename = "GB")]
    UnitedKingdom,
    #[serde(rename = "US")]
    UnitedStates,
    #[serde(rename = "UY")]
    Uruguay,
    #[serde(rename = "VU")]
    Vanuatu,
    #[serde(rename = "VA")]
    VaticanCity,
    #[serde(rename = "VE")]
    Venezuela,
    #[serde(rename = "VN")]
    Vietnam,
    #[serde(rename = "WF")]
    WallisAndFutuna,
    #[serde(rename = "YE")]
    Yemen,
    #[serde(rename = "ZM")]
    Zambia,
    #[serde(rename = "ZW")]
    Zimbabwe,
}

impl CountryCodes {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Albania => "AL",
            Self::Algeria => "DZ",
            Self::Andorra => "AD",
            Self::Angola => "AO",
            Self::Anguilla => "AI",
            Self::AntiguaAndBarbuda => "AG",
            Self::Argentina => "AR",
            Self::Armenia => "AM",
            Self::Aruba => "AW",
            Self::Australia => "AU",
            Self::Austria => "AT",
            Self::Azerbaijan => "AZ",
            Self::Bahamas => "BS",
            Self::Bahrain => "BH",
            Self::Barbados => "BB",
            Self::Belarus => "BY",
            Self::Belgium => "BE",
            Self::Belize => "BZ",
            Self::Benin => "BJ",
            Self::Bermuda => "BM",
            Self::Bhutan => "BT",
            Self::Bolivia => "BO",
            Self::BosniaAndHerzegovina => "BA",
            Self::Botswana => "BW",
            Self::Brazil => "BR",
            Self::BritishVirginIslands => "VG",
            Self::Brunei => "BN",
            Self::Bulgaria => "BG",
            Self::BurkinaFaso => "BF",
            Self::Burundi => "BI",
            Self::Cambodia => "KH",
            Self::Cameroon => "CM",
            Self::Canada => "CA",
            Self::CapeVerde => "CV",
            Self::CaymanIslands => "KY",
            Self::Chad => "TD",
            Self::Chile => "CL",
            Self::China => "C2",
            Self::Colombia => "CO",
            Self::Comoros => "KM",
            Self::CongoBrazzaville => "CG",
            Self::CongoKinshasa => "CD",
            Self::CookIslands => "CK",
            Self::CostaRica => "CR",
            Self::CoteDIvoire => "CI",
            Self::Croatia => "HR",
            Self::Cyprus => "CY",
            Self::CzechRepublic => "CZ",
            Self::Denmark => "DK",
            Self::Djibouti => "DJ",
            Self::Dominica => "DM",
            Self::DominicanRepublic => "DO",
            Self::Ecuador => "EC",
            Self::Egypt => "EG",
            Self::ElSalvador => "SV",
            Self::Eritrea => "ER",
            Self::Estonia => "EE",
            Self::Ethiopia => "ET",
            Self::FalklandIslands => "FK",
            Self::FaroeIslands => "FO",
            Self::Fiji => "FJ",
            Self::Finland => "FI",
            Self::France => "FR",
            Self::FrenchGuiana => "GF",
            Self::FrenchPolynesia => "PF",
            Self::Gabon => "GA",
            Self::Gambia => "GM",
            Self::Georgia => "GE",
            Self::Germany => "DE",
            Self::Gibraltar => "GI",
            Self::Greece => "GR",
            Self::Greenland => "GL",
            Self::Grenada => "GD",
            Self::Guadeloupe => "GP",
            Self::Guatemala => "GT",
            Self::Guinea => "GN",
            Self::GuineaBissau => "GW",
            Self::Guyana => "GY",
            Self::Honduras => "HN",
            Self::HongKongSARChina => "HK",
            Self::Hungary => "HU",
            Self::Iceland => "IS",
            Self::India => "IN",
            Self::Indonesia => "ID",
            Self::Ireland => "IE",
            Self::Israel => "IL",
            Self::Italy => "IT",
            Self::Jamaica => "JM",
            Self::Japan => "JP",
            Self::Jordan => "JO",
            Self::Kazakhstan => "KZ",
            Self::Kenya => "KE",
            Self::Kiribati => "KI",
            Self::Kuwait => "KW",
            Self::Kyrgyzstan => "KG",
            Self::Laos => "LA",
            Self::Latvia => "LV",
            Self::Lesotho => "LS",
            Self::Liechtenstein => "LI",
            Self::Lithuania => "LT",
            Self::Luxembourg => "LU",
            Self::Macedonia => "MK",
            Self::Madagascar => "MG",
            Self::Malawi => "MW",
            Self::Malaysia => "MY",
            Self::Maldives => "MV",
            Self::Mali => "ML",
            Self::Malta => "MT",
            Self::MarshallIslands => "MH",
            Self::Martinique => "MQ",
            Self::Mauritania => "MR",
            Self::Mauritius => "MU",
            Self::Mayotte => "YT",
            Self::Mexico => "MX",
            Self::Micronesia => "FM",
            Self::Moldova => "MD",
            Self::Monaco => "MC",
            Self::Mongolia => "MN",
            Self::Montenegro => "ME",
            Self::Montserrat => "MS",
            Self::Morocco => "MA",
            Self::Mozambique => "MZ",
            Self::Namibia => "NA",
            Self::Nauru => "NR",
            Self::Nepal => "NP",
            Self::Netherlands => "NL",
            Self::NewCaledonia => "NC",
            Self::NewZealand => "NZ",
            Self::Nicaragua => "NI",
            Self::Niger => "NE",
            Self::Nigeria => "NG",
            Self::Niue => "NU",
            Self::NorfolkIsland => "NF",
            Self::Norway => "NO",
            Self::Oman => "OM",
            Self::Palau => "PW",
            Self::Panama => "PA",
            Self::PapuaNewGuinea => "PG",
            Self::Paraguay => "PY",
            Self::Peru => "PE",
            Self::Philippines => "PH",
            Self::PitcairnIslands => "PN",
            Self::Poland => "PL",
            Self::Portugal => "PT",
            Self::Qatar => "QA",
            Self::Reunion => "RE",
            Self::Romania => "RO",
            Self::Russia => "RU",
            Self::Rwanda => "RW",
            Self::Samoa => "WS",
            Self::SanMarino => "SM",
            Self::SaoTomeAndPrincipe => "ST",
            Self::SaudiArabia => "SA",
            Self::Senegal => "SN",
            Self::Serbia => "RS",
            Self::Seychelles => "SC",
            Self::SierraLeone => "SL",
            Self::Singapore => "SG",
            Self::Slovakia => "SK",
            Self::Slovenia => "SI",
            Self::SolomonIslands => "SB",
            Self::Somalia => "SO",
            Self::SouthAfrica => "ZA",
            Self::SouthKorea => "KR",
            Self::Spain => "ES",
            Self::SriLanka => "LK",
            Self::StHelena => "SH",
            Self::StKittsAndNevis => "KN",
            Self::StLucia => "LC",
            Self::StPierreAndMiquelon => "PM",
            Self::StVincentAndGrenadines => "VC",
            Self::Suriname => "SR",
            Self::SvalbardAndJanMayen => "SJ",
            Self::Swaziland => "SZ",
            Self::Sweden => "SE",
            Self::Switzerland => "CH",
            Self::Taiwan => "TW",
            Self::Tajikistan => "TJ",
            Self::Tanzania => "TZ",
            Self::Thailand => "TH",
            Self::Togo => "TG",
            Self::Tonga => "TO",
            Self::TrinidadAndTobago => "TT",
            Self::Tunisia => "TN",
            Self::Turkmenistan => "TM",
            Self::TurksAndCaicosIslands => "TC",
            Self::Tuvalu => "TV",
            Self::Uganda => "UG",
            Self::Ukraine => "UA",
            Self::UnitedArabEmirates => "AE",
            Self::UnitedKingdom => "GB",
            Self::UnitedStates => "US",
            Self::Uruguay => "UY",
            Self::Vanuatu => "VU",
            Self::VaticanCity => "VA",
            Self::Venezuela => "VE",
            Self::Vietnam => "VN",
            Self::WallisAndFutuna => "WF",
            Self::Yemen => "YE",
            Self::Zambia => "ZM",
            Self::Zimbabwe => "ZW",
        }
    }
}

impl AsRef<str> for CountryCodes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CountryCodes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
