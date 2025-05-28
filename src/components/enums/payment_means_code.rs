use serde::{Serialize, Serializer};

// based on UNTDID 4461:
// https://unece.org/fileadmin/DAM/trade/untdid/d16b/tred/tred4461.htm
// https://www.xrepository.de/details/urn:xoev-de:xrechnung:codeliste:untdid.4461_1

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PaymentMeansCode<'invoice> {
    /// Not defined legally enforceable agreement between two or
    /// more parties (expressing a contractual right or a right
    /// to the payment of money).
    InstrumentNotDefined,
    /// A credit transaction made through the automated clearing
    /// house system.
    AutomatedClearingHouseCredit,
    /// A debit transaction made through the automated clearing
    /// house system.
    AutomatedClearingHouseDebit,
    /// A request to reverse an ACH debit transaction to a
    /// demand deposit account.
    AchDemandDebitReversal,
    /// A request to reverse a credit transaction to a demand
    /// deposit account.
    AchDemandCreditReversal,
    /// A credit transaction made through the ACH system to a
    /// demand deposit account.
    AchDemandCredit,
    /// A debit transaction made through the ACH system to a
    /// demand deposit account.
    AchDemandDebit,
    /// Indicates that the bank should hold the payment for
    /// collection by the beneficiary or other instructions.
    Hold,
    /// Indicates that the payment should be made using the
    /// national or regional clearing.
    NationalOrRegionalClearing,
    /// Payment by currency (including bills and coins) in
    /// circulation, including checking account deposits.
    InCash,
    /// A request to reverse an ACH credit transaction to a
    /// savings account.
    AchSavingsCreditReversal,
    /// A request to reverse an ACH debit transaction to a
    /// savings account.
    AchSavingsDebitReversal,
    /// A credit transaction made through the ACH system to a
    /// savings account.
    AchSavingsCredit,
    /// A debit transaction made through the ACH system to a
    /// savings account.
    AchSavingsDebit,
    /// A credit entry between two accounts at the same bank
    /// branch. Synonym: house credit.
    BookentryCredit,
    /// A debit entry between two accounts at the same bank
    /// branch. Synonym: house debit.
    BookentryDebit,
    /// A credit transaction made through the ACH system to a
    /// demand deposit account using the CCD payment format.
    AchDemandCashConcentrationdisbursementCcdCredit,
    /// A debit transaction made through the ACH system to a
    /// demand deposit account using the CCD payment format.
    AchDemandCashConcentrationdisbursementCcdDebit,
    /// A credit transaction made through the ACH system to a
    /// demand deposit account using the CTP payment format.
    AchDemandCorporateTradePaymentCtpCredit,
    /// Payment by a pre-printed form on which instructions are
    /// given to an account holder (a bank or building society)
    /// to pay a stated sum to a named recipient.
    Cheque,
    /// Issue of a banker's draft in payment of the funds.
    BankersDraft,
    /// Cheque drawn by a bank on itself or its agent. A person
    /// who owes money to another buys the draft from a bank for
    /// cash and hands it to the creditor who need have no fear
    /// that it might be dishonoured.
    CertifiedBankersDraft,
    /// Payment by a pre-printed form, which has been completed
    /// by a financial institution, on which instructions are
    /// given to an account holder (a bank or building society)
    /// to pay a stated sum to a named recipient.
    BankChequeIssuedByABankingOrSimilarEstablishment,
    /// Bill drawn by the creditor on the debtor but not yet
    /// accepted by the debtor.
    BillOfExchangeAwaitingAcceptance,
    /// Payment by a pre-printed form stamped with the paying
    /// bank's certification on which instructions are given to
    /// an account holder (a bank or building society) to pay a
    /// stated sum to a named recipient .
    CertifiedCheque,
    /// Indicates that the cheque is given local to the
    /// recipient.
    LocalCheque,
    /// A debit transaction made through the ACH system to a
    /// demand deposit account using the CTP payment format.
    AchDemandCorporateTradePaymentCtpDebit,
    /// A credit transaction made through the ACH system to a
    /// demand deposit account using the CTX payment format.
    AchDemandCorporateTradeExchangeCtxCredit,
    /// A debit transaction made through the ACH system to a
    /// demand account using the CTX payment format.
    AchDemandCorporateTradeExchangeCtxDebit,
    /// Payment by credit movement of funds from one account to
    /// another.
    CreditTransfer,
    /// Payment by debit movement of funds from one account to
    /// another.
    DebitTransfer,
    /// credit
    /// A credit transaction made through the ACH system to a
    /// demand deposit account using the CCD+ payment format.
    AchDemandCashConcentrationdisbursementPlusCcdDebit,
    /// A debit transaction made through the ACH system to a
    /// demand deposit account using the CCD+ payment format.
    AchDemandCashConcentrationdisbursementPlusCcdCredit,
    /// A consumer credit transaction made through the ACH
    /// system to a demand deposit or savings account.
    AchPrearrangedPaymentAndDepositPpd,
    /// A credit transaction made through the ACH system to a
    /// demand deposit or savings account.
    AchSavingsCashConcentrationdisbursementCcdCredit,
    /// A debit transaction made through the ACH system to a
    /// savings account using the CCD payment format.
    AchSavingsCashConcentrationdisbursementCcdDebit,
    /// A credit transaction made through the ACH system to a
    /// savings account using the CTP payment format.
    AchSavingsCorporateTradePaymentCtpCredit,
    /// A debit transaction made through the ACH system to a
    /// savings account using the CTP payment format.
    AchSavingsCorporateTradePaymentCtpDebit,
    /// A credit transaction made through the ACH system to a
    /// savings account using the CTX payment format.
    AchSavingsCorporateTradeExchangeCtxCredit,
    /// A debit transaction made through the ACH system to a
    /// savings account using the CTX payment format.
    AchSavingsCorporateTradeExchangeCtxDebit,
    /// credit
    /// A credit transaction made through the ACH system to a
    /// savings account using the CCD+ payment format.
    AchSavingsCashConcentrationdisbursementPlusCcdCredit,
    /// Payment by an arrangement for settling debts that is
    /// operated by the Post Office.
    PaymentToBankAccount,
    /// debit
    /// A debit transaction made through the ACH system to a
    /// savings account using the CCD+ payment format.
    AchSavingsCashConcentrationdisbursementPlusCcdDebit,
    /// Bill drawn by the creditor on the debtor and accepted by
    /// the debtor.
    AcceptedBillOfExchange,
    /// A referenced credit transfer initiated through home-
    /// banking.
    ReferencedHomebankingCreditTransfer,
    /// A debit transfer via interbank means.
    InterbankDebitTransfer,
    /// A debit transfer initiated through home-banking.
    HomebankingDebitTransfer,
    /// Payment by means of a card issued by a bank or other
    /// financial institution.
    BankCard,
    /// The amount is to be, or has been, directly debited to
    /// the customer's bank account.
    DirectDebit,
    /// A method for the transmission of funds through the
    /// postal system rather than through the banking system.
    PaymentByPostgiro,
    /// Banking Standards) - Option A
    /// A French standard procedure that allows a debtor to pay
    /// an amount due to a creditor. The creditor will forward
    /// it to its bank, which will collect the money on the bank
    /// account of the debtor.
    FrNorme6_97telereglementCfonbFrenchOrganisationFor,
    /// Payment order which requires guaranteed processing by
    /// the most appropriate means to ensure it occurs on the
    /// requested execution date, provided that it is issued to
    /// the ordered bank before the agreed cut-off time.
    UrgentCommercialPayment,
    /// Payment order or transfer which must be executed, by the
    /// most appropriate means, as urgently as possible and
    /// before urgent commercial payments.
    UrgentTreasuryPayment,
    /// Payment made by means of credit card.
    CreditCard,
    /// Payment made by means of debit card.
    DebitCard,
    /// Payment will be, or has been, made by bankgiro.
    Bankgiro,
    /// The payment means have been previously agreed between
    /// seller and buyer and thus are not stated again.
    StandingAgreement,
    /// Credit transfer inside the Single Euro Payment Area
    /// (SEPA) system.
    SepaCreditTransfer,
    /// Direct debit inside the Single Euro Payment Area (SEPA)
    /// system.
    SepaDirectDebit,
    /// Payment by an unconditional promise in writing made by
    /// one person to another, signed by the maker, engaging to
    /// pay on demand or at a fixed or determinable future time
    /// a sum certain in money, to order or to bearer.
    PromissoryNote,
    /// Payment by an unconditional promise in writing made by
    /// the debtor to another person, signed by the debtor,
    /// engaging to pay on demand or at a fixed or determinable
    /// future time a sum certain in money, to order or to
    /// bearer.
    PromissoryNoteSignedByTheDebtor,
    /// Payment by an unconditional promise in writing made by
    /// the debtor to another person, signed by the debtor and
    /// endorsed by a bank, engaging to pay on demand or at a
    /// fixed or determinable future time a sum certain in
    /// money, to order or to bearer.
    PromissoryNoteSignedByTheDebtorAndEndorsedByABank,
    /// third party
    /// Payment by an unconditional promise in writing made by
    /// the debtor to another person, signed by the debtor and
    /// endorsed by a third party, engaging to pay on demand or
    /// at a fixed or determinable future time a sum certain in
    /// money, to order or to bearer.
    PromissoryNoteSignedByTheDebtorAndEndorsedByAThirdParty,
    /// Payment by an unconditional promise in writing made by
    /// the bank to another person, signed by the bank, engaging
    /// to pay on demand or at a fixed or determinable future
    /// time a sum certain in money, to order or to bearer.
    PromissoryNoteSignedByABank,
    /// Payment by an unconditional promise in writing made by
    /// the bank to another person, signed by the bank and
    /// endorsed by another bank, engaging to pay on demand or
    /// at a fixed or determinable future time a sum certain in
    /// money, to order or to bearer.
    PromissoryNoteSignedByABankAndEndorsedByAnotherBank,
    /// Payment by an unconditional promise in writing made by a
    /// third party to another person, signed by the third
    /// party, engaging to pay on demand or at a fixed or
    /// determinable future time a sum certain in money, to
    /// order or to bearer.
    PromissoryNoteSignedByAThirdParty,
    /// Payment by an unconditional promise in writing made by a
    /// third party to another person, signed by the third party
    /// and endorsed by a bank, engaging to pay on demand or at
    /// a fixed or determinable future time a sum certain in
    /// money, to order or to bearer.
    PromissoryNoteSignedByAThirdPartyAndEndorsedByABank,
    /// Payment will be made or has been made by an online
    /// payment service.
    OnlinePaymentService,
    /// Bill drawn by the creditor on the debtor.
    BillDrawnByTheCreditorOnTheDebtor,
    /// Bill drawn by the creditor on a bank.
    BillDrawnByTheCreditorOnABank,
    /// Bill drawn by the creditor, endorsed by another bank.
    BillDrawnByTheCreditorEndorsedByAnotherBank,
    /// third party
    /// Bill drawn by the creditor on a bank and endorsed by a
    /// third party.
    BillDrawnByTheCreditorOnABankAndEndorsedByAThirdParty,
    /// Bill drawn by the creditor on a third party.
    BillDrawnByTheCreditorOnAThirdParty,
    /// endorsed by bank
    /// Bill drawn by creditor on third party, accepted and
    /// endorsed by bank.
    BillDrawnByCreditorOnThirdPartyAcceptedAnd,
    /// Issue a bankers draft not endorsable.
    NotTransferableBankersDraft,
    /// Issue a cheque not endorsable in payment of the funds.
    NotTransferableLocalCheque,
    /// Ordering customer tells the bank to use the payment
    /// system 'Reference giro'. Used in the Finnish national
    /// banking system.
    ReferenceGiro,
    /// Ordering customer tells the bank to use the bank service
    /// 'Urgent Giro' when transferring the payment. Used in
    /// Finnish national banking system.
    UrgentGiro,
    /// Ordering customer tells the ordering bank to use the
    /// bank service 'Free Format Giro' when transferring the
    /// payment. Used in Finnish national banking system.
    FreeFormatGiro,
    /// If the requested method for payment was or could not be
    /// used, this code indicates that.
    RequestedMethodForPaymentWasNotUsed,
    /// Amounts which two partners owe to each other to be
    /// compensated in order to avoid useless payments.
    ClearingBetweenPartners,
    /// A code assigned within a code list to be used on an
    /// interim basis and as defined among trading partners
    /// until a precise code can be assigned to the code list.
    MutuallyDefined(&'invoice str),
}

impl<'invoice> PaymentMeansCode<'invoice> {
    pub fn as_str(&self) -> &str {
        match self {
            PaymentMeansCode::InstrumentNotDefined => "1",
            PaymentMeansCode::AutomatedClearingHouseCredit => "2",
            PaymentMeansCode::AutomatedClearingHouseDebit => "3",
            PaymentMeansCode::AchDemandDebitReversal => "4",
            PaymentMeansCode::AchDemandCreditReversal => "5",
            PaymentMeansCode::AchDemandCredit => "6",
            PaymentMeansCode::AchDemandDebit => "7",
            PaymentMeansCode::Hold => "8",
            PaymentMeansCode::NationalOrRegionalClearing => "9",
            PaymentMeansCode::InCash => "10",
            PaymentMeansCode::AchSavingsCreditReversal => "11",
            PaymentMeansCode::AchSavingsDebitReversal => "12",
            PaymentMeansCode::AchSavingsCredit => "13",
            PaymentMeansCode::AchSavingsDebit => "14",
            PaymentMeansCode::BookentryCredit => "15",
            PaymentMeansCode::BookentryDebit => "16",
            PaymentMeansCode::AchDemandCashConcentrationdisbursementCcdCredit => "17",
            PaymentMeansCode::AchDemandCashConcentrationdisbursementCcdDebit => "18",
            PaymentMeansCode::AchDemandCorporateTradePaymentCtpCredit => "19",
            PaymentMeansCode::Cheque => "20",
            PaymentMeansCode::BankersDraft => "21",
            PaymentMeansCode::CertifiedBankersDraft => "22",
            PaymentMeansCode::BankChequeIssuedByABankingOrSimilarEstablishment => "23",
            PaymentMeansCode::BillOfExchangeAwaitingAcceptance => "24",
            PaymentMeansCode::CertifiedCheque => "25",
            PaymentMeansCode::LocalCheque => "26",
            PaymentMeansCode::AchDemandCorporateTradePaymentCtpDebit => "27",
            PaymentMeansCode::AchDemandCorporateTradeExchangeCtxCredit => "28",
            PaymentMeansCode::AchDemandCorporateTradeExchangeCtxDebit => "29",
            PaymentMeansCode::CreditTransfer => "30",
            PaymentMeansCode::DebitTransfer => "31",
            PaymentMeansCode::AchDemandCashConcentrationdisbursementPlusCcdDebit => "32",
            PaymentMeansCode::AchDemandCashConcentrationdisbursementPlusCcdCredit => "33",
            PaymentMeansCode::AchPrearrangedPaymentAndDepositPpd => "34",
            PaymentMeansCode::AchSavingsCashConcentrationdisbursementCcdCredit => "35",
            PaymentMeansCode::AchSavingsCashConcentrationdisbursementCcdDebit => "36",
            PaymentMeansCode::AchSavingsCorporateTradePaymentCtpCredit => "37",
            PaymentMeansCode::AchSavingsCorporateTradePaymentCtpDebit => "38",
            PaymentMeansCode::AchSavingsCorporateTradeExchangeCtxCredit => "39",
            PaymentMeansCode::AchSavingsCorporateTradeExchangeCtxDebit => "40",
            PaymentMeansCode::AchSavingsCashConcentrationdisbursementPlusCcdCredit => "41",
            PaymentMeansCode::PaymentToBankAccount => "42",
            PaymentMeansCode::AchSavingsCashConcentrationdisbursementPlusCcdDebit => "43",
            PaymentMeansCode::AcceptedBillOfExchange => "44",
            PaymentMeansCode::ReferencedHomebankingCreditTransfer => "45",
            PaymentMeansCode::InterbankDebitTransfer => "46",
            PaymentMeansCode::HomebankingDebitTransfer => "47",
            PaymentMeansCode::BankCard => "48",
            PaymentMeansCode::DirectDebit => "49",
            PaymentMeansCode::PaymentByPostgiro => "50",
            PaymentMeansCode::FrNorme6_97telereglementCfonbFrenchOrganisationFor => "51",
            PaymentMeansCode::UrgentCommercialPayment => "52",
            PaymentMeansCode::UrgentTreasuryPayment => "53",
            PaymentMeansCode::CreditCard => "54",
            PaymentMeansCode::DebitCard => "55",
            PaymentMeansCode::Bankgiro => "56",
            PaymentMeansCode::StandingAgreement => "57",
            PaymentMeansCode::SepaCreditTransfer => "58",
            PaymentMeansCode::SepaDirectDebit => "59",
            PaymentMeansCode::PromissoryNote => "60",
            PaymentMeansCode::PromissoryNoteSignedByTheDebtor => "61",
            PaymentMeansCode::PromissoryNoteSignedByTheDebtorAndEndorsedByABank => "62",
            PaymentMeansCode::PromissoryNoteSignedByTheDebtorAndEndorsedByAThirdParty => "63",
            PaymentMeansCode::PromissoryNoteSignedByABank => "64",
            PaymentMeansCode::PromissoryNoteSignedByABankAndEndorsedByAnotherBank => "65",
            PaymentMeansCode::PromissoryNoteSignedByAThirdParty => "66",
            PaymentMeansCode::PromissoryNoteSignedByAThirdPartyAndEndorsedByABank => "67",
            PaymentMeansCode::OnlinePaymentService => "68",
            PaymentMeansCode::BillDrawnByTheCreditorOnTheDebtor => "70",
            PaymentMeansCode::BillDrawnByTheCreditorOnABank => "74",
            PaymentMeansCode::BillDrawnByTheCreditorEndorsedByAnotherBank => "75",
            PaymentMeansCode::BillDrawnByTheCreditorOnABankAndEndorsedByAThirdParty => "76",
            PaymentMeansCode::BillDrawnByTheCreditorOnAThirdParty => "77",
            PaymentMeansCode::BillDrawnByCreditorOnThirdPartyAcceptedAnd => "78",
            PaymentMeansCode::NotTransferableBankersDraft => "91",
            PaymentMeansCode::NotTransferableLocalCheque => "92",
            PaymentMeansCode::ReferenceGiro => "93",
            PaymentMeansCode::UrgentGiro => "94",
            PaymentMeansCode::FreeFormatGiro => "95",
            PaymentMeansCode::RequestedMethodForPaymentWasNotUsed => "96",
            PaymentMeansCode::ClearingBetweenPartners => "97",
            PaymentMeansCode::MutuallyDefined(v) => v,
        }
    }
}

impl<'invoice> Serialize for PaymentMeansCode<'invoice> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
