use printpdf::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct BankData {
    pub beneficiary_account_name: String,
    pub bank_name: String,
    pub bank_address: String,
    pub account_type: String,
    pub account_number: String,
    pub wire_routing: String,
    pub swift_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressData {
    pub beneficiary_address_line1: String,
    pub beneficiary_address_line2: Option<String>,
    pub beneficiary_address_state: String,
    pub beneficiary_address_city: String,
    pub beneficiary_address_zip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientData {
    pub name: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceData {
    pub client: ClientData,
    pub bank_data: BankData,
    pub address_data: AddressData,
    pub service_description: String,
    pub hourly_rate: f64,
    pub hours_worked: f64,
    pub invoice_number: String,
    pub notes: Option<String>,
    pub invoice_date: String,
}

impl InvoiceData {
    pub fn calculate_total(&self) -> f64 {
        self.hourly_rate * self.hours_worked
    }
}

pub fn generate_invoice_pdf(invoice_data: &InvoiceData, output_path: &Path) -> Result<(), String> {
    // Create PDF document
    let (doc, page1, layer1) = PdfDocument::new("Invoice", Mm(210.0), Mm(297.0), "Layer 1");

    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Failed to load font: {}", e))?;

    let font_bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Failed to load bold font: {}", e))?;

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Constants for layout
    let margin = Mm(20.0);
    let page_width = Mm(210.0);
    let page_height = Mm(297.0);
    let mut current_y = page_height - margin;

    // Title
    current_layer.use_text("INVOICE", 24.0, Mm(margin.0), Mm(current_y.0), &font_bold);
    current_y = current_y - Mm(10.0);

    // Invoice Number and Date
    current_layer.use_text(
        &format!("Invoice #: {}", invoice_data.invoice_number),
        12.0,
        Mm(margin.0),
        Mm(current_y.0),
        &font,
    );
    current_y = current_y - Mm(6.0);

    current_layer.use_text(
        &format!("Date: {}", invoice_data.invoice_date),
        12.0,
        Mm(margin.0),
        Mm(current_y.0),
        &font,
    );
    current_y = current_y - Mm(15.0);

    // Beneficiary (From) Section
    current_layer.use_text("FROM:", 12.0, Mm(margin.0), Mm(current_y.0), &font_bold);
    current_y = current_y - Mm(6.0);

    current_layer.use_text(
        &invoice_data.bank_data.beneficiary_account_name,
        11.0,
        Mm(margin.0),
        Mm(current_y.0),
        &font,
    );
    current_y = current_y - Mm(5.0);

    current_layer.use_text(
        &invoice_data.address_data.beneficiary_address_line1,
        11.0,
        Mm(margin.0),
        Mm(current_y.0),
        &font,
    );
    current_y = current_y - Mm(5.0);

    let address_line = format!(
        "{}, {} - {}",
        invoice_data.address_data.beneficiary_address_city,
        invoice_data.address_data.beneficiary_address_state,
        invoice_data.address_data.beneficiary_address_zip
    );

    // Display second address line if present
    if let Some(line2) = &invoice_data.address_data.beneficiary_address_line2 {
        if !line2.is_empty() {
            current_layer.use_text(line2, 11.0, Mm(margin.0), Mm(current_y.0), &font);
            current_y = current_y - Mm(5.0);
        }
    }

    current_layer.use_text(&address_line, 11.0, Mm(margin.0), Mm(current_y.0), &font);
    current_y = current_y - Mm(15.0);

    // Client (Bill To) Section
    current_layer.use_text("BILL TO:", 12.0, Mm(margin.0), Mm(current_y.0), &font_bold);
    current_y = current_y - Mm(6.0);

    current_layer.use_text(
        &invoice_data.client.name,
        11.0,
        Mm(margin.0),
        Mm(current_y.0),
        &font,
    );
    current_y = current_y - Mm(5.0);

    current_layer.use_text(
        &invoice_data.client.address_line1,
        11.0,
        Mm(margin.0),
        Mm(current_y.0),
        &font,
    );
    current_y = current_y - Mm(5.0);

    // Display second address line if present
    if let Some(line2) = &invoice_data.client.address_line2 {
        if !line2.is_empty() {
            current_layer.use_text(line2, 11.0, Mm(margin.0), Mm(current_y.0), &font);
            current_y = current_y - Mm(5.0);
        }
    }

    if let Some(email) = &invoice_data.client.email {
        if !email.is_empty() {
            current_layer.use_text(email, 11.0, Mm(margin.0), Mm(current_y.0), &font);
            current_y = current_y - Mm(5.0);
        }
    }
    current_y = current_y - Mm(10.0);

    // Invoice Details Table Header
    current_layer.use_text(
        "DESCRIPTION",
        11.0,
        Mm(margin.0),
        Mm(current_y.0),
        &font_bold,
    );
    current_layer.use_text("QUANTITY", 11.0, Mm(100.0), Mm(current_y.0), &font_bold);
    current_layer.use_text("RATE", 11.0, Mm(135.0), Mm(current_y.0), &font_bold);
    current_layer.use_text("AMOUNT", 11.0, Mm(170.0), Mm(current_y.0), &font_bold);
    current_y = current_y - Mm(2.0);

    // Draw line under header
    let line = Line {
        points: vec![
            (Point::new(Mm(margin.0), Mm(current_y.0)), false),
            (
                Point::new(Mm((page_width - margin).0), Mm(current_y.0)),
                false,
            ),
        ],
        is_closed: false,
    };
    current_layer.add_line(line);
    current_y = current_y - Mm(6.0);

    // Invoice Details Row
    current_layer.use_text(
        &invoice_data.service_description,
        11.0,
        Mm(margin.0),
        Mm(current_y.0),
        &font,
    );
    current_layer.use_text(
        &format!("{:.2} hrs", invoice_data.hours_worked),
        11.0,
        Mm(100.0),
        Mm(current_y.0),
        &font,
    );
    current_layer.use_text(
        &format!("${:.2}/hr", invoice_data.hourly_rate),
        11.0,
        Mm(135.0),
        Mm(current_y.0),
        &font,
    );
    let total = invoice_data.calculate_total();
    current_layer.use_text(
        &format!("${:.2}", total),
        11.0,
        Mm(170.0),
        Mm(current_y.0),
        &font,
    );
    current_y = current_y - Mm(8.0);

    // Draw line before total
    let line = Line {
        points: vec![
            (Point::new(Mm(135.0), Mm(current_y.0)), false),
            (
                Point::new(Mm((page_width - margin).0), Mm(current_y.0)),
                false,
            ),
        ],
        is_closed: false,
    };
    current_layer.add_line(line);
    current_y = current_y - Mm(6.0);

    // Total
    current_layer.use_text("TOTAL:", 12.0, Mm(135.0), Mm(current_y.0), &font_bold);
    current_layer.use_text(
        &format!("${:.2}", total),
        12.0,
        Mm(170.0),
        Mm(current_y.0),
        &font_bold,
    );
    current_y = current_y - Mm(15.0);

    // Bank Information Section
    current_layer.use_text(
        "PAYMENT INFORMATION",
        12.0,
        Mm(margin.0),
        Mm(current_y.0),
        &font_bold,
    );
    current_y = current_y - Mm(6.0);

    let bank_info = vec![
        format!(
            "Beneficiary/Account Name: {}",
            invoice_data.bank_data.beneficiary_account_name
        ),
        format!("Bank Name: {}", invoice_data.bank_data.bank_name),
        format!("Bank Address: {}", invoice_data.bank_data.bank_address),
        format!("Account Type: {}", invoice_data.bank_data.account_type),
        format!("Account Number: {}", invoice_data.bank_data.account_number),
        format!("Wire Routing: {}", invoice_data.bank_data.wire_routing),
        format!("SWIFT Code: {}", invoice_data.bank_data.swift_code),
    ];

    for info_line in bank_info {
        current_layer.use_text(&info_line, 10.0, Mm(margin.0), Mm(current_y.0), &font);
        current_y = current_y - Mm(5.0);
    }

    // Notes section (if provided)
    if let Some(notes) = &invoice_data.notes {
        if !notes.is_empty() {
            current_y = current_y - Mm(5.0);
            current_layer.use_text("NOTES:", 11.0, Mm(margin.0), Mm(current_y.0), &font_bold);
            current_y = current_y - Mm(6.0);
            current_layer.use_text(notes, 10.0, Mm(margin.0), Mm(current_y.0), &font);
        }
    }

    // Footer
    let footer_y = Mm(25.0);
    current_layer.use_text(
        "Thank you for your business!",
        10.0,
        Mm(margin.0),
        Mm(footer_y.0),
        &font,
    );

    // Save PDF
    let file =
        File::create(output_path).map_err(|e| format!("Failed to create PDF file: {}", e))?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;

    Ok(())
}
