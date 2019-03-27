use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

/**
 * @brief Cell enumuration:
 *     enumuration of the states each cell that
 *     comprise the board can be, where:
 */
enum Cell {
    Empty, //cell is empty
    O,     //cell is occupied by player O
    X      //cell is occupied by player X
}

//Default trait implementation for Cell
impl Default for Cell {
    /**
     * @brief default method
     *     initializes default value 
     */
    fn default() -> Self {
        Cell::Empty //cell is empty by default
    }
}

//Display trait implementation for Cell
impl Display for Cell {
    /**
     * @brief fmt method
     *     allows for formatted printing of
     *     the enumerated value
     *
     * @param[in] self:
     *     immutable reference to self
     *
     * @param[in] formatter:
     *     reference to mutable formatter for printing to
     *
     * @returns:
     *     formatter result
     */
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let value = match *self {
            Cell::Empty => ' '.to_string(),
            Cell::O     => 'O'.to_string(),
            Cell::X     => 'X'.to_string()
        };
        write!(formatter, "{}", value)
    }
}

//Cell trait
impl Cell {
    /**
     * @brief set method
     *     set the value of the cell
     *
     * @param[in] self:
     *     mutable reference to self
     *
     * @param[in] value:
     *     immutable value to set the cell to,
     *     this value can only be 'x' or 'o'
     *
     * @returns:
     *     success: true
     *     failure: false
     */
    fn set(&mut self, value: char) -> bool {
        match *self {
            Cell::Empty => (),           //only proceed if the cell is empty
            _           => return false, //otherwise return failure
        }
        match value {
            'o' => *self = Cell::O,      //only process if we are setting 'o'
            'x' => *self = Cell::X,      //or 'x'
            _   => return false          //otherwise return failure
        }
        return true;
    }
}

/**
 * @brief Board structure:
 *     structure containing the cells comprising
 *     the board
 */
#[derive(Default)]
pub struct Board {
    a0: Cell, b0: Cell, c0: Cell,
    a1: Cell, b1: Cell, c1: Cell,
    a2: Cell, b2: Cell, c2: Cell
}

//Board trait
impl Board {
    /**
     * @brief draw method:
     *     draw the current state of the
     *     board
     *
     * @param[in] self:
     *     immutable reference to self
     */
    pub fn draw(&self) {
        println!("  A B C");
        println!("0 {0} {1} {2}", self.a0, self.b0, self.c0);
        println!("1 {0} {1} {2}", self.a1, self.b1, self.c1);
        println!("2 {0} {1} {2}", self.a2, self.b2, self.c2);
    }
    
    /**
     * @brief process_turn method:
     *     process player turn and modify the
     *     state of the board accordingly
     *
     * @param[in] self:
     *     mutable reference to self
     *
     * @param[in] player:
     *     char representing which players turn
     *     it is (either 'o' or 'x')
     *
     * @param[in] position:
     *     position on the board that the turn
     *     being processed shall modify
     *
     * @returns:
     *     success: true
     *     failure: false
     */
    pub fn process_turn(&mut self, player: char, position: &str) -> bool {
        match player {
            'o' => (),          //ensure the player is either 'o'
            'x' => (),          //or 'x'
            _   => return false //otherwise return failure
        }
        match position {
            "a0" => return self.a0.set(player),
            "a1" => return self.a1.set(player),
            "a2" => return self.a2.set(player),
            "b0" => return self.b0.set(player),
            "b1" => return self.b1.set(player),
            "b2" => return self.b2.set(player),
            "c0" => return self.c0.set(player),
            "c1" => return self.c1.set(player),
            "c2" => return self.c2.set(player),
            _ => return false
        }
    }
}