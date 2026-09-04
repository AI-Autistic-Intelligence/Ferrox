use rust_xlsxwriter::Workbook;
use serde::Serialize;
use yalc_errors::AppError;

pub struct ReportGenerator;

impl ReportGenerator {
    /// Generates a CSV from a list of serializable items and returns the bytes
    pub fn to_csv<T: Serialize>(items: &[T]) -> Result<Vec<u8>, AppError> {
        let mut wtr = csv::Writer::from_writer(vec![]);
        for item in items {
            wtr.serialize(item).map_err(|e| AppError::InternalServerError(Box::new(e)))?;
        }
        let data = wtr.into_inner().map_err(|e| AppError::InternalServerError(Box::new(e)))?;
        Ok(data)
    }

    /// Generates a basic Excel (XLSX) file from raw string data for demonstration
    /// (In a real app, you would pass columns and rows generically)
    pub fn to_excel(headers: &[&str], rows: &[Vec<String>]) -> Result<Vec<u8>, AppError> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        // Write headers
        for (col, header) in headers.iter().enumerate() {
            worksheet.write_string(0, col as u16, *header)
                .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
        }

        // Write rows
        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                worksheet.write_string((row_idx + 1) as u32, col_idx as u16, cell)
                    .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
            }
        }

        let buffer = workbook.save_to_buffer()
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
            
        Ok(buffer)
    }
}
