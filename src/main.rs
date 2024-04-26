use std::process::exit;

fn get_integer_input() -> usize {
    let mut data = String::new();
    match std::io::stdin().read_line(&mut data) {
        Ok(_) => {}
        Err(..) => println!("Enter an integer"),
    };
    let mut ret_data: usize = 0;
    match data.trim().parse::<usize>() {
        Ok(i) => ret_data = i,
        Err(..) => println!("Enter an integer"),
    };
    return ret_data;
}

fn game_board(board: &mut [i8; 9]) {
    println!("Current State Of Board : \n\n");
    for i in 0..9 {
        if i > 0 && (i % 3) == 0 {
            println!("\n");
        }
        if board[i] == 0 {
            print!("-  ");
        }
        if board[i] == 1 {
            print!("O  ");
        }
        if board[i] == -1 {
            print!("X  ");
        }
    }
    println!("\n\n");
}

fn first_player(board: &mut [i8; 9]) {
    print!("Enter X's position from [1...9]: ");
    println!();
    let pos = get_integer_input();
    if board[pos - 1] != 0 {
        println!("Wring Move!!!");
        exit(0)
    }
    board[pos - 1] = -1
}

fn second_player(board: &mut [i8; 9]) {
    print!("Enter O's position from [1...9]: ");
    println!();
    let pos = get_integer_input();
    if board[pos - 1] != 0 {
        println!("Wring Move!!!");
        exit(0)
    }
    board[pos - 1] = 1
}

fn analyze_board(board: &mut [i8; 9]) -> i8 {
    let cb: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for i in 0..8 {
        if board[cb[i][0]] != 0
            && board[cb[i][0]] == board[cb[i][1]]
            && board[cb[i][0]] == board[cb[i][2]]
        {
            return board[cb[i][2]];
        }
    }
    return 0;
}

fn minimax(mut board: &mut [i8; 9], player: i8) -> i8 {
    let x = analyze_board(&mut board);
    if x != 0 {
        return x * player;
    }
    let mut pos = -1;
    let mut value = -2;
    for i in 0..9 {
        if board[i] == 0 {
            board[i] = player;
            let score = -minimax(&mut board, player * -1);
            if score > value {
                value = score;
                pos = i as i8;
            }
            board[i] = 0;
        }
    }
    if pos == -1 {
        return 0;
    }
    return value;
}

fn computer_turn(mut board: &mut [i8; 9]) {
    let mut pos = -1;
    let mut value = -2;
    for i in 0..9 {
        if board[i] == 0 {
            board[i] = 1;
            let score = -minimax(&mut board, -1);
            board[i] = 0;
            if score > value {
                value = score;
                pos = i as i8;
            }
        }
    }
    board[pos as usize] = 1;
}

fn main() {
    print!("Enter 1 for single player, 2 for multiplayer: ");
    println!();
    let choice = get_integer_input();
    let mut board: [i8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    if choice == 1 {
        println!("Computer : O Vs. You : X\n");
        print!("Enter to play 1(st) or 2(nd) :");
        println!();
        let player = get_integer_input();
        for i in 0..9 {
            if analyze_board(&mut board) != 0 {
                break;
            }
            if (i + player) % 2 == 0 {
                computer_turn(&mut board)
            } else {
                game_board(&mut board);
                first_player(&mut board);
            }
        }
    } else {
        for i in 0..9 {
            if analyze_board(&mut board) != 0 {
                break;
            }
            if i % 2 == 0 {
                game_board(&mut board);
                first_player(&mut board);
            } else {
                game_board(&mut board);
                second_player(&mut board);
            }
        }
    }
    let x = analyze_board(&mut board);
    if x == 0 {
        game_board(&mut board);
        println!("Draw!!!");
    }
    if x == -1 {
        game_board(&mut board);
        println!("X Wins!!! Y Loose !!!");
    }
    if x == 1 {
        game_board(&mut board);
        println!("X Loose!!! O Wins !!!!");
    }
}
