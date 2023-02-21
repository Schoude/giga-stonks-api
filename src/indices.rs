pub enum Index {
    DowJones,
    NASDAQ,
}

trait AsStr {
    fn as_str(&self) -> &str;
}

impl ToString for Index {
    fn to_string(&self) -> String {
        match self {
            Self::DowJones => "djia".to_string(),
            Self::NASDAQ => "nasdq".to_string(),
        }
    }
}

impl AsStr for Index {
    fn as_str(&self) -> &str {
        match self {
            Self::DowJones => "djia",
            Self::NASDAQ => "nasdq",
        }
    }
}

// 30 entries
pub const DOW_JONES: &[(&str, &str)] = &[
    ("JPM", "JPMorgan Chase & Co"),
    ("WBA", "Walgreens Boots Alliance Inc"),
    ("PG", "Procter & Gamble Co"),
    ("GS", "Goldman Sachs Group Inc"),
    ("AAPL", "Apple Inc"),
    ("CAT", "Caterpillar Inc"),
    ("BA", "Boeing Co"),
    ("HON", "Honeywell International Inc"),
    ("MRK", "Merck & Co Inc"),
    ("MMM", "3M Co"),
    ("IBM", "International Business Machines Corp"),
    ("DIS", "Walt Disney Co"),
    ("HD", "Home Depot Inc"),
    ("AMGN", "Amgen Inc"),
    ("V", "Visa Inc"),
    ("INTC", "Intel Corp"),
    ("DOW", "Dow Inc"),
    ("MSFT", "Microsoft Corp"),
    ("TRV", "Travelers Companies Inc"),
    ("JNJ", "Johnson & Johnson"),
    ("KO", "Coca-Cola Co"),
    ("NKE", "Nike Inc"),
    ("MCD", "McDonald's Corp"),
    ("WMT", "Walmart Inc"),
    ("CRM", "Salesforce Inc"),
    ("AXP", "American Express Co"),
    ("UNH", "UnitedHealth Group Inc"),
    ("CVX", "Chevron Corp"),
    ("VZ", "Verizon Communications Inc"),
    ("CSCO", "Cisco Systems Inc"),
];

// 60 entries
pub const NASDAQ: &[(&str, &str)] = &[
    ("AMZN", "Amazon.com, Inc."),
    ("FB", "Facebook, Inc."),
    ("TSLA", "Tesla, Inc."),
    ("NVDA", "NVIDIA Corporation"),
    ("PYPL", "PayPal Holdings, Inc."),
    ("ASML", "ASML Holding N.V."),
    ("ZM", "Zoom Video Communications, Inc."),
    ("MRNA", "Moderna, Inc."),
    ("VRTX", "Vertex Pharmaceuticals Incorporated"),
    ("REGN", "Regeneron Pharmaceuticals, Inc."),
    ("ILMN", "Illumina, Inc."),
    ("JD", "JD.com, Inc."),
    ("BIDU", "Baidu, Inc."),
    ("MELI", "MercadoLibre, Inc."),
    ("DOCU", "DocuSign, Inc."),
    ("PDD", "Pinduoduo Inc."),
    ("FSLY", "Fastly, Inc."),
    ("TEAM", "Atlassian Corporation Plc"),
    ("ALGN", "Align Technology, Inc."),
    ("MRVL", "Marvell Technology Group Ltd."),
    ("ZI", "ZoomInfo Technologies Inc."),
    ("CDW", "CDW Corporation"),
    ("CCI", "Crown Castle International Corp."),
    ("MSCI", "MSCI Inc."),
    ("NTES", "NetEase, Inc."),
    ("IDXX", "IDEXX Laboratories, Inc."),
    ("SGEN", "Seagen Inc."),
    ("OKTA", "Okta, Inc."),
    ("MTCH", "Match Group, Inc."),
    ("VRSK", "Verisk Analytics, Inc."),
    ("LULU", "Lululemon Athletica Inc."),
    ("MNST", "Monster Beverage Corporation"),
    ("KLAC", "KLA Corporation"),
    ("ANSS", "ANSYS, Inc."),
    ("PAYC", "Paycom Software, Inc."),
    ("RMD", "ResMed Inc."),
    ("CDNS", "Cadence Design Systems, Inc."),
    ("CHKP", "Check Point Software Technologies Ltd."),
    ("SWKS", "Skyworks Solutions, Inc."),
    ("CTXS", "Citrix Systems, Inc."),
    ("INCY", "Incyte Corporation"),
    ("SPLK", "Splunk Inc."),
    ("VRTX", "Vertex Pharmaceuticals Incorporated"),
    ("WBA", "Walgreens Boots Alliance, Inc."),
    ("SGEN", "Seagen Inc."),
    ("CERN", "Cerner Corporation"),
    ("CPRT", "Copart, Inc."),
    ("WDC", "Western Digital Corporation"),
    ("NTAP", "NetApp, Inc."),
    ("MXIM", "Maxim Integrated Products, Inc."),
    ("TTWO", "Take-Two Interactive Software, Inc."),
    ("PEP", "PepsiCo, Inc."),
    ("INTU", "Intuit Inc."),
    ("TXN", "Texas Instruments Incorporated"),
    ("BIIB", "Biogen Inc."),
    ("ADSK", "Autodesk, Inc."),
    ("MU", "Micron Technology, Inc."),
    ("CSCO", "Cisco Systems, Inc."),
    ("INTC", "Intel Corporation"),
    ("CMCSA", "Comcast Corporation"),
];
