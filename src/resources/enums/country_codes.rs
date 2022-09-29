use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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
    pub fn as_str(self) -> &'static str {
        match self {
            CountryCodes::Albania => "AL",
            CountryCodes::Algeria => "DZ",
            CountryCodes::Andorra => "AD",
            CountryCodes::Angola => "AO",
            CountryCodes::Anguilla => "AI",
            CountryCodes::AntiguaAndBarbuda => "AG",
            CountryCodes::Argentina => "AR",
            CountryCodes::Armenia => "AM",
            CountryCodes::Aruba => "AW",
            CountryCodes::Australia => "AU",
            CountryCodes::Austria => "AT",
            CountryCodes::Azerbaijan => "AZ",
            CountryCodes::Bahamas => "BS",
            CountryCodes::Bahrain => "BH",
            CountryCodes::Barbados => "BB",
            CountryCodes::Belarus => "BY",
            CountryCodes::Belgium => "BE",
            CountryCodes::Belize => "BZ",
            CountryCodes::Benin => "BJ",
            CountryCodes::Bermuda => "BM",
            CountryCodes::Bhutan => "BT",
            CountryCodes::Bolivia => "BO",
            CountryCodes::BosniaAndHerzegovina => "BA",
            CountryCodes::Botswana => "BW",
            CountryCodes::Brazil => "BR",
            CountryCodes::BritishVirginIslands => "VG",
            CountryCodes::Brunei => "BN",
            CountryCodes::Bulgaria => "BG",
            CountryCodes::BurkinaFaso => "BF",
            CountryCodes::Burundi => "BI",
            CountryCodes::Cambodia => "KH",
            CountryCodes::Cameroon => "CM",
            CountryCodes::Canada => "CA",
            CountryCodes::CapeVerde => "CV",
            CountryCodes::CaymanIslands => "KY",
            CountryCodes::Chad => "TD",
            CountryCodes::Chile => "CL",
            CountryCodes::China => "C2",
            CountryCodes::Colombia => "CO",
            CountryCodes::Comoros => "KM",
            CountryCodes::CongoBrazzaville => "CG",
            CountryCodes::CongoKinshasa => "CD",
            CountryCodes::CookIslands => "CK",
            CountryCodes::CostaRica => "CR",
            CountryCodes::CoteDIvoire => "CI",
            CountryCodes::Croatia => "HR",
            CountryCodes::Cyprus => "CY",
            CountryCodes::CzechRepublic => "CZ",
            CountryCodes::Denmark => "DK",
            CountryCodes::Djibouti => "DJ",
            CountryCodes::Dominica => "DM",
            CountryCodes::DominicanRepublic => "DO",
            CountryCodes::Ecuador => "EC",
            CountryCodes::Egypt => "EG",
            CountryCodes::ElSalvador => "SV",
            CountryCodes::Eritrea => "ER",
            CountryCodes::Estonia => "EE",
            CountryCodes::Ethiopia => "ET",
            CountryCodes::FalklandIslands => "FK",
            CountryCodes::FaroeIslands => "FO",
            CountryCodes::Fiji => "FJ",
            CountryCodes::Finland => "FI",
            CountryCodes::France => "FR",
            CountryCodes::FrenchGuiana => "GF",
            CountryCodes::FrenchPolynesia => "PF",
            CountryCodes::Gabon => "GA",
            CountryCodes::Gambia => "GM",
            CountryCodes::Georgia => "GE",
            CountryCodes::Germany => "DE",
            CountryCodes::Gibraltar => "GI",
            CountryCodes::Greece => "GR",
            CountryCodes::Greenland => "GL",
            CountryCodes::Grenada => "GD",
            CountryCodes::Guadeloupe => "GP",
            CountryCodes::Guatemala => "GT",
            CountryCodes::Guinea => "GN",
            CountryCodes::GuineaBissau => "GW",
            CountryCodes::Guyana => "GY",
            CountryCodes::Honduras => "HN",
            CountryCodes::HongKongSARChina => "HK",
            CountryCodes::Hungary => "HU",
            CountryCodes::Iceland => "IS",
            CountryCodes::India => "IN",
            CountryCodes::Indonesia => "ID",
            CountryCodes::Ireland => "IE",
            CountryCodes::Israel => "IL",
            CountryCodes::Italy => "IT",
            CountryCodes::Jamaica => "JM",
            CountryCodes::Japan => "JP",
            CountryCodes::Jordan => "JO",
            CountryCodes::Kazakhstan => "KZ",
            CountryCodes::Kenya => "KE",
            CountryCodes::Kiribati => "KI",
            CountryCodes::Kuwait => "KW",
            CountryCodes::Kyrgyzstan => "KG",
            CountryCodes::Laos => "LA",
            CountryCodes::Latvia => "LV",
            CountryCodes::Lesotho => "LS",
            CountryCodes::Liechtenstein => "LI",
            CountryCodes::Lithuania => "LT",
            CountryCodes::Luxembourg => "LU",
            CountryCodes::Macedonia => "MK",
            CountryCodes::Madagascar => "MG",
            CountryCodes::Malawi => "MW",
            CountryCodes::Malaysia => "MY",
            CountryCodes::Maldives => "MV",
            CountryCodes::Mali => "ML",
            CountryCodes::Malta => "MT",
            CountryCodes::MarshallIslands => "MH",
            CountryCodes::Martinique => "MQ",
            CountryCodes::Mauritania => "MR",
            CountryCodes::Mauritius => "MU",
            CountryCodes::Mayotte => "YT",
            CountryCodes::Mexico => "MX",
            CountryCodes::Micronesia => "FM",
            CountryCodes::Moldova => "MD",
            CountryCodes::Monaco => "MC",
            CountryCodes::Mongolia => "MN",
            CountryCodes::Montenegro => "ME",
            CountryCodes::Montserrat => "MS",
            CountryCodes::Morocco => "MA",
            CountryCodes::Mozambique => "MZ",
            CountryCodes::Namibia => "NA",
            CountryCodes::Nauru => "NR",
            CountryCodes::Nepal => "NP",
            CountryCodes::Netherlands => "NL",
            CountryCodes::NewCaledonia => "NC",
            CountryCodes::NewZealand => "NZ",
            CountryCodes::Nicaragua => "NI",
            CountryCodes::Niger => "NE",
            CountryCodes::Nigeria => "NG",
            CountryCodes::Niue => "NU",
            CountryCodes::NorfolkIsland => "NF",
            CountryCodes::Norway => "NO",
            CountryCodes::Oman => "OM",
            CountryCodes::Palau => "PW",
            CountryCodes::Panama => "PA",
            CountryCodes::PapuaNewGuinea => "PG",
            CountryCodes::Paraguay => "PY",
            CountryCodes::Peru => "PE",
            CountryCodes::Philippines => "PH",
            CountryCodes::PitcairnIslands => "PN",
            CountryCodes::Poland => "PL",
            CountryCodes::Portugal => "PT",
            CountryCodes::Qatar => "QA",
            CountryCodes::Reunion => "RE",
            CountryCodes::Romania => "RO",
            CountryCodes::Russia => "RU",
            CountryCodes::Rwanda => "RW",
            CountryCodes::Samoa => "WS",
            CountryCodes::SanMarino => "SM",
            CountryCodes::SaoTomeAndPrincipe => "ST",
            CountryCodes::SaudiArabia => "SA",
            CountryCodes::Senegal => "SN",
            CountryCodes::Serbia => "RS",
            CountryCodes::Seychelles => "SC",
            CountryCodes::SierraLeone => "SL",
            CountryCodes::Singapore => "SG",
            CountryCodes::Slovakia => "SK",
            CountryCodes::Slovenia => "SI",
            CountryCodes::SolomonIslands => "SB",
            CountryCodes::Somalia => "SO",
            CountryCodes::SouthAfrica => "ZA",
            CountryCodes::SouthKorea => "KR",
            CountryCodes::Spain => "ES",
            CountryCodes::SriLanka => "LK",
            CountryCodes::StHelena => "SH",
            CountryCodes::StKittsAndNevis => "KN",
            CountryCodes::StLucia => "LC",
            CountryCodes::StPierreAndMiquelon => "PM",
            CountryCodes::StVincentAndGrenadines => "VC",
            CountryCodes::Suriname => "SR",
            CountryCodes::SvalbardAndJanMayen => "SJ",
            CountryCodes::Swaziland => "SZ",
            CountryCodes::Sweden => "SE",
            CountryCodes::Switzerland => "CH",
            CountryCodes::Taiwan => "TW",
            CountryCodes::Tajikistan => "TJ",
            CountryCodes::Tanzania => "TZ",
            CountryCodes::Thailand => "TH",
            CountryCodes::Togo => "TG",
            CountryCodes::Tonga => "TO",
            CountryCodes::TrinidadAndTobago => "TT",
            CountryCodes::Tunisia => "TN",
            CountryCodes::Turkmenistan => "TM",
            CountryCodes::TurksAndCaicosIslands => "TC",
            CountryCodes::Tuvalu => "TV",
            CountryCodes::Uganda => "UG",
            CountryCodes::Ukraine => "UA",
            CountryCodes::UnitedArabEmirates => "AE",
            CountryCodes::UnitedKingdom => "GB",
            CountryCodes::UnitedStates => "US",
            CountryCodes::Uruguay => "UY",
            CountryCodes::Vanuatu => "VU",
            CountryCodes::VaticanCity => "VA",
            CountryCodes::Venezuela => "VE",
            CountryCodes::Vietnam => "VN",
            CountryCodes::WallisAndFutuna => "WF",
            CountryCodes::Yemen => "YE",
            CountryCodes::Zambia => "ZM",
            CountryCodes::Zimbabwe => "ZW",
        }
    }
}

impl Default for CountryCodes {
    fn default() -> Self {
        CountryCodes::Germany
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
