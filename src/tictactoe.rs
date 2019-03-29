use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::io::stdin;

/**
 * @brief Cell enumuration:
 *     enumuration of the states each cell that
 *     comprise the board
 */
#[derive(PartialEq)]
enum Cell {
    Empty, //cell is empty
    O,     //cell is occupied by player O
    X      //cell is occupied by player X
}

//Default trait implementation for Cell
impl Default for Cell {
    /**
     * @brief default method:
     *     initializes default value 
     */
    fn default() -> Self {
        Cell::Empty //cell is empty by default
    }
}

//Display trait implementation for Cell
impl Display for Cell {
    /**
     * @brief fmt method:
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
     * @brief set method:
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
    fn draw(&self) {
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
    fn process_turn(&mut self, player: char, position: &str) -> bool {
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

    /**
     * @brief get_state method:
     *     get the current state of the board
     *     and determine if there's a winner
     *
     * @param[in] self:
     *     immutable reference to self
     *
     * @returns:
     *     'o':  player o wins
     *     'x':  player x wins
     *     null: no winner
     */
     fn get_state(&self) -> char {
        //row 0
        if self.a0 == self.b0 && self.a0 == self.c0 {
            match self.a0 {
                Cell::Empty => return '\0',
                Cell::O     => return 'o',
                Cell::X     => return 'x'
            }
        //row 1
        } else if self.a1 == self.b1 && self.a1 == self.c1 {
            match self.a1 {
                Cell::Empty => return '\0',
                Cell::O     => return 'x',
                Cell::X     => return 'o'
            }
        //row 2
        } else if self.a2 == self.b2 && self.a2 == self.c2 {
            match self.a2 {
                Cell::Empty => return '\0',
                Cell::O     => return 'x',
                Cell::X     => return 'o'
            }
        //column a
        } else if self.a0 == self.a1 && self.a0 == self.a2 {
            match self.a0 {
                Cell::Empty => return '\0',
                Cell::O     => return 'x',
                Cell::X     => return 'o'
            }
        //column b
        } else if self.b0 == self.b1 && self.b0 == self.b2 {
            match self.b0 {
                Cell::Empty => return '\0',
                Cell::O     => return 'x',
                Cell::X     => return 'o'
            }
        //column c
        } else if self.c0 == self.c1 && self.c0 == self.c2 {
            match self.c0 {
                Cell::Empty => return '\0',
                Cell::O     => return 'x',
                Cell::X     => return 'o'
            }
        //diagonal top left
        } else if self.a0 == self.b1 && self.a0 == self.c2 {
            match self.a0 {
                Cell::Empty => return '\0',
                Cell::O     => return 'x',
                Cell::X     => return 'o'
            }
        //diagonal bot left
        } else if self.a2 == self.b1 && self.a2 == self.c0 {
            match self.a2 {
                Cell::Empty => return '\0',
                Cell::O     => return 'x',
                Cell::X     => return 'o'
            }
        }
        return '\0';
    }
}

/**
 * @brief State enumuration:
 *     enumuration of the game sate
 */
 #[derive(PartialEq)]
pub enum State {
    InProgress,
    Stalemate,
    VictoryO,
    VictoryX
}

//Default trait implementation for State
impl Default for State {
    /**
     * @brief default method:
     *     initializes default value
     */
    fn default() -> Self {
        State::InProgress //cell is empty by default
    }
}

//Display trait implementation for State
impl Display for State {
    /**
     * @brief fmt method:
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
            State::InProgress => "in progress".to_string(),
            State::Stalemate  => "stalemate".to_string(),
            State::VictoryO   => "o wins".to_string(),
            State::VictoryX   => "x wins".to_string()
        };
        write!(formatter, "{}", value)
    }
}

/**
 * @brief Game structure:
 *     structure of the objects which comprise
 *     the game
 */
pub struct Game {
    board: Board,
    pub state: State,
    turn:  i32
}

//Default trait implementation for Game
impl Default for Game {
    /**
     * @brief default method:
     *     initializes default value
     */
    fn default() -> Self {
        Game {
            board: Default::default(),
            state: Default::default(),
            turn:  1
        }
    }
}

//Game trait
impl Game {
    /**
     * @brief get_player method:
     *     gets current player based on
     *     the current turn number.
     *
     * @param[in] self:
     *     immutable reference to self
     *
     * @returns:
     *     player o: 'o'
     *     player x: 'x'
     */
    fn get_player(&self) -> char {
        match self.turn % 2 {
            0 => return 'o',
            _ => return 'x'
        }
     }

    /**
     * @brief set_state method:
     *     set the current game state
     *
     * @param[in] self:
     *     mutable reference to self
     */
     fn set_state(&mut self) {
        match self.board.get_state() {
            'o' => self.state = State::VictoryO,
            'x' => self.state = State::VictoryX,
            _   => ()
        }
        if self.turn > 9 {
            self.state = State::Stalemate;
        }
     }

    /**
     * @brief take_turn method:
     *     handle the current turn
     *
     * @param[in] self:
     *     mutable reference to self
     */
     pub fn take_turn(&mut self) {
        let player = self.get_player();
        println!("\n turn: {}", self.turn);
        self.board.draw();
        loop {
            let mut position = String::new();

            println!("\nplayer {} move: ", player);
            stdin().read_line(&mut position)
                   .expect("");
            position = position.trim_right()
                               .to_string();
            if self.board.process_turn(player, &position) {
                break;
            }
        }
        self.turn += 1;
        self.set_state();
     }
}
