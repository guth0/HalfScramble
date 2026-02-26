use crate::cube::Move;

use csv::Writer;
use serde::Serialize;
use std::fs::File;
use std::error::Error;

pub fn write_scramble_records(
    path: &str,
    records: &[ScrambleRecord],
) -> Result<(), Box<dyn Error>>
{
    use csv::Writer;
    use std::fs::File;

    let file = File::create("scramble_data.csv")?;
    let mut wtr = Writer::from_writer(file);

    for record in records {
        // Convert to serlialized struct
        let csv_record = ScrambleRecordCsv {
            scramble: record.scramble.to_string(),
            n: record.n,
            min_length: record.min_length,
            count: record.count,
            nodes_expanded: record.nodes_expanded,
        };

        // Save record
        wtr.serialize(csv_record)?;
    }

    // make sure everything is done
    wtr.flush()?;

    return Ok(());
}

#[derive(Serialize)]
struct ScrambleRecordCsv{
    pub scramble: String, 
    pub n: u8,            
    pub min_length: u8,   
    pub count: u32,       
    pub nodes_expanded: u64,
}

#[derive(Debug, Clone)]
pub struct ScrambleRecord {
    pub scramble: Vec<Move>, // move list (canonical)
    pub n: u8,            // scramble length
    pub min_length: u8,   // minimum alternate solution length
    pub count: u32,       // number of min alternates
    pub nodes_expanded: u64,
}

// pub struct SimpleScrambleRecord {
//     pub min_length: u8,
//     pub count: u32,
//     pub nodes_expanded: u64,
// }
//
// pub fn impliment_simple_record(record: &mut ScrambleRecord, simple_record: &SimpleScrambleRecord) {
//     record.min_length = simple_record.min_length;
//     record.count = simple_record.count;
//     record.nodes_expanded = simple_record.nodes_expanded;
// }
