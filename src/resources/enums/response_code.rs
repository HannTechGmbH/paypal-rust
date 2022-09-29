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
    pub fn as_str(self) -> &'static str {
        match self {
            ResponseCode::Approved => "0000",
            ResponseCode::REFERRAL => "0100",
            ResponseCode::BadResponseReversalRequired => "0800",
            ResponseCode::PartialAuthorization => "1000",
            ResponseCode::InvalidDataFormat => "1300",
            ResponseCode::InvalidAmount => "1310",
            ResponseCode::InvalidTransactionCardIssuerAcquirer => "1312",
            ResponseCode::InvalidCaptureDate => "1317",
            ResponseCode::InvalidCurrencyCode => "1320",
            ResponseCode::InvalidAccount => "1330",
            ResponseCode::InvalidAccountRecurring => "1335",
            ResponseCode::InvalidTerminal => "1340",
            ResponseCode::InvalidMerchant => "1350",
            ResponseCode::BadProcessingCode => "1360",
            ResponseCode::InvalidMcc => "1370",
            ResponseCode::InvalidExpiration => "1380",
            ResponseCode::InvalidCardVerificationValue => "1382",
            ResponseCode::InvalidLifeCycleOfTransaction => "1384",
            ResponseCode::InvalidOrder => "1390",
            ResponseCode::TransactionCannotBeCompleted => "1393",
            ResponseCode::DoNotHonor => "0500",
            ResponseCode::GenericDecline => "5100",
            ResponseCode::CVV2Failure => "5110",
            ResponseCode::InsufficientFunds => "5120",
            ResponseCode::InvalidPin => "5130",
            ResponseCode::CardClosed => "5140",
            ResponseCode::PickupCardSpecialConditions => "5150",
            ResponseCode::UnauthorizedUser => "5160",
            ResponseCode::AVSFailure => "5170",
            ResponseCode::InvalidOrRestrictedCard => "5180",
            ResponseCode::SoftAvs => "5190",
            ResponseCode::DuplicateTransaction => "5200",
            ResponseCode::InvalidTransaction => "5210",
            ResponseCode::ExpiredCard => "5400",
            ResponseCode::IncorrectPinReentered => "5500",
            ResponseCode::TransactionNotPermitted => "5700",
            ResponseCode::ReversalRejected => "5800",
            ResponseCode::InvalidIssue => "5900",
            ResponseCode::IssuerNotAvailableNotRetriable => "5910",
            ResponseCode::IssuerNotAvailableRetriable => "5920",
            ResponseCode::AccountNotOnFile => "6300",
            ResponseCode::ApprovedNonCapture => "7600",
            ResponseCode::Error3DS => "7700",
            ResponseCode::AuthenticationFailed => "7710",
            ResponseCode::BinError => "7800",
            ResponseCode::PinError => "7900",
            ResponseCode::ProcessorSystemError => "8000",
            ResponseCode::HostKeyError => "8010",
            ResponseCode::ConfigurationError => "8020",
            ResponseCode::UnsupportedTransaction => "8030",
            ResponseCode::FatalCommunicationError => "8100",
            ResponseCode::RetriableCommunicationError => "8110",
            ResponseCode::SystemUnavailable => "8220",
            ResponseCode::DeclinedPleaseRetry => "9100",
            ResponseCode::SuspectedFraud => "9500",
            ResponseCode::SecurityViolation => "9510",
            ResponseCode::LostOrStolen => "9520",
            ResponseCode::HoldCallCenter => "9530",
            ResponseCode::RefusedCard => "9540",
            ResponseCode::UnrecognizedResponseCode => "9600",
            ResponseCode::CardNotActivated => "5930",
            ResponseCode::ProMidUndefined => "PPMD",
            ResponseCode::CeRegistrationIncomplete => "PPCE",
            ResponseCode::NetworkError => "PPNT",
            ResponseCode::ConnectionError => "PPCN",
            ResponseCode::CardTypeUnsupported => "PPCT",
            ResponseCode::TransactionTypeUnsupported => "PPTT",
            ResponseCode::CurrencyUsedInvalid => "PPCU",
            ResponseCode::QuasiCashUnsupported => "PPQC",
            ResponseCode::ValidationError => "PPVE",
            ResponseCode::VirtualTerminalUnsupported => "PPVT",
            ResponseCode::DccUnsupported => "PPDC",
            ResponseCode::InternalSystemError => "PPER",
            ResponseCode::IdMismatch => "PPIM",
            ResponseCode::H1Error => "PPH1",
            ResponseCode::StatusDescription => "PPSD",
            ResponseCode::AdultGamingUnsupported => "PPAG",
            ResponseCode::LargeStatusCode => "PPLS",
            ResponseCode::Country => "PPCO",
            ResponseCode::BillingAddress => "PPAD",
            ResponseCode::MCCCode => "PPAU",
            ResponseCode::CurrencyCodeUnsupported => "PPUC",
            ResponseCode::UnsupportedReversal => "PPUR",
            ResponseCode::ValidateCurrency => "PPVC",
            ResponseCode::BankAuthRowMismatch => "PPS0",
            ResponseCode::BankAuthRowNotFound => "PPS1",
            ResponseCode::BankAuthRowVoided => "PPS2",
            ResponseCode::BankAuthExpired => "PPS3",
            ResponseCode::CurrencyMismatch => "PPS4",
            ResponseCode::CreditCardMismatch => "PPS5",
            ResponseCode::AmountMismatch => "PPS6",
            ResponseCode::InvalidParentTransactionStatus => "PPRF",
            ResponseCode::ExpiryDate => "PPEX",
            ResponseCode::AmountExceeded => "PPAX",
            ResponseCode::AuthMessage => "PPDV",
            ResponseCode::DinersReject => "PPDI",
            ResponseCode::AuthResult => "PPAR",
            ResponseCode::BadGaming => "PPBG",
            ResponseCode::GamingRefundError => "PPGR",
            ResponseCode::CreditError => "PPCR",
            ResponseCode::AmountIncompatible => "PPAI",
            ResponseCode::IdempotencyFailure => "PPIF",
            ResponseCode::BlockedMastercard => "PPMC",
            ResponseCode::AmexDisabled => "PPAE",
            ResponseCode::FieldValidationFailed => "PPFV",
            ResponseCode::InvalidInputFailure => "PPII",
            ResponseCode::InvalidPaymentMethod => "PPPM",
            ResponseCode::UserNotAuthorized => "PPUA",
            ResponseCode::InvalidFundingInstrument => "PPFI",
            ResponseCode::ExpiredFundingInstrument => "PPEF",
            ResponseCode::RestrictedFundingInstrument => "PPFR",
            ResponseCode::ExceedsFrequencyLimit => "PPEL",
            ResponseCode::CVVFailure => "PCVV",
            ResponseCode::InvalidVerificationToken => "PPTV",
            ResponseCode::VerificationTokenExpired => "PPTE",
            ResponseCode::InvalidProduct => "PPPI",
            ResponseCode::InvalidTraceId => "PPIT",
            ResponseCode::InvalidTraceReference => "PPTF",
            ResponseCode::FundingSourceAlreadyExists => "PPFE",
            ResponseCode::VerificationTokenRevoked => "PPTR",
            ResponseCode::InvalidTransactionId => "PPTI",
            ResponseCode::SecureError3DS => "PPD3",
            ResponseCode::NoPhoneForDCCTransaction => "PPPH",
            ResponseCode::ArcAvs => "PPAV",
            ResponseCode::ArcCvv => "PPC2",
            ResponseCode::NoDobPresent => "PPDB",
            ResponseCode::LateReversal => "PPLR",
            ResponseCode::NotSupportedNrc => "PPNC",
            ResponseCode::MerchantNotRegistered => "PPRR",
            ResponseCode::ArcScore => "PPSC",
            ResponseCode::AmexDenied => "PPSE",
            ResponseCode::UnsupportEntity => "PPUE",
            ResponseCode::UnsupportPosFlag => "PPUP",
            ResponseCode::UnsupportRefundOnPendingBc => "PPRE",
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
