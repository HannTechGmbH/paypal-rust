use serde::{Deserialize, Serialize};

/// Processor response code for the non-PayPal payment processor errors.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ResponseCode {
    #[serde(rename = "0000")]
    Approved,
    #[serde(rename = "0100")]
    REFERRAL,
    #[serde(rename = "0800")]
    BadResponseReversalRequired,
    #[serde(rename = "1000")]
    PartialAuthorization,
    #[serde(rename = "1300")]
    InvalidDataFormat,
    #[serde(rename = "1310")]
    InvalidAmount,
    #[serde(rename = "1312")]
    InvalidTransactionCardIssuerAcquirer,
    #[serde(rename = "1317")]
    InvalidCaptureDate,
    #[serde(rename = "1320")]
    InvalidCurrencyCode,
    #[serde(rename = "1330")]
    InvalidAccount,
    #[serde(rename = "1335")]
    InvalidAccountRecurring,
    #[serde(rename = "1340")]
    InvalidTerminal,
    #[serde(rename = "1350")]
    InvalidMerchant,
    #[serde(rename = "1360")]
    BadProcessingCode,
    #[serde(rename = "1370")]
    InvalidMcc,
    #[serde(rename = "1380")]
    InvalidExpiration,
    #[serde(rename = "1382")]
    InvalidCardVerificationValue,
    #[serde(rename = "1384")]
    InvalidLifeCycleOfTransaction,
    #[serde(rename = "1390")]
    InvalidOrder,
    #[serde(rename = "1393")]
    TransactionCannotBeCompleted,
    #[serde(rename = "0500")]
    DoNotHonor,
    #[serde(rename = "5100")]
    GenericDecline,
    #[serde(rename = "5110")]
    CVV2Failure,
    #[serde(rename = "5120")]
    InsufficientFunds,
    #[serde(rename = "5130")]
    InvalidPin,
    #[serde(rename = "5140")]
    CardClosed,
    #[serde(rename = "5150")]
    PickupCardSpecialConditions,
    #[serde(rename = "5160")]
    UnauthorizedUser,
    #[serde(rename = "5170")]
    AVSFailure,
    #[serde(rename = "5180")]
    InvalidOrRestrictedCard,
    #[serde(rename = "5190")]
    SoftAvs,
    #[serde(rename = "5200")]
    DuplicateTransaction,
    #[serde(rename = "5210")]
    InvalidTransaction,
    #[serde(rename = "5400")]
    ExpiredCard,
    #[serde(rename = "5500")]
    IncorrectPinReentered,
    #[serde(rename = "5700")]
    TransactionNotPermitted,
    #[serde(rename = "5800")]
    ReversalRejected,
    #[serde(rename = "5900")]
    InvalidIssue,
    #[serde(rename = "5910")]
    IssuerNotAvailableNotRetriable,
    #[serde(rename = "5920")]
    IssuerNotAvailableRetriable,
    #[serde(rename = "6300")]
    AccountNotOnFile,
    #[serde(rename = "7600")]
    ApprovedNonCapture,
    #[serde(rename = "7700")]
    Error3DS,
    #[serde(rename = "7710")]
    AuthenticationFailed,
    #[serde(rename = "7800")]
    BinError,
    #[serde(rename = "7900")]
    PinError,
    #[serde(rename = "8000")]
    ProcessorSystemError,
    #[serde(rename = "8010")]
    HostKeyError,
    #[serde(rename = "8020")]
    ConfigurationError,
    #[serde(rename = "8030")]
    UnsupportedTransaction,
    #[serde(rename = "8100")]
    FatalCommunicationError,
    #[serde(rename = "8110")]
    RetriableCommunicationError,
    #[serde(rename = "8220")]
    SystemUnavailable,
    #[serde(rename = "9100")]
    DeclinedPleaseRetry,
    #[serde(rename = "9500")]
    SuspectedFraud,
    #[serde(rename = "9510")]
    SecurityViolation,
    #[serde(rename = "9520")]
    LostOrStolen,
    #[serde(rename = "9530")]
    HoldCallCenter,
    #[serde(rename = "9540")]
    RefusedCard,
    #[serde(rename = "9600")]
    UnrecognizedResponseCode,
    #[serde(rename = "5930")]
    CardNotActivated,
    #[serde(rename = "PPMD")]
    ProMidUndefined,
    #[serde(rename = "PPCE")]
    CeRegistrationIncomplete,
    #[serde(rename = "PPNT")]
    NetworkError,
    #[serde(rename = "PPCN")]
    ConnectionError,
    #[serde(rename = "PPCT")]
    CardTypeUnsupported,
    #[serde(rename = "PPTT")]
    TransactionTypeUnsupported,
    #[serde(rename = "PPCU")]
    CurrencyUsedInvalid,
    #[serde(rename = "PPQC")]
    QuasiCashUnsupported,
    #[serde(rename = "PPVE")]
    ValidationError,
    #[serde(rename = "PPVT")]
    VirtualTerminalUnsupported,
    #[serde(rename = "PPDC")]
    DccUnsupported,
    #[serde(rename = "PPER")]
    InternalSystemError,
    #[serde(rename = "PPIM")]
    IdMismatch,
    #[serde(rename = "PPH1")]
    H1Error,
    #[serde(rename = "PPSD")]
    StatusDescription,
    #[serde(rename = "PPAG")]
    AdultGamingUnsupported,
    #[serde(rename = "PPLS")]
    LargeStatusCode,
    #[serde(rename = "PPCO")]
    Country,
    #[serde(rename = "PPAD")]
    BillingAddress,
    #[serde(rename = "PPAU")]
    MCCCode,
    #[serde(rename = "PPUC")]
    CurrencyCodeUnsupported,
    #[serde(rename = "PPUR")]
    UnsupportedReversal,
    #[serde(rename = "PPVC")]
    ValidateCurrency,
    #[serde(rename = "PPS0")]
    BankAuthRowMismatch,
    #[serde(rename = "PPS1")]
    BankAuthRowNotFound,
    #[serde(rename = "PPS2")]
    BankAuthRowVoided,
    #[serde(rename = "PPS3")]
    BankAuthExpired,
    #[serde(rename = "PPS4")]
    CurrencyMismatch,
    #[serde(rename = "PPS5")]
    CreditCardMismatch,
    #[serde(rename = "PPS6")]
    AmountMismatch,
    #[serde(rename = "PPRF")]
    InvalidParentTransactionStatus,
    #[serde(rename = "PPEX")]
    ExpiryDate,
    #[serde(rename = "PPAX")]
    AmountExceeded,
    #[serde(rename = "PPDV")]
    AuthMessage,
    #[serde(rename = "PPDI")]
    DinersReject,
    #[serde(rename = "PPAR")]
    AuthResult,
    #[serde(rename = "PPBG")]
    BadGaming,
    #[serde(rename = "PPGR")]
    GamingRefundError,
    #[serde(rename = "PPCR")]
    CreditError,
    #[serde(rename = "PPAI")]
    AmountIncompatible,
    #[serde(rename = "PPIF")]
    IdempotencyFailure,
    #[serde(rename = "PPMC")]
    BlockedMastercard,
    #[serde(rename = "PPAE")]
    AmexDisabled,
    #[serde(rename = "PPFV")]
    FieldValidationFailed,
    #[serde(rename = "PPII")]
    InvalidInputFailure,
    #[serde(rename = "PPPM")]
    InvalidPaymentMethod,
    #[serde(rename = "PPUA")]
    UserNotAuthorized,
    #[serde(rename = "PPFI")]
    InvalidFundingInstrument,
    #[serde(rename = "PPEF")]
    ExpiredFundingInstrument,
    #[serde(rename = "PPFR")]
    RestrictedFundingInstrument,
    #[serde(rename = "PPEL")]
    ExceedsFrequencyLimit,
    #[serde(rename = "PCVV")]
    CVVFailure,
    #[serde(rename = "PPTV")]
    InvalidVerificationToken,
    #[serde(rename = "PPTE")]
    VerificationTokenExpired,
    #[serde(rename = "PPPI")]
    InvalidProduct,
    #[serde(rename = "PPIT")]
    InvalidTraceId,
    #[serde(rename = "PPTF")]
    InvalidTraceReference,
    #[serde(rename = "PPFE")]
    FundingSourceAlreadyExists,
    #[serde(rename = "PPTR")]
    VerificationTokenRevoked,
    #[serde(rename = "PPTI")]
    InvalidTransactionId,
    #[serde(rename = "PPD3")]
    SecureError3DS,
    #[serde(rename = "PPPH")]
    NoPhoneForDCCTransaction,
    #[serde(rename = "PPAV")]
    ArcAvs,
    #[serde(rename = "PPC2")]
    ArcCvv,
    #[serde(rename = "PPDB")]
    NoDobPresent,
    #[serde(rename = "PPLR")]
    LateReversal,
    #[serde(rename = "PPNC")]
    NotSupportedNrc,
    #[serde(rename = "PPRR")]
    MerchantNotRegistered,
    #[serde(rename = "PPSC")]
    ArcScore,
    #[serde(rename = "PPSE")]
    AmexDenied,
    #[serde(rename = "PPUE")]
    UnsupportEntity,
    #[serde(rename = "PPUP")]
    UnsupportPosFlag,
    #[serde(rename = "PPRE")]
    UnsupportRefundOnPendingBc,
}

impl ResponseCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "0000",
            Self::REFERRAL => "0100",
            Self::BadResponseReversalRequired => "0800",
            Self::PartialAuthorization => "1000",
            Self::InvalidDataFormat => "1300",
            Self::InvalidAmount => "1310",
            Self::InvalidTransactionCardIssuerAcquirer => "1312",
            Self::InvalidCaptureDate => "1317",
            Self::InvalidCurrencyCode => "1320",
            Self::InvalidAccount => "1330",
            Self::InvalidAccountRecurring => "1335",
            Self::InvalidTerminal => "1340",
            Self::InvalidMerchant => "1350",
            Self::BadProcessingCode => "1360",
            Self::InvalidMcc => "1370",
            Self::InvalidExpiration => "1380",
            Self::InvalidCardVerificationValue => "1382",
            Self::InvalidLifeCycleOfTransaction => "1384",
            Self::InvalidOrder => "1390",
            Self::TransactionCannotBeCompleted => "1393",
            Self::DoNotHonor => "0500",
            Self::GenericDecline => "5100",
            Self::CVV2Failure => "5110",
            Self::InsufficientFunds => "5120",
            Self::InvalidPin => "5130",
            Self::CardClosed => "5140",
            Self::PickupCardSpecialConditions => "5150",
            Self::UnauthorizedUser => "5160",
            Self::AVSFailure => "5170",
            Self::InvalidOrRestrictedCard => "5180",
            Self::SoftAvs => "5190",
            Self::DuplicateTransaction => "5200",
            Self::InvalidTransaction => "5210",
            Self::ExpiredCard => "5400",
            Self::IncorrectPinReentered => "5500",
            Self::TransactionNotPermitted => "5700",
            Self::ReversalRejected => "5800",
            Self::InvalidIssue => "5900",
            Self::IssuerNotAvailableNotRetriable => "5910",
            Self::IssuerNotAvailableRetriable => "5920",
            Self::AccountNotOnFile => "6300",
            Self::ApprovedNonCapture => "7600",
            Self::Error3DS => "7700",
            Self::AuthenticationFailed => "7710",
            Self::BinError => "7800",
            Self::PinError => "7900",
            Self::ProcessorSystemError => "8000",
            Self::HostKeyError => "8010",
            Self::ConfigurationError => "8020",
            Self::UnsupportedTransaction => "8030",
            Self::FatalCommunicationError => "8100",
            Self::RetriableCommunicationError => "8110",
            Self::SystemUnavailable => "8220",
            Self::DeclinedPleaseRetry => "9100",
            Self::SuspectedFraud => "9500",
            Self::SecurityViolation => "9510",
            Self::LostOrStolen => "9520",
            Self::HoldCallCenter => "9530",
            Self::RefusedCard => "9540",
            Self::UnrecognizedResponseCode => "9600",
            Self::CardNotActivated => "5930",
            Self::ProMidUndefined => "PPMD",
            Self::CeRegistrationIncomplete => "PPCE",
            Self::NetworkError => "PPNT",
            Self::ConnectionError => "PPCN",
            Self::CardTypeUnsupported => "PPCT",
            Self::TransactionTypeUnsupported => "PPTT",
            Self::CurrencyUsedInvalid => "PPCU",
            Self::QuasiCashUnsupported => "PPQC",
            Self::ValidationError => "PPVE",
            Self::VirtualTerminalUnsupported => "PPVT",
            Self::DccUnsupported => "PPDC",
            Self::InternalSystemError => "PPER",
            Self::IdMismatch => "PPIM",
            Self::H1Error => "PPH1",
            Self::StatusDescription => "PPSD",
            Self::AdultGamingUnsupported => "PPAG",
            Self::LargeStatusCode => "PPLS",
            Self::Country => "PPCO",
            Self::BillingAddress => "PPAD",
            Self::MCCCode => "PPAU",
            Self::CurrencyCodeUnsupported => "PPUC",
            Self::UnsupportedReversal => "PPUR",
            Self::ValidateCurrency => "PPVC",
            Self::BankAuthRowMismatch => "PPS0",
            Self::BankAuthRowNotFound => "PPS1",
            Self::BankAuthRowVoided => "PPS2",
            Self::BankAuthExpired => "PPS3",
            Self::CurrencyMismatch => "PPS4",
            Self::CreditCardMismatch => "PPS5",
            Self::AmountMismatch => "PPS6",
            Self::InvalidParentTransactionStatus => "PPRF",
            Self::ExpiryDate => "PPEX",
            Self::AmountExceeded => "PPAX",
            Self::AuthMessage => "PPDV",
            Self::DinersReject => "PPDI",
            Self::AuthResult => "PPAR",
            Self::BadGaming => "PPBG",
            Self::GamingRefundError => "PPGR",
            Self::CreditError => "PPCR",
            Self::AmountIncompatible => "PPAI",
            Self::IdempotencyFailure => "PPIF",
            Self::BlockedMastercard => "PPMC",
            Self::AmexDisabled => "PPAE",
            Self::FieldValidationFailed => "PPFV",
            Self::InvalidInputFailure => "PPII",
            Self::InvalidPaymentMethod => "PPPM",
            Self::UserNotAuthorized => "PPUA",
            Self::InvalidFundingInstrument => "PPFI",
            Self::ExpiredFundingInstrument => "PPEF",
            Self::RestrictedFundingInstrument => "PPFR",
            Self::ExceedsFrequencyLimit => "PPEL",
            Self::CVVFailure => "PCVV",
            Self::InvalidVerificationToken => "PPTV",
            Self::VerificationTokenExpired => "PPTE",
            Self::InvalidProduct => "PPPI",
            Self::InvalidTraceId => "PPIT",
            Self::InvalidTraceReference => "PPTF",
            Self::FundingSourceAlreadyExists => "PPFE",
            Self::VerificationTokenRevoked => "PPTR",
            Self::InvalidTransactionId => "PPTI",
            Self::SecureError3DS => "PPD3",
            Self::NoPhoneForDCCTransaction => "PPPH",
            Self::ArcAvs => "PPAV",
            Self::ArcCvv => "PPC2",
            Self::NoDobPresent => "PPDB",
            Self::LateReversal => "PPLR",
            Self::NotSupportedNrc => "PPNC",
            Self::MerchantNotRegistered => "PPRR",
            Self::ArcScore => "PPSC",
            Self::AmexDenied => "PPSE",
            Self::UnsupportEntity => "PPUE",
            Self::UnsupportPosFlag => "PPUP",
            Self::UnsupportRefundOnPendingBc => "PPRE",
        }
    }
}

impl AsRef<str> for ResponseCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ResponseCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
