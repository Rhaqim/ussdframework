use csv::Writer;
use rusqlite::{Connection, Result};

struct Data {
    // Define the structure of your data here
    // Example:
    id: i32,
    name: String,
    age: i32,
}

fn fetch_data_from_database() -> Result<Vec<Data>> {
    // Connect to the database
    let conn = Connection::open("path/to/your/database.db")?;

    // Execute a query to fetch the data
    let mut stmt = conn.prepare("SELECT id, name, age FROM your_table")?;
    let rows = stmt.query_map([], |row| {
        Ok(Data {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    // Collect the fetched data into a vector
    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }

    Ok(data)
}

fn convert_data_to_csv(data: &[Data]) -> Result<()> {
    // Create a CSV writer
    let mut writer = Writer::from_path("path/to/your/output.csv")?;

    // Write the CSV header
    writer.write_record(&["ID", "Name", "Age"])?;

    // Write each data row to the CSV file
    for item in data {
        writer.write_record(&[item.id.to_string(), item.name.clone(), item.age.to_string()])?;
    }

    // Flush and close the CSV writer
    writer.flush()?;

    Ok(())
}

fn main() -> Result<()> {
    // Fetch data from the database
    let data = fetch_data_from_database()?;

    // Convert data to CSV and generate the file
    convert_data_to_csv(&data)?;

    Ok(())
}