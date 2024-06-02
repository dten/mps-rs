#![allow(clippy::just_underscores_and_digits)]

use std::io::{Read, BufRead, BufReader};
use std::rc::Rc;
use std::collections::BTreeMap;
use std::cmp::min;

#[derive(Debug, Eq, PartialEq)]
pub enum Objective {
    Maximize,
    Minimize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum RowType {
    Eq,
    Ge,
    Le,
    Objective,
    NoRestriction,
}

#[derive(Debug, Eq, PartialEq)]
pub enum BoundType {

}

#[derive(Debug, Eq, PartialEq)]
pub struct RowDef {
    pub name: String,
    pub index: usize,
    pub typ: RowType,
}

#[derive(Debug, PartialEq, Default)]
pub struct ColumnData {
    pub data: BTreeMap<usize, f64>
}

#[derive(Debug, PartialEq, Default)]
pub struct Problem {
    pub name: String,
    pub objective: Option<Objective>,

    pub rows_by_index: Vec<Rc<RowDef>>,
    pub rows_by_id: BTreeMap<String, Rc<RowDef>>,

    pub columns_by_id: BTreeMap<String, Box<ColumnData>>,

    pub rhs_by_id: BTreeMap<String, Box<ColumnData>>,
}

struct MpsReader<'a> {
    read_line: &'a mut dyn FnMut() -> Option<Result<String, String>>,
}

impl<'a> MpsReader<'a> {

    fn read_line(&mut self) -> Option<Result<String, String>> {
        (self.read_line)()
    }

    fn err<T>(section: &str, line: &str) -> Result<T, String> {
        Err(format!("Unexpected line reading {}:\n\"{}\"", section, line))
    }

    pub fn into_problem(mut self) -> Result<Problem, String> {
        let mut problem = Problem::default();
        let mut header = self.read_name(&mut problem)?;
        loop {
            header = match header.as_str() {
                "ROWS" => self.read_rows(&mut problem)?,
                "COLUMNS" => self.read_columns(&mut problem)?,
                "RHS" => self.read_rhs(&mut problem)?,
                "BOUNDS" => self.read_bounds(&mut problem)?,
                _ => break,
            }
        }
        Ok(problem)
    }

    pub fn read_name(&mut self, problem: &mut Problem) -> Result<String, String> {
        let line = match self.read_line() {
            None => return Err("Expected NAME line".to_string()),
            Some(r) => r?,
        };

        if !line.starts_with("NAME          ") {
            return Err(format!("Expected line starting \"{}\"", "NAME          "))
        }

        let name_field: String = line.chars().skip(14).take(15).collect();
        problem.name = name_field.trim().to_string();
        if problem.name.is_empty() {
            return Err("NAME cannot be empty".to_string())
        }
        let line = match self.read_line() {
            None => return Err("Unexpected EOF reading ROWS".to_string()),
            Some(r) => r?,
        };
        Ok(line)
    }

    pub fn read_rows(&mut self, problem: &mut Problem) -> Result<String, String> {

        let mut found_objective = false;

        loop {
            let line = match self.read_line() {
                None => return Err("Unexpected EOF reading ROWS".to_string()),
                Some(r) => r?,
            };
            let unexpected_line = || Self::err("ROWS", &line);

            let line_b = line.as_bytes();
            if line_b[0] != b' ' {
                return Ok(line)

            } else {

                let (row_type, row_id, _, _, _, _) = Self::line_as_fields(&line);

                let row_type = match row_type {
                    "N" if !found_objective => {
                        found_objective = true;
                        RowType::Objective
                    },
                    "N" => RowType::NoRestriction,
                    "L" => RowType::Le,
                    "G" => RowType::Ge,
                    "E" => RowType::Eq,
                    _ => return unexpected_line(),
                };

                let name = row_id.to_string();

                let row = Rc::new(
                    RowDef {
                        name: name.clone(),
                        index: problem.rows_by_index.len(),
                        typ: row_type,
                    }
                );

                problem.rows_by_index.push(row.clone());
                problem.rows_by_id.insert(name, row);
            }
        }
    }

    pub fn read_columns(&mut self, problem: &mut Problem) -> Result<String, String> {
        loop {
            let line = match self.read_line() {
                None => return Err("Unexpected EOF reading COLUMNS".to_string()),
                Some(r) => r?,
            };

            let line_b = line.as_bytes();
            if line_b[0] != b' ' {
                return Ok(line)

            } else {
                let (_, col_id, row_id_1, row_val_1, row_id_2, row_val_2,) = Self::line_as_fields(&line);

                let col_id = col_id.to_string();
                let col = problem.columns_by_id.entry(col_id)
                                               .or_default();

                for &(id, val) in &[(row_id_1, row_val_1), (row_id_2, row_val_2)] {
                    if !id.is_empty() {
                        match problem.rows_by_id.get(id) {
                            None => return Err(format!("Unknown row id in COLUMNS: \"{}\"", id)),
                            Some(row) => col.data.insert(row.index, val.parse().unwrap()),
                        };                        
                    }
                }
            }
        }
    }

    pub fn read_rhs(&mut self, problem: &mut Problem) -> Result<String, String> {
        loop {
            let line = match self.read_line() {
                None => return Err("Unexpected EOF reading RHS".to_string()),
                Some(r) => r?,
            };

            let line_b = line.as_bytes();
            if line_b[0] != b' ' {
                return Ok(line)

            } else {
                let (_, rhs_id, row_id_1, row_val_1, row_id_2, row_val_2,) = Self::line_as_fields(&line);

                let rhs_id = rhs_id.to_string();
                let rhs = problem.rhs_by_id.entry(rhs_id)
                                               .or_default();

                for &(id, val) in &[(row_id_1, row_val_1), (row_id_2, row_val_2)] {
                    if !id.is_empty() {
                        match problem.rows_by_id.get(id) {
                            None => return Err(format!("Unknown row id in RHS: \"{}\"", id)),
                            Some(row) => rhs.data.insert(row.index, val.parse().unwrap()),
                        };                        
                    }
                }
            }
        }
    }

    pub fn read_bounds(&mut self, _problem: &mut Problem) -> Result<String, String> {
        Ok("".into())
    }

    #[allow(clippy::identity_op)]
    #[allow(clippy::eq_op)]
    fn line_as_fields(line: &str) -> (&str,&str,&str,&str,&str,&str) {

        let rem = line.as_bytes();
        let (__, rem) = rem.split_at(min(rem.len(),   1-1 + 1));
        let (_1, rem) = rem.split_at(min(rem.len(),   3-2 + 1));
        let (__, rem) = rem.split_at(min(rem.len(),   4-4 + 1));
        let (_2, rem) = rem.split_at(min(rem.len(),  12-5 + 1));
        let (__, rem) = rem.split_at(min(rem.len(),  14-13 + 1));
        let (_3, rem) = rem.split_at(min(rem.len(),  22-15 + 1));
        let (__, rem) = rem.split_at(min(rem.len(),  24-23 + 1));
        let (_4, rem) = rem.split_at(min(rem.len(),  36-25 + 1));
        let (__, rem) = rem.split_at(min(rem.len(),  39-37 + 1));
        let (_5, rem) = rem.split_at(min(rem.len(),  47-40 + 1));
        let (__, rem) = rem.split_at(min(rem.len(),  49-48 + 1));
        let (_6,   _) = rem.split_at(min(rem.len(),  61-50 + 1));

        use std::str::from_utf8;
        (from_utf8(_1).unwrap().trim(),
         from_utf8(_2).unwrap().trim(),
         from_utf8(_3).unwrap().trim(),
         from_utf8(_4).unwrap().trim(),
         from_utf8(_5).unwrap().trim(),
         from_utf8(_6).unwrap().trim())
    }
}

pub fn read<'a, R: Read + 'a>(readable: R) -> Result<Problem, String> {
    let reader = BufReader::new(readable);
    let mut lines = reader.lines();
    let mut read_line = || -> Option<Result<String, String>> {
        loop {
            return match lines.next() {
                None => None,
                Some(r) => Some(match r {
                    Err(e) => Err(e.to_string()),
                    Ok(s) => {
                        let Some(pre_comment) = s.split('*').next().map(|s| s.trim_end()) else {
                            continue
                        };
                        if pre_comment.is_empty() {
                            continue
                        }
                        if !pre_comment.is_ascii() {
                            return Some(Err(format!("Line is not ascii:\"{}\"", s)))
                        }
                        Ok(pre_comment.to_owned())
                    }
                })
            }
        }
    };

    MpsReader {
        read_line: &mut read_line
    }.into_problem()
}

pub fn parse_fixed(file_content: &str) -> Result<Problem, String>
{
    read(file_content.as_bytes())
}