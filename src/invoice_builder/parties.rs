use super::InvoiceBuilder;
use crate::components::enums::country_code::CountryCode;

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
    /// Buyer reference
    ///
    /// An identifier assigned by the Buyer used for internal routing purposes.
    ///
    /// The identifier is defined by the Buyer (e.g. contact ID,
    /// department, office id, project code), but provided by
    /// the Seller in the Invoice.
    ///
    /// CHORUS PRO: for the public sector, it is the "Service Exécutant". It is mandatory for some buyers. It must
    /// belong to the Chorus Pro repository. It is limited to 100 characters.
    ///
    /// - /rsm:CrossIndustryInvoice
    /// - /rsm:SupplyChainTradeTransaction
    /// - /ram:ApplicableHeaderTradeAgreement
    /// - /ram:BuyerReference
    pub fn set_buyer_reference<T: Into<&'invoice_builder str>>(
        &mut self,
        buyer_reference: T,
    ) -> &mut Self {
        self.buyer_reference = Some(buyer_reference.into());
        self
    }

    pub fn set_sellers_name<T: Into<&'invoice_builder str>>(
        &mut self,
        sellers_name: T,
    ) -> &mut Self {
        self.sellers_name = Some(sellers_name.into());
        self
    }

    /// An identifier issued by an official registrar that identifies the Seller as a legal entity or person.
    ///
    /// BT-30
    ///
    /// ram:SpecifiedLegalOrganization -> ram:ID
    pub fn set_sellers_specified_legal_organization<T: Into<&'invoice_builder str>>(
        &mut self,
        sellers_specified_legal_organization: T,
    ) -> &mut Self {
        self.sellers_specified_legal_organization =
            Some(sellers_specified_legal_organization.into());
        self
    }

    /// Postal code, zip code or similar
    ///
    /// BT-38
    pub fn set_sellers_postal_trade_address_postcode_code<T: Into<&'invoice_builder str>>(
        &mut self,
        postcode_code: T,
    ) -> &mut Self {
        self.sellers_postal_trade_address.postcode_code = Some(postcode_code.into());
        self
    }

    /// Usually streetname and house number or post-office box number
    ///
    /// BT-35
    pub fn set_sellers_postal_trade_address_line_one<T: Into<&'invoice_builder str>>(
        &mut self,
        line: T,
    ) -> &mut Self {
        self.sellers_postal_trade_address.line_one = Some(line.into());
        self
    }

    /// Additional row to specify details or additions to line one
    ///
    /// BT-36
    pub fn set_sellers_postal_trade_address_line_two<T: Into<&'invoice_builder str>>(
        &mut self,
        line: T,
    ) -> &mut Self {
        self.sellers_postal_trade_address.line_two = Some(line.into());
        self
    }

    /// Additional row to specify details or additions to line one
    ///
    /// BT-162
    pub fn set_sellers_postal_trade_address_line_three<T: Into<&'invoice_builder str>>(
        &mut self,
        line: T,
    ) -> &mut Self {
        self.sellers_postal_trade_address.line_three = Some(line.into());
        self
    }

    /// Name of the city or community where the address is located
    ///
    /// BT-37
    pub fn set_sellers_postal_trade_address_city_name<T: Into<&'invoice_builder str>>(
        &mut self,
        city_name: T,
    ) -> &mut Self {
        self.sellers_postal_trade_address.city_name = Some(city_name.into());
        self
    }

    /// If there is no tax agent defined, the country where the sales tax is applied
    ///
    /// BT-40
    pub fn set_sellers_postal_trade_address_country_code(
        &mut self,
        country_code: CountryCode,
    ) -> &mut Self {
        self.sellers_postal_trade_address.country_id = country_code;
        self
    }

    pub fn set_sellers_specified_tax_registration<T: Into<&'invoice_builder str>>(
        &mut self,
        sellers_specified_tax_registration: T,
    ) -> &mut Self {
        self.sellers_specified_tax_registration = Some(sellers_specified_tax_registration.into());
        self
    }

    pub fn set_buyers_name<T: Into<&'invoice_builder str>>(&mut self, buyers_name: T) -> &mut Self {
        self.buyers_name = Some(buyers_name.into());
        self
    }

    /// An identifier issued by an official registrar that identifies the Buyer as a legal entity or person.
    ///
    /// BT-47
    ///
    /// ram:SpecifiedLegalOrganization -> ram:ID
    pub fn set_buyers_specified_legal_organization<T: Into<&'invoice_builder str>>(
        &mut self,
        buyers_specified_legal_organization: T,
    ) -> &mut Self {
        self.buyers_specified_legal_organization = Some(buyers_specified_legal_organization.into());
        self
    }

    /// Postal code, zip code or similar
    ///
    /// BT-53
    pub fn set_buyers_postal_trade_address_postcode_code<T: Into<&'invoice_builder str>>(
        &mut self,
        postcode_code: T,
    ) -> &mut Self {
        self.buyers_postal_trade_address.postcode_code = Some(postcode_code.into());
        self
    }

    /// Usually streetname and house number or post-office box number
    ///
    /// BT-50
    pub fn set_buyers_postal_trade_address_line_one<T: Into<&'invoice_builder str>>(
        &mut self,
        line: T,
    ) -> &mut Self {
        self.buyers_postal_trade_address.line_one = Some(line.into());
        self
    }

    /// Additional row to specify details or additions to line one
    ///
    /// BT-51
    pub fn set_buyers_postal_trade_address_line_two<T: Into<&'invoice_builder str>>(
        &mut self,
        line: T,
    ) -> &mut Self {
        self.buyers_postal_trade_address.line_two = Some(line.into());
        self
    }

    /// Additional row to specify details or additions to line one
    ///
    /// BT-163
    pub fn set_buyers_postal_trade_address_line_three<T: Into<&'invoice_builder str>>(
        &mut self,
        line: T,
    ) -> &mut Self {
        self.buyers_postal_trade_address.line_three = Some(line.into());
        self
    }

    /// Name of the city or community where the address is located
    ///
    /// BT-52
    pub fn set_buyers_postal_trade_address_city_name<T: Into<&'invoice_builder str>>(
        &mut self,
        city_name: T,
    ) -> &mut Self {
        self.buyers_postal_trade_address.city_name = Some(city_name.into());
        self
    }

    /// An identifier issued by an official registrar that identifies the Buyer as a legal entity or person.
    ///
    /// BT-55
    ///
    /// ram:SpecifiedLegalOrganization -> ram:ID
    pub fn set_buyers_postal_trade_address_country_code(
        &mut self,
        country_code: CountryCode,
    ) -> &mut Self {
        self.buyers_postal_trade_address.country_id = country_code;
        self
    }

    /// ## Buyer Order Referenced Document
    /// An identifier of a referenced purchase order, issued by the Buyer.
    ///
    /// /ram:BuyerOrderReferencedDocument
    pub fn set_buyers_order_specified_document<T: Into<&'invoice_builder str>>(
        &mut self,
        buyers_order_specified_document: T,
    ) -> &mut Self {
        self.buyers_order_specified_document = Some(buyers_order_specified_document.into());
        self
    }
}
