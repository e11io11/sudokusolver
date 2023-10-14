use std::fmt;

#[derive(Debug)]
enum Cell {
    Value(u8),
    Empty,
}

struct Sudoku {
    table: [Cell; 81],
}

impl TryFrom<&str> for Sudoku {
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let table = s
            .chars()
            .filter_map(|c| match c {
                '_' => Some(Cell::Empty),
                '0'..='9' => Some(Cell::Value(c.to_digit(10).unwrap().try_into().unwrap())),
                _ => None,
            })
            .take(81)
            .collect::<Vec<Cell>>()
            .try_into()?;
        Ok(Sudoku { table: table })
    }

    type Error = Vec<Cell>;
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "┌───────┬───────┬───────┐\n")?;
        for i in 0..9 {
            for j in 0..9 {
                if j % 3 == 0 {
                    write!(f, "│ ")?;
                }
                write!(f, "{} ", self.get_cell(j, i))?;
            }
            write!(f, "│\n")?;
            if i == 2 || i == 5 {
                write!(f, "├───────┼───────┼───────┤\n")?;
            }
        }
        write!(f, "└───────┴───────┴───────┘\n")
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Cell::Empty => write!(f, " "),
            Cell::Value(v) => write!(f, "{}", v),
        }
    }
}

impl Sudoku {
    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.table[y * 9 + x]
    }
}

fn main() {
    let sudoku = Sudoku::try_from(
        "427319__8
         ____8____
         _8_4_2___
         13__7__65
         __4__5_29
         __82613__
         8715_____
         __6_987_3
         34_62____",
    )
    .unwrap();
    println!("{}", sudoku);
}
