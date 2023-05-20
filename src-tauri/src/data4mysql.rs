use std::{
    error::Error, 
    fs::File, 
    io::BufReader,
};

use tauri::Manager;
use csv::WriterBuilder;
use sqlx::{MySqlPool, query, Row, Column};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;


#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    url: String,
    save_path: String,
    company_name: Vec<String>,
}

pub fn read_yaml(file_path: String) -> Result<Config, Box<dyn Error>> {
    let yaml_file = File::open(file_path)?;
    let yaml_reader = BufReader::new(yaml_file);
    let yaml: Config = serde_yaml::from_reader(yaml_reader)?;
    Ok(yaml)
}

pub async fn query_data(file_path: String) -> Result<(), Box<dyn Error>> {
    let yaml = read_yaml(file_path)?;
    let mut vec_code: Vec<String> = Vec::new();
    let pool: sqlx::Pool<sqlx::MySql> = MySqlPool::connect(&yaml.url).await?;
    for name in &yaml.company_name 
    {
        let sql_query_code = format!(
            "SELECT DbName FROM deloitte.b_projectlist WHERE ProjectName = '{}'",
            name
        );
        let unique_code = query(&sql_query_code).fetch_one(&pool).await?;
        let code: &str = unique_code.get(0);
        vec_code.push(code.to_string())
    }

    let mut company_count = 1;

    for (idx, code) in vec_code.iter().enumerate() 
    {
        let sql_query_len = format!(
            "SELECT COUNT(*) AS length FROM {}.凭证表",
            code
        );
        let len_gl = query(&sql_query_len).fetch_all(&pool).await?;
        let mut len_gl_vec = Vec::new();
        for row in len_gl {
            let get_len_gl: i32 = row.get("length");
            len_gl_vec.push(get_len_gl)
        }
        let mut start = 0;
        let stop = len_gl_vec[0];
        let step = 200_0000;
        let mut file_count = 1;
        let mut split_filename = yaml.company_name[idx].split("_");
        let filename = split_filename.nth(2).unwrap_or(&yaml.company_name[idx]);
        println!("<{}> {} - rows => {:?}", company_count, filename, len_gl_vec[0]);

        // query gl
        for _ in (start..=stop).step_by(step) 
        {
            let sql_query_gl = format!(
                "SELECT * FROM {}.凭证表 LIMIT {}, {}",
                code, start, step
            );
            let data_gl = query(&sql_query_gl).fetch_all(&pool).await?;

            let one_gl = query(&sql_query_gl).fetch_one(&pool).await?;
            let col_num = one_gl.columns().len();
            let mut vec_col_name = Vec::new();
            let mut vec_col_type = Vec::new();
            for num in 0..col_num {
                vec_col_name.push(one_gl.column(num).name());
                vec_col_type.push(one_gl.column(num).type_info().to_string())
            }

            let step_i32: i32 = step as i32;
            let output_path_single = format!("{}/{}_GL.csv", yaml.save_path, filename);
            let output_path_multi = format!("{}/{}_GL_{}.csv", yaml.save_path, filename, file_count);
            let output_path = if step_i32 > stop { output_path_single } else { output_path_multi };
            let mut csv_writer_gl = WriterBuilder::new()
                .delimiter(b'|')
                .from_path(output_path)?;

            csv_writer_gl.write_record(&vec_col_name)?;

            for data in data_gl 
            {
                let mut vec_wtr_str = Vec::new();
                for num in 0..col_num {
                    if vec_col_type[num] == "DECIMAL" {
                        let num: Decimal = data.get(num);
                        vec_wtr_str.push(num.to_string())
                    } else if vec_col_type[num] == "SMALLINT" || vec_col_type[num] == "TINYINT" {
                        let num: i32 = data.get(num);
                        vec_wtr_str.push(num.to_string())
                    } else {
                        let num: &str = data.get(num);
                        vec_wtr_str.push(num.to_string())
                    }
                }
                csv_writer_gl.serialize(vec_wtr_str)?;
            }
            csv_writer_gl.flush()?;
            let out_single = format!("{}/{}_GL.csv", yaml.save_path, filename);
            let output_multi = format!("{}/{}_GL_{}.csv", yaml.save_path, filename, file_count);
            let out_gl = if step_i32 > stop { out_single } else { output_multi };
            println!("save GL => {}", out_gl);
            start += step_i32;
            file_count += 1;
        }

        // query tb
        let sql_query_tb = format!(
            "SELECT * FROM {}.科目余额表", 
            code
        );
        let data_tb = query(&sql_query_tb).fetch_all(&pool).await?;
        let one_tb = query(&sql_query_tb).fetch_one(&pool).await?;
            let col_num = one_tb.columns().len();
            let mut vec_col_name = Vec::new();
            let mut vec_col_type = Vec::new();
            for num in 0..col_num {
                vec_col_name.push(one_tb.column(num).name());
                vec_col_type.push(one_tb.column(num).type_info().to_string())
            }
        let output_path = format!("{}/{}_TB.csv", yaml.save_path, filename);
        let mut csv_writer_tb = WriterBuilder::new()
            .delimiter(b'|')
            .from_path(output_path)?;
        csv_writer_tb.write_record(&vec_col_name)?;

        for data in data_tb 
        {
            let mut vec_wtr_str = Vec::new();
            for num in 0..col_num {
                if vec_col_type[num] == "DECIMAL" {
                    let num: Decimal = data.get(num);
                    vec_wtr_str.push(num.to_string())
                } else if vec_col_type[num] == "SMALLINT" || vec_col_type[num] == "TINYINT"{
                    let num: i32 = data.get(num);
                    vec_wtr_str.push(num.to_string())
                } else {
                    let num: &str = data.get(num);
                    vec_wtr_str.push(num.to_string())
                }
            }
            csv_writer_tb.serialize(vec_wtr_str)?;
        }
        csv_writer_tb.flush()?;
        let out_tb = format!("{}/{}_TB.csv", yaml.save_path, filename);
        println!("save TB => {}", out_tb);
        company_count += 1;
    }
    Ok(())
}

#[tauri::command]
pub fn download(file_path: String) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    std::thread::spawn(move || rt.block_on(query_data(file_path)).unwrap());
}

#[tauri::command]
pub async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}